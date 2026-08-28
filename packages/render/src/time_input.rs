//! TimeInput renderer.
//!
//! Contract: `docs/contracts/components/time-input.md`.
//!
//! There is no native input[type="time"], so the component renders a styled
//! text display of the current time value ("HH:MM" placeholder when empty —
//! the no-native-input substitute, contract §12).

use poodle_node::{CrossAxisAlignment, LayoutDirection, Node, NodeRole, TextChangeHandler};
use poodle_specs::TimeInputSpec;

use crate::context::RenderContext;
use crate::presentation::{
    rem_to_px, size_font_rem, size_height_offset_rem, size_padding_x_offset_rem,
};

pub fn time_input(spec: &TimeInputSpec, ctx: &RenderContext<'_>) -> Node {
    time_input_with_change(spec, ctx, None)
}

pub fn time_input_with_change(
    spec: &TimeInputSpec,
    ctx: &RenderContext<'_>,
    on_change: Option<TextChangeHandler>,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);

    // ── Token resolution ──
    let fill = ctx.theme().resolve_color(spec.fill_token());
    let border_color = ctx.theme().resolve_color(spec.border_token());
    let text_color = ctx.theme().resolve_color(spec.text_color_token());
    let placeholder_color = ctx.theme().resolve_color(spec.placeholder_color_token());
    let radius = ctx.theme().resolve_radius(spec.radius_token());

    // ── Sizing (contract section 8) ──
    let min_height = ctx.theme().resolve_space("size.control.height")
        + rem_to_px(size_height_offset_rem(effective_size));
    let pad_x = ctx.theme().resolve_space("space.control.x")
        + rem_to_px(size_padding_x_offset_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let border_width = rem_to_px(0.0625); // Contract: 0.0625rem solid

    // ── Display text ──
    let value = spec.current_value().unwrap_or_default();
    let has_value = spec.current_value().is_some();
    let display_color = if has_value {
        text_color
    } else {
        placeholder_color
    };

    // ── Build element ──
    let mut el = Node::input(value, "HH:MM");
    el.a11y.role = Some(NodeRole::TextInput);
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
        el.style.descriptor.opacity = ctx.theme().resolve_opacity(spec.disabled_opacity_token());
        el.interaction.disabled = true;
    } else {
        el.interaction.on_text_change = on_change;
    }

    // Named whether or not the host supplied one. A time field composed into a
    // date-time picker has no visible label of its own, and "unnamed text
    // input" is all a screen reader can say about it otherwise.
    el.a11y.label = Some(match spec.aria_label.as_deref() {
        Some(label) if !label.is_empty() => label.to_string(),
        _ => "Time".to_string(),
    });
    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn editable_time_input_reports_replacement_text() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let observed = Arc::new(Mutex::new(String::new()));
        let capture = Arc::clone(&observed);
        let node = time_input_with_change(
            &TimeInputSpec::new(),
            &ctx,
            Some(Arc::new(move |value| {
                *capture.lock().unwrap() = value.to_string();
            })),
        );

        assert!(matches!(node.kind, poodle_node::NodeKind::Input { .. }));
        node.interaction.on_text_change.unwrap()("09:30");
        assert_eq!(*observed.lock().unwrap(), "09:30");
    }

    #[test]
    fn disabled_time_input_is_not_editable() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let mut spec = TimeInputSpec::new();
        spec.is_disabled = true;
        let node = time_input_with_change(&spec, &ctx, Some(Arc::new(|_| {})));
        assert!(node.interaction.on_text_change.is_none());
    }
}
