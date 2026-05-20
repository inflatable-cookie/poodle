use poodle_tokens::semantic;

use crate::types::FormActionAlign;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormActionsSpec {
    pub align: FormActionAlign,
    pub show_top_separation: bool,
    pub show_top_border: bool,
}

impl Default for FormActionsSpec {
    fn default() -> Self {
        Self {
            align: FormActionAlign::End,
            show_top_separation: true,
            show_top_border: false,
        }
    }
}

impl FormActionsSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_align(mut self, align: FormActionAlign) -> Self {
        self.align = align;
        self
    }

    pub fn with_top_separation(mut self, show_top_separation: bool) -> Self {
        self.show_top_separation = show_top_separation;
        self
    }

    pub fn with_top_border(mut self, show_top_border: bool) -> Self {
        self.show_top_border = show_top_border;
        self
    }

    pub fn action_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_MD
    }

    pub fn stack_separation_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
    }

    pub fn shows_top_separation(&self) -> bool {
        self.show_top_separation
    }

    pub fn shows_top_border(&self) -> bool {
        self.show_top_border
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn wraps_on_narrow_widths(&self) -> bool {
        true
    }
}
