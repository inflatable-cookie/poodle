//! Application state for the Jetstream preview app.
//!
//! Mirrors the GPUI preview app's state structure: section navigation,
//! theme preset, density, control size, and per-specimen interaction state.

use poodle_specs::{reorder_nodes, DropPosition, TabDefinition, TreeNode, TreeSpec};

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

/// Which view of a specimen page is active (mirrors the Svelte
/// SpecimenLayout's Examples/Sizes/Densities tabs).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpecimenView {
    #[default]
    Examples,
    Sizes,
    Densities,
}

/// Which top-level section is active.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    Components,
    Demo,
    Tokens,
}

impl Section {
    pub const ALL: &[Section] = &[
        Section::Components,
        Section::Demo,
        Section::Tokens,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Section::Components => "Components",
            Section::Demo => "Demo",
            Section::Tokens => "Tokens",
        }
    }
}

/// Available theme presets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemePreset {
    Eclipse,
    Iceberg,
    Graphite,
    Midnight,
    Nord,
    Rose,
    Forest,
    Solarized,
    Hornet,
    Cobalt,
    Clay,
    Meadow,
}

impl ThemePreset {
    pub const ALL: &[ThemePreset] = &[
        ThemePreset::Eclipse,
        ThemePreset::Iceberg,
        ThemePreset::Graphite,
        ThemePreset::Midnight,
        ThemePreset::Nord,
        ThemePreset::Rose,
        ThemePreset::Forest,
        ThemePreset::Solarized,
        ThemePreset::Hornet,
        ThemePreset::Cobalt,
        ThemePreset::Clay,
        ThemePreset::Meadow,
    ];

