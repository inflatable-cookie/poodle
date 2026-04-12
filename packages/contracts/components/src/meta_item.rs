/// MetaItem — a label/value pair for use inside MetaBar.
///
/// Renders an uppercase compact label with inline value content.
/// Matches docs/contracts/components/meta-item.md.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetaItemSpec {
    /// Uppercase label rendered before the value slot. When None,
    /// the value renders without a leading label.
    pub label: Option<String>,
    pub aria_label: Option<String>,
}

impl Default for MetaItemSpec {
    fn default() -> Self {
        Self {
            label: None,
            aria_label: None,
        }
    }
}

impl MetaItemSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }
}
