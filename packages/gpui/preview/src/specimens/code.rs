use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::CodeSpec;
use pug_gpui_components::Code;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let text_primary = theme.resolve_color("semantic.color.text.primary");

    let ts_source = "import { Button } from \"@pug/svelte-primitives\";\n\nfunction handleClick(event: MouseEvent): void {\n  console.log(\"Button clicked\", event);\n}";

    let css_source = ".button {\n  display: inline-flex;\n  align-items: center;\n  border-radius: var(--pug-radius-control);\n  background: var(--pug-color-accent-base);\n}";

    div().flex().flex_col().gap(px(24.0))
        // --- Block with language label ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("BLOCK WITH LANGUAGE LABEL", text_secondary))
                .child(Code::from_spec(
                    CodeSpec::new()
                        .with_content(ts_source)
                        .with_language("typescript"),
                    theme,
                ))
        )
        // --- With line numbers and highlight ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("WITH LINE NUMBERS AND HIGHLIGHT", text_secondary))
                .child(Code::from_spec(
                    CodeSpec::new()
                        .with_content(ts_source)
                        .with_language("ts")
                        .with_show_line_numbers(true),
                    theme,
                ))
        )
        // --- CSS with max height ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("CSS WITH MAX HEIGHT", text_secondary))
                .child(Code::from_spec(
                    CodeSpec::new()
                        .with_content(css_source)
                        .with_language("css"),
                    theme,
                ))
        )
        // --- Inline code ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("INLINE CODE", text_secondary))
                .child(
                    div().flex().flex_row().flex_wrap().gap(px(4.0)).items_center()
                        .text_sm().text_color(color_to_hsla(text_primary))
                        .child("Use ".to_string())
                        .child(Code::from_spec(
                            CodeSpec::new().with_content("npm install"),
                            theme,
                        ))
                        .child(" to install dependencies.".to_string())
                )
        )
        // --- No copy button ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("NO COPY BUTTON", text_secondary))
                .child(Code::from_spec(
                    CodeSpec::new()
                        .with_content("echo 'hello world'")
                        .with_language("bash"),
                    theme,
                ))
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
