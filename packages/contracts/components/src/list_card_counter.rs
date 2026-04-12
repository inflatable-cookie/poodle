/// ListCardCounter — icon + count display for use inside ListCard.
///
/// Matches docs/contracts/components/list-card-counter.md.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListCardCounterSpec {
    /// Icon name passed to the Icon primitive.
    pub icon: String,
    /// Numeric display value.
    pub count: u32,
    /// Tooltip text shown on hover. When set, the root wraps in a
    /// Tooltip primitive.
    pub tooltip: Option<String>,
    /// When set, the counter renders as a link. Clicks call
    /// stopPropagation to prevent the parent ListCard click from
    /// firing.
    pub href: Option<String>,
}

impl ListCardCounterSpec {
    pub fn new(icon: impl Into<String>, count: u32) -> Self {
        Self {
            icon: icon.into(),
            count,
            tooltip: None,
            href: None,
        }
    }

    pub fn with_tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    pub fn is_linked(&self) -> bool {
        self.href.is_some()
    }
}
