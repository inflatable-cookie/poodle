use pug_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavCardGridSpec {
    pub card_count: usize,
    pub min_card_width: Option<String>,
}

impl NavCardGridSpec {
    pub fn new(card_count: usize) -> Self {
        Self {
            card_count,
            min_card_width: None,
        }
    }

    pub fn with_min_card_width(mut self, min_card_width: impl Into<String>) -> Self {
        self.min_card_width = Some(min_card_width.into());
        self
    }

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_MD
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }
}
