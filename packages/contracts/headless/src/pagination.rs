//! Pagination machinery. Mirror of core `pagination.ts`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisiblePage {
    Page(usize),
    Ellipsis,
}

/// Page-number window: first and last pages always visible, `siblings`
/// pages around the current one, ellipsis markers for gaps of more than one
/// page. Exact mirror of the TS `buildVisiblePages` (set-based).
pub fn build_visible_pages(page: usize, count: usize, siblings: usize) -> Vec<VisiblePage> {
    if count == 0 {
        return vec![];
    }

    let mut pages: Vec<usize> = vec![1, count];

    let low = page.saturating_sub(siblings).max(1);
    let high = (page + siblings).min(count);

    for candidate in low..=high {
        pages.push(candidate);
    }

    pages.sort_unstable();
    pages.dedup();

    let mut result = Vec::new();

    for (index, current) in pages.iter().enumerate() {
        if index > 0 && current - pages[index - 1] > 1 {
            result.push(VisiblePage::Ellipsis);
        }

        result.push(VisiblePage::Page(*current));
    }

    result
}

/// A page request is valid when in bounds and actually a navigation.
pub fn can_request_page(next_page: i64, current_page: i64, total_pages: i64) -> bool {
    next_page >= 1 && next_page <= total_pages && next_page != current_page
}
