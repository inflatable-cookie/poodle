use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{CardSpec, InlineListSectionSpec};

use crate::theme_ext::{resolve_color, resolve_px};
use crate::Card;

pub struct InlineListSection {
    spec: InlineListSectionSpec,
    theme: GpuiThemeProvider,
    items: Vec<AnyElement>,
    actions: Option<AnyElement>,
}

impl InlineListSection {
    pub fn from_spec(spec: InlineListSectionSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            items: Vec::new(),
            actions: None,
        }
    }

    pub fn with_action(mut self, action: impl IntoElement) -> Self {
        self.actions = Some(action.into_any_element());
        self
    }

    pub fn item(mut self, item: impl IntoElement) -> Self {
        self.items.push(item.into_any_element());
        self
    }
}

impl IntoElement for InlineListSection {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let text_secondary = resolve_color(&self.theme, "color.text.secondary");
        let border = resolve_color(&self.theme, "color.border.default");
        let gap = resolve_px(&self.theme, "space.stack.md");

        let mut body = div().flex().flex_col().gap(gap).child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(text_secondary)
                                .child(self.spec.title.clone().to_uppercase()),
                        )
                        .when_some(self.spec.count.as_ref(), |el, count| {
                            el.child(
                                div()
                                    .px(px(8.0))
                                    .py(px(2.0))
                                    .rounded(px(999.0))
                                    .border_1()
                                    .border_color(border)
                                    .text_xs()
                                    .text_color(text_secondary)
                                    .child(count.clone()),
                            )
                        }),
                )
                .when_some(self.actions, |el, action| el.child(action)),
        );

        if self.items.is_empty() {
            if let Some(message) = self.spec.empty_message.clone() {
                body = body.child(div().text_sm().text_color(text_secondary).child(message));
            }
        } else {
            let mut list = div().flex().flex_col().gap(px(6.0));
            for item in self.items {
                list = list.child(item);
            }
            body = body.child(list);
        }

        if self.spec.framed {
            Card::from_spec(CardSpec::new(), &self.theme)
                .with_body(body)
                .into_any_element()
        } else {
            body.into_any_element()
        }
    }
}
