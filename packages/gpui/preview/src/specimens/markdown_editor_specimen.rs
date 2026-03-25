use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_primitives::EyebrowSpec;
use poodle_composites::MarkdownEditorSpec;
use poodle_gpui_components::{MarkdownEditor, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let md_mode = state.specimens.text.get("editor-md-mode")
        .cloned()
        .unwrap_or_else(|| "split".to_string());
    let md_value = state.specimens.text.get("editor-md-value")
        .cloned()
        .unwrap_or_else(|| "# Hello World\n\nThis is a **markdown** editor with _split_ view mode.\n\n- Item one\n- Item two\n- Item three".to_string());

    div().flex().flex_col().gap(px(24.0))
        // --- Interactive ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Interactive"), theme))
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
        // --- Edit only ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Edit only"), theme))
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
        // --- Preview only ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Preview only"), theme))
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
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
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
}
