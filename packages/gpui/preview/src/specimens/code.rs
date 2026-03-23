use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{CodeSpec, EyebrowSpec};
use pug_gpui_components::{Code, Eyebrow};
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_primary = theme.resolve_color("semantic.color.text.primary");

    let ts_source = "import { Button } from \"@pug/svelte-primitives\";\n\nfunction handleClick(event: MouseEvent): void {\n  console.log(\"Button clicked\", event);\n}";

    let css_source = ".button {\n  display: inline-flex;\n  align-items: center;\n  border-radius: var(--pug-radius-control);\n  background: var(--pug-color-accent-base);\n}";

    div().flex().flex_col().gap(px(24.0))
        // --- Block with language label ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Block with language label"), theme))
                .child(Code::from_spec(
                    CodeSpec::new()
                        .with_content(ts_source)
                        .with_language("typescript"),
                    theme,
                ))
        )
        // --- With line numbers and highlight ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With line numbers and highlight"), theme))
                .child(Code::from_spec(
                    CodeSpec::new()
                        .with_content(ts_source)
                        .with_language("ts")
                        .with_show_line_numbers(true)
                        .with_highlight_lines(vec![3, 4]),
                    theme,
                ))
        )
        // --- CSS with max height ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("CSS with max height"), theme))
                .child(Code::from_spec(
                    CodeSpec::new()
                        .with_content(css_source)
                        .with_language("css")
                        .with_max_height(96.0),
                    theme,
                ))
        )
        // --- Inline code ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Inline code"), theme))
                .child(
                    div().flex().flex_row().flex_wrap().gap(px(4.0)).items_center()
                        .text_sm().text_color(color_to_hsla(text_primary))
                        .child("Use ".to_string())
                        .child(Code::from_spec(
                            CodeSpec::new()
                                .with_content("npm install")
                                .with_inline(true),
                            theme,
                        ))
                        .child(" to install dependencies.".to_string())
                )
        )
        // --- No copy button ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("No copy button"), theme))
                .child(Code::from_spec(
                    CodeSpec::new()
                        .with_content("echo 'hello world'")
                        .with_language("bash")
                        .with_copyable(false),
                    theme,
                ))
        )
}
