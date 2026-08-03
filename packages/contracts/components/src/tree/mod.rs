use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};

/// A single node in a [`TreeSpec`]. Nodes are recursive: a node with children
/// (or an explicit `is_branch` flag) renders as a branch with a twisty.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TreeNode {
    pub value: String,
    pub label: String,
    pub end_label: Option<String>,
    pub icon: Option<String>,
    pub children: Vec<TreeNode>,
    /// Force branch posture even when `children` is empty (empty / lazy folder).
    pub is_branch: bool,
    pub is_disabled: bool,
    pub is_muted: bool,
}

impl TreeNode {
    /// A leaf node with no children.
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            end_label: None,
            icon: None,
            children: Vec::new(),
            is_branch: false,
            is_disabled: false,
            is_muted: false,
        }
    }

    /// A branch node carrying the given children.
    pub fn branch(
        value: impl Into<String>,
        label: impl Into<String>,
        children: Vec<TreeNode>,
    ) -> Self {
        Self {
            children,
            ..Self::new(value, label)
        }
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_end_label(mut self, end_label: impl Into<String>) -> Self {
        self.end_label = Some(end_label.into());
        self
    }

    pub fn with_children(mut self, children: Vec<TreeNode>) -> Self {
        self.children = children;
        self
    }

    /// Mark this node as a branch even if it currently has no children.
    pub fn with_branch(mut self, is_branch: bool) -> Self {
        self.is_branch = is_branch;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_muted(mut self, is_muted: bool) -> Self {
        self.is_muted = is_muted;
        self
    }

    /// Whether this node renders as a branch (twisty + `aria-expanded`).
    ///
    /// A node is a branch iff it has children or the `is_branch` override is set.
    pub fn renders_as_branch(&self) -> bool {
        self.is_branch || !self.children.is_empty()
    }
}

impl poodle_headless::tree::TreeNodeLike for TreeNode {
    fn value(&self) -> &str {
        &self.value
    }
    fn children(&self) -> &[Self] {
        &self.children
    }
    fn is_branch_flag(&self) -> bool {
        self.is_branch
    }
    fn is_disabled(&self) -> bool {
        self.is_disabled
    }
}

/// A flattened, currently-visible tree row. Produced by
/// [`TreeSpec::visible_rows`] for keyboard navigation and reordering.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TreeVisibleRow {
    pub value: String,
    pub parent: Option<String>,
    pub depth: usize,
    pub is_branch: bool,
    pub is_expanded: bool,
}

/// How a click/keypress changes the multi-selection set.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TreeSelectionMode {
    /// Plain click / Enter: select only this value.
    Replace,
    /// Ctrl/Cmd+click / Space: toggle this value in the set.
    Toggle,
    /// Shift+click / Shift+Arrow: select the contiguous range from the anchor.
    Range,
}

/// The next selection set + range anchor computed by [`compute_selection`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TreeSelectionResult {
    pub values: Vec<String>,
    pub anchor: Option<String>,
}

/// The contiguous selectable range between `a` and `b` over `order`, skipping
/// non-selectable (e.g. disabled) values. Falls back to `[b]` if either is absent.
fn selection_range(order: &[String], selectable: &[String], a: &str, b: &str) -> Vec<String> {
    match (
        order.iter().position(|v| v == a),
        order.iter().position(|v| v == b),
    ) {
        (Some(ia), Some(ib)) => {
            let (lo, hi) = if ia <= ib { (ia, ib) } else { (ib, ia) };
            order[lo..=hi]
                .iter()
                .filter(|v| selectable.iter().any(|s| s == *v))
                .cloned()
                .collect()
        }
        _ => vec![b.to_string()],
    }
}

/// Compute the next multi-selection set for an interaction on `value`.
///
/// Shared, runtime-agnostic selection logic mirroring the Svelte reference:
/// `Replace` → `[value]`; `Toggle` → add/remove `value`; `Range` → the
/// contiguous selectable span from `anchor` (or `value`) to `value`. `order` is
/// the visible values in render order, `selectable` the non-disabled subset.
pub fn compute_selection(
    order: &[String],
    selectable: &[String],
    selected: &[String],
    anchor: Option<&str>,
    value: &str,
    mode: TreeSelectionMode,
) -> TreeSelectionResult {
    match mode {
        TreeSelectionMode::Replace => TreeSelectionResult {
            values: vec![value.to_string()],
            anchor: Some(value.to_string()),
        },
        TreeSelectionMode::Toggle => {
            let mut values = selected.to_vec();
            if let Some(p) = values.iter().position(|v| v == value) {
                values.remove(p);
            } else {
                values.push(value.to_string());
            }
            TreeSelectionResult {
                values,
                anchor: Some(value.to_string()),
            }
        }
        TreeSelectionMode::Range => {
            let a = anchor.unwrap_or(value);
            TreeSelectionResult {
                values: selection_range(order, selectable, a, value),
                anchor: Some(a.to_string()),
            }
        }
    }
}

