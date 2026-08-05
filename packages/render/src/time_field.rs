//! TimeField — time input.
//!
//! Contract: `docs/contracts/components/time-field.md`
//! Ported from: `packages/jetstream/components/src/time_field.rs`.
//!
//! There is no native input[type="time"], so the component renders a styled
//! text display of the current time value ("HH:MM" placeholder when empty —
//! the no-native-input substitute, contract §12).

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, Node};
use poodle_specs::{ControlSize, TimeFieldSpec};

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
    size_height_offset_rem,
};

/// Font size in rem per size override (contract section 8).
fn time_font_size_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm | ControlSize::Md => 0.8125, // body-size baseline
        ControlSize::Lg => 0.9375,
        ControlSize::Xl => 1.0,
    }
}

/// Padding-x offset in rem per size (contract section 8).
fn time_padding_x_offset_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => -0.125,
        ControlSize::Sm => -0.0625,
        ControlSize::Md => 0.0,
        ControlSize::Lg => 0.125,
        ControlSize::Xl => 0.1875,
    }
}

pub fn time_field(spec: &TimeFieldSpec, theme: &dyn ThemeProvider) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // ── Token resolution ──
    let fill = theme.resolve_color(spec.fill_token());
    let border_color = theme.resolve_color(spec.border_token());
    let text_color = theme.resolve_color(spec.text_color_token());
    let placeholder_color = theme.resolve_color(spec.placeholder_color_token());
    let radius = theme.resolve_radius(spec.radius_token());

    // ── Sizing (contract section 8) ──
    let min_height = rem_to_px(control_height_rem(effective_size))
        + rem_to_px(size_height_offset_rem(effective_size));
    let base_pad_x = rem_to_px(control_space_x_rem(spec.density));
    let pad_x = base_pad_x + rem_to_px(time_padding_x_offset_rem(effective_size));
    let font_size = rem_to_px(time_font_size_rem(effective_size));
    let border_width = rem_to_px(0.0625); // Contract: 0.0625rem solid

    // ── Display text ──
    let display_text = spec.current_value().unwrap_or("HH:MM");
    let has_value = spec.current_value().is_some();
    let display_color = if has_value {
        text_color
    } else {
        placeholder_color
    };

    // ── Build element ──
    let mut el = Node::button(display_text);
    {
        let s = &mut el.style;
        s.min_height = Some(min_height);
        s.self_stretch = true; // Contract: stretches to parent width
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = border_width;
        s.descriptor.border.color = border_color;
        s.descriptor.text_color = Some(display_color);
        s.text_size = Some(font_size);
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    }
    el.interaction.focusable = true;

    // ── Disabled state ──
    // No pointer cursor on the idle field: the contract input has a text
    // caret, not a pointer, so the enabled branch leaves the cursor unchanged.
    if spec.is_disabled {
        el.style.descriptor.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
        el.interaction.disabled = true;
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            el.a11y.label = Some(label.to_string());
        }
    }
    el
}
