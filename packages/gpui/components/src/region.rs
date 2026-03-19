//! PugRegion — presentational placeholder block with dashed border.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::RegionSpec;

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A presentational placeholder with a dashed border and centered label.
///
/// GPUI does not support native dashed borders, so a solid border is used
/// as the closest approximation (known delta from contract).
pub struct PugRegion {
    spec: RegionSpec,
    theme: GpuiThemeProvider,
}

impl PugRegion {
    pub fn new(spec: RegionSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for PugRegion {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let padding = resolve_px(theme, spec.padding_token());
        let radius = resolve_radius(theme, spec.radius_token());

        // Custom color overrides both border and label
        let border_color = if let Some(ref hex) = spec.color {
            resolve_color(theme, hex)
        } else {
            resolve_color(theme, spec.border_color_token())
        };
        let label_color = if let Some(ref hex) = spec.color {
            resolve_color(theme, hex)
        } else {
            resolve_color(theme, spec.label_color_token())
        };

        let mut el = div()
            .flex()
            .items_center()
            .justify_center()
            .min_h(px(spec.min_height_px))
            .p(padding)
            .rounded(radius)
            .border_2()
            .border_color(border_color);

        if !spec.label.is_empty() {
            el = el.child(
                div()
                    .text_color(label_color)
                    .text_xs()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(spec.label.to_uppercase()),
            );
        }

        el.into_any_element()
    }
}
