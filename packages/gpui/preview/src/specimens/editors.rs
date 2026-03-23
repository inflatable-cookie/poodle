use gpui::*;
use pug_adapter::ThemeProvider;
use pug_composites::{MarkdownEditorSpec, BlockEditorSpec};
use pug_gpui_components::{MarkdownEditor, BlockEditor};
use pug_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let text_primary = theme.resolve_color("semantic.color.text.primary");

    div().flex().flex_col().gap(px(24.0))
        // --- Markdown Editor: Split view ---
        .child(section_label("MARKDOWN EDITOR: SPLIT VIEW", text_secondary))
        .child(
            MarkdownEditor::from_spec(
                MarkdownEditorSpec::new()
                    .with_value("# Hello World\n\nThis is a **markdown** editor with _split_ view mode.\n\n- Item one\n- Item two\n- Item three")
                    .with_mode("split"),
                theme,
            )
        )
        // --- Markdown Editor: Edit only ---
        .child(section_label("MARKDOWN EDITOR: EDIT ONLY", text_secondary))
        .child(
            MarkdownEditor::from_spec(
                MarkdownEditorSpec::new()
                    .with_placeholder("Start writing markdown...")
                    .with_mode("edit")
                    .with_min_height("120px"),
                theme,
            )
        )
        // --- Markdown Editor: Preview only ---
        .child(section_label("MARKDOWN EDITOR: PREVIEW ONLY", text_secondary))
        .child(
            MarkdownEditor::from_spec(
                MarkdownEditorSpec::new()
                    .with_value("# Preview Mode\n\nThis editor is in preview-only mode. The toolbar still shows mode switches.")
                    .with_mode("preview")
                    .with_min_height("100px"),
                theme,
            )
        )
        // --- Markdown Editor: Disabled ---
        .child(section_label("MARKDOWN EDITOR: DISABLED", text_secondary))
        .child(
            MarkdownEditor::from_spec(
                MarkdownEditorSpec::new()
                    .with_value("This content cannot be edited.")
                    .with_disabled(true)
                    .with_min_height("80px"),
                theme,
            )
        )
        // --- Block Editor: Default blocks ---
        .child(section_label("BLOCK EDITOR: DEFAULT BLOCKS", text_secondary))
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
        // --- Block Editor: Custom blocks ---
        .child(section_label("BLOCK EDITOR: CUSTOM BLOCKS", text_secondary))
        .child(
            BlockEditor::from_spec(BlockEditorSpec::new(), theme)
                .with_child(
                    div().text_size(px(14.0))
                        .text_color(color_to_hsla(text_primary))
                        .child("A text block with regular content.")
                )
                .with_child(
                    div()
                        .bg(color_to_hsla(theme.resolve_color("semantic.color.accent.base")).opacity(0.08))
                        .border_l_2()
                        .border_color(color_to_hsla(theme.resolve_color("semantic.color.accent.base")))
                        .rounded(px(4.0))
                        .px(px(12.0)).py(px(8.0))
                        .child(
                            div().text_size(px(12.0)).font_weight(FontWeight::SEMIBOLD)
                                .text_color(color_to_hsla(theme.resolve_color("semantic.color.accent.base")))
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
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
