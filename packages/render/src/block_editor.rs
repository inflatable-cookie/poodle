//! BlockEditor — per-block toolbar chrome + typed content area.
//!
//! Contract: `docs/contracts/components/block-editor.md`
//! Ported from: `packages/jetstream/components/src/block_editor.rs`.
//!
//! Pure shell: renders the contract chrome (per-block toolbar with drag grip,
//! TypeSelect, move buttons, AddSelect, remove button) plus a content area
//! that renders each block by its type. Block payloads are consumer-owned.
//! Interactivity is host-bound; the controls render at the current spec state.

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment,
    Node,
};
use poodle_specs::{
    BlockEditorMode, BlockEditorSpec, ChoiceOption, ControlDensity, ControlSize, EditorBlock,
    SelectSpec, SelectVariant,
};

use crate::color::with_alpha;
use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::select::{select, SelectHandlers};

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
fn tool_btn(
    icon_name: &str,
    icon_color: ColorValue,
    icon_size: f32,
    control_size: f32,
    radius: f32,
    disabled: bool,
) -> Node {
    let mut btn = Node::container();
    {
        let s = &mut btn.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.descriptor.layout.width = LayoutSizing::Fixed(control_size);
        s.descriptor.layout.height = LayoutSizing::Fixed(control_size);
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
        if disabled {
            // Contract `.block-editor__tool-btn:disabled` opacity 0.3.
            s.descriptor.opacity = 0.3;
        } else {
            s.descriptor.cursor = CursorHint::Pointer;
        }
    }
    let mut glyph = Node::icon(icon_name, icon_size);
    glyph.style.descriptor.text_color = Some(icon_color);
    btn.child(glyph)
}

/// Build a ghost Select seeded from `block_types`. When `value` is `Some`,
/// the trigger shows the matching label (TypeSelect); when `None` the menu
/// is the bare add-block picker (AddSelect). Per-option icons are not
/// rendered — `ChoiceOption` carries no icon field (accepted Select gap).
fn build_type_select(
    spec: &BlockEditorSpec,
    value: Option<&str>,
    disabled: bool,
    theme: &dyn ThemeProvider,
) -> Node {
    let options: Vec<ChoiceOption> = spec
        .block_types
        .iter()
        .map(|t| ChoiceOption::new(t.block_type.clone(), t.label.clone()))
        .collect();

    let mut sel = SelectSpec {
        options,
        variant: SelectVariant::Ghost,
        menu_min_width: Some(String::from("10rem")),
        is_disabled: disabled,
        size: spec.size,
        size_role: spec.size_role,
        density: spec.density,
        ..SelectSpec::default()
    };
    if let Some(v) = value {
        sel.value = Some(v.to_string());
    }

    select(&sel, theme, &SelectHandlers::default())
}

/// Render a single block's content by its type. Unknown types fall back to
/// body paragraph rendering. `content` font-size is the contract input
/// `0.875rem`; heading/code use the GPUI sibling's display sizes.
fn render_block_content(
    block: &EditorBlock,
    content_padding: (f32, f32),
    text_primary: ColorValue,
    text_secondary: ColorValue,
    accent: ColorValue,
    code_bg: ColorValue,
    radius_control: f32,
) -> Node {
    let (content_x, content_y) = content_padding;
    let body_size = rem_to_px(0.875); // contract input font-size
    let heading_size = rem_to_px(1.125);
    let code_size = rem_to_px(0.8125);

    // Content padding (`.block-editor__content`), min-height 1.5rem.
    let mut container = Node::container();
    {
        let s = &mut container.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = content_x;
        pad.right = content_x;
        pad.top = content_y;
        pad.bottom = content_y;
        s.min_height = Some(rem_to_px(1.5));
    }
    let text_content = block.content.clone().unwrap_or_default();

    let label = |content: &str, color, size, weight: Option<u16>| -> Node {
        let mut t = Node::text(content);
        t.style.descriptor.text_color = Some(color);
        t.style.text_size = Some(size);
        t.style.text_weight = weight;
        t
    };

    match block.block_type.as_str() {
        "heading" | "heading-1" | "heading-2" => {
            container.child(label(&text_content, text_primary, heading_size, Some(700)))
        }
        "quote" | "blockquote" => {
            // Accent left border + italic-substitute (secondary text).
            let mut quote = Node::container();
            {
                let s = &mut quote.style;
                // Explicit Row (see switch.rs).
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.border_left_width = Some(2.0);
                s.descriptor.border.color = accent;
                s.descriptor.layout.spacing.padding.left = content_x;
            }
            container.child(quote.child(label(&text_content, text_secondary, body_size, None)))
        }
        "code" => {
            // Elevated background + monospace-substitute (closest faithful
            // subset: elevated panel + body text).
            let mut frame = Node::container();
            {
                let s = &mut frame.style;
                // Explicit Row (see switch.rs).
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.background = Some(code_bg);
                let c = &mut s.descriptor.corner_radii;
                c.top_left = radius_control;
                c.top_right = radius_control;
                c.bottom_right = radius_control;
                c.bottom_left = radius_control;
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = content_x;
                pad.right = content_x;
                pad.top = content_y;
                pad.bottom = content_y;
            }
            container.child(frame.child(label(&text_content, text_primary, code_size, None)))
        }
        "list" | "bulleted-list" => {
            let mut list = Node::container();
            list.style.descriptor.layout.direction = LayoutDirection::Column;
            let mut list = list;
            for line in text_content.lines() {
                list = list.child(label(
                    &format!("\u{2022} {}", line),
                    text_primary,
                    body_size,
                    None,
                ));
            }
            container.child(list)
        }
        // paragraph / unknown → body paragraph
        _ => container.child(label(&text_content, text_primary, body_size, None)),
    }
}

