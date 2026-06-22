//! Eyebrow specimen — small uppercase label component.
//!
//! Mirrors the GPUI specimen's coverage: content/category framing plus the
//! full `EyebrowSize` (Xs/Sm/Md) and `EyebrowSpacing` (None/Bottom) axes from
//! the contract. Every node is a real `js_eyebrow` resolving tokens.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::eyebrow::js_eyebrow;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{EyebrowSize, EyebrowSpacing, EyebrowSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let primary = resolve_color(theme, "color.text.primary");

    div().flex_col().gap(24.0)
        // --- Section label (above a heading) ---
        .child(group("Section label", secondary,
            div().flex_col().gap(4.0)
                .child(js_eyebrow(&EyebrowSpec::new().with_content("Section label"), theme))
                .child(
                    label("Eyebrow renders small uppercase text used for categorizing content above headings.")
                        .text_color(secondary).text_size(13.0)
                )
        ))
        // --- Category over a title (primitive / composite framing) ---
        .child(group("Category", secondary,
            div().flex_col().gap(12.0)
                .child(
                    div().flex_col().gap(4.0)
                        .child(js_eyebrow(&EyebrowSpec::new().with_content("Primitive"), theme))
                        .child(label("Button").text_color(primary).text_size(18.0).text_weight(600))
                        .child(label("Primary interactive control for triggering actions.")
                            .text_color(secondary).text_size(13.0))
                )
                .child(
                    div().flex_col().gap(4.0)
                        .child(js_eyebrow(&EyebrowSpec::new().with_content("Composite"), theme))
                        .child(label("DataTable").text_color(primary).text_size(18.0).text_weight(600))
                        .child(label("Feature-rich table with sorting, selection, and pagination.")
                            .text_color(secondary).text_size(13.0))
                )
        ))
        // --- Sizes (Xs / Sm / Md) ---
        .child(group("Sizes", secondary,
            div().flex_col().gap(8.0)
                .child(size_row("xs", secondary,
                    js_eyebrow(&EyebrowSpec::new().with_content("Extra small").with_size(EyebrowSize::Xs), theme)))
                .child(size_row("sm", secondary,
                    js_eyebrow(&EyebrowSpec::new().with_content("Small").with_size(EyebrowSize::Sm), theme)))
                .child(size_row("md", secondary,
                    js_eyebrow(&EyebrowSpec::new().with_content("Medium").with_size(EyebrowSize::Md), theme)))
        ))
        // --- Spacing (None vs Bottom margin before content) ---
        .child(group("Spacing", secondary,
            div().flex_col().gap(12.0)
                .child(
                    div().flex_col()
                        .child(js_eyebrow(
                            &EyebrowSpec::new().with_content("None").with_spacing(EyebrowSpacing::None),
                            theme,
                        ))
                        .child(label("No margin below the eyebrow.").text_color(secondary).text_size(13.0))
                )
                .child(
                    div().flex_col()
                        .child(js_eyebrow(
                            &EyebrowSpec::new().with_content("Bottom").with_spacing(EyebrowSpacing::Bottom),
                            theme,
                        ))
                        .child(label("Token margin separates the eyebrow from following content.")
                            .text_color(secondary).text_size(13.0))
                )
        ))
}

fn size_row(size_label: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_row().gap(16.0).items_center()
        .child(label(size_label).text_color(text_secondary).text_size(11.0).w(24.0))
        .child(content)
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
