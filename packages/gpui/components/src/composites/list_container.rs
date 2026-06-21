//! ListContainer — paginated browse shell with header and state handling.
//!
//! Contract: `docs/contracts/components/list-container.md`
//! Reference: `packages/svelte/components/src/ListContainer.svelte`
//!
//! Composes the real `PageHeader`, `Callout`, `EmptyState`, `Pagination`, and
//! `PaginationSummary` components rather than hand-rolling header / state /
//! pager chrome. Root gap is `space.stack.lg`; the filters / batch / content /
//! state / pagination regions each use `space.stack.md`, matching contract
//! §7/§8.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    CallOutSpec, CalloutAnnounceMode, EmptyStateSpec, ListContainerSpec, ListContainerState,
    PageHeaderSpec, PaginationSpec, PaginationSummarySpec, StatusTone,
};

use crate::composites::{EmptyState, PageHeader};
use crate::primitives::{Callout, Pagination, PaginationSummary};
use crate::theme_ext::resolve_px;

pub struct ListContainer {
    spec: ListContainerSpec,
    theme: GpuiThemeProvider,
    content: Option<AnyElement>,
    filters: Option<AnyElement>,
    batch: Option<AnyElement>,
    breadcrumbs: Option<AnyElement>,
    actions: Option<AnyElement>,
    on_page_change: Option<Box<dyn Fn(&usize, &mut Window, &mut App) + 'static>>,
}

impl ListContainer {
    pub fn from_spec(spec: ListContainerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            content: None,
            filters: None,
            batch: None,
            breadcrumbs: None,
            actions: None,
            on_page_change: None,
        }
    }

    pub fn with_content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    /// Host-provided filter controls (contract `filters` slot).
    pub fn with_filters(mut self, filters: impl IntoElement) -> Self {
        self.filters = Some(filters.into_any_element());
        self
    }

    /// Host-provided batch-action / selection strip (contract `batch` slot).
    pub fn with_batch(mut self, batch: impl IntoElement) -> Self {
        self.batch = Some(batch.into_any_element());
        self
    }

    /// Page breadcrumbs forwarded to the composed PageHeader.
    pub fn with_breadcrumbs(mut self, breadcrumbs: impl IntoElement) -> Self {
        self.breadcrumbs = Some(breadcrumbs.into_any_element());
        self
    }

    /// Page-level actions forwarded to the composed PageHeader.
    pub fn with_actions(mut self, actions: impl IntoElement) -> Self {
        self.actions = Some(actions.into_any_element());
        self
    }

    /// Fired when the built-in pagination requests a different page.
    pub fn on_page_change(
        mut self,
        handler: impl Fn(&usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_page_change = Some(Box::new(handler));
        self
    }
}

impl IntoElement for ListContainer {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // Contract §8: root gap = space.stack.lg between major regions;
        // region gap = space.stack.md inside filters/batch/content/state.
        let root_gap = resolve_px(theme, "space.stack.lg");
        let region_gap = resolve_px(theme, "space.stack.md");

        let mut container = div().flex().flex_col().gap(root_gap).w_full();

        // ── Header — delegated to PageHeader ──────────────────────────────────
        let mut header_spec = PageHeaderSpec::new(spec.title.clone());
        if let Some(ref subtitle) = spec.subtitle {
            header_spec = header_spec.with_subtitle(subtitle.clone());
        }
        if let Some(ref eyebrow) = spec.eyebrow {
            header_spec = header_spec.with_eyebrow(eyebrow.clone());
        }
        let mut header = PageHeader::from_spec(header_spec, theme);
        if let Some(breadcrumbs) = self.breadcrumbs {
            header = header.with_breadcrumbs(breadcrumbs);
        }
        if let Some(actions) = self.actions {
            header = header.with_actions(actions);
        }
        container = container.child(header);

