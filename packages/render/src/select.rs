//! Select — one implementation, emitted as nodes.
//!
//! Contract: `docs/contracts/components/select.md`
//! Reference: `packages/svelte/components/src/Select.svelte`
//! Ported from: `packages/jetstream/components/src/select.rs`, whose structure
//! this transcribes exactly — same anatomy, same token paths, same state
//! recipes — so a backend interpreting these nodes reproduces that tier's
//! output verbatim. That is the parity gate, and it is asserted by diffing
//! draw commands in the Jetstream adapter's suite, not by inspection here.
//!
//! Closed state: trigger with chevron. Open state: relative wrapper holding
//! the trigger plus an absolutely-positioned overlay panel of options.

use std::sync::Arc;

use poodle_headless::select::{
    select_transition, select_visible_options, SelectContext, SelectEffect, SelectEvent,
};
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutOverflow, LayoutSizing,
    MainAxisAlignment, Node, NodePosition, NodeRole, StylePatch,
};
use poodle_specs::{SelectSpec, ValidationState};

use crate::color::with_alpha;
use crate::context::RenderContext;
use crate::presentation::{
    rem_to_px, resolve_supporting_visual_size, size_font_rem, size_height_offset_rem,
    size_padding_x_offset_rem,
};

/// One atomic transition result. Hosts apply `context`, then dispatch from
/// `effects` in order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectTransitionResult {
    pub context: SelectContext,
    pub effects: Vec<SelectEffect>,
}

/// Host-owned native interaction for one Select instance.
///
/// `instance_scope` is the lifetime-stable scope. It is not a web public prop,
/// and the renderer never invents one from render order or selected value.
#[derive(Clone)]
pub struct SelectHandlers {
    pub instance_scope: String,
    pub on_transition: Option<Arc<dyn Fn(SelectTransitionResult) + Send + Sync>>,
}

impl SelectHandlers {
    pub fn new(instance_scope: impl Into<String>) -> Self {
        Self {
            instance_scope: instance_scope.into(),
            on_transition: None,
        }
    }

    pub fn on_transition(
        mut self,
        handler: Arc<dyn Fn(SelectTransitionResult) + Send + Sync>,
    ) -> Self {
        self.on_transition = Some(handler);
        self
    }
}

fn select_part_id(scope: &str, part: &str) -> String {
    format!("select:{scope}:{part}")
}

fn select_option_id(scope: &str, value: &str) -> String {
    format!("select:{scope}:option:{value}")
}

pub(crate) fn composite_select_scope(
    instance_id: Option<&str>,
    authored: Option<&str>,
    fallback: &str,
) -> String {
    instance_id
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .or_else(|| authored.map(str::trim).filter(|value| !value.is_empty()))
        .unwrap_or(fallback)
        .to_string()
}

fn emit_select(spec: &SelectSpec, handlers: &SelectHandlers, event: SelectEvent) {
    let Some(handler) = &handlers.on_transition else {
        return;
    };
    let (context, effects) = select_transition(spec.select_context(), event);
    handler(SelectTransitionResult { context, effects });
}

