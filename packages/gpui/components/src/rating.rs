//! PugRating — real GPUI component backed by RatingSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{IconSize, IconSpec, RatingSpec};

use crate::icon::PugIcon;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px};

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
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        let inline_gap = resolve_px(theme, "semantic.space.inline.sm");

        let mut el = div().flex().items_center().gap(inline_gap);

        for i in 0..spec.max {
            let color = if i < filled {
                active_color
            } else {
                inactive_color
            };

            el = el.child(
                PugIcon::new(IconSpec::new("star").with_size(IconSize::Sm), theme)
                    .with_color(color),
            );
        }

        if spec.is_disabled {
            el = el.opacity(disabled_opacity);
        }

        el.into_any_element()
    }
}
