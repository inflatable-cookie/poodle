//! Code specimen — block + inline code, backed by `js_code` and `CodeSpec`.
//!
//! Contract: `docs/contracts/components/code.md`
//! Reference: `packages/svelte/preview/src/specimens/CodeSpecimen.svelte`
//!
//! Every group resolves through the real `js_code` builder + token system — no
//! hand-rolled code boxes. Covers the full contract specimen set: block with
//! language label, line numbers + highlight, CSS with max-height, inline code,
//! inline variants (plain + inline-typography), no-copy-button, plus the
//! orthogonal size and density scales.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::code::js_code;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{CodeInlineVariant, CodeSpec, CodeTypography, ControlDensity, ControlSize};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let primary = resolve_color(theme, "color.text.primary");

    let ts_source = "import { Button } from \"@poodle/svelte\";\n\nfunction handleClick(event: MouseEvent): void {\n  console.log(\"Button clicked\", event);\n}";
    let css_source = ".button {\n  display: inline-flex;\n  align-items: center;\n  border-radius: var(--poodle-radius-control);\n  background: var(--poodle-color-accent-base);\n}";

    div().flex_col().gap(24.0)
        // ── Block with language label ──
        .child(group("Block with language label", secondary,
            div().w(420.0)
                .child(js_code(
                    &CodeSpec::new().with_content(ts_source).with_language("typescript"),
                    theme,
                ))
        ))
        // ── With line numbers and highlight ──
        .child(group("With line numbers and highlight", secondary,
            div().w(420.0)
                .child(js_code(
                    &CodeSpec::new()
                        .with_content(ts_source)
                        .with_language("ts")
                        .with_show_line_numbers(true)
                        .with_highlight_lines(vec![3, 4]),
                    theme,
                ))
        ))
        // ── CSS with max height ──
        .child(group("CSS with max height", secondary,
            div().w(420.0)
                .child(js_code(
                    &CodeSpec::new()
                        .with_content(css_source)
                        .with_language("css")
                        .with_max_height(96.0),
                    theme,
                ))
        ))
        // ── Inline code ──
        .child(group("Inline code", secondary,
            div().flex_row().flex_wrap().gap(4.0).items_center()
                .child(label("Use ").text_color(primary).text_size(13.0))
                .child(js_code(
                    &CodeSpec::new().with_content("npm install").with_inline(true),
                    theme,
                ))
                .child(label(" to install dependencies.").text_color(primary).text_size(13.0))
        ))
        // ── Inline variants (plain + inline typography) ──
        .child(group("Inline variants", secondary,
            div().flex_row().flex_wrap().gap(4.0).items_center()
                .child(label("Plain ").text_color(primary).text_size(13.0))
                .child(js_code(
                    &CodeSpec::new()
                        .with_content("git status")
                        .with_inline(true)
                        .with_inline_variant(CodeInlineVariant::Plain),
                    theme,
                ))
                .child(label(" and inline-typography ").text_color(primary).text_size(13.0))
                .child(js_code(
                    &CodeSpec::new()
                        .with_content("git log")
                        .with_inline(true)
                        .with_typography(CodeTypography::Inline),
                    theme,
                ))
                .child(label(" fragments.").text_color(primary).text_size(13.0))
        ))
        // ── No copy button ──
        .child(group("No copy button", secondary,
            div().w(420.0)
                .child(js_code(
                    &CodeSpec::new()
                        .with_content("echo 'hello world'")
                        .with_language("bash")
                        .with_copyable(false),
                    theme,
                ))
        ))
        // ── Sizes (intrinsic font scale) ──
        .child(group("Sizes", secondary,
            div().flex_col().gap(8.0)
                .child(size_row("xs", ControlSize::Xs, theme))
                .child(size_row("sm", ControlSize::Sm, theme))
                .child(size_row("md", ControlSize::Md, theme))
                .child(size_row("lg", ControlSize::Lg, theme))
                .child(size_row("xl", ControlSize::Xl, theme))
        ))
        // ── Densities (pre padding) ──
        .child(group("Densities", secondary,
            div().flex_col().gap(8.0)
                .child(density_row("compact", ControlDensity::Compact, theme))
                .child(density_row("default", ControlDensity::Default, theme))
                .child(density_row("comfortable", ControlDensity::Comfortable, theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}

fn size_row(label_text: &str, size: ControlSize, theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    div().flex_row().gap(8.0).items_center()
        .child(label(label_text).w(28.0).text_color(secondary).text_size(11.0))
        .child(div().w(280.0).child(js_code(
            &CodeSpec::new()
                .with_content("const x: number = 42;")
                .with_language("ts")
                .with_size(size),
            theme,
        )))
}

fn density_row(label_text: &str, density: ControlDensity, theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    div().flex_row().gap(8.0).items_center()
        .child(label(label_text).w(84.0).text_color(secondary).text_size(11.0))
        .child(div().w(280.0).child(js_code(
            &CodeSpec::new()
                .with_content("const x: number = 42;")
                .with_language("ts")
                .with_density(density),
            theme,
        )))
}
