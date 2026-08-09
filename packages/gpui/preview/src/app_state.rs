//! Application state for the preview app.
//!
//! Mirrors the current Svelte preview shell: theme, density, control size,
//! appearance treatment, component search, active section, and component selection.

use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{reorder_nodes, DropPosition, TreeNode};
use std::collections::HashMap;

/// Demo tree for the rename / context-menu / reorder specimen.
pub fn docs_tree() -> Vec<TreeNode> {
    vec![
        TreeNode::branch(
            "docs",
            "docs",
            vec![
                TreeNode::new("docs/intro.md", "intro.md").with_icon("file"),
                TreeNode::new("docs/guide.md", "guide.md").with_icon("file"),
            ],
        )
        .with_icon("folder"),
        TreeNode::new("notes.txt", "notes.txt").with_icon("file"),
    ]
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
    Components,
    Tokens,
    Treatments,
}

impl Section {
    pub fn label(self) -> &'static str {
        match self {
            Section::Components => "Components",
            Section::Tokens => "Tokens",
            Section::Treatments => "Treatments",
        }
    }
}

/// Available theme presets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemePreset {
    Graphite,
    Eclipse,
    Iceberg,
    Midnight,
    Nord,
    Rose,
    Forest,
    Solarized,
    Hornet,
    Cobalt,
    Clay,
    Meadow,
    Default,
}

impl ThemePreset {
    /// Order matches Svelte preview: eclipse, iceberg, graphite.
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

    pub fn label(self) -> &'static str {
        match self {
            ThemePreset::Default => "default",
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

    pub fn build_theme(self) -> GpuiThemeProvider {
        match self {
            ThemePreset::Default => GpuiThemeProvider::new(),
            ThemePreset::Eclipse => {
                GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ECLIPSE)
            }
            ThemePreset::Iceberg => {
                GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ICEBERG)
            }
            ThemePreset::Graphite => {
                GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::GRAPHITE)
            }
            ThemePreset::Midnight => {
                GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::MIDNIGHT)
            }
            ThemePreset::Nord => GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::NORD),
            ThemePreset::Rose => GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ROSE),
            ThemePreset::Forest => {
                GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::FOREST)
            }
            ThemePreset::Solarized => {
                GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::SOLARIZED)
            }
            ThemePreset::Hornet => {
                GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::HORNET)
            }
            ThemePreset::Cobalt => {
                GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::COBALT)
            }
            ThemePreset::Clay => GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::CLAY),
            ThemePreset::Meadow => {
                GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::MEADOW)
            }
        }
    }
}

/// Swatch options for the header theme picker.
///
/// Each preset's swatch is resolved from that preset's own tokens rather than
/// hardcoded hex, so a token change cannot leave the picker previewing a colour
/// the theme no longer uses. Built once — resolving twelve themes per frame
/// would be pure waste.
fn build_theme_options() -> Vec<poodle_specs::ThemeOption> {
    use poodle_adapter::ThemeProvider;
    fn hex(c: poodle_tokens::typed::ColorValue) -> String {
        let ch = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
        format!("#{:02x}{:02x}{:02x}", ch(c.0), ch(c.1), ch(c.2))
    }
    ThemePreset::ALL
        .iter()
        .map(|preset| {
            let t = preset.build_theme();
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

/// Density mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Density {
    Compact,
    Default,
    Comfortable,
}

impl Density {
    /// Order matches Svelte preview: compact, default, comfortable.
    pub const ALL: &[Density] = &[Density::Compact, Density::Default, Density::Comfortable];

    pub fn label(self) -> &'static str {
        match self {
            Density::Compact => "compact",
            Density::Default => "default",
            Density::Comfortable => "comfortable",
        }
    }

    /// Return the token density definition for this variant.
    pub fn token_definition(self) -> &'static poodle_tokens::density::DensityDefinition {
        match self {
            Density::Compact => &poodle_tokens::density::COMPACT,
            Density::Default => &poodle_tokens::density::DEFAULT,
            Density::Comfortable => &poodle_tokens::density::COMFORTABLE,
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
    /// Order matches Svelte preview: xs, sm, md, lg, xl.
    pub const ALL: &[ControlSize] = &[
        ControlSize::Xs,
        ControlSize::Sm,
        ControlSize::Md,
        ControlSize::Lg,
        ControlSize::Xl,
    ];

    pub fn label(self) -> &'static str {
        match self {
            ControlSize::Xs => "xs",
            ControlSize::Sm => "sm",
            ControlSize::Md => "md",
            ControlSize::Lg => "lg",
            ControlSize::Xl => "xl",
        }
    }

    /// Return the token control-size definition for this variant.
    pub fn token_definition(self) -> &'static poodle_tokens::density::ControlSizeDefinition {
        match self {
            ControlSize::Xs => &poodle_tokens::density::CONTROL_SIZE_XS,
            ControlSize::Sm => &poodle_tokens::density::CONTROL_SIZE_SM,
            ControlSize::Md => &poodle_tokens::density::CONTROL_SIZE_MD,
            ControlSize::Lg => &poodle_tokens::density::CONTROL_SIZE_LG,
            ControlSize::Xl => &poodle_tokens::density::CONTROL_SIZE_XL,
        }
    }
}

