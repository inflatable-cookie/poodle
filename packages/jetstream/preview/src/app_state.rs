//! Application state for the Jetstream preview app.
//!
//! Mirrors the GPUI preview app's state structure: section navigation,
//! theme preset, density, control size, and per-specimen interaction state.

use poodle_specs::{reorder_nodes, DropPosition, TreeNode, TreeSpec};

fn set_label(nodes: &mut [TreeNode], value: &str, label: &str) {
    for n in nodes.iter_mut() {
        if n.value == value {
            n.label = label.to_string();
            return;
        }
        set_label(&mut n.children, value, label);
    }
}

fn remove_node(nodes: Vec<TreeNode>, value: &str) -> Vec<TreeNode> {
    nodes
        .into_iter()
        .filter(|n| n.value != value)
        .map(|mut n| {
            n.children = remove_node(n.children, value);
            n
        })
        .collect()
}

/// Which top-level section is active.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    Primitives,
    Composites,
    Demo,
    Tokens,
}

impl Section {
    pub const ALL: &[Section] = &[
        Section::Primitives,
        Section::Composites,
        Section::Demo,
        Section::Tokens,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Section::Primitives => "Primitives",
            Section::Composites => "Composites",
            Section::Demo => "Demo",
            Section::Tokens => "Tokens",
        }
    }
}

/// Available theme presets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemePreset {
    Dark,
    Light,
    LoopholeStudio,
}

impl ThemePreset {
    pub const ALL: &[ThemePreset] = &[
        ThemePreset::Dark,
        ThemePreset::Light,
        ThemePreset::LoopholeStudio,
    ];

    pub fn label(self) -> &'static str {
        match self {
            ThemePreset::Dark => "dark",
            ThemePreset::Light => "light",
            ThemePreset::LoopholeStudio => "loophole-studio",
        }
    }
}

/// Density mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Density {
    Comfortable,
    Compact,
}

impl Density {
    pub const ALL: &[Density] = &[Density::Comfortable, Density::Compact];

    pub fn label(self) -> &'static str {
        match self {
            Density::Comfortable => "comfortable",
            Density::Compact => "compact",
        }
    }
}

/// Control size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

impl ControlSize {
    pub const ALL: &[ControlSize] = &[ControlSize::Xl, ControlSize::Lg, ControlSize::Md, ControlSize::Sm, ControlSize::Xs];

    pub fn label(self) -> &'static str {
        match self {
            ControlSize::Xs => "xs",
            ControlSize::Sm => "sm",
            ControlSize::Md => "md",
            ControlSize::Lg => "lg",
            ControlSize::Xl => "xl",
        }
    }
}

/// Demo screen identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DemoScreen {
    OverviewShell,
    FormAndValidation,
    BrowseAndTable,
    DetailAndRelatedData,
    PickerAndMedia,
    CommandAndWorkspace,
}

impl DemoScreen {
    pub const ALL: &[DemoScreen] = &[
        DemoScreen::OverviewShell,
        DemoScreen::FormAndValidation,
        DemoScreen::BrowseAndTable,
        DemoScreen::DetailAndRelatedData,
        DemoScreen::PickerAndMedia,
        DemoScreen::CommandAndWorkspace,
    ];

    pub fn label(self) -> &'static str {
        match self {
            DemoScreen::OverviewShell => "Overview",
            DemoScreen::FormAndValidation => "Form",
            DemoScreen::BrowseAndTable => "Browse",
            DemoScreen::DetailAndRelatedData => "Detail",
            DemoScreen::PickerAndMedia => "Picker",
            DemoScreen::CommandAndWorkspace => "Workspace",
        }
    }
}

