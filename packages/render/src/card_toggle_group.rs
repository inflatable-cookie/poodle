//! CardToggleGroup — multi-select card group in a capped-column grid.
//!
//! Contract: `docs/contracts/components/card-toggle-group.md`
//! Ported from: `packages/jetstream/components/src/card_toggle_group.rs`.
//!
//! Options lay out in a responsive auto-fit grid: each cell seeds just under
//! a `1/columns` share of the row, grows to fill it, and wraps onto the next
//! row once the size-adjusted minimum width no longer fits. `columns` (1–4) is
//! the row's upper bound, not a fixed count.
//! Recipe reconciled to the old GPUI tier
//! (`packages/gpui/components/src/composites/card_toggle_group.rs`): the
//! density-table grid gap, spec-helper fonts, Card-composed selection
//! treatment, and focusable option cells carrying the toggle handler.
//!
//! Selection runs through the shared ToggleGroup machine in single mode with
//! the spec's `allow_deactivation`, so the handlers entry point receives the
//! resulting `string | null`, not the pressed option. Arrow navigation walks
//! the enabled options, wraps at the ends, selects the target, and moves real
//! backend focus — the Svelte `menuListNavigate` behaviour.

use std::sync::Arc;

use poodle_headless::single_select::SelectOption;
use poodle_headless::toggle_group::{
    toggle_group_transition, SelectionMode, ToggleGroupContext, ToggleGroupEffect,
    ToggleGroupEvent, ToggleGroupValue,
};
use poodle_node::{
    CrossAxisAlignment, CursorHint, FocusRing, LayoutDirection, MainAxisAlignment, Node, NodeKey,
    NodeModifiers, NodeRole, NodeToggled,
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

/// Stable identity for an observable option slot (title, count). Distinct per
/// caller scope, so duplicate instances keep distinguishable slot bounds.
fn option_part_id(instance_scope: &str, value: &str, part: &str) -> String {
    format!("card-toggle:{instance_scope}:option:{value}:{part}")
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

/// Share of a row reserved from each cell's `1/columns` seed so the flex line
/// breaker accounts for the inter-column gaps the CSS `calc()` track subtracts.
/// The flex algorithm breaks a line on the unscaled base size, so an exact
/// `1/columns` seed plus a gap can never fit `columns` cells in one row. The
/// reservation covers the largest density gap at the smallest size floor
/// (2 columns × xs 9.5rem with comfortable 1rem gaps) and `flex_grow` fills
/// the slack, so the configured column count still bounds a row.
const GRID_GAP_SHARE: f32 = 0.03;

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

/// How an activation reports. The original entry point kept its pressed-value
/// payload; the contract entry point runs the shared machine and reports the
/// resulting value (including `null`).
#[derive(Clone)]
enum ToggleEmit {
    Pressed(Arc<dyn Fn(&str) + Send + Sync>),
    Result(Arc<dyn Fn(Option<&str>) + Send + Sync>),
}

impl ToggleEmit {
    fn emit(&self, context: &ToggleGroupContext, option_value: &str) {
        match self {
            ToggleEmit::Pressed(handler) => handler(option_value),
            ToggleEmit::Result(handler) => {
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
        }
    }
}

/// Arrow navigation over the enabled options, wrapping at both ends. The
/// target is selected (emitting through the active emitter) and returned as
/// the focus identity the backend moves to. Both axes are live, matching
/// Svelte's `menuListNavigate` over the enabled list.
fn roving_key_handler(
    value: &str,
    roving: &[String],
    instance_scope: String,
    context: ToggleGroupContext,
    emit: Option<ToggleEmit>,
) -> Option<Arc<dyn Fn(NodeKey, NodeModifiers) -> Option<String> + Send + Sync>> {
    let index = roving.iter().position(|candidate| candidate == value)?;
    let ids = roving.to_vec();
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
        // A single enabled option wraps to itself. Svelte's `menuListNavigate`
        // returns index 0 there and still calls `select`, so the same target
        // must run the toggle machine rather than short-circuiting; with
        // `allowDeactivation` that clears the active value. Focus stays on
        // the wrapped option.
        if let Some(emit) = &emit {
            emit.emit(&context, &target);
        }
        Some(option_focus_id(&instance_scope, &target))
    }))
}

