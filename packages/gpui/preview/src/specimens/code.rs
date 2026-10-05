use crate::app_state::AppState;
use crate::node_compat::{Code, Eyebrow};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{CodeInlineVariant, CodeSpec, CodeTypography, EyebrowSpec};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_primary = theme.resolve_color("color.text.primary");

    let ts_source = "import { Button } from \"@inflatable-cookie/poodle-svelte\";\n\nfunction handleClick(event: MouseEvent): void {\n  console.log(\"Button clicked\", event);\n}";

    let css_source = ".button {\n  display: inline-flex;\n  align-items: center;\n  border-radius: var(--poodle-radius-control);\n  background: var(--poodle-color-accent-base);\n}";

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Block with language label ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Block with language label"),
                    theme,
                ))
                .child(Code::live_code(
                    CodeSpec::new()
                        .with_content(ts_source)
                        .with_language("typescript"),
                    "code-copy-block-ts",
                    state,
                    theme,
                )),
        )
        // --- With line numbers and highlight ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With line numbers and highlight"),
                    theme,
                ))
                .child(Code::live_code(
                    CodeSpec::new()
                        .with_content(ts_source)
                        .with_language("ts")
                        .with_show_line_numbers(true)
                        .with_highlight_lines(vec![3, 4]),
                    "code-copy-block-highlight",
                    state,
                    theme,
                )),
        )
        // --- CSS with max height ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("CSS with max height"),
                    theme,
                ))
                .child(Code::live_code(
                    CodeSpec::new()
                        .with_content(css_source)
                        .with_language("css")
                        .with_max_height(96.0),
                    "code-copy-block-css",
                    state,
                    theme,
                )),
        )
        // --- Inline code ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Inline code"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .gap(px(4.0))
                        .items_center()
                        .text_sm()
                        .text_color(color_to_hsla(text_primary))
                        .child("Use ".to_string())
                        .child(Code::live_code(
                            CodeSpec::new()
                                .with_content("npm install")
                                .with_inline(true),
                            "code-copy-inline-npm",
                            state,
                            theme,
                        ))
                        .child(" to install dependencies.".to_string()),
                ),
        )
        // --- Inline plain + inline typography ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Inline variants"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .gap(px(4.0))
                        .items_center()
                        .text_sm()
                        .text_color(color_to_hsla(text_primary))
                        .child("Plain ".to_string())
                        .child(Code::live_code(
                            CodeSpec::new()
                                .with_content("git status")
                                .with_inline(true)
                                .with_inline_variant(CodeInlineVariant::Plain),
                            "code-copy-inline-status",
                            state,
                            theme,
                        ))
                        .child(" and inline-typography ".to_string())
                        .child(Code::live_code(
                            CodeSpec::new()
                                .with_content("git log")
                                .with_inline(true)
                                .with_typography(CodeTypography::Inline),
                            "code-copy-inline-log",
                            state,
                            theme,
                        ))
                        .child(" fragments.".to_string()),
                ),
        )
        // --- No copy button ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("No copy button"),
                    theme,
                ))
                .child(Code::from_spec(
                    CodeSpec::new()
                        .with_content("echo 'hello world'")
                        .with_language("bash")
                        .with_copyable(false),
                    theme,
                )),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "code",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                Code::from_spec(
                    CodeSpec::new()
                        .with_content("const x: number = 42;")
                        .with_language("ts"),
                    theme,
                )
                .size(size)
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                Code::from_spec(
                    CodeSpec::new()
                        .with_content("const x: number = 42;")
                        .with_language("ts"),
                    theme,
                )
                .with_density(density)
                .into_any_element()
            }),
    )
}
