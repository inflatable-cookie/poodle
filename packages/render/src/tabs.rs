//! Tabs — a tab bar in the card, pill, and block variants.
//!
//! Contract: `docs/contracts/components/tabs.md`
//! Ported from: `packages/jetstream/components/src/tabs/`. Content for the
//! active tab renders below this element, by the caller. The card variant
//! renders icon, count, and close-button accessories; close buttons wire
//! through `on_close` (inert when unwired, so an unwired X does not bubble to
//! the tab and select what it was closing).
//!
//! There is no `TabVariant::Strip`: the strip renders through the separate
//! `TabStripSpec`/`TabStrip` component on the native targets. Known gap,
//! deliberately deferred — recorded in the g13-013 batch log.

use std::sync::Arc;

use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment,
    Node, NodeDropCommit, NodeKey, NodeRole, ShadowLayer, StylePatch,
};
use poodle_specs::{ActiveEdge, ActiveFill, Orientation, TabActivationMode, TabVariant, TabsSpec};

use crate::color::{mix_srgb, with_alpha, TRANSPARENT};
use crate::context::RenderContext;
use crate::presentation::{control_height_rem, control_space_x_rem, rem_to_px, size_font_rem};

pub type TabHandler = Arc<dyn Fn(&str) + Send + Sync>;
/// Complete next tab-value order. The host retains this owned result and
/// rebuilds; it does not clone from a borrowed slice.
pub type TabOrderHandler = Arc<dyn Fn(Vec<String>) + Send + Sync>;
pub type TabDropTargetHandler = Arc<dyn Fn(Option<&str>) + Send + Sync>;

#[derive(Clone, Default)]
pub struct TabsHandlers {
    pub on_change: Option<TabHandler>,
    pub on_close: Option<TabHandler>,
    pub on_focus: Option<TabHandler>,
    pub on_reorder: Option<TabOrderHandler>,
    pub on_drag_start: Option<TabHandler>,
    pub on_drag_end: Option<TabHandler>,
    pub on_drop_target_change: Option<TabDropTargetHandler>,
    pub focused_value: Option<String>,
    /// Stable native instance scope. Semantic ids remain readable, but two
    /// tabsets with the same values must never share backend focus handles.
    pub instance_id: Option<String>,
    pub has_panel: bool,
}

/// Per-instance drag scope. Two mounted tab sets with the same values must
/// not mint one source or target id — the controller treats a duplicate live
/// id as an error rather than last-writer-wins.
fn drag_scope(handlers: &TabsHandlers) -> String {
    handlers
        .instance_id
        .as_deref()
        .map(|scope| format!("tabs:{scope}"))
        .unwrap_or_else(|| "tabs".to_string())
}

fn tab_list_runtime_id(handlers: &TabsHandlers) -> Option<String> {
    handlers
        .instance_id
        .as_ref()
        .map(|scope| format!("tabs:{scope}:list"))
}

fn tab_runtime_id(handlers: &TabsHandlers, value: &str) -> String {
    handlers
        .instance_id
        .as_ref()
        .map(|scope| format!("tabs:{scope}:tab:{value}"))
        .unwrap_or_else(|| format!("tabs:{value}"))
}

fn tab_panel_runtime_id(handlers: &TabsHandlers, value: &str) -> Option<String> {
    handlers
        .instance_id
        .as_ref()
        .map(|scope| format!("tabs:{scope}:panel:{value}"))
}

fn tabs_items(spec: &TabsSpec) -> Vec<poodle_headless::tabs::TabsItem> {
    spec.tabs
        .iter()
        .map(|item| poodle_headless::tabs::TabsItem {
            value: item.value.clone(),
            disabled: item.is_disabled,
            closable: item.is_closable,
        })
        .collect()
}

fn tabs_activation(spec: &TabsSpec) -> poodle_headless::tabs::ActivationMode {
    match spec.activation_mode {
        TabActivationMode::Manual => poodle_headless::tabs::ActivationMode::Manual,
        TabActivationMode::Automatic => poodle_headless::tabs::ActivationMode::Automatic,
    }
}

fn apply_tab_effects(
    effects: Vec<poodle_headless::tabs::TabsEffect>,
    items: &[poodle_headless::tabs::TabsItem],
    instance_id: Option<&str>,
    on_change: Option<&TabHandler>,
    on_close: Option<&TabHandler>,
    on_reorder: Option<&TabOrderHandler>,
    on_focus: Option<&TabHandler>,
    notify_focus: bool,
) -> Option<String> {
    let mut focus_target = None;
    for effect in effects {
        match effect {
            poodle_headless::tabs::TabsEffect::FocusTab { index } => {
                if let Some(item) = items.get(index) {
                    focus_target = Some(
                        instance_id
                            .map(|scope| format!("tabs:{scope}:tab:{}", item.value))
                            .unwrap_or_else(|| format!("tabs:{}", item.value)),
                    );
                    if notify_focus {
                        if let Some(handler) = on_focus {
                            handler(&item.value);
                        }
                    }
                }
            }
            poodle_headless::tabs::TabsEffect::EmitValueChange { value } => {
                if let Some(handler) = on_change {
                    handler(&value);
                }
            }
            poodle_headless::tabs::TabsEffect::EmitClose { value } => {
                if let Some(handler) = on_close {
                    handler(&value);
                }
            }
            poodle_headless::tabs::TabsEffect::EmitReorder { order } => {
                if let Some(handler) = on_reorder {
                    handler(order);
                }
            }
        }
    }
    focus_target
}

fn rounded_all(node: &mut Node, r: f32) {
    let c = &mut node.style.descriptor.corner_radii;
    c.top_left = r;
    c.top_right = r;
    c.bottom_right = r;
    c.bottom_left = r;
}

/// The selection edge (contract §8 `activeEdge`): `Outline` draws a 1px accent
/// border around the active tab — `mix_srgb(accent, border-subtle, 0.32)`,
/// the former card selected-border value; `Underline` draws a 2px accent
/// border along the inline-end side — bottom horizontal, right vertical —
/// the former strip variant's indicator. Both keep a transparent reserve
/// border on every tab so selection never shifts layout. The edge axis is an
/// enum, so exactly one of these can apply. Applied after separator borders
/// (which use per-side color overrides), so block separators survive.
fn apply_active_edge(
    node: &mut Node,
    is_active: bool,
    vertical: bool,
    spec: &TabsSpec,
    ctx: &RenderContext<'_>,
) {
    match spec.active_edge {
        ActiveEdge::None => {}
        ActiveEdge::Outline => {
            let accent = ctx.theme().resolve_color(spec.indicator_token());
            let border = ctx.theme().resolve_color(spec.list_border_token());
            let selected = mix_srgb(accent, border, 0.32);
            node.style.descriptor.border.width = 1.0;
            node.style.descriptor.border.color = if is_active { selected } else { TRANSPARENT };
        }
        ActiveEdge::Underline => {
            let accent = ctx.theme().resolve_color(spec.indicator_token());
            let edge = if is_active { accent } else { TRANSPARENT };
            if vertical {
                node.style.border_right_width = Some(rem_to_px(0.125));
                node.style.descriptor.border.color = edge;
            } else {
                node.style.border_bottom_width = Some(rem_to_px(0.125));
                node.style.border_color_bottom = Some(edge);
            }
        }
    }
}

