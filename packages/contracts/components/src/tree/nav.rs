//! TreeSpec query + navigation helpers (visible rows, checkbox cascade,
//! sibling/parent lookup). Split out of `tree/mod.rs` (god-file
//! decomposition); delegates cascade to poodle-headless. Unchanged.

use crate::CheckState;
use poodle_tokens::semantic;

use super::*;

impl TreeSpec {
    pub fn is_expanded(&self, value: &str) -> bool {
        self.current_expanded().iter().any(|v| v == value)
    }

    pub fn is_selected(&self, value: &str) -> bool {
        self.selected_values.iter().any(|v| v == value)
    }

    pub fn is_checked(&self, value: &str) -> bool {
        self.checked_values.iter().any(|v| v == value)
    }

    pub fn is_loading(&self, value: &str) -> bool {
        self.loading_values.iter().any(|v| v == value)
    }

    pub fn is_editing(&self, value: &str) -> bool {
        self.editing_value.as_deref() == Some(value)
    }

    pub fn is_drag_value(&self, value: &str) -> bool {
        self.drag_value.as_deref() == Some(value)
    }

    pub fn is_drop_target(&self, value: &str) -> bool {
        self.drop_target_value.as_deref() == Some(value)
    }

    /// The ordered sibling values of `value` (the children of its parent, or the
    /// root values when it has no parent).
    pub fn siblings_of(&self, value: &str) -> Vec<String> {
        match self.parent_of(value) {
            Some(parent) => find_node_ref(&self.nodes, &parent)
                .map(|p| p.children.iter().map(|c| c.value.clone()).collect())
                .unwrap_or_default(),
            None => self.nodes.iter().map(|n| n.value.clone()).collect(),
        }
    }

    /// The checkable atoms under `node`: the node itself when it has no children
    /// (leaf or empty/lazy branch), otherwise every leaf descendant. Used to
    /// cascade a branch check and to derive a branch's check state.
    pub fn checkable_values_under(&self, node: &TreeNode) -> Vec<String> {
        poodle_headless::tree::tree_checkable_under(node)
    }

    /// Tri-state check state for a node, derived from its checkable descendants:
    /// `Checked` when all are checked, `Unchecked` when none, else `Mixed`.
    pub fn check_state(&self, node: &TreeNode) -> CheckState {
        // Delegates to the poodle-headless cascade (conformance-tested
        // against the TS core). Leafless nodes cannot occur (a childless
        // node is its own checkable atom).
        match poodle_headless::tree::tree_check_state(node, &self.checked_values) {
            poodle_headless::tree::TreeCheckState::Checked => CheckState::Checked,
            poodle_headless::tree::TreeCheckState::Unchecked => CheckState::Unchecked,
            poodle_headless::tree::TreeCheckState::Mixed => CheckState::Mixed,
        }
    }

    /// Whether the given node renders as a branch.
    pub fn is_branch(&self, node: &TreeNode) -> bool {
        node.renders_as_branch()
    }

    pub fn selected_count(&self) -> usize {
        self.selected_values.len()
    }

    /// Total number of nodes in the tree (all depths).
    pub fn total_node_count(&self) -> usize {
        fn count(nodes: &[TreeNode]) -> usize {
            nodes.iter().map(|n| 1 + count(&n.children)).sum()
        }
        count(&self.nodes)
    }

    /// Number of rows currently visible: a node is visible iff every ancestor
    /// branch is expanded. Roots are always visible.
    pub fn visible_node_count(&self) -> usize {
        fn count(spec: &TreeSpec, nodes: &[TreeNode]) -> usize {
            nodes
                .iter()
                .map(|n| {
                    let mut c = 1;
                    if spec.is_branch(n) && spec.is_expanded(&n.value) {
                        c += count(spec, &n.children);
                    }
                    c
                })
                .sum()
        }
        count(self, &self.nodes)
    }

    // ── Keyboard navigation helpers ──────────────────────────────

    /// Flattened list of currently-visible rows in render order. A node is
    /// visible when every ancestor branch is expanded. Used by the Rust
    /// runtimes to drive keyboard navigation and reordering.
    pub fn visible_rows(&self) -> Vec<TreeVisibleRow> {
        fn walk(
            spec: &TreeSpec,
            nodes: &[TreeNode],
            depth: usize,
            parent: Option<&str>,
            out: &mut Vec<TreeVisibleRow>,
        ) {
            for node in nodes {
                let is_branch = spec.is_branch(node);
                let is_expanded = is_branch && spec.is_expanded(&node.value);
                out.push(TreeVisibleRow {
                    value: node.value.clone(),
                    parent: parent.map(str::to_string),
                    depth,
                    is_branch,
                    is_expanded,
                });
                if is_expanded {
                    walk(spec, &node.children, depth + 1, Some(&node.value), out);
                }
            }
        }
        let mut out = Vec::new();
        walk(self, &self.nodes, 0, None, &mut out);
        out
    }

    /// Values of all currently-visible nodes, in render order.
    pub fn visible_values_in_order(&self) -> Vec<String> {
        self.visible_rows().into_iter().map(|r| r.value).collect()
    }

    /// The visible node after `value`, or `None` if it is the last/absent.
    pub fn next_visible(&self, value: &str) -> Option<String> {
        let order = self.visible_values_in_order();
        let idx = order.iter().position(|v| v == value)?;
        order.get(idx + 1).cloned()
    }

    /// The visible node before `value`, or `None` if it is the first/absent.
    pub fn prev_visible(&self, value: &str) -> Option<String> {
        let order = self.visible_values_in_order();
        let idx = order.iter().position(|v| v == value)?;
        if idx == 0 {
            None
        } else {
            order.get(idx - 1).cloned()
        }
    }

    /// The parent value of `value` (search the full tree, not just visible).
    pub fn parent_of(&self, value: &str) -> Option<String> {
        fn walk(nodes: &[TreeNode], target: &str, parent: Option<&str>) -> Option<String> {
            for node in nodes {
                if node.value == target {
                    return parent.map(str::to_string);
                }
                if let Some(found) = walk(&node.children, target, Some(&node.value)) {
                    return Some(found);
                }
            }
            None
        }
        walk(&self.nodes, value, None)
    }

    // ── Token targets ────────────────────────────────────────────

    pub fn row_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn row_selected_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn row_hover_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    /// Accent base, tinted to 10% for the selected row fill (and the Svelte-only
    /// inset ring at 20%).
    pub fn selected_fill_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    /// Elevated background, tinted to 60% for the hover fill.
    pub fn hover_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn guide_color_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn twisty_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn icon_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }
}
