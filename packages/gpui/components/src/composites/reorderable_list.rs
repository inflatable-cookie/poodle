//! ReorderableList — drag-to-reorder list backed by ReorderableListSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_composites::ReorderableListSpec;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px};

pub struct ReorderableList {
    spec: ReorderableListSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
}

impl std::ops::Deref for ReorderableList {
    type Target = ReorderableListSpec;
    fn deref(&self) -> &ReorderableListSpec { &self.spec }
}

impl ReorderableList {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: ReorderableListSpec::new(), theme: theme.clone(), children: Vec::new() }
    }
    pub fn from_spec(spec: ReorderableListSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), children: Vec::new() }
    }
    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element()); self
    }
}

impl IntoElement for ReorderableList {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let fill = resolve_color(theme, spec.fill_token());
        let gap = resolve_px(theme, spec.item_gap_token());
        let handle_color = resolve_color(theme, spec.handle_color_token());

        let mut el = div().flex().flex_col().gap(gap);
        for child in self.children {
            let row = div().flex().flex_row().items_center().gap(px(8.0))
                .child(div().text_color(handle_color).text_size(px(14.0)).child("⋮⋮"))
                .child(child);
            el = el.child(row);
        }
        if spec.is_disabled {
            let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
            el = el.opacity(opacity);
        }
        el.into_any_element()
    }
}
