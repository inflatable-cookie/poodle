use flint_tokens::semantic;

use crate::types::{RuleTone, SeparatorOrientation};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeparatorSpec {
    pub orientation: SeparatorOrientation,
    pub decorative: bool,
    pub tone: RuleTone,
}

impl Default for SeparatorSpec {
    fn default() -> Self {
        Self {
            orientation: SeparatorOrientation::Horizontal,
            decorative: true,
            tone: RuleTone::Subtle,
        }
    }
}

impl SeparatorSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_orientation(mut self, orientation: SeparatorOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn with_decorative(mut self, decorative: bool) -> Self {
        self.decorative = decorative;
        self
    }

    pub fn with_tone(mut self, tone: RuleTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn resolved_color(&self) -> &'static str {
        self.tone.color_token()
    }

    pub fn resolved_stroke_width(&self) -> &'static str {
        semantic::BORDER_WIDTH_DEFAULT
    }
}
