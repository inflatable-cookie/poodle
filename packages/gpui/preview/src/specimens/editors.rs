use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_primitives::EyebrowSpec;
use poodle_composites::{MarkdownEditorSpec, BlockEditorSpec};
use poodle_gpui_components::{MarkdownEditor, BlockEditor, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let text_primary = theme.resolve_color("semantic.color.text.primary");

    let md_mode = state.specimens.text.get("editor-md-mode")
        .cloned()
        .unwrap_or_else(|| "split".to_string());
    let md_value = state.specimens.text.get("editor-md-value")
        .cloned()
        .unwrap_or_else(|| "# Hello World\n\nThis is a **markdown** editor with _split_ view mode.\n\n- Item one\n- Item two\n- Item three".to_string());

    div().flex().flex_col().gap(px(24.0))
        // --- Markdown Editor: Interactive ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Markdown editor: Interactive"), theme))
                .child(
                    MarkdownEditor::from_spec(
                        MarkdownEditorSpec::new()
                            .with_value(&md_value)
                            .with_mode(&md_mode),
                        theme,
                    )
                    .on_change(cx.listener(|this, val: &str, _w, cx| {
                        this.state.specimens.text.insert("editor-md-value".to_string(), val.to_string());
                        cx.notify();
                    }))
                    .on_mode_change(cx.listener(|this, mode: &str, _w, cx| {
                        this.state.specimens.text.insert("editor-md-mode".to_string(), mode.to_string());
                        cx.notify();
                    }))
                )
                .child(
                    div().text_xs().text_color(color_to_hsla(text_secondary))
                        .child(format!("Mode: {} | Length: {} chars", md_mode, md_value.len()))
                )
        )
        // --- Markdown Editor: Edit only ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Markdown editor: Edit only"), theme))
                .child(
                    MarkdownEditor::from_spec(
                        MarkdownEditorSpec::new()
                            .with_placeholder("Start writing markdown...")
                            .with_mode("edit")
                            .with_min_height("120px"),
                        theme,
                    )
                )
        )
        // --- Markdown Editor: Preview only ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Markdown editor: Preview only"), theme))
                .child(
                    MarkdownEditor::from_spec(
                        MarkdownEditorSpec::new()
                            .with_value("# Preview Mode\n\nThis editor is in preview-only mode. The toolbar still shows mode switches.")
                            .with_mode("preview")
                            .with_min_height("100px"),
                        theme,
                    )
                )
        )
        // --- Markdown Editor: Disabled ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Markdown editor: Disabled"), theme))
                .child(
                    MarkdownEditor::from_spec(
                        MarkdownEditorSpec::new()
                            .with_value("This content cannot be edited.")
                            .with_disabled(true)
                            .with_min_height("80px"),
                        theme,
                    )
                )
        )
        // --- Block Editor: Default blocks ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Block editor: Default blocks"), theme))
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
        // --- Block Editor: Custom blocks ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Block editor: Custom blocks"), theme))
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
        )
}
