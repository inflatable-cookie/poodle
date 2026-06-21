use poodle_tokens::semantic;

/// Rendered element / heading semantics for an `Eyebrow`.
/// Mirrors the Svelte `as` prop (`span` | `p` | `h2` | `h3` | `h4`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum EyebrowElement {
    #[default]
    Span,
    P,
    H2,
    H3,
    H4,
}

impl EyebrowElement {
    /// True when the element carries heading semantics (`h2`/`h3`/`h4`).
    pub fn is_heading(self) -> bool {
        matches!(self, Self::H2 | Self::H3 | Self::H4)
    }

    /// ARIA heading level, when the element is a heading.
    pub fn heading_level(self) -> Option<u8> {
        match self {
            Self::H2 => Some(2),
            Self::H3 => Some(3),
            Self::H4 => Some(4),
            _ => None,
        }
    }
}

/// Visual size of an `Eyebrow`. `Sm` preserves the historical default.
/// Mirrors the Svelte `size` prop (`xs` | `sm` | `md`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum EyebrowSize {
    Xs,
    #[default]
    Sm,
    Md,
}

/// Optional bottom margin for heading-like use.
/// Mirrors the Svelte `spacing` prop (`none` | `bottom`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum EyebrowSpacing {
    #[default]
    None,
    Bottom,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EyebrowSpec {
    pub content: Option<String>,
    /// Rendered element + heading semantics (contract §3 `as`, default `span`).
    pub element: EyebrowElement,
    /// Optional explicit accessible name (contract §3 `ariaLabel`).
    pub aria_label: Option<String>,
    /// Visual size (contract §3 `size`, default `sm`).
    pub size: EyebrowSize,
    /// Optional bottom margin for heading-like use (contract §3 `spacing`).
    pub spacing: EyebrowSpacing,
}

impl Default for EyebrowSpec {
    fn default() -> Self {
        Self {
            content: None,
            element: EyebrowElement::Span,
            aria_label: None,
            size: EyebrowSize::Sm,
            spacing: EyebrowSpacing::None,
        }
    }
}

impl EyebrowSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn with_element(mut self, element: EyebrowElement) -> Self {
        self.element = element;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_size(mut self, size: EyebrowSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_spacing(mut self, spacing: EyebrowSpacing) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn font_size_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_LABEL_SIZE
    }

    /// Label font-family token string (contract §8 `font-family`).
    pub fn font_family(&self) -> &'static str {
        semantic::TYPOGRAPHY_LABEL_FAMILY
    }

    /// Contract §8 `font-weight: 600` — a literal, not the label-weight token
    /// (which is 500). Eyebrow is intentionally heavier than a plain label.
    pub fn font_weight(&self) -> u16 {
        600
    }

    /// Contract §8 `line-height: 1.5`.
    pub fn line_height(&self) -> f32 {
        1.5
    }

    /// Contract §8 font-size per size variant (rem). `xs`/`sm` share `0.6875rem`;
    /// `md` is `0.85rem`. Contract-exact literals.
    pub fn font_size_rem(&self) -> f32 {
        match self.size {
            EyebrowSize::Xs | EyebrowSize::Sm => 0.6875,
            EyebrowSize::Md => 0.85,
        }
    }

    /// Contract §8 letter-spacing per size variant (em). Base `0.12em`; `md`
    /// tightens to `0.04em`. Contract-exact literals. Runtimes without a
    /// letter-spacing channel record this as an accepted typographic delta.
    pub fn letter_spacing_em(&self) -> f32 {
        match self.size {
            EyebrowSize::Md => 0.04,
            _ => 0.12,
        }
    }

    /// Contract §8 bottom margin when `spacing="bottom"` (rem). `0.5rem`
    /// normally, `0.35rem` for the `xs` size. Returns `0.0` when spacing is
    /// `none`. Contract-exact literals.
    pub fn margin_bottom_rem(&self) -> f32 {
        match self.spacing {
            EyebrowSpacing::None => 0.0,
            EyebrowSpacing::Bottom => match self.size {
                EyebrowSize::Xs => 0.35,
                _ => 0.5,
            },
        }
    }
}
