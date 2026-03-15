use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{SeparatorSpec, RuleTone};
use pug_gpui_components::PugSeparator;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let spec = SeparatorSpec::new().with_tone(RuleTone::Default);

    div().flex().flex_col().gap(px(6.0))
        .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Above"))
        .child(PugSeparator::new(spec, theme))
        .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Below"))
}