pub fn select(spec: &SelectSpec, ctx: &RenderContext<'_>, handlers: &SelectHandlers) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    // Axis-faithful metrics (g12.019 recipe correction): the axis-layered
    // token plus the per-size offset — the old GPUI tier's form, matching
    // Svelte's `--poodle-select-control-height` / inline-padding CSS vars —
    // not the fixed per-size tables (`control_height_rem` /
    // `control_space_x_rem`), which ignore the theme's density/control-size
    // layering. At base tokens (the Jetstream provider, no axes) md/default
    // reproduces the old fixed values; under a preview axis the select now
    // follows the axis like Svelte does.
    let height = ctx.theme().resolve_space("size.control.height")
        + rem_to_px(size_height_offset_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = ctx.theme().resolve_space("space.control.x")
        + rem_to_px(size_padding_x_offset_rem(effective_size));
    // The indicator is always the sm icon token (the old GPUI tier's
    // `IconSize::Sm`), not a per-control-size ladder stop.
    let icon_size = ctx.theme().resolve_space("size.icon.sm");
    let item_gap = ctx.theme().resolve_space("space.inline.sm");

    let surface = ctx.theme().resolve_color("color.background.surface");
    let border_default = ctx.theme().resolve_color("color.border.default");
    let radius = ctx.theme().resolve_radius("radius.control");
    let surface_radius = ctx.theme().resolve_radius("radius.surface");
    let text_primary = ctx.theme().resolve_color("color.text.primary");
    let text_secondary = ctx.theme().resolve_color("color.text.secondary");
    // Svelte paints the placeholder in text-secondary
    // (`select.css`: `--poodle-recipe-select-placeholder-control-text` falls
    // back to `--poodle-color-text-secondary`). `color.text.placeholder` is
    // not a token — no provider resolves it (it fell back to black).
    let text_placeholder = text_secondary;
    let icon_muted = ctx.theme().resolve_color("color.icon.muted");
    let panel_fill = ctx.theme().resolve_color(spec.overlay_fill_token());

    // Trigger appearance follows the shared Select recipe defaults:
    // surface/border at reduced alpha over the page, not the full-strength
    // tokens.
    let fill = with_alpha(surface, surface.3 * 0.82);
    let base_border = with_alpha(border_default, border_default.3 * 0.72);

    let validation_border = match spec.validation_state {
        ValidationState::Invalid => Some(ctx.theme().resolve_color("color.status.danger")),
        ValidationState::Valid => Some(ctx.theme().resolve_color("color.status.success")),
        ValidationState::Pending => Some(ctx.theme().resolve_color("color.accent.base")),
        ValidationState::None => None,
    };
    let border_color = validation_border.unwrap_or(base_border);

    // Hover: border toward full strength (validation colour holds when set),
    // background slightly more opaque. Same recipe as the old GPUI tier.
    let hover_border =
        validation_border.unwrap_or_else(|| with_alpha(border_default, border_default.3 * 0.92));
    let hover_fill = with_alpha(surface, surface.3 * 0.88);

    let display_text = spec
        .trigger_text()
        .map(str::to_string)
        .or_else(|| spec.placeholder.clone())
        .unwrap_or_else(|| "Select…".to_string());
    // Placeholder styling keys on having no VALUE (Svelte:
    // `data-placeholder = !hasSelection`), not on the trigger text —
    // `trigger_text()` itself falls back to the placeholder string.
    let display_color = if spec.current_value().is_some() {
        text_primary
    } else {
        text_placeholder
    };

    let show_clear = spec.clearable && spec.current_value().is_some() && !spec.is_disabled;

    let trigger = build_trigger(
        spec,
        &display_text,
        display_color,
        icon_muted,
        text_secondary,
        font_size,
        icon_size,
        pad_x,
        height,
        item_gap,
        fill,
        border_color,
        radius,
        hover_border,
        hover_fill,
        show_clear,
        spec.is_disabled,
        ctx,
        handlers,
    );

    // The old GPUI tier wraps the select in a `min_w(size.select.minWidth)`
    // container in both states; carry that floor on the returned root.
    let root_min_width = ctx.theme().resolve_space("size.select.minWidth");

    if !spec.current_open() {
        let mut trigger = trigger;
        trigger.style.min_width = Some(root_min_width);
        trigger.runtime_id = Some(select_part_id(&handlers.instance_scope, "trigger"));
        return trigger;
    }

    let panel_top = height + ctx.theme().resolve_space("space.stack.sm");

    let mut panel = build_panel(
        spec,
        ctx,
        effective_size,
        font_size,
        icon_size,
        pad_x,
        height,
        item_gap,
        panel_top,
        panel_fill,
        border_color,
        surface_radius,
        text_primary,
        text_secondary,
        text_placeholder,
        icon_muted,
        handlers,
    );

    // Contract `dismissOnOutsideInteract` (default `true`): a *refusal* flag —
    // native overlays dismiss on outside interact by default. The refusal rides
    // the surface's interaction as an inert activation: a host implementing
    // outside-dismissal must not dismiss a panel carrying this marker (see
    // menu.rs for the full contract note).
    if !spec.dismiss_on_outside_interact {
        panel.interaction.on_activate = Some(Arc::new(|| {}));
    }

    let mut root = Node::container().child(trigger).child(panel);
    root.style.descriptor.layout.direction = LayoutDirection::Column;
    root.style.min_width = Some(root_min_width);
    root.position = NodePosition::Relative;
    if let Some(label) = spec.aria_label.as_deref() {
        root.a11y.label = Some(label.to_string());
    }
    root.a11y.role = Some(NodeRole::ComboBox);
    root.a11y.expanded = Some(spec.open.unwrap_or(false));
    root
}

#[expect(
    clippy::too_many_arguments,
    reason = "trigger rendering keeps resolved state and token metrics explicit"
)]
fn build_trigger(
    spec: &SelectSpec,
    display_text: &str,
    display_color: ColorValue,
    icon_muted: ColorValue,
    text_secondary: ColorValue,
    font_size: f32,
    icon_size: f32,
    pad_x: f32,
    height: f32,
    item_gap: f32,
    fill: ColorValue,
    border_color: ColorValue,
    radius: f32,
    hover_border: ColorValue,
    hover_fill: ColorValue,
    show_clear: bool,
    is_disabled: bool,
    ctx: &RenderContext<'_>,
    handlers: &SelectHandlers,
) -> Node {
    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.fill_width = true; // contract: trigger width 100%
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border_color;
        s.descriptor.corner_radii = poodle_node::StyleDescriptor::default().corner_radii;
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.layout.height = LayoutSizing::Fixed(height);
        s.descriptor.layout.spacing.padding.left = pad_x;
        s.descriptor.layout.spacing.padding.right = pad_x;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = item_gap;
        s.descriptor.cursor = CursorHint::Pointer;
        s.hover = Some(StylePatch {
            background: Some(hover_fill),
            border_color: Some(hover_border),
            text_color: None,
            opacity: None,
        });
    }
    el.interaction.focusable = true;
    el.runtime_id = Some(select_part_id(&handlers.instance_scope, "trigger"));

    if !is_disabled && handlers.on_transition.is_some() {
        let spec = spec.clone();
        let handlers = handlers.clone();
        el.interaction.on_activate = Some(Arc::new(move || {
            emit_select(&spec, &handlers, SelectEvent::Toggle);
        }));
    }

    let mut label = Node::text(display_text);
    label.style.descriptor.text_color = Some(display_color);
    label.style.text_size = Some(font_size);
    label.style.descriptor.layout.width = LayoutSizing::Grow;
    label.style.text_ellipsis = true;
    el = el.child(label);

    // Clear pill — its own handler always: it sits inside the trigger and an
    // unwired clear would bubble to toggle, opening the panel it was clearing.
    if show_clear {
        let radius_pill = ctx.theme().resolve_radius("radius.pill");
        let clear_pill = with_alpha(text_secondary, 0.18);
        let mut clear = Node::container();
        {
            let s = &mut clear.style;
            s.descriptor.background = Some(clear_pill);
            s.descriptor.corner_radii.top_left = radius_pill;
            s.descriptor.corner_radii.top_right = radius_pill;
            s.descriptor.corner_radii.bottom_right = radius_pill;
            s.descriptor.corner_radii.bottom_left = radius_pill;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            s.descriptor.cursor = CursorHint::Pointer;
        }
        let mut x = Node::icon("x", icon_size);
        x.style.descriptor.text_color = Some(text_secondary);
        clear = clear.child(x);

        clear.runtime_id = Some(select_part_id(&handlers.instance_scope, "clear"));
        clear.interaction.on_activate = Some(if handlers.on_transition.is_some() {
            let spec = spec.clone();
            let handlers = handlers.clone();
            Arc::new(move || emit_select(&spec, &handlers, SelectEvent::Clear))
        } else {
            Arc::new(|| {})
        });

        el = el.child(clear);
    }

    let mut chevron = Node::icon("chevron-down", icon_size);
    chevron.style.descriptor.text_color = Some(icon_muted);
    el = el.child(chevron);

    if is_disabled {
        el.style.descriptor.opacity = ctx.theme().resolve_opacity("state.opacity.disabled");
        el.interaction.disabled = true;
    }

    el
}

