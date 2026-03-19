use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::ProgressSpec;
use pug_gpui_components::PugProgress;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let text_tertiary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(16.0))
        // --- Determinate ---
        .child(section_label("DETERMINATE", text_secondary))
        .child(
            div().flex().flex_col().gap(px(16.0))
                .child(progress_item("Empty", ProgressSpec::new().with_value(0.0), theme, text_tertiary))
                .child(progress_item("35%", ProgressSpec::new().with_value(35.0), theme, text_tertiary))
                .child(progress_item("72%", ProgressSpec::new().with_value(72.0), theme, text_tertiary))
                .child(progress_item("Complete", ProgressSpec::new().with_value(100.0), theme, text_tertiary))
        )
        // --- Indeterminate ---
        .child(section_label("INDETERMINATE", text_secondary))
        .child(
            progress_item("Loading", ProgressSpec::new().with_indeterminate(true), theme, text_tertiary)
        )
        // --- Custom Max ---
        .child(section_label("CUSTOM MAX", text_secondary))
        .child({
            let mut spec = ProgressSpec::new().with_value(3.0);
            spec.max = 5.0;
            progress_item("Steps", spec, theme, text_tertiary)
        })
}

fn progress_item(
    label: &str,
    spec: ProgressSpec,
    theme: &GpuiThemeProvider,
    label_color: pug_tokens::typed::ColorValue,
) -> Div {
    div().flex().flex_col().gap(px(4.0))
        .child(
            div().text_xs().text_color(color_to_hsla(label_color))
                .child(label.to_string())
        )
        .child(PugProgress::new(spec, theme))
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