/// The neutral-contrast axis is continuous, like the web preview's range input
/// and the Jetstream shell's slider. It used to be four preset stops behind a
/// toggle group here, which could not express the values between them.
pub const CONTRAST_MIN: f32 = 0.0;
pub const CONTRAST_MAX: f32 = 1.0;
/// Where the preview starts. Lower than the tokens' own midpoint: the flatter
/// neutral ramp is the one most of the component work is judged against.
pub const CONTRAST_DEFAULT: f32 = 0.25;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenPanel {
    Summary,
    Inspector,
}

impl TokenPanel {
    pub fn label(self) -> &'static str {
        match self {
            TokenPanel::Summary => "Runtime values",
            TokenPanel::Inspector => "Inspector",
        }
    }

    pub fn value(self) -> &'static str {
        match self {
            TokenPanel::Summary => "token-summary-section",
            TokenPanel::Inspector => "token-inspector",
        }
    }
}

/// Generic specimen interaction state.
/// Keyed by specimen-scoped string IDs so each specimen can store
/// toggles, selections, and counters without dedicated struct fields.
pub struct SpecimenState {
    pub toggles: HashMap<String, bool>,
    pub selections: HashMap<String, usize>,
    pub counters: HashMap<String, u32>,
    pub text: HashMap<String, String>,
    /// Caret/selection per text specimen key. The Rust targets have no native
    /// editor, so the caret is host state exactly like the value is — a field
    /// whose value is stored but whose caret is not inserts every keystroke at
    /// index 0, which spells typed text backwards.
    pub carets: HashMap<String, (usize, usize)>,
}

impl SpecimenState {
    pub fn new() -> Self {
        // Initialize default toggle values matching Svelte specimen defaults
        let mut toggles = HashMap::new();
        toggles.insert("switch-dark-mode".to_string(), true);
        toggles.insert("switch-compact".to_string(), true);
        toggles.insert("checkbox-email".to_string(), true);
        // Accordion defaults matching Svelte
        toggles.insert("accordion-single-getting-started".to_string(), true);
        toggles.insert("accordion-multi-design-tokens".to_string(), true);
        toggles.insert("accordion-multi-keyboard-shortcuts".to_string(), true);
        // IconButton pin default
        toggles.insert("icon-btn-pinned".to_string(), true);
        Self {
            toggles,
            selections: HashMap::new(),
            counters: HashMap::new(),
            text: HashMap::new(),
            carets: HashMap::new(),
        }
    }

    pub fn toggle(&mut self, key: &str) -> bool {
        let val = self.toggles.entry(key.to_string()).or_insert(false);
        *val = !*val;
        *val
    }

    pub fn set_toggle(&mut self, key: &str, value: bool) {
        self.toggles.insert(key.to_string(), value);
    }

    pub fn is_on(&self, key: &str) -> bool {
        self.toggles.get(key).copied().unwrap_or(false)
    }

