use pug_gpui_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaginationSummarySpec {
    pub page: usize,
    pub page_size: usize,
    pub total_items: usize,
}

impl PaginationSummarySpec {
    pub fn new(page: usize, page_size: usize, total_items: usize) -> Self {
        Self {
            page,
            page_size,
            total_items,
        }
    }

    pub fn start_index(&self) -> usize {
        if self.total_items == 0 {
            0
        } else {
            ((self.page.saturating_sub(1)) * self.page_size) + 1
        }
    }

    pub fn end_index(&self) -> usize {
        usize::min(self.page * self.page_size, self.total_items)
    }

    pub fn summary_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }
}
