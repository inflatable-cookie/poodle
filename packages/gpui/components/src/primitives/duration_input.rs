//! DurationInput — real GPUI component backed by DurationInputSpec.
//!
//! Contract: inline-flex segments with labels, gap 0.125rem,
//! focus-within border change. No hover on root.

use gpui::*;
use flint_gpui::GpuiThemeProvider;
use flint_primitives::{DurationInputSpec, ValidationState};

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
    fn deref(&self) -> &DurationInputSpec { &self.spec }
}

impl DurationInput {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: DurationInputSpec::new(), theme: theme.clone(), id_suffix: None, on_change: None }
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
    pub fn value(mut self, v: impl Into<String>) -> Self { self.spec.value = Some(v.into()); self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn validation_state(mut self, v: ValidationState) -> Self { self.spec.validation_state = v; self }
    pub fn show_seconds(mut self, v: bool) -> Self { self.spec.show_seconds = v; self }

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

        let control_padding_x = resolve_px(theme, "semantic.space.control.x");
        let control_radius = resolve_radius(theme, spec.radius_token());

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
        let mut segments = div()
            .flex()
            .items_end()
            .gap(px(2.0));

        let parts: Vec<&str> = display.split(':').collect();
        for (i, part) in parts.iter().enumerate() {
            if i > 0 {
                // Separator colon — contract: body-size, weight 600, line-height 1
                segments = segments.child(
                    div()
                        .text_size(px(14.0))
                        .line_height(px(14.0))
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
                .gap(px(2.0)) // 0.125rem
                .p(px(2.0))
                .rounded(px(3.0)) // 0.1875rem
                .child(
                    // Label: 0.5625rem, uppercase, secondary, line-height 1
                    div()
                        .text_size(px(9.0)) // 0.5625rem
                        .line_height(px(9.0))
                        .text_color(text_secondary)
                        .child(label.to_string()),
                )
                .child(
                    // Field: 1.75rem wide, body size, centered, line-height 1
                    div()
                        .w(px(28.0)) // 1.75rem
                        .text_center()
                        .text_size(px(14.0))
                        .line_height(px(14.0))
                        .text_color(text_primary)
                        .child(part.to_string()),
                );

            segments = segments.child(segment);
        }

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("flint-duration-input-{}", suffix)
        } else {
            "flint-duration-input".to_string()
        };

        // Contract: padding 0.25rem control-x, border, radius, surface bg
        let mut wrapper = div()
            .id(SharedString::from(id_str))
            .focusable()
            .py(px(4.0)) // 0.25rem
            .px(control_padding_x)
            .rounded(control_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(border)
            .flex()
            .items_center()
            // Contract: focus-within = border switches to focus ring color
            .focus(move |s| s.border_color(focus_ring))
            .child(segments);

        // ArrowUp/ArrowDown to increment/decrement total seconds
        if !spec.is_disabled {
            if let Some(handler) = self.on_change {
                let current_display = display.to_string();
                let show_seconds = spec.show_seconds;
                wrapper = wrapper.on_key_down(move |event: &KeyDownEvent, window, cx| {
                    let delta: i64 = if event.keystroke.key == "up" {
                        60 // increment by 1 minute
                    } else if event.keystroke.key == "down" {
                        -60 // decrement by 1 minute
                    } else {
                        return;
                    };
                    // Parse current value as total seconds
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
                    total_secs = (total_secs + delta).max(0);
                    let h = total_secs / 3600;
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