/// Render a group without a caller-provided identity, keeping the original
/// callback contract: the handler receives the pressed option value. Prefer
/// [`card_toggle_group_with_handlers`] for the contract's resulting-value
/// payload (including `null` deactivation) and caller-scoped focus identity.
pub fn card_toggle_group(
    spec: &CardToggleGroupSpec,
    ctx: &RenderContext<'_>,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    render_card_toggle_group(
        spec,
        ctx,
        "card-toggle-group",
        on_change.map(ToggleEmit::Pressed),
    )
}

pub fn card_toggle_group_with_handlers(
    spec: &CardToggleGroupSpec,
    ctx: &RenderContext<'_>,
    handlers: CardToggleGroupHandlers,
) -> Node {
    render_card_toggle_group(
        spec,
        ctx,
        &handlers.instance_id,
        handlers.on_value_change.map(ToggleEmit::Result),
    )
}

fn render_card_toggle_group(
    spec: &CardToggleGroupSpec,
    ctx: &RenderContext<'_>,
    instance_scope: &str,
    emit: Option<ToggleEmit>,
) -> Node {
    let theme = ctx.theme();
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);

    // Contract §7 size scale — via the spec helpers.
    let title_font = rem_to_px(CardToggleGroupSpec::title_font_rem(effective_size));
    let description_font = rem_to_px(CardToggleGroupSpec::description_font_rem(effective_size));

    // Density-driven grid gap (contract §7 density table) + Card body rhythm.
    let grid_gap = rem_to_px(control_space_x_rem(density));
    let body_gap = rem_to_px(0.25);
    // Contract §7 header: gap 0.75rem, align-items center.
    let header_gap = rem_to_px(0.75);

    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let border_subtle = theme.resolve_color("color.border.subtle");
    let pill_radius = theme.resolve_radius("radius.pill");
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

    // Contract §6: responsive auto-fit grid. The `columns` prop is an upper
    // bound, not a fixed count; each card seeds at `1/columns` of the row and
    // grows to fill it, wrapping onto the next row once its size-adjusted
    // minimum width can no longer fit. Baseline root gap is the grid gap.
    let cols = spec.column_count();
    let min_width = rem_to_px(CardToggleGroupSpec::min_width_rem(effective_size));
    let mut cells: Vec<Node> = Vec::new();

    for option in &spec.options {
        let is_selected = spec.is_selected(&option.value);
        let is_option_disabled = spec.disabled || option.disabled;

        // Header row: title + optional count pill (contract §2/§7).
        let mut header = Node::container();
        {
            let s = &mut header.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = header_gap;
            if option.count.is_some() {
                // The web pins the count to the end of the row with
                // `margin-left: auto`; the node vocabulary has no auto margin,
                // so the two ends distribute instead.
                s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
            }
        }
        let mut title = Node::text(option.title.clone());
        title.id = Some(option_part_id(instance_scope, &option.value, "title"));
        title.style.text_size = Some(title_font);
        title.style.text_weight = Some(600);
        title.style.descriptor.text_color = Some(text_primary);
        title.style.min_width = Some(0.0);
        let mut header = header.child(title);
        if let Some(count) = &option.count {
            let (pad_y, pad_x) = CardToggleGroupSpec::count_padding_rem(effective_size);
            let mut count_el = Node::text(count.clone());
            count_el.id = Some(option_part_id(instance_scope, &option.value, "count"));
            count_el.style.text_size = Some(rem_to_px(CardToggleGroupSpec::count_font_rem(
                effective_size,
            )));
            count_el.style.text_weight = Some(700);
            count_el.style.line_height = Some(1.25);
            count_el.style.descriptor.text_color = Some(text_secondary);
            count_el.style.descriptor.border.width = rem_to_px(0.0625);
            count_el.style.descriptor.border.color = border_subtle;
            let c = &mut count_el.style.descriptor.corner_radii;
            c.top_left = pill_radius;
            c.top_right = pill_radius;
            c.bottom_right = pill_radius;
            c.bottom_left = pill_radius;
            let pad = &mut count_el.style.descriptor.layout.spacing.padding;
            pad.top = rem_to_px(pad_y);
            pad.bottom = rem_to_px(pad_y);
            pad.left = rem_to_px(pad_x);
            pad.right = rem_to_px(pad_x);
            count_el.style.flex_none = true;
            header = header.child(count_el);
        }

        // Card body: header row + optional description.
        let mut body = Node::container();
        {
            let s = &mut body.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = body_gap;
        }
        let mut body = body.child(header);
        if let Some(description) = &option.description {
            let mut d = Node::text(description.clone());
            d.style.text_size = Some(description_font);
            d.style.descriptor.text_color = Some(text_secondary);
            body = body.child(d);
        }

        // Compose the Card primitive — selected state owns the fill/border.
        // A disabled option never composes the interactive tier, so the inner
        // card keeps no pointer cursor and no hover patch; the option cell
        // owns the not-allowed cursor for the whole disabled card.
        let mut card_spec = CardSpec::new();
        if !is_option_disabled {
            card_spec = card_spec.interactive();
        }
        if is_selected {
            card_spec = card_spec.selected();
        }
        card_spec = card_spec.with_aria_label(option.title.clone());
        // The old Card's body slot wraps content in a flex-growing box.
        let mut body_slot = Node::container();
        body_slot.style.descriptor.layout.direction = LayoutDirection::Row;
        body_slot.style.flex_grow = Some(1.0);
        // The option cell owns the button semantics. The composed Card is a
        // surface child (Svelte's interactive Card is a plain div), so clear
        // the inner button role and label: the option exposes exactly one
        // Button, named by the option title.
        let mut option_card = card(&card_spec, ctx, vec![body_slot.child(body)]);
        option_card.id = Some(option_part_id(instance_scope, &option.value, "card"));
        option_card.a11y.role = None;
        option_card.a11y.label = None;

        // Wrap each Card so it can grow within the grid. The cell is the
        // focusable activation target; a disabled option dims and shows the
        // not-allowed cursor instead of wiring the toggle.
        let mut option_el = flex1_cell();
        option_el.id = Some(option_id(&option.value));
        option_el.runtime_id = Some(option_focus_id(instance_scope, &option.value));
        option_el.a11y.role = Some(NodeRole::Button);
        option_el.a11y.label = Some(option.title.clone());
        // aria-pressed, not aria-selected: the option is a toggle button, the
        // same projection ToggleGroup and SegmentedControl use.
        option_el.a11y.selected = None;
        option_el.a11y.toggled = Some(if is_selected {
            NodeToggled::True
        } else {
            NodeToggled::False
        });
        option_el.style.min_width = Some(min_width);
        // Seed just under one column share so the gap reservation keeps the
        // configured column count as the row's upper bound; `flex_grow` then
        // fills the row and `flex_basis_pct` wins over the zero pixel basis.
        option_el.style.flex_basis_pct = Some(1.0 / cols as f32 - GRID_GAP_SHARE);
        option_el.style.flex_basis = None;
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
            if let Some(emit) = &emit {
                let emit = emit.clone();
                let context = context.clone();
                let value = option.value.clone();
                option_el.interaction.on_activate = Some(Arc::new(move || {
                    emit.emit(&context, &value);
                }));
            }
            option_el.interaction.on_key = roving_key_handler(
                &option.value,
                &roving,
                instance_scope.to_string(),
                context.clone(),
                emit.clone(),
            );
        }

        cells.push(option_el);
    }

    // Responsive root: one wrapping row of column-share cells. Cards in a row
    // share the stretch cross size, so equal-height cards follow from the
    // Card's own height.
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Stretch;
        s.flex_wrap = true;
        s.fill_width = true;
        s.descriptor.layout.spacing.gap = grid_gap;
    }
    for cell in cells {
        root = root.child(cell);
    }

    // Group-level disabled reaches every option through `is_option_disabled`;
    // each option dims once there. The root itself never dims (the web
    // contract has no group opacity rule), so a disabled group is not dimmed
    // twice.
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
    fn count_pill_follows_the_size_ladder() {
        // Contract §7 count column: padding and font per size, pill radius,
        // subtle border, secondary tone and no flex shrink.
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let subtle = theme.resolve_color("color.border.subtle");
        let secondary = theme.resolve_color("color.text.secondary");
        let pill = theme.resolve_radius("radius.pill");
        let cases = [
            (poodle_specs::ControlSize::Xs, 10.0, 5.0, 0.5),
            (poodle_specs::ControlSize::Sm, 11.0, 6.0, 0.5),
            (poodle_specs::ControlSize::Md, 11.5, 7.0, 0.5),
            (poodle_specs::ControlSize::Lg, 13.0, 9.0, 1.5),
            (poodle_specs::ControlSize::Xl, 14.0, 10.0, 2.0),
        ];
        for (size, expected_font, expected_pad_x, expected_pad_y) in cases {
            let spec = CardToggleGroupSpec::new(vec![
                CardToggleOption::new("alpha", "Alpha").with_count("24")
            ])
            .with_size(size);
            let node = card_toggle_group(&spec, &ctx, None);
            let count = node
                .find(&|n| matches!(&n.kind, poodle_node::NodeKind::Text { content } if content == "24"))
                .expect("count text");
            assert_eq!(
                count.style.text_size,
                Some(expected_font),
                "font for {size:?}"
            );
            assert_eq!(count.style.text_weight, Some(700));
            assert_eq!(count.style.line_height, Some(1.25));
            assert_eq!(count.style.descriptor.text_color, Some(secondary));
            assert_eq!(count.style.descriptor.border.width, 1.0);
            assert_eq!(count.style.descriptor.border.color, subtle);
            assert_eq!(count.style.descriptor.corner_radii.top_left, pill);
            assert_eq!(
                count.style.descriptor.layout.spacing.padding.left, expected_pad_x,
                "inline padding for {size:?}"
            );
            assert_eq!(
                count.style.descriptor.layout.spacing.padding.top, expected_pad_y,
                "block padding for {size:?}"
            );
            assert!(count.style.flex_none);
            // No count, no node.
            let without = card_toggle_group(&CardToggleGroupSpec::new(options()), &ctx, None);
            assert!(without
                .find(&|n| n.id.as_deref().is_some_and(|id| id.ends_with(":count")))
                .is_none());
        }
    }

    #[test]
    fn selected_card_carries_the_accent_border_and_option_label() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let accent = theme.resolve_color("color.accent.base");
        let spec = CardToggleGroupSpec::new(options()).with_values(vec!["alpha".to_string()]);
        let node = card_toggle_group(&spec, &ctx, None);

        // Selection owns the border through the composed Card primitive; the
        // option cell (not the card) carries the accessible name.
        let alpha_card = node
            .find(&|n| n.has_text("Alpha") && n.style.descriptor.border.width > 0.0)
            .expect("alpha card");
        assert_eq!(alpha_card.style.descriptor.border.color, accent);
        assert_eq!(alpha_card.a11y.label, None);
        let beta_card = node
            .find(&|n| n.has_text("Beta") && n.style.descriptor.border.width > 0.0)
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
        assert_eq!(alpha.a11y.selected, None);
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
    fn toggling_an_option_reports_the_resulting_value_through_the_handlers() {
        use std::sync::Mutex;
        let seen: Arc<Mutex<Vec<Option<String>>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let on_change: Arc<dyn Fn(Option<&str>) + Send + Sync> =
            Arc::new(move |v: Option<&str>| sink.lock().unwrap().push(v.map(str::to_owned)));
        let spec = CardToggleGroupSpec::new(options());
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = card_toggle_group_with_handlers(
            &spec,
            &ctx,
            CardToggleGroupHandlers::new("toggling").on_value_change(on_change),
        );

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
        let node = card_toggle_group_with_handlers(
            &spec,
            &ctx,
            CardToggleGroupHandlers::new("deactivate").on_value_change(on_change),
        );

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
    fn the_legacy_entry_point_keeps_the_pressed_value_payload() {
        use std::sync::Mutex;
        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let on_change: Arc<dyn Fn(&str) + Send + Sync> =
            Arc::new(move |v: &str| sink.lock().unwrap().push(v.to_owned()));
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = card_toggle_group(&CardToggleGroupSpec::new(options()), &ctx, Some(on_change));

        let cells = cells(&node);
        let beta = cells
            .iter()
            .find(|c| c.has_text("Beta"))
            .expect("beta cell");
        (beta
            .interaction
            .on_activate
            .as_ref()
            .expect("beta is activatable"))();
        assert_eq!(seen.lock().unwrap().as_slice(), ["beta"]);
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
        let node = card_toggle_group(&spec, &ctx, Some(Arc::new(|_: &str| {})));

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
    fn a_disabled_group_dims_each_option_once_and_wires_nothing() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let disabled_opacity = theme.resolve_opacity("state.opacity.disabled");
        let spec = CardToggleGroupSpec::new(options()).with_disabled(true);
        let node = card_toggle_group(&spec, &ctx, Some(Arc::new(|_: &str| {})));
        // The web contract dims the option, never the root; a root that dimmed
        // too would double-dim the group.
        assert_eq!(node.style.descriptor.opacity, 1.0);
        let cells = cells(&node);
        assert_eq!(cells.len(), 3);
        assert!(cells
            .iter()
            .all(|c| c.style.descriptor.opacity == disabled_opacity));
        assert!(cells.iter().all(|c| c.interaction.on_activate.is_none()));
    }

    #[test]
    fn responsive_cells_seed_at_a_column_share_and_wrap_at_the_minimum() {
        // Contract §6: the root is a responsive auto-fit grid. `columns` is an
        // upper bound, so each cell seeds at `1/columns` and the size-adjusted
        // minimum width is what forces a wrap; there is no fixed row scaffold.
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = CardToggleGroupSpec::new(options()).with_columns(3);
        let node = card_toggle_group(&spec, &ctx, None);
        assert_eq!(node.style.descriptor.layout.direction, LayoutDirection::Row);
        assert!(
            node.style.flex_wrap,
            "the grid wraps when cards hit the floor"
        );
        assert!(node.style.fill_width);
        assert_eq!(node.children.len(), 3, "one wrapping row holds every cell");
        let min_width = rem_to_px(CardToggleGroupSpec::min_width_rem(
            ctx.resolve_size(spec.size, spec.size_role),
        ));
        for cell in &node.children {
            assert_eq!(cell.style.flex_basis_pct, Some(1.0 / 3.0 - GRID_GAP_SHARE));
            assert_eq!(cell.style.flex_grow, Some(1.0));
            assert_eq!(cell.style.min_width, Some(min_width));
        }

        // The 1–4 clamp still bounds the share.
        let clamped = card_toggle_group(
            &CardToggleGroupSpec::new(options()).with_columns(9),
            &ctx,
            None,
        );
        assert_eq!(
            clamped.children[0].style.flex_basis_pct,
            Some(1.0 / 4.0 - GRID_GAP_SHARE)
        );
    }
}