/// Canonical demo file tree shared by the Tree specimen and the keyboard loop.
pub fn demo_file_tree() -> Vec<TreeNode> {
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

/// Keyboard actions the Tree specimen responds to.
#[derive(Debug, Clone, Copy)]
pub enum TreeKey {
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    Enter,
}

/// Find a node by value anywhere in `nodes`.
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

/// Interactive state for the Tree specimen (expansion, selection, keyboard focus).
pub struct TreePreviewState {
    pub expanded: Vec<String>,
    pub selected: Vec<String>,
    pub focused: Option<String>,
    pub checked: Vec<String>,
    pub editing_value: Option<String>,
    pub editing_text: String,
    /// Mutable working copy of the tree (rename / delete / reorder mutate it).
    pub nodes: Vec<TreeNode>,
    /// Node grabbed at mouse-down, for drag-reorder on release.
    pub drag_source: Option<String>,
    pub menu_value: Option<String>,
    pub menu_pos: (f32, f32),
}

impl TreePreviewState {
    pub fn new() -> Self {
        Self {
            expanded: vec!["src".to_string(), "src/components".to_string()],
            selected: vec!["src/components/Tree.svelte".to_string()],
            focused: Some("src/components/Tree.svelte".to_string()),
            checked: vec!["src/components/Button.svelte".to_string()],
            editing_value: None,
            editing_text: String::new(),
            nodes: demo_file_tree(),
            drag_source: None,
            menu_value: None,
            menu_pos: (0.0, 0.0),
        }
    }

    pub fn toggle_expanded(&mut self, value: &str) {
        if let Some(i) = self.expanded.iter().position(|v| v == value) {
            self.expanded.remove(i);
        } else {
            self.expanded.push(value.to_string());
        }
    }

    pub fn select_only(&mut self, value: &str) {
        self.selected = vec![value.to_string()];
        self.focused = Some(value.to_string());
    }

    pub fn start_rename(&mut self, value: &str, label: &str) {
        self.editing_value = Some(value.to_string());
        self.editing_text = label.to_string();
    }
    pub fn cancel_rename(&mut self) {
        self.editing_value = None;
    }
    pub fn commit_rename(&mut self) {
        if let Some(value) = self.editing_value.take() {
            set_label(&mut self.nodes, &value, &self.editing_text);
        }
    }
    pub fn delete_node(&mut self, value: &str) {
        self.nodes = remove_node(std::mem::take(&mut self.nodes), value);
    }
    pub fn reorder(&mut self, from: &str, to: &str, position: DropPosition) {
        self.nodes = reorder_nodes(&self.nodes, from, to, position);
    }
    pub fn insert_char(&mut self, c: char) {
        if self.editing_value.is_some() && !c.is_control() {
            self.editing_text.push(c);
        }
    }
    pub fn backspace(&mut self) {
        if self.editing_value.is_some() {
            self.editing_text.pop();
        }
    }
    pub fn open_menu(&mut self, value: &str, x: f32, y: f32) {
        self.menu_value = Some(value.to_string());
        self.menu_pos = (x, y);
    }
    pub fn close_menu(&mut self) {
        self.menu_value = None;
    }
}

/// Global application state.
pub struct AppState {
    pub section: Section,
    pub theme_preset: ThemePreset,
    pub density: Density,
    pub control_size: ControlSize,
    pub active_primitive: Option<usize>,
    pub active_composite: Option<usize>,
    pub active_demo_screen: DemoScreen,
    pub disabled: bool,
    pub invalid: bool,
    pub busy: bool,
    /// Set to true when the UI tree needs a full rebuild.
    pub dirty: bool,
    /// Whether to reset the sidebar scroll on next rebuild.
    pub reset_sidebar_scroll: bool,
    /// Whether to reset the content scroll on next rebuild.
    pub reset_content_scroll: bool,
    /// Interactive state for the Tree specimen.
    pub tree: TreePreviewState,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            section: Section::Primitives,
            theme_preset: ThemePreset::Dark,
            density: Density::Comfortable,
            control_size: ControlSize::Md,
            active_primitive: None,
            active_composite: None,
            active_demo_screen: DemoScreen::OverviewShell,
            disabled: false,
            invalid: false,
            busy: false,
            dirty: true,
            reset_sidebar_scroll: true,
            reset_content_scroll: true,
            tree: TreePreviewState::new(),
        }
    }

    /// Current active component index for the active section.
    pub fn active_component(&self) -> Option<usize> {
        match self.section {
            Section::Primitives => self.active_primitive,
            Section::Composites => self.active_composite,
            _ => None,
        }
    }

    /// Set the active component for the current section.
    pub fn set_active_component(&mut self, idx: Option<usize>) {
        match self.section {
            Section::Primitives => self.active_primitive = idx,
            Section::Composites => self.active_composite = idx,
            _ => {}
        }
        self.dirty = true;
        // Only reset content scroll — sidebar stays where it is.
        self.reset_content_scroll = true;
    }

    /// Whether the Tree specimen is the active component (for keyboard routing).
    pub fn is_tree_active(&self) -> bool {
        if let Some(idx) = self.active_component() {
            let comps = crate::component_registry::components_for_section(self.section);
            comps.get(idx).map_or(false, |c| c.slug == "tree")
        } else {
            false
        }
    }

    /// Build the Tree spec from the current interactive state.
    pub fn tree_spec(&self) -> TreeSpec {
        let mut spec = TreeSpec::new(self.tree.nodes.clone())
            .with_expanded_values(self.tree.expanded.clone())
            .with_selected_values(self.tree.selected.clone())
            .with_checked_values(self.tree.checked.clone())
            .with_reorderable(true);
        if let Some(f) = &self.tree.focused {
            spec = spec.with_focused_value(f.clone());
        }
        if let Some(ev) = &self.tree.editing_value {
            spec = spec.with_editing(ev.clone(), self.tree.editing_text.clone());
        }
        spec
    }

    /// Begin renaming the focused node, seeding the draft from its label.
    pub fn tree_start_rename(&mut self) {
        if let Some(f) = self.tree.focused.clone() {
            let label = find_node(&self.tree.nodes, &f)
                .map(|n| n.label.clone())
                .unwrap_or_default();
            self.tree.start_rename(&f, &label);
            self.dirty = true;
        }
    }

    /// Commit the in-progress rename.
    pub fn tree_commit_rename(&mut self) {
        self.tree.commit_rename();
        self.dirty = true;
    }

    /// Open the context menu for the node under the cursor, if any.
    pub fn tree_open_menu(&mut self, value: &str, x: f32, y: f32) {
        self.tree.open_menu(value, x, y);
        self.dirty = true;
    }

    /// Context-menu "Rename" action: start renaming the menu's target node.
    pub fn tree_menu_rename(&mut self) {
        if let Some(value) = self.tree.menu_value.clone() {
            let label = find_node(&self.tree.nodes, &value)
                .map(|n| n.label.clone())
                .unwrap_or_default();
            self.tree.start_rename(&value, &label);
        }
        self.tree.close_menu();
        self.dirty = true;
    }

    /// Context-menu "Delete" action: remove the menu's target node.
    pub fn tree_menu_delete(&mut self) {
        if let Some(value) = self.tree.menu_value.clone() {
            self.tree.delete_node(&value);
        }
        self.tree.close_menu();
        self.dirty = true;
    }

    /// Apply a reorder (drag-drop or keyboard) to the working tree.
    pub fn tree_reorder(&mut self, from: &str, to: &str, position: DropPosition) {
        self.tree.reorder(from, to, position);
        self.dirty = true;
    }

    /// Drop position for a drag onto `to`: into branches, after leaves.
    pub fn tree_drop_position(&self, to: &str) -> DropPosition {
        let is_branch = find_node(&self.tree.nodes, to)
            .map_or(false, |n| !n.children.is_empty() || n.is_branch);
        if is_branch {
            DropPosition::Inside
        } else {
            DropPosition::After
        }
    }

    /// Alt+Up/Down: move the focused node among its siblings.
    pub fn tree_move_sibling(&mut self, up: bool) {
        let Some(focused) = self.tree.focused.clone() else {
            return;
        };
        let spec = self.tree_spec();
        let sibs = spec.siblings_of(&focused);
        let Some(i) = sibs.iter().position(|v| v == &focused) else {
            return;
        };
        if up && i > 0 {
            self.tree_reorder(&focused, &sibs[i - 1], DropPosition::Before);
        } else if !up && i + 1 < sibs.len() {
            self.tree_reorder(&focused, &sibs[i + 1], DropPosition::After);
        }
    }

    /// Cascade-toggle the checkbox for `value`: when all checkable descendants are
    /// checked, uncheck them all; otherwise check them all.
    pub fn tree_check(&mut self, value: &str) {
        let nodes = demo_file_tree();
        let Some(node) = find_node(&nodes, value) else {
            return;
        };
        let leaves = self.tree_spec().checkable_values_under(node);
        let all_on = leaves.iter().all(|v| self.tree.checked.contains(v));
        if all_on {
            self.tree.checked.retain(|v| !leaves.contains(v));
        } else {
            for v in leaves {
                if !self.tree.checked.contains(&v) {
                    self.tree.checked.push(v);
                }
            }
        }
        self.dirty = true;
    }

    /// Apply a keyboard action to the Tree specimen, mirroring the Svelte/GPUI
    /// keyboard model via the shared spec navigation helpers.
    pub fn tree_key(&mut self, key: TreeKey) {
        let spec = self.tree_spec();
        let rows = spec.visible_rows();
        let order: Vec<String> = rows.iter().map(|r| r.value.clone()).collect();
        let focused = self.tree.focused.clone();
        match key {
            TreeKey::Down => {
                let next = match focused.as_deref() {
                    Some(f) => spec.next_visible(f),
                    None => order.first().cloned(),
                };
                if let Some(n) = next {
                    self.tree.focused = Some(n);
                }
            }
            TreeKey::Up => {
                let prev = match focused.as_deref() {
                    Some(f) => spec.prev_visible(f),
                    None => order.last().cloned(),
                };
                if let Some(p) = prev {
                    self.tree.focused = Some(p);
                }
            }
            TreeKey::Right => {
                if let Some(f) = focused.as_deref() {
                    if let Some(r) = rows.iter().find(|r| r.value == f) {
                        if r.is_branch && !r.is_expanded {
                            self.tree.toggle_expanded(f);
                        } else if r.is_branch {
                            if let Some(n) = spec.next_visible(f) {
                                self.tree.focused = Some(n);
                            }
                        }
                    }
                }
            }
            TreeKey::Left => {
                if let Some(f) = focused.as_deref() {
                    if let Some(r) = rows.iter().find(|r| r.value == f) {
                        if r.is_branch && r.is_expanded {
                            self.tree.toggle_expanded(f);
                        } else if let Some(p) = spec.parent_of(f) {
                            self.tree.focused = Some(p);
                        }
                    }
                }
            }
            TreeKey::Home => {
                if let Some(first) = order.first() {
                    self.tree.focused = Some(first.clone());
                }
            }
            TreeKey::End => {
                if let Some(last) = order.last() {
                    self.tree.focused = Some(last.clone());
                }
            }
            TreeKey::Enter => {
                if let Some(f) = focused.as_deref() {
                    self.tree.select_only(f);
                }
            }
        }
        self.dirty = true;
    }

    /// Switch to a new section.
    pub fn set_section(&mut self, section: Section) {
        if self.section != section {
            self.section = section;
            self.dirty = true;
            // New section → reset both sidebar and content scroll.
            self.reset_sidebar_scroll = true;
            self.reset_content_scroll = true;
        }
    }

    /// Set theme preset, marking dirty for rebuild.
    pub fn set_theme(&mut self, preset: ThemePreset) {
        if self.theme_preset != preset {
            self.theme_preset = preset;
            self.dirty = true;
        }
    }

    /// Set density, marking dirty for rebuild.
    pub fn set_density(&mut self, density: Density) {
        if self.density != density {
            self.density = density;
            self.dirty = true;
        }
    }

    /// Set control size, marking dirty for rebuild.
    pub fn set_control_size(&mut self, size: ControlSize) {
        if self.control_size != size {
            self.control_size = size;
            self.dirty = true;
        }
    }

    /// Toggle disabled state.
    pub fn toggle_disabled(&mut self) {
        self.disabled = !self.disabled;
        self.dirty = true;
    }

    /// Toggle invalid state.
    pub fn toggle_invalid(&mut self) {
        self.invalid = !self.invalid;
        self.dirty = true;
    }

    /// Toggle busy state.
    pub fn toggle_busy(&mut self) {
        self.busy = !self.busy;
        self.dirty = true;
    }

    /// Set the active demo screen.
    pub fn set_demo_screen(&mut self, screen: DemoScreen) {
        if self.active_demo_screen != screen {
            self.active_demo_screen = screen;
            self.reset_content_scroll = true;
            self.dirty = true;
        }
    }
}