/// Icon + label + count badge, the anatomy shared by all variants.
fn build_tab_label(
    tab: &poodle_specs::TabDefinition,
    ctx: &RenderContext<'_>,
    text_color: ColorValue,
    font_size: f32,
    icon_only: bool,
) -> Node {
    let has_icon = tab.icon.is_some();
    let has_count = tab.count.is_some();

    // Vertical/icon-only: icon alone, label fallback so the tab is never empty.
    if icon_only {
        if let Some(ref icon_name) = tab.icon {
            let mut i = Node::icon(
                icon_name.as_str(),
                ctx.theme().resolve_space("size.icon.sm"),
            );
            i.style.descriptor.text_color = Some(text_color);
            return i;
        }
        let mut l = Node::text(&tab.label);
        l.style.text_size = Some(font_size);
        l.style.descriptor.text_color = Some(text_color);
        return l;
    }

    if !has_icon && !has_count {
        let mut l = Node::text(&tab.label);
        l.style.text_size = Some(font_size);
        l.style.descriptor.text_color = Some(text_color);
        return l;
    }

    let gap = ctx.theme().resolve_space("space.inline.sm");
    let mut row = Node::container();
    {
        let s = &mut row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = gap;
    }

    if let Some(ref icon_name) = tab.icon {
        let mut i = Node::icon(
            icon_name.as_str(),
            ctx.theme().resolve_space("size.icon.sm"),
        );
        i.style.descriptor.text_color = Some(text_color);
        row = row.child(i);
    }

    let mut l = Node::text(&tab.label);
    l.style.text_size = Some(font_size);
    l.style.descriptor.text_color = Some(text_color);
    row = row.child(l);

    if let Some(count) = tab.count {
        let caption_size = ctx.theme().resolve_space("typography.caption.size");
        let surface = ctx.theme().resolve_color("color.background.surface");
        let badge_bg = mix_srgb(text_color, surface, 0.14);
        let mut badge = Node::text(format!("{count}"));
        {
            let s = &mut badge.style;
            s.text_size = Some(caption_size);
            s.text_weight = Some(600);
            s.descriptor.text_color = Some(text_color);
            s.descriptor.background = Some(badge_bg);
            s.descriptor.layout.spacing.padding.left = rem_to_px(0.3125);
            s.descriptor.layout.spacing.padding.right = rem_to_px(0.3125);
            s.min_width = Some(rem_to_px(1.125));
        }
        rounded_all(&mut badge, rem_to_px(0.5625));
        row = row.child(badge);
    }

    row
}

/// The card variant's close button; interaction wires through `on_close`.
fn build_close_button(ctx: &RenderContext<'_>, tab_label: &str) -> Node {
    let icon_color = ctx.theme().resolve_color("color.icon.muted");
    let icon_size = ctx.theme().resolve_space("size.icon.sm");
    let mut btn = Node::button("");
    btn.a11y.label = Some(format!("Close {tab_label}"));
    {
        let s = &mut btn.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.descriptor.cursor = CursorHint::Pointer;
    }
    btn.interaction.focusable = true;
    let mut x = Node::icon("x", icon_size);
    x.style.descriptor.text_color = Some(icon_color);
    btn.child(x)
}

/// Transient reorder-drag visuals: source dims, target rings.
fn apply_drag_state(
    node: &mut Node,
    tab_value: &str,
    spec: &TabsSpec,
    handlers: &TabsHandlers,
    ctx: &RenderContext<'_>,
) {
    node.id = Some(format!("tabs:{tab_value}"));
    node.runtime_id = handlers
        .instance_id
        .as_ref()
        .map(|_| tab_runtime_id(handlers, tab_value));
    if spec.is_drag_value(tab_value) {
        node.style.descriptor.opacity = 0.4;
    }
    if spec.is_drop_target(tab_value) {
        let accent = ctx.theme().resolve_color("color.accent.base");
        rounded_all(node, ctx.theme().resolve_radius("radius.control"));
        node.style.shadow_layers = vec![ShadowLayer {
            offset_x: 0.0,
            offset_y: 0.0,
            blur: 0.0,
            spread: rem_to_px(0.125),
            color: accent,
            inset: true,
        }];
    }
}

pub fn tabs(
    spec: &TabsSpec,
    ctx: &RenderContext<'_>,
    on_change: Option<TabHandler>,
    on_close: Option<TabHandler>,
) -> Node {
    tabs_with_handlers(
        spec,
        ctx,
        TabsHandlers {
            on_change,
            on_close,
            ..TabsHandlers::default()
        },
    )
}

pub fn tabs_with_handlers(
    spec: &TabsSpec,
    ctx: &RenderContext<'_>,
    handlers: TabsHandlers,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let tab_bar = match spec.variant {
        TabVariant::Card => render_card(spec, ctx, &handlers),
        TabVariant::Pill => render_pill(spec, ctx, &handlers),
        TabVariant::Block => render_block(spec, ctx, &handlers),
    };

    let mut root = Node::container();
    root.style.descriptor.layout.direction = LayoutDirection::Column;
    root.roles.insert(
        "variant".to_owned(),
        format!("{:?}", spec.variant).to_ascii_lowercase(),
    );
    root.roles.insert(
        "orientation".to_owned(),
        format!("{:?}", spec.orientation).to_ascii_lowercase(),
    );
    root.roles.insert(
        "size".to_owned(),
        format!("{effective_size:?}").to_ascii_lowercase(),
    );
    root.roles.insert(
        "density".to_owned(),
        format!("{density:?}").to_ascii_lowercase(),
    );
    let root = root.child(tab_bar);
    root
}

pub fn tabs_with_panel(
    spec: &TabsSpec,
    ctx: &RenderContext<'_>,
    mut handlers: TabsHandlers,
    panel: Node,
) -> Node {
    handlers.has_panel = true;
    let Some(value) = spec.current_value() else {
        return tabs_with_handlers(spec, ctx, handlers);
    };
    let panel_runtime_id = tab_panel_runtime_id(&handlers, value);
    let mut root = tabs_with_handlers(spec, ctx, handlers);
    let mut panel = panel;
    panel.id = Some(format!("tabs-panel:{value}"));
    panel.runtime_id = panel_runtime_id;
    panel.a11y.role = Some(NodeRole::TabPanel);
    panel.a11y.labelled_by = Some(format!("tabs:{value}"));
    panel.interaction.focusable = true;
    panel.a11y.tab_index = Some(0);
    root.children.push(panel);
    root
}

fn wire_select(node: &mut Node, is_disabled: bool, value: &str, on_change: Option<&TabHandler>) {
    if let (false, Some(handler)) = (is_disabled, on_change) {
        let handler = Arc::clone(handler);
        let value = value.to_string();
        node.interaction.on_activate = Some(Arc::new(move || handler(&value)));
    }
}

