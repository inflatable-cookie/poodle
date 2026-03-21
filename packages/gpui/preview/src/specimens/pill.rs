use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::PillSpec;
use pug_gpui_components::Pill;
pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(16.0))
        // --- Tones ---
        .child(section_label("TONES", text_secondary))
        .child(
            div().flex().gap(px(8.0)).items_center()
                .child(Pill::from_spec(PillSpec::new().with_label("Neutral"), theme))
                .child(Pill::from_spec(PillSpec::new().with_label("Info"), theme))
                .child(Pill::from_spec(PillSpec::new().with_label("Success"), theme))
                .child(Pill::from_spec(PillSpec::new().with_label("Warning"), theme))
                .child(Pill::from_spec(PillSpec::new().with_label("Danger"), theme))
        )
        // --- Sizes ---
        .child(section_label("SIZES", text_secondary))
        .child(
            div().flex().gap(px(8.0)).items_center()
                .child(Pill::from_spec(PillSpec::new().with_label("Small"), theme))
                .child(Pill::from_spec(PillSpec::new().with_label("Medium"), theme))
        )
        // --- Code font ---
        .child(section_label("CODE FONT", text_secondary))
        .child(
            div().flex().gap(px(8.0)).items_center()
                .child(Pill::from_spec(PillSpec::new().with_label("v2.4.1"), theme))
                .child(Pill::from_spec(PillSpec::new().with_label("stable"), theme))
                .child(Pill::from_spec(PillSpec::new().with_label("beta"), theme))
        )
        // --- Muted ---
        .child(section_label("MUTED", text_secondary))
        .child(
            div().flex().gap(px(8.0)).items_center()
                .child(Pill::from_spec(PillSpec::new().with_label("Muted neutral"), theme))
                .child(Pill::from_spec(PillSpec::new().with_label("Muted success"), theme))
                .child(Pill::from_spec(PillSpec::new().with_label("Muted danger"), theme))
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(crate::style_bridge::color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
