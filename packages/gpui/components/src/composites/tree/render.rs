//! Tree — row / loading-row rendering.
//!
//! Split out of `tree/mod.rs` (god-file decomposition); the navigation and
//! cascade logic lives in `poodle-headless`/`poodle-specs`. Unchanged.

use std::rc::Rc;

use std::collections::HashMap;

use gpui::*;
use poodle_specs::{
    CheckState, CheckboxSpec, ControlSize, DropPosition, SpinnerSpec, TreeNode,
};

use crate::primitives::{Checkbox, EditableLabel, Icon, Spinner};
use crate::theme_ext::focus_ring_shadow;



/// Whether the node with `value` is disabled (searched across the whole tree).

use super::*;

impl Tree {
    /// Build the root key handler from the current visible rows + callbacks.
    pub(super) fn key_handler(
        &self,
        sel: Rc<SelectionCtx>,
    ) -> impl Fn(&KeyDownEvent, &mut Window, &mut App) + 'static {
        let nav = self.spec.visible_rows();
        let focused = self.spec.focused_value.clone();
        let on_focus = self.on_focus_change.clone();
        let on_toggle = self.on_toggle_expand.clone();
        let on_activate = self.on_activate.clone();
        let on_select = self.on_select.clone();
        let on_rename = self.on_rename_start.clone();
        let on_reorder = self.on_reorder.clone();
        let reorderable = self.spec.reorderable;

        // value -> (prev sibling, next sibling), for Alt+Up/Down reorder.
        pub(super) fn build_siblings(
            nodes: &[TreeNode],
            map: &mut HashMap<String, (Option<String>, Option<String>)>,
        ) {
            for (i, n) in nodes.iter().enumerate() {
                let prev = if i > 0 { Some(nodes[i - 1].value.clone()) } else { None };
                let next = nodes.get(i + 1).map(|n| n.value.clone());
                map.insert(n.value.clone(), (prev, next));
                build_siblings(&n.children, map);
            }
        }
        let mut sib_map: HashMap<String, (Option<String>, Option<String>)> = HashMap::new();
        build_siblings(&self.spec.nodes, &mut sib_map);

