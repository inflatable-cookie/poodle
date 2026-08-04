//! RadioGroup — one choice from a list.
//!
//! Contract: `docs/contracts/components/radio-group.md`
//! Ported from: `packages/jetstream/components/src/radio_group.rs`.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment, Node,
    NodeRole,
};
use poodle_specs::{ControlDensity, ControlSize, Orientation, RadioGroupSpec};

use crate::color::hex_color;
use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};

/// Indicator (outer circle) size — contract §8 table over the icon-md token.
fn indicator_size_px(size: ControlSize, icon_md_px: f32) -> f32 {
    match size {
        ControlSize::Xs => icon_md_px - rem_to_px(0.125),
        ControlSize::Sm => icon_md_px,
        ControlSize::Md => rem_to_px(1.125),
        ControlSize::Lg => icon_md_px + rem_to_px(0.375),
        ControlSize::Xl => icon_md_px + rem_to_px(0.625),
    }
}

/// Dot (inner circle) size — contract §8 ratios over icon-md; md explicit.
fn dot_size_px(size: ControlSize, icon_md_px: f32) -> f32 {
    match size {
        ControlSize::Xs => icon_md_px * 0.40,
        ControlSize::Sm => icon_md_px * 0.45,
        ControlSize::Md => rem_to_px(0.5),
        ControlSize::Lg => icon_md_px * 0.55,
        ControlSize::Xl => icon_md_px * 0.60,
    }
}

fn circle(node: &mut Node, diameter: f32) {
    let s = &mut node.style;
    s.descriptor.layout.width = LayoutSizing::Fixed(diameter);
    s.descriptor.layout.height = LayoutSizing::Fixed(diameter);
    let r = diameter * 0.5;
    s.descriptor.corner_radii.top_left = r;
    s.descriptor.corner_radii.top_right = r;
    s.descriptor.corner_radii.bottom_right = r;
    s.descriptor.corner_radii.bottom_left = r;
}

pub fn radio_group(
    spec: &RadioGroupSpec,
    theme: &dyn ThemeProvider,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let icon_md = theme.resolve_space("size.icon.md");
    let indicator_size = indicator_size_px(effective_size, icon_md);
    let dot_size = dot_size_px(effective_size, icon_md);
    let border_width = rem_to_px(0.0625);

    // Density override wins over the orientation gap.
    let group_gap = match spec.density {
        ControlDensity::Compact => theme.resolve_space("space.stack.sm"),
        ControlDensity::Comfortable => theme.resolve_space("space.stack.lg"),
        ControlDensity::Default => theme.resolve_space(spec.option_gap_token()),
    };
    let item_gap = theme.resolve_space("space.inline.sm");

    // Custom hex wins over accent. Colour-space note as in checkbox: the hex
    // lands in sRGB and converts at the backend edge — the old tier passed it
    // raw; divergence pinned in the parity suite.
    let accent = spec
        .selected_color
        .as_deref()
        .and_then(hex_color)
        .unwrap_or_else(|| theme.resolve_color("color.accent.base"));
    let border = theme.resolve_color("color.border.default");
    let text_color = theme.resolve_color("color.text.primary");
    let selected_value = spec.value.as_deref().or(spec.default_value.as_deref());
    let disabled_opacity = theme.resolve_opacity("state.opacity.disabled");

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = match spec.orientation {
            Orientation::Horizontal => LayoutDirection::Row,
            Orientation::Vertical => LayoutDirection::Column,
        };
        s.descriptor.layout.spacing.gap = group_gap;
    }

    for option in &spec.options {
        let is_selected = selected_value == Some(option.value.as_str());
        let indicator_color = if is_selected { accent } else { border };
        let indicator_bg = theme.resolve_color("color.background.surface");

        let mut indicator = Node::container();
        circle(&mut indicator, indicator_size);
        {
            let s = &mut indicator.style;
            s.descriptor.background = Some(indicator_bg);
            s.descriptor.border.width = border_width;
            s.descriptor.border.color = indicator_color;
            // Explicit Row (see switch.rs): the old tier got taffy's Row default.
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        }

        if is_selected {
            let mut dot = Node::container();
            circle(&mut dot, dot_size);
            dot.style.descriptor.background = Some(accent);
            indicator = indicator.child(dot);
        }

        let option_disabled = spec.is_disabled || option.is_disabled;

        let mut row = Node::container();
        {
            let s = &mut row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = item_gap;
            s.descriptor.cursor = if option_disabled {
                CursorHint::Default
            } else {
                CursorHint::Pointer
            };
        }
        if !option_disabled {
            row.interaction.focusable = true;
        }
        row = row.child(indicator);

        let mut label = Node::text(&option.label);
        label.style.descriptor.text_color = Some(text_color);
        label.style.text_size = Some(font_size);
        row = row.child(label);

        // Per-option disabled dims that row only.
        if option.is_disabled {
            row.style.descriptor.opacity = disabled_opacity;
        }

        if let (false, Some(handler)) = (option_disabled, &on_change) {
            let handler = Arc::clone(handler);
            let value = option.value.clone();
            row.interaction.on_activate = Some(Arc::new(move || handler(&value)));
        }

        el = el.child(row);
    }

    if spec.is_disabled {
        el.style.descriptor.opacity = disabled_opacity;
    }

    if let Some(label) = spec.aria_label.as_deref() {
        el.a11y.label = Some(label.to_string());
    }
    el.a11y.role = Some(NodeRole::RadioGroup);
    el
}
