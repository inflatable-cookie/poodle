//! PugRating — real GPUI component backed by RatingSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::RatingSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI rating component backed by `RatingSpec`.
pub struct PugRating {
    spec: RatingSpec,
    theme: GpuiThemeProvider,
}

impl PugRating {
    pub fn new(spec: RatingSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for PugRating {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let active_color = resolve_color(theme, spec.active_color_token());
        let inactive_color = resolve_color(theme, spec.inactive_color_token());
        let filled = spec.filled_count();

        let mut el = div().flex().items_center().gap(px(2.0));

        for i in 0..spec.max {
            let color = if i < filled {
                active_color
            } else {
                inactive_color
            };

            el = el.child(
                div()
                    .text_sm()
                    .text_color(color)
                    .child("*"),
            );
        }

        if spec.is_disabled {
            el = el.opacity(0.5);
        }

        el.into_any_element()
    }
}