    /// The token theme this preset selects. Mirrors `rebuild_shell`'s mapping;
    /// shared so the swatch builder and the renderer cannot drift apart.
    pub fn theme_definition(self) -> &'static poodle_tokens::themes::ThemeDefinition {
        match self {
            ThemePreset::Eclipse => &poodle_tokens::themes::ECLIPSE,
            ThemePreset::Iceberg => &poodle_tokens::themes::ICEBERG,
            ThemePreset::Graphite => &poodle_tokens::themes::GRAPHITE,
            ThemePreset::Midnight => &poodle_tokens::themes::MIDNIGHT,
            ThemePreset::Nord => &poodle_tokens::themes::NORD,
            ThemePreset::Rose => &poodle_tokens::themes::ROSE,
            ThemePreset::Forest => &poodle_tokens::themes::FOREST,
            ThemePreset::Solarized => &poodle_tokens::themes::SOLARIZED,
            ThemePreset::Hornet => &poodle_tokens::themes::HORNET,
            ThemePreset::Cobalt => &poodle_tokens::themes::COBALT,
            ThemePreset::Clay => &poodle_tokens::themes::CLAY,
            ThemePreset::Meadow => &poodle_tokens::themes::MEADOW,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            ThemePreset::Eclipse => "eclipse",
            ThemePreset::Iceberg => "iceberg",
            ThemePreset::Graphite => "graphite",
            ThemePreset::Midnight => "midnight",
            ThemePreset::Nord => "nord",
            ThemePreset::Rose => "rose",
            ThemePreset::Forest => "forest",
            ThemePreset::Solarized => "solarized",
            ThemePreset::Hornet => "hornet",
            ThemePreset::Cobalt => "cobalt",
            ThemePreset::Clay => "clay",
            ThemePreset::Meadow => "meadow",
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

/// Interactive state for the Tabs specimen (one reorderable instance).
///
/// Mirrors the Tree specimen's drag-reorder model: `drag_source` holds the tab
/// grabbed at mouse-down, `drop_target` follows the cursor during the drag so
/// the drop-target ring renders live, and `reorder` moves the dragged tab to
/// the target's position in the flat ordered list.
pub struct TabsPreviewState {
    /// Ordered list of tabs (drag-reorder mutates the order).
    pub tabs: Vec<TabDefinition>,
    /// Currently active tab value.
    pub active: String,
    /// Tab grabbed at mouse-down, for drag-reorder on release.
    pub drag_source: Option<String>,
    /// Tab under the cursor during a drag (drop-target ring).
    pub drop_target: Option<String>,
}

impl TabsPreviewState {
    pub fn new() -> Self {
        Self {
            tabs: vec![
                TabDefinition::new("overview", "Overview"),
                TabDefinition::new("details", "Details"),
                TabDefinition::new("settings", "Settings"),
                TabDefinition::new("activity", "Activity"),
            ],
            active: "overview".to_string(),
            drag_source: None,
            drop_target: None,
        }
    }

    /// Move tab `from` to `to`'s position in the flat list. No-op if either
    /// value is missing or they are the same. Reorder math comes from the
    /// shared tabs machinery in poodle-headless.
    pub fn reorder(&mut self, from: &str, to: &str) {
        let Some(from_idx) = self.tabs.iter().position(|t| t.value == from) else {
            return;
        };
        let Some(to_idx) = self.tabs.iter().position(|t| t.value == to) else {
            return;
        };
        let items: Vec<poodle_headless::tabs::TabsItem> = self
            .tabs
            .iter()
            .map(|t| poodle_headless::tabs::TabsItem {
                value: t.value.clone(),
                disabled: false,
                closable: false,
            })
            .collect();
        let (reordered, _) = poodle_headless::tabs::apply_reorder(&items, from_idx, to_idx);
        let order: Vec<String> = reordered.into_iter().map(|item| item.value).collect();
        self.tabs.sort_by_key(|t| order.iter().position(|v| *v == t.value).unwrap_or(usize::MAX));
    }
}

/// Global application state.
/// Swatch options for the header theme picker, resolved from each preset's own
/// tokens rather than hardcoded hex, and built once.
fn build_theme_options() -> Vec<poodle_specs::ThemeOption> {
    use poodle_adapter::ThemeProvider;
    fn hex(c: poodle_tokens::typed::ColorValue) -> String {
        let ch = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
        format!("#{:02x}{:02x}{:02x}", ch(c.0), ch(c.1), ch(c.2))
    }
    ThemePreset::ALL
        .iter()
        .map(|preset| {
            let t = poodle_jetstream::JetstreamThemeProvider::from_theme(
                preset.theme_definition(),
            );
            poodle_specs::ThemeOption::new(
                preset.label(),
                preset.label(),
                poodle_specs::ThemeSwatch::new(
                    hex(t.resolve_color("color.background.canvas")),
                    hex(t.resolve_color("color.background.panel")),
                    hex(t.resolve_color("color.accent.base")),
                    hex(t.resolve_color("color.text.primary")),
                    hex(t.resolve_color("color.border.default")),
                ),
            )
        })
        .collect()
}

pub struct AppState {
    pub section: Section,
    pub theme_preset: ThemePreset,
    pub density: Density,
    pub control_size: ControlSize,
    /// Neutral-contrast knob (mirrors the CSS `--poodle-contrast`; 0.5 = default).
    pub contrast: f32,
    /// Sidebar/catalogue filter (toolbar search box).
    pub search: String,
    /// The header theme picker's popover state; `ThemeSelectSpec::is_open` is
    /// controlled, so the shell owns it.
    pub theme_select_open: bool,
    /// Swatch options for the header theme picker, resolved once from each
    /// preset's own tokens.
    pub theme_options: Vec<poodle_specs::ThemeOption>,
    /// Active specimen-page view (Examples/Sizes/Densities).
    pub specimen_view: SpecimenView,
    pub active_component_idx: Option<usize>,
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
    /// Interactive state for the Tabs specimen (reorderable instance).
    pub tabs: TabsPreviewState,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            section: Section::Components,
            theme_preset: ThemePreset::Eclipse,
            density: Density::Comfortable,
            control_size: ControlSize::Md,
            contrast: 0.5,
            search: String::new(),
            theme_select_open: false,
            theme_options: build_theme_options(),
            specimen_view: SpecimenView::Examples,
            active_component_idx: None,
            active_demo_screen: DemoScreen::OverviewShell,
            disabled: false,
            invalid: false,
            busy: false,
            dirty: true,
            reset_sidebar_scroll: true,
            reset_content_scroll: true,
            tree: TreePreviewState::new(),
            tabs: TabsPreviewState::new(),
        }
    }

    /// Case-insensitive search filter over component display names.
    pub fn matches_search(&self, name: &str) -> bool {
        self.search.is_empty()
            || name.to_lowercase().contains(&self.search.to_lowercase())
    }

    /// Current active component index for the active section.
    pub fn active_component(&self) -> Option<usize> {
        match self.section {
            Section::Components => self.active_component_idx,
            _ => None,
        }
    }

