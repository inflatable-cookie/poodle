//! JsDurationInput — segmented duration entry backed by DurationInputSpec.
//!
//! Contract: `docs/contracts/components/duration-input.md`
//! Reference: `packages/svelte/primitives/src/DurationInput.svelte`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_components::{ControlSize, DurationInputSpec, ValidationState};

use crate::presentation::{control_space_x_rem, rem_to_px, resolve_semantic_size};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

/// Root vertical padding in rem per size (contract section 8).
fn root_pad_y_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.125,
        ControlSize::Sm => 0.1875,
        ControlSize::Md => 0.25,
        ControlSize::Lg => 0.3125,
        ControlSize::Xl => 0.375,
    }
}

/// Root horizontal padding offset in rem per size (contract section 8).
fn root_pad_x_offset_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => -0.125,
        ControlSize::Sm => -0.0625,
        ControlSize::Md => 0.0,
        ControlSize::Lg => 0.125,
        ControlSize::Xl => 0.1875,
    }
}

/// Field width in rem per size (contract section 8).
fn field_width_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.5,
        ControlSize::Sm | ControlSize::Md => 1.75,
        ControlSize::Lg => 2.0,
        ControlSize::Xl => 2.25,
    }
}

/// Field font size in rem per size (contract section 8).
fn field_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm | ControlSize::Md => 0.8125, // typography-body-size
        ControlSize::Lg => 0.9375,
        ControlSize::Xl => 1.0,
    }
}

/// Label font size in rem per size (contract section 8).
fn label_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.5,
        _ => 0.5625,
    }
}

/// Build a Jetstream duration input element from a DurationInputSpec.
///
/// Anatomy (from contract):
/// ```text
/// [Root .duration-input]  <div role="group">
///   ├── [Hours Segment .duration-input__segment]
///   │   ├── [Label]  "h"
///   │   └── [Field]  <input>
///   ├── [Separator]  ":"
///   ├── [Minutes Segment]
///   │   ├── [Label]  "m"
///   │   └── [Field]  <input>
///   ├── [Separator]  ":" (conditional)
///   └── [Seconds Segment]  (conditional, when showSeconds)
///       ├── [Label]  "s"
///       └── [Field]  <input>
/// ```
pub fn js_duration_input(
    spec: &DurationInputSpec,
    theme: &JetstreamThemeProvider,
) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // ── Token resolution ──
    let fill: Color = resolve_color(theme, spec.fill_token()).into();
    let border_color: Color = resolve_color(theme, spec.border_token()).into();
    let text_primary: Color = resolve_color(theme, spec.text_color_token()).into();
    let text_secondary: Color = resolve_color(theme, spec.text_secondary_token()).into();
    let radius = resolve_radius(theme, spec.radius_token());

    // ── Sizing (contract section 8) ──
    let pad_y = rem_to_px(root_pad_y_rem(effective_size));
    let base_pad_x = rem_to_px(control_space_x_rem(spec.density));
    let pad_x = base_pad_x + rem_to_px(root_pad_x_offset_rem(effective_size));
    let field_w = rem_to_px(field_width_rem(effective_size));
    let field_font = rem_to_px(field_font_rem(effective_size));
    let label_font = rem_to_px(label_font_rem(effective_size));
    let border_width = rem_to_px(0.0625); // Contract: 0.0625rem solid
    let segment_gap = rem_to_px(0.125); // Contract: gap 0.125rem
    let segment_pad = rem_to_px(0.125); // Contract: segment padding 0.125rem
    let segment_radius = rem_to_px(0.1875); // Contract: 0.1875rem

    // ── Segment builder ──
    let build_segment = |unit_label: &str, value_text: &str| -> JsEl {
        // Label: uppercase unit abbreviation
        let label = ui_element::label(unit_label)
            .text_size(label_font)
            .text_color(text_secondary);

        // Field: centered numeric value
        let field = ui_element::label(value_text)
            .w(field_w)
            .text_size(field_font)
            .text_color(text_primary)
            .text_align_center();

        ui_element::div()
            .flex_col()
            .items_center()
            .gap(segment_gap)
            .p(segment_pad)
            .rounded(segment_radius)
            .child(label)
            .child(field)
    };

    // ── Separator builder ──
    let build_separator = || -> JsEl {
        ui_element::label(":")
            .text_size(field_font)
            .text_color(text_secondary)
            .text_weight(600) // Contract: font-weight 600
    };

    // Parse the value "HH:MM:SS" or display zeros
    let (hours_str, minutes_str, seconds_str) = parse_duration(spec.value.as_deref());

    // ── Root ──
    let mut root = ui_element::div()
        .flex_row()
        .items_end() // Contract: align-items: flex-end
        .gap(segment_gap)
        .pt(pad_y)
        .pb(pad_y)
        .pl(pad_x)
        .pr(pad_x)
        .rounded(radius)
        .bg(fill)
        .border(border_width)
        .border_color(border_color)
        .focusable();

    // Hours segment
    root = root.child(build_segment("H", &hours_str));

    // Separator
    root = root.child(build_separator());

    // Minutes segment
    root = root.child(build_segment("M", &minutes_str));

    // Optional seconds
    if spec.show_seconds {
        root = root.child(build_separator());
        root = root.child(build_segment("S", &seconds_str));
    }

    // ── Invalid state ──
    if spec.validation_state == ValidationState::Invalid {
        let danger: Color = resolve_color(theme, "color.status.danger").into();
        root = root.border_color(danger);
    }

    // ── Disabled state ──
    if spec.is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        root = root.opacity(opacity).disabled(true);
    }

    root
}

/// Parse a duration string "HH:MM:SS" or "HH:MM" into display strings.
fn parse_duration(value: Option<&str>) -> (String, String, String) {
    match value {
        Some(s) => {
            let parts: Vec<&str> = s.split(':').collect();
            let hours = parts.first().copied().unwrap_or("00");
            let minutes = parts.get(1).copied().unwrap_or("00");
            let seconds = parts.get(2).copied().unwrap_or("00");
            (
                hours.to_string(),
                minutes.to_string(),
                seconds.to_string(),
            )
        }
        None => ("00".to_string(), "00".to_string(), "00".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn default_has_two_segments_and_separator() {
        // showSeconds defaults to false, so: hours, separator, minutes = 3 children
        let spec = DurationInputSpec::new();
        let el = js_duration_input(&spec, &theme());
        assert_eq!(el.children.len(), 3, "hours + separator + minutes");
    }

    #[test]
    fn show_seconds_adds_extra_segment() {
        let spec = DurationInputSpec::new().with_show_seconds(true);
        let el = js_duration_input(&spec, &theme());
        // hours + sep + minutes + sep + seconds = 5
        assert_eq!(el.children.len(), 5);
    }

    #[test]
    fn disabled_has_reduced_opacity() {
        let spec = DurationInputSpec::new().with_disabled(true);
        let el = js_duration_input(&spec, &theme());
        assert!(el.style.disabled);
        assert!(el.style.opacity < 1.0);
    }

    #[test]
    fn invalid_state_changes_border() {
        let valid = js_duration_input(&DurationInputSpec::new(), &theme());
        let invalid = js_duration_input(
            &DurationInputSpec::new().with_validation_state(ValidationState::Invalid),
            &theme(),
        );
        assert_ne!(
            valid.style.border_color, invalid.style.border_color,
            "Invalid state should change border color"
        );
    }
}
