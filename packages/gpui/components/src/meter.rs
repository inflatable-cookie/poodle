//! PugMeter — real GPUI component backed by MeterSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::MeterSpec;

use crate::theme_ext::{resolve_color, resolve_radius};

/// A real GPUI meter component backed by `MeterSpec`.
pub struct PugMeter {
    spec: MeterSpec,
    theme: GpuiThemeProvider,
}

impl PugMeter {
    pub fn new(spec: MeterSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for PugMeter {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let fill = resolve_color(theme, spec.fill_token());
        let track_fill = resolve_color(theme, spec.track_fill_token());
        let radius = resolve_radius(theme, "semantic.radius.pill");
        let progress = spec.normalized_progress();

        let fill_width_pct = (progress * 100.0) as f32;

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
