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

use std::sync::Arc;

use poodle_headless::duration::{
    adjust_duration_segment, duration_total_seconds, type_duration_digit, DurationSegment,
    DurationValue,
};
use poodle_node::{
    CrossAxisAlignment, LayoutDirection, LayoutSizing, MainAxisAlignment, Node, StylePatch,
};
use poodle_specs::{ControlSize, DurationInputSpec};

use crate::color::with_alpha;
use crate::context::RenderContext;
use crate::presentation::{control_space_x_rem, rem_to_px};

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

/// Label font size in rem per size (contract section 8).
fn label_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.5,
        _ => 0.5625,
    }
}

/// Host callback. `on_change` reports every segment plus the total, after the
/// carry rules have been applied — the payload the contract documents.
#[derive(Default)]
pub struct DurationInputHandlers {
    pub on_change: Option<Arc<dyn Fn(u32, u32, u32, u32) + Send + Sync>>,
}

pub fn duration_input(spec: &DurationInputSpec, ctx: &RenderContext<'_>) -> Node {
    duration_input_with_handlers(spec, ctx, DurationInputHandlers::default())
}

/// Render a duration input whose segments take keys.
///
/// Each segment is separately focusable, which is what the contract's Tab and
/// Shift+Tab rows describe: focus moves between segments, and the arrows and
/// digits act on whichever holds it. The carry rules come from
/// `poodle_headless::duration`, a port of the same `duration.ts` the web target
/// uses, so a keystroke cannot mean two different things on two targets.
pub fn duration_input_with_handlers(
    spec: &DurationInputSpec,
    ctx: &RenderContext<'_>,
    handlers: DurationInputHandlers,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);

    // ── Token resolution ──
    let fill = ctx.theme().resolve_color(spec.fill_token());
    // Border resolves through the spec: ValidationState::Invalid →
    // color.status.danger, otherwise color.border.default (contract §4/§8).
    let border_color = ctx.theme().resolve_color(spec.border_token());
    let text_primary = ctx.theme().resolve_color(spec.text_color_token());
    let text_secondary = ctx.theme().resolve_color(spec.text_secondary_token());
    let radius = ctx.theme().resolve_radius(spec.radius_token());
    // Segment-focus highlight = color-mix(accent-base 12%, transparent).
    let accent = ctx.theme().resolve_color("color.accent.base");
    let segment_focus_bg = with_alpha(accent, accent.3 * 0.12);

    // ── Sizing (contract section 8) ──
    let pad_y = rem_to_px(root_pad_y_rem(effective_size));
    let base_pad_x = rem_to_px(control_space_x_rem(density)) - 3.0;
    let pad_x = base_pad_x + rem_to_px(root_pad_x_offset_rem(effective_size));
    let field_w = rem_to_px(field_width_rem(effective_size));
    let field_font = match effective_size {
        ControlSize::Xs => rem_to_px(0.75),
        // GPUI's text raster lands between whole-pixel sizes at this tier;
        // 13.25px matches the old glyph coverage while the line box stays 13px.
        ControlSize::Sm | ControlSize::Md => rem_to_px(0.828125),
        ControlSize::Lg => rem_to_px(0.9375),
        ControlSize::Xl => rem_to_px(1.0),
    };
    let field_line_box = match effective_size {
        ControlSize::Sm | ControlSize::Md => rem_to_px(0.8125),
        _ => field_font,
    };
    let raster_phase_adjust = matches!(effective_size, ControlSize::Sm | ControlSize::Md);
    let label_font = rem_to_px(label_font_rem(effective_size));
    let border_width = rem_to_px(0.0625); // Contract: 0.0625rem solid
                                          // The GPUI flex implementation includes a two-pixel separator edge in
                                          // each inter-segment gap; the node backend's intrinsic text boxes do not.
    let segment_gap = rem_to_px(0.125) + 1.0; // Contract gap + backend compensation
    let segment_pad = rem_to_px(0.125); // Contract: segment padding 0.125rem
    let segment_radius = rem_to_px(0.1875); // Contract: 0.1875rem
    let label_gap = rem_to_px(0.125); // label→field gap inside a segment

    // The segments' keys act on this, so it has to be parsed before the
    // builder closes over it.
    let current_value = parse_duration_value(spec.value.as_deref());

    // ── Segment builder ──
    // Column of label + field. Typography is inherited from the preview's
    // Inter root, matching the old GPUI tier.
    let build_segment =
        |unit_label: &str, value_text: &str, unit: Option<DurationSegment>| -> Node {
            // Label: per-size font, secondary, line-height 1, tracking 0.05em.
            let mut label = Node::text(unit_label);
            {
                let s = &mut label.style;
                s.text_size = Some(label_font);
                // Node backend interprets line-height as a font-size multiple.
                s.line_height = Some(1.0);
                s.descriptor.text_color = Some(text_secondary);
                s.letter_spacing_em = Some(0.05);
            }

            // Field: per-size width, digit font, centered, line-height 1.
            let mut field = Node::text(value_text);
            {
                let s = &mut field.style;
                s.descriptor.layout.width = LayoutSizing::Fixed(field_w);
                // Keep the segment's 13px line box while the fractional glyph
                // size above reproduces the old GPUI coverage.
                s.descriptor.layout.height = LayoutSizing::Fixed(field_line_box);
                s.text_size = Some(field_font);
                // Node backend interprets line-height as a font-size multiple.
                s.line_height = Some(1.0);
                s.descriptor.text_color = Some(text_primary);
                s.text_weight = Some(600);
                s.text_align = Some(poodle_node::TextAlign::Center);
                if raster_phase_adjust {
                    s.descriptor.layout.spacing.margin.top = 0.5;
                    s.descriptor.layout.spacing.margin.bottom = -0.5;
                }
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
            // Keys act on the focused segment. Disabled inputs stay inert, and a
            // segment with no handler stays a plain visual rather than advertising
            // a focus stop that does nothing.
            if let (Some(unit), Some(on_change), false) =
                (unit, handlers.on_change.clone(), spec.is_disabled)
            {
                seg.interaction.focusable = true;
                let current = current_value;
                let max_hours = spec.max_hours;
                seg.interaction.on_edit_key = Some(Arc::new(move |key: &str, _mods| {
                    let next = match key {
                        "up" => adjust_duration_segment(current, unit, 1, max_hours),
                        "down" => adjust_duration_segment(current, unit, -1, max_hours),
                        key => {
                            let mut chars = key.chars();
                            match (chars.next(), chars.next()) {
                                (Some(c), None) if c.is_ascii_digit() => type_duration_digit(
                                    current,
                                    unit,
                                    c.to_digit(10).expect("checked ascii digit"),
                                    max_hours,
                                ),
                                // Not ours: Tab and Enter have to reach the host.
                                _ => return,
                            }
                        }
                    };
                    if next == current {
                        return;
                    }
                    on_change(
                        next.hours,
                        next.minutes,
                        next.seconds,
                        duration_total_seconds(next),
                    );
                }));
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
            // Mirrors the GPUI separator's `.flex_1()`: occupy the remaining
            // field row after the label spacer so the colon is vertically
            // centered with the digits.
            s.flex_grow = Some(1.0);
            s.text_size = Some(field_font);
            // Node backend interprets line-height as a font-size multiple.
            s.line_height = Some(1.0);
            s.descriptor.text_color = Some(text_secondary);
            s.text_weight = Some(600);
            if raster_phase_adjust {
                s.descriptor.layout.spacing.margin.top = 0.5;
                s.descriptor.layout.spacing.margin.bottom = -0.5;
            }
        }
        let colon = Node::text(":");
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
    let mut segments = Node::container();
    segments.style.descriptor.layout.direction = LayoutDirection::Row;
    segments.style.descriptor.layout.spacing.gap = segment_gap;

    // The GPUI tier wraps the segments in one row child. Keeping that extra
    // box preserves its intrinsic cross-axis sizing and vertical rhythm.
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.top = pad_y;
        // GPUI's border-box sizing leaves the node backend one physical pixel
        // short versus the old tier; the extra CSS pixel restores the exact
        // control height without moving the segment content.
        pad.bottom = pad_y + 1.0;
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
    segments = segments
        .child(build_segment("H", &hours_str, Some(DurationSegment::Hours)))
        .child(build_separator())
        .child(build_segment(
            "M",
            &minutes_str,
            Some(DurationSegment::Minutes),
        ));

    // Optional seconds
    if spec.show_seconds {
        segments = segments.child(build_separator());
        segments = segments.child(build_segment(
            "S",
            &seconds_str,
            Some(DurationSegment::Seconds),
        ));
    }
    root = root.child(segments);

    // ── Disabled state ──
    if spec.is_disabled {
        root.style.descriptor.opacity = ctx
            .theme()
            .resolve_opacity(spec.disabled_opacity_token());
        root.interaction.disabled = true;
    } else {
        // Segment-focus highlight on hover (focus tracking is host-owned;
        // hover is the closest rendered approximation of the accent-12% band).
        root.style.hover = Some(StylePatch {
            background: Some(segment_focus_bg),
            border_color: None,
            text_color: None,
            opacity: None,
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
/// The same parse as `parse_duration`, as numbers the carry rules can use.
fn parse_duration_value(value: Option<&str>) -> DurationValue {
    let (h, m, sec) = parse_duration(value);
    DurationValue {
        hours: h.trim().parse().unwrap_or(0),
        minutes: m.trim().parse().unwrap_or(0),
        seconds: sec.trim().parse().unwrap_or(0),
    }
}

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

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    type Reports = std::sync::Arc<std::sync::Mutex<Vec<(u32, u32, u32, u32)>>>;

    fn armed(spec: &DurationInputSpec) -> (Node, Reports) {
        let seen: Reports = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let sink = std::sync::Arc::clone(&seen);
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = duration_input_with_handlers(
            spec,
            &ctx,
            DurationInputHandlers {
                on_change: Some(Arc::new(move |h, m, s, total| {
                    sink.lock().unwrap().push((h, m, s, total))
                })),
            },
        );
        (node, seen)
    }

    /// Segments in tree order: hours, minutes, seconds.
    fn segments(node: &Node) -> Vec<&Node> {
        fn walk<'a>(n: &'a Node, out: &mut Vec<&'a Node>) {
            if n.interaction.on_edit_key.is_some() {
                out.push(n);
            }
            for c in &n.children {
                walk(c, out);
            }
        }
        let mut out = Vec::new();
        walk(node, &mut out);
        out
    }

    #[test]
    fn every_segment_is_its_own_focus_stop() {
        let (node, _) = armed(&DurationInputSpec::new().with_show_seconds(true));
        let segs = segments(&node);
        assert_eq!(segs.len(), 3, "hours, minutes, seconds");
        assert!(segs.iter().all(|s| s.interaction.focusable));

        let (node, _) = armed(&DurationInputSpec::new().with_show_seconds(false));
        assert_eq!(segments(&node).len(), 2, "no seconds segment to focus");
    }

    /// The arrows carry between segments, which is the whole reason the rules
    /// are shared with the web target rather than reimplemented.
    #[test]
    fn arrow_up_on_minutes_carries_into_hours() {
        let (node, seen) = armed(&DurationInputSpec::new().with_value("00:59:00"));
        let minutes = segments(&node)[1];
        (minutes.interaction.on_edit_key.as_ref().unwrap())(
            "up",
            poodle_node::NodeModifiers::default(),
        );
        assert_eq!(
            seen.lock().unwrap().last().copied(),
            Some((1, 0, 0, 3600)),
            "59 minutes + 1 is one hour, and the total comes with it"
        );
    }

    #[test]
    fn digits_shift_into_the_focused_segment() {
        let (node, seen) = armed(&DurationInputSpec::new().with_value("00:04:00"));
        let minutes = segments(&node)[1];
        (minutes.interaction.on_edit_key.as_ref().unwrap())(
            "5",
            poodle_node::NodeModifiers::default(),
        );
        assert_eq!(seen.lock().unwrap().last().copied(), Some((0, 45, 0, 2700)));
    }

    /// Tab and Enter have to reach the host, so a segment must not claim them.
    #[test]
    fn keys_that_are_not_ours_pass_through() {
        let (node, seen) = armed(&DurationInputSpec::new().with_value("00:04:00"));
        let minutes = segments(&node)[1];
        for key in ["tab", "enter", "escape", "left"] {
            (minutes.interaction.on_edit_key.as_ref().unwrap())(
                key,
                poodle_node::NodeModifiers::default(),
            );
        }
        assert!(seen.lock().unwrap().is_empty());
    }

    #[test]
    fn a_disabled_duration_input_takes_no_keys() {
        let (node, _) = armed(&DurationInputSpec::new().with_disabled(true));
        assert!(segments(&node).is_empty());
    }
}
