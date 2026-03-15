//! PugInline — horizontal flex layout wrapper with configurable gap.

use gpui::*;
use pug_gpui::GpuiThemeProvider;

use crate::theme_ext::resolve_px;

/// A horizontal flex layout wrapper with configurable gap.
pub struct PugInline {
    theme: GpuiThemeProvider,
    gap_token: Option<&'static str>,
    children: Vec<AnyElement>,
}

impl PugInline {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            theme: theme.clone(),
            gap_token: None,
            children: Vec::new(),
        }
    }

    pub fn with_gap(mut self, gap_token: &'static str) -> Self {
        self.gap_token = Some(gap_token);
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl IntoElement for PugInline {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;

        let gap = self
            .gap_token
            .map(|t| resolve_px(theme, t))
            .unwrap_or(px(8.0));

        let mut el = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(gap);

        for child in self.children {
            el = el.child(child);
        }

        el.into_any_element()
    }
}
