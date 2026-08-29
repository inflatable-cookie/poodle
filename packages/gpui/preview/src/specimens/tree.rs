use crate::app_state::{AppState, NodeSpecimenEvent, TreeEvent};
use crate::node_compat::{ContextMenu, Eyebrow, Tree};
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::{DropEdge, NodeKey, NodeModifiers, NodePoint};
use poodle_specs::{
    compute_selection, ContextMenuSpec, DropPosition, EyebrowSpec, MenuEntry, TreeNode,
    TreeSelectionMode, TreeSpec,
};

use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use std::sync::Arc;

fn context_action(
    state: &AppState,
    value: String,
    label: String,
) -> Arc<dyn Fn(&str) + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move |action| {
        events
            .lock()
            .unwrap()
            .push(NodeSpecimenEvent::TreeContextAction {
                action: action.to_string(),
                value: value.clone(),
                label: label.clone(),
            });
    })
}

/// Shorthand for the specimen's context-free event queue.
type Queue = Arc<std::sync::Mutex<Vec<NodeSpecimenEvent>>>;

fn push(queue: &Queue, event: TreeEvent) {
    queue.lock().unwrap().push(NodeSpecimenEvent::Tree(event));
}

/// A drop edge in the vocabulary's terms becomes the spec's drop position.
fn drop_position(edge: DropEdge) -> DropPosition {
    match edge {
        DropEdge::Before => DropPosition::Before,
        DropEdge::Inside => DropPosition::Inside,
        DropEdge::After => DropPosition::After,
    }
}

/// Resolve a modifier-aware click into the next selection, using the shared
/// `compute_selection` the web target drives. The visible order and the
/// selectable subset come from the spec, so the maths stays contract-owned.
fn selection_for(
    spec: &TreeSpec,
    selected: &[String],
    anchor: Option<&str>,
    value: &str,
    mods: NodeModifiers,
) -> TreeEvent {
    let rows = spec.visible_rows();
    let order: Vec<String> = rows.iter().map(|row| row.value.clone()).collect();
    let selectable: Vec<String> = rows
        .iter()
        .filter(|row| find_node(&spec.nodes, &row.value).is_none_or(|node| !node.is_disabled))
        .map(|row| row.value.clone())
        .collect();
    let mode = if mods.shift {
        TreeSelectionMode::Range
    } else if mods.accel {
        TreeSelectionMode::Toggle
    } else {
        TreeSelectionMode::Replace
    };
    let result = compute_selection(&order, &selectable, selected, anchor, value, mode);
    TreeEvent::Select {
        values: result.values,
        anchor: result.anchor,
        focused: value.to_string(),
    }
}

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
                        TreeNode::new("src/components/Tree.svelte", "Tree.svelte")
                            .with_icon("file"),
                    ],
                )
                .with_icon("folder"),
                // Empty-but-branch folder (lazy / no children yet).
                TreeNode::new("src/lib", "lib")
                    .with_icon("folder")
                    .with_branch(true),
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

fn large_nodes() -> Vec<TreeNode> {
    (0..24)
        .map(|i| {
            TreeNode::branch(
                format!("folder-{i}"),
                format!("Folder {i}"),
                (0..6)
                    .map(|j| {
                        TreeNode::new(
                            format!("folder-{i}/file-{j}"),
                            format!("file-{j}.ts"),
                        )
                        .with_icon("file")
                    })
                    .collect(),
            )
            .with_icon("folder")
        })
        .collect()
}

fn large_expanded() -> Vec<String> {
    (0..24).map(|i| format!("folder-{i}")).collect()
}

fn flat_nodes() -> Vec<TreeNode> {
    vec![
        TreeNode::new("beige", "BeigeButtonShadow").with_icon("monitor"),
        TreeNode::new("c28", "Component28").with_icon("monitor"),
        TreeNode::new("home", "Home").with_icon("monitor"),
        TreeNode::new("line129", "Line129").with_icon("monitor"),
    ]
}

