use poodle_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaginationSummarySpec {
    pub page: usize,
    pub page_size: usize,
    pub total_items: usize,
    /// Total page count. Derivable from `total_items / page_size`, but the
    /// contract lets a host state it directly when its pager is authoritative.
    pub total_pages: usize,
}

impl PaginationSummarySpec {
    pub fn with_total_pages(mut self, total_pages: usize) -> Self {
        self.total_pages = total_pages;
        self
    }

    pub fn new(page: usize, page_size: usize, total_items: usize) -> Self {
        Self {
            page,
            page_size,
            total_items,
            total_pages: 1,
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

    /// Visible copy — "Showing 26-50 of 67".
    pub fn summary_text(&self) -> String {
        format!(
            "Showing {}-{} of {}",
            self.start_index(),
            self.end_index(),
            self.total_items,
        )
    }

    /// Accessible name, which says more than the visible copy: it adds the page
    /// count, so a screen-reader user hears where the range sits in the whole.
    /// This is the only thing `total_pages` is for.
    pub fn accessible_label(&self) -> String {
        format!("{} across {} pages", self.summary_text(), self.total_pages)
    }

    pub fn summary_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }
}

#[cfg(test)]
mod label_tests {
    use super::*;

    /// Both strings are transcribed from the Svelte component; the visible copy
    /// and the accessible name deliberately differ.
    #[test]
    fn matches_the_svelte_strings() {
        let spec = PaginationSummarySpec::new(2, 25, 67).with_total_pages(3);
        assert_eq!(spec.summary_text(), "Showing 26-50 of 67");
        assert_eq!(spec.accessible_label(), "Showing 26-50 of 67 across 3 pages");
    }

    /// An empty result set reads "0-0 of 0", not "1-0".
    #[test]
    fn an_empty_set_starts_at_zero() {
        let spec = PaginationSummarySpec::new(1, 25, 0);
        assert_eq!(spec.summary_text(), "Showing 0-0 of 0");
    }
}
