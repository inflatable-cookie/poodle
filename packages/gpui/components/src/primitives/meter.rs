//! Meter — real GPUI component backed by MeterSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::MeterSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{color_mix, resolve_color, resolve_radius};

/// A real GPUI meter component backed by `MeterSpec`.
pub struct Meter {
    spec: MeterSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for Meter {
    type Target = MeterSpec;
    fn deref(&self) -> &MeterSpec { &self.spec }
}

impl Meter {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: MeterSpec::new(), theme: theme.clone() }
    }

    pub fn from_spec(spec: MeterSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: f64) -> Self { self.spec.value = v; self }
    pub fn min(mut self, v: f64) -> Self { self.spec.min = v; self }
    pub fn max(mut self, v: f64) -> Self { self.spec.max = v; self }
    pub fn low(mut self, v: f64) -> Self { self.spec.low = Some(v); self }
    pub fn high(mut self, v: f64) -> Self { self.spec.high = Some(v); self }
    pub fn optimum(mut self, v: f64) -> Self { self.spec.optimum = Some(v); self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }

}

impl IntoElement for Meter {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // Svelte: fill uses status-success, not generic fill_token
        let success_color = resolve_color(theme, "color.status.success");
        let surface_bg = resolve_color(theme, "color.background.surface");
        let text_primary = resolve_color(theme, "color.text.primary");
        let radius = resolve_radius(theme, "radius.pill");
        let progress = spec.normalized_progress();

        // Svelte: track bg = color-mix(surface 96%, text-primary)
        let track_fill = color_mix(surface_bg, text_primary, 0.96);
        // Use status-success as the fill color (Svelte uses gradient but GPUI doesn't support gradients)
        let fill = success_color;

        let fill_width_pct = (progress * 100.0) as f32;

        // Contract: track min-height 0.5rem — resolved from spec
        let track_height = px(rem_to_px(spec.track_height_rem()));

        // Accessibility: meter semantics — bounded-value measurement display
        // GPUI equivalent of role="meter", aria-valuenow, aria-valuemin, aria-valuemax
        // Native accessibility should expose: value={spec.value}, min={spec.min},
        // max={spec.max}, low/high/optimum when set, aria-label from spec.aria_label
        div()
            .w_full()
            .h(track_height)
            .rounded(radius)
            .bg(track_fill)
            .overflow_hidden()
            .child(
                div()
                    .h_full()
                    .rounded(radius)
                    .bg(fill)
                    .w(relative(fill_width_pct / 100.0)),
            )
            .into_any_element()
    }
}
