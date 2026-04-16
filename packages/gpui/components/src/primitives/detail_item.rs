use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{DetailItemLayout, DetailItemPresentation, DetailItemSpec};

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub struct DetailItem {
    spec: DetailItemSpec,
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

impl std::ops::Deref for DetailItem {
    type Target = DetailItemSpec;
    fn deref(&self) -> &DetailItemSpec {
        &self.spec
    }
}

impl DetailItem {
    pub fn new(label: impl Into<String>, theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(DetailItemSpec::new(label), theme)
    }

    pub fn from_spec(spec: DetailItemSpec, theme: &GpuiThemeProvider) -> Self {
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
    pub fn label(mut self, v: impl Into<String>) -> Self {
        self.spec.label = v.into();
        self
    }
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.spec.description = Some(v.into());
        self
    }
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.spec.value = Some(v.into());
        self
    }
    pub fn empty_text(mut self, v: impl Into<String>) -> Self {
        self.spec.empty_text = v.into();
        self
    }
    pub fn truncate_value(mut self, v: bool) -> Self {
        self.spec.truncate_value = v;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn presentation(mut self, v: DetailItemPresentation) -> Self {
        self.spec.presentation = v;
        self
    }

    pub fn with_action(mut self, action: impl IntoElement) -> Self {
        self.action = Some(action.into_any_element());
        self
    }

    pub fn with_value_content(mut self, content: impl IntoElement) -> Self {
        self.value_content = Some(content.into_any_element());
        self
    }
}

impl IntoElement for DetailItem {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let is_stacked = self.spec.layout == DetailItemLayout::Stacked;

        let mut label_el = div()
            .flex()
            .flex_col()
            .child(
                div()
                    .text_color(self.label_color)
                    .child(self.spec.label.clone()),
            )
            .when_some(self.spec.description.as_ref(), |el, desc| {
                el.child(div().text_color(self.description_color).child(desc.clone()))
            });

        // Inline layout: fixed label width; stacked: full width
        if !is_stacked {
            label_el = label_el.w(px(rem_to_px(11.25))).flex_shrink_0(); // Svelte: 11.25rem
        }

        let value_block = if let Some(content) = self.value_content {
            div().flex_1().child(content)
        } else if let Some(ref value) = self.spec.value {
            div()
                .flex_1()
                .text_color(self.value_color)
                .when(self.spec.truncate_value, |el| {
                    el.overflow_x_hidden().text_ellipsis()
                })
                .child(value.clone())
        } else {
            // Empty state: show empty_text (default "--") in muted color
            div()
                .flex_1()
                .text_color(self.description_color)
                .child(self.spec.empty_text.clone())
        };

        // Surface presentation adds background, padding, and radius; Simple is plain
        let is_surface = self.spec.presentation == DetailItemPresentation::Surface;

        let mut row = div().flex().gap(self.gap);

        if is_surface {
            row = row
                .bg(self.background)
                .rounded(self.radius)
                .px(self.padding_x)
                .py(self.padding_y);
        }

        if is_stacked {
            row = row.flex_col();
        } else {
            row = row.flex_row().items_center();
        }

        row.child(label_el)
            .child(value_block)
            .when_some(self.action, |el, action| el.child(action))
            .into_any_element()
    }
}
