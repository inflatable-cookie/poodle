/// ListContainer — paginated list view with header, state handling, and built-in pagination.
///
/// Composes PageHeader + EmptyState + Callout + Pagination + PaginationSummary
/// into a single orchestrated browse surface.
use crate::composite_types::EmptyStateVariant;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ListContainerState {
    #[default]
    Ready,
    Loading,
    Error,
    Empty,
}

#[derive(Clone, Debug, Default)]
pub struct ListContainerSpec {
    pub title: String,
    pub subtitle: Option<String>,
    pub eyebrow: Option<String>,
    pub state: ListContainerState,
    pub loading_message: Option<String>,
    pub error_title: Option<String>,
    pub error_message: Option<String>,
    pub empty_title: Option<String>,
    pub empty_message: Option<String>,
    pub current_page: usize,
    pub total_pages: usize,
    pub total_items: Option<usize>,
    pub page_size: Option<usize>,
    pub show_pagination: bool,
    /// Whether the built-in pagination summary row renders alongside
    /// pagination controls when total_items and page_size are known.
    /// Matches the contract doc's `showPaginationSummary` prop.
    pub show_pagination_summary: bool,
    /// Number of sibling pages shown on each side of the current
    /// page in the pagination UI (e.g. 1 → "…, 4, 5, 6, …").
    /// Matches `siblingCount`. Defaults to 1.
    pub sibling_count: usize,
    /// Accessible label for the pagination nav region. When None,
    /// renderers should use a sensible default (e.g. "Pagination").
    pub pagination_aria_label: Option<String>,
    /// Accessible name (contract §7). `None` falls back to the visible label.
    pub aria_label: Option<String>,
    /// Posture of the empty state this container falls back to.
    pub empty_variant: EmptyStateVariant,
}

impl ListContainerSpec {
    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            current_page: 1,
            total_pages: 1,
            show_pagination: true,
            show_pagination_summary: true,
            sibling_count: 1,
            ..Self::default()
        }
    }

    pub fn with_subtitle(mut self, v: impl Into<String>) -> Self {
        self.subtitle = Some(v.into());
        self
    }
    pub fn with_eyebrow(mut self, v: impl Into<String>) -> Self {
        self.eyebrow = Some(v.into());
        self
    }
    pub fn with_state(mut self, v: ListContainerState) -> Self {
        self.state = v;
        self
    }
    pub fn with_loading_message(mut self, v: impl Into<String>) -> Self {
        self.loading_message = Some(v.into());
        self
    }
    pub fn with_error_title(mut self, v: impl Into<String>) -> Self {
        self.error_title = Some(v.into());
        self
    }
    pub fn with_error_message(mut self, v: impl Into<String>) -> Self {
        self.error_message = Some(v.into());
        self
    }
    pub fn with_empty_title(mut self, v: impl Into<String>) -> Self {
        self.empty_title = Some(v.into());
        self
    }
    pub fn with_empty_message(mut self, v: impl Into<String>) -> Self {
        self.empty_message = Some(v.into());
        self
    }
    pub fn with_current_page(mut self, v: usize) -> Self {
        self.current_page = v;
        self
    }
    pub fn with_total_pages(mut self, v: usize) -> Self {
        self.total_pages = v;
        self
    }
    pub fn with_total_items(mut self, v: usize) -> Self {
        self.total_items = Some(v);
        self
    }
    pub fn with_page_size(mut self, v: usize) -> Self {
        self.page_size = Some(v);
        self
    }
    pub fn with_show_pagination_summary(mut self, v: bool) -> Self {
        self.show_pagination_summary = v;
        self
    }
    pub fn with_sibling_count(mut self, v: usize) -> Self {
        self.sibling_count = v;
        self
    }
    pub fn with_pagination_aria_label(mut self, v: impl Into<String>) -> Self {
        self.pagination_aria_label = Some(v.into());
        self
    }
}
