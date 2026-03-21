//! Box — real GPUI component backed by BoxSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{BoxSpec, Dimension, Overflow, PaddingScale};

use crate::theme_ext::resolve_px;

/// A layout box with padding and overflow control.
pub struct Box {
    spec: BoxSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
}

impl std::ops::Deref for Box {
    type Target = BoxSpec;
    fn deref(&self) -> &BoxSpec { &self.spec }
}

impl Box {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: BoxSpec::new(), theme: theme.clone(), children: Vec::new() }
    }

    pub fn from_spec(spec: BoxSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn padding(mut self, v: PaddingScale) -> Self { self.spec.padding = v; self }
    pub fn width(mut self, v: Dimension) -> Self { self.spec.width = Some(v); self }
    pub fn height(mut self, v: Dimension) -> Self { self.spec.height = Some(v); self }
    pub fn min_width(mut self, v: Dimension) -> Self { self.spec.min_width = Some(v); self }
    pub fn min_height(mut self, v: Dimension) -> Self { self.spec.min_height = Some(v); self }
    pub fn overflow(mut self, v: Overflow) -> Self { self.spec.overflow = v; self }
    pub fn role(mut self, v: impl Into<String>) -> Self { self.spec.role = Some(v.into()); self }


    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl IntoElement for Box {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let padding = spec.resolved_padding();

        let mut el = div();

        // Padding
        if let Some(h) = padding.horizontal {
            el = el.px(resolve_px(theme, h));
        }
        if let Some(v) = padding.vertical {
            el = el.py(resolve_px(theme, v));
        }

        // Overflow
        match spec.overflow {
            Overflow::Hidden | Overflow::Clip => {
                el = el.overflow_hidden();
            }
            Overflow::Visible => {}
        }

        // Children
        for child in self.children {
            el = el.child(child);
        }

        el.into_any_element()
    }
}
