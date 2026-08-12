#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AvatarSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AvatarShape {
    #[default]
    Circle,
    Rounded,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AvatarTone {
    #[default]
    Neutral,
    Accent,
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

    /// True when the avatar renders an image (`src` present). Image decode is a
    /// host/runtime concern; the spec only reports the URL via `src`.
    pub fn has_image(&self) -> bool {
        self.src.as_deref().is_some_and(|s| !s.trim().is_empty())
    }

    // ── Tone token targets (contract §3 + Avatar.svelte tone rules) ──
    //
    // Both Rust targets resolve these instead of inlining the token strings.
    // Background is `color-mix(base ratio%, mix)`; ratios live in
    // `background_mix_ratio()`.

    /// Base color token mixed to produce the fallback background.
    /// Neutral → `background.surface`; accent → `accent.base`.
    pub fn background_base_token(&self) -> &'static str {
        match self.tone {
            AvatarTone::Neutral => "color.background.surface",
            AvatarTone::Accent => "color.accent.base",
        }
    }

    /// Color token mixed *into* the base to produce the fallback background.
    /// Neutral → `text.primary`; accent → `background.surface`.
    pub fn background_mix_token(&self) -> &'static str {
        match self.tone {
            AvatarTone::Neutral => "color.text.primary",
            AvatarTone::Accent => "color.background.surface",
        }
    }

    /// Proportion of the base color kept in the mix (CSS `base X%`).
    /// Neutral → 0.82 (surface 82%); accent → 0.76 (accent 76%).
    pub fn background_mix_ratio(&self) -> f32 {
        match self.tone {
            AvatarTone::Neutral => 0.82,
            AvatarTone::Accent => 0.76,
        }
    }

    /// Foreground (initials) color token.
    /// Neutral → `text.secondary`; accent → `text.primary`.
    pub fn color_token(&self) -> &'static str {
        match self.tone {
            AvatarTone::Neutral => "color.text.secondary",
            AvatarTone::Accent => "color.text.primary",
        }
    }

    /// Rounded-shape radius token (contract §3 `--poodle-radius-control`).
    /// Circle shape is computed as half the resolved size (`border-radius: 50%`),
    /// so it has no token of its own; see `is_circle`.
    pub fn radius_token(&self) -> &'static str {
        "radius.control"
    }

    /// True when the shape is a circle (`border-radius: 50%`).
    pub fn is_circle(&self) -> bool {
        self.shape == AvatarShape::Circle
    }

    /// Circle radius in rem: half the box size (CSS `border-radius: 50%` on a
    /// square box). Only meaningful when `is_circle()`.
    pub fn circle_radius_rem(&self) -> f32 {
        self.size_rem() / 2.0
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
