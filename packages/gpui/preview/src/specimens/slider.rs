use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::SliderSpec;
use pug_gpui_components::Slider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(16.0)).max_w(px(320.0))
        // --- Default ---
        .child(section_label("DEFAULT", text_secondary))
        .child({
            let mut spec = SliderSpec::new(65.0).with_bounds(0.0, 100.0);
            spec.step = 1.0;
            spec.aria_label = Some("Volume".to_string());

            div().flex().flex_col().gap(px(4.0))
                .child(Slider::from_spec(spec, theme))
        })
        // --- With step ---
        .child(section_label("WITH STEP", text_secondary))
        .child({
            let mut spec = SliderSpec::new(100.0).with_bounds(0.0, 100.0);
            spec.step = 10.0;
            spec.aria_label = Some("Opacity".to_string());

            div().flex().flex_col().gap(px(4.0))
                .child(Slider::from_spec(spec, theme))
        })
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child({
            let mut spec = SliderSpec::new(40.0).with_bounds(0.0, 100.0);
            spec.is_disabled = true;
            spec.aria_label = Some("Disabled".to_string());

            div().flex().flex_col().gap(px(4.0))
                .child(Slider::from_spec(spec, theme))
        })
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
