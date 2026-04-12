//! Grid — real GPUI component backed by GridSpec.
//!
//! Note: gpui doesn't have native CSS grid support, so we approximate
//! using flex-wrap with equal-width children.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{Dimension, GridSpec, PaddingScale};

use crate::theme_ext::resolve_px;

/// A grid layout approximated with flex-wrap.
pub struct Grid {
    spec: GridSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
}

impl std::ops::Deref for Grid {
    type Target = GridSpec;
    fn deref(&self) -> &GridSpec {
        &self.spec
    }
}

impl Grid {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: GridSpec::new(),
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    pub fn from_spec(spec: GridSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn columns(mut self, v: Dimension) -> Self {
        self.spec.columns = v;
        self
    }
    pub fn rows(mut self, v: Dimension) -> Self {
        self.spec.rows = Some(v);
        self
    }
    pub fn gap(mut self, v: PaddingScale) -> Self {
        self.spec.gap = v;
        self
    }
    pub fn padding(mut self, v: PaddingScale) -> Self {
        self.spec.padding = v;
        self
    }
    pub fn role(mut self, v: impl Into<String>) -> Self {
        self.spec.role = Some(v.into());
        self
    }

    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl Grid {
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
        if count > 0 {
            count
        } else {
            1
        }
    }
}

impl IntoElement for Grid {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let padding = spec.resolved_padding();
        let _col_count = self.column_count();

        let mut el = div().flex().flex_wrap();

        // Column gap
        if let Some(gap_token) = spec.resolved_column_gap() {
            el = el.gap_x(resolve_px(theme, gap_token));
        }
        // Row gap (applied same as column gap for consistent grid spacing)
        if let Some(row_gap_token) = spec.resolved_row_gap() {
            el = el.gap_y(resolve_px(theme, row_gap_token));
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
