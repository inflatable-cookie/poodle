//! DurationInput — real GPUI component backed by DurationInputSpec.
//!
//! Contract: inline-flex segments with labels, gap 0.125rem,
//! focus-within border change. No hover on root.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ControlDensity, ControlSize, DurationInputSpec, SemanticControlSizeRole, ValidationState,
};

use crate::presentation::{
    duration_digit_font_rem, duration_field_width_rem, duration_gap_density_adjust_rem,
    duration_label_font_rem, duration_pad_x_density_adjust_rem, duration_pad_x_offset_rem,
    duration_pad_y_rem, rem_to_px, resolve_semantic_size,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI duration input (HH:MM:SS) component backed by `DurationInputSpec`.
pub struct DurationInput {
    spec: DurationInputSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for DurationInput {
    type Target = DurationInputSpec;
    fn deref(&self) -> &DurationInputSpec {
        &self.spec
    }
}

impl DurationInput {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: DurationInputSpec::new(),
            theme: theme.clone(),
            id_suffix: None,
            on_change: None,
        }
    }

    pub fn from_spec(spec: DurationInputSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_change: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.spec.value = Some(v.into());
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.spec.is_disabled = v;
        self
    }
    pub fn validation_state(mut self, v: ValidationState) -> Self {
        self.spec.validation_state = v;
        self
    }
    pub fn show_seconds(mut self, v: bool) -> Self {
        self.spec.show_seconds = v;
        self
    }
    pub fn size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn with_density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }

    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    /// Called when the duration value changes.
    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl IntoElement for DurationInput {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        // ── Size-driven geometry (contract §8 size table) ──────────
        // Root padding-inline = space.control.x + size-adjust + density-adjust.
        let base_pad = resolve_px(theme, "space.control.x");
        let control_padding_x = base_pad
            + px(rem_to_px(duration_pad_x_offset_rem(effective_size)))
            + px(rem_to_px(duration_pad_x_density_adjust_rem(spec.density)));
        // Root padding-block varies by size (NOT density — vertical pad is a
        // size axis per the contract's size/density split).
        let control_padding_y = px(rem_to_px(duration_pad_y_rem(effective_size)));
        // Inter-segment gap = 0.125rem base + density adjust.
        let segment_gap =
            px(rem_to_px(0.125 + duration_gap_density_adjust_rem(spec.density)));

        let control_radius = resolve_radius(theme, spec.radius_token());
        let segment_radius = px(rem_to_px(0.1875)); // contract: segment radius 0.1875rem

        // Field / glyph digit font: per-size override or the body-size token.
        let body_size = resolve_px(theme, spec.body_size_token());
        let digit_size = match duration_digit_font_rem(effective_size) {
            Some(rem) => px(rem_to_px(rem)),
            None => body_size,
        };
        // Label font: per-size rem from the contract size table.
        let label_size = px(rem_to_px(duration_label_font_rem(effective_size)));
        let field_width = px(rem_to_px(duration_field_width_rem(effective_size)));

        // Border resolves through the spec: ValidationState::Invalid →
        // color.status.danger, otherwise color.border.default (contract §4/§8).
        let border = resolve_color(theme, spec.border_token());
        let surface_bg = resolve_color(theme, spec.fill_token());
        let text_primary = resolve_color(theme, spec.text_color_token());
        let text_secondary = resolve_color(theme, spec.text_secondary_token());
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        // Segment-focus highlight: color-mix(accent-base 12%, transparent).
        let accent = resolve_color(theme, "color.accent.base");
        let segment_focus_bg = accent.opacity(accent.a * 0.12);

        let display = spec.value.as_deref().unwrap_or(if spec.show_seconds {
            "00:00:00"
        } else {
            "00:00"
        });

        // Contract/Svelte source labels are lowercase h/m/s (CSS uppercases);
        // GPUI has no text-transform, so emit the visual uppercase form.
        let labels = if spec.show_seconds {
            vec!["H", "M", "S"]
        } else {
            vec!["H", "M"]
        };

        // Contract: root align-items: stretch (flexbox default — no explicit
        // alignment), gap base 0.125rem (+ density).
        let mut segments = div().flex().gap(segment_gap);

        let parts: Vec<&str> = display.split(':').collect();
        for (i, part) in parts.iter().enumerate() {
            if i > 0 {
                // Separator: 2-row grid (spacer matched to label + glyph row)
                // so the colon aligns with the field, not the labels.
                // Contract: glyph font = digit-size, weight 600, line-height 1.
                segments = segments.child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .text_color(text_secondary)
                        .child(
                            // Spacer row: label height + label gap (0.125rem).
                            div()
                                .h(label_size)
                                .mb(px(rem_to_px(0.125))),
                        )
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_size(digit_size)
                                .line_height(digit_size)
                                .font_weight(FontWeight::SEMIBOLD)
                                .child(":"),
                        ),
                );
            }

            // Contract: segment = column with label + field; padding 0.125rem,
            // radius 0.1875rem; focused segment gets the accent 12% highlight.
            let label = labels.get(i).unwrap_or(&"");
            let mut segment = div()
                .id(SharedString::from(format!("poodle-duration-segment-{i}")))
                .flex()
                .flex_col()
                .items_center()
                .gap(px(rem_to_px(0.125)))
                .p(px(rem_to_px(0.125)))
                .rounded(segment_radius)
                .child(
                    // Label: per-size font, uppercase, secondary, line-height 1.
                    div()
                        .text_size(label_size)
                        .line_height(label_size)
                        .text_color(text_secondary)
                        .child(label.to_string()),
                )
                .child(
                    // Field: per-size width, digit font, centered, line-height 1.
                    // (Borderless text element — accepted Known Delta vs <input>.)
                    div()
                        .w(field_width)
                        .text_center()
                        .text_size(digit_size)
                        .line_height(digit_size)
                        .text_color(text_primary)
                        .child(part.to_string()),
                );

            // Segment-focus highlight covers label + field.
            if !spec.is_disabled {
                segment = segment.hover(move |s| s.bg(segment_focus_bg));
            }

            segments = segments.child(segment);
        }

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("poodle-duration-input-{}", suffix)
        } else {
            "poodle-duration-input".to_string()
        };

        // Contract: root inline-flex, width: fit-content, align-items: stretch,
        // padding (size-driven block + size/density inline), border, radius,
        // surface bg. Border is danger when invalid (resolved above).
        let mut wrapper = div()
            .id(SharedString::from(id_str))
            .focusable()
            .w_auto()
            .py(control_padding_y)
            .px(control_padding_x)
            .rounded(control_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(border)
            .flex()
            // align-items: stretch is the flexbox default (no explicit call).
            // Contract: focus-within = border + shadow ring at 28% opacity.
            // Invalid border persists until focus (matches Svelte).
            .focus(move |s| {
                s.border_color(focus_ring)
                    .shadow(crate::theme_ext::focus_ring_shadow(focus_ring))
            })
            .child(segments);

        // ArrowUp/ArrowDown to increment/decrement total seconds
        if !spec.is_disabled {
            if let Some(handler) = self.on_change {
                let current_display = display.to_string();
                let show_seconds = spec.show_seconds;
                let max_hours = spec.max_hours as i64;
                let min_total = spec.min_total_seconds as i64;
                let max_total = spec.max_total_seconds.map(|v| v as i64);
                wrapper = wrapper.on_key_down(move |event: &KeyDownEvent, window, cx| {
                    let delta: i64 = if event.keystroke.key == "up" {
                        60
                    } else if event.keystroke.key == "down" {
                        -60
                    } else {
                        return;
                    };
                    let parts: Vec<&str> = current_display.split(':').collect();
                    let mut total_secs: i64 = 0;
                    if let Some(h) = parts.first().and_then(|s| s.parse::<i64>().ok()) {
                        total_secs += h * 3600;
                    }
                    if let Some(m) = parts.get(1).and_then(|s| s.parse::<i64>().ok()) {
                        total_secs += m * 60;
                    }
                    if let Some(s) = parts.get(2).and_then(|s| s.parse::<i64>().ok()) {
                        total_secs += s;
                    }
                    total_secs = (total_secs + delta).max(min_total);
                    if let Some(cap) = max_total {
                        total_secs = total_secs.min(cap);
                    }
                    let h = (total_secs / 3600).min(max_hours);
                    let m = (total_secs % 3600) / 60;
                    let s = total_secs % 60;
                    let new_val = if show_seconds {
                        format!("{:02}:{:02}:{:02}", h, m, s)
                    } else {
                        format!("{:02}:{:02}", h, m)
                    };
                    handler(&new_val, window, cx);
                });
            }
        }

        if spec.is_disabled {
            wrapper = wrapper
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        wrapper.into_any_element()
    }
}