/// Where a dragged node lands relative to the drop-target node.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum DropPosition {
    /// As a sibling immediately before the target.
    Before,
    /// As a sibling immediately after the target.
    #[default]
    After,
    /// As a (last) child of the target.
    Inside,
}

fn find_node_ref<'a>(nodes: &'a [TreeNode], value: &str) -> Option<&'a TreeNode> {
    for node in nodes {
        if node.value == value {
            return Some(node);
        }
        if let Some(found) = find_node_ref(&node.children, value) {
            return Some(found);
        }
    }
    None
}

fn subtree_contains(nodes: &[TreeNode], value: &str) -> bool {
    nodes
        .iter()
        .any(|n| n.value == value || subtree_contains(&n.children, value))
}

fn remove_value(nodes: &[TreeNode], value: &str) -> (Vec<TreeNode>, Option<TreeNode>) {
    let mut removed = None;
    let mut out = Vec::with_capacity(nodes.len());
    for node in nodes {
        if node.value == value {
            removed = Some(node.clone());
            continue;
        }
        let (children, r) = remove_value(&node.children, value);
        if r.is_some() {
            removed = r;
        }
        let mut copy = node.clone();
        copy.children = children;
        out.push(copy);
    }
    (out, removed)
}

/// Insert `node` relative to `to`. Returns `Some(node)` if `to` was not found
/// (the node is handed back, uninserted).
fn insert_relative(
    nodes: &mut Vec<TreeNode>,
    to: &str,
    node: TreeNode,
    position: DropPosition,
) -> Option<TreeNode> {
    match position {
        DropPosition::Before | DropPosition::After => {
            if let Some(idx) = nodes.iter().position(|n| n.value == to) {
                let at = if matches!(position, DropPosition::After) {
                    idx + 1
                } else {
                    idx
                };
                nodes.insert(at, node);
                return None;
            }
        }
        DropPosition::Inside => {
            if let Some(parent) = nodes.iter_mut().find(|n| n.value == to) {
                parent.children.push(node);
                return None;
            }
        }
    }
    let mut carried = node;
    for n in nodes.iter_mut() {
        match insert_relative(&mut n.children, to, carried, position) {
            None => return None,
            Some(returned) => carried = returned,
        }
    }
    Some(carried)
}

/// Move `from` to land relative to `to` per `position`, returning the new tree.
/// No-ops (returns a clone of `nodes`) when the move is invalid: same node,
/// missing node, or dropping a node into its own subtree.
pub fn reorder_nodes(
    nodes: &[TreeNode],
    from: &str,
    to: &str,
    position: DropPosition,
) -> Vec<TreeNode> {
    if from == to || find_node_ref(nodes, to).is_none() {
        return nodes.to_vec();
    }
    let Some(from_node) = find_node_ref(nodes, from) else {
        return nodes.to_vec();
    };
    // Cannot drop a node into its own descendant.
    if subtree_contains(&from_node.children, to) {
        return nodes.to_vec();
    }
    let (mut tree, extracted) = remove_value(nodes, from);
    let Some(moved) = extracted else {
        return nodes.to_vec();
    };
    if insert_relative(&mut tree, to, moved, position).is_some() {
        return nodes.to_vec();
    }
    tree
}

/// Tree -- a hierarchical, collapsible disclosure list (file-explorer style).
///
/// Selection is multi-select (controlled via `selected_values`). Expansion is
/// controlled when `expanded_values` is `Some`, otherwise it falls back to
/// `default_expanded_values` for uncontrolled use.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TreeSpec {
    pub nodes: Vec<TreeNode>,
    pub selected_values: Vec<String>,
    pub expanded_values: Option<Vec<String>>,
    pub default_expanded_values: Vec<String>,
    /// Value of the node holding keyboard focus (roving tabindex / focus ring).
    pub focused_value: Option<String>,
    /// Values of checked checkable nodes (leaves + empty branches). Branch
    /// check state is derived from these, not stored.
    pub checked_values: Vec<String>,
    /// Branches whose children are currently being lazily loaded.
    pub loading_values: Vec<String>,
    /// Value of the node currently in inline-rename mode, if any.
    pub editing_value: Option<String>,
    /// Draft text for the node being renamed.
    pub editing_text: String,
    /// Whether rows can be drag-reordered.
    pub reorderable: bool,
    /// The node currently being dragged, if any.
    pub drag_value: Option<String>,
    /// The current drop-target node, if any.
    pub drop_target_value: Option<String>,
    /// Where the dragged node would land relative to the drop target.
    pub drop_position: DropPosition,
    pub aria_label: Option<String>,
    /// Reclaim the twisty gutter while nothing in the tree can expand.
    ///
    /// A leaf renders a twisty-sized spacer so its label lines up with branch
    /// labels; on a genuinely flat tree that aligns labels with a chevron that
    /// will never appear, leaving an empty column down the left. Opt-in rather
    /// than automatic, because a tree whose nodes arrive asynchronously would
    /// otherwise shift every row sideways the first time a branch loads.
    pub collapse_twisty_when_flat: bool,
    pub show_guides: bool,
    pub show_icons: bool,
    /// Render a leading checkbox per row (cascade selection).
    pub show_checkboxes: bool,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for TreeSpec {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            selected_values: Vec::new(),
            expanded_values: None,
            default_expanded_values: Vec::new(),
            focused_value: None,
            checked_values: Vec::new(),
            loading_values: Vec::new(),
            editing_value: None,
            editing_text: String::new(),
            reorderable: false,
            drag_value: None,
            drop_target_value: None,
            drop_position: DropPosition::After,
            aria_label: None,
            collapse_twisty_when_flat: false,
            show_guides: true,
            show_icons: true,
            show_checkboxes: false,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Chrome,
            density: ControlDensity::Default,
        }
    }
}

