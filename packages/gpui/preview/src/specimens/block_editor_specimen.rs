use crate::app_state::AppState;
use crate::node_compat::{BlockEditor, Eyebrow};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::{FontFamily, LayoutDirection, Node};
use poodle_render::RenderContext;
use poodle_specs::EyebrowSpec;
use poodle_specs::{BlockEditorMode, BlockEditorSpec, BlockTypeDefinition, EditorBlock};
use poodle_tokens::typed::ColorValue;

/// A caller-owned block body: a text run at a declared size, weight and tone.
///
/// These are the consumer-supplied bodies BlockEditor's slot path takes — the
/// spec-driven examples further down carry no children at all.
fn text_block(text: &str, size: f32, color: ColorValue) -> Node {
    let mut node = Node::text(text.to_string());
    node.style.text_size = Some(size);
    node.style.descriptor.text_color = Some(color);
    node
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let examples = div().flex().flex_col().gap(px(24.0))
        // --- Default blocks ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default blocks"), theme))
                .child(
                    BlockEditor::from_spec(BlockEditorSpec::new(), theme, "block-editor-1")
                        .with_child(|ctx: &RenderContext<'_>| {
                            let mut heading = text_block(
                                "Getting Started",
                                18.0,
                                ctx.theme().resolve_color("color.text.primary"),
                            );
                            heading.style.text_weight = Some(700);
                            heading
                        })
                        .with_child(|ctx: &RenderContext<'_>| {
                            text_block(
                                "This is a paragraph block. Each block can be reordered, changed, or removed using the toolbar that appears on hover.",
                                14.0,
                                ctx.theme().resolve_color("color.text.primary"),
                            )
                        })
                        .with_child(|ctx: &RenderContext<'_>| {
                            let text_secondary =
                                ctx.theme().resolve_color("color.text.secondary");
                            // Blockquote: italic, tinted left rule, inset text.
                            let mut quote = Node::container();
                            {
                                let s = &mut quote.style;
                                s.descriptor.layout.direction = LayoutDirection::Column;
                                s.descriptor.layout.spacing.padding.left = 12.0;
                                s.border_left_width = Some(2.0);
                                s.border_color_left =
                                    Some(ColorValue(text_secondary.0, text_secondary.1, text_secondary.2, text_secondary.3 * 0.3));
                                s.text_italic = true;
                            }
                            quote.child(text_block(
                                "A blockquote block for highlighted content.",
                                14.0,
                                text_secondary,
                            ))
                        })
                        .with_child(|ctx: &RenderContext<'_>| {
                            let text_primary = ctx.theme().resolve_color("color.text.primary");
                            let text_secondary =
                                ctx.theme().resolve_color("color.text.secondary");
                            // Code block: mono face on a faint panel.
                            let mut code = Node::container();
                            {
                                let s = &mut code.style;
                                s.descriptor.layout.direction = LayoutDirection::Column;
                                s.descriptor.background = Some(ColorValue(
                                    text_secondary.0,
                                    text_secondary.1,
                                    text_secondary.2,
                                    text_secondary.3 * 0.08,
                                ));
                                let c = &mut s.descriptor.corner_radii;
                                c.top_left = 4.0;
                                c.top_right = 4.0;
                                c.bottom_right = 4.0;
                                c.bottom_left = 4.0;
                                let pad = &mut s.descriptor.layout.spacing.padding;
                                pad.left = 10.0;
                                pad.right = 10.0;
                                pad.top = 8.0;
                                pad.bottom = 8.0;
                                s.font_family = Some(FontFamily::Mono);
                            }
                            code.child(text_block(
                                "fn main() {\n    println!(\"Hello, world!\");\n}",
                                13.0,
                                text_primary,
                            ))
                        })
                )
        )
        // --- Custom blocks ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Custom blocks"), theme))
                .child(
                    BlockEditor::from_spec(BlockEditorSpec::new(), theme, "block-editor-2")
                        .with_child(|ctx: &RenderContext<'_>| {
                            text_block(
                                "A text block with regular content.",
                                14.0,
                                ctx.theme().resolve_color("color.text.primary"),
                            )
                        })
                        .with_child(|ctx: &RenderContext<'_>| {
                            let text_primary = ctx.theme().resolve_color("color.text.primary");
                            let accent = ctx.theme().resolve_color("color.accent.base");
                            // Callout: accent left rule over an accent wash.
                            let mut callout = Node::container();
                            {
                                let s = &mut callout.style;
                                s.descriptor.layout.direction = LayoutDirection::Column;
                                s.descriptor.background =
                                    Some(ColorValue(accent.0, accent.1, accent.2, accent.3 * 0.08));
                                s.border_left_width = Some(2.0);
                                s.border_color_left = Some(accent);
                                let c = &mut s.descriptor.corner_radii;
                                c.top_left = 4.0;
                                c.top_right = 4.0;
                                c.bottom_right = 4.0;
                                c.bottom_left = 4.0;
                                let pad = &mut s.descriptor.layout.spacing.padding;
                                pad.left = 12.0;
                                pad.right = 12.0;
                                pad.top = 8.0;
                                pad.bottom = 8.0;
                            }
                            let mut label = text_block("Callout", 12.0, accent);
                            label.style.text_weight = Some(600);
                            label.style.descriptor.layout.spacing.margin.bottom = 4.0;
                            callout.child(label).child(text_block(
                                "This is a callout block with custom styling.",
                                14.0,
                                text_primary,
                            ))
                        })
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
                        "block-editor-3",
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
                        "block-editor-4",
                    )
                )
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "block-editor",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                BlockEditor::from_spec(BlockEditorSpec::new().with_size(size), theme, "block-editor-5")
                    .with_child(|ctx: &RenderContext<'_>| {
                        text_block(
                            "A text block with regular content.",
                            14.0,
                            ctx.theme().resolve_color("color.text.primary"),
                        )
                    })
                    .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                BlockEditor::from_spec(BlockEditorSpec::new().with_density(density), theme, "block-editor-6")
                    .with_child(|ctx: &RenderContext<'_>| {
                        text_block(
                            "A text block with regular content.",
                            14.0,
                            ctx.theme().resolve_color("color.text.primary"),
                        )
                    })
                    .into_any_element()
            }),
    )
}
