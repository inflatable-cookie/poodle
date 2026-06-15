use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{TextSpec, TextWeight};

use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_color;

pub struct Text {
    spec: TextSpec,
    theme: GpuiThemeProvider,
}

impl Text {
    pub fn new(content: impl Into<String>, theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(TextSpec::new(content), theme)
    }

    pub fn from_spec(spec: TextSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for Text {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let color = resolve_color(&self.theme, self.spec.color_token());
        let weight = match self.spec.weight {
            TextWeight::Normal => FontWeight::NORMAL,
            TextWeight::Medium => FontWeight::MEDIUM,
            TextWeight::Semibold => FontWeight::SEMIBOLD,
            TextWeight::Bold => FontWeight::BOLD,
        };

        div()
            .text_size(px(rem_to_px(self.spec.font_size_rem())))
            .line_height(relative(self.spec.line_height()))
            .font_weight(weight)
            .text_color(color)
            .when(self.spec.clamp.is_some(), |el| el.overflow_hidden())
            .child(self.spec.content)
            .into_any_element()
    }
}
