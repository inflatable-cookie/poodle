use pug_gpui_tokens::semantic;

/// Spec for NavCardGrid — a responsive grid layout container for NavCard items.
///
/// Props map directly from the contract:
/// - `columns`: 1–4 (default 2), number of grid columns
/// - `aria_label`: optional accessible name for navigation landmark
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavCardGridSpec {
    /// Number of grid columns (1–4). Clamped on construction.
    pub columns: u8,
    /// Accessible name for the navigation landmark.
    pub aria_label: Option<String>,
}

impl Default for NavCardGridSpec {
    fn default() -> Self {
        Self {
            columns: 2,
            aria_label: None,
        }
    }
}

impl NavCardGridSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_columns(mut self, columns: u8) -> Self {
        self.columns = columns.clamp(1, 4);
        self
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    /// Gap between cards — contract specifies 0.75rem.
    /// Maps to `semantic.space.inline.md` which resolves to 0.75rem.
    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_MD
    }
}
