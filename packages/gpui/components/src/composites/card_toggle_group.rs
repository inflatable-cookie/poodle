use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{CardSpec, CardToggleGroupSpec, TextSpec, TextTone};

use crate::{Card, Text};

pub struct CardToggleGroup {
    spec: CardToggleGroupSpec,
    theme: GpuiThemeProvider,
}

impl CardToggleGroup {
    pub fn from_spec(spec: CardToggleGroupSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for CardToggleGroup {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let mut root = div().flex().flex_col().gap(px(8.0));
        for option in &self.spec.options {
            let selected = self.spec.is_selected(&option.value);
            let mut card_spec = CardSpec::new().interactive();
            if selected {
                card_spec = card_spec.selected();
            }
            let body = div()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .child(Text::from_spec(
                    TextSpec::new(option.title.clone()),
                    &self.theme,
                ))
                .when_some(option.description.as_ref(), |el, description| {
                    el.child(Text::from_spec(
                        TextSpec::new(description.clone()).with_tone(TextTone::Secondary),
                        &self.theme,
                    ))
                });
            root = root.child(Card::from_spec(card_spec, &self.theme).with_body(body));
        }
        root.into_any_element()
    }
}
