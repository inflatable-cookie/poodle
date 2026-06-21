#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextElement {
    P,
    Span,
    Div,
}

impl Default for TextElement {
    fn default() -> Self {
        Self::P
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextTone {
    Default,
    Secondary,
    Muted,
    Success,
    Danger,
    Warning,
}

impl Default for TextTone {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextSize {
    Xs,
    Sm,
    Md,
}

impl Default for TextSize {
    fn default() -> Self {
        Self::Md
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextWeight {
    Normal,
    Medium,
    Semibold,
    Bold,
}

impl Default for TextWeight {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextLeading {
    Normal,
    Relaxed,
}

impl Default for TextLeading {
    fn default() -> Self {
        Self::Normal
    }
}

/// Compact-grid spacing for child paragraphs (contract §2 `spacing`).
/// `None` is the default (no gap); `Compact` renders a stacked grid with a
/// `space.stack.sm` gap between children.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextSpacing {
    None,
    Compact,
}

impl Default for TextSpacing {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TextSpec {
    pub content: String,
    pub element: TextElement,
    pub tone: TextTone,
    pub size: TextSize,
    pub weight: TextWeight,
    pub leading: TextLeading,
    pub spacing: TextSpacing,
    pub clamp: Option<u8>,
}

impl TextSpec {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            ..Self::default()
        }
    }

    pub fn with_element(mut self, element: TextElement) -> Self {
        self.element = element;
        self
    }

    pub fn with_tone(mut self, tone: TextTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn with_size(mut self, size: TextSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_weight(mut self, weight: TextWeight) -> Self {
        self.weight = weight;
        self
    }

    pub fn with_leading(mut self, leading: TextLeading) -> Self {
        self.leading = leading;
        self
    }

    pub fn with_spacing(mut self, spacing: TextSpacing) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn with_clamp(mut self, clamp: u8) -> Self {
        self.clamp = Some(clamp.clamp(1, 3));
        self
    }

    /// Stack gap token for `spacing="compact"`, else `None`.
    /// Contract §3: compact renders a grid with `--poodle-space-stack-sm` gap.
    pub fn spacing_gap_token(&self) -> Option<&'static str> {
        match self.spacing {
            TextSpacing::None => None,
            TextSpacing::Compact => Some(poodle_tokens::semantic::SPACE_STACK_SM),
        }
    }

    pub fn color_token(&self) -> &'static str {
        match self.tone {
            TextTone::Default => "color.text.primary",
            TextTone::Secondary | TextTone::Muted => "color.text.secondary",
            TextTone::Success => "color.status.success",
            TextTone::Danger => "color.status.danger",
            TextTone::Warning => "color.status.warning",
        }
    }

    pub fn font_size_rem(&self) -> f32 {
        match self.size {
            TextSize::Xs => 0.75,
            TextSize::Sm => 0.8125,
            TextSize::Md => 0.875,
        }
    }

    pub fn line_height(&self) -> f32 {
        match self.leading {
            TextLeading::Normal => 1.5,
            TextLeading::Relaxed => 1.6,
        }
    }
}
