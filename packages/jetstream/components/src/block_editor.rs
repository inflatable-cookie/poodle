//! BlockEditor — Jetstream block editor backed by BlockEditorSpec.
//!
//! Contract: `docs/contracts/components/block-editor.md`
//! Reference: `packages/svelte/components/src/BlockEditor.svelte`
//! Mirrors: `packages/gpui/components/src/composites/block_editor.rs`
//!
//! Pure shell: renders the contract chrome (per-block toolbar with drag grip,
//! TypeSelect, move buttons, AddSelect, remove button) plus a content area that
//! renders each block by its type. Block payloads are consumer-owned; the shell
//! resolves all spacing/typography from tokens (recipe rems via `rem_to_px`).
//!
//! Interactivity (editing, type change, add/remove/reorder via the selects and
//! buttons) is preview-event-loop bound and intentionally not wired here — the
//! controls render at the current spec state. ARIA (`role="group"`, aria-labels)
//! is N/A (Jetstream has no accessibility channel).

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    BlockEditorMode, BlockEditorSpec, ChoiceOption, ControlDensity, ControlSize, EditorBlock,
    SelectSpec, SelectVariant,
};

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::select::js_select;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius, tint};

/// Contract `--poodle-block-editor-control-size` per size (rem).
fn control_size_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.25,
        ControlSize::Sm => 1.5,
        ControlSize::Md => 1.75,
        ControlSize::Lg => 2.0,
        ControlSize::Xl => 2.25,
    }
}

/// Contract density recipe rems: (toolbar-y, toolbar-x, content-x, content-y,
/// stack-gap, input-x). toolbar-gap is the same across densities (`0.125rem`).
fn density_recipe(density: ControlDensity) -> (f32, f32, f32, f32, f32, f32) {
    match density {
        // toolbar-y, toolbar-x, content-x, content-y, stack-gap, input-x
        ControlDensity::Compact => (0.1875, 0.25, 0.375, 0.25, 0.375, 0.25),
        ControlDensity::Default => (0.25, 0.375, 0.5, 0.375, 0.5, 0.375),
        ControlDensity::Comfortable => (0.3125, 0.5, 0.625, 0.5, 0.625, 0.5),
    }
}

/// Build a contract tool button (`.block-editor__tool-btn`): control-size box,
/// rounded, icon glyph. `disabled` dims to the contract `0.3` opacity.
#[allow(clippy::too_many_arguments)]
fn tool_btn(
    icon_name: &str,
    icon_color: glam::Vec4,
    icon_size: f32,
    control_size: f32,
    radius: f32,
    disabled: bool,
) -> JsEl {
    let mut btn = ui_element::div()
        .flex_row()
        .items_center()
        .justify_center()
        .w(control_size)
        .h(control_size)
        .rounded(radius)
        .child(
            ui_element::icon(icon_name)
                .size(icon_size)
                .text_color(icon_color),
        );

    if disabled {
        // Contract `.block-editor__tool-btn:disabled` opacity 0.3.
        btn = btn.opacity(0.3);
    } else {
        btn = btn.cursor_pointer();
    }
    btn
}

/// Build a ghost Select seeded from `block_types`. When `value` is `Some`,
/// the trigger shows the matching label (TypeSelect); when `None` the menu
/// is the bare add-block picker (AddSelect). Callbacks are preview-loop bound
/// and intentionally omitted. Per-option icons are not rendered — `ChoiceOption`
/// carries no icon field (accepted Select-primitive gap, as in GPUI).
fn build_type_select(
    spec: &BlockEditorSpec,
    value: Option<&str>,
    disabled: bool,
    theme: &JetstreamThemeProvider,
) -> JsEl {
    let options: Vec<ChoiceOption> = spec
        .block_types
        .iter()
        .map(|t| ChoiceOption::new(t.block_type.clone(), t.label.clone()))
        .collect();

    let mut sel = SelectSpec::default();
    sel.options = options;
    sel.variant = SelectVariant::Ghost;
    sel.menu_min_width = Some(String::from("10rem"));
    sel.is_disabled = disabled;
    sel.size = spec.size;
    sel.size_role = spec.size_role;
    sel.density = spec.density;
    if let Some(v) = value {
        sel.value = Some(v.to_string());
    }

    js_select(&sel, theme)
}

