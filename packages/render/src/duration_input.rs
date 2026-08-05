//! DurationInput — segmented duration entry (h : m [: s]).
//!
//! Contract: `docs/contracts/components/duration-input.md`
//! Ported from: `packages/jetstream/components/src/duration_input.rs`.
//!
//! Size drives field width + vertical padding (contract §8 size table);
//! density drives only the inline padding/gap. The root border resolves
//! through `spec.border_token()` (ValidationState::Invalid →
//! color.status.danger, else color.border.default). Each separator is a
//! 2-row column (spacer matched to the label row + glyph) so the colon
//! aligns with the field rather than the labels. Focus tracking + keyboard
//! ±1 / onChange are host-owned; hover renders the accent-12% band.

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, FontFamily, LayoutDirection, LayoutSizing, MainAxisAlignment, Node,
    StylePatch,
};
use poodle_specs::{ControlSize, DurationInputSpec};

use crate::color::with_alpha;
use crate::presentation::{control_space_x_rem, rem_to_px, resolve_semantic_size};

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

pub fn duration_input(spec: &DurationInputSpec, theme: &dyn ThemeProvider) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // ── Token resolution ──
    let fill = theme.resolve_color(spec.fill_token());
    // Border resolves through the spec: ValidationState::Invalid →
    // color.status.danger, otherwise color.border.default (contract §4/§8).
    let border_color = theme.resolve_color(spec.border_token());
    let text_primary = theme.resolve_color(spec.text_color_token());
    let text_secondary = theme.resolve_color(spec.text_secondary_token());
    let radius = theme.resolve_radius(spec.radius_token());
    // Segment-focus highlight = color-mix(accent-base 12%, transparent).
    let accent = theme.resolve_color("color.accent.base");
    let segment_focus_bg = with_alpha(accent, accent.3 * 0.12);

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
    // Column of label + field. No inheritance, so each text leaf carries
    // FontFamily::Mono explicitly (root sets code-family in the contract).
    let build_segment = |unit_label: &str, value_text: &str| -> Node {
        // Label: per-size font, secondary, line-height 1, tracking 0.05em.
        let mut label = Node::text(unit_label);
        {
            let s = &mut label.style;
            s.text_size = Some(label_font);
            s.line_height = Some(label_font);
            s.descriptor.text_color = Some(text_secondary);
            s.letter_spacing_em = Some(0.05);
            s.font_family = Some(FontFamily::Mono);
        }

        // Field: per-size width, digit font, centered, line-height 1.
        let mut field = Node::text(value_text);
        {
            let s = &mut field.style;
            s.descriptor.layout.width = LayoutSizing::Fixed(field_w);
            s.text_size = Some(field_font);
            s.line_height = Some(field_font);
            s.descriptor.text_color = Some(text_primary);
            s.font_family = Some(FontFamily::Mono);
            s.text_align = Some(poodle_node::TextAlign::Center);
        }

        let mut seg = Node::container();
        {
            let s = &mut seg.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = label_gap;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = segment_pad;
            pad.right = segment_pad;
            pad.top = segment_pad;
            pad.bottom = segment_pad;
            s.descriptor.corner_radii.top_left = segment_radius;
            s.descriptor.corner_radii.top_right = segment_radius;
            s.descriptor.corner_radii.bottom_right = segment_radius;
            s.descriptor.corner_radii.bottom_left = segment_radius;
        }
        seg.child(label).child(field)
    };

    // ── Separator builder ──
    // 2-row column: a spacer matched to the label row (label height + label
    // gap) so the colon glyph aligns with the field, not the labels. Glyph
    // carries the digit font-size, weight 600, line-height 1.
    let build_separator = || -> Node {
        let mut spacer = Node::container();
        {
            let s = &mut spacer.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.height = LayoutSizing::Fixed(label_font);
            s.descriptor.layout.spacing.margin.bottom = label_gap;
        }
        let mut glyph_wrap = Node::container();
        {
            let s = &mut glyph_wrap.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            s.text_size = Some(field_font);
            s.line_height = Some(field_font);
            s.descriptor.text_color = Some(text_secondary);
            s.text_weight = Some(600);
        }
        let mut colon = Node::text(":");
        colon.style.font_family = Some(FontFamily::Mono);
        let glyph = glyph_wrap.child(colon);

        let mut sep = Node::container();
        {
            let s = &mut sep.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        }
        sep.child(spacer).child(glyph)
    };

    // Parse the value "HH:MM:SS" / "HH:MM" or display zeros.
    let (hours_str, minutes_str, seconds_str) = parse_duration(spec.value.as_deref());

    // ── Root ──
    // Contract: inline-flex, width: fit-content (default flex sizing),
    // align-items: flex-end, surface bg, border (danger when invalid),
    // radius. Vertical pad is size-driven; inline pad is size + density.
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::End; // flex-end
        s.descriptor.layout.spacing.gap = segment_gap;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.top = pad_y;
        pad.bottom = pad_y;
        pad.left = pad_x;
        pad.right = pad_x;
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = border_width;
        s.descriptor.border.color = border_color;
    }
    root.interaction.focusable = true;

    // Hours segment / separator / minutes.
    let mut root = root
        .child(build_segment("h", &hours_str))
        .child(build_separator())
        .child(build_segment("m", &minutes_str));

    // Optional seconds
    if spec.show_seconds {
        root = root.child(build_separator());
        root = root.child(build_segment("s", &seconds_str));
    }

    // ── Disabled state ──
    if spec.is_disabled {
        root.style.descriptor.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
        root.interaction.disabled = true;
    } else {
        // Segment-focus highlight on hover (focus tracking is host-owned;
        // hover is the closest rendered approximation of the accent-12% band).
        root.style.hover = Some(StylePatch {
            background: Some(segment_focus_bg),
            border_color: None,
            text_color: None,
        });
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            root.a11y.label = Some(label.to_string());
        }
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
