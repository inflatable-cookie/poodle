use poodle_tokens::semantic;

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

/// Visual appearance / style of a pill.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PillAppearance {
    #[default]
    Solid,
    Subtle,
    Badge,
}

/// Size of a pill.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PillSize {
    Sm,
    #[default]
    Md,
    Lg,
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
    pub is_muted: bool,
    pub is_removable: bool,
    pub is_selected: bool,
    pub is_disabled: bool,
}

impl Default for PillSpec {
    fn default() -> Self {
        Self {
            label: String::new(),
            tone: PillTone::default(),
            appearance: PillAppearance::default(),
            size: PillSize::default(),
            font: PillFont::default(),
            is_muted: false,
            is_removable: false,
            is_selected: false,
            is_disabled: false,
        }
    }
}

impl PillSpec {
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
            PillTone::Info => semantic::COLOR_ACCENT_BASE,
            PillTone::Success => semantic::COLOR_STATUS_SUCCESS,
            PillTone::Warning => semantic::COLOR_STATUS_WARNING,
            PillTone::Danger => semantic::COLOR_STATUS_DANGER,
        }
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }
}