#[expect(
    clippy::too_many_arguments,
    reason = "panel rendering keeps resolved state and token metrics explicit"
)]
fn build_panel(
    spec: &SelectSpec,
    ctx: &RenderContext<'_>,
    effective_size: poodle_specs::ControlSize,
    font_size: f32,
    icon_size: f32,
    pad_x: f32,
    row_height: f32,
    item_gap: f32,
    panel_top: f32,
    panel_fill: ColorValue,
    border_color: ColorValue,
    surface_radius: f32,
    text_primary: ColorValue,
    text_secondary: ColorValue,
    text_placeholder: ColorValue,
    icon_muted: ColorValue,
    handlers: &SelectHandlers,
) -> Node {
    let panel_py = rem_to_px(0.25);
    let token_min_width = ctx.theme().resolve_space("size.select.minWidth");
    let min_width = spec
        .menu_min_width
        .as_deref()
        .map(parse_css_length_to_px)
        .filter(|w| *w > 0.0)
        .unwrap_or(token_min_width);
    let max_height = ctx.theme().resolve_space("size.menu.maxHeight");

    let mut panel = Node::container();
    {
        let s = &mut panel.style;
        // Token-accurate `elevation.overlay` (single layer, spread 0 — the
        // shared mapping both backends implement).
        s.descriptor.shadow = Some(poodle_tokens::typed::semantic::ELEVATION_OVERLAY);
        s.descriptor.background = Some(panel_fill);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border_color;
        s.descriptor.corner_radii.top_left = surface_radius;
        s.descriptor.corner_radii.top_right = surface_radius;
        s.descriptor.corner_radii.bottom_right = surface_radius;
        s.descriptor.corner_radii.bottom_left = surface_radius;
        s.min_width = Some(min_width);
        s.max_height = Some(max_height);
        s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.padding.top = panel_py;
        s.descriptor.layout.spacing.padding.bottom = panel_py;
        s.overlay = true;
    }
    panel.position = NodePosition::Absolute {
        top: Some(panel_top),
        left: Some(0.0),
        right: None,
        bottom: None,
    };
    panel.a11y.role = Some(NodeRole::ListBox);
    panel.runtime_id = Some(select_part_id(&handlers.instance_scope, "listbox"));

    if spec.shows_search_input() {
        let query = spec.search_query.as_deref().unwrap_or("");
        let mut search_row = Node::container();
        {
            let s = &mut search_row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = item_gap;
            s.descriptor.layout.spacing.padding.left = pad_x;
            s.descriptor.layout.spacing.padding.right = pad_x;
            s.descriptor.layout.height = LayoutSizing::Fixed(row_height);
        }
        let mut glass = Node::icon("search", icon_size);
        glass.style.descriptor.text_color = Some(icon_muted);
        search_row = search_row.child(glass);

        let mut q = if query.is_empty() {
            let mut n = Node::text("Search…");
            n.style.descriptor.text_color = Some(text_placeholder);
            n
        } else {
            let mut n = Node::text(query);
            n.style.descriptor.text_color = Some(text_primary);
            n
        };
        q.style.text_size = Some(font_size);
        q.style.descriptor.layout.width = LayoutSizing::Grow;
        search_row = search_row.child(q);
        search_row.runtime_id = Some(select_part_id(&handlers.instance_scope, "search"));

        panel = panel.child(search_row);
    }

    let current_value = spec.current_value();
    let visible_values: Vec<String> = select_visible_options(&spec.select_context())
        .into_iter()
        .map(|option| option.value.clone())
        .collect();
    let filtered: Vec<&poodle_specs::ChoiceOption> = spec
        .options
        .iter()
        .filter(|opt| visible_values.iter().any(|value| value == &opt.value))
        .collect();
    let accent = ctx.theme().resolve_color("color.accent.base");
    let highlight_fill = with_alpha(accent, 0.14);

    if filtered.is_empty() {
        let mut empty = Node::text(&spec.empty_message);
        {
            let s = &mut empty.style;
            s.descriptor.text_color = Some(text_secondary);
            s.text_size = Some(font_size);
            s.descriptor.layout.spacing.padding.left = pad_x;
            s.descriptor.layout.spacing.padding.right = pad_x;
            s.descriptor.layout.spacing.padding.top = rem_to_px(0.5);
            s.descriptor.layout.spacing.padding.bottom = rem_to_px(0.5);
        }
        panel = panel.child(empty);
    } else {
        let mut seen_groups: Vec<Option<String>> = Vec::new();
        for opt in &filtered {
            let key = opt.group.clone();
            if !seen_groups.contains(&key) {
                seen_groups.push(key);
            }
        }

        for group_key in &seen_groups {
            if let Some(ref name) = group_key {
                let header_py = rem_to_px(0.25);
                let header_font = rem_to_px(size_font_rem(resolve_supporting_visual_size(
                    effective_size,
                )));
                let mut header = Node::text(name.as_str());
                {
                    let s = &mut header.style;
                    s.descriptor.text_color = Some(text_secondary);
                    s.text_size = Some(header_font);
                    s.text_weight = Some(600);
                    s.descriptor.layout.spacing.padding.left = pad_x;
                    s.descriptor.layout.spacing.padding.right = pad_x;
                    s.descriptor.layout.spacing.padding.top = header_py;
                    s.descriptor.layout.spacing.padding.bottom = header_py;
                }
                panel = panel.child(header);
            }

            for opt in filtered.iter().filter(|o| &o.group == group_key) {
                let is_selected = current_value
                    .map(|v| v == opt.value.as_str())
                    .unwrap_or(false);
                let is_highlighted = spec.highlighted_value.as_deref() == Some(opt.value.as_str());

                let label_color = if opt.is_disabled {
                    text_secondary
                } else {
                    text_primary
                };

                let mut row = Node::container();
                {
                    let s = &mut row.style;
                    s.descriptor.layout.direction = LayoutDirection::Row;
                    s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                    s.descriptor.layout.spacing.gap = item_gap;
                    s.descriptor.layout.spacing.padding.left = pad_x;
                    s.descriptor.layout.spacing.padding.right = pad_x;
                    s.descriptor.cursor = CursorHint::Pointer;
                    if is_highlighted {
                        s.descriptor.background = Some(highlight_fill);
                    }
                }
                row.interaction.focusable = true;
                row.runtime_id = Some(select_option_id(&handlers.instance_scope, &opt.value));
                row.a11y.role = Some(NodeRole::ListBoxOption);
                row.a11y.label = Some(opt.label.clone());
                row.a11y.selected = Some(is_selected);

                // Rows with a description are taller; plain rows keep the
                // fixed control row height.
                if opt.description.is_none() {
                    row.style.descriptor.layout.height = LayoutSizing::Fixed(row_height);
                }

                if let Some(ref description) = opt.description {
                    let mut stack = Node::container();
                    stack.style.descriptor.layout.direction = LayoutDirection::Column;
                    stack.style.descriptor.layout.width = LayoutSizing::Grow;

                    let mut l = Node::text(opt.label.as_str());
                    l.style.descriptor.text_color = Some(label_color);
                    l.style.text_size = Some(font_size);
                    l.style.text_ellipsis = true;
                    stack = stack.child(l);

                    let mut d = Node::text(description.as_str());
                    d.style.descriptor.text_color = Some(text_secondary);
                    d.style.text_size = Some(rem_to_px(0.6875));
                    stack = stack.child(d);

                    row = row.child(stack);
                } else {
                    let mut l = Node::text(opt.label.as_str());
                    l.style.descriptor.text_color = Some(label_color);
                    l.style.text_size = Some(font_size);
                    l.style.descriptor.layout.width = LayoutSizing::Grow;
                    l.style.text_ellipsis = true;
                    row = row.child(l);
                }

                if is_selected {
                    let mut check = Node::icon("check", icon_size);
                    check.style.descriptor.text_color = Some(icon_muted);
                    row = row.child(check);
                }

                if opt.is_disabled {
                    row.style.descriptor.opacity =
                        ctx.theme().resolve_opacity("state.opacity.disabled");
                    row.interaction.disabled = true;
                }

                if !opt.is_disabled && handlers.on_transition.is_some() {
                    let spec = spec.clone();
                    let handlers = handlers.clone();
                    let value = opt.value.clone();
                    row.interaction.on_activate = Some(Arc::new(move || {
                        emit_select(
                            &spec,
                            &handlers,
                            SelectEvent::CommitOption {
                                value: value.clone(),
                            },
                        );
                    }));
                }

                panel = panel.child(row);
            }
        }
    }

    panel
}

