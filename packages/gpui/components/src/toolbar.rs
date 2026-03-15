//! PugToolbar — real GPUI component backed by ToolbarSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{Alignment, ToolbarSpec};

use crate::theme_ext::{resolve_color, resolve_px};

/// A real GPUI horizontal toolbar component backed by `ToolbarSpec`.
pub struct PugToolbar {
    spec: ToolbarSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
}

impl PugToolbar {
    pub fn new(spec: ToolbarSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl IntoElement for PugToolbar {
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
