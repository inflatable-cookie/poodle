//! PugProgress — real GPUI component backed by ProgressSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::ProgressSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI progress bar component backed by `ProgressSpec`.
pub struct PugProgress {
    spec: ProgressSpec,
    theme: GpuiThemeProvider,
}

impl PugProgress {
    pub fn new(spec: ProgressSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for PugProgress {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let accent = resolve_color(theme, spec.indicator_fill_token());
        let border = resolve_color(theme, "semantic.color.border.default");
        let track_bg = resolve_color(theme, "semantic.color.background.surface");

        let progress = spec.normalized_progress();

        let track = div()
            .w_full()
            .h(px(6.0))
            .rounded(px(3.0))
            .bg(track_bg)
            .border_1()
            .border_color(border.opacity(0.3));

        let fill_width = match progress {
            Some(p) => {
                // Clamp to 0..1
                let pct = p.clamp(0.0, 1.0);
                // Use relative sizing — approximate with fixed widths
                // In a real app we'd use percentage-based width
                div()
                    .h_full()
                    .rounded(px(3.0))
                    .bg(accent)
                    .w(relative(pct as f32))
            }
            None => {
                // Indeterminate — show a partial bar
                div()
                    .h_full()
                    .rounded(px(3.0))
                    .bg(accent)
                    .w(relative(0.4))
            }
        };

        track.child(fill_width).into_any_element()
    }
}