/// Parse a CSS length ("12rem", "200px") to logical pixels; 0.0 on failure.
fn parse_css_length_to_px(value: &str) -> f32 {
    let trimmed = value.trim();
    if let Some(num) = trimmed.strip_suffix("rem") {
        num.trim().parse::<f32>().map(rem_to_px).unwrap_or(0.0)
    } else if let Some(num) = trimmed.strip_suffix("px") {
        num.trim().parse::<f32>().unwrap_or(0.0)
    } else {
        trimmed.parse::<f32>().unwrap_or(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::ChoiceOption;

    /// The real token resolver over the ECLIPSE theme. Pure — no backend.
    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn fruit_options() -> Vec<ChoiceOption> {
        vec![
            ChoiceOption::new("apple", "Apple"),
            ChoiceOption::new("banana", "Banana"),
            ChoiceOption::new("cherry", "Cherry"),
        ]
    }

    #[test]
    fn closed_trigger_shows_placeholder_and_chevron_only() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = SelectSpec::new(fruit_options()).with_placeholder("Choose a fruit");
        let node = select(&spec, &ctx, &SelectHandlers::new("test"));
        assert!(node.has_text("Choose a fruit"), "{:?}", node.texts());
        assert!(node.has_text("chevron-down"), "{:?}", node.texts());
        assert!(!node.has_text("Apple"), "options leaked when closed");
    }

    #[test]
    fn trigger_metrics_follow_the_axis_faithful_recipe() {
        // height = size.control.height token (36px at base) + per-size offset
        let cases = [
            (poodle_specs::ControlSize::Xs, 28.0),
            (poodle_specs::ControlSize::Sm, 30.0),
            (poodle_specs::ControlSize::Md, 36.0),
            (poodle_specs::ControlSize::Lg, 42.0),
            (poodle_specs::ControlSize::Xl, 44.0),
        ];
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        for (size, expected) in cases {
            let spec = SelectSpec::new(fruit_options()).with_size(size);
            let node = select(&spec, &ctx, &SelectHandlers::new("test"));
            match node.style.descriptor.layout.height {
                LayoutSizing::Fixed(h) => {
                    assert_eq!(h, expected, "height for {size:?}");
                }
                ref other => panic!("expected fixed height, got {other:?}"),
            }
        }
        // The placeholder paints in text.secondary (Svelte's recipe), not the
        // nonexistent `color.text.placeholder` token.
        let secondary =
            poodle_adapter::ThemeProvider::resolve_color(&theme, "color.text.secondary");
        let spec = SelectSpec::new(fruit_options()).with_placeholder("Choose a fruit");
        let node = select(&spec, &ctx, &SelectHandlers::new("test"));
        let label = node
            .find(
                &|n| matches!(&n.kind, poodle_node::NodeKind::Text { content } if content == "Choose a fruit"),
            )
            .expect("placeholder label");
        assert_eq!(label.style.descriptor.text_color, Some(secondary));
    }

    #[test]
    fn open_renders_options_in_an_absolute_overlay_listbox() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = SelectSpec::new(fruit_options()).with_open(true);
        let node = select(&spec, &ctx, &SelectHandlers::new("test"));
        assert!(node.has_text("Apple") && node.has_text("Banana") && node.has_text("Cherry"));

        let panel = node
            .find(&|n| n.style.overlay)
            .expect("an overlay panel exists");
        assert!(matches!(
            panel.position,
            NodePosition::Absolute { top: Some(t), left: Some(0.0), .. } if t > 0.0
        ));
        assert_eq!(panel.a11y.role, Some(NodeRole::ListBox));
        assert!(matches!(node.position, NodePosition::Relative));
    }

    #[test]
    fn choosing_an_option_reports_its_value_through_the_node_handler() {
        use std::sync::Mutex;
        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let handlers = SelectHandlers::new("pick").on_transition(Arc::new(move |result| {
            for effect in &result.effects {
                if let SelectEffect::ValueChanged { value } = effect {
                    sink.lock().unwrap().push(value.clone());
                }
            }
        }));
        let spec = SelectSpec::new(fruit_options()).with_open(true);
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = select(&spec, &ctx, &handlers);

        let banana_row = node
            .find(&|n| {
                n.a11y.label.as_deref() == Some("Banana") && n.interaction.on_activate.is_some()
            })
            .expect("banana row is activatable");
        (banana_row.interaction.on_activate.as_ref().unwrap())();
        assert_eq!(seen.lock().unwrap().as_slice(), ["banana"]);
    }

    #[test]
    fn a_disabled_select_has_no_activation() {
        let handlers = SelectHandlers::new("disabled").on_transition(Arc::new(|_| {}));
        let spec = SelectSpec {
            is_disabled: true,
            ..SelectSpec::new(fruit_options())
        };
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = select(&spec, &ctx, &handlers);
        assert!(node.interaction.on_activate.is_none());
        assert!(node.interaction.disabled);
    }

    #[test]
    fn search_filters_and_empty_query_shows_message() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = SelectSpec::new(fruit_options())
            .with_searchable(true)
            .with_search_query("ban")
            .with_open(true);
        let node = select(&spec, &ctx, &SelectHandlers::new("test"));
        assert!(node.has_text("Banana") && !node.has_text("Apple"));

        let spec = SelectSpec::new(fruit_options())
            .with_searchable(true)
            .with_search_query("zzz")
            .with_empty_message("No matches")
            .with_open(true);
        let node = select(&spec, &ctx, &SelectHandlers::new("test"));
        assert!(node.has_text("No matches"));
    }

    #[test]
    fn outside_interact_refusal_marks_the_open_panel() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        // Web default `true` + open: the panel carries no refusal marker.
        let spec = SelectSpec::new(fruit_options()).with_open(true);
        let node = select(&spec, &ctx, &SelectHandlers::new("test"));
        let panel = node
            .find(&|n| n.a11y.role == Some(poodle_node::NodeRole::ListBox))
            .expect("open panel");
        assert!(panel.interaction.on_activate.is_none());

        // Refusal: the open panel carries the inert activation marker a host
        // keys outside-dismissal on.
        let refusing = spec.with_dismiss_on_outside_interact(false);
        let node = select(&refusing, &ctx, &SelectHandlers::new("test"));
        let panel = node
            .find(&|n| n.a11y.role == Some(poodle_node::NodeRole::ListBox))
            .expect("open panel");
        assert!(panel.interaction.on_activate.is_some());
    }

    #[test]
    fn instance_scopes_do_not_collide() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = SelectSpec::new(fruit_options())
            .with_open(true)
            .with_clearable(true)
            .with_value("apple");
        let left = select(&spec, &ctx, &SelectHandlers::new("one"));
        let right = select(&spec, &ctx, &SelectHandlers::new("two"));
        assert_eq!(
            left.find(&|n| n.runtime_id.as_deref() == Some("select:one:trigger"))
                .and_then(|n| n.runtime_id.clone()),
            Some("select:one:trigger".to_string())
        );
        assert!(right
            .find(&|n| n.runtime_id.as_deref() == Some("select:two:option:banana"))
            .is_some());
        assert!(left
            .find(&|n| n.runtime_id.as_deref() == Some("select:two:trigger"))
            .is_none());
    }

    #[test]
    fn highlighted_option_is_projected() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = SelectSpec::new(fruit_options())
            .with_open(true)
            .with_highlighted_value("banana");
        let node = select(&spec, &ctx, &SelectHandlers::new("hl"));
        let banana = node
            .find(&|n| n.runtime_id.as_deref() == Some("select:hl:option:banana"))
            .expect("banana option");
        assert!(banana.style.descriptor.background.is_some());
    }

    #[test]
    fn freeform_alone_does_not_show_search() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = SelectSpec::new(fruit_options())
            .with_freeform(true)
            .with_open(true);
        let node = select(&spec, &ctx, &SelectHandlers::new("ff"));
        assert!(!node.has_text("Search…"));
        assert!(!spec.shows_search_input());
    }

    #[test]
    fn composite_scope_prefers_instance_id_then_authored_label() {
        assert_eq!(
            composite_select_scope(Some("host"), Some("label"), "fallback"),
            "host"
        );
        assert_eq!(
            composite_select_scope(None, Some("label"), "fallback"),
            "label"
        );
        assert_eq!(
            composite_select_scope(Some(""), Some("label"), "fallback"),
            "label"
        );
        assert_eq!(composite_select_scope(None, None, "fallback"), "fallback");
    }
}
