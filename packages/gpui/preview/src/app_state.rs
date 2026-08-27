//! Application state for the preview app.
//!
//! Mirrors the current Svelte preview shell: theme, density, control size,
//! component search, active section, and component selection.

use gpui::App;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_node_backend::file_capability::{FilePickOutcome, SingleFilePickSpec};
use poodle_headless::licence::LicenceSeat;
use poodle_headless::model_connection::{
    model_catalogue_fixtures, model_connection_picker_fixtures, ModelCatalogueItem,
    ModelConnectionSetupStage,
};
use poodle_specs::{reorder_nodes, DropPosition, TreeNode};
use std::collections::HashMap;

/// Land one resolved pick outcome in specimen state under its key prefix.
/// Returns whether any specimen key changed.
fn apply_file_pick_outcome(
    specimens: &mut SpecimenState,
    key: &str,
    outcome: &FilePickOutcome,
    failed_message: Option<&str>,
) -> bool {
    let mut changed = false;
    match outcome {
        FilePickOutcome::Selected {
            name,
            contents_base64,
        } => {
            let name_key = format!("{key}-name");
            let base64_key = format!("{key}-base64");
            let error_key = format!("{key}-error");
            changed |= specimens.text.get(&base64_key) != Some(contents_base64);
            changed |= specimens.text.get(&name_key) != Some(name);
            specimens.text.insert(name_key, name.clone());
            specimens.text.insert(base64_key, contents_base64.clone());
            changed |= specimens.text.remove(&error_key).is_some();
        }
        FilePickOutcome::Cancelled => {}
        FilePickOutcome::Rejected(message) => {
            // Honest accept/size copy is preserved verbatim.
            let error_key = format!("{key}-error");
            let base64_key = format!("{key}-base64");
            changed |= specimens.text.get(&error_key) != Some(message);
            specimens.text.insert(error_key, message.clone());
            changed |= specimens.text.remove(&base64_key).is_some();
        }
        FilePickOutcome::Failed(message) => {
            // A read failure is a local polite error on the component surface
            // (the raw OS text never reaches the operator); the capability
            // outcome stays honest, the visible copy is the approved message.
            let visible = failed_message.unwrap_or(message).to_string();
            let error_key = format!("{key}-error");
            let base64_key = format!("{key}-base64");
            changed |= specimens.text.get(&error_key) != Some(&visible);
            specimens.text.insert(error_key, visible);
            changed |= specimens.text.remove(&base64_key).is_some();
        }
    }
    changed
}

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
}

impl Section {
    pub fn label(self) -> &'static str {
        match self {
            Section::Components => "Components",
            Section::Tokens => "Tokens",
        }
    }
}

/// Available theme presets and control sizes live in `presentation_axes` —
/// the one domain authority shared with the offscreen capture target
/// (g15.045). Re-exported here so existing `app_state::ThemePreset` /
/// `app_state::ControlSize` paths keep resolving.
pub use crate::presentation_axes::{ControlSize, ThemePreset};

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

