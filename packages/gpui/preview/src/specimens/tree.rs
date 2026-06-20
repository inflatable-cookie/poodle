use crate::app_state::AppState;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{
    ContextMenu, Eyebrow, Tree, TreeContextRequest, TreeDragOver, TreeReorderRequest,
    TreeSelectionUpdate,
};
use poodle_specs::{
    ContextMenuSpec, ControlDensity, ControlSize, EyebrowSpec, MenuEntry, TreeNode, TreeSpec,
};

use crate::style_bridge::color_to_hsla;

fn file_tree() -> Vec<TreeNode> {
    vec![
        TreeNode::branch(
            "src",
            "src",
            vec![
                TreeNode::branch(
                    "src/components",
                    "components",
                    vec![
                        TreeNode::new("src/components/Button.svelte", "Button.svelte")
                            .with_icon("file"),
                        TreeNode::new("src/components/Tree.svelte", "Tree.svelte").with_icon("file"),
                    ],
                )
                .with_icon("folder"),
                // Empty-but-branch folder (lazy / no children yet).
                TreeNode::new("src/lib", "lib").with_icon("folder").with_branch(true),
                TreeNode::new("src/index.ts", "index.ts").with_icon("file"),
            ],
        )
        .with_icon("folder"),
        TreeNode::new("package.json", "package.json").with_icon("file"),
        TreeNode::new("README.md", "README.md").with_icon("file"),
        TreeNode::new("node_modules", "node_modules")
            .with_icon("folder")
            .with_branch(true)
            .with_disabled(true),
    ]
}

fn expanded() -> Vec<String> {
    vec!["src".into(), "src/components".into()]
}

fn find_node<'a>(nodes: &'a [TreeNode], value: &str) -> Option<&'a TreeNode> {
    for node in nodes {
        if node.value == value {
            return Some(node);
        }
        if let Some(found) = find_node(&node.children, value) {
            return Some(found);
        }
    }
    None
}

fn framed(theme: &GpuiThemeProvider, content: impl IntoElement) -> Div {
    div()
        .w(px(288.0))
        .min_h(px(220.0))
        .border_1()
        .border_color(color_to_hsla(theme.resolve_color("color.border.subtle")))
        .rounded(px(6.0))
        .overflow_hidden()
        .child(content)
}

