use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{TextSpec, TextWeight};

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px};

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

        // `spacing="compact"` renders a stacked grid with a `space.stack.sm` gap
        // between child paragraphs (contract §3). GPUI uses a flex column gap.
        let spacing_gap = self
            .spec
            .spacing_gap_token()
            .map(|token| resolve_px(&self.theme, token));

        // `element` (p/span/div) has no rendering effect in GPUI — there is no DOM
        // semantics layer, so every variant renders one node (matches Jetstream).
        div()
            .text_size(px(rem_to_px(self.spec.font_size_rem())))
            .line_height(relative(self.spec.line_height()))
            .font_weight(weight)
            .text_color(color)
            .when_some(spacing_gap, |el, gap| el.flex().flex_col().gap(gap))
            .when(self.spec.clamp.is_some(), |el| el.overflow_hidden())
            .child(self.spec.content)
            .into_any_element()
    }
}