fn wire_reorder(node: &mut Node, spec: &TabsSpec, index: usize, handlers: &TabsHandlers) {
    let tab = &spec.tabs[index];
    if !spec.is_reorderable || tab.is_disabled {
        return;
    }
    node.style.descriptor.cursor = CursorHint::Grab;
    let scope = drag_scope(handlers);
    // The semantic family may be shared by an owning composite; the
    // registration namespace never is.
    let kind = spec
        .drag_subject_kind
        .clone()
        .unwrap_or_else(|| crate::drag_drop::reorder_kind(&scope));
    let owned: Vec<String> = spec.tabs.iter().map(|tab| tab.value.clone()).collect();
    let value = tab.value.clone();

    // Source. `on_drag_start` and `on_drag_end` keep their `Fn(&str)` shape:
    // the tab value IS the subject id, so the row's own value answers both
    // without the terminal outcome having to carry a subject back.
    let mut source = crate::drag_drop::reorder_source_in_family(&scope, &kind, &value, &tab.label);
    if let Some(handler) = &handlers.on_drag_start {
        let handler = Arc::clone(handler);
        source.on_drag_start = Some(Arc::new(move |session| handler(&session.subject.id)));
    }
    if let Some(handler) = &handlers.on_drag_end {
        let handler = Arc::clone(handler);
        let value = value.clone();
        source.on_drag_end = Some(Arc::new(move |_outcome| handler(&value)));
    }
    crate::drag_drop::attach_source(node, true, source);

    // Target. A tab bar reorders along its main axis, so the band rule reads
    // the horizontal fraction; the drop itself reorders onto this tab's index
    // and does not branch on the band. The indicator paints the whole tab, so
    // a drop on a sibling lands at that sibling.
    let mut target =
        crate::drag_drop::reorder_target_in_family(&scope, &kind, &value, &tab.label, owned);
    target.resolve_position = Some(if spec.orientation == Orientation::Vertical {
        crate::drag_drop::vertical_band_resolver(false)
    } else {
        crate::drag_drop::horizontal_band_resolver(false)
    });
    if let Some(handler) = &handlers.on_drop_target_change {
        let hover = Arc::clone(handler);
        let value = value.clone();
        target.on_intent = Some(Arc::new(move |_event| hover(Some(&value))));
        let leave = Arc::clone(handler);
        target.on_intent_cleared = Some(Arc::new(move || leave(None)));
    }
    let items = tabs_items(spec);
    let reorderable = spec.is_reorderable;
    let on_reorder = handlers.on_reorder.clone();
    let on_focus = handlers.on_focus.clone();
    let instance_id = handlers.instance_id.clone();
    let to_index = index;
    target.on_drop = Some(Arc::new(move |event| {
        let Some(from_index) = items.iter().position(|item| item.value == event.subject.id) else {
            return NodeDropCommit::Rejected {
                reason: Some("The dragged tab is no longer in this tab set".to_string()),
            };
        };
        let context = poodle_headless::tabs::TabsContext {
            items: items.clone(),
            value: None,
            focus_index: from_index,
            activation_mode: poodle_headless::tabs::ActivationMode::Automatic,
            reorderable,
        };
        let (next, effects) = poodle_headless::tabs::tabs_transition(
            context,
            poodle_headless::tabs::TabsEvent::Reorder {
                from_index,
                to_index,
            },
        );
        apply_tab_effects(
            effects,
            &next.items,
            instance_id.as_deref(),
            None,
            None,
            on_reorder.as_ref(),
            on_focus.as_ref(),
            true,
        );
        NodeDropCommit::Committed
    }));
    crate::drag_drop::attach_target(node, true, target);
}

fn wire_collection_semantics(
    node: &mut Node,
    spec: &TabsSpec,
    index: usize,
    handlers: &TabsHandlers,
    ctx: &RenderContext<'_>,
) {
    let tab = &spec.tabs[index];
    let is_selected = spec.current_value() == Some(tab.value.as_str());
    node.a11y.role = Some(NodeRole::Tab);
    node.a11y.label = Some(tab.label.clone());
    let focused_value = handlers
        .focused_value
        .as_deref()
        .or_else(|| spec.current_value());
    node.interaction.disabled = tab.is_disabled;
    node.interaction.focusable = !tab.is_disabled;
    if !tab.is_disabled {
        node.style.focus = Some(StylePatch {
            border_color: Some(ctx.theme().resolve_color(spec.focus_ring_color_token())),
            ..StylePatch::default()
        });
    }
    node.a11y.tab_index = Some(
        if !tab.is_disabled && focused_value == Some(tab.value.as_str()) {
            0
        } else {
            -1
        },
    );
    node.a11y.selected = Some(is_selected);
    if !tab.is_disabled {
        if let Some(handler) = handlers.on_focus.as_ref() {
            let handler = Arc::clone(handler);
            let value = tab.value.clone();
            node.interaction.on_focus_change = Some(Arc::new(move |focused| {
                if focused {
                    handler(&value);
                }
            }));
        }
    }
    if handlers.has_panel {
        node.a11y.controls = Some(format!("tabs-panel:{}", tab.value));
    }
    if tab.is_disabled {
        return;
    }

    let items = tabs_items(spec);
    let value = spec.current_value().map(str::to_owned);
    let activation_mode = tabs_activation(spec);
    let orientation = spec.orientation;
    let reorderable = spec.is_reorderable;
    let on_change = handlers.on_change.clone();
    let on_close = handlers.on_close.clone();
    let on_reorder = handlers.on_reorder.clone();
    let instance_id = handlers.instance_id.clone();
    let tab_value = tab.value.clone();
    node.interaction.on_key = Some(Arc::new(move |key, modifiers| {
        let context = poodle_headless::tabs::TabsContext {
            items: items.clone(),
            value: value.clone(),
            focus_index: index,
            activation_mode,
            reorderable,
        };
        let event = if key == NodeKey::Delete {
            poodle_headless::tabs::TabsEvent::Close {
                value: tab_value.clone(),
            }
        } else if reorderable && modifiers.alt {
            let direction = match (orientation, key) {
                (Orientation::Horizontal, NodeKey::ArrowRight)
                | (Orientation::Vertical, NodeKey::ArrowDown) => Some(1),
                (Orientation::Horizontal, NodeKey::ArrowLeft)
                | (Orientation::Vertical, NodeKey::ArrowUp) => Some(-1),
                _ => None,
            };
            let Some(direction) = direction else {
                return None;
            };
            poodle_headless::tabs::TabsEvent::ReorderStep {
                direction,
                from_index: Some(index),
            }
        } else {
            let direction = match (orientation, key) {
                (Orientation::Horizontal, NodeKey::ArrowRight)
                | (Orientation::Vertical, NodeKey::ArrowDown) => {
                    Some(poodle_headless::tabs::FocusDirection::Next)
                }
                (Orientation::Horizontal, NodeKey::ArrowLeft)
                | (Orientation::Vertical, NodeKey::ArrowUp) => {
                    Some(poodle_headless::tabs::FocusDirection::Prev)
                }
                (_, NodeKey::Home) => Some(poodle_headless::tabs::FocusDirection::First),
                (_, NodeKey::End) => Some(poodle_headless::tabs::FocusDirection::Last),
                _ => None,
            };
            let Some(direction) = direction else {
                return None;
            };
            poodle_headless::tabs::TabsEvent::FocusMove {
                direction,
                from_index: Some(index),
            }
        };
        let (next, effects) = poodle_headless::tabs::tabs_transition(context, event);
        apply_tab_effects(
            effects,
            &next.items,
            instance_id.as_deref(),
            on_change.as_ref(),
            on_close.as_ref(),
            on_reorder.as_ref(),
            None,
            false,
        )
    }));
}

