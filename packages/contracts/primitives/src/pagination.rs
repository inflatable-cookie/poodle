use flint_tokens::semantic;

#[derive(Debug, Clone, PartialEq)]
pub enum PageItem {
    Page(usize),
    Ellipsis,
}

#[derive(Debug, Clone)]
pub struct PaginationSpec {
    pub current_page: usize,
    pub total_pages: usize,
    pub sibling_count: usize,
    pub aria_label: Option<String>,
}

impl Default for PaginationSpec {
    fn default() -> Self {
        Self {
            current_page: 1,
            total_pages: 1,
            sibling_count: 1,
            aria_label: None,
        }
    }
}

impl PaginationSpec {
    pub fn new() -> Self {
        Self::default()
    }

    // Builder methods

    pub fn with_current_page(mut self, page: usize) -> Self {
        self.current_page = page.max(1);
        self
    }

    pub fn with_total_pages(mut self, total: usize) -> Self {
        self.total_pages = total.max(1);
        self
    }

    pub fn with_sibling_count(mut self, count: usize) -> Self {
        self.sibling_count = count;
        self
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    // Token methods

    pub fn button_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn button_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn button_text_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn current_fill_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn current_border_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn hover_fill_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    pub fn ellipsis_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    // State helpers

    pub fn is_first_page(&self) -> bool {
        self.current_page == 1
    }

    pub fn is_last_page(&self) -> bool {
        self.current_page == self.total_pages
    }

    /// Compute the visible page items: first, last, siblings around current,
    /// and ellipsis markers for gaps.
    pub fn visible_pages(&self) -> Vec<PageItem> {
        if self.total_pages <= 1 {
            return vec![PageItem::Page(1)];
        }

        let current = self.current_page.clamp(1, self.total_pages);
        let siblings = self.sibling_count;

        let sibling_start = if current > siblings + 1 {
            current - siblings
        } else {
            1
        };
        let sibling_end = (current + siblings).min(self.total_pages);

        let mut items = Vec::new();

        // Always include page 1
        items.push(PageItem::Page(1));

        // Left ellipsis if sibling range doesn't touch page 2
        if sibling_start > 2 {
            items.push(PageItem::Ellipsis);
        }

        // Sibling range (skip 1 and total_pages, they're always added)
        for page in sibling_start..=sibling_end {
            if page != 1 && page != self.total_pages {
                items.push(PageItem::Page(page));
            }
        }

        // Right ellipsis if sibling range doesn't touch second-to-last
        if sibling_end < self.total_pages - 1 {
            items.push(PageItem::Ellipsis);
        }

        // Always include last page (if different from first)
        if self.total_pages > 1 {
            items.push(PageItem::Page(self.total_pages));
        }

        items
    }
}
