//! TreeSpec unit tests. Split out of `tree/mod.rs`.

use super::*;

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
    fn compute_selection_replace_toggle_range() {
        let order: Vec<String> = ["a", "b", "c", "d", "e"].iter().map(|s| s.to_string()).collect();
        // "c" is disabled (not selectable).
        let selectable: Vec<String> = ["a", "b", "d", "e"].iter().map(|s| s.to_string()).collect();

        // Replace -> just the value, anchor = value.
        let r = compute_selection(&order, &selectable, &["a".into()], Some("a"), "d", TreeSelectionMode::Replace);
        assert_eq!(r.values, vec!["d".to_string()]);
        assert_eq!(r.anchor.as_deref(), Some("d"));

        // Toggle add then remove.
        let r = compute_selection(&order, &selectable, &["a".into()], Some("a"), "b", TreeSelectionMode::Toggle);
        assert_eq!(r.values, vec!["a".to_string(), "b".to_string()]);
        let r = compute_selection(&order, &selectable, &["a".into(), "b".into()], Some("b"), "a", TreeSelectionMode::Toggle);
        assert_eq!(r.values, vec!["b".to_string()]);

        // Range from anchor "a" to "e", skipping disabled "c"; anchor preserved.
        let r = compute_selection(&order, &selectable, &[], Some("a"), "e", TreeSelectionMode::Range);
        assert_eq!(r.values, vec!["a".to_string(), "b".to_string(), "d".to_string(), "e".to_string()]);
        assert_eq!(r.anchor.as_deref(), Some("a"));

        // Range is order-independent (b..a same as a..b).
        let r = compute_selection(&order, &selectable, &[], Some("e"), "b", TreeSelectionMode::Range);
        assert_eq!(r.values, vec!["b".to_string(), "d".to_string(), "e".to_string()]);

        // Range with no anchor falls back to the single clicked value.
        let r = compute_selection(&order, &selectable, &[], None, "d", TreeSelectionMode::Range);
        assert_eq!(r.values, vec!["d".to_string()]);
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
