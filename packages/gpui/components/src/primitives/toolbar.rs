//! Toolbar — real GPUI component backed by ToolbarSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{Alignment, ToolbarSpec};

use crate::theme_ext::{resolve_color, resolve_px};

/// A real GPUI horizontal toolbar component backed by `ToolbarSpec`.
pub struct Toolbar {
    spec: ToolbarSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
}

impl std::ops::Deref for Toolbar {
    type Target = ToolbarSpec;
    fn deref(&self) -> &ToolbarSpec { &self.spec }
}

impl Toolbar {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: ToolbarSpec::new(), theme: theme.clone(), children: Vec::new() }
    }

    pub fn from_spec(spec: ToolbarSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn alignment(mut self, v: Alignment) -> Self { self.spec.alignment = v; self }
    pub fn has_separator(mut self, v: bool) -> Self { self.spec.has_separator = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }


    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl IntoElement for Toolbar {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let border = resolve_color(theme, spec.border_token());
        let gap = resolve_px(theme, spec.gap_token());

        let mut el = div()
            .flex()
            .items_center()
            .gap(gap)
            .px(px(8.0))
            .py(px(4.0));

        match spec.alignment {
            Alignment::Start => {}
            Alignment::Center => {
                el = el.justify_center();
            }
            Alignment::End => {
                el = el.justify_end();
            }
            Alignment::Stretch => {}
        }

        if spec.has_separator {
            el = el.border_b_1().border_color(border);
        }

        for child in self.children {
            el = el.child(child);
        }

        el.into_any_element()
    }
}