    /// Set the active component for the current section.
    pub fn set_active_component(&mut self, idx: Option<usize>) {
        if self.section == Section::Components {
            self.active_component_idx = idx;
        }
        // A fresh component starts on its Examples view.
        self.specimen_view = SpecimenView::Examples;
        self.dirty = true;
        // Only reset content scroll — sidebar stays where it is.
        self.reset_content_scroll = true;
    }

    /// Whether the Tree specimen is the active component (for keyboard routing).
    pub fn is_tree_active(&self) -> bool {
        if let Some(idx) = self.active_component() {
            crate::component_registry::ALL_COMPONENTS
                .get(idx)
                .map_or(false, |c| c.slug == "tree")
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

    /// Move the dragged tab `from` to `to`'s position in the reorderable list.
    pub fn tabs_reorder(&mut self, from: &str, to: &str) {
        self.tabs.reorder(from, to);
        self.dirty = true;
    }

    /// Build the reorderable Tabs spec from the current interactive state.
    pub fn tabs_spec(&self) -> poodle_specs::TabsSpec {
        poodle_specs::TabsSpec::new(self.tabs.tabs.clone())
            .with_value(&self.tabs.active)
            .with_reorderable(true)
            .with_drag_value(self.tabs.drag_source.clone())
            .with_drop_target_value(self.tabs.drop_target.clone())
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
        let sibs = self.tree_spec().siblings_of(&focused);
        let Some(step) = poodle_headless::tree::tree_sibling_reorder_target(&sibs, &focused, up) else {
            return;
        };
        let position = if step.before { DropPosition::Before } else { DropPosition::After };
        self.tree_reorder(&focused, &step.target, position);
    }

    /// Cascade-toggle the checkbox for `value`: when all checkable descendants are
    /// checked, uncheck them all; otherwise check them all.
    pub fn tree_check(&mut self, value: &str) {
        let nodes = demo_file_tree();
        let Some(node) = find_node(&nodes, value) else {
            return;
        };
        self.tree.checked = poodle_headless::tree::tree_toggle_check(node, &self.tree.checked);
        self.dirty = true;
    }

    /// Apply a keyboard action to the Tree specimen, mirroring the Svelte/GPUI
    /// keyboard model via the shared spec navigation helpers.
    pub fn tree_key(&mut self, key: TreeKey) {
        use poodle_headless::tree::{tree_keydown_intent, TreeKeyIntent, TreeKeyModifiers};

        let spec = self.tree_spec();
        let rows = spec.visible_rows();
        let order: Vec<String> = rows.iter().map(|r| r.value.clone()).collect();

        // Without focus, Down/Up seed focus at the boundary (host behavior
        // that precedes the shared keyboard model).
        let Some(focused) = self.tree.focused.clone() else {
            match key {
                TreeKey::Down | TreeKey::Home => self.tree.focused = order.first().cloned(),
                TreeKey::Up | TreeKey::End => self.tree.focused = order.last().cloned(),
                _ => {}
            }
            self.dirty = true;
            return;
        };

        // Map the host key onto the shared model, then run the same
        // poodle-headless intent resolver the Svelte layer uses.
        let key_name = match key {
            TreeKey::Down => "ArrowDown",
            TreeKey::Up => "ArrowUp",
            TreeKey::Left => "ArrowLeft",
            TreeKey::Right => "ArrowRight",
            TreeKey::Home => "Home",
            TreeKey::End => "End",
            TreeKey::Enter => "Enter",
        };

        let headless_rows: Vec<poodle_headless::tree::TreeRow> = rows
            .iter()
            .map(|r| poodle_headless::tree::TreeRow {
                value: r.value.clone(),
                depth: r.depth,
                parent: r.parent.clone(),
                disabled: false,
                branch: r.is_branch,
                expanded: r.is_expanded,
            })
            .collect();

        let expanded = self.tree.expanded.clone();
        let intent = tree_keydown_intent(
            &headless_rows,
            &focused,
            key_name,
            TreeKeyModifiers { alt: false, shift: false },
            false,
            &expanded,
        );

        match intent {
            Some(TreeKeyIntent::Focus { value: Some(next), .. }) => {
                self.tree.focused = Some(next);
            }
            Some(TreeKeyIntent::Expand { value }) => self.tree.toggle_expanded(&value),
            Some(TreeKeyIntent::Collapse { value }) => self.tree.toggle_expanded(&value),
            Some(TreeKeyIntent::FocusParent { parent: Some(parent) }) => {
                self.tree.focused = Some(parent);
            }
            Some(TreeKeyIntent::Activate) => self.tree.select_only(&focused),
            _ => {}
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
