//! Listbox — one-dimensional option navigation over host-rendered rows.
//!
//! Contract: `docs/contracts/components/listbox.md`.
//! The shared headless machine owns focus, typeahead and selection transitions;
//! the renderer owns list/option semantics and the native input boundary.

use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use poodle_headless::listbox::{
    listbox_transition, ListboxBoundary, ListboxEffect, ListboxEvent, ListboxResult,
};
use poodle_node::{CursorHint, FocusRing, LayoutDirection, Node, NodeKey, NodeModifiers, NodeRole};
use poodle_specs::{ListboxItem, ListboxSelectionMode, ListboxSpec};

use crate::context::RenderContext;
use crate::presentation::rem_to_px;

pub type ListboxTransitionResult = ListboxResult;
pub type ListboxRowRenderer<'a> = dyn Fn(&ListboxItem, bool, bool) -> Node + 'a;

/// Host-owned interaction for one Listbox. Hosts apply the returned context,
/// call the selection/activation effects, and execute focus effects using the
/// stable option IDs returned by [`listbox_option_focus_id`].
#[derive(Clone)]
pub struct ListboxHandlers {
    pub instance_id: String,
    pub on_transition: Option<Arc<dyn Fn(ListboxTransitionResult) + Send + Sync>>,
}

impl ListboxHandlers {
    pub fn new(instance_id: impl Into<String>) -> Self {
        let instance_id = instance_id.into();
        assert!(
            !instance_id.trim().is_empty(),
            "ListboxHandlers requires a non-empty lifetime-stable instance_id"
        );
        Self {
            instance_id,
            on_transition: None,
        }
    }

    pub fn on_transition(
        mut self,
        handler: Arc<dyn Fn(ListboxTransitionResult) + Send + Sync>,
    ) -> Self {
        self.on_transition = Some(handler);
        self
    }
}

fn part_id(scope: &str, part: &str) -> String {
    format!("listbox:{scope}:{part}")
}

pub fn listbox_root_id(scope: &str) -> String {
    part_id(scope, "root")
}

/// Stable backend focus identity for one option in one Listbox instance.
pub fn listbox_option_focus_id(scope: &str, value: &str) -> String {
    format!("listbox:{scope}:option:{value}")
}

fn emit(
    spec: &ListboxSpec,
    handlers: &ListboxHandlers,
    event: ListboxEvent,
) -> ListboxTransitionResult {
    let result = listbox_transition(&spec.listbox_context(), event);
    if let Some(handler) = &handlers.on_transition {
        handler(result.clone());
    }
    result
}

fn focus_effect_target(result: &ListboxTransitionResult, scope: &str) -> Option<String> {
    result.effects.iter().find_map(|effect| match effect {
        ListboxEffect::Focus { value } => Some(listbox_option_focus_id(scope, value)),
        _ => None,
    })
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .min(u128::from(u64::MAX)) as u64
}

fn direction_for_key(spec: &ListboxSpec, key: NodeKey) -> Option<i8> {
    match (spec.orientation, key) {
        (poodle_specs::ListboxOrientation::Vertical, NodeKey::ArrowDown)
        | (poodle_specs::ListboxOrientation::Horizontal, NodeKey::ArrowRight) => Some(1),
        (poodle_specs::ListboxOrientation::Vertical, NodeKey::ArrowUp)
        | (poodle_specs::ListboxOrientation::Horizontal, NodeKey::ArrowLeft) => Some(-1),
        _ => None,
    }
}

#[derive(Clone, Copy)]
enum KeyboardActivation {
    Space,
    Enter,
}

