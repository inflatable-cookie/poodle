//! Progress — real GPUI component backed by ProgressSpec.
//!
//! Implements the progress contract with animated indeterminate state.

use std::time::Duration;

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::ProgressSpec;

use crate::theme_ext::{color_mix, resolve_color};

/// A real GPUI progress bar component backed by `ProgressSpec`.
pub struct Progress {
    spec: ProgressSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for Progress {
    type Target = ProgressSpec;
    fn deref(&self) -> &ProgressSpec { &self.spec }
}

impl Progress {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: ProgressSpec::new(), theme: theme.clone() }
    }

    pub fn from_spec(spec: ProgressSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: f64) -> Self { self.spec.value = Some(v); self }
    pub fn max(mut self, v: f64) -> Self { self.spec.max = v; self }
    pub fn indeterminate(mut self, v: bool) -> Self { self.spec.is_indeterminate = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }
    pub fn value_text(mut self, v: impl Into<String>) -> Self { self.spec.value_text = Some(v.into()); self }

}

impl IntoElement for Progress {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let accent = resolve_color(theme, spec.indicator_fill_token());
        let surface = resolve_color(theme, "semantic.color.background.surface");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        // Contract: track bg = color-mix(surface 96%, text-primary)
        let track_bg = color_mix(surface, text_primary, 0.96);

        let progress = spec.normalized_progress();

        // Contract: track height 0.5rem (8px), radius 999px
        let mut track = div()
            .w_full()
            .h(px(8.0))
            .rounded(px(999.0))
            .bg(track_bg)
            .overflow_hidden();

        match progress {
            Some(p) => {
                let pct = p.clamp(0.0, 1.0) as f32;
                // Determinate: solid accent fill at percentage width
                track = track.child(
                    div()
                        .h_full()
                        .rounded(px(999.0))
                        .bg(accent)
                        .w(relative(pct)),
                );
            }
            None => {
                // Indeterminate: animated bar that slides back and forth
                // Contract: width 40%, animation translateX(-100%) to translateX(250%), 1.2s ease-in-out infinite
                track = track.child(
                    div()
                        .h_full()
                        .rounded(px(999.0))
                        .bg(accent)
                        .w(relative(0.4))
                        .with_animation(
                            "progress-indeterminate",
                            Animation::new(Duration::from_millis(1200))
                                .repeat()
                                .with_easing(gpui::ease_in_out),
                            |el, delta| {
                                // Slide from -40% to 140% of track width
                                let offset = -0.4 + delta * 1.8;
                                el.ml(relative(offset))
                            },
                        ),
                );
            }
        }

        track.into_any_element()
    }
}