/// Control size lives in `presentation_axes` (see the re-export above).

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
    /// Hours/minutes/seconds per DurationInput specimen key. The formatted
    /// display is derived; the host stores the three segments.
    pub durations: HashMap<String, (u32, u32, u32)>,
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
        // Keep the MessageCenter surface visible in visual captures; the
        // trigger can still close and reopen it through the host event queue.
        toggles.insert("message-center-open".to_string(), true);
        Self {
            toggles,
            selections: HashMap::new(),
            counters: HashMap::new(),
            text: HashMap::new(),
            carets: HashMap::new(),
            durations: HashMap::new(),
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

    pub fn selected(&self, key: &str) -> usize {
        self.selections.get(key).copied().unwrap_or(0)
    }

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
    /// Write a DurationInput's three segment values directly.
    SetDuration {
        key: String,
        hours: u32,
        minutes: u32,
        seconds: u32,
    },
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
    /// The generic single-file browse seam reported a request (g15.007). The
    /// host opens the OS prompt on its next frame and awaits the result; the
    /// outcome lands in the specimen keys below.
    FileBrowse {
        key: String,
        spec: SingleFilePickSpec,
        /// The polite message to surface for a read failure on this component
        /// surface (e.g. LicenceActivation's "That file could not be read.").
        /// The generic capability outcome stays honest; only the visible copy
        /// is remapped.
        failed_message: Option<String>,
    },
    /// A route/mode change invalidated a selected or pending file read: the
    /// completed bytes are cleared and in-flight/pending outcomes are made
    /// stale by generation so they can never land.
    FileInvalidate,
    /// The machine-name edit was cancelled (Escape): the committed value
    /// snapped at edit start is restored and editing closes.
    MachineLabelCancel,
    /// A LicenceSeats interaction.
    LicenceSeats(LicenceSeatsEvent),
    /// A model-connection family interaction (picker, setup, card, catalogue).
    ModelConnection(ModelConnectionEvent),
}

/// State changes the model-connection specimens can request. One enum for the
/// four components because they share one host-state struct, exactly as the
/// Swallowtail route that will consume them would.
#[derive(Clone, Debug)]
pub enum ModelConnectionEvent {
    PickerValue(String),
    PickerQuery(String),
    SetupStage(ModelConnectionSetupStage),
    SetupValue(String),
    SetupQuery(String),
    SetupSubmit(String),
    SetupCancel,
    CardOpen { id: String, open: bool },
    CardEnabled { id: String, enabled: bool },
    /// The complete shown-id order the editor asked for.
    CatalogueOrder(Vec<String>),
    CatalogueVisibility { id: String, visible: bool },
    CatalogueGrab(Option<String>),
    CatalogueDropTarget(Option<String>),
    CatalogueHiddenOpen(bool),
    CatalogueAnnounce(String),
    CatalogueInfo(String),
    /// A component named an element id it wants focused. The backend performs
    /// the move; the component never touches focus itself.
    FocusRequest(String),
}

/// Host state for the model-connection specimens. Poodle owns none of this:
/// the preview plays the part Nucleus will, holding the current values and
/// applying every requested change itself.
pub struct ModelConnectionPreviewState {
    pub picker_value: Option<String>,
    pub picker_query: String,
    pub setup_stage: ModelConnectionSetupStage,
    pub setup_value: Option<String>,
    pub setup_query: String,
    /// The last submit or cancel the workflow reported, shown as safe copy.
    pub setup_outcome: Option<String>,
    /// Disclosure and enable preference per configured-connection id.
    pub card_open: HashMap<String, bool>,
    pub card_enabled: HashMap<String, bool>,
    pub catalogue_items: Vec<ModelCatalogueItem>,
    pub grabbed_id: Option<String>,
    pub drop_target_id: Option<String>,
    pub hidden_open: bool,
    pub live_message: String,
    /// The last info request, so the optional action proves it reaches a host.
    pub info_id: Option<String>,
}

impl ModelConnectionPreviewState {
    pub fn new() -> Self {
        Self {
            picker_value: None,
            picker_query: String::new(),
            setup_stage: ModelConnectionSetupStage::Choose,
            setup_value: None,
            setup_query: String::new(),
            setup_outcome: None,
            card_open: HashMap::new(),
            card_enabled: HashMap::new(),
            catalogue_items: model_catalogue_fixtures(),
            grabbed_id: None,
            drop_target_id: None,
            hidden_open: false,
            live_message: String::new(),
            info_id: None,
        }
    }

    pub fn options() -> Vec<poodle_headless::model_connection::ModelConnectionOption> {
        model_connection_picker_fixtures()
    }

    /// A specimen that seeds its disclosure open passes `default: true`. The
    /// host map still owns the value once the reader toggles it, so the live
    /// control keeps working in both directions.
    pub fn card_is_open(&self, id: &str, default: bool) -> bool {
        self.card_open.get(id).copied().unwrap_or(default)
    }