    #[allow(dead_code)]
    pub fn selected(&self, key: &str) -> usize {
        self.selections.get(key).copied().unwrap_or(0)
    }

    #[allow(dead_code)]
    pub fn select(&mut self, key: &str, idx: usize) {
        self.selections.insert(key.to_string(), idx);
    }

    pub fn count(&self, key: &str) -> u32 {
        self.counters.get(key).copied().unwrap_or(0)
    }

    pub fn increment(&mut self, key: &str) {
        let val = self.counters.entry(key.to_string()).or_insert(0);
        *val += 1;
    }
}

/// Interactive state for the Tree specimen (expansion, selection, keyboard focus).
pub struct TreePreviewState {
    pub expanded: Vec<String>,
    pub selected: Vec<String>,
    pub focused: Option<String>,
    pub checked: Vec<String>,
    pub selection_anchor: Option<String>,
    pub editing_value: Option<String>,
    pub editing_text: String,
    /// Mutable demo tree for the rename / menu / reorder specimen.
    pub rename_nodes: Vec<TreeNode>,
    pub menu_value: Option<String>,
    pub menu_pos: (i32, i32),
    /// Live drag drop-target indicator state (rename/reorder tree).
    pub drop_target: Option<String>,
    pub drop_position: DropPosition,
}

impl TreePreviewState {
    pub fn new() -> Self {
        Self {
            expanded: vec!["src".to_string(), "src/components".to_string()],
            selected: vec!["src/components/Tree.svelte".to_string()],
            focused: Some("src/components/Tree.svelte".to_string()),
            checked: vec!["src/components/Button.svelte".to_string()],
            selection_anchor: Some("src/components/Tree.svelte".to_string()),
            editing_value: None,
            editing_text: String::new(),
            rename_nodes: docs_tree(),
            menu_value: None,
            menu_pos: (0, 0),
            drop_target: None,
            drop_position: DropPosition::After,
        }
    }

    /// Apply a multi-select update (replace / toggle / range) from the Tree.
    pub fn apply_selection(&mut self, values: Vec<String>, anchor: Option<String>, focused: &str) {
        self.selected = values;
        self.selection_anchor = anchor;
        self.focused = Some(focused.to_string());
    }
    pub fn set_drop(&mut self, target: &str, position: DropPosition) {
        self.drop_target = Some(target.to_string());
        self.drop_position = position;
    }
    pub fn clear_drop(&mut self) {
        self.drop_target = None;
    }

    pub fn start_rename(&mut self, value: &str, label: &str) {
        self.editing_value = Some(value.to_string());
        self.editing_text = label.to_string();
    }
    pub fn delete_node(&mut self, value: &str) {
        self.rename_nodes = remove_node(std::mem::take(&mut self.rename_nodes), value);
    }
    pub fn reorder(&mut self, from: &str, to: &str, position: DropPosition) {
        self.rename_nodes = reorder_nodes(&self.rename_nodes, from, to, position);
    }
    pub fn open_menu(&mut self, value: &str, x: i32, y: i32) {
        self.menu_value = Some(value.to_string());
        self.menu_pos = (x, y);
    }
    pub fn close_menu(&mut self) {
        self.menu_value = None;
    }

    /// Cascade-toggle a set of checkable leaf values: uncheck all when all are
    /// checked, otherwise check all.
    pub fn toggle_checked(&mut self, leaves: &[String]) {
        let all_on = leaves.iter().all(|v| self.checked.contains(v));
        if all_on {
            self.checked.retain(|v| !leaves.contains(v));
        } else {
            for v in leaves {
                if !self.checked.contains(v) {
                    self.checked.push(v.clone());
                }
            }
        }
    }

    pub fn toggle_expanded(&mut self, value: &str) {
        if let Some(i) = self.expanded.iter().position(|v| v == value) {
            self.expanded.remove(i);
        } else {
            self.expanded.push(value.to_string());
        }
    }

    pub fn set_focused(&mut self, value: &str) {
        self.focused = Some(value.to_string());
    }
}

