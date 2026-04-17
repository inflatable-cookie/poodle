//! DurationInput — real GPUI component backed by DurationInputSpec.
//!
//! Contract: inline-flex segments with labels, gap 0.125rem,
//! focus-within border change. No hover on root.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ControlDensity, ControlSize, DurationInputSpec, SemanticControlSizeRole, ValidationState,
};

use crate::presentation::{rem_to_px, resolve_semantic_size, size_padding_x_offset_rem};
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

        let base_pad = resolve_px(theme, "space.control.x");
        let control_padding_x = base_pad + px(rem_to_px(size_padding_x_offset_rem(effective_size)));
        let control_radius = resolve_radius(theme, spec.radius_token());
        let body_size = resolve_px(theme, spec.body_size_token());
        let caption_size = resolve_px(theme, "typography.caption.size");
        let radius_sm = resolve_radius(theme, "radius.control");

        let border = resolve_color(theme, spec.border_token());
        let surface_bg = resolve_color(theme, spec.fill_token());
        let text_primary = resolve_color(theme, spec.text_color_token());
        let text_secondary = resolve_color(theme, spec.text_secondary_token());
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());

        let display = spec.value.as_deref().unwrap_or(if spec.show_seconds {
            "00:00:00"
        } else {
            "00:00"
        });

        let labels = if spec.show_seconds {
            vec!["HRS", "MIN", "SEC"]
        } else {
            vec!["HRS", "MIN"]
        };

        // Contract: gap 0.125rem between segments
        let mut segments = div().flex().items_end().gap(px(rem_to_px(0.125)));

        let parts: Vec<&str> = display.split(':').collect();
        for (i, part) in parts.iter().enumerate() {
            if i > 0 {
                // Separator colon — contract: body-size, weight 600, line-height 1
                segments = segments.child(
                    div()
                        .text_size(body_size)
                        .line_height(px(rem_to_px(0.875)))
                        .text_color(text_secondary)
                        .font_weight(FontWeight::SEMIBOLD)
                        .child(":"),
                );
            }

            // Contract: segment = column with label + field
            let label = labels.get(i).unwrap_or(&"");
            let segment = div()
                .flex()
                .flex_col()
                .items_center()
                .gap(px(rem_to_px(0.125)))
                .p(px(rem_to_px(0.125)))
                .rounded(radius_sm)
                .child(
                    // Label: caption-size (≈11px), uppercase, secondary, line-height 1
                    div()
                        .text_size(caption_size)
                        .line_height(caption_size)
                        .text_color(text_secondary)
                        .child(label.to_string()),
                )
                .child(
                    // Field: 1.75rem wide, body size, centered, line-height 1
                    div()
                        .w(px(rem_to_px(1.75)))
                        .text_center()
                        .text_size(body_size)
                        .line_height(px(rem_to_px(0.875)))
                        .text_color(text_primary)
                        .child(part.to_string()),
                );

            segments = segments.child(segment);
        }

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("poodle-duration-input-{}", suffix)
        } else {
            "poodle-duration-input".to_string()
        };

        // Contract: padding 0.25rem control-x, border, radius, surface bg
        let mut wrapper = div()
            .id(SharedString::from(id_str))
            .focusable()
            .w_full()
            .py(px(rem_to_px(0.25))) // 0.25rem
            .px(control_padding_x)
            .rounded(control_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(border)
            .flex()
            .items_center()
            // Contract: focus-within = border + shadow ring at 28% opacity
            .focus(move |s| {
                s.border_color(focus_ring).shadow(vec![gpui::BoxShadow {
                    color: Hsla {
                        a: focus_ring.a * 0.28,
                        ..focus_ring
                    },
                    offset: point(px(0.0), px(0.0)),
                    blur_radius: px(0.0),
                    spread_radius: px(2.0),
                }])
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
