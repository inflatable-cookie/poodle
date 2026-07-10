//! Tree behavior machinery. Mirror of core `tree.ts`: visible-row
//! flattening, tri-state checkbox cascade, keyboard intents, shift-range
//! selection, sibling-reorder targets, and virtual-scroll windowing.
//! Generic over the host's node type via [`TreeNodeLike`] so `poodle-specs`
//! and runtime hosts can delegate without converting their trees.

pub trait TreeNodeLike: Sized {
    fn value(&self) -> &str;
    fn children(&self) -> &[Self];
    fn is_branch_flag(&self) -> bool;
    fn is_disabled(&self) -> bool;
}

pub fn is_tree_branch<N: TreeNodeLike>(node: &N) -> bool {
    node.is_branch_flag() || !node.children().is_empty()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeRow {
    pub value: String,
    pub depth: usize,
    pub parent: Option<String>,
    pub disabled: bool,
    pub branch: bool,
    pub expanded: bool,
}

/// Depth-first flatten of the rows currently visible given the expansion set.
pub fn flatten_visible_tree_rows<N: TreeNodeLike>(nodes: &[N], expanded: &[String]) -> Vec<TreeRow> {
    fn walk<N: TreeNodeLike>(
        nodes: &[N],
        expanded: &[String],
        depth: usize,
        parent: Option<&str>,
        out: &mut Vec<TreeRow>,
    ) {
        for node in nodes {
            let branch = is_tree_branch(node);
            let is_expanded = expanded.iter().any(|value| value == node.value());

            out.push(TreeRow {
                value: node.value().to_string(),
                depth,
                parent: parent.map(str::to_string),
                disabled: node.is_disabled(),
                branch,
                expanded: branch && is_expanded,
            });

            if branch && is_expanded && !node.children().is_empty() {
                walk(node.children(), expanded, depth + 1, Some(node.value()), out);
            }
        }
    }

    let mut out = Vec::new();
    walk(nodes, expanded, 0, None, &mut out);
    out
}

pub fn find_tree_node<'a, N: TreeNodeLike>(nodes: &'a [N], value: &str) -> Option<&'a N> {
    for node in nodes {
        if node.value() == value {
            return Some(node);
        }

        if let Some(found) = find_tree_node(node.children(), value) {
            return Some(found);
        }
    }

    None
}

// ── Tri-state checkbox cascade ──

