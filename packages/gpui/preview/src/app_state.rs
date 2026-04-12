//! Application state for the preview app.
//!
//! Mirrors the current Svelte preview shell: theme, density, control size,
//! appearance treatment, component search, active section, and component selection.

use poodle_gpui::GpuiThemeProvider;
use std::collections::HashMap;

/// Which top-level section is active.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    Components,
    Demo,
    Tokens,
    Treatments,
}

impl Section {
    pub fn label(self) -> &'static str {
        match self {
            Section::Components => "Components",
            Section::Demo => "Demo",
            Section::Tokens => "Tokens",
            Section::Treatments => "Treatments",
        }
    }

    pub fn cli_value(self) -> &'static str {
        match self {
            Section::Components => "components",
            Section::Demo => "demo",
            Section::Tokens => "tokens",
            Section::Treatments => "treatments",
        }
    }
}

/// Available theme presets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemePreset {
    LoopholeStudio,
    Dark,
    Light,
    Default,
}

impl ThemePreset {
    /// Order matches Svelte preview: dark, light, loophole-studio.
    pub const ALL: &[ThemePreset] = &[
        ThemePreset::Dark,
        ThemePreset::Light,
        ThemePreset::LoopholeStudio,
    ];

    pub fn label(self) -> &'static str {
        match self {
            ThemePreset::Default => "default",
            ThemePreset::Dark => "dark",
            ThemePreset::Light => "light",
            ThemePreset::LoopholeStudio => "loophole-studio",
        }
    }

    pub fn build_theme(self) -> GpuiThemeProvider {
        match self {
            ThemePreset::Default => GpuiThemeProvider::new(),
            ThemePreset::Dark => GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::DARK),
            ThemePreset::Light => {
                GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::LIGHT)
            }
            ThemePreset::LoopholeStudio => {
                GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::LOOPHOLE_STUDIO)
            }
        }
    }
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

/// Appearance treatment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppearanceTreatment {
    System,
    BrandRaised,
}

impl AppearanceTreatment {
    pub const ALL: &[AppearanceTreatment] = &[
        AppearanceTreatment::System,
        AppearanceTreatment::BrandRaised,
    ];

    pub fn label(self) -> &'static str {
        match self {
            AppearanceTreatment::System => "system",
            AppearanceTreatment::BrandRaised => "brand-raised",
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

    pub fn cli_value(self) -> &'static str {
        match self {
            TokenPanel::Summary => "summary",
            TokenPanel::Inspector => "inspector",
        }
    }
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

