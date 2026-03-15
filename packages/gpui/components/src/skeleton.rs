//! PugSkeleton — real GPUI component backed by SkeletonSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::SkeletonSpec;

use crate::theme_ext::{resolve_color, resolve_radius};

/// A real GPUI skeleton placeholder component backed by `SkeletonSpec`.
pub struct PugSkeleton {
    spec: SkeletonSpec,
    theme: GpuiThemeProvider,
}

impl PugSkeleton {
    pub fn new(spec: SkeletonSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for PugSkeleton {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let fill = resolve_color(theme, spec.fill_token());
        let radius = resolve_radius(theme, spec.radius_token());

        let mut el = div()
            .rounded(radius)
            .bg(fill)
            .opacity(0.5);

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

        if let Some(ref h) = spec.height {
            if let Ok(val) = h.parse::<f32>() {
                el = el.h(px(val));
            } else {
                el = el.h(px(16.0));
            }
        } else {
            el = el.h(px(16.0));
        }

        el.into_any_element()
    }
}