/// Render a single block's content by its type. Block payloads are
/// consumer-owned; this renders the legacy `content` string with type-aware
/// presentation (heading bold / quote accent-left-border / code monospace +
/// elevated bg / list bulleted / paragraph body). Unknown types fall back to
/// body paragraph rendering. `content` font-size is the contract input
/// `0.875rem`; heading/code use the GPUI sibling's display sizes.
#[allow(clippy::too_many_arguments)]
fn render_block_content(
    block: &EditorBlock,
    content_x: f32,
    content_y: f32,
    text_primary: glam::Vec4,
    text_secondary: glam::Vec4,
    accent: glam::Vec4,
    code_bg: glam::Vec4,
    radius_control: f32,
) -> JsEl {
    let body_size = rem_to_px(0.875); // contract input font-size
    let heading_size = rem_to_px(1.125);
    let code_size = rem_to_px(0.8125);

    // Content padding (`.block-editor__content`), min-height 1.5rem.
    let container = ui_element::div()
        .flex_col()
        .pl(content_x)
        .pr(content_x)
        .pt(content_y)
        .pb(content_y)
        .min_h(rem_to_px(1.5));

    let text = block.content.clone().unwrap_or_default();

    match block.block_type.as_str() {
        "heading" | "heading-1" | "heading-2" => container.child(
            ui_element::label(&text)
                .text_size(heading_size)
                .text_weight(700)
                .text_color(text_primary),
        ),
        "quote" | "blockquote" => container.child(
            // Accent left border + italic-substitute (secondary text).
            ui_element::div()
                .border_l(2.0)
                .border_color(accent)
                .pl(content_x)
                .child(
                    ui_element::label(&text)
                        .text_size(body_size)
                        .text_color(text_secondary),
                ),
        ),
        "code" => container.child(
            // Elevated background + monospace-substitute (mono glyphs unavailable;
            // closest faithful subset: elevated panel + body text).
            ui_element::div()
                .bg(code_bg)
                .rounded(radius_control)
                .pl(content_x)
                .pr(content_x)
                .pt(content_y)
                .pb(content_y)
                .child(
                    ui_element::label(&text)
                        .text_size(code_size)
                        .text_color(text_primary),
                ),
        ),
        "list" | "bulleted-list" => {
            let mut list = ui_element::div().flex_col();
            for line in text.lines() {
                list = list.child(
                    ui_element::label(&format!("\u{2022} {}", line))
                        .text_size(body_size)
                        .text_color(text_primary),
                );
            }
            container.child(list)
        }
        // paragraph / unknown → body paragraph
        _ => container.child(
            ui_element::label(&text)
                .text_size(body_size)
                .text_color(text_primary),
        ),
    }
}

