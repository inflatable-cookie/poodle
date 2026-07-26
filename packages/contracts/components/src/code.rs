use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

/// Inline rendering variant. `Plain` drops the inline padding, radius, and
/// background so the fragment reads as bare text (contract §3/§8).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodeInlineVariant {
    Default,
    Plain,
}

/// Inline typography mode. `Inline` sets font-size `1em × adjustmentRatio` with
/// inherited line-height so the fragment matches surrounding text; `Body` uses
/// the `0.8125em × adjustmentRatio` body scale (contract §3/§8).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodeTypography {
    Body,
    Inline,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CodeSpec {
    pub content: String,
    pub language: Option<String>,
    pub show_line_numbers: bool,
    pub is_copyable: bool,
    pub highlight_lines: Vec<usize>,
    pub max_height: Option<f64>,
    pub is_inline: bool,
    /// Inline-only variant; `Plain` drops inline chrome (contract §3).
    pub inline_variant: CodeInlineVariant,
    /// Inline-only typography mode (contract §3).
    pub typography: CodeTypography,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
    /// Accessible name (contract §7). `None` falls back to the visible label.
    pub aria_label: Option<String>,
    /// Whether the copy-to-clipboard control renders.
    pub shows_copy_button: bool,
}

impl Default for CodeSpec {
    fn default() -> Self {
        Self {
            content: String::new(),
            language: None,
            show_line_numbers: false,
            is_copyable: true,
            highlight_lines: Vec::new(),
            max_height: None,
            is_inline: false,
            inline_variant: CodeInlineVariant::Default,
            typography: CodeTypography::Body,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Chrome,
            density: ControlDensity::Default,
            aria_label: None,
            shows_copy_button: true,
        }
    }
}

impl CodeSpec {
    pub fn with_copy_button(mut self, value: bool) -> Self {
        self.shows_copy_button = value;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    pub fn with_show_line_numbers(mut self, show_line_numbers: bool) -> Self {
        self.show_line_numbers = show_line_numbers;
        self
    }

    pub fn with_copyable(mut self, is_copyable: bool) -> Self {
        self.is_copyable = is_copyable;
        self
    }

    pub fn with_highlight_lines(mut self, lines: Vec<usize>) -> Self {
        self.highlight_lines = lines;
        self
    }

    pub fn with_max_height(mut self, max_height: f64) -> Self {
        self.max_height = Some(max_height);
        self
    }

    pub fn with_inline(mut self, is_inline: bool) -> Self {
        self.is_inline = is_inline;
        self
    }

    pub fn with_inline_variant(mut self, variant: CodeInlineVariant) -> Self {
        self.inline_variant = variant;
        self
    }

    pub fn with_typography(mut self, typography: CodeTypography) -> Self {
        self.typography = typography;
        self
    }

    pub fn text_secondary_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn accent_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn surface_radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    pub fn canvas_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_CANVAS
    }

    pub fn panel_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }

    pub fn elevated_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn font_family_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_CODE_FAMILY
    }

    pub fn font_size_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_CODE_SIZE
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
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