    pub fn title(self) -> &'static str {
        match self {
            DemoScreen::OverviewShell => "Overview shell",
            DemoScreen::FormAndValidation => "Form and validation",
            DemoScreen::BrowseAndTable => "Browse and table",
            DemoScreen::DetailAndRelatedData => "Detail and related data",
            DemoScreen::PickerAndMedia => "Picker and media",
            DemoScreen::CommandAndWorkspace => "Command and workspace",
        }
    }

    pub fn cli_value(self) -> &'static str {
        match self {
            DemoScreen::OverviewShell => "overview",
            DemoScreen::FormAndValidation => "form",
            DemoScreen::BrowseAndTable => "browse",
            DemoScreen::DetailAndRelatedData => "detail",
            DemoScreen::PickerAndMedia => "picker",
            DemoScreen::CommandAndWorkspace => "workspace",
        }
    }

    pub fn comparison_mode(self) -> &'static str {
        match self {
            DemoScreen::OverviewShell => "direct-parity",
            DemoScreen::FormAndValidation => "direct-parity",
            DemoScreen::BrowseAndTable => "native-adaptation",
            DemoScreen::DetailAndRelatedData => "direct-parity",
            DemoScreen::PickerAndMedia => "native-adaptation",
            DemoScreen::CommandAndWorkspace => "direct-parity",
        }
    }

    pub fn summary(self) -> &'static str {
        match self {
            DemoScreen::OverviewShell => {
                "Establish shell hierarchy, identity, and status posture without docs-shell noise."
            }
            DemoScreen::FormAndValidation => {
                "Keep fields, validation, remediation, and actions inside one realistic workflow."
            }
            DemoScreen::BrowseAndTable => {
                "Show filters, table posture, selection, and pagination as one browse story."
            }
            DemoScreen::DetailAndRelatedData => {
                "Keep headers, metadata, and related summaries inside one coherent detail workflow."
            }
            DemoScreen::PickerAndMedia => {
                "Pair selection flow, preview framing, and media fallback in one workflow."
            }
            DemoScreen::CommandAndWorkspace => {
                "Keep command discovery, shell navigation, split regions, and status inside one workstation target."
            }
        }
    }

    pub fn source_sections(self) -> &'static [&'static str] {
        match self {
            DemoScreen::OverviewShell => &["notification-suite", "workspace-suite"],
            DemoScreen::FormAndValidation => &["form-suite", "notification-suite"],
            DemoScreen::BrowseAndTable => &["browse-suite", "table-suite"],
            DemoScreen::DetailAndRelatedData => &["detail-suite"],
            DemoScreen::PickerAndMedia => &["picker-suite", "media-suite"],
            DemoScreen::CommandAndWorkspace => &["command-suite", "workspace-suite"],
        }
    }

    pub fn state_matrix(self) -> &'static [&'static str] {
        match self {
            DemoScreen::OverviewShell => &["default", "status-active", "review-blocked"],
            DemoScreen::FormAndValidation => &["default", "invalid", "pending", "disabled"],
            DemoScreen::BrowseAndTable => &[
                "ready",
                "selection-active",
                "empty",
                "no-results",
                "loading",
            ],
            DemoScreen::DetailAndRelatedData => &["default", "metadata-dense", "action-emphasis"],
            DemoScreen::PickerAndMedia => &[
                "inline-ready",
                "modal-open",
                "media-ready",
                "media-empty",
                "media-error",
            ],
            DemoScreen::CommandAndWorkspace => &[
                "shell-ready",
                "command-open",
                "docking-visible",
                "persistence-visible",
            ],
        }
    }

    pub fn region_ids(self) -> &'static [&'static str] {
        match self {
            DemoScreen::OverviewShell => &[
                "app-header",
                "screen-tabs",
                "primary-content",
                "companion-panel",
                "status-bar",
            ],
            DemoScreen::FormAndValidation => &[
                "context-toolbar",
                "primary-content",
                "companion-panel",
                "modal-layer",
            ],
            DemoScreen::BrowseAndTable => {
                &["context-toolbar", "primary-content", "companion-panel"]
            }
            DemoScreen::DetailAndRelatedData => {
                &["primary-content", "companion-panel", "context-toolbar"]
            }
            DemoScreen::PickerAndMedia => &[
                "context-toolbar",
                "primary-content",
                "companion-panel",
                "modal-layer",
            ],
            DemoScreen::CommandAndWorkspace => &[
                "app-header",
                "screen-tabs",
                "primary-content",
                "companion-panel",
                "status-bar",
                "modal-layer",
            ],
        }
    }

    pub fn has_modal_layer(self) -> bool {
        matches!(
            self,
            DemoScreen::FormAndValidation
                | DemoScreen::PickerAndMedia
                | DemoScreen::CommandAndWorkspace
        )
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
        }
    }

    pub fn toggle(&mut self, key: &str) -> bool {
        let val = self.toggles.entry(key.to_string()).or_insert(false);
        *val = !*val;
        *val
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

/// Global application state.
pub struct AppState {
    pub section: Section,
    pub theme: GpuiThemeProvider,
    pub theme_preset: ThemePreset,
    pub density: Density,
    pub control_size: ControlSize,
    pub appearance_treatment: AppearanceTreatment,
    pub component_search: String,
    pub active_component_slug: Option<String>,
    pub active_token_panel: TokenPanel,
    pub token_inspector_query: String,
    pub active_demo_screen: DemoScreen,
    #[allow(dead_code)]
    pub debug_clicks: u32,
    pub specimens: SpecimenState,
}

impl AppState {
    pub fn new() -> Self {
        let preset = ThemePreset::Dark;
        let density = Density::Compact;
        let control_size = ControlSize::Sm;
        let appearance_treatment = AppearanceTreatment::System;

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
            appearance_treatment,
            component_search: String::new(),
            active_component_slug: None,
            active_token_panel: TokenPanel::Summary,
            token_inspector_query: String::new(),
            active_demo_screen: DemoScreen::OverviewShell,
            debug_clicks: 0,
            specimens: SpecimenState::new(),
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
        theme.brand_raised = self.appearance_treatment == AppearanceTreatment::BrandRaised;
        self.theme = theme;
    }

    pub fn native_launch_args(&self) -> Vec<String> {
        let defaults = AppState::new();
        let mut args = Vec::new();

        if self.section != defaults.section {
            args.push("--section".to_string());
            args.push(self.section.cli_value().to_string());
        }

        if self.theme_preset != defaults.theme_preset {
            args.push("--theme".to_string());
            args.push(self.theme_preset.label().to_string());
        }

        if self.density != defaults.density {
            args.push("--density".to_string());
            args.push(self.density.label().to_string());
        }

        if self.control_size != defaults.control_size {
            args.push("--size".to_string());
            args.push(self.control_size.label().to_string());
        }

        if self.appearance_treatment != defaults.appearance_treatment {
            args.push("--treatment".to_string());
            args.push(self.appearance_treatment.label().to_string());
        }

        if !self.component_search.trim().is_empty() {
            args.push("--search".to_string());
            args.push(self.component_search.clone());
        }

        if let Some(slug) = self.active_component_slug.as_ref() {
            args.push("--component".to_string());
            args.push(slug.clone());
        }

        if self.active_token_panel != defaults.active_token_panel {
            args.push("--token-panel".to_string());
            args.push(self.active_token_panel.cli_value().to_string());
        }

        if !self.token_inspector_query.trim().is_empty() {
            args.push("--token-query".to_string());
            args.push(self.token_inspector_query.clone());
        }

        if self.active_demo_screen != defaults.active_demo_screen {
            args.push("--demo-screen".to_string());
            args.push(self.active_demo_screen.cli_value().to_string());
        }

        args
    }

    pub fn has_native_review_state(&self) -> bool {
        !self.native_launch_args().is_empty()
    }

    pub fn native_launch_command(&self) -> String {
        let args = self.native_launch_args();
        let mut parts = vec![
            "cargo".to_string(),
            "run".to_string(),
            "--manifest-path".to_string(),
            "packages/gpui/preview/Cargo.toml".to_string(),
        ];

        if !args.is_empty() {
            parts.push("--".to_string());
            parts.extend(args);
        }

        parts
            .into_iter()
            .map(|part| shell_quote(&part))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn shell_quote(value: &str) -> String {
    if value.is_empty() {
        return "''".to_string();
    }

    if value
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '/' | '.' | ':'))
    {
        return value.to_string();
    }

    format!("'{}'", value.replace('\'', "'\"'\"'"))
}