pub fn js_block_editor(spec: &BlockEditorSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // ── Resolve chrome from tokens / contract recipe rems ───────────────────
    let fill = resolve_color(theme, spec.fill_token());
    let radius_control = resolve_radius(theme, "radius.control");
    let elevated = resolve_color(theme, "color.background.elevated");
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let text_tertiary = resolve_color(theme, "color.text.tertiary");
    let accent = resolve_color(theme, "color.accent.base");

    let control_size = rem_to_px(control_size_rem(effective_size));
    let (toolbar_y, toolbar_x, content_x, content_y, stack_gap, input_x) =
        density_recipe(spec.density);
    let toolbar_y = rem_to_px(toolbar_y);
    let toolbar_x = rem_to_px(toolbar_x);
    let content_x = rem_to_px(content_x);
    let content_y = rem_to_px(content_y);
    let stack_gap = rem_to_px(stack_gap);
    let input_x = rem_to_px(input_x);
    let toolbar_gap = rem_to_px(0.125); // `--poodle-block-editor-toolbar-gap`
    let icon_size = rem_to_px(0.75); // `.tool-btn` font-size 0.75rem

    // Per-block background mix (contract: elevated 42%).
    let block_bg = tint(elevated, 0.42);
    // Code block panel: elevated 40% (mirrors GPUI sibling).
    let code_bg = tint(elevated, 0.4);

    // ── Posture flags (mode + explicit overrides) ───────────────────────────
    let is_multi = spec.mode == BlockEditorMode::Multi;
    let can_reorder = spec.allow_reorder.unwrap_or(is_multi);
    let can_add = spec.allow_add.unwrap_or(is_multi);
    let can_remove = spec.allow_remove.unwrap_or(is_multi);
    let can_type_change = spec.allow_type_change.unwrap_or(true);
    let disabled = spec.is_disabled;

    // ── Root: flex column, surface bg, stack-gap, no border/radius/pad ──────
    let mut root = ui_element::div().flex_col().gap(stack_gap).bg(fill);

    let block_count = spec.blocks.len();
    for (i, block) in spec.blocks.iter().enumerate() {
        // ── Toolbar left: drag grip + TypeSelect ───────────────────────────
        let mut toolbar_left = ui_element::div()
            .flex_row()
            .items_center()
            .gap(toolbar_gap);

        if can_reorder {
            // `.block-editor__drag-grip` — control-size box, tertiary grip icon.
            toolbar_left = toolbar_left.child(
                ui_element::div()
                    .flex_row()
                    .items_center()
                    .justify_center()
                    .w(control_size)
                    .h(control_size)
                    .rounded(radius_control)
                    .child(
                        ui_element::icon("grip-vertical")
                            .size(icon_size)
                            .text_color(text_tertiary),
                    ),
            );
        }

        if can_type_change {
            // TypeSelect inset margin-left when reorder is disabled (grip hidden)
            // to keep alignment with block content:
            // calc(content-x + input-x − toolbar-x).
            let mut type_wrap = ui_element::div().flex_shrink_0();
            if !can_reorder {
                let inset = content_x + input_x - toolbar_x;
                if inset > 0.0 {
                    type_wrap = type_wrap.ml(inset);
                }
            }
            toolbar_left = toolbar_left
                .child(type_wrap.child(build_type_select(spec, Some(&block.block_type), disabled, theme)));
        }

        // ── Toolbar right: move up/down, AddSelect, remove ─────────────────
        let mut toolbar_right = ui_element::div()
            .flex_row()
            .items_center()
            .gap(toolbar_gap);

        if can_reorder {
            toolbar_right = toolbar_right
                .child(tool_btn(
                    "arrow-up",
                    text_tertiary,
                    icon_size,
                    control_size,
                    radius_control,
                    disabled || i == 0,
                ))
                .child(tool_btn(
                    "arrow-down",
                    text_tertiary,
                    icon_size,
                    control_size,
                    radius_control,
                    disabled || i == block_count - 1,
                ));
        }

        if can_add {
            // AddSelect: ghost Select with plus-icon trigger. The Select primitive
            // has no trigger-slot override, so the plus tool button sits ahead of
            // the (value-less) picker — closest faithful subset of the contract's
            // trigger-slot pattern (mirrors GPUI).
            toolbar_right = toolbar_right
                .child(tool_btn(
                    "plus",
                    text_tertiary,
                    icon_size,
                    control_size,
                    radius_control,
                    disabled,
                ))
                .child(
                    ui_element::div()
                        .flex_shrink_0()
                        .child(build_type_select(spec, None, disabled, theme)),
                );
        }

        if can_remove && block_count > 1 {
            toolbar_right = toolbar_right.child(tool_btn(
                "x",
                text_tertiary,
                icon_size,
                control_size,
                radius_control,
                disabled,
            ));
        }

        let toolbar = ui_element::div()
            .flex_row()
            .items_center()
            .justify_between()
            .pt(toolbar_y)
            .pb(toolbar_y)
            .pl(toolbar_x)
            .pr(toolbar_x)
            .child(toolbar_left)
            .child(ui_element::div().grow())
            .child(toolbar_right);

        // ── Block: flex column, elevated-42% bg, control radius ────────────
        let block_el = ui_element::div()
            .flex_col()
            .rounded(radius_control)
            .bg(block_bg)
            .child(toolbar)
            .child(render_block_content(
                block,
                content_x,
                content_y,
                text_primary,
                text_secondary,
                accent,
                code_bg,
                radius_control,
            ));

        root = root.child(block_el);
    }

    if disabled {
        // Contract `.block-editor--disabled` opacity — resolved from token.
        root = root.opacity(resolve_opacity(theme, "state.opacity.disabled"));
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{BlockTypeDefinition, EditorBlock};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn block_types() -> Vec<BlockTypeDefinition> {
        vec![
            BlockTypeDefinition::new("paragraph", "Paragraph", "text"),
            BlockTypeDefinition::new("heading", "Heading", "heading-1"),
            BlockTypeDefinition::new("quote", "Quote", "quote"),
            BlockTypeDefinition::new("code", "Code", "code"),
            BlockTypeDefinition::new("list", "List", "list"),
        ]
    }

    fn spec_with_blocks() -> BlockEditorSpec {
        BlockEditorSpec::new()
            .with_block_types(block_types())
            .with_blocks(vec![
                EditorBlock::new("b1", "heading").with_content("Title here"),
                EditorBlock::new("b2", "paragraph").with_content("Body paragraph text"),
                EditorBlock::new("b3", "quote").with_content("A quoted line"),
                EditorBlock::new("b4", "code").with_content("let x = 1;"),
                EditorBlock::new("b5", "list").with_content("first\nsecond"),
            ])
    }

    #[test]
    fn renders_block_content_text() {
        let el = js_block_editor(&spec_with_blocks(), &theme());
        let tree = probe(&el, 600.0, 600.0);

        assert!(!tree.is_empty(), "probe produced no nodes");
        assert!(
            tree.has_text("Title here"),
            "heading content missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Body paragraph text"),
            "paragraph content missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("A quoted line"),
            "quote content missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("let x = 1;"),
            "code content missing: {:?}",
            tree.texts()
        );
        // List lines get a bullet prefix.
        assert!(
            tree.has_text("\u{2022} first") && tree.has_text("\u{2022} second"),
            "list lines missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn type_select_renders_block_type_value() {
        // TypeSelect trigger should show the per-block type label, seeded from
        // block_types (value = block.block_type).
        let el = js_block_editor(&spec_with_blocks(), &theme());
        let tree = probe(&el, 600.0, 600.0);

        // Each block's type label appears in its TypeSelect trigger.
        assert!(tree.has_text("Heading"), "Heading type label missing: {:?}", tree.texts());
        assert!(tree.has_text("Paragraph"), "Paragraph type label missing: {:?}", tree.texts());
        assert!(tree.has_text("Quote"), "Quote type label missing: {:?}", tree.texts());
        assert!(tree.has_text("Code"), "Code type label missing: {:?}", tree.texts());
    }

    #[test]
    fn renders_toolbar_icons_not_unicode_glyphs() {
        let el = js_block_editor(&spec_with_blocks(), &theme());
        let tree = probe(&el, 600.0, 600.0);

        // Real icon-registry icons present (Icon widgets carry their name as text).
        assert!(tree.has_text("grip-vertical"), "grip icon missing: {:?}", tree.texts());
        assert!(tree.has_text("arrow-up"), "move-up icon missing: {:?}", tree.texts());
        assert!(tree.has_text("arrow-down"), "move-down icon missing: {:?}", tree.texts());
        assert!(tree.has_text("plus"), "add icon missing: {:?}", tree.texts());
        assert!(tree.has_text("x"), "remove icon missing: {:?}", tree.texts());

        // NO unicode-glyph fallback remains (old impl used ↑ ↓ + ×).
        for glyph in ["\u{2191}", "\u{2193}", "\u{00D7}"] {
            assert!(
                !tree.has_text(glyph),
                "unicode glyph {:?} should not appear: {:?}",
                glyph,
                tree.texts()
            );
        }
    }

    #[test]
    fn move_gating_first_last_and_remove_hidden_for_single_block() {
        // Multi-block: move buttons present, remove present.
        let multi = js_block_editor(&spec_with_blocks(), &theme());
        let multi_tree = probe(&multi, 600.0, 600.0);
        // 5 blocks → 5 up + 5 down arrows, 5 plus, 5 grip, remove on each.
        assert_eq!(multi_tree.texts().iter().filter(|t| **t == "arrow-up").count(), 5);
        assert_eq!(multi_tree.texts().iter().filter(|t| **t == "arrow-down").count(), 5);
        assert!(multi_tree.has_text("x"), "remove should be present with >1 block");

        // Single block: remove hidden, move/add still render (gating is opacity,
        // not removal, so they remain in the tree).
        let single = BlockEditorSpec::new()
            .with_block_types(block_types())
            .with_blocks(vec![EditorBlock::new("only", "paragraph").with_content("solo")]);
        let single_tree = probe(&js_block_editor(&single, &theme()), 600.0, 600.0);
        assert!(
            !single_tree.has_text("x"),
            "remove icon must be hidden when block_count <= 1: {:?}",
            single_tree.texts()
        );
    }

    #[test]
    fn single_posture_hides_reorder_add_remove() {
        // mode=Single → reorder/add/remove default off; type-change defaults on.
        let single = BlockEditorSpec::new()
            .with_mode(BlockEditorMode::Single)
            .with_block_types(block_types())
            .with_blocks(vec![
                EditorBlock::new("b1", "heading").with_content("H"),
                EditorBlock::new("b2", "paragraph").with_content("P"),
            ]);
        let tree = probe(&js_block_editor(&single, &theme()), 600.0, 600.0);

        // No grip / move / add / remove chrome.
        assert!(!tree.has_text("grip-vertical"), "single posture should hide grip");
        assert!(!tree.has_text("arrow-up"), "single posture should hide move-up");
        assert!(!tree.has_text("arrow-down"), "single posture should hide move-down");
        assert!(!tree.has_text("plus"), "single posture should hide add");
        assert!(!tree.has_text("x"), "single posture should hide remove");
        // TypeSelect still present (type-change defaults on).
        assert!(tree.has_text("Heading"), "type select should remain: {:?}", tree.texts());
    }

    #[test]
    fn empty_blocks_render_no_block_chrome() {
        // No blocks → empty surface, no toolbar/content nodes.
        let el = js_block_editor(&BlockEditorSpec::new().with_block_types(block_types()), &theme());
        let tree = probe(&el, 600.0, 600.0);
        assert!(!tree.is_empty(), "root should still render");
        assert!(!tree.has_text("arrow-up"), "no blocks → no toolbar");
    }
}
