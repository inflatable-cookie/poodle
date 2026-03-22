//! Meter — real GPUI component backed by MeterSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::MeterSpec;

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

        let fill = resolve_color(theme, spec.fill_token());
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let radius = resolve_radius(theme, "semantic.radius.pill");
        let progress = spec.normalized_progress();

        // Contract: track bg = color-mix(surface 92%, transparent-black)
        let track_fill = color_mix(surface_bg, gpui::transparent_black(), 0.92);

        let fill_width_pct = (progress * 100.0) as f32;

        // Contract: track height 0.5rem (8px)
        div()
            .w_full()
            .h(px(8.0))
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
