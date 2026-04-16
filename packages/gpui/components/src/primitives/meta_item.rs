use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::MetaItemSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_color;

pub struct MetaItem {
    spec: MetaItemSpec,
    theme: GpuiThemeProvider,
    value: Option<AnyElement>,
}

impl std::ops::Deref for MetaItem {
    type Target = MetaItemSpec;
    fn deref(&self) -> &MetaItemSpec {
        &self.spec
    }
}

impl MetaItem {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(MetaItemSpec::new(), theme)
    }

    pub fn from_spec(spec: MetaItemSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            value: None,
        }
    }

    pub fn label(mut self, v: impl Into<String>) -> Self {
        self.spec.label = Some(v.into());
        self
    }

    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }

    pub fn with_value(mut self, value: impl IntoElement) -> Self {
        self.value = Some(value.into_any_element());
        self
    }
}

impl IntoElement for MetaItem {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let label_color = resolve_color(&self.theme, "color.text.secondary");
        let value_color = resolve_color(&self.theme, "color.text.primary");

        let mut item = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(rem_to_px(0.375)))
            .min_w(px(0.0));

        if let Some(label) = self.spec.label {
            item = item.child(
                div()
                    .text_size(px(rem_to_px(0.6875)))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(label_color)
                    .child(label.to_uppercase()),
            );
        }

        let value = self.value.unwrap_or_else(|| {
            div()
                .text_size(px(rem_to_px(0.875)))
                .text_color(value_color)
                .child("Value")
                .into_any_element()
        });

        item.child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(px(rem_to_px(0.375)))
                .min_w(px(0.0))
                .text_size(px(rem_to_px(0.875)))
                .text_color(value_color)
                .child(value),
        )
        .into_any_element()
    }
}
