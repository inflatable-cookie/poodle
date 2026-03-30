//! Skeleton — real GPUI component backed by SkeletonSpec.

use std::time::Duration;

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::SkeletonSpec;

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI skeleton placeholder component backed by `SkeletonSpec`.
pub struct Skeleton {
    spec: SkeletonSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for Skeleton {
    type Target = SkeletonSpec;
    fn deref(&self) -> &SkeletonSpec { &self.spec }
}

impl Skeleton {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: SkeletonSpec::new(), theme: theme.clone() }
    }

    pub fn from_spec(spec: SkeletonSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn shape(mut self, v: impl Into<String>) -> Self { self.spec.shape = v.into(); self }
    pub fn width(mut self, v: impl Into<String>) -> Self { self.spec.width = Some(v.into()); self }
    pub fn height(mut self, v: impl Into<String>) -> Self { self.spec.height = Some(v.into()); self }
    pub fn animated(mut self, v: bool) -> Self { self.spec.is_animated = v; self }

}

impl IntoElement for Skeleton {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let fill = resolve_color(theme, spec.fill_token());
        let radius = resolve_radius(theme, spec.radius_token());

        let mut el = div()
            .rounded(radius)
            .bg(fill);

        // Dimensions
        if let Some(ref w) = spec.width {
            if let Ok(val) = w.parse::<f32>() {
                el = el.w(px(val));
            } else {
                el = el.w_full();
            }
        } else {
            el = el.w_full();
        }

        // Contract: default height 0.875rem — resolved from body typography size token
        let default_height = resolve_px(theme, spec.default_height_token());
        if let Some(ref h) = spec.height {
            if let Ok(val) = h.parse::<f32>() {
                el = el.h(px(val));
            } else {
                el = el.h(default_height);
            }
        } else {
            el = el.h(default_height);
        }

        // Shimmer animation or static opacity
        if spec.is_animated {
            el.with_animation(
                "skeleton-shimmer",
                Animation::new(Duration::from_millis(1500))
                    .repeat()
                    .with_easing(gpui::ease_in_out),
                |el, delta| {
                    // Pulse opacity between 0.3 and 0.7
                    let opacity = 0.3 + delta * 0.4;
                    el.opacity(opacity)
                },
            ).into_any_element()
        } else {
            el.opacity(0.5).into_any_element()
        }
    }
}
