//! PugGrid — real GPUI component backed by GridSpec.
//!
//! Note: gpui doesn't have native CSS grid support, so we approximate
//! using flex-wrap with equal-width children.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::GridSpec;

use crate::theme_ext::resolve_px;

/// A grid layout approximated with flex-wrap.
pub struct PugGrid {
    spec: GridSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
}

impl PugGrid {
    pub fn new(spec: GridSpec, theme: &GpuiThemeProvider) -> Self {
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

impl PugGrid {
    /// Parse column count from spec.columns (e.g. "1fr" -> 1, "repeat(3, 1fr)" -> 3, "1fr 1fr 1fr" -> 3).
    fn column_count(&self) -> usize {
        let cols_str = self.spec.columns.as_str();
        // Try "repeat(N, ...)" pattern
        if cols_str.starts_with("repeat(") {
            if let Some(n) = cols_str
                .trim_start_matches("repeat(")
                .split(',')
                .next()
                .and_then(|s| s.trim().parse::<usize>().ok())
            {
                return n;
            }
        }
        // Count space-separated tracks
        let count = cols_str.split_whitespace().count();
        if count > 0 { count } else { 1 }
    }
}

impl IntoElement for PugGrid {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let padding = spec.resolved_padding();
        let col_count = self.column_count();

        let mut el = div().flex().flex_wrap();

        // Column gap
        if let Some(gap_token) = spec.resolved_column_gap() {
            el = el.gap(resolve_px(theme, gap_token));
        }

        // Padding
        if let Some(h) = padding.horizontal {
            el = el.px(resolve_px(theme, h));
        }
        if let Some(v) = padding.vertical {
            el = el.py(resolve_px(theme, v));
        }

        // Wrap each child in a container that enforces equal columns
        // Using flex-basis percentage approximation
        for child in self.children {
            let wrapper = div().flex_1().min_w(px(0.0)).child(child);
            el = el.child(wrapper);
        }

        el.into_any_element()
    }
}
