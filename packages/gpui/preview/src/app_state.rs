//! Application state for the preview app.
//!
//! Mirrors the Svelte preview app's state: theme, density, control size,
//! appearance treatment, state probes, active section, and component selection.

use std::collections::HashMap;
use poodle_gpui::GpuiThemeProvider;

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
                GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::DARK)
            }
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
    pub const ALL: &[ControlSize] = &[ControlSize::Xs, ControlSize::Sm, ControlSize::Md, ControlSize::Lg, ControlSize::Xl];

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
    pub disabled: bool,
    pub invalid: bool,
    pub busy: bool,
    pub active_primitive: Option<usize>,
    pub active_composite: Option<usize>,
    pub active_shell: Option<usize>,
    pub active_demo_screen: DemoScreen,
    #[allow(dead_code)]
    pub debug_clicks: u32,
    pub specimens: SpecimenState,
}

impl AppState {
    pub fn new() -> Self {
        let preset = ThemePreset::Dark;
        let density = Density::Compact;
        let control_size = ControlSize::Md;
        let appearance_treatment = AppearanceTreatment::System;

        // Build theme with density + control-size layered on top
        let theme = preset.build_theme()
            .with_density(density.token_definition())
            .with_control_size(control_size.token_definition());

        Self {
            section: Section::Primitives,
            theme,
            theme_preset: preset,
            density,
            control_size,
            appearance_treatment,
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
}
