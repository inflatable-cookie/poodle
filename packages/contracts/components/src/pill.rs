use crate::types::{ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

use crate::InlineTypographyMode;

/// Tone / semantic color of a pill.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PillTone {
    #[default]
    Neutral,
    Info,
    Success,
    Warning,
    Danger,
}

/// Visual appearance / style of a pill. Mutually exclusive: `Tint` is the
/// ordinary tone-tinted shell, `Solid` the opaque high-contrast treatment.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PillAppearance {
    #[default]
    Tint,
    Solid,
    Subtle,
    Badge,
}

/// Size of a pill.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PillSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

/// Font family of a pill.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PillFont {
    #[default]
    Normal,
    Mono,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PillSpec {
    pub label: String,
    pub tone: PillTone,
    pub appearance: PillAppearance,
    pub size: PillSize,
    pub font: PillFont,
    pub typography: InlineTypographyMode,
    pub is_muted: bool,
    pub is_removable: bool,
    pub is_selected: bool,
    pub is_disabled: bool,
    pub density: crate::types::ControlDensity,
    /// Optional CSS hex string for a custom accent color (overrides tone fill).
    pub accent_color: Option<String>,
    /// Accessible name (contract §7). `None` falls back to the visible label.
    pub aria_label: Option<String>,
    /// Presentation axes (contract §3): size is intrinsic, density is sibling
    /// spacing, size_role resolves size from the inherited presentation.
    pub size_role: SemanticControlSizeRole,
    /// Whether the pill grows to fit its label instead of holding a floor width.
    pub has_adaptive_width: bool,
    /// Whether a tone-coloured status dot renders before the label.
    pub has_dot: bool,
    /// Native tooltip text.
    pub title: Option<String>,
}

impl Default for PillSpec {
    fn default() -> Self {
        Self {
            label: String::new(),
            tone: PillTone::default(),
            appearance: PillAppearance::default(),
            size: PillSize::default(),
            font: PillFont::default(),
            typography: InlineTypographyMode::default(),
            is_muted: false,
            is_removable: false,
            is_selected: false,
            is_disabled: false,
            density: crate::types::ControlDensity::default(),
            accent_color: None,
            aria_label: None,
            size_role: SemanticControlSizeRole::Control,
            has_adaptive_width: false,
            has_dot: false,
            title: None,
        }
    }
}

impl PillSpec {
    /// Size after the semantic role is applied. Pill shares the five-stop
    /// control scale, so it reuses the shared shift rather than its own copy.
    pub fn resolved_size(&self) -> PillSize {
        let control = match self.size {
            PillSize::Xs => ControlSize::Xs,
            PillSize::Sm => ControlSize::Sm,
            PillSize::Md => ControlSize::Md,
            PillSize::Lg => ControlSize::Lg,
            PillSize::Xl => ControlSize::Xl,
        };
        match crate::types::resolve_semantic_control_size(control, self.size_role) {
            ControlSize::Xs => PillSize::Xs,
            ControlSize::Sm => PillSize::Sm,
            ControlSize::Md => PillSize::Md,
            ControlSize::Lg => PillSize::Lg,
            ControlSize::Xl => PillSize::Xl,
        }
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn with_tone(mut self, tone: PillTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn with_appearance(mut self, appearance: PillAppearance) -> Self {
        self.appearance = appearance;
        self
    }

    pub fn with_size(mut self, size: PillSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_font(mut self, font: PillFont) -> Self {
        self.font = font;
        self
    }

    pub fn with_typography(mut self, typography: InlineTypographyMode) -> Self {
        self.typography = typography;
        self
    }

    pub fn with_muted(mut self, is_muted: bool) -> Self {
        self.is_muted = is_muted;
        self
    }

    pub fn with_removable(mut self, is_removable: bool) -> Self {
        self.is_removable = is_removable;
        self
    }

    pub fn with_selected(mut self, is_selected: bool) -> Self {
        self.is_selected = is_selected;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_density(mut self, density: crate::types::ControlDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_accent_color(mut self, color: impl Into<String>) -> Self {
        self.accent_color = Some(color.into());
        self
    }

    pub fn is_solid_appearance(&self) -> bool {
        matches!(self.appearance, PillAppearance::Solid)
    }

    /// Background fill token based on tone + appearance.
    pub fn fill_token(&self) -> &'static str {
        if self.is_selected {
            return semantic::COLOR_ACCENT_BASE;
        }
        match self.appearance {
            PillAppearance::Badge => semantic::COLOR_ACCENT_BASE,
            _ => semantic::COLOR_BACKGROUND_SURFACE,
        }
    }

    /// Text color token.
    pub fn text_color_token(&self) -> &'static str {
        if self.is_selected {
            semantic::COLOR_TEXT_INVERSE
        } else {
            semantic::COLOR_TEXT_PRIMARY
        }
    }

    /// Tone-specific accent token for tinted background.
    pub fn tone_color_token(&self) -> &'static str {
        match self.tone {
            PillTone::Neutral => semantic::COLOR_TEXT_SECONDARY,
            PillTone::Info => semantic::COLOR_STATUS_INFO,
            PillTone::Success => semantic::COLOR_STATUS_SUCCESS,
            PillTone::Warning => semantic::COLOR_STATUS_WARNING,
            PillTone::Danger => semantic::COLOR_STATUS_DANGER,
        }
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }
}
