use pug_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrderBySpec {
    pub columns: Vec<String>,
    pub active_column: Option<String>,
    pub direction: Option<String>,
}

impl OrderBySpec {
    pub fn new(columns: Vec<String>) -> Self {
        Self {
            columns,
            active_column: None,
            direction: None,
        }
    }

    pub fn with_active_column(mut self, active_column: impl Into<String>) -> Self {
        self.active_column = Some(active_column.into());
        self
    }

    pub fn with_direction(mut self, direction: impl Into<String>) -> Self {
        self.direction = Some(direction.into());
        self
    }

    pub fn is_active(&self) -> bool {
        self.active_column.is_some()
    }

    pub fn icon_color_token(&self) -> &'static str {
        if self.is_active() {
            semantic::COLOR_ICON_PRIMARY
        } else {
            semantic::COLOR_ICON_MUTED
        }
    }

    pub fn text_color_token(&self) -> &'static str {
        if self.is_active() {
            semantic::COLOR_TEXT_PRIMARY
        } else {
            semantic::COLOR_TEXT_SECONDARY
        }
    }
}
