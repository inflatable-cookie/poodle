//! Tree — hierarchical disclosure tree.
//!
//! Contract: `docs/contracts/components/tree.md`
//! Ported from: `packages/jetstream/components/src/tree.rs`.
//!
//! Renders the currently-visible rows (a node is visible when every ancestor
//! branch is expanded) as a flat flex column. Depth is expressed with indent
//! cells whose left border draws the ancestor guide lines. Branches show a
//! chevron glyph twisty; leaves reserve a twisty-sized spacer for alignment.
//!
//! The three targets in a row are distinct events, each with its own handler
//! so a click on one is never reported as another: the twisty expands, the
//! checkbox checks, and the rest of the row selects.

use std::sync::Arc;

use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, DropEdge, LayoutDirection, LayoutSizing,
    MainAxisAlignment, Node, NodeDropCommit, NodeKey, NodeModifiers, NodePoint, NodePosition,
    NodeRole,
};
use poodle_specs::{
    CheckState, CheckboxSpec, ControlDensity, ControlSize, DropPosition, SpinnerSpec, TreeNode,
    TreeSpec,
};

use crate::checkbox::checkbox;
use crate::color::with_alpha;
use crate::context::RenderContext;
use crate::presentation::{rem_to_px, size_font_rem};
use crate::spinner::spinner;

/// Drag scope for Tree registrations. Tree already names its rows globally
/// (`tree:{value}`), so its source and target ids follow the same identity.
const TREE_DRAG_SCOPE: &str = "tree";

/// Host callback for keyboard commands on a focused tree row.
pub type TreeKeyHandler = Arc<dyn Fn(&str, NodeKey, NodeModifiers) + Send + Sync>;

/// Host callback for drag-over and reorder events between tree rows.
pub type TreeDropHandler = Arc<dyn Fn(&str, &str, DropEdge) + Send + Sync>;

/// Host callbacks: select, expand-toggle and check, each carrying the node's
/// value.
#[derive(Default)]
pub struct TreeHandlers {
    pub on_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_toggle_expand: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_check: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Row activation carrying the modifier state, for multi-select. When set
    /// it replaces `on_select`: Shift extends from the anchor, the platform
    /// accel toggles one row, a bare click replaces the selection.
    pub on_select_modified: Option<Arc<dyn Fn(&str, NodeModifiers) + Send + Sync>>,
    /// Right-click on a row, with the pointer anchor for the menu.
    pub on_context_menu: Option<Arc<dyn Fn(&str, NodePoint) + Send + Sync>>,
    /// A navigation or command key while a row holds focus. The component
    /// reports the key and the row it landed on; resolving it into a focus
    /// move, an expand, or a rename is the host's job, because only the host
    /// knows the flattened visible order.
    pub on_key: Option<TreeKeyHandler>,
    /// A drag is hovering `over`, landing at `edge`. Drives the drop
    /// indicator; `dragged` is the row the gesture started on.
    pub on_drag_over: Option<TreeDropHandler>,
    /// The hovered row stopped being the drop target — the pointer moved to
    /// another row, left every row, or the row became ineligible.
    ///
    /// Without it the host's `drop_target_value` latches on the last row the
    /// pointer touched and the indicator outlives the hover.
    pub on_drag_leave: Option<Arc<dyn Fn() + Send + Sync>>,
    /// The drag ended — committed, rejected, or cancelled. Fires exactly once
    /// per gesture, on every path including Escape and a rebuild that removes
    /// the dragged row, so the host can clear `drag_value` and
    /// `drop_target_value` without inferring the terminal from `on_reorder`.
    pub on_drag_end: Option<Arc<dyn Fn() + Send + Sync>>,
    /// A drag was released: move `dragged` to `edge` of `over`.
    pub on_reorder: Option<TreeDropHandler>,
}

/// Tree-specific row height (denser than SidebarNav); density never alters it.
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
    row_color: ColorValue,
    selected_color: ColorValue,
    selected_fill: ColorValue,
    guide_color: ColorValue,
    twisty_color: ColorValue,
    icon_color: ColorValue,
    focus_ring: ColorValue,
    disabled_opacity: f32,
    focused: Option<String>,
    drag_accent: ColorValue,
    drop_target: Option<String>,
    drop_position: DropPosition,
    reorderable: bool,
}

