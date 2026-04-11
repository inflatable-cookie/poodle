use poodle_tokens::semantic;
use crate::{ControlDensity, ControlSize, SemanticControlSizeRole, StatusTone};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PageHeaderAlign {
    Start,
    #[default]
    Between,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PageHeaderSpec {
    pub title: String,
    pub subtitle: Option<String>,
    pub eyebrow: Option<String>,
    /// Context label rendered above the title (distinct from `eyebrow`,
    /// which reads as a category tag). Matches Svelte `section` prop.
    pub section: Option<String>,
    /// Optional count rendered as a badge next to the title (e.g. "128").
    pub count: Option<u32>,
    /// Back-navigation link target. When set together with `back_label`
    /// a "Back to X" affordance is rendered above the header.
    pub back_href: Option<String>,
    /// Label for the back link.
    pub back_label: Option<String>,
    /// When true, the back link uses the contextual chrome-tone style
    /// (smaller, muted); otherwise it renders in the default link style.
    pub back_is_contextual: bool,
    /// Optional banner message shown below the header row.
    pub banner_message: Option<String>,
    /// Tone for the banner background/border color.
    pub banner_tone: StatusTone,
    pub align: PageHeaderAlign,
    pub aria_label: Option<String>,
    /// Heading level for the title element (1–6). Matches HTML
    /// <h1>–<h6> semantics. Renderers use this for both accessibility
    /// semantics and visual sizing hierarchy — higher levels render
    /// at smaller text sizes. Defaults to 1 (primary page heading).
    pub level: u8,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl PageHeaderSpec {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            subtitle: None,
            eyebrow: None,
            section: None,
            count: None,
            back_href: None,
            back_label: None,
            back_is_contextual: false,
            banner_message: None,
            banner_tone: StatusTone::Info,
            align: PageHeaderAlign::default(),
            aria_label: None,
            level: 1,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }

    pub fn with_subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn with_eyebrow(mut self, eyebrow: impl Into<String>) -> Self {
        self.eyebrow = Some(eyebrow.into());
        self
    }

    pub fn with_section(mut self, section: impl Into<String>) -> Self {
        self.section = Some(section.into());
        self
    }

    pub fn with_count(mut self, count: u32) -> Self {
        self.count = Some(count);
        self
    }

    /// Set the heading level (1–6). Values outside that range are
    /// clamped. Level 1 is the default and is used for primary page
    /// titles; higher levels are for nested headings in stacked
    /// layouts.
    pub fn with_level(mut self, level: u8) -> Self {
        self.level = level.clamp(1, 6);
        self
    }

    pub fn with_back(mut self, href: impl Into<String>, label: impl Into<String>) -> Self {
        self.back_href = Some(href.into());
        self.back_label = Some(label.into());
        self
    }

    pub fn with_back_is_contextual(mut self, back_is_contextual: bool) -> Self {
        self.back_is_contextual = back_is_contextual;
        self
    }

    pub fn with_banner(mut self, message: impl Into<String>, tone: StatusTone) -> Self {
        self.banner_message = Some(message.into());
        self.banner_tone = tone;
        self
    }

    pub fn has_back_link(&self) -> bool {
        self.back_href.is_some() && self.back_label.is_some()
    }

    pub fn has_banner(&self) -> bool {
        self.banner_message.is_some()
    }

    pub fn banner_color_token(&self) -> &'static str {
        self.banner_tone.color_token()
    }

    pub fn with_align(mut self, align: PageHeaderAlign) -> Self {
        self.align = align;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn title_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn subtitle_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn eyebrow_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn separator_color_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn back_color_token(&self) -> &'static str {
        if self.back_is_contextual {
            semantic::COLOR_TEXT_SECONDARY
        } else {
            semantic::COLOR_ACCENT_BASE
        }
    }

    pub fn count_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
    }

    pub fn header_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_MD
    }

    pub fn padding_y_token(&self) -> &'static str {
        semantic::SPACE_PANEL_Y
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
}