fn option_node(
    spec: &ListboxSpec,
    item: &ListboxItem,
    handlers: &ListboxHandlers,
    render_row: &ListboxRowRenderer<'_>,
    ctx: &RenderContext<'_>,
) -> Node {
    let selected = spec.is_selected(&item.value);
    let focused = spec.listbox_context().focused_value.as_deref() == Some(item.value.as_str());
    let disabled = spec.is_item_disabled(item);
    let mut row = Node::container();
    row.id = Some(listbox_option_focus_id(&handlers.instance_id, &item.value));
    row.runtime_id = row.id.clone();
    row.a11y.role = Some(NodeRole::ListBoxOption);
    row.a11y.label = Some(item.label.clone());
    row.a11y.selected = Some(selected);
    row.a11y.tab_index = Some(if focused && !disabled { 0 } else { -1 });
    row.style.descriptor.layout.direction = LayoutDirection::Row;
    row.style.self_stretch = spec.orientation == poodle_specs::ListboxOrientation::Vertical;
    row.style.min_width = Some(0.0);
    row.style.descriptor.cursor = CursorHint::Pointer;
    row.style.focus_ring = Some(FocusRing {
        color: ctx.theme().resolve_color("color.accent.focusRing"),
        width: ctx.theme().resolve_border_width("border.width.focus"),
        offset: rem_to_px(-0.0625),
    });
    row.interaction.focusable = !disabled;
    row.interaction.disabled = disabled;
    if disabled {
        row.style.descriptor.opacity = ctx.theme().resolve_opacity("state.opacity.disabled");
    }
    row = row.child(render_row(item, selected, focused));

    if disabled || handlers.on_transition.is_none() {
        return row;
    }

    let focus_spec = spec.clone();
    let focus_handlers = handlers.clone();
    let focus_value = item.value.clone();
    row.interaction.on_focus_change = Some(Arc::new(move |has_focus| {
        if has_focus {
            emit(
                &focus_spec,
                &focus_handlers,
                ListboxEvent::Focus {
                    value: focus_value.clone(),
                },
            );
        }
    }));

    let keyboard_spec = spec.clone();
    let keyboard_handlers = handlers.clone();
    let keyboard_scope = handlers.instance_id.clone();
    row.interaction.on_key = Some(Arc::new(move |key, modifiers| {
        let event = if let Some(direction) = direction_for_key(&keyboard_spec, key) {
            Some(ListboxEvent::Move {
                direction,
                extend_selection: modifiers.shift
                    && keyboard_spec.selection_mode == ListboxSelectionMode::Multiple,
            })
        } else {
            match key {
                NodeKey::Home => Some(ListboxEvent::Boundary {
                    boundary: ListboxBoundary::First,
                }),
                NodeKey::End => Some(ListboxEvent::Boundary {
                    boundary: ListboxBoundary::Last,
                }),
                _ => None,
            }
        };
        let Some(event) = event else {
            return None;
        };
        let result = emit(&keyboard_spec, &keyboard_handlers, event);
        focus_effect_target(&result, &keyboard_scope)
    }));

    let click_spec = spec.clone();
    let click_handlers = handlers.clone();
    let click_value = item.value.clone();
    row.interaction.on_activate_modified = Some(Arc::new(move |modifiers: NodeModifiers| {
        emit(
            &click_spec,
            &click_handlers,
            ListboxEvent::Select {
                value: click_value.clone(),
                additive: modifiers.accel,
                range: modifiers.shift,
            },
        );
    }));

    let double_spec = spec.clone();
    let double_handlers = handlers.clone();
    let double_value = item.value.clone();
    row.interaction.on_double_activate = Some(Arc::new(move |_| {
        emit(
            &double_spec,
            &double_handlers,
            ListboxEvent::Activate {
                value: Some(double_value.clone()),
            },
        );
    }));

    // The node backend reports raw key names for this channel. The host
    // transition callback receives typeahead focus effects and applies them
    // through the normal GPUI focus request path.
    let pending_activation: Arc<Mutex<Option<KeyboardActivation>>> = Arc::default();
    let edit_spec = spec.clone();
    let edit_handlers = handlers.clone();
    let edit_pending = Arc::clone(&pending_activation);
    row.interaction.on_edit_key = Some(Arc::new(move |key, modifiers| {
        match key {
            "space" => {
                *edit_pending.lock().unwrap() = Some(KeyboardActivation::Space);
                return;
            }
            "enter" => {
                *edit_pending.lock().unwrap() = Some(KeyboardActivation::Enter);
                return;
            }
            _ => *edit_pending.lock().unwrap() = None,
        }
        if modifiers.accel && key.eq_ignore_ascii_case("a") {
            emit(&edit_spec, &edit_handlers, ListboxEvent::SelectAll);
            return;
        }
        if modifiers.alt || modifiers.accel || key.chars().count() != 1 || key.trim().is_empty() {
            return;
        }
        emit(
            &edit_spec,
            &edit_handlers,
            ListboxEvent::Typeahead {
                character: key.to_string(),
                now: now_ms(),
            },
        );
    }));

    let activation_spec = spec.clone();
    let activation_handlers = handlers.clone();
    let activation_value = item.value.clone();
    row.interaction.on_key_activate = Some(Arc::new(move || {
        let activation = pending_activation.lock().unwrap().take();
        match activation {
            Some(KeyboardActivation::Space) => {
                emit(&activation_spec, &activation_handlers, ListboxEvent::Space);
            }
            Some(KeyboardActivation::Enter) => {
                emit(
                    &activation_spec,
                    &activation_handlers,
                    ListboxEvent::Activate {
                        value: Some(activation_value.clone()),
                    },
                );
            }
            None => {}
        }
        None
    }));

    row
}

/// Render labels as plain text. Use [`listbox_with_rows`] when a host needs
/// richer rows; its callback receives the item, selected state, and focus
/// state while Listbox keeps ownership of the option node.
pub fn listbox(spec: &ListboxSpec, ctx: &RenderContext<'_>, handlers: &ListboxHandlers) -> Node {
    listbox_with_rows(spec, ctx, handlers, |item, _, _| {
        Node::text(item.label.clone())
    })
}

