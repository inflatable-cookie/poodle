use poodle_tokens::semantic;

#[derive(Clone, Debug)]
pub struct RegionSpec {
    pub label: String,
    pub color: Option<String>,
    pub min_height_px: f32,
}

impl Default for RegionSpec {
    fn default() -> Self {
        Self {
            label: String::new(),
            color: None,
            min_height_px: 64.0, // 4rem = 64px
        }
    }
}

impl RegionSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn with_min_height(mut self, px: f32) -> Self {
        self.min_height_px = px;
        self
    }

    pub fn border_color_token(&self) -> &str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn label_color_token(&self) -> &str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    pub fn padding_token(&self) -> &'static str {
        semantic::SPACE_INLINE_MD
    }
}