/// Checkable atoms under a node: itself when childless, else every leaf descendant.
pub fn tree_checkable_under<N: TreeNodeLike>(node: &N) -> Vec<String> {
    if node.children().is_empty() {
        return vec![node.value().to_string()];
    }

    node.children().iter().flat_map(tree_checkable_under).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeCheckState {
    Checked,
    Unchecked,
    Mixed,
}

pub fn tree_check_state<N: TreeNodeLike>(node: &N, checked_values: &[String]) -> TreeCheckState {
    let leaves = tree_checkable_under(node);
    let checked = leaves.iter().filter(|value| checked_values.contains(value)).count();

    if checked == 0 {
        TreeCheckState::Unchecked
    } else if checked == leaves.len() {
        TreeCheckState::Checked
    } else {
        TreeCheckState::Mixed
    }
}

/// Cascade toggle: all-on clears the subtree, otherwise checks it fully.
pub fn tree_toggle_check<N: TreeNodeLike>(node: &N, checked_values: &[String]) -> Vec<String> {
    let leaves = tree_checkable_under(node);
    let all_on = leaves.iter().all(|value| checked_values.contains(value));

    if all_on {
        checked_values
            .iter()
            .filter(|value| !leaves.contains(value))
            .cloned()
            .collect()
    } else {
        let mut next = checked_values.to_vec();

        for leaf in leaves {
            if !next.contains(&leaf) {
                next.push(leaf);
            }
        }

        next
    }
}

// ── Shift-range selection over the visible order ──

pub fn tree_range_selection(rows: &[TreeRow], anchor: Option<&str>, to_value: &str) -> Option<Vec<String>> {
    let order: Vec<&str> = rows.iter().map(|row| row.value.as_str()).collect();
    let a = order.iter().position(|value| *value == anchor.unwrap_or(to_value))?;
    let b = order.iter().position(|value| *value == to_value)?;
    let (lo, hi) = if a <= b { (a, b) } else { (b, a) };

    Some(
        rows[lo..=hi]
            .iter()
            .filter(|row| !row.disabled)
            .map(|row| row.value.clone())
            .collect(),
    )
}

// ── Sibling reorder target ──

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeReorderMove {
    pub target: String,
    pub before: bool,
}

pub fn tree_sibling_reorder_target(siblings: &[String], value: &str, up: bool) -> Option<TreeReorderMove> {
    let index = siblings.iter().position(|candidate| candidate == value)?;
    let next_index = if up { index.checked_sub(1)? } else { index + 1 };
    let target = siblings.get(next_index)?;

    Some(TreeReorderMove { target: target.clone(), before: up })
}

// ── Keyboard intents ──

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeKeyIntent {
    Focus { value: Option<String>, extend_selection: bool },
    Expand { value: String },
    Collapse { value: String },
    FocusParent { parent: Option<String> },
    MoveSibling { up: bool },
    Activate,
    ToggleSelection,
    StartRename,
}

pub struct TreeKeyModifiers {
    pub alt: bool,
    pub shift: bool,
}

/// Resolve a treeitem keydown to an intent the host executes. Mirrors the
/// TS core (and the pre-machine Svelte behavior) exactly.
pub fn tree_keydown_intent(
    rows: &[TreeRow],
    current_value: &str,
    key: &str,
    modifiers: TreeKeyModifiers,
    reorderable: bool,
    expanded: &[String],
) -> Option<TreeKeyIntent> {
    let index = rows.iter().position(|row| row.value == current_value)?;
    let row = &rows[index];

    match key {
        "ArrowDown" => {
            if modifiers.alt && reorderable {
                return Some(TreeKeyIntent::MoveSibling { up: false });
            }

            let next = rows.get(index + 1);

            Some(TreeKeyIntent::Focus {
                value: next.map(|r| r.value.clone()),
                extend_selection: modifiers.shift && next.map_or(false, |r| !r.disabled),
            })
        }
        "ArrowUp" => {
            if modifiers.alt && reorderable {
                return Some(TreeKeyIntent::MoveSibling { up: true });
            }

            let prev = index.checked_sub(1).and_then(|i| rows.get(i));

            Some(TreeKeyIntent::Focus {
                value: prev.map(|r| r.value.clone()),
                extend_selection: modifiers.shift && prev.map_or(false, |r| !r.disabled),
            })
        }
        "ArrowRight" => {
            if !row.branch {
                return None;
            }

            if !expanded.iter().any(|value| *value == row.value) {
                return Some(TreeKeyIntent::Expand { value: row.value.clone() });
            }

            Some(TreeKeyIntent::Focus {
                value: rows.get(index + 1).map(|r| r.value.clone()),
                extend_selection: false,
            })
        }
        "ArrowLeft" => {
            if row.branch && expanded.iter().any(|value| *value == row.value) {
                return Some(TreeKeyIntent::Collapse { value: row.value.clone() });
            }

            Some(TreeKeyIntent::FocusParent { parent: row.parent.clone() })
        }
        "Home" => Some(TreeKeyIntent::Focus {
            value: rows.first().map(|r| r.value.clone()),
            extend_selection: false,
        }),
        "End" => Some(TreeKeyIntent::Focus {
            value: rows.last().map(|r| r.value.clone()),
            extend_selection: false,
        }),
        "Enter" => {
            if row.disabled {
                None
            } else {
                Some(TreeKeyIntent::Activate)
            }
        }
        " " => {
            if row.disabled {
                None
            } else {
                Some(TreeKeyIntent::ToggleSelection)
            }
        }
        "F2" => Some(TreeKeyIntent::StartRename),
        _ => None,
    }
}

// ── Virtual-scroll windowing ──

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TreeVirtualWindow {
    pub start_index: usize,
    pub end_index: usize,
    pub offset_y: f64,
    pub total_height: f64,
}

pub fn tree_virtual_window(
    row_count: usize,
    row_height_px: f64,
    scroll_top: f64,
    viewport_height_px: f64,
    overscan: usize,
) -> TreeVirtualWindow {
    let start_index = ((scroll_top / row_height_px).floor() as i64 - overscan as i64).max(0) as usize;
    let end_index = (((scroll_top + viewport_height_px) / row_height_px).ceil() as usize + overscan).min(row_count);

    TreeVirtualWindow {
        start_index,
        end_index,
        offset_y: start_index as f64 * row_height_px,
        total_height: row_count as f64 * row_height_px,
    }
}
