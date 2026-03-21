//! BlockEditor — block-based content editor backed by BlockEditorSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_composites::BlockEditorSpec;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

pub struct BlockEditor {
    spec: BlockEditorSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
}

impl std::ops::Deref for BlockEditor {
    type Target = BlockEditorSpec;
    fn deref(&self) -> &BlockEditorSpec { &self.spec }
}

impl BlockEditor {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: BlockEditorSpec::new(), theme: theme.clone(), children: Vec::new() }
    }
    pub fn from_spec(spec: BlockEditorSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), children: Vec::new() }
    }
    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element()); self
    }
}

impl IntoElement for BlockEditor {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let fill = resolve_color(&self.theme, self.spec.fill_token());
        let border = resolve_color(&self.theme, self.spec.border_token());
        let gap = resolve_px(&self.theme, self.spec.block_gap_token());
        let radius = resolve_radius(&self.theme, "semantic.radius.surface");

        let mut el = div()
            .bg(fill).border_1().border_color(border).rounded(radius)
            .flex().flex_col().gap(gap)
            .px(px(12.0)).py(px(8.0))
            .min_h(px(120.0));
        for child in self.children { el = el.child(child); }
        if self.spec.is_disabled {
            el = el.opacity(resolve_opacity(&self.theme, "semantic.state.opacity.disabled"));
        }
        el.into_any_element()
    }
}