pub fn block_editor(spec: &BlockEditorSpec, theme: &dyn ThemeProvider) -> Node {
    block_editor_with_children(spec, theme, Vec::new())
}

/// Block editor with caller-owned block bodies.
///
/// The spec drives blocks whenever `spec.blocks` is non-empty; `children` is
/// the escape hatch for consumers that own their block vocabulary and hand
/// over already-rendered bodies. Each child is wrapped in the same block shell
/// so spacing and background stay contract-consistent, but without a toolbar —
/// there is no block-type metadata to drive the selects. Ignored when the spec
/// carries blocks.
pub fn block_editor_with_children(
    spec: &BlockEditorSpec,
    theme: &dyn ThemeProvider,
    children: Vec<Node>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // ── Resolve chrome from tokens / contract recipe rems ───────────────────
    let fill = theme.resolve_color(spec.fill_token());
    let radius_control = theme.resolve_radius("radius.control");
    let elevated = theme.resolve_color("color.background.elevated");
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let text_tertiary = theme.resolve_color("color.text.tertiary");
    let accent = theme.resolve_color("color.accent.base");

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
    let block_bg = with_alpha(elevated, elevated.3 * 0.42);
    // Code block panel: elevated 40% (mirrors GPUI sibling).
    let code_bg = with_alpha(elevated, elevated.3 * 0.4);

    // ── Posture flags (mode + explicit overrides) ───────────────────────────
    let is_multi = spec.mode == BlockEditorMode::Multi;
    let can_reorder = spec.allow_reorder.unwrap_or(is_multi);
    let can_add = spec.allow_add.unwrap_or(is_multi);
    let can_remove = spec.allow_remove.unwrap_or(is_multi);
    let can_type_change = spec.allow_type_change.unwrap_or(true);
    let disabled = spec.is_disabled;

    // ── Root: flex column, surface bg, stack-gap, no border/radius/pad ──────
    let mut root = Node::container();
    root.style.descriptor.layout.direction = LayoutDirection::Column;
    root.style.descriptor.layout.spacing.gap = stack_gap;
    root.style.descriptor.background = Some(fill);
    let mut root = root;

    let block_count = spec.blocks.len();
    for (i, block) in spec.blocks.iter().enumerate() {
        // ── Toolbar left: drag grip + TypeSelect ───────────────────────────
        let mut toolbar_left = Node::container();
        toolbar_left.style.descriptor.layout.direction = LayoutDirection::Row;
        toolbar_left.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        toolbar_left.style.descriptor.layout.spacing.gap = toolbar_gap;
        let mut toolbar_left = toolbar_left;

        if can_reorder {
            // `.block-editor__drag-grip` — control-size box, tertiary grip icon.
            let mut grip = Node::container();
            {
                let s = &mut grip.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
                s.descriptor.layout.width = LayoutSizing::Fixed(control_size);
                s.descriptor.layout.height = LayoutSizing::Fixed(control_size);
                let c = &mut s.descriptor.corner_radii;
                c.top_left = radius_control;
                c.top_right = radius_control;
                c.bottom_right = radius_control;
                c.bottom_left = radius_control;
            }
            let mut glyph = Node::icon("grip-vertical", icon_size);
            glyph.style.descriptor.text_color = Some(text_tertiary);
            toolbar_left = toolbar_left.child(grip.child(glyph));
        }

        if can_type_change {
            // TypeSelect inset margin-left when reorder is disabled (grip
            // hidden) to keep alignment with block content:
            // calc(content-x + input-x − toolbar-x).
            let mut type_wrap = Node::container();
            // Explicit Row (see switch.rs).
            type_wrap.style.descriptor.layout.direction = LayoutDirection::Row;
            type_wrap.style.flex_shrink_zero = true;
            if !can_reorder {
                let inset = content_x + input_x - toolbar_x;
                if inset > 0.0 {
                    type_wrap.style.descriptor.layout.spacing.margin.left = inset;
                }
            }
            toolbar_left = toolbar_left.child(type_wrap.child(build_type_select(
                spec,
                Some(&block.block_type),
                disabled,
                theme,
            )));
        }

        // ── Toolbar right: move up/down, AddSelect, remove ─────────────────
        let mut toolbar_right = Node::container();
        toolbar_right.style.descriptor.layout.direction = LayoutDirection::Row;
        toolbar_right.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        toolbar_right.style.descriptor.layout.spacing.gap = toolbar_gap;
        let mut toolbar_right = toolbar_right;

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
            // AddSelect: ghost Select with plus-icon trigger. The Select
            // primitive has no trigger-slot override, so the plus tool button
            // sits ahead of the (value-less) picker — closest faithful subset
            // of the contract's trigger-slot pattern (mirrors GPUI).
            let mut add_wrap = Node::container();
            // Explicit Row (see switch.rs).
            add_wrap.style.descriptor.layout.direction = LayoutDirection::Row;
            add_wrap.style.flex_shrink_zero = true;
            toolbar_right = toolbar_right
                .child(tool_btn(
                    "plus",
                    text_tertiary,
                    icon_size,
                    control_size,
                    radius_control,
                    disabled,
                ))
                .child(add_wrap.child(build_type_select(spec, None, disabled, theme)));
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

        let mut toolbar = Node::container();
        {
            let s = &mut toolbar.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.top = toolbar_y;
            pad.bottom = toolbar_y;
            pad.left = toolbar_x;
            pad.right = toolbar_x;
        }
        let mut spacer = Node::container();
        // Explicit Row (see switch.rs).
        spacer.style.descriptor.layout.direction = LayoutDirection::Row;
        spacer.style.descriptor.layout.width = LayoutSizing::Grow;
        let toolbar = toolbar
            .child(toolbar_left)
            .child(spacer)
            .child(toolbar_right);

        // ── Block: flex column, elevated-42% bg, control radius ────────────
        let mut block_el = Node::container();
        {
            let s = &mut block_el.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            let c = &mut s.descriptor.corner_radii;
            c.top_left = radius_control;
            c.top_right = radius_control;
            c.bottom_right = radius_control;
            c.bottom_left = radius_control;
            s.descriptor.background = Some(block_bg);
        }
        root = root.child(block_el.child(toolbar).child(render_block_content(
            block,
            (content_x, content_y),
            text_primary,
            text_secondary,
            accent,
            code_bg,
            radius_control,
        )));
    }

    if spec.blocks.is_empty() {
        for child in children {
            let mut shell = Node::container();
            {
                let s = &mut shell.style;
                s.descriptor.layout.direction = LayoutDirection::Column;
                let c = &mut s.descriptor.corner_radii;
                c.top_left = radius_control;
                c.top_right = radius_control;
                c.bottom_right = radius_control;
                c.bottom_left = radius_control;
                s.descriptor.background = Some(block_bg);
            }
            let mut body = Node::container();
            {
                let s = &mut body.style;
                s.descriptor.layout.direction = LayoutDirection::Column;
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = content_x;
                pad.right = content_x;
                pad.top = content_y;
                pad.bottom = content_y;
            }
            root = root.child(shell.child(body.child(child)));
        }
    }

    if disabled {
        // Contract `.block-editor--disabled` opacity — resolved from token.
        root.style.descriptor.opacity = theme.resolve_opacity("state.opacity.disabled");
    }

    if !spec.aria_label.is_empty() {
        root.a11y.label = Some(spec.aria_label.clone());
    }
    root
}