/// An interaction a node-backed specimen reported through a context-free
/// handler. Node interaction closures are `Arc<dyn Fn() + Send + Sync>` — no
/// `&mut App` — so a specimen handler records intent here and the next render
/// drains the queue into real specimen state (the backend requests the
/// repaint after invoking a handler).
pub enum NodeSpecimenEvent {
    /// Toggle a boolean specimen key (e.g. `select-native-open`).
    Toggle(String),
    /// Set a boolean specimen key to a specific value (e.g. opening a dialog
    /// whose trigger must remain idempotent).
    SetToggle { key: String, value: bool },
    /// Set a text specimen key and close the owning overlay
    /// (e.g. select's change: record the value, close the panel).
    Change {
        open_key: String,
        value_key: String,
        value: String,
    },
    /// Set a text specimen key without touching any overlay state
    /// (e.g. segmented-control change, button "last clicked" captions).
    SetText { key: String, value: String },
    /// Set or clear an optional text specimen key (e.g. either endpoint of a
    /// partially selected calendar range).
    SetOptionalText { key: String, value: Option<String> },
    /// Move a field's caret or selection, as character indices into its value.
    SetCaret {
        key: String,
        start: usize,
        end: usize,
    },
    /// Set a selection index (e.g. a tri-state control's chosen segment).
    Select { key: String, index: usize },
    /// Increment a counter specimen key (e.g. `btn-clicks`).
    Increment(String),
    /// Update the DataTable sort state using the component's host-owned
    /// sort-cycle contract.
    DataTableSort { column: String },
    /// Toggle a DataTable row and record the visible row action caption.
    DataTableRowClick { row_id: String },
    /// Apply a context-menu action to the mutable tree demo, then close it.
    TreeContextAction {
        action: String,
        value: String,
        label: String,
    },
    /// A preview-chrome control (the shell's own nav, search and panel tabs,
    /// not a specimen) reporting through the same context-free seam. These
    /// mutate `AppState` directly rather than specimen state.
    Chrome(ChromeEvent),
    /// A Tree interaction. Tree drives more host state than any other
    /// specimen — selection, focus, expansion, rename, drag and menu — so it
    /// gets its own event rather than a dozen flat variants.
    Tree(TreeEvent),
}

/// State changes the node-backed Tree specimen can request.
#[derive(Clone, Debug)]
pub enum TreeEvent {
    Focus(String),
    ToggleExpand(String),
    /// A resolved multi-selection, already computed by the specimen through
    /// the shared `compute_selection`.
    Select {
        values: Vec<String>,
        anchor: Option<String>,
        focused: String,
    },
    /// Toggle every checkable leaf under the clicked row.
    Check(Vec<String>),
    RenameStart {
        value: String,
        label: String,
    },
    OpenMenu {
        value: String,
        x: i32,
        y: i32,
    },
    SetDrop {
        value: String,
        position: DropPosition,
    },
    Reorder {
        from: String,
        to: String,
        position: DropPosition,
    },
}

/// State changes the preview shell's own node-backed controls can request.
#[derive(Clone, Debug)]
pub enum ChromeEvent {
    Section(Section),
    /// The theme picker's trigger reported the open state it is moving to.
    ThemeSelectOpen(bool),
    /// Neutral-contrast slider moved; clamped and applied to the theme.
    Contrast(f32),
    /// A theme swatch was chosen; rebuilds the theme provider and closes.
    Theme(ThemePreset),
    ComponentSearch(String),
    /// Caret/selection moved in the header search box.
    SearchSelection(usize, usize),
    SearchFocused(bool),
    ActiveComponent(String),
    TokenPanel(TokenPanel),
    TokenInspectorQuery(String),
}

