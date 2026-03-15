//! PugBox — real GPUI component backed by BoxSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{BoxSpec, Overflow};

use crate::theme_ext::resolve_px;

/// A layout box with padding and overflow control.
pub struct PugBox {
    spec: BoxSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
}

impl PugBox {
    pub fn new(spec: BoxSpec, theme: &GpuiThemeProvider) -> Self {
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

impl IntoElement for PugBox {
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
