//! Stack — real GPUI component backed by StackSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{Alignment, PaddingScale, StackSpec};

use crate::theme_ext::resolve_px;

/// A vertical stack layout with gap and alignment.
pub struct Stack {
    spec: StackSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
}

impl std::ops::Deref for Stack {
    type Target = StackSpec;
    fn deref(&self) -> &StackSpec { &self.spec }
}

impl Stack {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: StackSpec::new(), theme: theme.clone(), children: Vec::new() }
    }

    pub fn from_spec(spec: StackSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), children: Vec::new() }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn gap(mut self, v: PaddingScale) -> Self { self.spec.gap = v; self }
    pub fn align(mut self, v: Alignment) -> Self { self.spec.align = v; self }
    pub fn padding(mut self, v: PaddingScale) -> Self { self.spec.padding = v; self }
    pub fn role(mut self, v: impl Into<String>) -> Self { self.spec.role = Some(v.into()); self }

    // ── GPUI-specific builders ────────────────────────────────
    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl IntoElement for Stack {
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