fn labelled(theme: &GpuiThemeProvider, label: &str, content: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(EyebrowSpec::new().with_content(label), theme))
        .child(content)
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    // Interactive file explorer: keyboard nav + click drive AppState.tree.
    let mut interactive = Tree::from_spec(
        TreeSpec::new(file_tree())
            .with_expanded_values(state.tree.expanded.clone())
            .with_selected_values(state.tree.selected.clone())
            .with_aria_label("Project files"),
        theme,
    );
    if let Some(focused) = &state.tree.focused {
        interactive = interactive.focused_value(focused.clone());
    }
    interactive = interactive
        .selection_anchor(state.tree.selection_anchor.clone())
        .on_focus_change(cx.listener(|this, v: &str, _w, cx| {
            this.state.tree.set_focused(v);
            cx.notify();
        }))
        .on_toggle_expand(cx.listener(|this, v: &str, _w, cx| {
            this.state.tree.toggle_expanded(v);
            cx.notify();
        }))
        .on_selection_change(cx.listener(|this, u: &TreeSelectionUpdate, _w, cx| {
            this.state
                .tree
                .apply_selection(u.values.clone(), u.anchor.clone(), &u.focused);
            cx.notify();
        }));

    // Sizes row (static).
    let mut sizes = div().flex().gap(px(16.0));
    for size in [
        ControlSize::Xs,
        ControlSize::Sm,
        ControlSize::Md,
        ControlSize::Lg,
        ControlSize::Xl,
    ] {
        sizes = sizes.child(
            Tree::from_spec(
                TreeSpec::new(file_tree())
                    .with_expanded_values(expanded())
                    .with_selected_values(vec!["src/components/Tree.svelte".into()])
                    .with_size(size),
                theme,
            ),
        );
    }

    // Densities row (static).
    let mut densities = div().flex().gap(px(16.0));
    for density in [
        ControlDensity::Compact,
        ControlDensity::Default,
        ControlDensity::Comfortable,
    ] {
        densities = densities.child(
            Tree::from_spec(
                TreeSpec::new(file_tree())
                    .with_expanded_values(expanded())
                    .with_selected_values(vec!["src/components/Tree.svelte".into()])
                    .with_density(density),
                theme,
            ),
        );
    }

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(labelled(
            theme,
            "File explorer — click · Ctrl/Cmd+click · Shift+click · ↑/↓ ←/→ Space",
            framed(theme, interactive),
        ))
        .child(labelled(
            theme,
            "Checkbox cascade — click a checkbox",
            framed(
                theme,
                Tree::from_spec(
                    TreeSpec::new(file_tree())
                        .with_expanded_values(expanded())
                        .with_show_checkboxes(true)
                        .with_checked_values(state.tree.checked.clone()),
                    theme,
                )
                .on_check(cx.listener(|this, v: &str, _w, cx| {
                    let nodes = file_tree();
                    let spec = TreeSpec::new(nodes.clone());
                    if let Some(node) = find_node(&nodes, v) {
                        let leaves = spec.checkable_values_under(node);
                        this.state.tree.toggle_checked(&leaves);
                    }
                    cx.notify();
                })),
            ),
        ))
        .child(labelled(theme, "Rename (F2) · right-click menu · Alt+↑/↓ reorder", {
            let mut rt_spec = TreeSpec::new(state.tree.rename_nodes.clone())
                .with_expanded_values(vec!["docs".into()])
                .with_reorderable(true)
                .with_drag(None, state.tree.drop_target.clone(), state.tree.drop_position)
                .with_aria_label("Rename tree");
            if let Some(ev) = &state.tree.editing_value {
                rt_spec = rt_spec.with_editing(ev.clone(), state.tree.editing_text.clone());
            }
            let rt = Tree::from_spec(rt_spec, theme)
                .on_rename_start(cx.listener(|this, v: &str, _w, cx| {
                    let label = find_node(&this.state.tree.rename_nodes, v)
                        .map(|n| n.label.clone())
                        .unwrap_or_default();
                    this.state.tree.start_rename(v, &label);
                    cx.notify();
                }))
                .on_rename_change(cx.listener(|this, text: &str, _w, cx| {
                    this.state.tree.editing_text = text.to_string();
                    cx.notify();
                }))
                .on_rename_commit(cx.listener(|this, text: &str, _w, cx| {
                    this.state.tree.commit_rename(text);
                    cx.notify();
                }))
                .on_rename_cancel(cx.listener(|this, _v: &str, _w, cx| {
                    this.state.tree.cancel_rename();
                    cx.notify();
                }))
                .on_context_menu(cx.listener(|this, req: &TreeContextRequest, _w, cx| {
                    this.state.tree.open_menu(
                        &req.value,
                        f32::from(req.position.x) as i32,
                        f32::from(req.position.y) as i32,
                    );
                    cx.notify();
                }))
                .on_drag_over(cx.listener(|this, o: &TreeDragOver, _w, cx| {
                    this.state.tree.set_drop(&o.value, o.position);
                    cx.notify();
                }))
                .on_reorder(cx.listener(|this, req: &TreeReorderRequest, _w, cx| {
                    this.state.tree.reorder(&req.from, &req.to, req.position);
                    this.state.tree.clear_drop();
                    cx.notify();
                }));

            let mut section = div().relative().child(framed(theme, rt));
            if state.tree.menu_value.is_some() {
                let (mx, my) = state.tree.menu_pos;
                let menu = ContextMenu::from_spec(
                    ContextMenuSpec::new(vec![
                        MenuEntry::new("rename", "Rename"),
                        MenuEntry::new("delete", "Delete"),
                    ])
                    .with_default_open(true),
                    theme,
                )
                .anchor_point((mx, my))
                .on_select(cx.listener(|this, action: &str, _w, cx| {
                    if let Some(v) = this.state.tree.menu_value.clone() {
                        match action {
                            "rename" => {
                                let label = find_node(&this.state.tree.rename_nodes, &v)
                                    .map(|n| n.label.clone())
                                    .unwrap_or_default();
                                this.state.tree.start_rename(&v, &label);
                            }
                            "delete" => this.state.tree.delete_node(&v),
                            _ => {}
                        }
                    }
                    this.state.tree.close_menu();
                    cx.notify();
                }));
                section = section.child(menu);
            }
            section
        }))
        .child(labelled(
            theme,
            "Lazy / async children (loading row)",
            framed(
                theme,
                Tree::from_spec(
                    TreeSpec::new(vec![TreeNode::new("remote", "remote")
                        .with_icon("folder")
                        .with_branch(true)])
                    .with_expanded_values(vec!["remote".into()])
                    .with_loading_values(vec!["remote".into()]),
                    theme,
                ),
            ),
        ))
        .child(labelled(
            theme,
            "No guides, no icons",
            framed(
                theme,
                Tree::from_spec(
                    TreeSpec::new(file_tree())
                        .with_expanded_values(expanded())
                        .with_show_guides(false)
                        .with_show_icons(false),
                    theme,
                ),
            ),
        ))
        .child(labelled(theme, "Sizes (xs · sm · md · lg · xl)", sizes))
        .child(labelled(
            theme,
            "Densities (compact · default · comfortable)",
            densities,
        ))
}
