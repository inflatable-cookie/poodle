use pug_tokens::semantic;

use crate::types::FormActionAlign;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormActionsSpec {
    pub align: FormActionAlign,
}

impl Default for FormActionsSpec {
    fn default() -> Self {
        Self {
            align: FormActionAlign::End,
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

    pub fn action_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_MD
    }

    pub fn stack_separation_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
    }

    pub fn wraps_on_narrow_widths(&self) -> bool {
        true
    }
}
