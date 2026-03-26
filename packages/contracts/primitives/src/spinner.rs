use poodle_tokens::semantic;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpinnerVariant {
    Ring,
    Grid,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpinnerSize {
    Sm,
    Md,
    Lg,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpinnerTone {
    Current,
    Accent,
    Muted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpinnerSpec {
    pub variant: SpinnerVariant,
    pub size: SpinnerSize,
    pub tone: SpinnerTone,
    pub aria_label: Option<String>,
}

impl Default for SpinnerSpec {
    fn default() -> Self {
        Self {
            variant: SpinnerVariant::Ring,
            size: SpinnerSize::Md,
            tone: SpinnerTone::Current,
            aria_label: None,
        }
    }
}

impl SpinnerSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_variant(mut self, variant: SpinnerVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_size(mut self, size: SpinnerSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_tone(mut self, tone: SpinnerTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn size_px(&self) -> f32 {
        match self.size {
            SpinnerSize::Sm => 12.0,
            SpinnerSize::Md => 16.0,
            SpinnerSize::Lg => 24.0,
        }
    }

    pub fn tone_color_token(&self) -> Option<&'static str> {
        match self.tone {
            SpinnerTone::Current => None,
            SpinnerTone::Accent => Some(semantic::COLOR_ACCENT_BASE),
            SpinnerTone::Muted => Some(semantic::COLOR_TEXT_SECONDARY),
        }
    }
}