        move |event: &KeyDownEvent, window, cx| {
            let key = event.keystroke.key.as_str();
            let idx = focused
                .as_deref()
                .and_then(|f| nav.iter().position(|r| r.value == f));
            let focus = |value: &str, window: &mut Window, cx: &mut App| {
                if let Some(h) = &on_focus {
                    h(value, window, cx);
                }
            };
            match key {
                "down" => {
                    if event.keystroke.modifiers.alt && reorderable {
                        if let (Some(f), Some(h)) = (focused.as_deref(), &on_reorder) {
                            if let Some((_, Some(next))) = sib_map.get(f) {
                                h(
                                    &TreeReorderRequest {
                                        from: f.to_string(),
                                        to: next.clone(),
                                        position: DropPosition::After,
                                    },
                                    window,
                                    cx,
                                );
                            }
                        }
                    } else {
                        let target = match idx {
                            Some(i) => nav.get(i + 1),
                            None => nav.first(),
                        };
                        if let Some(r) = target {
                            if event.keystroke.modifiers.shift && sel.handler.is_some() {
                                sel.emit(sel.extend(&r.value), window, cx);
                            } else {
                                focus(&r.value, window, cx);
                            }
                        }
                    }
                }
                "up" => {
                    if event.keystroke.modifiers.alt && reorderable {
                        if let (Some(f), Some(h)) = (focused.as_deref(), &on_reorder) {
                            if let Some((Some(prev), _)) = sib_map.get(f) {
                                h(
                                    &TreeReorderRequest {
                                        from: f.to_string(),
                                        to: prev.clone(),
                                        position: DropPosition::Before,
                                    },
                                    window,
                                    cx,
                                );
                            }
                        }
                    } else {
                        let target = match idx {
                            Some(i) if i > 0 => nav.get(i - 1),
                            Some(_) => None,
                            None => nav.last(),
                        };
                        if let Some(r) = target {
                            if event.keystroke.modifiers.shift && sel.handler.is_some() {
                                sel.emit(sel.extend(&r.value), window, cx);
                            } else {
                                focus(&r.value, window, cx);
                            }
                        }
                    }
                }
                "right" => {
                    if let Some(i) = idx {
                        let r = &nav[i];
                        if r.is_branch && !r.is_expanded {
                            if let Some(h) = &on_toggle {
                                h(&r.value, window, cx);
                            }
                        } else if r.is_branch {
                            if let Some(n) = nav.get(i + 1) {
                                focus(&n.value, window, cx);
                            }
                        }
                    }
                }
                "left" => {
                    if let Some(i) = idx {
                        let r = &nav[i];
                        if r.is_branch && r.is_expanded {
                            if let Some(h) = &on_toggle {
                                h(&r.value, window, cx);
                            }
                        } else if let Some(p) = &r.parent {
                            focus(p, window, cx);
                        }
                    }
                }
                "home" => {
                    if let Some(r) = nav.first() {
                        focus(&r.value, window, cx);
                    }
                }
                "end" => {
                    if let Some(r) = nav.last() {
                        focus(&r.value, window, cx);
                    }
                }
                "enter" => {
                    if let Some(i) = idx {
                        let value = &nav[i].value;
                        if sel.handler.is_some() {
                            sel.emit(sel.replace(value), window, cx);
                        } else if let Some(h) = &on_select {
                            h(value, window, cx);
                        }
                        if let Some(h) = &on_activate {
                            h(value, window, cx);
                        }
                    }
                }
                "space" | " " => {
                    if let Some(i) = idx {
                        let value = &nav[i].value;
                        if sel.handler.is_some() {
                            sel.emit(sel.toggle(value), window, cx);
                        } else if let Some(h) = &on_select {
                            h(value, window, cx);
                        }
                    }
                }
                "f2" => {
                    if let Some(i) = idx {
                        if let Some(h) = &on_rename {
                            h(&nav[i].value, window, cx);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    pub(super) fn push_rows(
        &self,
        nodes: &[TreeNode],
        depth: usize,
        m: &TreeMetrics,
        out: &mut Vec<AnyElement>,
    ) {
        for node in nodes {
            out.push(self.render_row(node, depth, m));
            if self.spec.is_branch(node) && self.spec.is_expanded(&node.value) {
                if node.children.is_empty() {
                    if self.spec.is_loading(&node.value) {
                        out.push(self.render_loading_row(depth + 1, m));
                    }
                } else {
                    self.push_rows(&node.children, depth + 1, m, out);
                }
            }
        }
    }

    /// A non-interactive "Loading…" row with a spinner, shown under a lazy branch.
    pub(super) fn render_loading_row(&self, depth: usize, m: &TreeMetrics) -> AnyElement {
        let mut row = div()
            .w_full()
            .min_h(m.row_height)
            .px(m.row_pad_inline)
            .flex()
            .items_center()
            .gap(m.row_gap)
            .text_size(m.row_font)
            .text_color(m.row_color);
        for _ in 0..depth {
            row = row.child(div().w(m.indent).h(m.row_height).flex_none());
        }
        row = row.child(
            div()
                .w(m.twisty_size)
                .h(m.row_height)
                .flex_none()
                .flex()
                .items_center()
                .justify_center()
                .child(Spinner::from_spec(SpinnerSpec::new(), &self.theme)),
        );
        row.child(div().child("Loading…")).into_any_element()
    }

    pub(super) fn render_row(&self, node: &TreeNode, depth: usize, m: &TreeMetrics) -> AnyElement {
        let spec = &self.spec;
        let is_branch = spec.is_branch(node);
        let is_expanded = is_branch && spec.is_expanded(&node.value);
        let is_selected = spec.is_selected(&node.value);
        let is_focused = m.focused.as_deref() == Some(node.value.as_str());

        let row_id = SharedString::from(format!("tree-row-{}", node.value));

        let (text_color, weight) = if is_selected {
            (m.selected_color, FontWeight::SEMIBOLD)
        } else {
            (m.row_color, FontWeight::MEDIUM)
        };

        let mut row = div()
            .id(row_id)
            .w_full()
            .min_w(px(0.0))
            .min_h(m.row_height)
            .px(m.row_pad_inline)
            .rounded(m.row_radius)
            .flex()
            .items_center()
            .gap(m.row_gap)
            .text_size(m.row_font)
            .text_color(text_color)
            .font_weight(weight)
            .line_height(relative(1.3));

        // App-driven focus ring on the focused node (keyboard focus is tracked in
        // app state via focused_value, not GPUI's per-element focus).
        if is_focused {
            row = row.shadow(focus_ring_shadow(m.focus_ring));
        }

        // Indent cells (left border draws the ancestor guide line).
        for _ in 0..depth {
            let mut cell = div().w(m.indent).h(m.row_height).flex_none();
            if m.show_guides {
                cell = cell.border_l_1().border_color(m.guide_color);
            }
            row = row.child(cell);
        }

        // Twisty: chevron glyph for branches, empty spacer for leaves. Clicking it
        // toggles expansion without changing selection.
        let mut twisty = div()
            .id(SharedString::from(format!("tree-twisty-{}", node.value)))
            .w(m.twisty_size)
            .h(m.row_height)
            .flex_none()
            .flex()
            .items_center()
            .justify_center()
            .text_size(m.chevron_font)
            .text_color(m.twisty_color);
        if is_branch {
            twisty = twisty.child(if is_expanded { "▾" } else { "▸" });
            if !node.is_disabled {
                if let Some(ref handler) = self.on_toggle_expand {
                    let handler = Rc::clone(handler);
                    let val = node.value.clone();
                    twisty = twisty.cursor_pointer().on_click(move |_event, window, cx| {
                        handler(&val, window, cx);
                    });
                }
            }
        }
        row = row.child(twisty);

        // Optional cascade checkbox (leading, before the icon).
        if spec.show_checkboxes {
            let cs = spec.check_state(node);
            let mut checkbox = Checkbox::from_spec(
                CheckboxSpec::new()
                    .with_checked(matches!(cs, CheckState::Checked))
                    .with_mixed(matches!(cs, CheckState::Mixed))
                    .with_disabled(node.is_disabled)
                    .with_size(ControlSize::Xs),
                &self.theme,
            )
            .with_id(format!("tree-check-{}", node.value));
            if !node.is_disabled {
                if let Some(ref handler) = self.on_check {
                    let handler = Rc::clone(handler);
                    let val = node.value.clone();
                    checkbox = checkbox.on_change(move |_checked: &bool, window, cx| {
                        handler(&val, window, cx);
                    });
                }
            }
            row = row.child(div().flex_none().flex().items_center().child(checkbox));
        }

        // Optional leading icon (reserve the slot even when the node has none).
        if m.show_icons {
            match &node.icon {
                Some(name) => {
                    row = row.child(
                        Icon::new(name.clone(), &self.theme)
                            .with_px_size(m.icon_font)
                            .with_color(m.icon_color),
                    );
                }
                None => {
                    row = row.child(div().w(px(m.icon_font)).flex_none());
                }
            }
        }

        // Label, or an inline-rename editor when this node is being edited.
        if spec.is_editing(&node.value) {
            let mut editor = EditableLabel::new(&self.theme)
                .value(spec.editing_text.clone())
                .editing(true)
                .with_id(format!("tree-edit-{}", node.value))
                .size(ControlSize::Xs);
            if let Some(ref change) = self.on_rename_change {
                let change = Rc::clone(change);
                editor = editor.on_change(move |text, window, cx| change(text, window, cx));
            }
            if let Some(ref commit) = self.on_rename_commit {
                let commit = Rc::clone(commit);
                editor = editor.on_commit(move |text, window, cx| commit(text, window, cx));
            }
            if let Some(ref cancel) = self.on_rename_cancel {
                let cancel = Rc::clone(cancel);
                let val = node.value.clone();
                editor = editor.on_cancel(move |window, cx| cancel(&val, window, cx));
            }
            row = row.child(div().flex_1().min_w(px(0.0)).child(editor));
        } else {
            row = row.child(
                div()
                    .flex_1()
                    .min_w(px(0.0))
                    .overflow_hidden()
                    .text_ellipsis()
                    .child(SharedString::from(node.label.clone())),
            );
        }

        // Selected fill (alpha-only). The contract's inset ring is a Svelte-only
        // refinement; a layout-affecting border would shift unselected rows.
        if is_selected {
            row = row.bg(m.selected_fill);
        }

        if node.is_disabled {
            row = row
                .opacity(m.disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        } else {
            let hover_bg = m.hover_bg;
            let hover_text = m.selected_color;
            row = row.cursor_pointer().hover(move |s| s.text_color(hover_text).bg(hover_bg));

            // Click selects — multi-select aware (Ctrl/Cmd toggle, Shift range).
            let on_focus = self.on_focus_change.clone();
            let on_select = self.on_select.clone();
            let sel = m.sel.clone();
            if sel.handler.is_some() || on_focus.is_some() || on_select.is_some() {
                let val = node.value.clone();
                row = row.on_click(move |event, window, cx| {
                    if sel.handler.is_some() {
                        let mods = event.modifiers();
                        let update = if mods.shift {
                            sel.extend(&val)
                        } else if mods.control || mods.platform {
                            sel.toggle(&val)
                        } else {
                            sel.replace(&val)
                        };
                        sel.emit(update, window, cx);
                    } else {
                        if let Some(h) = &on_focus {
                            h(&val, window, cx);
                        }
                        if let Some(h) = &on_select {
                            h(&val, window, cx);
                        }
                    }
                });
            }

            // Right-click requests a context menu at the pointer.
            if let Some(ref ctx) = self.on_context_menu {
                let ctx = Rc::clone(ctx);
                let val = node.value.clone();
                row = row.on_mouse_down(MouseButton::Right, move |event, window, cx| {
                    ctx(
                        &TreeContextRequest {
                            value: val.clone(),
                            position: event.position,
                        },
                        window,
                        cx,
                    );
                });
            }

            // Drag-and-drop reorder.
            if spec.reorderable {
                let preview_bg = Hsla { a: 1.0, ..m.hover_bg };
                let preview_fg = m.selected_color;
                row = row.on_drag(
                    NodeDragPayload {
                        value: node.value.clone(),
                        label: node.label.clone(),
                    },
                    move |payload, _offset, _window, cx| {
                        cx.new(|_| TreeDragPreview {
                            label: payload.label.clone(),
                            bg: preview_bg,
                            fg: preview_fg,
                        })
                    },
                );

                // While hovering, compute before/after/inside from the pointer Y
                // within the row's bounds and report it for the indicator.
                if let Some(ref over) = self.on_drag_over {
                    let over = Rc::clone(over);
                    let to = node.value.clone();
                    let to_branch = is_branch;
                    row = row.on_drag_move::<NodeDragPayload>(move |ev, window, cx| {
                        let height = f32::from(ev.bounds.size.height).max(1.0);
                        let rel = f32::from(ev.event.position.y - ev.bounds.origin.y) / height;
                        let position = if to_branch {
                            if rel < 0.25 {
                                DropPosition::Before
                            } else if rel > 0.75 {
                                DropPosition::After
                            } else {
                                DropPosition::Inside
                            }
                        } else if rel < 0.5 {
                            DropPosition::Before
                        } else {
                            DropPosition::After
                        };
                        over(
                            &TreeDragOver {
                                value: to.clone(),
                                position,
                            },
                            window,
                            cx,
                        );
                    });
                }

                // Drop applies the move using the last computed drop position.
                if let Some(ref reorder) = self.on_reorder {
                    let reorder = Rc::clone(reorder);
                    let to = node.value.clone();
                    let pos = m.drop_position;
                    row = row.on_drop::<NodeDragPayload>(move |payload, window, cx| {
                        reorder(
                            &TreeReorderRequest {
                                from: payload.value.clone(),
                                to: to.clone(),
                                position: pos,
                            },
                            window,
                            cx,
                        );
                    });
                }
            }

            // Drop indicator: before/after line or inside highlight.
            if m.drop_target.as_deref() == Some(node.value.as_str()) {
                match m.drop_position {
                    DropPosition::Inside => {
                        row = row.bg(Hsla { a: m.drag_accent.a * 0.15, ..m.drag_accent });
                    }
                    DropPosition::Before | DropPosition::After => {
                        let mut line = div()
                            .absolute()
                            .left(px(0.0))
                            .right(px(0.0))
                            .h(px(2.0))
                            .bg(m.drag_accent);
                        line = if matches!(m.drop_position, DropPosition::Before) {
                            line.top(px(-1.0))
                        } else {
                            line.bottom(px(-1.0))
                        };
                        row = row.relative().child(line);
                    }
                }
            }
        }

        row.into_any_element()
    }
}
