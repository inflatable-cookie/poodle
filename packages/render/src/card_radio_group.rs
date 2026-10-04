//! CardRadioGroup — single-select card group.
//!
//! Contract: `docs/contracts/components/card-radio-group.md`
//! Ported from: `packages/jetstream/components/src/card_radio_group.rs`.
//!
//! Each option composes the `card` primitive (interactive, selected when
//! chosen) so the selected fill/border/focus ring come from Card's own
//! token-resolved treatment. The body carries a header row (radio indicator
//! + title) and an optional description.
//!
//! Selection runs through the shared ToggleGroup machine in single mode with
//! `allowDeactivation: false`, so a card re-emits on reselect (toggle-group
//! semantics, not native-radio semantics). Arrow navigation walks the enabled
//! options, wraps at the ends, selects the target, and moves real backend
//! focus — the Svelte `menuListNavigate` behaviour.

use std::sync::Arc;

use poodle_node::{
    CrossAxisAlignment, CursorHint, FocusRing, LayoutDirection, LayoutSizing, MainAxisAlignment,
    Node, NodeKey, NodeModifiers, NodeRole, NodeToggled,
};
use poodle_specs::{CardRadioGroupSpec, CardSpec};

use crate::card::card;
use crate::context::RenderContext;
use crate::presentation::{control_space_x_rem, rem_to_px};

/// Host-owned native interaction for one CardRadioGroup instance.
///
/// `instance_id` is the lifetime-stable scope. It is not the web form name and
/// the renderer never invents one from render order or option values; two
/// groups with identical options keep distinct focus identities when the host
/// scopes them apart.
#[derive(Clone)]
pub struct CardRadioGroupHandlers {
    pub instance_id: String,
    pub on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

impl CardRadioGroupHandlers {
    pub fn new(instance_id: impl Into<String>) -> Self {
        Self {
            instance_id: instance_id.into(),
            on_change: None,
        }
    }

    pub fn on_change(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_change = Some(handler);
        self
    }
}

fn option_id(value: &str) -> String {
    format!("card-radio:{value}")
}

fn option_focus_id(instance_scope: &str, value: &str) -> String {
    format!("card-radio:{instance_scope}:option:{value}")
}

/// A flex-growing grid cell. The card shares the row equally before intrinsic
/// labels can claim width (the web grid's `1fr`), and the zero minimum permits
/// descriptions to wrap.
fn flex1_cell() -> Node {
    let mut n = Node::container();
    let s = &mut n.style;
    // Explicit Row (see switch.rs).
    s.descriptor.layout.direction = LayoutDirection::Row;
    s.flex_grow = Some(1.0);
    s.flex_basis = Some(0.0);
    s.min_width = Some(0.0);
    n
}

/// Enabled option values in authored order — the roving order.
fn roving_values(spec: &CardRadioGroupSpec) -> Vec<String> {
    spec.options
        .iter()
        .filter(|option| !spec.is_disabled && !option.is_disabled)
        .map(|option| option.value.clone())
        .collect()
}

/// The single tab stop: the selected option when it is enabled, else the first
/// enabled option. Contract §6 roving tabindex.
fn tab_stop_value<'a>(spec: &'a CardRadioGroupSpec, roving: &'a [String]) -> Option<&'a str> {
    let selected = spec.current_value();
    if selected.is_some_and(|value| roving.iter().any(|candidate| candidate == value)) {
        selected
    } else {
        roving.first().map(String::as_str)
    }
}

/// Arrow navigation over the enabled options, wrapping at both ends. The
/// target is selected (re-emitting through `on_change`) and returned as the
/// focus identity the backend moves to. Both axes are live: the contract lists
/// ArrowRight/ArrowDown as "next" and ArrowLeft/ArrowUp as "previous", without
/// an orientation qualifier.
fn roving_key_handler(
    value: &str,
    roving: &[String],
    instance_scope: String,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
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
        if let Some(handler) = &on_change {
            handler(&target);
        }
        Some(option_focus_id(&instance_scope, &target))
    }))
}

/// Render a group without a caller-provided identity. Prefer
/// [`card_radio_group_with_handlers`] so duplicate instances keep distinct
/// focus handles; this entry point exists for static and single-instance
/// compositions.
pub fn card_radio_group(
    spec: &CardRadioGroupSpec,
    ctx: &RenderContext<'_>,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    card_radio_group_with_handlers(
        spec,
        ctx,
        CardRadioGroupHandlers {
            instance_id: "card-radio-group".to_string(),
            on_change,
        },
    )
}