fn render_card(spec: &TabsSpec, ctx: &RenderContext<'_>, handlers: &TabsHandlers) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(density));
    let control_y = ctx.theme().resolve_space("space.control.y");

    let accent = ctx.theme().resolve_color(spec.indicator_token());
    let border = ctx.theme().resolve_color(spec.list_border_token());
    let text_primary = ctx.theme().resolve_color("color.text.primary");
    let text_secondary = ctx.theme().resolve_color("color.text.secondary");
    let text_inverse = ctx.theme().resolve_color("color.text.inverse");
    let disabled_opacity = ctx.theme().resolve_opacity(spec.disabled_opacity_token());
    let radius = ctx.theme().resolve_radius("radius.control");

    // activeEdge: outline/underline borders on the selected tab, with a
    // transparent reserve border on every tab so the bar never shifts when
    // the selected border becomes visible (see `apply_active_edge`).
    let selected = spec.current_value().map(|s| s.to_string());
    let vertical = spec.is_vertical();
    let full_width = spec.uses_full_width();
    let solid = spec.active_fill == ActiveFill::Solid;

    let mut tab_bar = Node::container();
    tab_bar.id = Some("tabs-list".to_owned());
    tab_bar.runtime_id = tab_list_runtime_id(handlers);
    tab_bar.a11y.role = Some(NodeRole::TabList);
    tab_bar.a11y.label = spec.aria_label.clone();
    tab_bar.a11y.orientation = Some(format!("{:?}", spec.orientation).to_ascii_lowercase());
    {
        let s = &mut tab_bar.style;
        if vertical {
            s.descriptor.layout.direction = LayoutDirection::Column;
            if spec.is_bordered {
                s.border_right_width = Some(1.0);
                s.descriptor.border.color = border;
                s.descriptor.layout.spacing.padding.right = rem_to_px(0.5);
            }
        } else {
            s.descriptor.layout.direction = LayoutDirection::Row;
            if spec.is_bordered {
                s.border_bottom_width = Some(1.0);
                s.descriptor.border.color = border;
            }
            if full_width {
                s.fill_width = true;
            }
        }
    }

    for tab in &spec.tabs {
        let is_active = selected.as_deref() == Some(tab.value.as_str());
        let is_disabled = tab.is_disabled;
        let text_color = if is_active && solid {
            text_inverse
        } else if is_active {
            text_primary
        } else {
            text_secondary
        };

        let mut tab_el = Node::container();
        tab_el.a11y.role = Some(NodeRole::Tab);
        tab_el.a11y.label = Some(tab.label.clone());
        tab_el.a11y.selected = Some(is_active);
        {
            let s = &mut tab_el.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.padding.left = pad_x;
            s.descriptor.layout.spacing.padding.right = pad_x;
            s.descriptor.layout.spacing.padding.top = control_y;
            s.descriptor.layout.spacing.padding.bottom = control_y;
            s.text_size = Some(font_size);
            s.text_weight = Some(600);
            s.descriptor.text_color = Some(text_color);
            s.descriptor.cursor = CursorHint::Pointer;
            if full_width {
                s.flex_fill = true;
                s.fill_width = true;
                s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            }
            if is_active && spec.active_fill != ActiveFill::None {
                s.descriptor.background = if solid {
                    Some(accent)
                } else {
                    Some(with_alpha(accent, accent.3 * 0.18))
                };
            }
            if is_disabled {
                s.descriptor.opacity = disabled_opacity;
            }
        }
        apply_active_edge(&mut tab_el, is_active, vertical, spec, ctx);
        rounded_all(&mut tab_el, radius);
        tab_el.interaction.focusable = true;
        let mut tab_el = tab_el.child(build_tab_label(tab, ctx, text_color, font_size, vertical));

        if tab.is_closable {
            let mut close = build_close_button(ctx, &tab.label);
            close.id = Some(format!("tabs-close:{}", tab.value));
            close.runtime_id = handlers
                .instance_id
                .as_ref()
                .map(|scope| format!("tabs:{scope}:close:{}", tab.value));
            close.interaction.on_activate = Some(match (is_disabled, handlers.on_close.as_ref()) {
                (false, Some(handler)) => {
                    let handler = Arc::clone(handler);
                    let value = tab.value.clone();
                    Arc::new(move || handler(&value))
                }
                // Inert but still the nearest clickable: an unwired X would
                // bubble to the tab and select what it was closing.
                _ => Arc::new(|| {}),
            });
            tab_el = tab_el.child(close);
        }

        apply_drag_state(&mut tab_el, tab.value.as_str(), spec, handlers, ctx);
        wire_reorder(&mut tab_el, spec, tab_bar.children.len(), handlers);
        wire_select(
            &mut tab_el,
            is_disabled,
            &tab.value,
            handlers.on_change.as_ref(),
        );
        wire_collection_semantics(&mut tab_el, spec, tab_bar.children.len(), handlers, ctx);
        tab_bar = tab_bar.child(tab_el);
    }
    tab_bar
}

