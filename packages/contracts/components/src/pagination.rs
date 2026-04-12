use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

#[derive(Debug, Clone, PartialEq)]
pub enum PageItem {
    Page(usize),
    Ellipsis,
}

/// Rendering variant for Pagination. Matches the Svelte `variant` prop:
/// - `Numbered` (default): numbered page buttons with first/last + ellipsis.
/// - `Simple`: just Prev/Next buttons, no numbered pages.
/// - `Full`: numbered pages plus a "Go to page" input field.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum PaginationVariant {
    #[default]
    Numbered,
    Simple,
    Full,
}

#[derive(Debug, Clone)]
pub struct PaginationSpec {
    pub current_page: usize,
    pub total_pages: usize,
    pub sibling_count: usize,
    pub aria_label: Option<String>,
    pub variant: PaginationVariant,
    /// When true the component renders without the outer panel chrome
    /// (border + background). Matches Svelte `standalone` prop.
    pub standalone: bool,
    /// Whether to show the "Showing X to Y of Z" info row.
    pub show_info: bool,
    /// Optional "showing X of Y" info text rendered alongside simple
    /// and full variants. When None and show_info is true, a default
    /// is derived from total_items / page_size / current_page.
    pub info_text: Option<String>,
    /// Total item count (not pages). Used with page_size to compute
    /// totalPages if total_pages is not explicitly set, and for the
    /// info row "Showing X of Y" text.
    pub total_items: Option<usize>,
    /// Current page size (items per page) shown alongside the info
    /// text. None suppresses the page-size selector.
    pub page_size: Option<usize>,
    /// Show a dropdown to change page size.
    pub show_limit_selector: bool,
    /// Options presented in the limit selector dropdown.
    pub limit_options: Vec<usize>,
    /// Reduced padding and gap for tight layouts.
    pub is_compact: bool,
    /// Loading state — dims the component and prevents interaction.
    pub is_loading: bool,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for PaginationSpec {
    fn default() -> Self {
        Self {
            current_page: 1,
            total_pages: 1,
            sibling_count: 1,
            aria_label: None,
            variant: PaginationVariant::Numbered,
            standalone: false,
            show_info: true,
            info_text: None,
            total_items: None,
            page_size: None,
            show_limit_selector: false,
            limit_options: vec![30, 50, 100],
            is_compact: false,
            is_loading: false,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
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

    pub fn with_variant(mut self, variant: PaginationVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_standalone(mut self, standalone: bool) -> Self {
        self.standalone = standalone;
        self
    }

    pub fn with_info_text(mut self, info_text: impl Into<String>) -> Self {
        self.info_text = Some(info_text.into());
        self
    }

    pub fn with_page_size(mut self, page_size: usize) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn with_total_items(mut self, total: usize) -> Self {
        self.total_items = Some(total);
        self
    }

    pub fn with_show_info(mut self, v: bool) -> Self {
        self.show_info = v;
        self
    }

    pub fn with_show_limit_selector(mut self, v: bool) -> Self {
        self.show_limit_selector = v;
        self
    }

    pub fn with_limit_options(mut self, options: Vec<usize>) -> Self {
        self.limit_options = options;
        self
    }

    pub fn with_compact(mut self, v: bool) -> Self {
        self.is_compact = v;
        self
    }

    pub fn with_loading(mut self, v: bool) -> Self {
        self.is_loading = v;
        self
    }

    pub fn is_simple(&self) -> bool {
        matches!(self.variant, PaginationVariant::Simple)
    }

    pub fn is_full(&self) -> bool {
        matches!(self.variant, PaginationVariant::Full)
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

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }
}
