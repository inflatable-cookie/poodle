//! CardToggleGroup — multi-select card group in a capped-column grid.
//!
//! Contract: `docs/contracts/components/card-toggle-group.md`
//! Ported from: `packages/jetstream/components/src/card_toggle_group.rs`.
//!
//! Options lay out in rows of `column_count()` cells (1–4); a short final
//! row is padded with flex spacers so card widths stay aligned across rows.
//! Recipe reconciled to the old GPUI tier
//! (`packages/gpui/components/src/composites/card_toggle_group.rs`): the
//! density-table grid gap, spec-helper fonts, Card-composed selection
//! treatment, and focusable option cells carrying the toggle handler.
//!
//! Selection runs through the shared ToggleGroup machine in single mode with
//! the spec's `allow_deactivation`, so the callback receives the resulting
//! `string | null`, not the pressed option. Arrow navigation walks the enabled
//! options, wraps at the ends, selects the target, and moves real backend
//! focus — the Svelte `menuListNavigate` behaviour.

use std::sync::Arc;

use poodle_headless::single_select::SelectOption;
use poodle_headless::toggle_group::{
    toggle_group_transition, SelectionMode, ToggleGroupContext, ToggleGroupEffect,
    ToggleGroupEvent, ToggleGroupValue,
};
use poodle_node::{
    CursorHint, FocusRing, LayoutDirection, Node, NodeKey, NodeModifiers, NodeRole, NodeToggled,
};
use poodle_specs::{CardSpec, CardToggleGroupSpec};

use crate::card::card;
use crate::context::RenderContext;
use crate::presentation::{control_space_x_rem, rem_to_px};

/// Host-owned native interaction for one CardToggleGroup instance.
///
/// `instance_id` is the lifetime-stable scope. It is not the web form name and
/// the renderer never invents one from render order or option values.
#[derive(Clone)]
pub struct CardToggleGroupHandlers {
    pub instance_id: String,
    /// Receives the resulting value after `toggle_group_transition`, so
    /// `null` (deactivation) is a real payload rather than a dropped event.
    pub on_value_change: Option<Arc<dyn Fn(Option<&str>) + Send + Sync>>,
}

impl CardToggleGroupHandlers {
    pub fn new(instance_id: impl Into<String>) -> Self {
        Self {
            instance_id: instance_id.into(),
            on_value_change: None,
        }
    }

    pub fn on_value_change(mut self, handler: Arc<dyn Fn(Option<&str>) + Send + Sync>) -> Self {
        self.on_value_change = Some(handler);
        self
    }
}

fn option_id(value: &str) -> String {
    format!("card-toggle:{value}")
}

fn option_focus_id(instance_scope: &str, value: &str) -> String {
    format!("card-toggle:{instance_scope}:option:{value}")
}

fn flex1_cell() -> Node {
    let mut n = Node::container();
    let s = &mut n.style;
    // Explicit Row (see switch.rs).
    s.descriptor.layout.direction = LayoutDirection::Row;
    s.flex_grow = Some(1.0);
    s.flex_basis = Some(0.0);
    n
}

fn toggle_context(spec: &CardToggleGroupSpec) -> ToggleGroupContext {
    ToggleGroupContext {
        // The spec stores a single selection as a vector; the first stored
        // value is the selection in single mode.
        value: ToggleGroupValue::Single(spec.values.first().cloned()),
        options: spec
            .options
            .iter()
            .map(|option| SelectOption {
                value: option.value.clone(),
                disabled: option.disabled,
            })
            .collect(),
        selection_mode: SelectionMode::Single,
        allow_deactivation: spec.allow_deactivation,
        disabled: spec.disabled,
    }
}

fn roving_values(spec: &CardToggleGroupSpec) -> Vec<String> {
    spec.options
        .iter()
        .filter(|option| !spec.disabled && !option.disabled)
        .map(|option| option.value.clone())
        .collect()
}

fn tab_stop_value<'a>(spec: &'a CardToggleGroupSpec, roving: &'a [String]) -> Option<&'a str> {
    let selected = spec.values.first().map(String::as_str);
    if selected.is_some_and(|value| roving.iter().any(|candidate| candidate == value)) {
        selected
    } else {
        roving.first().map(String::as_str)
    }
}

