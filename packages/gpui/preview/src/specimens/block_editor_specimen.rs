use crate::style_bridge::color_to_hsla;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{BlockEditor, Eyebrow};
use poodle_specs::EyebrowSpec;
use poodle_specs::{BlockEditorMode, BlockEditorSpec, BlockTypeDefinition, EditorBlock};

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
        // --- Consumer-driven block types ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Consumer-driven block types"), theme))
                .child(
                    BlockEditor::from_spec(
                        BlockEditorSpec::new()
                            .with_block_types(vec![
                                BlockTypeDefinition::new("heading", "Heading", "heading-1"),
                                BlockTypeDefinition::new("paragraph", "Paragraph", "text"),
                                BlockTypeDefinition::new("quote", "Quote", "quote"),
                                BlockTypeDefinition::new("code", "Code", "code"),
                            ])
                            .with_blocks(vec![
                                EditorBlock::new("b1", "heading")
                                    .with_content("Project Roadmap"),
                                EditorBlock::new("b2", "paragraph")
                                    .with_content("Each block renders by type. The per-block toolbar carries a ghost TypeSelect plus add/move/remove controls."),
                                EditorBlock::new("b3", "quote")
                                    .with_content("\"Pure shell components let consumers own the block vocabulary.\""),
                                EditorBlock::new("b4", "code")
                                    .with_content("const blocks: EditorBlock[] = [];"),
                                EditorBlock::new("b5", "list")
                                    .with_content("First item\nSecond item\nThird item"),
                            ]),
                        theme,
                    )
                )
        )
        // --- Single posture (multi-block controls hidden) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Single posture"), theme))
                .child(
                    BlockEditor::from_spec(
                        BlockEditorSpec::new()
                            .with_mode(BlockEditorMode::Single)
                            .with_block_types(vec![
                                BlockTypeDefinition::new("heading", "Heading", "heading-1"),
                                BlockTypeDefinition::new("paragraph", "Paragraph", "text"),
                            ])
                            .with_blocks(vec![
                                EditorBlock::new("s1", "paragraph")
                                    .with_content("Single posture hides reorder/add/remove; only the TypeSelect remains, inset to align with content."),
                            ]),
                        theme,
                    )
                )
        )
}
