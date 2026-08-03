//! ThemeSelectSpec — theme picker backed by a swatch catalogue.
//!
//! Contract: `docs/contracts/components/theme-select.md`.
//!
//! The component renders a trigger (current theme's swatch) that opens a popover
//! grid of theme swatch tiles. Swatch colors are literal per-theme hex strings
//! (from the token metadata); the chrome resolves from semantic tokens. Applying
//! the selected theme is host/controller work (web: `data-theme`); native switches
//! its own ThemeProvider.

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThemeSwatch {
    pub canvas: String,
    pub surface: String,
    pub accent: String,
    pub text: String,
    pub border: String,
}

impl ThemeSwatch {
    pub fn new(
        canvas: impl Into<String>,
        surface: impl Into<String>,
        accent: impl Into<String>,
        text: impl Into<String>,
        border: impl Into<String>,
    ) -> Self {
        Self {
            canvas: canvas.into(),
            surface: surface.into(),
            accent: accent.into(),
            text: text.into(),
            border: border.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThemeOption {
    pub value: String,
    pub label: String,
    pub description: String,
    pub swatch: ThemeSwatch,
}

impl ThemeOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>, swatch: ThemeSwatch) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: String::new(),
            swatch,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
}

#[derive(Clone)]
pub struct ThemeSelectSpec {
    pub themes: Vec<ThemeOption>,
    pub value: String,
    pub aria_label: String,
    pub is_disabled: bool,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
    pub columns: usize,
    pub show_label: bool,
    pub is_open: bool,
}

impl ThemeSelectSpec {
    pub fn new() -> Self {
        Self {
            themes: Vec::new(),
            value: String::new(),
            aria_label: "Theme".to_string(),
            is_disabled: false,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
            columns: 3,
            show_label: true,
            is_open: false,
        }
    }

    pub fn with_themes(mut self, themes: Vec<ThemeOption>) -> Self {
        self.themes = themes;
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = label.into();
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_columns(mut self, columns: usize) -> Self {
        self.columns = columns.max(1);
        self
    }

    pub fn with_show_label(mut self, show: bool) -> Self {
        self.show_label = show;
        self
    }

    pub fn with_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    /// The currently selected theme option, if present.
    pub fn current_option(&self) -> Option<&ThemeOption> {
        self.themes.iter().find(|theme| theme.value == self.value)
    }

    /// Trigger label: the current theme's label, or "Theme".
    pub fn trigger_label(&self) -> String {
        self.current_option()
            .map(|theme| theme.label.clone())
            .unwrap_or_else(|| "Theme".to_string())
    }

    pub fn is_selected(&self, option: &ThemeOption) -> bool {
        option.value == self.value
    }

    // ── Chrome token accessors (shared by GPUI + Jetstream) ───────────────

    pub fn field_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn field_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn field_text_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn label_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn surface_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn surface_radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    pub fn item_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn accent_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }
}

impl Default for ThemeSelectSpec {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ThemeSelectSpec {
        ThemeSelectSpec::new()
            .with_themes(vec![
                ThemeOption::new(
                    "eclipse",
                    "Eclipse",
                    ThemeSwatch::new("#0e1012", "#15181b", "#f0b24d", "#eef2f6", "#333"),
                ),
                ThemeOption::new(
                    "iceberg",
                    "Iceberg",
                    ThemeSwatch::new("#e7eef5", "#dbe5ef", "#2d86f3", "#131a22", "#75869b"),
                ),
            ])
            .with_value("iceberg")
    }

    #[test]
    fn current_option_and_label() {
        let spec = sample();
        assert_eq!(
            spec.current_option().map(|t| t.value.as_str()),
            Some("iceberg")
        );
        assert_eq!(spec.trigger_label(), "Iceberg");
    }

    #[test]
    fn is_selected_matches_value() {
        let spec = sample();
        assert!(spec.is_selected(&spec.themes[1]));
        assert!(!spec.is_selected(&spec.themes[0]));
    }

    #[test]
    fn unknown_value_falls_back_to_theme_label() {
        let spec = ThemeSelectSpec::new().with_value("nope");
        assert!(spec.current_option().is_none());
        assert_eq!(spec.trigger_label(), "Theme");
    }
}