fn emit_toggle(
    context: &ToggleGroupContext,
    option_value: &str,
    on_value_change: &Option<Arc<dyn Fn(Option<&str>) + Send + Sync>>,
) {
    let Some(handler) = on_value_change else {
        return;
    };
    let (_, effects) = toggle_group_transition(
        context.clone(),
        ToggleGroupEvent::Toggle {
            value: option_value.to_string(),
        },
    );
    for effect in effects {
        let ToggleGroupEffect::EmitValueChange { value } = effect;
        if let ToggleGroupValue::Single(single) = value {
            handler(single.as_deref());
        }
    }
}

/// Arrow navigation over the enabled options, wrapping at both ends. The
/// target is selected (emitting through the toggle machine) and returned as
/// the focus identity the backend moves to. Both axes are live, matching
/// Svelte's `menuListNavigate` over the enabled list.
fn roving_key_handler(
    value: &str,
    roving: &[String],
    instance_scope: String,
    context: ToggleGroupContext,
    on_value_change: Option<Arc<dyn Fn(Option<&str>) + Send + Sync>>,
) -> Option<Arc<dyn Fn(NodeKey, NodeModifiers) -> Option<String> + Send + Sync>> {
    let index = roving.iter().position(|candidate| candidate == value)?;
    let ids = roving.to_vec();
    let current = value.to_string();
    Some(Arc::new(move |key, _modifiers| {
        if ids.is_empty() {
            return None;
        }
        let last = ids.len() - 1;
        let next = match key {
            NodeKey::ArrowRight | NodeKey::ArrowDown => {
                if index == last {
                    0
                } else {
                    index + 1
                }
            }
            NodeKey::ArrowLeft | NodeKey::ArrowUp => {
                if index == 0 {
                    last
                } else {
                    index - 1
                }
            }
            _ => return None,
        };
        let target = ids[next].clone();
        if target == current {
            return None;
        }
        emit_toggle(&context, &target, &on_value_change);
        Some(option_focus_id(&instance_scope, &target))
    }))
}

/// Render a group without a caller-provided identity. Prefer
/// [`card_toggle_group_with_handlers`] so duplicate instances keep distinct
/// focus handles; this entry point exists for static and single-instance
/// compositions.
pub fn card_toggle_group(
    spec: &CardToggleGroupSpec,
    ctx: &RenderContext<'_>,
    on_value_change: Option<Arc<dyn Fn(Option<&str>) + Send + Sync>>,
) -> Node {
    card_toggle_group_with_handlers(
        spec,
        ctx,
        CardToggleGroupHandlers {
            instance_id: "card-toggle-group".to_string(),
            on_value_change,
        },
    )
}

