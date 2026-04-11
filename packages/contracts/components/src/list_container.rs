/// ListContainer — paginated list view with header, state handling, and built-in pagination.
///
/// Composes PageHeader + EmptyState + Callout + Pagination + PaginationSummary
/// into a single orchestrated browse surface.

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListContainerState {
    Ready,
    Loading,
    Error,
    Empty,
}

impl Default for ListContainerState {
    fn default() -> Self { Self::Ready }
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
}

impl ListContainerSpec {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            current_page: 1,
            total_pages: 1,
            show_pagination: true,
            ..Self::default()
        }
    }

    pub fn with_subtitle(mut self, v: impl Into<String>) -> Self { self.subtitle = Some(v.into()); self }
    pub fn with_eyebrow(mut self, v: impl Into<String>) -> Self { self.eyebrow = Some(v.into()); self }
    pub fn with_state(mut self, v: ListContainerState) -> Self { self.state = v; self }
    pub fn with_loading_message(mut self, v: impl Into<String>) -> Self { self.loading_message = Some(v.into()); self }
    pub fn with_error_title(mut self, v: impl Into<String>) -> Self { self.error_title = Some(v.into()); self }
    pub fn with_error_message(mut self, v: impl Into<String>) -> Self { self.error_message = Some(v.into()); self }
    pub fn with_empty_title(mut self, v: impl Into<String>) -> Self { self.empty_title = Some(v.into()); self }
    pub fn with_empty_message(mut self, v: impl Into<String>) -> Self { self.empty_message = Some(v.into()); self }
    pub fn with_current_page(mut self, v: usize) -> Self { self.current_page = v; self }
    pub fn with_total_pages(mut self, v: usize) -> Self { self.total_pages = v; self }
    pub fn with_total_items(mut self, v: usize) -> Self { self.total_items = Some(v); self }
    pub fn with_page_size(mut self, v: usize) -> Self { self.page_size = Some(v); self }
}
