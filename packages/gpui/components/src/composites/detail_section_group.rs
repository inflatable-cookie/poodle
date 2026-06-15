use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{DetailSectionGroupLayout, DetailSectionGroupSpec};

use crate::presentation::rem_to_px;

pub struct DetailSectionGroup {
    spec: DetailSectionGroupSpec,
    children: Vec<AnyElement>,
}

impl DetailSectionGroup {
    pub fn from_spec(spec: DetailSectionGroupSpec, _theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            children: Vec::new(),
        }
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl IntoElement for DetailSectionGroup {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let mut root = div().gap(px(rem_to_px(self.spec.gap_rem())));
        root = match self.spec.layout {
            DetailSectionGroupLayout::Grid => root.flex().flex_row().flex_wrap(),
            DetailSectionGroupLayout::Stack => root.flex().flex_col(),
        };
        for child in self.children {
            root = root.child(div().flex_1().min_w(px(224.0)).child(child));
        }
        root.into_any_element()
    }
}