pub fn card_toggle_group_with_handlers(
    spec: &CardToggleGroupSpec,
    ctx: &RenderContext<'_>,
    handlers: CardToggleGroupHandlers,
) -> Node {
    let theme = ctx.theme();
    let instance_scope = handlers.instance_id.as_str();
    let on_value_change = handlers.on_value_change;
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);

    // Contract §7 size scale — via the spec helpers.
    let title_font = rem_to_px(CardToggleGroupSpec::title_font_rem(effective_size));
    let description_font = rem_to_px(CardToggleGroupSpec::description_font_rem(effective_size));

    // Density-driven grid gap (contract §7 density table) + Card body rhythm.
    let grid_gap = rem_to_px(control_space_x_rem(density));
    let body_gap = rem_to_px(0.25);

    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let disabled_opacity = theme.resolve_opacity("state.opacity.disabled");
    let focus_ring = FocusRing {
        color: theme.resolve_color("color.accent.focusRing"),
        width: theme.resolve_border_width("border.width.focus"),
        // Contract §7 option focus: outline-offset 0.125rem.
        offset: rem_to_px(0.125),
    };

    let context = toggle_context(spec);
    let roving = roving_values(spec);
    let tab_stop = tab_stop_value(spec, &roving);

    // Contract §6: rows of `column_count()` cells.
    let cols = spec.column_count();
    let mut cells: Vec<Node> = Vec::new();

    for option in &spec.options {
        let is_selected = spec.is_selected(&option.value);
        let is_option_disabled = spec.disabled || option.disabled;

        // Card body: title + optional description.
        let mut body = Node::container();
        {
            let s = &mut body.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = body_gap;
        }
        let mut title = Node::text(option.title.clone());
        title.style.text_size = Some(title_font);
        title.style.text_weight = Some(600);
        title.style.descriptor.text_color = Some(text_primary);
        let mut body = body.child(title);
        if let Some(description) = &option.description {
            let mut d = Node::text(description.clone());
            d.style.text_size = Some(description_font);
            d.style.descriptor.text_color = Some(text_secondary);
            body = body.child(d);
        }

        // Compose the Card primitive — selected state owns the fill/border.
        let mut card_spec = CardSpec::new().interactive();
        if is_selected {
            card_spec = card_spec.selected();
        }
        card_spec = card_spec.with_aria_label(option.title.clone());
        // The old Card's body slot wraps content in a flex-growing box.
        let mut body_slot = Node::container();
        body_slot.style.descriptor.layout.direction = LayoutDirection::Row;
        body_slot.style.flex_grow = Some(1.0);
        let option_card = card(&card_spec, ctx, vec![body_slot.child(body)]);

        // Wrap each Card so it can grow within the grid. The cell is the
        // focusable activation target; a disabled option dims and shows the
        // not-allowed cursor instead of wiring the toggle.
        let mut option_el = flex1_cell();
        option_el.id = Some(option_id(&option.value));
        option_el.runtime_id = Some(option_focus_id(instance_scope, &option.value));
        option_el.a11y.role = Some(NodeRole::Button);
        option_el.a11y.label = Some(option.title.clone());
        option_el.a11y.selected = Some(is_selected);
        option_el.a11y.toggled = Some(if is_selected {
            NodeToggled::True
        } else {
            NodeToggled::False
        });
        option_el.style.min_width = Some(0.0);
        option_el.interaction.focusable = true;
        let mut option_el = option_el.child(option_card);
        if is_option_disabled {
            option_el.style.descriptor.opacity = disabled_opacity;
            option_el.style.descriptor.cursor = CursorHint::NotAllowed;
            option_el.interaction.disabled = true;
            option_el.interaction.focusable = false;
            option_el.a11y.tab_index = Some(-1);
        } else {
            option_el.style.descriptor.cursor = CursorHint::Pointer;
            option_el.a11y.tab_index = Some(if tab_stop == Some(option.value.as_str()) {
                0
            } else {
                -1
            });
            option_el.style.focus_ring = Some(focus_ring);
            if let Some(handler) = &on_value_change {
                let handler = Arc::clone(handler);
                let context = context.clone();
                let value = option.value.clone();
                option_el.interaction.on_activate = Some(Arc::new(move || {
                    emit_toggle(&context, &value, &Some(Arc::clone(&handler)));
                }));
            }
            option_el.interaction.on_key = roving_key_handler(
                &option.value,
                &roving,
                instance_scope.to_string(),
                context.clone(),
                on_value_change.clone(),
            );
        }

        cells.push(option_el);
    }

    // Assemble rows; pad a short final row with flex spacers.
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = grid_gap;
    }
    let mut iter = cells.into_iter();
    let mut remaining = spec.options.len();
    while remaining > 0 {
        let take = cols.min(remaining);
        let mut row = Node::container();
        {
            let s = &mut row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.spacing.gap = grid_gap;
        }
        for _ in 0..take {
            if let Some(cell) = iter.next() {
                row = row.child(cell);
            }
        }
        for _ in take..cols {
            row = row.child(flex1_cell());
        }
        root = root.child(row);
        remaining -= take;
    }

    if spec.disabled {
        root.style.descriptor.opacity = disabled_opacity;
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            root.a11y.label = Some(label.to_string());
        }
    }
    root.a11y.role = Some(NodeRole::Group);
    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_adapter::ThemeProvider;
    use poodle_specs::CardToggleOption;

    /// The real token resolver over the ECLIPSE theme. Pure — no backend.
    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn options() -> Vec<CardToggleOption> {
        vec![
            CardToggleOption::new("alpha", "Alpha").with_description("First option"),
            CardToggleOption::new("beta", "Beta"),
            CardToggleOption::new("gamma", "Gamma"),
        ]
    }

    /// The option cells are the identified wrappers around each card.
    fn cells(node: &Node) -> Vec<&Node> {
        fn walk<'a>(node: &'a Node, out: &mut Vec<&'a Node>) {
            if node
                .runtime_id
                .as_deref()
                .is_some_and(|id| id.starts_with("card-toggle:"))
            {
                out.push(node);
            }
            for child in &node.children {
                walk(child, out);
            }
        }
        let mut out = Vec::new();
        walk(node, &mut out);
        out
    }

    #[test]
    fn grid_gap_follows_the_density_ladder() {
        // Contract §7 density table (rem_to_px = rem * 16):
        // compact 0.5rem · default 0.75rem · comfortable 1rem.
        let cases = [
            (poodle_specs::ControlDensity::Compact, 8.0),
            (poodle_specs::ControlDensity::Default, 12.0),
            (poodle_specs::ControlDensity::Comfortable, 16.0),
        ];
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        for (density, expected) in cases {
            let spec = CardToggleGroupSpec::new(options()).with_density(density);
            let node = card_toggle_group(&spec, &ctx, None);
            assert_eq!(
                node.style.descriptor.layout.spacing.gap, expected,
                "root gap for {density:?}"
            );
            assert_eq!(
                node.children[0].style.descriptor.layout.spacing.gap, expected,
                "row gap for {density:?}"
            );
        }
    }

    #[test]
    fn title_and_description_fonts_follow_the_size_ladder() {
        // Contract §7 size tables: title xs 0.6875 · sm 0.75 · md 0.875 ·
        // lg 1 · xl 1.125; description xs 0.625 · sm 0.6875 · md 0.75 ·
        // lg 0.875 · xl 0.9375.
        let cases = [
            (poodle_specs::ControlSize::Xs, 11.0, 10.0),
            (poodle_specs::ControlSize::Sm, 12.0, 11.0),
            (poodle_specs::ControlSize::Md, 14.0, 12.0),
            (poodle_specs::ControlSize::Lg, 16.0, 14.0),
            (poodle_specs::ControlSize::Xl, 18.0, 15.0),
        ];
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let text_primary = theme.resolve_color("color.text.primary");
        let text_secondary = theme.resolve_color("color.text.secondary");
        for (size, expected_title, expected_description) in cases {
            let spec = CardToggleGroupSpec::new(options()).with_size(size);
            let node = card_toggle_group(&spec, &ctx, None);

            let title = node
                .find(
                    &|n| matches!(&n.kind, poodle_node::NodeKind::Text { content } if content == "Alpha"),
                )
                .expect("title text");
            assert_eq!(
                title.style.text_size,
                Some(expected_title),
                "title font for {size:?}"
            );
            assert_eq!(title.style.text_weight, Some(600));
            assert_eq!(title.style.descriptor.text_color, Some(text_primary));

            let description = node
                .find(
                    &|n| matches!(&n.kind, poodle_node::NodeKind::Text { content } if content == "First option"),
                )
                .expect("description text");
            assert_eq!(
                description.style.text_size,
                Some(expected_description),
                "description font for {size:?}"
            );
            assert_eq!(
                description.style.descriptor.text_color,
                Some(text_secondary)
            );
        }
    }

    #[test]
    fn selected_card_carries_the_accent_border_and_option_label() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let accent = theme.resolve_color("color.accent.base");
        let spec = CardToggleGroupSpec::new(options()).with_values(vec!["alpha".to_string()]);
        let node = card_toggle_group(&spec, &ctx, None);

        // Each card is labelled with its option title; selection owns the
        // border through the composed Card primitive.
        let alpha_card = node
            .find(&|n| {
                n.a11y.label.as_deref() == Some("Alpha") && n.style.descriptor.border.width > 0.0
            })
            .expect("alpha card");
        assert_eq!(alpha_card.style.descriptor.border.color, accent);
        let beta_card = node
            .find(&|n| {
                n.a11y.label.as_deref() == Some("Beta") && n.style.descriptor.border.width > 0.0
            })
            .expect("beta card");
        assert_ne!(beta_card.style.descriptor.border.color, accent);
    }

    #[test]
    fn option_cells_carry_the_button_role_and_pressed_state() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = CardToggleGroupSpec::new(options()).with_values(vec!["alpha".to_string()]);
        let node =
            card_toggle_group_with_handlers(&spec, &ctx, CardToggleGroupHandlers::new("views"));

        assert_eq!(node.a11y.role, Some(NodeRole::Group));
        let cells = cells(&node);
        assert_eq!(cells.len(), 3, "one focusable cell per option");
        let alpha = cells
            .iter()
            .find(|c| c.has_text("Alpha"))
            .expect("alpha cell");
        assert_eq!(alpha.a11y.role, Some(NodeRole::Button));
        assert_eq!(alpha.a11y.label.as_deref(), Some("Alpha"));
        assert_eq!(alpha.a11y.selected, Some(true));
        assert_eq!(alpha.a11y.toggled, Some(NodeToggled::True));
        assert_eq!(
            alpha.runtime_id.as_deref(),
            Some("card-toggle:views:option:alpha")
        );
        assert_eq!(alpha.a11y.tab_index, Some(0));
        assert!(alpha.style.focus_ring.is_some());
        let beta = cells
            .iter()
            .find(|c| c.has_text("Beta"))
            .expect("beta cell");
        assert_eq!(beta.a11y.toggled, Some(NodeToggled::False));
        assert_eq!(beta.a11y.tab_index, Some(-1));
    }

    #[test]
    fn toggling_an_option_reports_the_resulting_value_through_the_node_handler() {
        use std::sync::Mutex;
        let seen: Arc<Mutex<Vec<Option<String>>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let on_change: Arc<dyn Fn(Option<&str>) + Send + Sync> =
            Arc::new(move |v: Option<&str>| sink.lock().unwrap().push(v.map(str::to_owned)));
        let spec = CardToggleGroupSpec::new(options());
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = card_toggle_group(&spec, &ctx, Some(on_change));

        let cells = cells(&node);
        assert_eq!(cells.len(), 3, "one focusable cell per option");
        let beta = cells
            .iter()
            .find(|c| c.has_text("Beta"))
            .expect("beta cell");
        (beta
            .interaction
            .on_activate
            .as_ref()
            .expect("beta is activatable"))();
        assert_eq!(seen.lock().unwrap().as_slice(), [Some("beta".to_string())]);
    }

    #[test]
    fn deactivation_reports_null_when_the_spec_allows_it() {
        use std::sync::Mutex;
        let seen: Arc<Mutex<Vec<Option<String>>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let on_change: Arc<dyn Fn(Option<&str>) + Send + Sync> =
            Arc::new(move |v: Option<&str>| sink.lock().unwrap().push(v.map(str::to_owned)));
        let spec = CardToggleGroupSpec::new(options())
            .with_values(vec!["alpha".to_string()])
            .with_allow_deactivation(true);
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = card_toggle_group(&spec, &ctx, Some(on_change));

        let cells = cells(&node);
        let alpha = cells
            .iter()
            .find(|c| c.has_text("Alpha"))
            .expect("alpha cell");
        (alpha
            .interaction
            .on_activate
            .as_ref()
            .expect("alpha is activatable"))();
        assert_eq!(seen.lock().unwrap().as_slice(), [None]);
    }

    #[test]
    fn enabled_options_keep_the_old_tiers_pointer_without_a_handler() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = card_toggle_group(&CardToggleGroupSpec::new(options()), &ctx, None);
        assert!(cells(&node)
            .iter()
            .all(|cell| cell.style.descriptor.cursor == CursorHint::Pointer));
    }

    #[test]
    fn a_disabled_option_dims_and_shows_the_not_allowed_cursor() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let disabled_opacity = theme.resolve_opacity("state.opacity.disabled");
        let mut opts = options();
        opts[2] = opts[2].clone().with_disabled(true);
        let spec = CardToggleGroupSpec::new(opts);
        let node = card_toggle_group(&spec, &ctx, Some(Arc::new(|_: Option<&str>| {})));

        let cells = cells(&node);
        let gamma = cells
            .iter()
            .find(|c| c.has_text("Gamma"))
            .expect("gamma cell");
        assert!(gamma.interaction.on_activate.is_none());
        assert_eq!(gamma.style.descriptor.opacity, disabled_opacity);
        assert!(matches!(
            gamma.style.descriptor.cursor,
            CursorHint::NotAllowed
        ));

        let alpha = cells
            .iter()
            .find(|c| c.has_text("Alpha"))
            .expect("alpha cell");
        assert!(alpha.interaction.on_activate.is_some());
        assert!(matches!(alpha.style.descriptor.cursor, CursorHint::Pointer));
    }

    #[test]
    fn a_disabled_group_dims_the_root_and_wires_nothing() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let disabled_opacity = theme.resolve_opacity("state.opacity.disabled");
        let spec = CardToggleGroupSpec::new(options()).with_disabled(true);
        let node = card_toggle_group(&spec, &ctx, Some(Arc::new(|_: Option<&str>| {})));
        assert_eq!(node.style.descriptor.opacity, disabled_opacity);
        assert!(cells(&node)
            .iter()
            .all(|c| c.interaction.on_activate.is_none()));
    }

    #[test]
    fn a_short_final_row_is_padded_with_spacers() {
        // 3 options in 2 columns: rows of 2 and 1, the short row padded so
        // card widths stay aligned across rows.
        let spec = CardToggleGroupSpec::new(options()).with_columns(2);
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = card_toggle_group(&spec, &ctx, None);
        assert_eq!(node.children.len(), 2, "two rows");
        assert_eq!(node.children[0].children.len(), 2, "full first row");
        assert_eq!(node.children[1].children.len(), 2, "padded second row");
        // The spacer carries no card.
        assert!(!node.children[1].children[1].has_text("Gamma"));
    }
}