fn render_pill(spec: &TabsSpec, ctx: &RenderContext<'_>, handlers: &TabsHandlers) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let control_height = rem_to_px(control_height_rem(effective_size));
    let tab_height = control_height - rem_to_px(0.5);
    let pad_x = rem_to_px(control_space_x_rem(density));

    let accent = ctx.theme().resolve_color(spec.indicator_token());
    let border_subtle = ctx.theme().resolve_color(spec.list_border_token());
    let text_primary = ctx.theme().resolve_color("color.text.primary");
    let text_secondary = ctx.theme().resolve_color("color.text.secondary");
    let disabled_opacity = ctx.theme().resolve_opacity(spec.disabled_opacity_token());
    let pill_radius = ctx.theme().resolve_radius("radius.pill");

    let container_border = with_alpha(border_subtle, border_subtle.3 * spec.pill_border_opacity());
    let active_bg = with_alpha(accent, accent.3 * spec.pill_active_bg_opacity());

    let selected = spec.current_value().map(|s| s.to_string());

    let mut container = Node::container();
    container.id = Some("tabs-list".to_owned());
    container.runtime_id = tab_list_runtime_id(handlers);
    container.a11y.role = Some(NodeRole::TabList);
    container.a11y.label = spec.aria_label.clone();
    container.a11y.orientation = Some(format!("{:?}", spec.orientation).to_ascii_lowercase());
    {
        let s = &mut container.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = rem_to_px(0.125);
        s.descriptor.border.width = 2.0;
        s.descriptor.border.color = container_border;
        let p = rem_to_px(0.1875);
        s.descriptor.layout.spacing.padding.left = p;
        s.descriptor.layout.spacing.padding.right = p;
        s.descriptor.layout.spacing.padding.top = p;
        s.descriptor.layout.spacing.padding.bottom = p;
    }
    rounded_all(&mut container, pill_radius);

    for tab in &spec.tabs {
        let is_active = selected.as_deref() == Some(tab.value.as_str());
        let is_disabled = tab.is_disabled;
        let text_color = if is_active {
            text_primary
        } else {
            text_secondary
        };

        let mut tab_el = Node::container();
        {
            let s = &mut tab_el.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.padding.left = pad_x;
            s.descriptor.layout.spacing.padding.right = pad_x;
            s.descriptor.layout.height = LayoutSizing::Fixed(tab_height);
            s.text_size = Some(font_size);
            s.text_weight = Some(600);
            s.descriptor.text_color = Some(text_color);
            s.descriptor.cursor = CursorHint::Pointer;
            if is_active && spec.active_fill != ActiveFill::None {
                s.descriptor.background = Some(active_bg);
            }
            if is_disabled {
                s.descriptor.opacity = disabled_opacity;
            }
        }
        apply_active_edge(&mut tab_el, is_active, false, spec, ctx);
        rounded_all(&mut tab_el, pill_radius);
        tab_el.interaction.focusable = true;
        // Pill is always horizontal.
        let mut tab_el = tab_el.child(build_tab_label(tab, ctx, text_color, font_size, false));

        apply_drag_state(&mut tab_el, tab.value.as_str(), spec, handlers, ctx);
        wire_reorder(&mut tab_el, spec, container.children.len(), handlers);
        wire_select(
            &mut tab_el,
            is_disabled,
            &tab.value,
            handlers.on_change.as_ref(),
        );
        wire_collection_semantics(&mut tab_el, spec, container.children.len(), handlers, ctx);
        container = container.child(tab_el);
    }
    container
}

