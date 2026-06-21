use crate::types::{Alignment, Inset, PaddingScale};

/// Stack layout direction.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum StackDirection {
    #[default]
    Column,
    Row,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LayoutJustify {
    Start,
    End,
    Center,
    SpaceBetween,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StackSpec {
    pub direction: StackDirection,
    pub gap: PaddingScale,
    /// Cross-axis alignment. `None` resolves to a direction-aware default
    /// (`Column` → `Stretch`, `Row` → `Center`) per Svelte authority.
    pub align: Option<Alignment>,
    pub justify: Option<LayoutJustify>,
    pub wrap: bool,
    pub padding: PaddingScale,
    pub role: Option<String>,
    pub aria_label: Option<String>,
}

impl Default for StackSpec {
    fn default() -> Self {
        Self {
            direction: StackDirection::Column,
            gap: PaddingScale::Md,
            align: None,
            justify: None,
            wrap: false,
            padding: PaddingScale::None,
            role: None,
            aria_label: None,
        }
    }
}

impl StackSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_direction(mut self, direction: StackDirection) -> Self {
        self.direction = direction;
        self
    }
    pub fn with_gap(mut self, gap: PaddingScale) -> Self {
        self.gap = gap;
        self
    }
    pub fn with_align(mut self, align: Alignment) -> Self {
        self.align = Some(align);
        self
    }
    pub fn with_justify(mut self, justify: LayoutJustify) -> Self {
        self.justify = Some(justify);
        self
    }
    pub fn with_wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }
    pub fn with_padding(mut self, padding: PaddingScale) -> Self {
        self.padding = padding;
        self
    }
    pub fn with_role(mut self, role: impl Into<String>) -> Self {
        self.role = Some(role.into());
        self
    }
    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn resolved_gap(&self) -> Option<&'static str> {
        match self.direction {
            StackDirection::Column => self.gap.stack_gap(),
            StackDirection::Row => self.gap.inline_gap(),
        }
    }

    /// Cross-axis alignment with the direction-aware default applied when
    /// `align` is unset (Svelte: `column` → `stretch`, `row` → `center`).
    pub fn resolved_align(&self) -> Alignment {
        self.align.unwrap_or(match self.direction {
            StackDirection::Column => Alignment::Stretch,
            StackDirection::Row => Alignment::Center,
        })
    }

    pub fn resolved_padding(&self) -> Inset {
        self.padding.layout_inset()
    }
}
