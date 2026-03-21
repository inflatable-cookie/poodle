//! CardRadioGroup — selectable card group backed by CardRadioGroupSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_composites::CardRadioGroupSpec;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub struct CardRadioGroup {
    spec: CardRadioGroupSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for CardRadioGroup {
    type Target = CardRadioGroupSpec;
    fn deref(&self) -> &CardRadioGroupSpec { &self.spec }
}

impl CardRadioGroup {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: CardRadioGroupSpec::new(Vec::new()), theme: theme.clone() }
    }
    pub fn from_spec(spec: CardRadioGroupSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone() }
    }
}

impl IntoElement for CardRadioGroup {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let selected_fill = resolve_color(theme, spec.selected_fill_token());
        let unselected_fill = resolve_color(theme, spec.unselected_fill_token());
        let border = resolve_color(theme, spec.border_token());
        let radius = resolve_radius(theme, "semantic.radius.surface");
        let text_color = resolve_color(theme, "semantic.color.text.primary");
        let accent = resolve_color(theme, "semantic.color.accent.base");
        let selected = spec.value.as_deref().or(spec.default_value.as_deref());

        let mut el = div().flex().flex_col().gap(px(8.0));
        for option in &spec.options {
            let is_selected = selected == Some(option.value.as_str());
            let fill = if is_selected { selected_fill } else { unselected_fill };
            let border_c = if is_selected { accent } else { border };
            let bw = if is_selected { 2.0 } else { 1.0 };
            let card = div()
                .bg(fill).rounded(radius)
                .border(px(bw)).border_color(border_c)
                .px(px(16.0)).py(px(12.0))
                .cursor_pointer()
                .child(div().text_sm().text_color(text_color).child(option.label.clone()));
            el = el.child(card);
        }
        if spec.is_disabled {
            let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
            el = el.opacity(opacity);
        }
        el.into_any_element()
    }
}
