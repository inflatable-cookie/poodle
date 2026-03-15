//! PugSeparator — real GPUI component backed by SeparatorSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{SeparatorOrientation, SeparatorSpec};

use crate::theme_ext::resolve_color;

/// A real GPUI separator component backed by `SeparatorSpec`.
pub struct PugSeparator {
    spec: SeparatorSpec,
    theme: GpuiThemeProvider,
}

impl PugSeparator {
    pub fn new(spec: SeparatorSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for PugSeparator {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let color = resolve_color(&self.theme, self.spec.resolved_color());

        match self.spec.orientation {
            SeparatorOrientation::Horizontal => {
                div().w_full().h(px(1.0)).bg(color).into_any_element()
            }
            SeparatorOrientation::Vertical => {
                div().h_full().w(px(1.0)).bg(color).into_any_element()
            }
        }
    }
}