    pub fn card_is_enabled(&self, id: &str, default: bool) -> bool {
        self.card_enabled.get(id).copied().unwrap_or(default)
    }

    /// Apply a requested shown order. Hidden entries keep their own slots, so
    /// reordering the shown models never disturbs what is hidden.
    fn apply_order(&mut self, order: &[String]) {
        let mut shown = order.iter();
        let mut next = Vec::with_capacity(self.catalogue_items.len());
        for item in &self.catalogue_items {
            if !item.visible {
                next.push(item.clone());
                continue;
            }
            let Some(id) = shown.next() else { continue };
            let Some(moved) = self.catalogue_items.iter().find(|entry| &entry.id == id) else {
                continue;
            };
            next.push(moved.clone());
        }
        self.catalogue_items = next;
    }

    fn apply_visibility(&mut self, id: &str, visible: bool) {
        for item in &mut self.catalogue_items {
            if item.id == id {
                item.visible = visible;
            }
        }
    }
}

impl Default for ModelConnectionPreviewState {
    fn default() -> Self {
        Self::new()
    }
}

/// One requested single-file pick, waiting for the host to open its prompt.
#[derive(Clone)]
pub struct FilePickRequest {
    /// Specimen-state key prefix; the resolved outcome lands in
    /// `{key}-name`, `{key}-base64`, and `{key}-error`.
    pub key: String,
    pub spec: SingleFilePickSpec,
    /// Polite copy for a read failure, surfaced instead of the raw OS text.
    pub failed_message: Option<String>,
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

/// Host state for the LicenceSeats specimen: the authority-reported seats
/// and the controlled per-row confirm/edit open state.
pub struct LicencePreviewState {
    pub seats: Vec<LicenceSeat>,
    pub editing_machine_id: Option<String>,
    pub open_confirm_machine_id: Option<String>,
}

impl LicencePreviewState {
    /// The web specimen's "mixed labels" set, kept label-only in render.
    pub fn mixed() -> Self {
        Self {
            seats: vec![
                LicenceSeat {
                    machine_id: "cmd-9f3a2b7c".to_string(),
                    label: Some("Studio Mac".to_string()),
                    this_machine: true,
                },
                LicenceSeat {
                    machine_id: "cmd-41ee80d2".to_string(),
                    label: Some("Tour laptop".to_string()),
                    this_machine: false,
                },
                LicenceSeat {
                    machine_id: "cmd-77c1a5be".to_string(),
                    label: None,
                    this_machine: false,
                },
            ],
            editing_machine_id: None,
            open_confirm_machine_id: None,
        }
    }

