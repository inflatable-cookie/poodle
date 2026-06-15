#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AvatarSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

impl Default for AvatarSize {
    fn default() -> Self {
        Self::Md
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AvatarShape {
    Circle,
    Rounded,
}

impl Default for AvatarShape {
    fn default() -> Self {
        Self::Circle
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AvatarTone {
    Neutral,
    Accent,
}

impl Default for AvatarTone {
    fn default() -> Self {
        Self::Neutral
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AvatarSpec {
    pub src: Option<String>,
    pub alt: Option<String>,
    pub initials: Option<String>,
    pub aria_label: Option<String>,
    pub decorative: bool,
    pub size: AvatarSize,
    pub shape: AvatarShape,
    pub tone: AvatarTone,
}

impl AvatarSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_src(mut self, src: impl Into<String>) -> Self {
        self.src = Some(src.into());
        self
    }

    pub fn with_alt(mut self, alt: impl Into<String>) -> Self {
        self.alt = Some(alt.into());
        self
    }

    pub fn with_initials(mut self, initials: impl Into<String>) -> Self {
        self.initials = Some(initials.into());
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_decorative(mut self, decorative: bool) -> Self {
        self.decorative = decorative;
        self
    }

    pub fn with_size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_shape(mut self, shape: AvatarShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn with_tone(mut self, tone: AvatarTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn fallback_text(&self) -> String {
        let raw = self.initials.as_deref().unwrap_or("?").trim();
        let text = if raw.is_empty() { "?" } else { raw };
        text.chars().take(3).collect::<String>().to_uppercase()
    }

    pub fn accessible_label(&self) -> Option<String> {
        if self.decorative {
            return None;
        }
        Some(
            self.aria_label
                .as_deref()
                .or(self.alt.as_deref())
                .or(self.initials.as_deref())
                .unwrap_or("Avatar")
                .to_string(),
        )
    }

    pub fn size_rem(&self) -> f32 {
        match self.size {
            AvatarSize::Xs => 1.5,
            AvatarSize::Sm => 2.0,
            AvatarSize::Md => 2.75,
            AvatarSize::Lg => 4.5,
            AvatarSize::Xl => 6.0,
        }
    }

    pub fn font_size_rem(&self) -> f32 {
        match self.size {
            AvatarSize::Xs => 0.625,
            AvatarSize::Sm => 0.75,
            AvatarSize::Md => 1.0,
            AvatarSize::Lg => 1.5,
            AvatarSize::Xl => 2.0,
        }
    }
}
