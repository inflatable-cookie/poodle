//! Icon — real GPUI component backed by IconSpec.
//!
//! Resolves icon names from the icon asset directory and renders them
//! using GPUI's svg() element at the correct token-resolved size.

use gpui::*;
use poodle_specs::{IconSize, IconSpec};

use crate::theme_ext::{resolve_color, resolve_px};

/// A real GPUI icon component backed by `IconSpec`.
///
/// Usage:
/// ```ignore
/// Icon::from_spec(IconSpec::new("plus"), &theme)
/// ```
pub struct Icon {
    spec: IconSpec,
    theme: poodle_gpui::GpuiThemeProvider,
    /// Explicit color override. If None, uses theme's icon.primary color.
    color: Option<Hsla>,
}

impl std::ops::Deref for Icon {
    type Target = IconSpec;
    fn deref(&self) -> &IconSpec {
        &self.spec
    }
}

impl Icon {
    pub fn new(name: impl Into<String>, theme: &poodle_gpui::GpuiThemeProvider) -> Self {
        Self {
            spec: IconSpec::new(name),
            theme: theme.clone(),
            color: None,
        }
    }

    pub fn from_spec(spec: IconSpec, theme: &poodle_gpui::GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            color: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.spec.name = v.into();
        self
    }
    pub fn size(mut self, v: IconSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }

    /// Create a small icon for use inside buttons.
    pub fn button_icon(name: impl Into<String>, theme: &poodle_gpui::GpuiThemeProvider) -> Self {
        Self::from_spec(IconSpec::new(name).with_size(IconSize::Sm), theme)
    }

    /// Set an explicit color for this icon. When used inside a button or
    /// other colored container, pass the container's text color here so
    /// the icon matches.
    pub fn with_color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

impl IntoElement for Icon {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let size = resolve_px(&self.theme, self.spec.size_token());
        let icon_path = format!("assets/icons/{}.svg", self.spec.name);
        let shared_path = SharedString::from(icon_path);

        // GPUI's svg() element requires an explicit text_color to render —
        // it does NOT inherit from the parent. We must set it here.
        let color = self
            .color
            .unwrap_or_else(|| resolve_color(&self.theme, "color.icon.primary"));

        svg()
            .path(shared_path)
            .size(size)
            .flex_shrink_0()
            .text_color(color)
            .into_any_element()
    }
}
