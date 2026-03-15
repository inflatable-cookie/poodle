//! PugScrollShell — real GPUI component backed by ScrollShellSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::ScrollShellSpec;

use crate::theme_ext::resolve_px;

/// A scrollable container with directional overflow.
///
/// Note: gpui handles scrolling differently from web — this component
/// applies overflow_hidden and padding from the spec. For true scrolling,
/// use gpui's built-in scroll view primitives.
pub struct PugScrollShell {
    spec: ScrollShellSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
}

impl PugScrollShell {
    pub fn new(spec: ScrollShellSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl IntoElement for PugScrollShell {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let padding = spec.resolved_padding();

        let mut el = div().overflow_hidden();

        // Padding
        if let Some(h) = padding.horizontal {
            el = el.px(resolve_px(theme, h));
        }
        if let Some(v) = padding.vertical {
            el = el.py(resolve_px(theme, v));
        }

        for child in self.children {
            el = el.child(child);
        }

        el.into_any_element()
    }
}