        // ── State-dependent body ──────────────────────────────────────────────
        match spec.state {
            ListContainerState::Ready => {
                // Filters region (optional)
                if let Some(filters) = self.filters {
                    container = container.child(
                        div().flex().flex_col().gap(region_gap).child(filters),
                    );
                }
                // Batch region (optional)
                if let Some(batch) = self.batch {
                    container = container.child(
                        div().flex().flex_col().gap(region_gap).child(batch),
                    );
                }
                // Content region
                let mut content_region = div().flex().flex_col().gap(region_gap);
                if let Some(content) = self.content {
                    content_region = content_region.child(content);
                }
                container = container.child(content_region);

                // Pagination region — contract §4: shouldShowPagination =
                // show_pagination && state==ready && total_pages > 1.
                if spec.show_pagination && spec.total_pages > 1 {
                    let mut pager_region = div().flex().flex_col().gap(region_gap);

                    // PaginationSummary — when summary enabled and totals known.
                    if spec.show_pagination_summary {
                        if let (Some(total_items), Some(page_size)) =
                            (spec.total_items, spec.page_size)
                        {
                            let summary_spec = PaginationSummarySpec::new(
                                spec.current_page.max(1),
                                page_size.max(1),
                                total_items,
                            );
                            pager_region = pager_region.child(div().w_full().child(
                                PaginationSummary::from_spec(summary_spec, theme),
                            ));
                        }
                    }

                    let mut pagination_spec = PaginationSpec::new()
                        .with_current_page(spec.current_page.max(1))
                        .with_total_pages(spec.total_pages)
                        .with_sibling_count(spec.sibling_count);
                    pagination_spec = pagination_spec.with_aria_label(
                        spec.pagination_aria_label
                            .clone()
                            .unwrap_or_else(|| "List pagination".to_string()),
                    );
                    let mut pagination = Pagination::from_spec(pagination_spec, theme);
                    if let Some(handler) = self.on_page_change {
                        pagination = pagination.on_page_change(handler);
                    }
                    // Contract §8: pagination controls justify to the end.
                    pager_region = pager_region.child(
                        div().flex().justify_end().w_full().child(pagination),
                    );

                    container = container.child(pager_region);
                }
            }
            ListContainerState::Loading => {
                // Contract: Callout tone="pending", message={loadingMessage}.
                let msg = spec
                    .loading_message
                    .clone()
                    .unwrap_or_else(|| "Loading items...".to_string());
                let callout = Callout::from_spec(
                    CallOutSpec::new().with_tone(StatusTone::Pending).with_content(msg),
                    theme,
                );
                container = container
                    .child(div().flex().flex_col().gap(region_gap).child(callout));
            }
            ListContainerState::Error => {
                // Contract: Callout tone="danger", title={errorTitle},
                // message={errorMessage}, announceMode="assertive".
                let title = spec
                    .error_title
                    .clone()
                    .unwrap_or_else(|| "Unable to load list".to_string());
                let mut callout_spec = CallOutSpec::new()
                    .with_tone(StatusTone::Danger)
                    .with_title(title)
                    .with_announce_mode(CalloutAnnounceMode::Assertive);
                if let Some(ref msg) = spec.error_message {
                    callout_spec = callout_spec.with_content(msg.clone());
                }
                let callout = Callout::from_spec(callout_spec, theme);
                container = container
                    .child(div().flex().flex_col().gap(region_gap).child(callout));
            }
            ListContainerState::Empty => {
                // Contract: EmptyState title/message/variant.
                let title = spec
                    .empty_title
                    .clone()
                    .unwrap_or_else(|| "Nothing here yet".to_string());
                let mut empty_spec = EmptyStateSpec::new(title);
                if let Some(ref msg) = spec.empty_message {
                    empty_spec = empty_spec.with_message(msg.clone());
                }
                let empty = EmptyState::from_spec(empty_spec, theme);
                container = container
                    .child(div().flex().flex_col().gap(region_gap).child(empty));
            }
        }

        container.into_any_element()
    }
}
