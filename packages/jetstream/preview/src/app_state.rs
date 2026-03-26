//! Application state for the Jetstream preview app.
//!
//! Mirrors the GPUI preview app's state structure: section navigation,
//! theme preset, density, control size, and per-specimen interaction state.

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