pub fn card_radio_group_with_handlers(
    spec: &CardRadioGroupSpec,
    ctx: &RenderContext<'_>,
    handlers: CardRadioGroupHandlers,
) -> Node {
    let theme = ctx.theme();
    let instance_scope = handlers.instance_id.as_str();
    let on_change = handlers.on_change;
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);

    // Contract §7/§8 size scale — resolved through the spec helpers.
    let indicator_size = rem_to_px(CardRadioGroupSpec::indicator_size_rem(effective_size));
    let dot_size = rem_to_px(CardRadioGroupSpec::dot_size_rem(effective_size));
    let indicator_border = rem_to_px(spec.indicator_border_rem());
    let title_font = rem_to_px(CardRadioGroupSpec::title_font_rem(effective_size));
    let description_font = rem_to_px(CardRadioGroupSpec::description_font_rem(effective_size));

    // Density-driven grid gap; header gap density-fixed 0.5rem; body rhythm.
    let grid_gap = rem_to_px(control_space_x_rem(density));
    let header_gap = rem_to_px(0.5);
    let body_gap = rem_to_px(0.25);

    let indicator_border_color = theme.resolve_color(spec.border_token());
    let accent = theme.resolve_color("color.accent.base");
    let dot_color = theme.resolve_color("color.text.inverse");
    let pill_radius = theme.resolve_radius("radius.pill");
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let disabled_opacity = theme.resolve_opacity("state.opacity.disabled");
    let focus_ring = FocusRing {
        color: theme.resolve_color("color.accent.focusRing"),
        width: theme.resolve_border_width("border.width.focus"),
        // Contract §8 option focus: outline-offset 0.125rem.
        offset: rem_to_px(0.125),
    };

    let current_value = spec.current_value();
    let roving = roving_values(spec);
    let tab_stop = tab_stop_value(spec, &roving);

    // Root grid: options lay out in rows of `column_count()` cells (contract
    // §7 `repeat(var(--columns), 1fr)`); a short final row is padded with flex
    // spacers so card widths stay aligned across rows.
    let cols = spec.column_count();
    let mut cells: Vec<Node> = Vec::new();

    for option in &spec.options {
        let is_selected = current_value == Some(option.value.as_str());
        let is_item_disabled = spec.is_disabled || option.is_disabled;

        // Radio indicator: border-only unchecked; accent fill + dot checked.
        let mut indicator = Node::container();
        {
            let s = &mut indicator.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.width = LayoutSizing::Fixed(indicator_size);
            s.descriptor.layout.height = LayoutSizing::Fixed(indicator_size);
            let c = &mut s.descriptor.corner_radii;
            c.top_left = pill_radius;
            c.top_right = pill_radius;
            c.bottom_right = pill_radius;
            c.bottom_left = pill_radius;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            s.descriptor.border.width = indicator_border;
            if is_selected {
                s.descriptor.background = Some(accent);
                s.descriptor.border.color = accent;
            } else {
                s.descriptor.border.color = indicator_border_color;
            }
        }
        let indicator = if is_selected {
            let mut dot = Node::container();
            {
                let s = &mut dot.style;
                // Explicit Row (see switch.rs).
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.width = LayoutSizing::Fixed(dot_size);
                s.descriptor.layout.height = LayoutSizing::Fixed(dot_size);
                let c = &mut s.descriptor.corner_radii;
                c.top_left = pill_radius;
                c.top_right = pill_radius;
                c.bottom_right = pill_radius;
                c.bottom_left = pill_radius;
                s.descriptor.background = Some(dot_color);
            }
            indicator.child(dot)
        } else {
            indicator
        };

        // Header row: indicator + title.
        let mut header = Node::container();
        {
            let s = &mut header.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = header_gap;
        }
        let mut title = Node::text(&option.label);
        title.style.descriptor.text_color = Some(text_primary);
        title.style.text_size = Some(title_font);
        title.style.text_weight = Some(600);
        let header = header.child(indicator).child(title);

        // Card body: header row + optional description.
        let mut body = Node::container();
        {
            let s = &mut body.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = body_gap;
        }
        let mut body = body.child(header);
        if let Some(description) = &option.description {
            let mut d = Node::text(description);
            d.style.descriptor.text_color = Some(text_secondary);
            d.style.text_size = Some(description_font);
            body = body.child(d);
        }

        // Compose the Card primitive — selected state owns the fill/border.
        let mut card_spec = CardSpec::new().interactive();
        if is_selected {
            card_spec = card_spec.selected();
        }
        let aria = option
            .aria_label
            .clone()
            .unwrap_or_else(|| option.label.clone());
        card_spec = card_spec.with_aria_label(aria);

        // These are mutually exclusive choices, so each card is a `radio`
        // carrying its own checked state (overriding Card's `button`).
        let mut option_card = card(&card_spec, ctx, vec![body]);
        option_card.id = Some(option_id(&option.value));
        option_card.runtime_id = Some(option_focus_id(instance_scope, &option.value));
        option_card.a11y.role = Some(NodeRole::RadioButton);
        option_card.a11y.selected = Some(is_selected);
        option_card.a11y.toggled = Some(if is_selected {
            NodeToggled::True
        } else {
            NodeToggled::False
        });
        option_card.a11y.label = Some(
            option
                .aria_label
                .clone()
                .unwrap_or_else(|| option.label.clone()),
        );
        option_card.style.descriptor.layout.width = LayoutSizing::Grow;

        if is_item_disabled {
            // Group-level disabled reaches every option through
            // `is_item_disabled`; the option dims once here. The root itself
            // never dims (the web contract has no group opacity rule), so a
            // disabled group is not dimmed twice.
            option_card.style.descriptor.opacity = disabled_opacity;
            option_card.interaction.disabled = true;
            option_card.interaction.focusable = false;
            option_card.a11y.tab_index = Some(-1);
        } else {
            option_card.interaction.focusable = true;
            option_card.a11y.tab_index = Some(if tab_stop == Some(option.value.as_str()) {
                0
            } else {
                -1
            });
            option_card.style.focus_ring = Some(focus_ring);
            option_card.style.descriptor.cursor = CursorHint::Pointer;
            if let Some(handler) = &on_change {
                let handler = Arc::clone(handler);
                let value = option.value.clone();
                option_card.interaction.on_activate = Some(Arc::new(move || handler(&value)));
            }
            option_card.interaction.on_key = roving_key_handler(
                &option.value,
                &roving,
                instance_scope.to_string(),
                on_change.clone(),
            );
        }

        cells.push(flex1_cell().child(option_card));
    }

    // Assemble rows; pad a short final row with flex spacers.
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.fill_width = true;
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

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            root.a11y.label = Some(label.to_string());
        }
    }
    // Contract: the group of cards is a `radiogroup`.
    root.a11y.role = Some(NodeRole::RadioGroup);
    root
}