impl TreeSpec {
    /// Whether the twisty gutter should be reclaimed for this render.
    ///
    /// The condition is the **whole tree, not the node**: one branch anywhere
    /// restores the spacer for every row, because the moment one label needs
    /// the gutter they all need it to stay aligned.
    pub fn is_flat(&self) -> bool {
        fn any_branch(nodes: &[TreeNode]) -> bool {
            nodes.iter().any(|node| {
                node.is_branch || !node.children.is_empty() || any_branch(&node.children)
            })
        }
        self.collapse_twisty_when_flat && !any_branch(&self.nodes)
    }

    pub fn with_collapse_twisty_when_flat(mut self, value: bool) -> Self {
        self.collapse_twisty_when_flat = value;
        self
    }

    pub fn new(nodes: Vec<TreeNode>) -> Self {
        Self {
            nodes,
            ..Self::default()
        }
    }

    pub fn with_selected_values(mut self, selected_values: Vec<String>) -> Self {
        self.selected_values = selected_values;
        self
    }

    /// Set the controlled expanded set.
    pub fn with_expanded_values(mut self, expanded_values: Vec<String>) -> Self {
        self.expanded_values = Some(expanded_values);
        self
    }

    pub fn with_default_expanded_values(mut self, default_expanded_values: Vec<String>) -> Self {
        self.default_expanded_values = default_expanded_values;
        self
    }

    pub fn with_focused_value(mut self, focused_value: impl Into<String>) -> Self {
        self.focused_value = Some(focused_value.into());
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_show_guides(mut self, show_guides: bool) -> Self {
        self.show_guides = show_guides;
        self
    }

    pub fn with_show_icons(mut self, show_icons: bool) -> Self {
        self.show_icons = show_icons;
        self
    }

    pub fn with_show_checkboxes(mut self, show_checkboxes: bool) -> Self {
        self.show_checkboxes = show_checkboxes;
        self
    }

    pub fn with_checked_values(mut self, checked_values: Vec<String>) -> Self {
        self.checked_values = checked_values;
        self
    }

    pub fn with_loading_values(mut self, loading_values: Vec<String>) -> Self {
        self.loading_values = loading_values;
        self
    }

    /// Put `value` into inline-rename mode seeded with `text`.
    pub fn with_editing(mut self, value: impl Into<String>, text: impl Into<String>) -> Self {
        self.editing_value = Some(value.into());
        self.editing_text = text.into();
        self
    }

    pub fn with_reorderable(mut self, reorderable: bool) -> Self {
        self.reorderable = reorderable;
        self
    }

    /// Set the active drag + drop-target indicator state.
    pub fn with_drag(
        mut self,
        drag_value: Option<String>,
        drop_target_value: Option<String>,
        drop_position: DropPosition,
    ) -> Self {
        self.drag_value = drag_value;
        self.drop_target_value = drop_target_value;
        self.drop_position = drop_position;
        self
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }

    // ── Resolution helpers ───────────────────────────────────────

    /// The effective expanded set: the controlled `expanded_values` when set,
    /// otherwise `default_expanded_values`.
    pub fn current_expanded(&self) -> &[String] {
        self.expanded_values
            .as_deref()
            .unwrap_or(&self.default_expanded_values)
    }
}

mod nav;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod flat_tests {
    use super::*;

    fn leaf(v: &str) -> TreeNode {
        TreeNode::new(v, v)
    }

    /// One branch anywhere restores the gutter for every row — the alignment is
    /// a property of the tree, not of the node.
    #[test]
    fn a_single_branch_anywhere_un_flattens_the_tree() {
        let flat = TreeSpec::new(vec![leaf("a"), leaf("b")]).with_collapse_twisty_when_flat(true);
        assert!(flat.is_flat());

        let mut parent = TreeNode::new("group", "Group");
        parent.children = vec![leaf("child")];
        let nested = TreeSpec::new(vec![leaf("a"), parent]).with_collapse_twisty_when_flat(true);
        assert!(
            !nested.is_flat(),
            "a branch anywhere brings the gutter back"
        );
    }

    /// Opt-in: flatness alone does not collapse the gutter, because a tree
    /// loading nodes asynchronously would otherwise shift every row sideways
    /// the first time a branch arrives.
    #[test]
    fn flatness_alone_does_not_collapse_the_gutter() {
        assert!(!TreeSpec::new(vec![leaf("a")]).is_flat());
    }
}
