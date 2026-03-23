use gpui::prelude::FluentBuilder;
use gpui::*;
use flint_gpui::GpuiThemeProvider;
use flint_primitives::DetailRowSpec;

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub struct DetailRow {
    spec: DetailRowSpec,
    label_color: Hsla,
    value_color: Hsla,
    description_color: Hsla,
    background: Hsla,
    radius: Pixels,
    padding_x: Pixels,
    padding_y: Pixels,
    gap: Pixels,
    action: Option<AnyElement>,
    value_content: Option<AnyElement>,
}

impl std::ops::Deref for DetailRow {
    type Target = DetailRowSpec;
    fn deref(&self) -> &DetailRowSpec { &self.spec }
}

impl DetailRow {
    pub fn new(label: impl Into<String>, theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(DetailRowSpec::new(label), theme)
    }

    pub fn from_spec(spec: DetailRowSpec, theme: &GpuiThemeProvider) -> Self {
        let label_color = resolve_color(theme, spec.label_color_token());
        let value_color = resolve_color(theme, spec.value_color_token());
        let description_color = resolve_color(theme, spec.description_color_token());
        let background = resolve_color(theme, spec.background_token());
        let radius = resolve_radius(theme, spec.radius_token());
        let padding_x = resolve_px(theme, spec.padding_x_token());
        let padding_y = resolve_px(theme, spec.padding_y_token());
        let gap = resolve_px(theme, spec.gap_token());

        Self {
            spec,
            label_color,
            value_color,
            description_color,
            background,
            radius,
            padding_x,
            padding_y,
            gap,
            action: None,
            value_content: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn label(mut self, v: impl Into<String>) -> Self { self.spec.label = v.into(); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.spec.description = Some(v.into()); self }
    pub fn value(mut self, v: impl Into<String>) -> Self { self.spec.value = Some(v.into()); self }
    pub fn truncate_value(mut self, v: bool) -> Self { self.spec.truncate_value = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }


    pub fn with_action(mut self, action: impl IntoElement) -> Self {
        self.action = Some(action.into_any_element());
        self
    }

    pub fn with_value_content(mut self, content: impl IntoElement) -> Self {
        self.value_content = Some(content.into_any_element());
        self
    }
}

impl IntoElement for DetailRow {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let label_block = div()
            .flex()
            .flex_col()
            .w(px(180.0))
            .flex_shrink_0()
            .child(
                div()
                    .text_color(self.label_color)
                    .child(self.spec.label.clone()),
            )
            .when_some(self.spec.description.as_ref(), |el, desc| {
                el.child(
                    div()
                        .text_color(self.description_color)
                        .child(desc.clone()),
                )
            });

        let value_block = if let Some(content) = self.value_content {
            div().flex_1().child(content)
        } else if let Some(ref value) = self.spec.value {
            let value_el = div()
                .flex_1()
                .text_color(self.value_color)
                .when(self.spec.truncate_value, |el| {
                    el.overflow_x_hidden().text_ellipsis()
                })
                .child(value.clone());
            value_el
        } else {
            div().flex_1()
        };

        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(self.gap)
            .bg(self.background)
            .rounded(self.radius)
            .px(self.padding_x)
            .py(self.padding_y)
            .child(label_block)
            .child(value_block)
            .when_some(self.action, |el, action| el.child(action))
            .into_any_element()
    }
}
