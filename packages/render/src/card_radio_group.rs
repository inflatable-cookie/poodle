//! CardRadioGroup — single-select card group.
//!
//! Contract: `docs/contracts/components/card-radio-group.md`
//! Ported from: `packages/jetstream/components/src/card_radio_group.rs`.
//!
//! Each option composes the `card` primitive (interactive, selected when
//! chosen) so the selected fill/border/focus ring come from Card's own
//! token-resolved treatment. The body carries a header row (radio indicator
//! + title) and an optional description.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment, Node,
    NodeRole, NodeToggled,
};
use poodle_specs::{CardRadioGroupSpec, CardSpec};

use crate::card::card;
use crate::presentation::{control_space_x_rem, rem_to_px, resolve_semantic_size};

pub fn card_radio_group(
    spec: &CardRadioGroupSpec,
    theme: &dyn ThemeProvider,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // Contract §7/§8 size scale — resolved through the spec helpers.
    let indicator_size = rem_to_px(CardRadioGroupSpec::indicator_size_rem(effective_size));
    let dot_size = rem_to_px(CardRadioGroupSpec::dot_size_rem(effective_size));
    let indicator_border = rem_to_px(spec.indicator_border_rem());
    let title_font = rem_to_px(CardRadioGroupSpec::title_font_rem(effective_size));
    let description_font = rem_to_px(CardRadioGroupSpec::description_font_rem(effective_size));

    // Density-driven grid gap; header gap density-fixed 0.5rem; body rhythm.
    let grid_gap = rem_to_px(control_space_x_rem(spec.density));
    let header_gap = rem_to_px(0.5);
    let body_gap = rem_to_px(0.25);

    let indicator_border_color = theme.resolve_color(spec.border_token());
    let accent = theme.resolve_color("color.accent.base");
    let dot_color = theme.resolve_color("color.text.inverse");
    let pill_radius = theme.resolve_radius("radius.pill");
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");

    let current_value = spec.current_value();

    // Root: wrapping grid container.
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.fill_width = true;
        s.flex_wrap = true;
        s.descriptor.layout.spacing.gap = grid_gap;
    }

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
        let mut option_card = card(&card_spec, theme, vec![body]);
        option_card.a11y.role = Some(NodeRole::RadioButton);
        option_card.a11y.toggled = Some(if is_selected {
            NodeToggled::True
        } else {
            NodeToggled::False
        });
        option_card.style.descriptor.layout.width = LayoutSizing::Grow;
        // Match the old GPUI option wrapper's `flex_1().min_w(0)`: a zero
        // basis makes every radio cell share the row before intrinsic labels
        // can claim width, and the zero minimum permits descriptions to wrap.
        option_card.style.flex_basis = Some(0.0);
        option_card.style.min_width = Some(0.0);

        if is_item_disabled {
            option_card.style.descriptor.opacity = theme.resolve_opacity("state.opacity.disabled");
        } else if let Some(handler) = &on_change {
            let handler = Arc::clone(handler);
            let value = option.value.clone();
            option_card.style.descriptor.cursor = CursorHint::Pointer;
            option_card.interaction.on_activate = Some(Arc::new(move || handler(&value)));
        }

        root = root.child(option_card);
    }

    if spec.is_disabled {
        root.style.descriptor.opacity = theme.resolve_opacity("state.opacity.disabled");
    }

    // Contract: the group of cards is a `radiogroup`.
    root.a11y.role = Some(NodeRole::RadioGroup);
    root
}
