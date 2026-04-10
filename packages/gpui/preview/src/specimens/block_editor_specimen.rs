use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_primitives::EyebrowSpec;
use poodle_composites::BlockEditorSpec;
use poodle_gpui_components::{BlockEditor, Eyebrow};
use poodle_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");

    div().flex().flex_col().gap(px(24.0))
        // --- Default blocks ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default blocks"), theme))
                .child(
                    BlockEditor::from_spec(BlockEditorSpec::new(), theme)
                        .with_child(
                            div().text_size(px(18.0)).font_weight(FontWeight::BOLD)
                                .text_color(color_to_hsla(text_primary))
                                .child("Getting Started")
                        )
                        .with_child(
                            div().text_size(px(14.0))
                                .text_color(color_to_hsla(text_primary))
                                .child("This is a paragraph block. Each block can be reordered, changed, or removed using the toolbar that appears on hover.")
                        )
                        .with_child(
                            div().text_size(px(14.0)).italic()
                                .text_color(color_to_hsla(text_secondary))
                                .pl(px(12.0))
                                .border_l_2()
                                .border_color(color_to_hsla(text_secondary).opacity(0.3))
                                .child("A blockquote block for highlighted content.")
                        )
                        .with_child(
                            div().text_size(px(13.0))
                                .font_family("Monaco, Menlo, monospace")
                                .text_color(color_to_hsla(text_primary))
                                .bg(color_to_hsla(text_secondary).opacity(0.08))
                                .rounded(px(4.0))
                                .px(px(10.0)).py(px(8.0))
                                .child("fn main() {\n    println!(\"Hello, world!\");\n}")
                        )
                )
        )
        // --- Custom blocks ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Custom blocks"), theme))
                .child(
                    BlockEditor::from_spec(BlockEditorSpec::new(), theme)
                        .with_child(
                            div().text_size(px(14.0))
                                .text_color(color_to_hsla(text_primary))
                                .child("A text block with regular content.")
                        )
                        .with_child(
                            div()
                                .bg(color_to_hsla(theme.resolve_color("color.accent.base")).opacity(0.08))
                                .border_l_2()
                                .border_color(color_to_hsla(theme.resolve_color("color.accent.base")))
                                .rounded(px(4.0))
                                .px(px(12.0)).py(px(8.0))
                                .child(
                                    div().text_size(px(12.0)).font_weight(FontWeight::SEMIBOLD)
                                        .text_color(color_to_hsla(theme.resolve_color("color.accent.base")))
                                        .mb(px(4.0))
                                        .child("Callout")
                                )
                                .child(
                                    div().text_size(px(14.0))
                                        .text_color(color_to_hsla(text_primary))
                                        .child("This is a callout block with custom styling.")
                                )
                        )
                )
        )
}