pub fn tree(spec: &TreeSpec, ctx: &RenderContext<'_>, handlers: TreeHandlers) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);

    let row_font = rem_to_px(size_font_rem(effective_size));
    let ctrl_radius = ctx.theme().resolve_radius("radius.control");

    let m = TreeMetrics {
        row_height: rem_to_px(row_height_rem(effective_size)),
        row_font,
        // Zeroed on a flat tree so the gutter collapses at both render sites
        // — the leaf spacer and the loading row — from one decision.
        twisty_size: if spec.is_flat() { 0.0 } else { row_font * 1.5 },
        chevron_font: row_font * 0.85,
        icon_font: row_font,
        indent: rem_to_px(indent_rem(density)),
        row_gap: rem_to_px(row_gap_rem(density)),
        row_pad_inline: rem_to_px(row_pad_inline_rem(density)),
        // Row radius is slightly tighter than the panel control radius.
        row_radius: (ctrl_radius - rem_to_px(0.125)).max(0.0),
        show_guides: spec.show_guides,
        show_icons: spec.show_icons,
        row_color: ctx.theme().resolve_color(spec.row_color_token()),
        selected_color: ctx.theme().resolve_color(spec.row_selected_color_token()),
        selected_fill: ctx.theme().resolve_color(spec.selected_fill_token()),
        guide_color: ctx.theme().resolve_color(spec.guide_color_token()),
        twisty_color: ctx.theme().resolve_color(spec.twisty_color_token()),
        icon_color: ctx.theme().resolve_color(spec.icon_color_token()),
        focus_ring: ctx.theme().resolve_color(spec.focus_ring_color_token()),
        disabled_opacity: ctx.theme().resolve_opacity(spec.disabled_opacity_token()),
        focused: spec.focused_value.clone(),
        drag_accent: ctx.theme().resolve_color(spec.selected_fill_token()),
        drop_target: spec.drop_target_value.clone(),
        drop_position: spec.drop_position,
        reorderable: spec.reorderable,
    };

    let pad_y = ctx.theme().resolve_space("space.panel.y");

    let mut rows: Vec<Node> = Vec::new();
    push_rows(&mut rows, spec, &m, ctx, &spec.nodes, 0, &handlers);

    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.min_width = Some(0.0);
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.top = pad_y;
        pad.bottom = pad_y;
        pad.left = rem_to_px(0.25);
        pad.right = rem_to_px(0.25);
    }
    for row in rows {
        root = root.child(row);
    }
    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            root.a11y.label = Some(label.to_string());
        }
    }
    root.a11y.role = Some(NodeRole::Tree);
    root
}

/// Append the visible rows for `nodes` at `depth`, recursing into expanded
/// branches.
fn push_rows(
    out: &mut Vec<Node>,
    spec: &TreeSpec,
    m: &TreeMetrics,
    ctx: &RenderContext<'_>,
    nodes: &[TreeNode],
    depth: usize,
    handlers: &TreeHandlers,
) {
    for node in nodes {
        out.push(render_row(spec, m, ctx, node, depth, handlers));
        if spec.is_branch(node) && spec.is_expanded(&node.value) {
            if node.children.is_empty() {
                // Lazy branch: show a loading row while its children load.
                if spec.is_loading(&node.value) {
                    out.push(render_loading_row(m, ctx, depth + 1));
                }
            } else {
                push_rows(out, spec, m, ctx, &node.children, depth + 1, handlers);
            }
        }
    }
}

fn indent_cell(m: &TreeMetrics) -> Node {
    let mut cell = Node::container();
    let s = &mut cell.style;
    // Explicit Row (see switch.rs).
    s.descriptor.layout.direction = LayoutDirection::Row;
    s.descriptor.layout.width = LayoutSizing::Fixed(m.indent);
    s.self_stretch = true;
    s.flex_none = true;
    cell
}

/// A non-interactive "Loading…" row with a spinner, shown under a lazy branch.
fn render_loading_row(m: &TreeMetrics, ctx: &RenderContext<'_>, depth: usize) -> Node {
    let mut row = Node::container();
    {
        let s = &mut row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.self_stretch = true;
        s.min_width = Some(0.0);
        s.min_height = Some(m.row_height);
        s.descriptor.layout.spacing.gap = m.row_gap;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = m.row_pad_inline;
        pad.right = m.row_pad_inline;
    }
    let mut row = row;
    for _ in 0..depth {
        row = row.child(indent_cell(m));
    }
    let mut spinner_box = Node::container();
    {
        let s = &mut spinner_box.style;
        s.descriptor.layout.width = LayoutSizing::Fixed(m.twisty_size);
        s.flex_none = true;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
    }
    let spinner_box = spinner_box.child(spinner(&SpinnerSpec::new(), ctx));
    let mut label = Node::text("Loading…");
    label.style.descriptor.text_color = Some(m.row_color);
    label.style.text_size = Some(m.row_font);
    row.child(spinner_box).child(label)
}

