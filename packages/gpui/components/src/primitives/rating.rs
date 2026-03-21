//! Rating — real GPUI component backed by RatingSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{IconSize, IconSpec, RatingSpec};

use super::icon::Icon;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px};

/// A real GPUI rating component backed by `RatingSpec`.
pub struct Rating {
    spec: RatingSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for Rating {
    type Target = RatingSpec;
    fn deref(&self) -> &RatingSpec { &self.spec }
}

impl Rating {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: RatingSpec::new(), theme: theme.clone() }
    }

    pub fn from_spec(spec: RatingSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: f64) -> Self { self.spec.value = v; self }
    pub fn readonly(mut self, v: bool) -> Self { self.spec.is_readonly = v; self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn precision(mut self, v: f64) -> Self { self.spec.precision = v; self }

}

impl IntoElement for Rating {
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
                Icon::from_spec(IconSpec::new("star").with_size(IconSize::Sm), theme)
                    .with_color(color),
            );
        }

        if spec.is_disabled {
            el = el.opacity(disabled_opacity);
        }

        el.into_any_element()
    }
}