fn render_block(spec: &TabsSpec, ctx: &RenderContext<'_>, handlers: &TabsHandlers) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let control_height = rem_to_px(control_height_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(density));

    let accent = ctx.theme().resolve_color(spec.indicator_token());
    let border = ctx.theme().resolve_color(spec.list_border_token());
    let text_primary = ctx.theme().resolve_color("color.text.primary");
    let text_secondary = ctx.theme().resolve_color("color.text.secondary");
    let surface_bg = ctx.theme().resolve_color("color.background.surface");
    let panel_bg = ctx.theme().resolve_color("color.background.panel");
    let disabled_opacity = ctx.theme().resolve_opacity(spec.disabled_opacity_token());

    let list_bg = with_alpha(panel_bg, panel_bg.3 * spec.block_list_bg_opacity());
    let separator = with_alpha(border, border.3 * spec.block_separator_opacity());
    let selected_bg = mix_srgb(accent, surface_bg, spec.block_selected_accent_mix());

    let selected = spec.current_value().map(|s| s.to_string());
    let vertical = spec.is_vertical();

    let mut tab_bar = Node::container();
    tab_bar.id = Some("tabs-list".to_owned());
    tab_bar.runtime_id = tab_list_runtime_id(handlers);
    tab_bar.a11y.role = Some(NodeRole::TabList);
    tab_bar.a11y.label = spec.aria_label.clone();
    tab_bar.a11y.orientation = Some(format!("{:?}", spec.orientation).to_ascii_lowercase());
    {
        let s = &mut tab_bar.style;
        s.descriptor.background = Some(list_bg);
        s.descriptor.border.color = border;
        if vertical {
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.border_right_width = Some(1.0);
        } else {
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.fill_width = true;
            s.border_bottom_width = Some(1.0);
        }
    }

    for (idx, tab) in spec.tabs.iter().enumerate() {
        let is_active = selected.as_deref() == Some(tab.value.as_str());
        let is_disabled = tab.is_disabled;
        let text_color = if is_active {
            text_primary
        } else {
            text_secondary
        };

        let mut tab_el = Node::container();
        {
            let s = &mut tab_el.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            s.descriptor.layout.spacing.padding.left = pad_x;
            s.descriptor.layout.spacing.padding.right = pad_x;
            s.descriptor.layout.height = LayoutSizing::Fixed(control_height);
            s.text_size = Some(font_size);
            s.text_weight = Some(600);
            s.descriptor.text_color = Some(text_color);
            s.descriptor.cursor = CursorHint::Pointer;
            if vertical {
                s.fill_width = true;
            } else if spec.uses_full_width() {
                s.flex_fill = true;
                s.fill_width = true;
            }
            // Sibling separator: left border (horizontal) / top border
            // (vertical). Per-side color overrides, so the selection edge
            // (which owns `descriptor.border.color`) does not clobber them.
            if idx > 0 {
                if vertical {
                    s.border_top_width = Some(1.0);
                    s.border_color_top = Some(separator);
                } else {
                    s.border_left_width = Some(1.0);
                    s.border_color_left = Some(separator);
                }
            }
            if is_active && spec.active_fill != ActiveFill::None {
                s.descriptor.background = Some(selected_bg);
            }
            if is_disabled {
                s.descriptor.opacity = disabled_opacity;
            }
        }
        apply_active_edge(&mut tab_el, is_active, vertical, spec, ctx);
        tab_el.interaction.focusable = true;
        let mut tab_el = tab_el.child(build_tab_label(tab, ctx, text_color, font_size, vertical));

        apply_drag_state(&mut tab_el, tab.value.as_str(), spec, handlers, ctx);
        wire_reorder(&mut tab_el, spec, tab_bar.children.len(), handlers);
        wire_select(
            &mut tab_el,
            is_disabled,
            &tab.value,
            handlers.on_change.as_ref(),
        );
        wire_collection_semantics(&mut tab_el, spec, tab_bar.children.len(), handlers, ctx);
        tab_bar = tab_bar.child(tab_el);
    }
    tab_bar
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_adapter::ThemeProvider;
    use poodle_specs::{ActiveEdge, ActiveFill, TabDefinition};
    use std::sync::Mutex;

    /// The real token resolver over the ECLIPSE theme. Pure — no backend.
    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    /// The tab element for `value` — `apply_drag_state` tags it `tabs:{value}`.
    fn tab_of<'a>(root: &'a Node, value: &str) -> &'a Node {
        root.find(&|n| n.id.as_deref() == Some(&format!("tabs:{value}")))
            .unwrap_or_else(|| panic!("tab {value} exists"))
    }

    #[test]
    fn card_renderer_renders_icon_count_and_close_wired_to_on_close() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let closed = Arc::new(Mutex::new(Vec::new()));
        let on_close: TabHandler = {
            let closed = Arc::clone(&closed);
            Arc::new(move |value: &str| closed.lock().unwrap().push(value.to_string()))
        };
        let spec = TabsSpec::new(vec![
            TabDefinition::new("index.ts", "index.ts").with_icon("file"),
            TabDefinition::new("App.svelte", "App.svelte")
                .with_count(3)
                .with_closable(true),
        ])
        .with_variant(TabVariant::Card)
        .with_value("index.ts");

        let root = tabs(&spec, &ctx, None, Some(on_close));

        assert!(
            root.find(
                &|n| matches!(&n.kind, poodle_node::NodeKind::Icon { name, .. } if name == "file")
            )
            .is_some(),
            "the card renderer draws the item icon"
        );
        assert!(
            root.find(
                &|n| matches!(&n.kind, poodle_node::NodeKind::Text { content } if content == "3")
            )
            .is_some(),
            "the card renderer draws the count badge"
        );

        let close = root
            .find(&|n| n.a11y.label.as_deref() == Some("Close App.svelte"))
            .unwrap_or_else(|| panic!("the closable tab renders a close button"));
        close.interaction.on_activate.as_ref().unwrap()();
        assert_eq!(*closed.lock().unwrap(), vec!["App.svelte"]);
    }

    #[test]
    fn card_renderer_solid_fill_uses_accent_with_inverse_foreground() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = TabsSpec::new(vec![
            TabDefinition::new("a", "A"),
            TabDefinition::new("b", "B"),
        ])
        .with_variant(TabVariant::Card)
        .with_active_fill(ActiveFill::Solid)
        .with_value("a");

        let root = tabs(&spec, &ctx, None, None);
        let accent = theme.resolve_color(spec.indicator_token());
        let inverse = theme.resolve_color("color.text.inverse");

        let active = tab_of(&root, "a");
        assert_eq!(active.style.descriptor.background, Some(accent));
        assert_eq!(active.style.descriptor.text_color, Some(inverse));

        let inactive = tab_of(&root, "b");
        assert_eq!(inactive.style.descriptor.background, None);
        assert_eq!(
            inactive.style.descriptor.text_color,
            Some(theme.resolve_color("color.text.secondary"))
        );
    }

    #[test]
    fn card_renderer_active_edge_outline_borders_only_the_selected_tab() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = TabsSpec::new(vec![
            TabDefinition::new("a", "A"),
            TabDefinition::new("b", "B"),
        ])
        .with_variant(TabVariant::Card)
        .with_active_edge(ActiveEdge::Outline)
        .with_value("a");

        let root = tabs(&spec, &ctx, None, None);
        let accent = theme.resolve_color(spec.indicator_token());
        let border = theme.resolve_color(spec.list_border_token());
        let expected = mix_srgb(accent, border, 0.32);

        let active = tab_of(&root, "a");
        assert_eq!(active.style.descriptor.border.width, 1.0);
        assert_eq!(active.style.descriptor.border.color, expected);

        // Unselected tabs keep a transparent border so the outline never
        // shifts the bar when selection moves.
        let inactive = tab_of(&root, "b");
        assert_eq!(inactive.style.descriptor.border.width, 1.0);
        assert_eq!(
            inactive.style.descriptor.border.color,
            ColorValue(0.0, 0.0, 0.0, 0.0)
        );
    }

    #[test]
    fn card_renderer_does_not_draw_outline_by_default() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = TabsSpec::new(vec![
            TabDefinition::new("a", "A"),
            TabDefinition::new("b", "B"),
        ])
        .with_variant(TabVariant::Card)
        .with_value("a");

        let root = tabs(&spec, &ctx, None, None);
        assert_eq!(tab_of(&root, "a").style.descriptor.border.width, 0.0);
        assert_eq!(tab_of(&root, "b").style.descriptor.border.width, 0.0);
    }

    #[test]
    fn block_renderer_underline_edges_only_the_selected_tab() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = TabsSpec::new(vec![
            TabDefinition::new("a", "A"),
            TabDefinition::new("b", "B"),
        ])
        .with_variant(TabVariant::Block)
        .with_active_edge(ActiveEdge::Underline)
        .with_value("a");

        let root = tabs(&spec, &ctx, None, None);
        let accent = theme.resolve_color(spec.indicator_token());

        let active = tab_of(&root, "a");
        assert_eq!(active.style.border_bottom_width, Some(rem_to_px(0.125)));
        assert_eq!(active.style.border_color_bottom, Some(accent));

        // Unselected tabs keep a transparent reserve edge so the underline
        // never shifts the bar when selection moves.
        let inactive = tab_of(&root, "b");
        assert_eq!(inactive.style.border_bottom_width, Some(rem_to_px(0.125)));
        assert_eq!(inactive.style.border_color_bottom, Some(TRANSPARENT));
    }

    #[test]
    fn block_renderer_vertical_underline_uses_the_inline_end_edge() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = TabsSpec::new(vec![
            TabDefinition::new("a", "A"),
            TabDefinition::new("b", "B"),
        ])
        .with_variant(TabVariant::Block)
        .with_orientation(poodle_specs::Orientation::Vertical)
        .with_active_edge(ActiveEdge::Underline)
        .with_value("a");

        let root = tabs(&spec, &ctx, None, None);
        let accent = theme.resolve_color(spec.indicator_token());

        let active = tab_of(&root, "a");
        assert_eq!(active.style.border_right_width, Some(rem_to_px(0.125)));
        assert_eq!(active.style.descriptor.border.color, accent);
    }

    #[test]
    fn block_renderer_keeps_separators_under_outline() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = TabsSpec::new(vec![
            TabDefinition::new("a", "A"),
            TabDefinition::new("b", "B"),
        ])
        .with_variant(TabVariant::Block)
        .with_active_edge(ActiveEdge::Outline)
        .with_value("a");

        let root = tabs(&spec, &ctx, None, None);
        let separator = with_alpha(
            theme.resolve_color("color.border.subtle"),
            theme.resolve_color("color.border.subtle").3 * 0.72,
        );

        // The second item's left separator survives the outline: per-side
        // color override wins over the uniform outline border.
        let second = tab_of(&root, "b");
        assert_eq!(second.style.border_left_width, Some(1.0));
        assert_eq!(second.style.border_color_left, Some(separator));
        // The outline still applies to the remaining sides.
        assert_eq!(second.style.descriptor.border.width, 1.0);
        assert_eq!(second.style.descriptor.border.color, TRANSPARENT);
    }

    #[test]
    fn none_fill_suppresses_selected_background_on_every_variant() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let text_primary = theme.resolve_color("color.text.primary");
        for variant in [TabVariant::Card, TabVariant::Pill, TabVariant::Block] {
            let spec = TabsSpec::new(vec![
                TabDefinition::new("a", "A"),
                TabDefinition::new("b", "B"),
            ])
            .with_variant(variant)
            .with_active_fill(ActiveFill::None)
            .with_value("a");

            let root = tabs(&spec, &ctx, None, None);
            let active = tab_of(&root, "a");
            assert_eq!(
                active.style.descriptor.background, None,
                "{variant:?} must not fill the selected tab under None"
            );
            // The selected text colour is unaffected: text-primary, never the
            // inverse swap solid uses.
            assert_eq!(
                active.style.descriptor.text_color,
                Some(text_primary),
                "{variant:?} selected text colour must be unaffected"
            );
            let inactive = tab_of(&root, "b");
            assert_eq!(inactive.style.descriptor.background, None);
        }
    }

    #[test]
    fn block_none_fill_keeps_underline_edge() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = TabsSpec::new(vec![
            TabDefinition::new("a", "A"),
            TabDefinition::new("b", "B"),
        ])
        .with_variant(TabVariant::Block)
        .with_active_fill(ActiveFill::None)
        .with_active_edge(ActiveEdge::Underline)
        .with_value("a");

        let root = tabs(&spec, &ctx, None, None);
        let accent = theme.resolve_color(spec.indicator_token());

        // The strip equivalent: underline and no fill.
        let active = tab_of(&root, "a");
        assert_eq!(active.style.descriptor.background, None);
        assert_eq!(active.style.border_bottom_width, Some(rem_to_px(0.125)));
        assert_eq!(active.style.border_color_bottom, Some(accent));

        // Unselected tabs keep the transparent reserve edge.
        let inactive = tab_of(&root, "b");
        assert_eq!(inactive.style.border_bottom_width, Some(rem_to_px(0.125)));
        assert_eq!(inactive.style.border_color_bottom, Some(TRANSPARENT));
    }

    #[test]
    fn instance_scope_isolates_ids_relationships_and_focus_requests() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = TabsSpec::new(vec![
            TabDefinition::new("a", "A"),
            TabDefinition::new("b", "B"),
        ])
        .with_activation_mode(TabActivationMode::Manual)
        .with_value("a");
        let focused = Arc::new(Mutex::new(Vec::new()));
        let on_focus: TabHandler = {
            let focused = Arc::clone(&focused);
            Arc::new(move |value| focused.lock().unwrap().push(value.to_owned()))
        };
        let root = tabs_with_panel(
            &spec,
            &ctx,
            TabsHandlers {
                on_focus: Some(on_focus),
                instance_id: Some("first".to_owned()),
                ..TabsHandlers::default()
            },
            Node::text("Panel"),
        );

        let first = root
            .find(&|node| node.runtime_id.as_deref() == Some("tabs:first:tab:a"))
            .expect("scoped trigger exists");
        assert_eq!(first.id.as_deref(), Some("tabs:a"));
        assert_eq!(first.a11y.controls.as_deref(), Some("tabs-panel:a"));
        assert_eq!(
            first.interaction.on_key.as_ref().expect("key handler")(
                NodeKey::ArrowRight,
                poodle_node::NodeModifiers::default(),
            ),
            Some("tabs:first:tab:b".to_owned())
        );
        assert!(
            focused.lock().unwrap().is_empty(),
            "focus notification belongs to the backend focus event"
        );
        let focus_listener = root
            .find(&|node| node.runtime_id.as_deref() == Some("tabs:first:tab:b"))
            .expect("focus target exists")
            .interaction
            .on_focus_change
            .as_ref()
            .expect("focus listener");
        focus_listener(true);
        assert_eq!(*focused.lock().unwrap(), vec!["b"]);

        let panel = root
            .find(&|node| node.runtime_id.as_deref() == Some("tabs:first:panel:a"))
            .expect("scoped panel exists");
        assert_eq!(panel.id.as_deref(), Some("tabs-panel:a"));
        assert_eq!(panel.a11y.labelled_by.as_deref(), Some("tabs:a"));

        let second = tabs_with_handlers(
            &spec,
            &ctx,
            TabsHandlers {
                instance_id: Some("second".to_owned()),
                ..TabsHandlers::default()
            },
        );
        assert!(second
            .find(&|node| node.runtime_id.as_deref() == Some("tabs:second:tab:a"))
            .is_some());
        assert!(second
            .find(&|node| node.runtime_id.as_deref() == Some("tabs:first:tab:a"))
            .is_none());
    }

    fn reorder_spec() -> TabsSpec {
        TabsSpec::new(vec![
            TabDefinition::new("a", "A"),
            TabDefinition::new("b", "B").with_closable(true),
            TabDefinition::new("c", "C").with_disabled(true),
            TabDefinition::new("d", "D"),
        ])
        .with_reorderable(true)
        .with_value("a")
    }

    /// g16.026. The semantic family is choosable; the registration namespace
    /// is not. A composite that shares a kind must not also make two strips
    /// collide, and a strip must refuse a subject it does not own during
    /// eligibility so an ancestor composite target can win.
    #[test]
    fn an_explicit_subject_kind_shares_the_family_without_sharing_ids() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let mut spec = reorder_spec();
        spec.drag_subject_kind = Some("poodle.dock-panel".to_string());
        let root = tabs(&spec, &ctx, None, None);
        let a = tab_of(&root, "a");

        let source = a.interaction.drag_source.as_ref().expect("source");
        let target = a.interaction.drop_target.as_ref().expect("target");

        assert_eq!(source.subject.kind, "poodle.dock-panel");
        assert_eq!(source.subject.id, "a", "the tab value is the subject id");
        assert_eq!(
            target.accepted_kinds,
            vec!["poodle.dock-panel".to_string()],
            "the strip joins the composite's family"
        );

        // Ids stay instance-scoped, so a second strip with the same values
        // cannot collide with this one.
        assert_eq!(source.source_id, "tabs:source:a");
        assert_eq!(target.target_id, "tabs:target:a");

        let can_drop = target.can_drop.as_ref().expect("eligibility");
        let intent = poodle_node::DropIntent {
            target_id: target.target_id.clone(),
            position: "after".to_string(),
            operation: poodle_node::DragOperation::Move,
            destination: None,
        };
        let subject = |id: &str| poodle_node::DragSubject {
            kind: "poodle.dock-panel".to_string(),
            id: id.to_string(),
        };

        assert!(
            matches!(
                can_drop(&intent, &subject("b")),
                poodle_node::DropEligibility::Accepted { .. }
            ),
            "a row this strip owns is eligible"
        );
        assert!(
            matches!(
                can_drop(&intent, &subject("from-another-strip")),
                poodle_node::DropEligibility::Rejected { .. }
            ),
            "a same-family row from elsewhere is refused during eligibility"
        );
    }

    #[test]
    fn reorderable_tabs_register_a_scoped_source_and_target() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let root = tabs(&reorder_spec(), &ctx, None, None);
        let a = tab_of(&root, "a");
        let source = a.interaction.drag_source.as_ref().expect("source");
        assert_eq!(source.source_id, "tabs:source:a");
        assert_eq!(source.subject.id, "a");
        assert_eq!(source.subject.kind, crate::drag_drop::reorder_kind("tabs"));
        assert_eq!(
            a.interaction
                .drop_target
                .as_ref()
                .expect("target")
                .accepted_kinds,
            vec![crate::drag_drop::reorder_kind("tabs")],
            "one tab set is ineligible for another surface's rows"
        );
        assert_eq!(
            a.interaction
                .drop_target
                .as_ref()
                .expect("target")
                .target_id,
            "tabs:target:a"
        );
        assert_eq!(a.style.descriptor.cursor, CursorHint::Grab);
        let disabled = tab_of(&root, "c");
        assert!(disabled.interaction.drag_source.is_none());
        assert!(disabled.interaction.drop_target.is_none());
        assert!(disabled.interaction.on_key.is_none());
    }

    /// A tab bar runs along its main axis, so the band rule must read the
    /// horizontal fraction. Reading the vertical one would answer `before`
    /// for the whole strip and make every pointer reorder identical.
    #[test]
    fn the_tab_band_rule_reads_the_horizontal_fraction() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let root = tabs(&reorder_spec(), &ctx, None, None);
        let target = tab_of(&root, "a")
            .interaction
            .drop_target
            .clone()
            .expect("target");
        let resolve = target.resolve_position.expect("resolver");
        let input = |x: f32, y: f32| poodle_node::NodeDropPositionInput {
            fraction_x: x,
            fraction_y: y,
            subject: crate::drag_drop::reorder_subject("tabs", "b"),
            operation: poodle_node::DragOperation::Move,
            input_kind: poodle_node::NodeDragInputKind::Mouse,
        };

        assert_eq!(
            resolve(&input(0.1, 0.9)).as_deref(),
            Some(poodle_node::DROP_POSITION_BEFORE)
        );
        assert_eq!(
            resolve(&input(0.9, 0.1)).as_deref(),
            Some(poodle_node::DROP_POSITION_AFTER)
        );
    }

    #[test]
    fn pointer_drop_emits_the_complete_next_order() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let orders = Arc::new(Mutex::new(Vec::new()));
        let focused = Arc::new(Mutex::new(Vec::new()));
        let root = tabs_with_handlers(
            &reorder_spec(),
            &ctx,
            TabsHandlers {
                on_reorder: Some({
                    let orders = Arc::clone(&orders);
                    Arc::new(move |order: Vec<String>| {
                        orders.lock().unwrap().push(order);
                    })
                }),
                on_focus: Some({
                    let focused = Arc::clone(&focused);
                    Arc::new(move |value| focused.lock().unwrap().push(value.to_owned()))
                }),
                ..TabsHandlers::default()
            },
        );
        let target = tab_of(&root, "d")
            .interaction
            .drop_target
            .clone()
            .expect("target");
        let commit = (target.on_drop.as_ref().expect("drop"))(&poodle_node::NodeDropCommitEvent {
            subject: crate::drag_drop::reorder_subject("tabs", "a"),
            intent: poodle_node::DropIntent {
                target_id: target.target_id.clone(),
                position: poodle_node::DROP_POSITION_AFTER.to_string(),
                operation: poodle_node::DragOperation::Move,
                destination: None,
            },
            inbound_files: None,
        });
        assert_eq!(commit, poodle_node::NodeDropCommit::Committed);
        assert_eq!(
            orders.lock().unwrap().as_slice(),
            [vec![
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "a".to_string()
            ]]
        );
        assert_eq!(*focused.lock().unwrap(), vec!["a"]);
    }

    #[test]
    fn a_vertical_tab_band_rule_reads_the_vertical_fraction() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = reorder_spec().with_orientation(poodle_specs::Orientation::Vertical);
        let root = tabs(&spec, &ctx, None, None);
        let target = tab_of(&root, "a")
            .interaction
            .drop_target
            .clone()
            .expect("target");
        let resolve = target.resolve_position.expect("resolver");
        let input = |x: f32, y: f32| poodle_node::NodeDropPositionInput {
            fraction_x: x,
            fraction_y: y,
            subject: crate::drag_drop::reorder_subject("tabs", "b"),
            operation: poodle_node::DragOperation::Move,
            input_kind: poodle_node::NodeDragInputKind::Mouse,
        };

        assert_eq!(
            resolve(&input(0.9, 0.1)).as_deref(),
            Some(poodle_node::DROP_POSITION_BEFORE)
        );
        assert_eq!(
            resolve(&input(0.1, 0.9)).as_deref(),
            Some(poodle_node::DROP_POSITION_AFTER)
        );
    }

    #[test]
    fn alt_arrow_reorders_before_ordinary_focus_movement() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let orders = Arc::new(Mutex::new(Vec::new()));
        let changes = Arc::new(Mutex::new(Vec::new()));
        let root = tabs_with_handlers(
            &reorder_spec(),
            &ctx,
            TabsHandlers {
                on_change: Some({
                    let changes = Arc::clone(&changes);
                    Arc::new(move |value| changes.lock().unwrap().push(value.to_owned()))
                }),
                on_reorder: Some({
                    let orders = Arc::clone(&orders);
                    Arc::new(move |order: Vec<String>| {
                        orders.lock().unwrap().push(order);
                    })
                }),
                instance_id: Some("keys".to_owned()),
                ..TabsHandlers::default()
            },
        );
        let a = tab_of(&root, "a");
        let keys = a.interaction.on_key.as_ref().expect("key handler");
        let mut alt = poodle_node::NodeModifiers::default();
        alt.alt = true;
        assert_eq!(
            keys(NodeKey::ArrowRight, alt),
            Some("tabs:keys:tab:a".to_owned())
        );
        assert_eq!(
            orders.lock().unwrap().as_slice(),
            [vec![
                "b".to_string(),
                "a".to_string(),
                "c".to_string(),
                "d".to_string()
            ]]
        );
        assert!(
            changes.lock().unwrap().is_empty(),
            "Alt+Arrow must not fall through to automatic selection"
        );
        assert_eq!(
            keys(NodeKey::ArrowRight, poodle_node::NodeModifiers::default()),
            Some("tabs:keys:tab:b".to_owned())
        );
        assert_eq!(*changes.lock().unwrap(), vec!["b"]);
    }

    #[test]
    fn delete_closes_only_closable_enabled_tabs() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let closed = Arc::new(Mutex::new(Vec::new()));
        let root = tabs_with_handlers(
            &reorder_spec(),
            &ctx,
            TabsHandlers {
                on_close: Some({
                    let closed = Arc::clone(&closed);
                    Arc::new(move |value| closed.lock().unwrap().push(value.to_owned()))
                }),
                ..TabsHandlers::default()
            },
        );
        let keys = tab_of(&root, "b")
            .interaction
            .on_key
            .as_ref()
            .expect("key handler");
        keys(NodeKey::Delete, poodle_node::NodeModifiers::default());
        assert_eq!(*closed.lock().unwrap(), vec!["b"]);
        let a_keys = tab_of(&root, "a")
            .interaction
            .on_key
            .as_ref()
            .expect("key handler");
        a_keys(NodeKey::Delete, poodle_node::NodeModifiers::default());
        assert_eq!(*closed.lock().unwrap(), vec!["b"]);
    }
}
