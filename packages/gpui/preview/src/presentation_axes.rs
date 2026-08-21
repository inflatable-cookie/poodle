//! Presentation axes shared by the preview shell and the offscreen capture
//! target: the theme preset and control size domains.
//!
//! These are the single authority for "which themes / control sizes exist" on
//! the GPUI surface. The capture target includes this file by path so it
//! validates its CLI against exactly the same domain the interactive preview
//! offers — no second enumeration to drift.

use poodle_gpui::GpuiThemeProvider;

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

    /// Parse a CLI value against the domain; unknown names are rejected, never
    /// silently defaulted.
    // Used by the offscreen capture target, which includes this file by path;
    // the interactive preview never parses theme names.
    #[allow(dead_code)]
    pub fn parse(value: &str) -> Option<Self> {
        if value == ThemePreset::Default.label() {
            return Some(ThemePreset::Default);
        }
        Self::ALL.iter().copied().find(|p| p.label() == value)
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
            ThemePreset::Cobalt => GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::COBALT),
            ThemePreset::Clay => GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::CLAY),
            ThemePreset::Meadow => {
                GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::MEADOW)
            }
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

    /// Parse a CLI value against the domain; unknown names are rejected, never
    /// silently defaulted.
    // Used by the offscreen capture target, which includes this file by path;
    // the interactive preview never parses control-size names.
    #[allow(dead_code)]
    pub fn parse(value: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|s| s.label() == value)
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