    pub fn rename(&mut self, machine_id: &str, label: Option<String>) {
        if let Some(seat) = self.seats.iter_mut().find(|seat| seat.machine_id == machine_id) {
            seat.label = label;
        }
        self.editing_machine_id = None;
    }
}

/// State changes the LicenceSeats specimen can request.
#[derive(Clone, Debug)]
pub enum LicenceSeatsEvent {
    Rename { machine_id: String, label: Option<String> },
    Edit { machine_id: String },
    ReleaseTrigger { machine_id: String },
    ReleaseConfirm { machine_id: String },
    /// No payload: cancellation applies to whatever row is open.
    ReleaseCancel,
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
    pub specimens: SpecimenState,
    /// Pending events from node-backed specimens; drained at render start.
    pub node_events: std::sync::Arc<std::sync::Mutex<Vec<NodeSpecimenEvent>>>,
    pub tree: TreePreviewState,
    /// LicenceSeats specimen host state.
    pub licence_seats: LicencePreviewState,
    /// Model-connection family specimen host state.
    pub model_connection: ModelConnectionPreviewState,
    /// GPUI-owned offset and pin posture for the AgentTranscript specimen's
    /// real bounded viewport.
    pub agent_transcript_scroll: poodle_gpui_node_backend::TrackedScrollState,
    /// Single-file picks requested through the generic browse seam whose OS
    /// prompt has not been opened yet.
    pub pending_file_picks: Vec<FilePickRequest>,
    /// Every key with a pick whose prompt was opened (completed or in
    /// flight). An invalidation clears these keys' specimen state.
    pub active_file_keys: Vec<String>,
    /// Bumped whenever a route/mode change invalidates a file read. A pick
    /// task captures the generation at spawn and only lands its outcome while
    /// it still matches — a stale result can never repopulate a route the
    /// operator left.
    pub file_generation: u64,
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
            specimens: SpecimenState::new(),
            node_events: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
            tree: TreePreviewState::new(),
            licence_seats: LicencePreviewState::mixed(),
            model_connection: ModelConnectionPreviewState::new(),
            agent_transcript_scroll: poodle_gpui_node_backend::TrackedScrollState::new(),
            pending_file_picks: Vec::new(),
            active_file_keys: Vec::new(),
            file_generation: 0,
        }
    }

    /// Apply queued node-specimen events to the specimen state. Called at the
    /// top of every render so handler-triggered changes take effect in the
    /// frame the backend's repaint request produces. Returns whether specimen
    /// content changed and its virtualized page measurements must be reset.
    pub fn drain_node_events(&mut self) -> bool {
        let events: Vec<NodeSpecimenEvent> = std::mem::take(&mut *self.node_events.lock().unwrap());
        let mut specimen_changed = false;
        for event in events {
            specimen_changed |= !matches!(&event, NodeSpecimenEvent::Chrome(_));
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
                NodeSpecimenEvent::SetDuration {
                    key,
                    hours,
                    minutes,
                    seconds,
                } => {
                    self.specimens
                        .durations
                        .insert(key, (hours, minutes, seconds));
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
                NodeSpecimenEvent::FileBrowse {
                    key,
                    spec,
                    failed_message,
                } => {
                    // The OS prompt opens on the next frame via
                    // `start_file_picks`, which owns the app context.
                    self.pending_file_picks.push(FilePickRequest {
                        key,
                        spec,
                        failed_message,
                    });
                }
                NodeSpecimenEvent::FileInvalidate => {
                    self.invalidate_file_picks();
                }
                NodeSpecimenEvent::MachineLabelCancel => {
                    // Escape restores the value snapped when editing started
                    // (the web EditableLabel reverts the draft) and closes
                    // editing.
                    let original = self
                        .specimens
                        .text
                        .remove("la-machine-label-original")
                        .unwrap_or_default();
                    self.specimens
                        .text
                        .insert("la-machine-label".to_string(), original);
                    self.specimens
                        .toggles
                        .insert("la-machine-editing".to_string(), false);
                }
                NodeSpecimenEvent::LicenceSeats(event) => match event {
                    LicenceSeatsEvent::Rename { machine_id, label } => {
                        self.licence_seats.rename(&machine_id, label);
                    }
                    LicenceSeatsEvent::Edit { machine_id } => {
                        self.licence_seats.editing_machine_id = Some(machine_id);
                    }
                    LicenceSeatsEvent::ReleaseTrigger { machine_id } => {
                        self.licence_seats.open_confirm_machine_id = Some(machine_id);
                    }
                    LicenceSeatsEvent::ReleaseConfirm { machine_id } => {
                        self.licence_seats.seats.retain(|seat| seat.machine_id != machine_id);
                        self.licence_seats.open_confirm_machine_id = None;
                    }
                    LicenceSeatsEvent::ReleaseCancel => {
                        self.licence_seats.open_confirm_machine_id = None;
                    }
                },
                NodeSpecimenEvent::ModelConnection(event) => {
                    let state = &mut self.model_connection;
                    match event {
                        ModelConnectionEvent::PickerValue(id) => state.picker_value = Some(id),
                        ModelConnectionEvent::PickerQuery(query) => state.picker_query = query,
                        ModelConnectionEvent::SetupStage(stage) => state.setup_stage = stage,
                        ModelConnectionEvent::SetupValue(id) => state.setup_value = Some(id),
                        ModelConnectionEvent::SetupQuery(query) => state.setup_query = query,
                        ModelConnectionEvent::SetupSubmit(id) => {
                            state.setup_outcome = Some(format!("Requested {id}"));
                        }
                        ModelConnectionEvent::SetupCancel => {
                            state.setup_stage = ModelConnectionSetupStage::Choose;
                            state.setup_value = None;
                            state.setup_outcome = Some("Cancelled".to_string());
                        }
                        ModelConnectionEvent::CardOpen { id, open } => {
                            state.card_open.insert(id, open);
                        }
                        ModelConnectionEvent::CardEnabled { id, enabled } => {
                            state.card_enabled.insert(id, enabled);
                        }
                        ModelConnectionEvent::CatalogueOrder(order) => state.apply_order(&order),
                        ModelConnectionEvent::CatalogueVisibility { id, visible } => {
                            state.apply_visibility(&id, visible);
                        }
                        ModelConnectionEvent::CatalogueGrab(id) => state.grabbed_id = id,
                        ModelConnectionEvent::CatalogueDropTarget(id) => {
                            state.drop_target_id = id;
                        }
                        ModelConnectionEvent::CatalogueHiddenOpen(open) => {
                            state.hidden_open = open;
                        }
                        ModelConnectionEvent::CatalogueAnnounce(message) => {
                            state.live_message = message;
                        }
                        ModelConnectionEvent::CatalogueInfo(id) => state.info_id = Some(id),
                        ModelConnectionEvent::FocusRequest(id) => {
                            // The backend owns the focus operation; the
                            // component only named the destination.
                            poodle_gpui_node_backend::request_focus(&id);
                        }
                    }
                }
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
        specimen_changed
    }

    pub fn set_theme(&mut self, preset: ThemePreset) {
        self.theme_preset = preset;
        self.rebuild_theme();
    }

    /// Invalidate every selected or pending file read (route/mode change).
    ///
    /// Bumps the pick generation so an in-flight OS dialog that resolves
    /// later is stale and cannot land; clears the completed bytes for every
    /// key that had a pick, and drops pending requests (they capture a stale
    /// generation when started). Mirrors the contract: returning offline
    /// requires a new file.
    pub fn invalidate_file_picks(&mut self) {
        self.file_generation += 1;
        self.pending_file_picks.clear();
        for key in std::mem::take(&mut self.active_file_keys) {
            self.specimens.text.remove(&format!("{key}-name"));
            self.specimens.text.remove(&format!("{key}-base64"));
            self.specimens.text.remove(&format!("{key}-error"));
        }
    }

    /// Open the OS prompts for every pending pick (g15.007).
    ///
    /// Each prompt's oneshot receiver is **awaited** in a GPUI task — dialog
    /// completion itself schedules the render that consumes it, so a result
    /// never waits on an unrelated repaint. The outcome runs the shared
    /// post-selection pipeline and lands in specimen state as
    /// `{key}-name` / `{key}-base64` / `{key}-error`, guarded by the
    /// generation captured at spawn: a route change after the dialog opened
    /// makes the result stale and it is dropped.
    pub fn start_file_picks(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut App,
        root: &gpui::WeakEntity<crate::PreviewRoot>,
    ) {
        let pending = std::mem::take(&mut self.pending_file_picks);
        for request in pending {
            let options = poodle_gpui_node_backend::file_capability::os_pick_options(&request.spec);
            let receiver = cx.prompt_for_paths(options);
            self.active_file_keys.push(request.key.clone());
            let generation = self.file_generation;
            let key = request.key.clone();
            let spec = request.spec.clone();
            let failed_message = request.failed_message.clone();
            let root = root.clone();
            window.spawn(cx, async move |cx| {
                deliver_os_pick(
                    &root,
                    receiver,
                    key,
                    generation,
                    spec,
                    failed_message,
                    cx,
                )
                .await;
            })
            .detach();
        }
    }

    /// Apply a resolved pick outcome, but only while it belongs to the
    /// current route. A stale generation (the operator switched away after
    /// the dialog opened) drops the outcome entirely. A read failure lands
    /// the request's polite message rather than the raw OS text.
    pub fn apply_pick_outcome(
        &mut self,
        key: &str,
        generation: u64,
        outcome: &FilePickOutcome,
        failed_message: Option<&str>,
    ) {
        if generation != self.file_generation {
            return;
        }
        apply_file_pick_outcome(&mut self.specimens, key, outcome, failed_message);
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

/// The completion-driven landing seam for one OS pick (g15.007).
///
/// Awaits the prompt's oneshot receiver — a dialog result *schedules* this
/// task, never a poll — resolves it through the shared post-selection
/// pipeline, and lands the outcome in the preview's specimen state through
/// the root entity with an explicit notify. Guarded by the generation
/// captured at spawn: a route change after the dialog opened drops the
/// result entirely. This is the exact seam `start_file_picks` runs; tests
/// drive it with an injected receiver completed after the first frame.
pub async fn deliver_os_pick(
    root: &gpui::WeakEntity<crate::PreviewRoot>,
    receiver: futures::channel::oneshot::Receiver<anyhow::Result<Option<Vec<std::path::PathBuf>>>>,
    key: String,
    generation: u64,
    spec: SingleFilePickSpec,
    failed_message: Option<String>,
    cx: &mut gpui::AsyncApp,
) {
    let selection = match receiver.await {
        Ok(selection) => selection,
        // The sender dropped without a selection: cancelled.
        Err(_) => Ok(None),
    };
    let outcome = poodle_gpui_node_backend::file_capability::resolve_os_selection(selection, &spec);
    let root = root.clone();
    let _ = cx.update(|cx| {
        root.update(cx, |this, cx| {
            this.state
                .apply_pick_outcome(&key, generation, &outcome, failed_message.as_deref());
            cx.notify();
        })
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_gpui_node_backend::file_capability::FilePickOutcome;

    fn request(key: &str) -> NodeSpecimenEvent {
        NodeSpecimenEvent::FileBrowse {
            key: key.to_string(),
            spec: SingleFilePickSpec {
                prompt: "Choose a licence file".to_string(),
                accept: Some(".licence".to_string()),
                max_size: None,
            },
            failed_message: None,
        }
    }

    fn selected(name: &str) -> FilePickOutcome {
        FilePickOutcome::Selected {
            name: name.to_string(),
            contents_base64: "c3R1ZmY=".to_string(),
        }
    }

    /// A completed pick lands under the current generation; a route change
    /// invalidates the bytes AND makes a late outcome stale by generation so
    /// it can never repopulate a route the operator left.
    #[test]
    fn a_route_change_invalidates_file_state_and_stales_late_outcomes() {
        let mut state = AppState::new();
        // The operator selected a file; the task applies it.
        state
            .specimens
            .text
            .insert("la-file-name".to_string(), "machine.lic".to_string());
        state
            .specimens
            .text
            .insert("la-file-base64".to_string(), "c3R1ZmY=".to_string());
        state.active_file_keys.push("la-file".to_string());
        let generation = state.file_generation;
        state.apply_pick_outcome("la-file", generation, &selected("machine.lic"), None);

        // Switching away from offline invalidates the read: completed bytes
        // are cleared and the generation bumps.
        let mut events = Vec::new();
        events.push(NodeSpecimenEvent::FileInvalidate);
        state.drain_node_events_into(&mut events);
        assert!(state
            .specimens
            .text
            .get("la-file-base64")
            .is_none(), "bytes cleared on route change");
        assert!(state.specimens.text.get("la-file-name").is_none());
        assert!(state.file_generation > generation, "generation bumped");

        // The in-flight dialog resolves later with the old generation: stale,
        // so it cannot land.
        state.apply_pick_outcome("la-file", generation, &selected("machine.lic"), None);
        assert!(
            state.specimens.text.get("la-file-base64").is_none(),
            "a late outcome after the route change must not land"
        );

        // A fresh pick with the current generation does land.
        state.apply_pick_outcome(
            "la-file",
            state.file_generation,
            &selected("machine.lic"),
            None,
        );
        assert_eq!(
            state.specimens.text.get("la-file-name").map(String::as_str),
            Some("machine.lic")
        );
    }

    /// A pending request is dropped by invalidation and a later resolution
    /// for it is stale.
    #[test]
    fn a_pending_pick_is_dropped_by_route_change() {
        let mut state = AppState::new();
        let mut events = vec![request("la-file")];
        state.drain_node_events_into(&mut events);
        assert_eq!(state.pending_file_picks.len(), 1);

        let mut events = vec![NodeSpecimenEvent::FileInvalidate];
        state.drain_node_events_into(&mut events);
        assert!(
            state.pending_file_picks.is_empty(),
            "pending requests are dropped on route change"
        );
    }

    /// A read failure lands the request's polite message, never the raw OS
    /// error; an accept/size rejection keeps its honest copy verbatim.
    #[test]
    fn a_read_failure_lands_the_polite_message_not_the_os_error() {
        let mut state = AppState::new();
        state.apply_pick_outcome(
            "la-file",
            state.file_generation,
            &FilePickOutcome::Failed("No such file or directory (os error 2)".to_string()),
            Some("That file could not be read."),
        );
        assert_eq!(
            state
                .specimens
                .text
                .get("la-file-error")
                .map(String::as_str),
            Some("That file could not be read."),
            "the approved component message is pinned"
        );
        assert!(state.specimens.text.get("la-file-base64").is_none());

        // Without a polite override the raw reason is retained (generic seam).
        state.apply_pick_outcome(
            "la-file",
            state.file_generation,
            &FilePickOutcome::Failed("raw os reason".to_string()),
            None,
        );
        assert_eq!(
            state
                .specimens
                .text
                .get("la-file-error")
                .map(String::as_str),
            Some("raw os reason")
        );

        // An accept rejection keeps its honest copy.
        state.apply_pick_outcome(
            "la-file",
            state.file_generation,
            &FilePickOutcome::Rejected(
                "File type not accepted. Accepted types: .licence".to_string(),
            ),
            Some("That file could not be read."),
        );
        assert_eq!(
            state
                .specimens
                .text
                .get("la-file-error")
                .map(String::as_str),
            Some("File type not accepted. Accepted types: .licence")
        );
    }

    /// Escape restores the value snapped when editing started and closes the
    /// edit; the draft typed in between is discarded.
    #[test]
    fn a_machine_label_cancel_restores_the_snapshot() {
        let mut state = AppState::new();
        state
            .specimens
            .text
            .insert("la-machine-label".to_string(), "Studio Mac".to_string());
        state
            .specimens
            .toggles
            .insert("la-machine-editing".to_string(), true);
        state
            .specimens
            .text
            .insert("la-machine-label-original".to_string(), "Studio Mac".to_string());
        // Typing edits the draft.
        state
            .specimens
            .text
            .insert("la-machine-label".to_string(), "Studio Mac 2".to_string());

        let mut events = vec![NodeSpecimenEvent::MachineLabelCancel];
        state.drain_node_events_into(&mut events);
        assert_eq!(
            state
                .specimens
                .text
                .get("la-machine-label")
                .map(String::as_str),
            Some("Studio Mac"),
            "the original label is restored, not the typed draft"
        );
        assert!(!state.specimens.is_on("la-machine-editing"));
        assert!(state.specimens.text.get("la-machine-label-original").is_none());
    }

    impl AppState {
        fn drain_node_events_into(&mut self, events: &mut Vec<NodeSpecimenEvent>) {
            self.node_events
                .lock()
                .unwrap()
                .extend(events.drain(..));
            self.drain_node_events();
        }
    }
}