pub fn listbox_with_rows(
    spec: &ListboxSpec,
    ctx: &RenderContext<'_>,
    handlers: &ListboxHandlers,
    render_row: impl Fn(&ListboxItem, bool, bool) -> Node,
) -> Node {
    let enabled = !spec.disabled && spec.items.iter().any(|item| !item.disabled);
    let empty = !enabled;
    let mut root = Node::container();
    root.id = Some(listbox_root_id(&handlers.instance_id));
    root.runtime_id = root.id.clone();
    root.a11y.role = Some(NodeRole::ListBox);
    root.a11y.label = spec.aria_label.clone();
    root.a11y.labelled_by = spec.aria_labelledby.clone();
    root.a11y.orientation = Some(spec.orientation.as_str().to_string());
    root.a11y.multiselectable =
        (spec.selection_mode == ListboxSelectionMode::Multiple).then_some(true);
    root.a11y.tab_index = if spec.disabled {
        Some(-1)
    } else if empty {
        Some(0)
    } else {
        None
    };
    root.style.descriptor.layout.direction = match spec.orientation {
        poodle_specs::ListboxOrientation::Vertical => LayoutDirection::Column,
        poodle_specs::ListboxOrientation::Horizontal => LayoutDirection::Row,
    };
    root.style.min_width = Some(0.0);
    root.style.focus_ring = (empty && !spec.disabled).then(|| FocusRing {
        color: ctx.theme().resolve_color("color.accent.focusRing"),
        width: ctx.theme().resolve_border_width("border.width.focus"),
        offset: rem_to_px(0.125),
    });
    root.interaction.focusable = empty && !spec.disabled;
    root.interaction.disabled = spec.disabled;
    if spec.disabled {
        root.style.descriptor.opacity = ctx.theme().resolve_opacity("state.opacity.disabled");
    }

    for item in &spec.items {
        root = root.child(option_node(spec, item, handlers, &render_row, ctx));
    }
    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::{ListboxItem, ListboxOrientation, ListboxSelectionMode};

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn options() -> Vec<ListboxItem> {
        vec![
            ListboxItem::new("alpha", "Alpha"),
            ListboxItem::new("beta", "Beta").with_disabled(true),
            ListboxItem::new("gamma", "Gamma"),
        ]
    }

    #[test]
    fn emits_listbox_option_semantics_focus_and_token_ring() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = ListboxSpec::new(options())
            .with_aria_label("Packages")
            .with_selection_mode(ListboxSelectionMode::Multiple)
            .with_orientation(ListboxOrientation::Horizontal)
            .with_values(vec!["gamma".into()]);
        let root = listbox(&spec, &ctx, &ListboxHandlers::new("test"));
        assert_eq!(root.a11y.role, Some(NodeRole::ListBox));
        assert_eq!(root.a11y.label.as_deref(), Some("Packages"));
        assert_eq!(root.a11y.orientation.as_deref(), Some("horizontal"));
        assert_eq!(root.a11y.multiselectable, Some(true));
        assert_eq!(root.children.len(), 3);
        assert_eq!(root.children[0].a11y.role, Some(NodeRole::ListBoxOption));
        assert_eq!(root.children[0].a11y.label.as_deref(), Some("Alpha"));
        assert_eq!(root.children[0].a11y.tab_index, Some(-1));
        assert_eq!(root.children[1].interaction.disabled, true);
        assert_eq!(root.children[2].a11y.selected, Some(true));
        assert_eq!(root.children[2].a11y.tab_index, Some(0));
        assert!(root.children[2].style.focus_ring.is_some());
    }

    #[test]
    fn host_row_renderer_receives_selection_and_focus() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = ListboxSpec::new(options()).with_value(Some("gamma".into()));
        let root = listbox_with_rows(
            &spec,
            &ctx,
            &ListboxHandlers::new("rows"),
            |item, selected, focused| {
                let state = if selected {
                    "selected"
                } else if focused {
                    "focused"
                } else {
                    "idle"
                };
                Node::text(format!("{}:{state}", item.label))
            },
        );
        match &root.children[2].children[0].kind {
            poodle_node::NodeKind::Text { content } => assert_eq!(content, "Gamma:selected"),
            _ => panic!("host renderer node is retained inside the option"),
        }
    }

    #[test]
    fn empty_and_disabled_roots_use_the_contract_tab_stop() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let empty = listbox(
            &ListboxSpec::new(Vec::new()).with_aria_label("Empty"),
            &ctx,
            &ListboxHandlers::new("empty"),
        );
        assert_eq!(empty.a11y.tab_index, Some(0));
        assert!(empty.interaction.focusable);

        let disabled = listbox(
            &ListboxSpec::new(options()).with_disabled(true),
            &ctx,
            &ListboxHandlers::new("disabled"),
        );
        assert_eq!(disabled.a11y.tab_index, Some(-1));
        assert!(disabled.interaction.disabled);
        assert!(disabled.style.focus_ring.is_none());
        assert!(disabled
            .children
            .iter()
            .all(|node| node.interaction.disabled));
    }
}
