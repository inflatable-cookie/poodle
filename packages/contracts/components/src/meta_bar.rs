/// MetaBar — compact inline metadata row with optional separators.
///
/// Lays out MetaItem, Pill, Code, or arbitrary children in a
/// wrapping inline row. Matches docs/contracts/components/meta-bar.md.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetaBarSpec {
    pub aria_label: Option<String>,
    /// Insert subtle separator dots between adjacent child items.
    pub show_separators: bool,
}

impl Default for MetaBarSpec {
    fn default() -> Self {
        Self {
            aria_label: None,
            show_separators: true,
        }
    }
}

impl MetaBarSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn with_show_separators(mut self, v: bool) -> Self {
        self.show_separators = v;
        self
    }
}
