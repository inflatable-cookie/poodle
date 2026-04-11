//! Region — presentational placeholder block with dashed border.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_components::RegionSpec;

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A presentational placeholder with a dashed border and centered label.
///
/// GPUI does not support native dashed borders, so a solid border is used
/// as the closest approximation (known delta from contract).
pub struct Region {
    spec: RegionSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for Region {
    type Target = RegionSpec;
    fn deref(&self) -> &RegionSpec { &self.spec }
}

impl Region {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: RegionSpec::new(), theme: theme.clone() }
    }

    pub fn from_spec(spec: RegionSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn label(mut self, v: impl Into<String>) -> Self { self.spec.label = v.into(); self }
    pub fn color(mut self, v: impl Into<String>) -> Self { self.spec.color = Some(v.into()); self }
    pub fn min_height_px(mut self, v: f32) -> Self { self.spec.min_height_px = v; self }

}

impl IntoElement for Region {
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
                    // Contract: label font 0.6875rem (11px), semibold, uppercase, line-height 1.5
                    .text_size(px(11.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .line_height(relative(1.5))
                    .child(spec.label.to_uppercase()),
            );
        }

        el.into_any_element()
    }
}
