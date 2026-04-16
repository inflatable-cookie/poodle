use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::MetaBarSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px};

pub struct MetaBar {
    spec: MetaBarSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
}

impl std::ops::Deref for MetaBar {
    type Target = MetaBarSpec;
    fn deref(&self) -> &MetaBarSpec {
        &self.spec
    }
}

impl MetaBar {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(MetaBarSpec::new(), theme)
    }

    pub fn from_spec(spec: MetaBarSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }

    pub fn show_separators(mut self, v: bool) -> Self {
        self.spec.show_separators = v;
        self
    }

    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn with_children(mut self, children: impl IntoIterator<Item = AnyElement>) -> Self {
        self.children.extend(children);
        self
    }
}

impl IntoElement for MetaBar {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let separator_color = resolve_color(theme, "color.text.secondary");

        let mut row = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(resolve_px(theme, "space.inline.sm"))
            .min_w(px(0.0));

        for (idx, child) in self.children.into_iter().enumerate() {
            if idx > 0 && self.spec.show_separators {
                row = row.child(div().w(px(rem_to_px(0.25))).h(px(rem_to_px(0.25))).rounded(px(999.0)).bg(Hsla {
                    a: separator_color.a * 0.72,
                    ..separator_color
                }));
            }

            row = row.child(div().min_w(px(0.0)).child(child));
        }

        row.into_any_element()
    }
}
