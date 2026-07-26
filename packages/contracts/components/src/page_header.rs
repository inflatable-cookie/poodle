use crate::{ControlDensity, ControlSize, SemanticControlSizeRole, StatusTone};
use poodle_tokens::semantic;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PageHeaderAlign {
    Start,
    #[default]
    Between,
}

/// Layout posture of a page header.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PageHeaderPosture {
    #[default]
    Default,
    EntityDetail,
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
    /// When true and a section+title split exists, swaps the section/title
    /// roles (section becomes the primary heading, title drops to subtitle)
    /// and moves breadcrumbs into the subtitle region. Matches the Svelte
    /// `posture="entity-detail"` behavior.
    pub entity_detail_posture: bool,
    /// Heading level for the title element (1–6). Matches HTML
    /// <h1>–<h6> semantics. Renderers use this for both accessibility
    /// semantics and visual sizing hierarchy — higher levels render
    /// at smaller text sizes. Defaults to 2 (matches Svelte/contract).
    pub level: u8,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
    /// Layout posture — an entity detail header sits tighter than a page one.
    pub posture: PageHeaderPosture,
    /// Whether the subtitle stays visible when breadcrumbs are present.
    pub shows_subtitle_with_breadcrumbs: bool,
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
            banner_tone: StatusTone::Warning,
            align: PageHeaderAlign::default(),
            aria_label: None,
            entity_detail_posture: false,
            level: 2,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
            posture: PageHeaderPosture::Default,
            shows_subtitle_with_breadcrumbs: false,
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

    pub fn with_entity_detail_posture(mut self, entity_detail: bool) -> Self {
        self.entity_detail_posture = entity_detail;
        self
    }

    pub fn has_back_link(&self) -> bool {
        self.back_href.is_some() && self.back_label.is_some()
    }

    /// `Boolean(section && title)` — a two-level header hierarchy.
    pub fn has_section_title_split(&self) -> bool {
        self.section.is_some() && !self.title.trim().is_empty()
    }

    /// `entity_detail_posture && has_section_title_split` — when true the
    /// section/title roles swap.
    pub fn is_entity_detail_posture(&self) -> bool {
        self.entity_detail_posture && self.has_section_title_split()
    }

    /// The primary heading text. In entity-detail posture the section becomes
    /// the heading; otherwise the title (falling back to section).
    pub fn primary_title(&self) -> String {
        if self.is_entity_detail_posture() {
            self.section.clone().unwrap_or_else(|| self.title.clone())
        } else if !self.title.trim().is_empty() {
            self.title.clone()
        } else {
            self.section.clone().unwrap_or_default()
        }
    }

    /// The resolved subtitle. In entity-detail posture the title drops into the
    /// subtitle slot; otherwise the explicit subtitle is used.
    pub fn resolved_subtitle(&self) -> Option<String> {
        if self.is_entity_detail_posture() {
            Some(self.title.clone()).or_else(|| self.subtitle.clone())
        } else {
            self.subtitle.clone()
        }
    }

    /// Resolve the back-link display label: trims, strips a leading
    /// `"Back"` / `"Back to "` prefix (case-insensitive), defaults to `"Back"`.
    pub fn back_display_label(&self) -> String {
        let raw = self.back_label.clone().unwrap_or_default();
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return "Back".to_string();
        }
        let lower = trimmed.to_lowercase();
        let stripped = if let Some(rest) = lower.strip_prefix("back to ") {
            trimmed[trimmed.len() - rest.len()..].trim()
        } else if let Some(rest) = lower.strip_prefix("back ") {
            trimmed[trimmed.len() - rest.len()..].trim()
        } else {
            trimmed
        };
        if stripped.is_empty() {
            "Back".to_string()
        } else {
            stripped.to_string()
        }
    }

    /// Resolve the back-link aria label: `"Back"` or `"Back to {label}"`.
    pub fn back_aria_label(&self) -> String {
        let display = self.back_display_label();
        if display == "Back" {
            "Back".to_string()
        } else {
            format!("Back to {display}")
        }
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

    /// Color of the contextual back-link dot. Matches Svelte
    /// `--poodle-color-status-success`.
    pub fn context_dot_color_token(&self) -> &'static str {
        semantic::COLOR_STATUS_SUCCESS
    }

    /// Section label color (`text-secondary`).
    pub fn section_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    /// Base heading-size token; renderers scale it per `level`.
    pub fn heading_size_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_HEADING_SIZE
    }

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_MD
    }

    /// Gap between the title text and the inline count pill (`space.inline.sm`).
    pub fn title_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }

    /// Gap inside the title block between stacked rows (`space.inline.sm`).
    pub fn title_block_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }

    /// Gap between action buttons in the actions row (`space.inline.sm`,
    /// closest token to Svelte `0.375rem`).
    pub fn actions_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }

    pub fn header_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_MD
    }

    pub fn padding_y_token(&self) -> &'static str {
        semantic::SPACE_PANEL_Y
    }

    /// Radius for the banner callout block.
    pub fn banner_radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
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
