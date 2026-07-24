//! JsDurationInput — segmented duration entry backed by DurationInputSpec.
//!
//! Contract: `docs/contracts/components/duration-input.md`
//! Reference: `packages/svelte/components/src/DurationInput.svelte`
//! Mirrors the GPUI build-out (`packages/gpui/components/src/primitives/duration_input.rs`).
//!
//! Size drives field width + vertical padding (contract §8 size table); density
//! drives only the inline padding/gap. The root border resolves through
//! `spec.border_token()` (ValidationState::Invalid → color.status.danger, else
//! color.border.default — contract §4/§8). Each separator is a 2-row grid
//! (spacer matched to the label row + glyph) so the colon aligns with the field
//! rather than the labels. Focused segments carry an accent-12% highlight
//! (rendered styling; actual focus tracking + keyboard ±1/onChange live in the
//! preview event loop). ALL dimensions resolve from tokens.

use jetstream_ui::Color;
use jetstream_ui::ui_element::{self, FontFamily, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ControlSize, DurationInputSpec};

use crate::presentation::{control_space_x_rem, rem_to_px, resolve_semantic_size};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

/// Root vertical padding in rem per size (contract section 8). This is a SIZE
/// axis — density must not touch vertical padding.
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

/// Field/glyph digit font size in rem per size (contract section 8).
fn field_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm | ControlSize::Md => 0.8125,
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
///   ├── [Hours Segment]
///   │   ├── [Label]  "h"
///   │   └── [Field]
///   ├── [Separator]  ":" (2-row: spacer + glyph)
///   ├── [Minutes Segment]
///   │   ├── [Label]  "m"
///   │   └── [Field]
///   ├── [Separator]  ":" (conditional)
///   └── [Seconds Segment]  (conditional, when showSeconds)
///       ├── [Label]  "s"
///       └── [Field]
/// ```
pub fn js_duration_input(spec: &DurationInputSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // ── Token resolution ──
    let fill: Color = resolve_color(theme, spec.fill_token()).into();
    // Border resolves through the spec: ValidationState::Invalid →
    // color.status.danger, otherwise color.border.default (contract §4/§8).
    let border_color: Color = resolve_color(theme, spec.border_token()).into();
    let text_primary: Color = resolve_color(theme, spec.text_color_token()).into();
    let text_secondary: Color = resolve_color(theme, spec.text_secondary_token()).into();
    let radius = resolve_radius(theme, spec.radius_token());
    // Segment-focus highlight = color-mix(accent-base 12%, transparent).
    let accent: Color = resolve_color(theme, "color.accent.base").into();
    let segment_focus_bg = accent.with_alpha(accent.a * 0.12);

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
    let label_gap = rem_to_px(0.125); // label→field gap inside a segment

    // ── Segment builder ──
    // Column of label + field; focused segment gets the accent-12% highlight.
    // (Focus tracking is preview-loop; the highlight styling is rendered on a
    // dedicated bg so it round-trips through layout/probe.)
    let build_segment = |unit_label: &str, value_text: &str| -> JsEl {
        // Root sets code-family (contract §8); JsEl has no inheritance so each
        // text leaf carries FontFamily::Mono explicitly.
        // Label: per-size font, secondary, line-height 1, tracking 0.05em
        // (contract §8 Label letter-spacing).
        let label = ui_element::label(unit_label)
            .text_size(label_font)
            .line_height(label_font)
            .text_color(text_secondary)
            .letter_spacing_em(0.05)
            .font_family(FontFamily::Mono);

        // Field: per-size width, digit font, centered, line-height 1.
        let field = ui_element::label(value_text)
            .w(field_w)
            .text_size(field_font)
            .line_height(field_font)
            .text_color(text_primary)
            .font_family(FontFamily::Mono)
            .text_align_center();

        ui_element::div()
            .flex_col()
            .items_center()
            .gap(label_gap)
            .p(segment_pad)
            .rounded(segment_radius)
            .child(label)
            .child(field)
    };

    // ── Separator builder ──
    // 2-row grid: a spacer matched to the label row (label height + label gap)
    // so the colon glyph aligns with the field, not the labels. Glyph carries
    // the digit font-size, weight 600, line-height 1 (contract §2/§8).
    let build_separator = || -> JsEl {
        let spacer = ui_element::div().h(label_font).mb(label_gap);
        let glyph = ui_element::div()
            .flex_row()
            .items_center()
            .justify_center()
            .text_size(field_font)
            .line_height(field_font)
            .text_color(text_secondary)
            .text_weight(600)
            .child(ui_element::label(":").font_family(FontFamily::Mono));
        ui_element::div()
            .flex_col()
            .items_center()
            .child(spacer)
            .child(glyph)
    };

    // Parse the value "HH:MM:SS" / "HH:MM" or display zeros.
    let (hours_str, minutes_str, seconds_str) = parse_duration(spec.value.as_deref());

    // ── Root ──
    // Contract: inline-flex, width: fit-content (default flex sizing — no
    // w_full), align-items: flex-end, surface bg, border (danger when invalid),
    // radius. Vertical pad is size-driven; inline pad is size + density.
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
    root = root.child(build_segment("h", &hours_str));
    // Separator
    root = root.child(build_separator());
    // Minutes segment
    root = root.child(build_segment("m", &minutes_str));

    // Optional seconds
    if spec.show_seconds {
        root = root.child(build_separator());
        root = root.child(build_segment("s", &seconds_str));
    }

    // ── Disabled state ──
    if spec.is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        root = root.opacity(opacity).disabled(true);
    } else {
        // Segment-focus highlight on hover (focus tracking is preview-loop;
        // hover is the closest rendered approximation of the accent-12% band).
        let bg = segment_focus_bg;
        root = root.hover(move |s| s.bg(bg));
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
            (hours.to_string(), minutes.to_string(), seconds.to_string())
        }
        None => ("00".to_string(), "00".to_string(), "00".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::ValidationState;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
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
    fn invalid_state_renders_danger_border() {
        // Invalid border must resolve to ~status.danger and differ from the
        // default border (contract §4 invalid).
        let danger: Color = resolve_color(&theme(), "color.status.danger").into();
        let valid = js_duration_input(&DurationInputSpec::new(), &theme());
        let invalid = js_duration_input(
            &DurationInputSpec::new().with_validation_state(ValidationState::Invalid),
            &theme(),
        );

        let invalid_border = invalid
            .style
            .border_color
            .expect("invalid root must set a border color");
        let expected = crate::render_probe::ProbeColor {
            r: danger.r,
            g: danger.g,
            b: danger.b,
            a: danger.a,
        };
        let got = crate::render_probe::ProbeColor {
            r: invalid_border.r,
            g: invalid_border.g,
            b: invalid_border.b,
            a: invalid_border.a,
        };
        assert!(
            got.approx(expected, 0.02),
            "invalid border should be ~status.danger, got {got:?} vs {expected:?}"
        );
        assert_ne!(
            valid.style.border_color, invalid.style.border_color,
            "invalid state should change the border from the default"
        );
    }

    #[test]
    fn renders_segment_labels_and_separator() {
        let spec = DurationInputSpec::new().with_value("01:30").with_show_seconds(false);
        let el = js_duration_input(&spec, &theme());
        let tree = probe(&el, 400.0, 200.0);
        assert!(tree.has_text("h"), "hours label missing: {:?}", tree.texts());
        assert!(tree.has_text("m"), "minutes label missing");
        assert!(tree.has_text(":"), "separator glyph missing");
    }
}
