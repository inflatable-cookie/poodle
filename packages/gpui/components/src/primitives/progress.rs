//! Progress — real GPUI component backed by ProgressSpec.
//!
//! Implements the progress contract with animated indeterminate state.

use std::time::Duration;

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ControlDensity, ControlSize, ProgressSpec, SemanticControlSizeRole};

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{color_mix, resolve_color};

/// Mix a color toward white at `ratio` (proportion of `color` retained),
/// matching CSS `color-mix(in srgb, color {ratio*100}%, white)`.
fn mix_white(color: Hsla, ratio: f32) -> Hsla {
    color_mix(color, gpui::white(), ratio)
}

/// A real GPUI progress bar component backed by `ProgressSpec`.
pub struct Progress {
    spec: ProgressSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for Progress {
    type Target = ProgressSpec;
    fn deref(&self) -> &ProgressSpec {
        &self.spec
    }
}

impl Progress {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: ProgressSpec::new(),
            theme: theme.clone(),
        }
    }

    pub fn from_spec(spec: ProgressSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: f64) -> Self {
        self.spec.value = Some(v);
        self
    }
    pub fn max(mut self, v: f64) -> Self {
        self.spec.max = v;
        self
    }
    pub fn indeterminate(mut self, v: bool) -> Self {
        self.spec.is_indeterminate = v;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn value_text(mut self, v: impl Into<String>) -> Self {
        self.spec.value_text = Some(v.into());
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
}

impl IntoElement for Progress {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        // Contract §8 Size Variants — height ladder owned by the spec.
        let bar_height = px(rem_to_px(ProgressSpec::min_height_rem(effective_size)));

        let accent = resolve_color(theme, spec.indicator_fill_token());
        // Contract §8 Root: track bg = color-mix(surface 96%, text-primary).
        // Mix ratio + both endpoints are spec-owned token methods.
        let surface = resolve_color(theme, spec.track_fill_token());
        let track_mix = resolve_color(theme, spec.track_mix_token());
        let track_bg = color_mix(surface, track_mix, spec.track_mix_ratio());

        // Contract §8 Indicator: linear-gradient(90deg,
        //   color-mix(accent 88%, white), accent).
        let gradient_lead = mix_white(accent, spec.indicator_gradient_accent_ratio());
        let indicator_fill = gpui::linear_gradient(
            90.0,
            gpui::linear_color_stop(gradient_lead, 0.0),
            gpui::linear_color_stop(accent, 1.0),
        );

        let progress = spec.normalized_progress();

        // Contract: track height per effective size, radius 999px
        let mut track = div()
            .w_full()
            .h(bar_height)
            .rounded(px(999.0))
            .bg(track_bg)
            .overflow_hidden();

        match progress {
            Some(p) => {
                let pct = p.clamp(0.0, 1.0) as f32;
                // Determinate: contract accent gradient fill at percentage width.
                track = track.child(
                    div()
                        .h_full()
                        .rounded(px(999.0))
                        .bg(indicator_fill)
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
                        .bg(indicator_fill)
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
