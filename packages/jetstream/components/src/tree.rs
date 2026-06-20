//! JsTree — hierarchical disclosure tree backed by `TreeSpec`.
//!
//! Renders the currently-visible rows (a node is visible when every ancestor
//! branch is expanded) as a flat flex column. Depth is expressed with indent
//! cells whose left border draws the ancestor guide lines. Branches show a
//! chevron glyph twisty; leaves reserve a twisty-sized spacer for alignment.

use glam::Vec4;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{CheckState, CheckboxSpec, ControlDensity, ControlSize, SpinnerSpec, TreeNode, TreeSpec};

use crate::checkbox::js_checkbox;
use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::spinner::js_spinner;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius, tint};

/// Tree-specific row height (denser than `SidebarNav`); density never alters it.
fn row_height_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.375,
        ControlSize::Sm => 1.5,
        ControlSize::Md => 1.75,
        ControlSize::Lg => 2.0,
        ControlSize::Xl => 2.25,
    }
}

/// Width of one depth indent cell.
fn indent_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.75,
        ControlDensity::Default => 1.0,
        ControlDensity::Comfortable => 1.25,
    }
}

/// Gap between twisty, icon, and label.
fn row_gap_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.1875,
        ControlDensity::Default => 0.25,
        ControlDensity::Comfortable => 0.375,
    }
}

/// Row leading / trailing inline padding.
fn row_pad_inline_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.25,
        ControlDensity::Default => 0.375,
        ControlDensity::Comfortable => 0.5,
    }
}

/// Resolved metrics + colors, computed once and shared across the recursion.
struct TreeMetrics {
    row_height: f32,
    row_font: f32,
    twisty_size: f32,
    chevron_font: f32,
    icon_font: f32,
    indent: f32,
    row_gap: f32,
    row_pad_inline: f32,
    row_radius: f32,
    show_guides: bool,
    show_icons: bool,
    row_color: Vec4,
    selected_color: Vec4,
    selected_fill: Vec4,
    guide_color: Vec4,
    twisty_color: Vec4,
    icon_color: Vec4,
    focus_ring: Vec4,
    disabled_opacity: f32,
    focused: Option<String>,
}

pub fn js_tree(spec: &TreeSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    let row_font = rem_to_px(size_font_rem(effective_size));
    let ctrl_radius = resolve_radius(theme, "radius.control");

    let m = TreeMetrics {
        row_height: rem_to_px(row_height_rem(effective_size)),
        row_font,
        twisty_size: row_font * 1.5,
        chevron_font: row_font * 0.85,
        icon_font: row_font,
        indent: rem_to_px(indent_rem(spec.density)),
        row_gap: rem_to_px(row_gap_rem(spec.density)),
        row_pad_inline: rem_to_px(row_pad_inline_rem(spec.density)),
        // Row radius is slightly tighter than the panel control radius.
        row_radius: (ctrl_radius - rem_to_px(0.125)).max(0.0),
        show_guides: spec.show_guides,
        show_icons: spec.show_icons,
        row_color: resolve_color(theme, spec.row_color_token()),
        selected_color: resolve_color(theme, spec.row_selected_color_token()),
        selected_fill: resolve_color(theme, spec.selected_fill_token()),
        guide_color: resolve_color(theme, spec.guide_color_token()),
        twisty_color: resolve_color(theme, spec.twisty_color_token()),
        icon_color: resolve_color(theme, spec.icon_color_token()),
        focus_ring: resolve_color(theme, spec.focus_ring_color_token()),
        disabled_opacity: resolve_opacity(theme, spec.disabled_opacity_token()),
        focused: spec.focused_value.clone(),
    };

    let pad_y = resolve_px(theme, "space.panel.y");

    let mut rows: Vec<JsEl> = Vec::new();
    push_rows(&mut rows, spec, &m, theme, &spec.nodes, 0);

    ui_element::div()
        .flex_col()
        .min_w_0()
        .pt(pad_y)
        .pb(pad_y)
        .pl(rem_to_px(0.25))
        .pr(rem_to_px(0.25))
        .children(rows)
}

/// Append the visible rows for `nodes` at `depth`, recursing into expanded branches.
fn push_rows(
    out: &mut Vec<JsEl>,
    spec: &TreeSpec,
    m: &TreeMetrics,
    theme: &JetstreamThemeProvider,
    nodes: &[TreeNode],
    depth: usize,
) {
    for node in nodes {
        out.push(render_row(spec, m, theme, node, depth));
        if spec.is_branch(node) && spec.is_expanded(&node.value) {
            if node.children.is_empty() {
                // Lazy branch: show a loading row while its children load.
                if spec.is_loading(&node.value) {
                    out.push(render_loading_row(m, theme, depth + 1));
                }
            } else {
                push_rows(out, spec, m, theme, &node.children, depth + 1);
            }
        }
    }
}