fn nearly_flat_nodes() -> Vec<TreeNode> {
    let mut nodes = flat_nodes();
    nodes.insert(
        2,
        TreeNode::branch(
            "group",
            "A group",
            vec![TreeNode::new("child", "Nested").with_icon("monitor")],
        )
        .with_icon("folder"),
    );
    nodes
}

fn stacked(children: impl IntoIterator<Item = Div>) -> Div {
    let mut col = div().flex().flex_col().gap(px(16.0));
    for child in children {
        col = col.child(child);
    }
    col
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

fn framed_scroll(theme: &GpuiThemeProvider, content: impl IntoElement) -> Div {
    div()
        .w(px(288.0))
        .h(px(320.0))
        .border_1()
        .border_color(color_to_hsla(theme.resolve_color("color.border.subtle")))
        .rounded(px(6.0))
        .overflow_hidden()
        .child(
            div()
                .id("tree-large-scroll")
                .size_full()
                .overflow_y_scroll()
                .child(content),
        )
}

fn labelled(theme: &GpuiThemeProvider, label: &str, content: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(content)
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    // Every Tree handler is context-free now; the queue is the only bridge.
    let _ = cx;
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
    let queue: Queue = Arc::clone(&state.node_events);
    {
        let spec = TreeSpec::new(file_tree())
            .with_expanded_values(state.tree.expanded.clone())
            .with_selected_values(state.tree.selected.clone());
        let nav_spec = spec.clone();
        let selected = state.tree.selected.clone();
        let nav_selected = selected.clone();
        let anchor = state.tree.selection_anchor.clone();
        let nav_anchor = anchor.clone();
        interactive = interactive
            .on_toggle_expand({
                let queue = Arc::clone(&queue);
                Arc::new(move |v: &str| push(&queue, TreeEvent::ToggleExpand(v.to_string())))
            })
            .on_select_modified({
                let queue = Arc::clone(&queue);
                Arc::new(move |v: &str, mods: NodeModifiers| {
                    push(
                        &queue,
                        selection_for(&spec, &selected, anchor.as_deref(), v, mods),
                    );
                })
            })
            .on_key({
                let queue = Arc::clone(&queue);
                let nav = nav_spec;
                let selected = nav_selected;
                let anchor = nav_anchor;
                Arc::new(move |v: &str, key: NodeKey, mods: NodeModifiers| {
                    let rows = nav.visible_rows();
                    let at = rows.iter().position(|row| row.value == v);
                    match key {
                        // Arrow navigation resolves here because the specimen
                        // holds the flattened visible order the spec derives.
                        NodeKey::ArrowUp | NodeKey::ArrowDown => {
                            let Some(at) = at else { return };
                            let next = if key == NodeKey::ArrowUp {
                                at.checked_sub(1)
                            } else if at + 1 < rows.len() {
                                Some(at + 1)
                            } else {
                                None
                            };
                            let Some(next) = next else { return };
                            let target = &rows[next].value;
                            // Shift+Arrow extends the selection as it moves;
                            // a bare arrow only moves focus.
                            if mods.shift {
                                push(
                                    &queue,
                                    selection_for(&nav, &selected, anchor.as_deref(), target, mods),
                                );
                            } else {
                                push(&queue, TreeEvent::Focus(target.clone()));
                            }
                        }
                        NodeKey::Home | NodeKey::End => {
                            let target = if key == NodeKey::Home {
                                rows.first()
                            } else {
                                rows.last()
                            };
                            if let Some(row) = target {
                                push(&queue, TreeEvent::Focus(row.value.clone()));
                            }
                        }
                        // Right opens a collapsed branch, left closes an open
                        // one; on a leaf both are inert.
                        NodeKey::ArrowRight | NodeKey::ArrowLeft => {
                            let Some(at) = at else { return };
                            let row = &rows[at];
                            let opening = key == NodeKey::ArrowRight;
                            if row.is_branch && row.is_expanded != opening {
                                push(&queue, TreeEvent::ToggleExpand(v.to_string()));
                            }
                        }
                        NodeKey::Space => push(
                            &queue,
                            selection_for(
                                &nav,
                                &selected,
                                anchor.as_deref(),
                                v,
                                NodeModifiers {
                                    accel: true,
                                    ..NodeModifiers::default()
                                },
                            ),
                        ),
                        NodeKey::F2 | NodeKey::Delete | NodeKey::PageUp | NodeKey::PageDown => {}
                    }
                })
            });
    }
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(labelled(
            theme,
            "File explorer",
            framed(theme, interactive),
        ))
        .child(labelled(
            theme,
            "Selection modes",
            stacked([
                framed(
                    theme,
                    Tree::from_spec(
                        TreeSpec::new(file_tree())
                            .with_expanded_values(expanded())
                            .with_selected_values(vec![
                                "src/components/Button.svelte".into(),
                                "src/components/Tree.svelte".into(),
                            ])
                            .with_aria_label("Multi-select files"),
                        theme,
                    ),
                ),
                framed(
                    theme,
                    Tree::from_spec(
                        TreeSpec::new(file_tree())
                            .with_expanded_values(expanded())
                            .with_show_checkboxes(true)
                            .with_checked_values(state.tree.checked.clone()),
                        theme,
                    )
                    .on_check({
                        let queue = Arc::clone(&state.node_events);
                        Arc::new(move |v: &str| {
                            let nodes = file_tree();
                            let spec = TreeSpec::new(nodes.clone());
                            if let Some(node) = find_node(&nodes, v) {
                                push(&queue, TreeEvent::Check(spec.checkable_values_under(node)));
                            }
                        })
                    }),
                ),
            ]),
        ))
        .child(labelled(
            theme,
            "Presentation options",
            stacked([
                framed(
                    theme,
                    Tree::from_spec(
                        TreeSpec::new(flat_nodes())
                            .with_collapse_twisty_when_flat(true)
                            .with_aria_label("Flat list"),
                        theme,
                    ),
                ),
                framed(
                    theme,
                    Tree::from_spec(
                        TreeSpec::new(nearly_flat_nodes())
                            .with_collapse_twisty_when_flat(true)
                            .with_aria_label("Same list with a branch"),
                        theme,
                    ),
                ),
                framed(
                    theme,
                    Tree::from_spec(
                        TreeSpec::new(file_tree())
                            .with_expanded_values(expanded())
                            .with_show_guides(false)
                            .with_show_icons(false)
                            .with_aria_label("Plain tree"),
                        theme,
                    ),
                ),
            ]),
        ))
        .child(labelled(
            theme,
            "Loading and large data",
            stacked([
                framed(
                    theme,
                    Tree::from_spec(
                        TreeSpec::new(vec![TreeNode::new("remote", "remote")
                            .with_icon("folder")
                            .with_branch(true)])
                        .with_expanded_values(vec!["remote".into()])
                        .with_loading_values(vec!["remote".into()])
                        .with_aria_label("Lazy tree"),
                        theme,
                    ),
                ),
                framed_scroll(
                    theme,
                    Tree::from_spec(
                        TreeSpec::new(large_nodes())
                            .with_expanded_values(large_expanded())
                            .with_aria_label("Large tree"),
                        theme,
                    ),
                ),
            ]),
        ))
        .child(labelled(
            theme,
            "Editing and reordering",
            {
                let mut rt_spec = TreeSpec::new(state.tree.rename_nodes.clone())
                    .with_expanded_values(vec!["docs".into()])
                    .with_reorderable(true)
                    .with_drag(
                        None,
                        state.tree.drop_target.clone(),
                        state.tree.drop_position,
                    )
                    .with_aria_label("Rename tree");
                if let Some(ev) = &state.tree.editing_value {
                    rt_spec = rt_spec.with_editing(ev.clone(), state.tree.editing_text.clone());
                }
                let rename_nodes = state.tree.rename_nodes.clone();
                let hover_target = state.tree.drop_target.clone();
                let hover_position = state.tree.drop_position;
                let rt = Tree::from_spec(rt_spec, theme)
                    .on_key({
                        let queue = Arc::clone(&queue);
                        let nodes = rename_nodes.clone();
                        Arc::new(move |v: &str, key: NodeKey, _mods: NodeModifiers| {
                            if key == NodeKey::F2 {
                                let label = find_node(&nodes, v)
                                    .map(|n| n.label.clone())
                                    .unwrap_or_default();
                                push(
                                    &queue,
                                    TreeEvent::RenameStart {
                                        value: v.to_string(),
                                        label,
                                    },
                                );
                            }
                        })
                    })
                    .on_context_menu({
                        let queue = Arc::clone(&queue);
                        Arc::new(move |v: &str, point: NodePoint| {
                            push(
                                &queue,
                                TreeEvent::OpenMenu {
                                    value: v.to_string(),
                                    x: point.x as i32,
                                    y: point.y as i32,
                                },
                            );
                        })
                    })
                    .on_drag_over({
                        let queue = Arc::clone(&queue);
                        Arc::new(move |_from: &str, over: &str, edge: DropEdge| {
                            push(
                                &queue,
                                TreeEvent::SetDrop {
                                    value: over.to_string(),
                                    position: drop_position(edge),
                                },
                            );
                        })
                    })
                    .on_reorder({
                        let queue = Arc::clone(&queue);
                        Arc::new(move |from: &str, over: &str, edge: DropEdge| {
                            // Prefer the live hover state the last drag-over
                            // recorded: gpui does not hand `on_drop` a pointer
                            // position, so the edge it reports is the default.
                            let to = hover_target.clone().unwrap_or_else(|| over.to_string());
                            let position = if hover_target.is_some() {
                                hover_position
                            } else {
                                drop_position(edge)
                            };
                            push(
                                &queue,
                                TreeEvent::Reorder {
                                    from: from.to_string(),
                                    to,
                                    position,
                                },
                            );
                        })
                    });

                let mut section = div().relative().child(framed(theme, rt));
                if let Some(menu_value) = state.tree.menu_value.clone() {
                    let (mx, my) = state.tree.menu_pos;
                    let menu_label = find_node(&state.tree.rename_nodes, &menu_value)
                        .map(|node| node.label.clone())
                        .unwrap_or_default();
                    let menu = ContextMenu::from_spec(
                        ContextMenuSpec::new(vec![
                            MenuEntry::new("rename", "Rename"),
                            MenuEntry::new("delete", "Delete"),
                        ])
                        .with_default_open(true),
                        theme,
                    )
                    .anchor_point((mx, my))
                    .on_select(context_action(state, menu_value, menu_label));
                    section = section.child(menu);
                }
                section
            },
        ))
        .child(labelled(
            theme,
            "Disabled nodes",
            framed(
                theme,
                Tree::from_spec(
                    TreeSpec::new(file_tree())
                        .with_expanded_values(expanded())
                        .with_aria_label("Disabled nodes"),
                    theme,
                ),
            ),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "tree",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                Tree::from_spec(
                    TreeSpec::new(file_tree())
                        .with_expanded_values(expanded())
                        .with_selected_values(vec!["src/components/Tree.svelte".into()])
                        .with_size(size),
                    theme,
                )
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                Tree::from_spec(
                    TreeSpec::new(file_tree())
                        .with_expanded_values(expanded())
                        .with_selected_values(vec!["src/components/Tree.svelte".into()])
                        .with_density(density),
                    theme,
                )
                .into_any_element()
            }),
    )
}
