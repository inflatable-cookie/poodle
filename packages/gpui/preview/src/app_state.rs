//! Application state for the preview app.
//!
//! Mirrors the Svelte preview app's state: theme, density, control size,
//! appearance treatment, state probes, active section, and component selection.

use std::collections::HashMap;
use pug_gpui::GpuiThemeProvider;

/// Which top-level section is active.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    Primitives,
    Composites,
    Shells,
    Demo,
    Tokens,
}

impl Section {
    pub const ALL: &[Section] = &[
        Section::Primitives,
        Section::Composites,
        Section::Shells,
        Section::Demo,
        Section::Tokens,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Section::Primitives => "Primitives",
            Section::Composites => "Composites",
            Section::Shells => "Shells",
            Section::Demo => "Demo",
            Section::Tokens => "Tokens",
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
            ThemePreset::Dark => {
                GpuiThemeProvider::new().with_theme(&pug_tokens::themes::DARK)
            }
            ThemePreset::Light => {
                GpuiThemeProvider::new().with_theme(&pug_tokens::themes::LIGHT)
            }
            ThemePreset::LoopholeStudio => {
                GpuiThemeProvider::new().with_theme(&pug_tokens::themes::LOOPHOLE_STUDIO)
            }
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
    Sm,
    Md,
    Lg,
}

impl ControlSize {
    /// Order matches Svelte preview: lg, md, sm.
    pub const ALL: &[ControlSize] = &[ControlSize::Lg, ControlSize::Md, ControlSize::Sm];

    pub fn label(self) -> &'static str {
        match self {
            ControlSize::Sm => "sm",
            ControlSize::Md => "md",
            ControlSize::Lg => "lg",
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
        Self {
            toggles: HashMap::new(),
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

/// Global application state.
pub struct AppState {
    pub section: Section,
    pub theme: GpuiThemeProvider,
    pub theme_preset: ThemePreset,
    pub density: Density,
    pub control_size: ControlSize,
    pub appearance_treatment: AppearanceTreatment,
    pub disabled: bool,
    pub invalid: bool,
    pub busy: bool,
    pub active_primitive: Option<usize>,
    pub active_composite: Option<usize>,
    pub active_shell: Option<usize>,
    pub active_demo_screen: DemoScreen,
    pub debug_clicks: u32,
    pub specimens: SpecimenState,
}

impl AppState {
    pub fn new() -> Self {
        let preset = ThemePreset::Dark;
        Self {
            section: Section::Primitives,
            theme: preset.build_theme(),
            theme_preset: preset,
            density: Density::Compact,
            control_size: ControlSize::Md,
            appearance_treatment: AppearanceTreatment::System,
            disabled: false,
            invalid: true,
            busy: false,
            active_primitive: None,
            active_composite: None,
            active_shell: None,
            active_demo_screen: DemoScreen::OverviewShell,
            debug_clicks: 0,
            specimens: SpecimenState::new(),
        }
    }

    pub fn set_theme(&mut self, preset: ThemePreset) {
        self.theme_preset = preset;
        self.theme = preset.build_theme();
    }
}
