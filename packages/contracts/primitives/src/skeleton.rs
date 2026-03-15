use pug_tokens::semantic;

#[derive(Clone, Debug, PartialEq)]
pub struct SkeletonSpec {
    pub shape: String,
    pub width: Option<String>,
    pub height: Option<String>,
    pub is_animated: bool,
}

impl Default for SkeletonSpec {
    fn default() -> Self {
        Self {
            shape: String::from("rectangle"),
            width: None,
            height: None,
            is_animated: true,
        }
    }
}

impl SkeletonSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_shape(mut self, shape: impl Into<String>) -> Self {
        self.shape = shape.into();
        self
    }

    pub fn with_width(mut self, width: impl Into<String>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn with_height(mut self, height: impl Into<String>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn with_animated(mut self, is_animated: bool) -> Self {
        self.is_animated = is_animated;
        self
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn radius_token(&self) -> &'static str {
        match self.shape.as_str() {
            "circle" => semantic::RADIUS_PILL,
            "text" => semantic::RADIUS_CONTROL,
            _ => semantic::RADIUS_SURFACE,
        }
    }
}