/// A non-interactive "Loading…" row with a spinner, shown under a lazy branch.
fn render_loading_row(m: &TreeMetrics, theme: &JetstreamThemeProvider, depth: usize) -> JsEl {
    let mut row = ui_element::div()
        .flex_row()
        .items_center()
        .self_stretch()
        .min_w_0()
        .min_h(m.row_height)
        .gap(m.row_gap)
        .pl(m.row_pad_inline)
        .pr(m.row_pad_inline);
    for _ in 0..depth {
        row = row.child(ui_element::div().w(m.indent).self_stretch().flex_none());
    }
    let spinner = ui_element::div()
        .w(m.twisty_size)
        .flex_none()
        .flex_row()
        .items_center()
        .justify_center()
        .child(js_spinner(&SpinnerSpec::new(), theme));
    row.child(spinner).child(
        ui_element::label("Loading…")
            .text_color(m.row_color)
            .text_size(m.row_font),
    )
}

fn render_row(
    spec: &TreeSpec,
    m: &TreeMetrics,
    theme: &JetstreamThemeProvider,
    node: &TreeNode,
    depth: usize,
) -> JsEl {
    let is_branch = spec.is_branch(node);
    let is_expanded = is_branch && spec.is_expanded(&node.value);
    let is_selected = spec.is_selected(&node.value);
    let is_focused = m.focused.as_deref() == Some(node.value.as_str());

    // Uniform 1px border (transparent unless focused) — app-driven focus ring
    // with no layout jitter between focused and unfocused rows.
    let ring_color = if is_focused {
        m.focus_ring
    } else {
        tint(m.focus_ring, 0.0)
    };

    let mut row = ui_element::div()
        .id(format!("tree:{}", node.value))
        .flex_row()
        .items_center()
        .self_stretch()
        .min_w_0()
        .min_h(m.row_height)
        .gap(m.row_gap)
        .pl(m.row_pad_inline)
        .pr(m.row_pad_inline)
        .rounded(m.row_radius)
        .border(1.0)
        .border_color(ring_color);

    // Indent cells (left border draws the ancestor guide line).
    for _ in 0..depth {
        let mut cell = ui_element::div().w(m.indent).self_stretch().flex_none();
        if m.show_guides {
            cell = cell.border_l(1.0).border_color_left(tint(m.guide_color, 0.54));
        }
        row = row.child(cell);
    }

    // Twisty: chevron glyph for branches, empty spacer for leaves.
    let mut twisty = ui_element::div()
        .w(m.twisty_size)
        .flex_none()
        .flex_row()
        .items_center()
        .justify_center();
    if is_branch {
        let glyph = if is_expanded { "▾" } else { "▸" };
        twisty = twisty
            .id(format!("tree-twisty:{}", node.value))
            .child(
                ui_element::label(glyph)
                    .text_color(m.twisty_color)
                    .text_size(m.chevron_font),
            );
    }
    row = row.child(twisty);

    // Optional cascade checkbox (leading, before the icon). Wrapped in an
    // id-bearing cell so clicks route to `TreeCheck` in the shell.
    if spec.show_checkboxes {
        let cs = spec.check_state(node);
        let checkbox = js_checkbox(
            &CheckboxSpec::new()
                .with_checked(matches!(cs, CheckState::Checked))
                .with_mixed(matches!(cs, CheckState::Mixed))
                .with_disabled(node.is_disabled)
                .with_size(ControlSize::Xs),
            theme,
        );
        row = row.child(
            ui_element::div()
                .id(format!("tree-check:{}", node.value))
                .flex_none()
                .flex_row()
                .items_center()
                .child(checkbox),
        );
    }

    // Optional leading icon (reserve the slot even when the node has no icon).
    if m.show_icons {
        let icon_box = match &node.icon {
            Some(name) => ui_element::icon(name)
                .w(m.icon_font)
                .h(m.icon_font)
                .text_color(m.icon_color),
            None => ui_element::div().w(m.icon_font).flex_none(),
        };
        row = row.child(icon_box);
    }

    // Label, or an inline-rename editor box (with caret) when editing this node.
    if spec.is_editing(&node.value) {
        let surface = resolve_color(theme, "color.background.surface");
        row = row.child(
            ui_element::div()
                .flex_1()
                .min_w_0()
                .flex_row()
                .items_center()
                .pl(rem_to_px(0.25))
                .pr(rem_to_px(0.25))
                .border(1.0)
                .border_color(m.focus_ring)
                .rounded(rem_to_px(0.1875))
                .bg(surface)
                .child(
                    ui_element::label(&format!("{}|", spec.editing_text))
                        .text_size(m.row_font)
                        .text_color(m.selected_color),
                ),
        );
    } else {
        let (text_color, weight) = if is_selected {
            (m.selected_color, 600)
        } else {
            (m.row_color, 500)
        };
        row = row.child(
            ui_element::label(&node.label)
                .flex_1()
                .min_w_0()
                .whitespace_nowrap()
                .text_ellipsis()
                .text_size(m.row_font)
                .text_color(text_color)
                .text_weight(weight),
        );
    }

    // Selected fill. The contract's inset ring is a Svelte-only refinement: a
    // layout-affecting border would shift row content relative to unselected
    // rows, so the Rust runtimes convey selection with the accent fill alone.
    if is_selected {
        row = row.bg(tint(m.selected_fill, 0.10));
    }

    if node.is_disabled {
        row = row.opacity(m.disabled_opacity);
    } else {
        row = row.focusable().cursor_pointer();
    }

    row
}
