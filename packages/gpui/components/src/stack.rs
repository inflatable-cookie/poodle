//! PugStack — real GPUI component backed by StackSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{Alignment, StackSpec};

use crate::theme_ext::resolve_px;

/// A vertical stack layout with gap and alignment.
pub struct PugStack {
    spec: StackSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
}

impl PugStack {
    pub fn new(spec: StackSpec, theme: &GpuiThemeProvider) -> Self {
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

impl IntoElement for PugStack {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let padding = spec.resolved_padding();

        let mut el = div().flex().flex_col();

        // Gap
        if let Some(gap_token) = spec.resolved_gap() {
            el = el.gap(resolve_px(theme, gap_token));
        }

        // Alignment (cross-axis for a column)
        match spec.align {
            Alignment::Start => { el = el.items_start(); }
            Alignment::Center => { el = el.items_center(); }
            Alignment::End => { el = el.items_end(); }
            Alignment::Stretch => {} // default flex behavior
        }

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
