use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::TooltipSpec;
use pug_gpui_components::PugTooltip;
use pug_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let border = theme.resolve_color("semantic.color.border.default");
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let tooltip_spec = TooltipSpec::new()
        .with_content("Tooltip content")
        .with_default_open(true);

    div().flex().flex_col().gap(px(4.0))
        .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Tooltip"))
        .child(
            PugTooltip::new(tooltip_spec, theme)
                .with_trigger(
                    div()
                        .px(px(8.0)).py(px(4.0)).rounded(px(4.0))
                        .border_1().border_color(color_to_hsla(border)).text_sm()
                        .child("Hover me")
                )
        )
}