/// Global application state.
pub struct AppState {
    pub section: Section,
    pub theme: GpuiThemeProvider,
    pub theme_preset: ThemePreset,
    pub density: Density,
    pub control_size: ControlSize,
    /// Neutral-contrast knob, 0.0..=1.0. Starts at `CONTRAST_DEFAULT`, which is
    /// the preview's starting point rather than the tokens' own midpoint (0.5).
    pub contrast: f32,
    pub component_search: String,
    /// Caret/selection in the header search box, and whether it holds focus.
    /// The Rust targets have no native editor, so the host owns the cursor.
    pub search_selection: (usize, usize),
    pub search_focused: bool,
    /// The header theme picker's popover state; `ThemeSelectSpec::is_open` is
    /// controlled, so the host owns it.
    pub theme_select_open: bool,
    /// Swatch options for the header theme picker, resolved once from each
    /// preset's own tokens rather than hardcoded hex.
    pub theme_options: Vec<poodle_specs::ThemeOption>,
    pub active_component_slug: Option<String>,
    pub active_token_panel: TokenPanel,
    pub token_inspector_query: String,
    #[allow(dead_code)]
    pub debug_clicks: u32,
    pub specimens: SpecimenState,
    /// Pending events from node-backed specimens; drained at render start.
    pub node_events: std::sync::Arc<std::sync::Mutex<Vec<NodeSpecimenEvent>>>,
    pub tree: TreePreviewState,
}

impl AppState {
    pub fn new() -> Self {
        let preset = ThemePreset::Eclipse;
        let density = Density::Compact;
        let control_size = ControlSize::Sm;

        // Build theme with density + control-size layered on top
        let theme = preset
            .build_theme()
            .with_density(density.token_definition())
            .with_control_size(control_size.token_definition());

        Self {
            section: Section::Components,
            theme,
            theme_preset: preset,
            density,
            control_size,
            contrast: CONTRAST_DEFAULT,
            component_search: String::new(),
            search_selection: (0, 0),
            search_focused: false,
            theme_select_open: false,
            theme_options: build_theme_options(),
            active_component_slug: None,
            active_token_panel: TokenPanel::Summary,
            token_inspector_query: String::new(),
            debug_clicks: 0,
            specimens: SpecimenState::new(),
            node_events: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
            tree: TreePreviewState::new(),
        }
    }

