//! BlockEditor specimen — block-based content editor.
//!
//! Mirrors the GPUI specimen (`packages/gpui/preview/src/specimens/block_editor_specimen.rs`)
//! against the contract (`docs/contracts/components/block-editor.md`). Every group
//! is a real `js_block_editor` driven by the spec's consumer block path
//! (`with_block_types` + `with_blocks`), which renders block content by type —
//! heading / paragraph / blockquote / code / list — through the per-block toolbar
//! (drag grip, ghost TypeSelect, move, AddSelect, remove). No `with_block_count`
//! placeholders, no hand-rolled boxes; all spacing/typography resolves from tokens.
//!
//! Note: the GPUI specimen's "Default blocks" / "Custom blocks" groups use GPUI's
//! legacy `BlockEditor::with_child(...)` host-child API (flagged legacy in
//! `docs/parity/block-editor.md`). Jetstream's `js_block_editor` has no host-child
//! channel; the faithful equivalent is the spec block path, which is what these
//! groups exercise. Editing / add / remove / reorder are preview-event-loop bound.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::block_editor::js_block_editor;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{
    BlockEditorMode, BlockEditorSpec, BlockTypeDefinition, EditorBlock,
};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    // Shared block vocabulary (consumer-owned per contract — the shell is a
    // pure host for these types).
    let block_types = || {
        vec![
            BlockTypeDefinition::new("heading", "Heading", "heading-1"),
            BlockTypeDefinition::new("paragraph", "Paragraph", "text"),
            BlockTypeDefinition::new("quote", "Quote", "quote"),
            BlockTypeDefinition::new("code", "Code", "code"),
            BlockTypeDefinition::new("list", "List", "list"),
        ]
    };

    // ── 1. Default blocks — heading / paragraph / blockquote / code ─────
    let default_editor = js_block_editor(
        &BlockEditorSpec::new()
            .with_block_types(block_types())
            .with_blocks(vec![
                EditorBlock::new("d1", "heading").with_content("Getting Started"),
                EditorBlock::new("d2", "paragraph").with_content(
                    "This is a paragraph block. Each block can be reordered, changed, or \
                     removed using the toolbar that appears on hover.",
                ),
                EditorBlock::new("d3", "quote")
                    .with_content("A blockquote block for highlighted content."),
                EditorBlock::new("d4", "code")
                    .with_content("fn main() { println!(\"Hello, world!\"); }"),
            ]),
        theme,
    );

    // ── 2. Consumer-driven block types (full vocabulary + list) ─────────
    let consumer_editor = js_block_editor(
        &BlockEditorSpec::new()
            .with_block_types(block_types())
            .with_blocks(vec![
                EditorBlock::new("b1", "heading").with_content("Project Roadmap"),
                EditorBlock::new("b2", "paragraph").with_content(
                    "Each block renders by type. The per-block toolbar carries a ghost \
                     TypeSelect plus add/move/remove controls.",
                ),
                EditorBlock::new("b3", "quote")
                    .with_content("\"Pure shell components let consumers own the block vocabulary.\""),
                EditorBlock::new("b4", "code")
                    .with_content("const blocks: EditorBlock[] = [];"),
                EditorBlock::new("b5", "list")
                    .with_content("First item\nSecond item\nThird item"),
            ]),
        theme,
    );

    // ── 3. Single posture — reorder/add/remove hidden, TypeSelect inset ─
    let single_editor = js_block_editor(
        &BlockEditorSpec::new()
            .with_mode(BlockEditorMode::Single)
            .with_block_types(vec![
                BlockTypeDefinition::new("heading", "Heading", "heading-1"),
                BlockTypeDefinition::new("paragraph", "Paragraph", "text"),
            ])
            .with_blocks(vec![EditorBlock::new("s1", "paragraph").with_content(
                "Single posture hides reorder/add/remove; only the TypeSelect remains, \
                 inset to align with content.",
            )]),
        theme,
    );

    // ── 4. Disabled — dimmed via state.opacity.disabled ─────────────────
    let disabled_editor = js_block_editor(
        &BlockEditorSpec::new()
            .with_block_types(block_types())
            .with_blocks(vec![
                EditorBlock::new("x1", "heading").with_content("Read-only document"),
                EditorBlock::new("x2", "paragraph")
                    .with_content("The whole editor dims when disabled; controls are inert."),
            ])
            .with_disabled(true),
        theme,
    );

    div().flex_col().gap(24.0)
        .child(group("Default blocks (heading / paragraph / quote / code)", secondary,
            div().w(560.0).child(default_editor)
        ))
        .child(group("Consumer-driven block types", secondary,
            div().w(560.0).child(consumer_editor)
        ))
        .child(group("Single posture (multi-block controls hidden)", secondary,
            div().w(560.0).child(single_editor)
        ))
        .child(group("Disabled", secondary,
            div().w(560.0).child(disabled_editor)
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