fn render_row(
    spec: &TreeSpec,
    m: &TreeMetrics,
    ctx: &RenderContext<'_>,
    node: &TreeNode,
    depth: usize,
    handlers: &TreeHandlers,
) -> Node {
    let is_branch = spec.is_branch(node);
    let is_expanded = is_branch && spec.is_expanded(&node.value);
    let is_selected = spec.is_selected(&node.value);
    let is_focused = m.focused.as_deref() == Some(node.value.as_str());

    let mut row = Node::container();
    // Contract: rows are treeitems carrying their depth so a screen reader
    // can announce "level 3".
    row.a11y.role = Some(NodeRole::TreeItem);
    row.a11y.level = Some(depth + 1);
    row.id = Some(format!("tree:{}", node.value));
    {
        let s = &mut row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.self_stretch = true;
        s.min_width = Some(0.0);
        s.min_height = Some(m.row_height);
        s.descriptor.layout.spacing.gap = m.row_gap;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = m.row_pad_inline;
        pad.right = m.row_pad_inline;
        let c = &mut s.descriptor.corner_radii;
        c.top_left = m.row_radius;
        c.top_right = m.row_radius;
        c.bottom_right = m.row_radius;
        c.bottom_left = m.row_radius;
    }

    // Modifier-aware selection wins over the plain one — a node wires either
    // `on_activate` or `on_activate_modified`, never both.
    if let (false, Some(handler)) = (node.is_disabled, &handlers.on_select_modified) {
        let handler = Arc::clone(handler);
        let value = node.value.clone();
        row.style.descriptor.cursor = CursorHint::Pointer;
        row.interaction.on_activate_modified = Some(Arc::new(move |mods| handler(&value, mods)));
    } else if let (false, Some(handler)) = (node.is_disabled, &handlers.on_select) {
        let handler = Arc::clone(handler);
        let value = node.value.clone();
        row.style.descriptor.cursor = CursorHint::Pointer;
        row.interaction.on_activate = Some(Arc::new(move || handler(&value)));
    }

    if let (false, Some(handler)) = (node.is_disabled, &handlers.on_context_menu) {
        let handler = Arc::clone(handler);
        let value = node.value.clone();
        row.interaction.on_context = Some(Arc::new(move |point| handler(&value, point)));
    }

    if let (false, Some(handler)) = (node.is_disabled, &handlers.on_key) {
        let handler = Arc::clone(handler);
        let value = node.value.clone();
        row.interaction.on_key = Some(Arc::new(move |key, mods| {
            handler(&value, key, mods);
            None
        }));
    }

    // Reorder: every row is both a source and a target, so a row can be
    // dropped onto any other. The band rule lives in the shared builder and
    // the controller hands it a fraction of this row's own bounds — the
    // component still never sees a coordinate.
    if !node.is_disabled && m.reorderable {
        let value = node.value.clone();
        let mut source = crate::drag_drop::reorder_source(TREE_DRAG_SCOPE, &value, &node.label);
        if let Some(handler) = &handlers.on_drag_end {
            let handler = Arc::clone(handler);
            source.on_drag_end = Some(Arc::new(move |_outcome| handler()));
        }
        crate::drag_drop::attach_source(&mut row, true, source);

        // Every tree row can take an `inside` drop: nesting is what the
        // component is for, so the band splits in thirds rather than halves.
        let mut target =
            crate::drag_drop::nested_target(TREE_DRAG_SCOPE, &value, &node.label, true);
        if let Some(handler) = &handlers.on_drag_over {
            let handler = Arc::clone(handler);
            let over = value.clone();
            target.on_intent = Some(Arc::new(move |event| {
                if let Some(edge) = crate::drag_drop::edge_from_position(&event.position) {
                    handler(&event.subject.id, &over, edge);
                }
            }));
        }
        if let Some(handler) = &handlers.on_drag_leave {
            let handler = Arc::clone(handler);
            target.on_intent_cleared = Some(Arc::new(move || handler()));
        }
        if let Some(handler) = &handlers.on_reorder {
            let handler = Arc::clone(handler);
            let over = value.clone();
            target.on_drop = Some(Arc::new(move |event| {
                match crate::drag_drop::edge_from_position(&event.intent.position) {
                    Some(edge) => {
                        handler(&event.subject.id, &over, edge);
                        NodeDropCommit::Committed
                    }
                    // A consumer-defined placement this contract has no edge
                    // for is refused rather than silently rounded to Inside.
                    None => NodeDropCommit::Rejected {
                        reason: Some(format!(
                            "Tree does not place a row at `{}`",
                            event.intent.position
                        )),
                    },
                }
            }));
        }
        crate::drag_drop::attach_target(&mut row, true, target);
    }

    // Indent cells (left border draws the ancestor guide line).
    let mut row = row;
    for _ in 0..depth {
        let mut cell = indent_cell(m);
        if m.show_guides {
            cell.style.border_left_width = Some(1.0);
            cell.style.border_color_left = Some(with_alpha(m.guide_color, m.guide_color.3 * 0.54));
        }
        row = row.child(cell);
    }

    // Twisty: chevron glyph for branches, empty spacer for leaves.
    let mut twisty = Node::container();
    {
        let s = &mut twisty.style;
        s.descriptor.layout.width = LayoutSizing::Fixed(m.twisty_size);
        s.flex_none = true;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
    }
    if is_branch {
        let glyph = if is_expanded { "▾" } else { "▸" };
        twisty.id = Some(format!("tree-twisty:{}", node.value));
        let mut g = Node::text(glyph);
        g.style.descriptor.text_color = Some(m.twisty_color);
        g.style.text_size = Some(m.chevron_font);
        twisty = twisty.child(g);

        // Its own handler, always — clicks bubble to the nearest clickable
        // ancestor, so an unwired twisty would select the row it expands.
        if let (false, Some(handler)) = (node.is_disabled, &handlers.on_toggle_expand) {
            let handler = Arc::clone(handler);
            let value = node.value.clone();
            twisty.style.descriptor.cursor = CursorHint::Pointer;
            twisty.interaction.on_activate = Some(Arc::new(move || handler(&value)));
        } else {
            twisty.interaction.on_activate = Some(Arc::new(|| {}));
        }
    }
    row = row.child(twisty);

    // Optional cascade checkbox (leading, before the icon), wrapped in an
    // id-bearing cell so clicks route in the shell.
    if spec.show_checkboxes {
        let cs = spec.check_state(node);
        let cb = checkbox(
            &CheckboxSpec::new()
                // A selection checkbox has no caption of its own; name it
                // after what it selects.
                .with_aria_label(format!("Select {}", node.label))
                .with_checked(matches!(cs, CheckState::Checked))
                .with_mixed(matches!(cs, CheckState::Mixed))
                .with_disabled(node.is_disabled)
                .with_size(ControlSize::Xs),
            ctx,
            None,
        );
        let mut cell = Node::container();
        cell.id = Some(format!("tree-check:{}", node.value));
        {
            let s = &mut cell.style;
            s.flex_none = true;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        }
        let mut cell = cell.child(cb);

        if let (false, Some(handler)) = (node.is_disabled, &handlers.on_check) {
            let handler = Arc::clone(handler);
            let value = node.value.clone();
            cell.style.descriptor.cursor = CursorHint::Pointer;
            cell.interaction.on_activate = Some(Arc::new(move || handler(&value)));
        } else {
            cell.interaction.on_activate = Some(Arc::new(|| {}));
        }

        row = row.child(cell);
    }

    // Optional leading icon (reserve the slot even when the node has none).
    if m.show_icons {
        let icon_box = match &node.icon {
            Some(name) => {
                let mut i = Node::icon(name, m.icon_font);
                i.style.descriptor.text_color = Some(m.icon_color);
                i
            }
            None => {
                let mut spacer = Node::container();
                let s = &mut spacer.style;
                // Explicit Row (see switch.rs).
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.width = LayoutSizing::Fixed(m.icon_font);
                s.flex_none = true;
                spacer
            }
        };
        row = row.child(icon_box);
    }

    // Label, or an inline-rename editor box (with caret) when editing.
    if spec.is_editing(&node.value) {
        let surface = ctx.theme().resolve_color("color.background.surface");
        let mut editor = Node::container();
        {
            let s = &mut editor.style;
            s.flex_grow = Some(1.0);
            s.flex_basis = Some(0.0);
            s.min_width = Some(0.0);
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = rem_to_px(0.25);
            pad.right = rem_to_px(0.25);
            s.descriptor.border.width = 1.0;
            // Contract `.tree__rename` border = accent-base (not focus ring).
            s.descriptor.border.color = m.selected_fill;
            let r = rem_to_px(0.1875);
            let c = &mut s.descriptor.corner_radii;
            c.top_left = r;
            c.top_right = r;
            c.bottom_right = r;
            c.bottom_left = r;
            s.descriptor.background = Some(surface);
        }
        let mut text = Node::text(format!("{}|", spec.editing_text));
        text.style.text_size = Some(m.row_font);
        text.style.descriptor.text_color = Some(m.selected_color);
        row = row.child(editor.child(text));
    } else {
        let (text_color, weight) = if is_selected {
            (m.selected_color, 600)
        } else {
            (m.row_color, 500)
        };
        let mut label = Node::text(&node.label);
        {
            let s = &mut label.style;
            s.flex_grow = Some(1.0);
            s.flex_basis = Some(0.0);
            s.min_width = Some(0.0);
            s.no_wrap = true;
            s.text_ellipsis = true;
            s.text_size = Some(m.row_font);
            s.descriptor.text_color = Some(text_color);
            s.text_weight = Some(weight);
        }
        row = row.child(label);
        if let Some(end_label) = &node.end_label {
            let mut end = Node::text(end_label);
            end.style.flex_none = true;
            end.style.text_size = Some(m.row_font);
            end.style.descriptor.text_color = Some(text_color);
            end.style.text_weight = Some(500);
            row = row.child(end);
        }
    }

    // Selected fill (the contract's inset ring is a Svelte-only refinement).
    if is_selected {
        row.style.descriptor.background =
            Some(with_alpha(m.selected_fill, m.selected_fill.3 * 0.10));
    }

    // Focus ring. The contract draws it as an `outline` (tree.css
    // `.tree__item:focus-visible > .tree__row`), which does not participate in
    // layout — so it is an absolutely-inset overlay here, not a border. A
    // border on the row insets its content box by 1px, which shortened every
    // indent cell by 2px and broke the ancestor guide lines into stubs.
    //
    // Contract §"Roving tabindex": the Rust runtimes track focus via
    // `focused_value` on the spec (the host owns it) and render the ring on
    // that node, rather than through a host focus pseudo-state.
    if is_focused {
        let mut ring = Node::container();
        {
            let s = &mut ring.style;
            s.descriptor.border.width = 1.0;
            s.descriptor.border.color = m.focus_ring;
            let c = &mut s.descriptor.corner_radii;
            c.top_left = m.row_radius;
            c.top_right = m.row_radius;
            c.bottom_right = m.row_radius;
            c.bottom_left = m.row_radius;
        }
        ring.position = NodePosition::Absolute {
            top: Some(0.0),
            left: Some(0.0),
            right: Some(0.0),
            bottom: Some(0.0),
        };
        row.position = NodePosition::Relative;
        row = row.child(ring);
    }

    // Drop indicator (contract §8): an accent line at the row top/bottom for
    // before/after, or an inset accent-12% fill for inside.
    if m.drop_target.as_deref() == Some(node.value.as_str()) {
        match m.drop_position {
            DropPosition::Inside => {
                row.style.descriptor.background =
                    Some(with_alpha(m.drag_accent, m.drag_accent.3 * 0.12));
            }
            DropPosition::Before | DropPosition::After => {
                // 0.125rem accent line spanning the row, pinned top or bottom.
                let mut line = Node::container();
                line.position = NodePosition::Absolute {
                    top: matches!(m.drop_position, DropPosition::Before)
                        .then(|| -rem_to_px(0.0625)),
                    left: Some(0.0),
                    right: Some(0.0),
                    bottom: matches!(m.drop_position, DropPosition::After)
                        .then(|| -rem_to_px(0.0625)),
                };
                {
                    let s = &mut line.style;
                    // Explicit Row (see switch.rs).
                    s.descriptor.layout.direction = LayoutDirection::Row;
                    s.descriptor.layout.height = LayoutSizing::Fixed(rem_to_px(0.125));
                    s.descriptor.background = Some(m.drag_accent);
                }
                row.position = NodePosition::Relative;
                row = row.child(line);
            }
        }
    }

    if node.is_disabled {
        row.style.descriptor.opacity = m.disabled_opacity;
    } else {
        if node.is_muted && !is_selected && !is_focused {
            row.style.descriptor.opacity = 0.55;
        }
        row.interaction.focusable = true;
        row.style.descriptor.cursor = CursorHint::Pointer;
    }

    row
}
