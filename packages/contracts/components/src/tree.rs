use crate::{CheckState, ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

/// A single node in a [`TreeSpec`]. Nodes are recursive: a node with children
/// (or an explicit `is_branch` flag) renders as a branch with a twisty.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TreeNode {
    pub value: String,
    pub label: String,
    pub icon: Option<String>,
    pub children: Vec<TreeNode>,
    /// Force branch posture even when `children` is empty (empty / lazy folder).
    pub is_branch: bool,
    pub is_disabled: bool,
}

impl TreeNode {
    /// A leaf node with no children.
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            icon: None,
            children: Vec::new(),
            is_branch: false,
            is_disabled: false,
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

    /// Whether this node renders as a branch (twisty + `aria-expanded`).
    ///
    /// A node is a branch iff it has children or the `is_branch` override is set.
    pub fn renders_as_branch(&self) -> bool {
        self.is_branch || !self.children.is_empty()
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
        if node.children.is_empty() {
            vec![node.value.clone()]
        } else {
            let mut out = Vec::new();
            for child in &node.children {
                out.extend(self.checkable_values_under(child));
            }
            out
        }
    }

    /// Tri-state check state for a node, derived from its checkable descendants:
    /// `Checked` when all are checked, `Unchecked` when none, else `Mixed`.
    pub fn check_state(&self, node: &TreeNode) -> CheckState {
        let leaves = self.checkable_values_under(node);
        if leaves.is_empty() {
            return CheckState::Unchecked;
        }
        let checked = leaves.iter().filter(|v| self.is_checked(v)).count();
        if checked == 0 {
            CheckState::Unchecked
        } else if checked == leaves.len() {
            CheckState::Checked
        } else {
            CheckState::Mixed
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
            nodes
                .iter()
                .map(|n| 1 + count(&n.children))
                .sum()
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

    /// Accent base, tinted to 10% for the selected row fill.
    pub fn selected_fill_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    /// Accent base, tinted to 20% for the selected row inset ring.
    pub fn selected_ring_token(&self) -> &'static str {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> TreeSpec {
        TreeSpec::new(vec![
            TreeNode::branch(
                "src",
                "src",
                vec![
                    TreeNode::branch(
                        "src/components",
                        "components",
                        vec![TreeNode::new("src/components/Button.svelte", "Button.svelte")],
                    ),
                    TreeNode::new("src/index.ts", "index.ts"),
                ],
            ),
            TreeNode::new("README.md", "README.md").with_branch(false),
        ])
    }

    #[test]
    fn defaults_match_contract() {
        let spec = TreeSpec::default();
        assert!(spec.show_guides);
        assert!(spec.show_icons);
        assert_eq!(spec.size, ControlSize::Md);
        assert_eq!(spec.size_role, SemanticControlSizeRole::Chrome);
        assert_eq!(spec.density, ControlDensity::Default);
        assert!(spec.expanded_values.is_none());
    }

    #[test]
    fn branch_rule_uses_children_or_override() {
        let leaf = TreeNode::new("a", "a");
        assert!(!leaf.renders_as_branch());

        let empty_branch = TreeNode::new("b", "b").with_branch(true);
        assert!(empty_branch.renders_as_branch());

        let with_children = TreeNode::branch("c", "c", vec![TreeNode::new("d", "d")]);
        assert!(with_children.renders_as_branch());
    }

    #[test]
    fn current_expanded_prefers_controlled_over_default() {
        let spec = sample().with_default_expanded_values(vec!["src".into()]);
        assert_eq!(spec.current_expanded(), &["src".to_string()]);

        let controlled = spec.clone().with_expanded_values(vec!["src/components".into()]);
        assert_eq!(controlled.current_expanded(), &["src/components".to_string()]);
        assert!(controlled.is_expanded("src/components"));
        assert!(!controlled.is_expanded("src"));
    }

    #[test]
    fn selection_membership() {
        let spec = sample().with_selected_values(vec![
            "src/index.ts".into(),
            "README.md".into(),
        ]);
        assert!(spec.is_selected("src/index.ts"));
        assert!(spec.is_selected("README.md"));
        assert!(!spec.is_selected("src"));
        assert_eq!(spec.selected_count(), 2);
    }

    #[test]
    fn visible_rows_and_nav_helpers() {
        // src expanded, src/components collapsed.
        let spec = sample().with_expanded_values(vec!["src".into()]);
        let order = spec.visible_values_in_order();
        assert_eq!(
            order,
            vec![
                "src".to_string(),
                "src/components".to_string(),
                "src/index.ts".to_string(),
                "README.md".to_string(),
            ]
        );

        // Branch/expanded flags on the flattened rows.
        let rows = spec.visible_rows();
        let src = &rows[0];
        assert!(src.is_branch && src.is_expanded && src.depth == 0);
        let comp = &rows[1];
        assert!(comp.is_branch && !comp.is_expanded && comp.depth == 1);

        // next / prev / parent.
        assert_eq!(spec.next_visible("src").as_deref(), Some("src/components"));
        assert_eq!(spec.prev_visible("src/index.ts").as_deref(), Some("src/components"));
        assert_eq!(spec.next_visible("README.md"), None);
        assert_eq!(spec.prev_visible("src"), None);
        assert_eq!(spec.parent_of("src/components").as_deref(), Some("src"));
        assert_eq!(
            spec.parent_of("src/components/Button.svelte").as_deref(),
            Some("src/components")
        );
        assert_eq!(spec.parent_of("src"), None);
    }

    #[test]
    fn check_state_cascades_from_descendants() {
        // dir with two files + a sibling file.
        let nodes = vec![
            TreeNode::branch(
                "dir",
                "dir",
                vec![
                    TreeNode::new("dir/a", "a"),
                    TreeNode::new("dir/b", "b"),
                ],
            ),
            TreeNode::new("c", "c"),
        ];
        let dir = &nodes[0];

        let spec = TreeSpec::new(nodes.clone());
        assert_eq!(
            spec.checkable_values_under(dir),
            vec!["dir/a".to_string(), "dir/b".to_string()]
        );
        assert_eq!(spec.check_state(dir), CheckState::Unchecked);

        // One of two leaves checked -> Mixed.
        let partial = TreeSpec::new(nodes.clone()).with_checked_values(vec!["dir/a".into()]);
        assert_eq!(partial.check_state(dir), CheckState::Mixed);

        // Both leaves checked -> Checked.
        let full = TreeSpec::new(nodes.clone())
            .with_checked_values(vec!["dir/a".into(), "dir/b".into()]);
        assert_eq!(full.check_state(dir), CheckState::Checked);
    }

    fn reorder_tree() -> Vec<TreeNode> {
        vec![
            TreeNode::new("a", "a"),
            TreeNode::branch("b", "b", vec![TreeNode::new("b1", "b1"), TreeNode::new("b2", "b2")]),
            TreeNode::new("c", "c"),
        ]
    }

    fn vals(nodes: &[TreeNode]) -> Vec<String> {
        nodes.iter().map(|n| n.value.clone()).collect()
    }

    #[test]
    fn reorder_moves_nodes() {
        let t = reorder_tree();

        // a -> after c at root.
        let r = reorder_nodes(&t, "a", "c", DropPosition::After);
        assert_eq!(vals(&r), vec!["b", "c", "a"]);

        // c -> inside b (appended to its children).
        let r = reorder_nodes(&t, "c", "b", DropPosition::Inside);
        assert!(!r.iter().any(|n| n.value == "c"));
        let b = r.iter().find(|n| n.value == "b").unwrap();
        assert_eq!(vals(&b.children), vec!["b1", "b2", "c"]);

        // a -> before b1 (nested insertion).
        let r = reorder_nodes(&t, "a", "b1", DropPosition::Before);
        let b = r.iter().find(|n| n.value == "b").unwrap();
        assert_eq!(vals(&b.children), vec!["a", "b1", "b2"]);

        // descendant guard: cannot drop b into its own child.
        assert_eq!(reorder_nodes(&t, "b", "b1", DropPosition::Inside), t);
        // self + missing are no-ops.
        assert_eq!(reorder_nodes(&t, "a", "a", DropPosition::After), t);
        assert_eq!(reorder_nodes(&t, "a", "zz", DropPosition::After), t);
    }

    #[test]
    fn reorder_state_and_siblings() {
        let spec = TreeSpec::new(reorder_tree());
        assert!(!spec.reorderable);
        assert_eq!(spec.siblings_of("b1"), vec!["b1", "b2"]);
        assert_eq!(spec.siblings_of("a"), vec!["a", "b", "c"]);

        let s = spec
            .with_reorderable(true)
            .with_drag(Some("a".into()), Some("c".into()), DropPosition::After);
        assert!(s.reorderable);
        assert!(s.is_drag_value("a"));
        assert!(s.is_drop_target("c"));
        assert_eq!(s.drop_position, DropPosition::After);
    }

    #[test]
    fn editing_state() {
        let spec = TreeSpec::default();
        assert!(spec.editing_value.is_none());
        assert!(!spec.is_editing("a"));
        let editing = TreeSpec::new(vec![]).with_editing("a", "draft");
        assert!(editing.is_editing("a"));
        assert!(!editing.is_editing("b"));
        assert_eq!(editing.editing_text, "draft");
    }

    #[test]
    fn checkbox_lazy_defaults() {
        let spec = TreeSpec::default();
        assert!(!spec.show_checkboxes);
        assert!(spec.checked_values.is_empty());
        assert!(spec.loading_values.is_empty());
        let spec = TreeSpec::new(vec![]).with_loading_values(vec!["x".into()]);
        assert!(spec.is_loading("x"));
        assert!(!spec.is_loading("y"));
    }

    #[test]
    fn focused_value_defaults_none_and_sets() {
        assert!(TreeSpec::default().focused_value.is_none());
        let spec = TreeSpec::new(vec![]).with_focused_value("a");
        assert_eq!(spec.focused_value.as_deref(), Some("a"));
    }

    #[test]
    fn node_counts_respect_expansion() {
        let spec = sample();
        // 5 nodes total: src, src/components, Button.svelte, index.ts, README.md
        assert_eq!(spec.total_node_count(), 5);

        // Nothing expanded -> only the 2 roots are visible.
        assert_eq!(spec.visible_node_count(), 2);

        // Expand src -> its 2 children become visible (4 total).
        let one = spec.clone().with_expanded_values(vec!["src".into()]);
        assert_eq!(one.visible_node_count(), 4);

        // Expand both -> all 5 visible.
        let all = spec.with_expanded_values(vec!["src".into(), "src/components".into()]);
        assert_eq!(all.visible_node_count(), 5);
    }
}
