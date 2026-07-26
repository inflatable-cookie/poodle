use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MarkdownEditorSpec {
    pub value: String,
    pub name: Option<String>,
    pub placeholder: Option<String>,
    pub mode: String,
    pub is_disabled: bool,
    pub is_required: bool,
    pub aria_label: String,
    pub min_height: Option<String>,
    pub render_html_label: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl MarkdownEditorSpec {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            name: None,
            placeholder: None,
            mode: String::from("edit"),
            is_disabled: false,
            is_required: false,
            aria_label: String::from("Markdown editor"),
            min_height: None,
            render_html_label: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn with_mode(mut self, mode: impl Into<String>) -> Self {
        self.mode = mode.into();
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_required(mut self, is_required: bool) -> Self {
        self.is_required = is_required;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = aria_label.into();
        self
    }

    pub fn with_min_height(mut self, min_height: impl Into<String>) -> Self {
        self.min_height = Some(min_height.into());
        self
    }

    pub fn with_render_html_label(mut self, render_html_label: impl Into<String>) -> Self {
        self.render_html_label = Some(render_html_label.into());
        self
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn toolbar_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }

    pub fn tool_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn tool_hover_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn tool_hover_fill_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn textarea_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn placeholder_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_TERTIARY
    }

    pub fn preview_empty_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_TERTIARY
    }

    pub fn split_divider_color_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn toolbar_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    /// Effective control size after resolving the semantic size role.
    /// Mirrors `presentation::resolve_semantic_size`; kept here as a pure
    /// helper so size-driven contract tables can be unit-tested in poodle-specs.
    pub fn effective_size(&self) -> ControlSize {
        resolve_semantic_size(self.size, self.size_role)
    }

    /// Tool button width/height in rem, by effective size (contract §8 Size table).
    pub fn tool_size_rem(&self) -> f32 {
        match self.effective_size() {
            ControlSize::Xs => 1.5,
            ControlSize::Sm => 1.75,
            ControlSize::Md => 2.0,
            ControlSize::Lg => 2.25,
            ControlSize::Xl => 2.5,
        }
    }

    /// Mode-switcher horizontal padding in rem, by effective size (contract §8 "Mode X").
    pub fn mode_x_rem(&self) -> f32 {
        match self.effective_size() {
            ControlSize::Xs => 0.375,
            ControlSize::Sm => 0.5,
            ControlSize::Md => 0.5,
            ControlSize::Lg => 0.625,
            ControlSize::Xl => 0.75,
        }
    }

    /// Toolbar vertical padding in rem, by density (contract §8 "Toolbar Y").
    pub fn toolbar_y_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.25,
            ControlDensity::Default => 0.375,
            ControlDensity::Comfortable => 0.5,
        }
    }

    /// Toolbar horizontal padding in rem, by density (contract §8 "Toolbar X").
    pub fn toolbar_x_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.375,
            ControlDensity::Default => 0.5,
            ControlDensity::Comfortable => 0.625,
        }
    }

    /// Tool/mode gap in rem, by density (contract §8 "Tool gap").
    pub fn tool_gap_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.0625,
            ControlDensity::Default => 0.125,
            ControlDensity::Comfortable => 0.1875,
        }
    }

    /// Mode-switcher vertical padding in rem, by density (contract §8 "Mode Y").
    pub fn mode_y_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.125,
            ControlDensity::Default => 0.1875,
            ControlDensity::Comfortable => 0.25,
        }
    }

    /// Editor/preview pane padding in rem, by density (contract §8 "Pane X / Pane Y").
    pub fn pane_pad_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.625,
            ControlDensity::Default => 0.75,
            ControlDensity::Comfortable => 0.875,
        }
    }

    /// Min-height of the textarea in rem. Parses the `minHeight` prop as a rem or
    /// px CSS length; defaults to the contract `"12rem"`.
    pub fn min_height_rem(&self) -> f32 {
        match self.min_height.as_deref() {
            None => 12.0,
            Some(raw) => {
                let raw = raw.trim();
                if let Some(v) = raw.strip_suffix("rem") {
                    v.trim().parse::<f32>().unwrap_or(12.0)
                } else if let Some(v) = raw.strip_suffix("px") {
                    v.trim().parse::<f32>().map(|px| px / 16.0).unwrap_or(12.0)
                } else {
                    raw.parse::<f32>().unwrap_or(12.0)
                }
            }
        }
    }

    /// Whether the textarea/edit pane is shown for the current mode.
    pub fn shows_editor(&self) -> bool {
        self.mode == "edit" || self.mode == "split"
    }

    /// Whether the preview pane is shown for the current mode.
    pub fn shows_preview(&self) -> bool {
        self.mode == "preview" || self.mode == "split"
    }

    /// Whether toolbar formatting tools are disabled (disabled prop or preview mode).
    pub fn tools_disabled(&self) -> bool {
        self.is_disabled || self.mode == "preview"
    }

    /// Character count of the current value (specimen status copy).
    pub fn char_count(&self) -> usize {
        self.value.chars().count()
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

impl Default for MarkdownEditorSpec {
    fn default() -> Self {
        Self::new()
    }
}

/// Resolve a semantic size role against a base size (chrome → one stop smaller,
/// prominent → one stop larger). Mirrors `presentation::resolve_semantic_size`.
fn resolve_semantic_size(size: ControlSize, role: SemanticControlSizeRole) -> ControlSize {
    crate::types::resolve_semantic_control_size(size, role)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_contract() {
        let s = MarkdownEditorSpec::new();
        assert_eq!(s.mode, "edit");
        assert_eq!(s.aria_label, "Markdown editor");
        assert!(!s.is_disabled);
        assert_eq!(s.min_height_rem(), 12.0); // contract default "12rem"
    }

    #[test]
    fn tool_size_tracks_size_table() {
        let xs = MarkdownEditorSpec::new().with_size(ControlSize::Xs);
        let xl = MarkdownEditorSpec::new().with_size(ControlSize::Xl);
        assert_eq!(xs.tool_size_rem(), 1.5);
        assert_eq!(xl.tool_size_rem(), 2.5);
    }

    #[test]
    fn density_spacing_tracks_table() {
        let compact = MarkdownEditorSpec::new().with_density(ControlDensity::Compact);
        let comfortable = MarkdownEditorSpec::new().with_density(ControlDensity::Comfortable);
        assert_eq!(compact.toolbar_y_rem(), 0.25);
        assert_eq!(compact.tool_gap_rem(), 0.0625);
        assert_eq!(comfortable.pane_pad_rem(), 0.875);
        assert_eq!(comfortable.tool_gap_rem(), 0.1875);
    }

    #[test]
    fn mode_visibility_flags() {
        assert!(MarkdownEditorSpec::new().with_mode("edit").shows_editor());
        assert!(!MarkdownEditorSpec::new().with_mode("edit").shows_preview());
        assert!(MarkdownEditorSpec::new().with_mode("preview").shows_preview());
        assert!(!MarkdownEditorSpec::new().with_mode("preview").shows_editor());
        let split = MarkdownEditorSpec::new().with_mode("split");
        assert!(split.shows_editor() && split.shows_preview());
    }

    #[test]
    fn tools_disabled_in_preview_or_when_disabled() {
        assert!(MarkdownEditorSpec::new().with_mode("preview").tools_disabled());
        assert!(MarkdownEditorSpec::new().with_disabled(true).tools_disabled());
        assert!(!MarkdownEditorSpec::new().with_mode("edit").tools_disabled());
    }

    #[test]
    fn min_height_parses_rem_and_px() {
        assert_eq!(MarkdownEditorSpec::new().with_min_height("8rem").min_height_rem(), 8.0);
        assert_eq!(MarkdownEditorSpec::new().with_min_height("160px").min_height_rem(), 10.0);
    }

    #[test]
    fn char_count_counts_unicode_scalars() {
        assert_eq!(MarkdownEditorSpec::new().with_value("héllo").char_count(), 5);
    }
}