    /// Apply queued node-specimen events to the specimen state. Called at the
    /// top of every render so handler-triggered changes take effect in the
    /// frame the backend's repaint request produces.
    pub fn drain_node_events(&mut self) {
        let events: Vec<NodeSpecimenEvent> = std::mem::take(&mut *self.node_events.lock().unwrap());
        for event in events {
            match event {
                NodeSpecimenEvent::Toggle(key) => {
                    self.specimens.toggle(&key);
                }
                NodeSpecimenEvent::SetToggle { key, value } => {
                    self.specimens.set_toggle(&key, value);
                }
                NodeSpecimenEvent::Change {
                    open_key,
                    value_key,
                    value,
                } => {
                    self.specimens.text.insert(value_key, value);
                    self.specimens.toggles.insert(open_key, false);
                }
                NodeSpecimenEvent::SetText { key, value } => {
                    // A replaced value can be shorter than the old caret.
                    let len = value.chars().count();
                    if let Some(caret) = self.specimens.carets.get_mut(&key) {
                        caret.0 = caret.0.min(len);
                        caret.1 = caret.1.min(len);
                    }
                    self.specimens.text.insert(key, value);
                }
                NodeSpecimenEvent::SetCaret { key, start, end } => {
                    self.specimens.carets.insert(key, (start, end));
                }
                NodeSpecimenEvent::SetOptionalText { key, value } => match value {
                    Some(value) => {
                        self.specimens.text.insert(key, value);
                    }
                    None => {
                        self.specimens.text.remove(&key);
                    }
                },
                NodeSpecimenEvent::Select { key, index } => {
                    self.specimens.select(&key, index);
                }
                NodeSpecimenEvent::Increment(key) => {
                    self.specimens.increment(&key);
                }
                NodeSpecimenEvent::DataTableSort { column } => {
                    let current_col = self
                        .specimens
                        .text
                        .get("dt-sort-col")
                        .cloned()
                        .unwrap_or_else(|| "name".to_string());
                    let current_dir = self
                        .specimens
                        .text
                        .get("dt-sort-dir")
                        .cloned()
                        .unwrap_or_else(|| "asc".to_string());
                    if column == current_col {
                        let next_dir = if current_dir == "asc" { "desc" } else { "asc" };
                        self.specimens
                            .text
                            .insert("dt-sort-dir".to_string(), next_dir.to_string());
                    } else {
                        self.specimens
                            .text
                            .insert("dt-sort-col".to_string(), column);
                        self.specimens
                            .text
                            .insert("dt-sort-dir".to_string(), "asc".to_string());
                    }
                }
                NodeSpecimenEvent::DataTableRowClick { row_id } => {
                    self.specimens.toggle(&format!("dt-row-{row_id}"));
                    self.specimens.text.insert(
                        "dt-last-action".to_string(),
                        format!("Clicked row {row_id}"),
                    );
                }
                NodeSpecimenEvent::TreeContextAction {
                    action,
                    value,
                    label,
                } => {
                    match action.as_str() {
                        "rename" => self.tree.start_rename(&value, &label),
                        "delete" => self.tree.delete_node(&value),
                        _ => {}
                    }
                    self.tree.close_menu();
                }
                NodeSpecimenEvent::Tree(event) => match event {
                    TreeEvent::Focus(value) => self.tree.set_focused(&value),
                    TreeEvent::ToggleExpand(value) => self.tree.toggle_expanded(&value),
                    TreeEvent::Select {
                        values,
                        anchor,
                        focused,
                    } => self.tree.apply_selection(values, anchor, &focused),
                    TreeEvent::Check(leaves) => self.tree.toggle_checked(&leaves),
                    TreeEvent::RenameStart { value, label } => {
                        self.tree.start_rename(&value, &label)
                    }
                    TreeEvent::OpenMenu { value, x, y } => self.tree.open_menu(&value, x, y),
                    TreeEvent::SetDrop { value, position } => {
                        self.tree.set_drop(&value, position)
                    }
                    TreeEvent::Reorder {
                        from,
                        to,
                        position,
                    } => {
                        self.tree.reorder(&from, &to, position);
                        self.tree.clear_drop();
                    }
                },
                NodeSpecimenEvent::Chrome(event) => match event {
                    ChromeEvent::Section(section) => self.section = section,
                    ChromeEvent::ThemeSelectOpen(open) => self.theme_select_open = open,
                    ChromeEvent::Contrast(value) => {
                        self.contrast = value.clamp(CONTRAST_MIN, CONTRAST_MAX);
                        self.rebuild_theme();
                    }
                    ChromeEvent::Theme(preset) => {
                        self.set_theme(preset);
                        self.theme_select_open = false;
                    }
                    ChromeEvent::ComponentSearch(query) => {
                        // Keep the caret inside a value the host just changed.
                        let len = query.chars().count();
                        self.component_search = query;
                        self.search_selection.0 = self.search_selection.0.min(len);
                        self.search_selection.1 = self.search_selection.1.min(len);
                    }
                    ChromeEvent::SearchSelection(start, end) => {
                        self.search_selection = (start, end);
                    }
                    ChromeEvent::SearchFocused(focused) => self.search_focused = focused,
                    ChromeEvent::ActiveComponent(slug) => {
                        self.active_component_slug = Some(slug);
                    }
                    ChromeEvent::TokenPanel(panel) => self.active_token_panel = panel,
                    ChromeEvent::TokenInspectorQuery(query) => {
                        self.token_inspector_query = query;
                    }
                },
            }
        }
    }

    pub fn set_theme(&mut self, preset: ThemePreset) {
        self.theme_preset = preset;
        self.rebuild_theme();
    }

    /// Rebuild the theme provider from the current preset, density, and control size.
    ///
    /// Layering order: base theme first, then density overrides, then control-size
    /// overrides. Later overrides win for conflicting tokens (e.g. control height).
    pub fn rebuild_theme(&mut self) {
        let mut theme = self.theme_preset.build_theme();
        theme = theme.with_density(self.density.token_definition());
        theme = theme.with_control_size(self.control_size.token_definition());
        theme = theme.with_contrast(self.contrast);
        self.theme = theme;
    }
}
