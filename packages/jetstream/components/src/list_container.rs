//! ListContainer — Jetstream paginated list view backed by ListContainerSpec.
//!
//! Contract: `docs/contracts/components/list-container.md`
//! Reference: `packages/svelte/components/src/ListContainer.svelte`
//!
//! Composes the real `PageHeader`, `Callout`, `EmptyState`, `Pagination`, and
//! `PaginationSummary` components rather than hand-rolling header/state/pager
//! chrome. Root gap is `space.stack.lg`; the filters/batch/content/state/
//! pagination regions each use `space.stack.md`, matching contract §7/§8.
//!
//! Interaction (page change via the built-in `Pagination`) lives in the preview
//! event loop — `js_pagination` renders the controls but emits no callback here.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    CallOutSpec, CalloutAnnounceMode, EmptyStateSpec, EmptyStateVariant, ListContainerSpec,
    ListContainerState, PageHeaderSpec, PaginationSpec, PaginationSummarySpec, StatusTone,
};

use crate::callout::js_callout;
use crate::empty_state::js_empty_state;
use crate::page_header::js_page_header_with_slots;
use crate::pagination_comp::js_pagination;
use crate::pagination_summary::js_pagination_summary;
use crate::theme_ext::resolve_px;

/// Render a list container with no header host slots (no breadcrumbs / actions).
/// Back-compat entry — existing 5-arg callers are unchanged; delegates to
/// [`js_list_container_with_slots`] with empty header slots.
pub fn js_list_container(
    spec: &ListContainerSpec,
    theme: &JetstreamThemeProvider,
    content: Option<JsEl>,
    filters: Option<JsEl>,
    batch: Option<JsEl>,
) -> JsEl {
    js_list_container_with_slots(spec, theme, content, filters, batch, None, None)
}

/// Render a list container with the contract's optional header host slots.
///
/// - `breadcrumbs` — page breadcrumbs forwarded into the composed PageHeader
///   (contract §2 PageHeader → Breadcrumbs; mirrors GPUI `with_breadcrumbs`).
/// - `actions` — page-level actions forwarded into the composed PageHeader
///   (contract §2 PageHeader → Actions; mirrors GPUI `with_actions`).
///
/// Both slots are host-composed `JsEl` content (not spec fields), so they
/// arrive through this entry and flow into `js_page_header_with_slots`, which is
/// where the contract anatomy places the header region.
pub fn js_list_container_with_slots(
    spec: &ListContainerSpec,
    theme: &JetstreamThemeProvider,
    content: Option<JsEl>,
    filters: Option<JsEl>,
    batch: Option<JsEl>,
    breadcrumbs: Option<JsEl>,
    actions: Option<JsEl>,
) -> JsEl {
    // Contract §8: root gap = space.stack.lg (between major regions);
    // region gap = space.stack.md (inside filters/batch/content/state).
    let root_gap = resolve_px(theme, "space.stack.lg");
    let region_gap = resolve_px(theme, "space.stack.md");

    let mut container = ui_element::div().flex_col().gap(root_gap).grow();

    // ── Header — delegated to PageHeader ──────────────────────────────────────
    let mut header_spec = PageHeaderSpec::new(spec.title.clone());
    if let Some(ref subtitle) = spec.subtitle {
        header_spec = header_spec.with_subtitle(subtitle.clone());
    }
    if let Some(ref eyebrow) = spec.eyebrow {
        header_spec = header_spec.with_eyebrow(eyebrow.clone());
    }
    // Forward host-owned breadcrumbs + actions into the PageHeader (contract §2
    // places both in the header region).
    container = container.child(js_page_header_with_slots(
        &header_spec,
        theme,
        breadcrumbs,
        actions,
        None,
    ));

    // ── State-dependent body ──────────────────────────────────────────────────
    match spec.state {
        ListContainerState::Ready => {
            // Filters region (optional)
            if let Some(filters) = filters {
                container = container.child(
                    ui_element::div().flex_col().gap(region_gap).child(filters),
                );
            }
            // Batch region (optional)
            if let Some(batch) = batch {
                container = container.child(
                    ui_element::div().flex_col().gap(region_gap).child(batch),
                );
            }
            // Content region
            let mut content_region = ui_element::div().flex_col().gap(region_gap);
            if let Some(content) = content {
                content_region = content_region.child(content);
            }
            container = container.child(content_region);

            // Pagination region — contract §4: shouldShowPagination =
            // show_pagination && state==ready && total_pages > 1.
            if spec.show_pagination && spec.total_pages > 1 {
                let mut pager_region = ui_element::div().flex_col().gap(region_gap);

                // PaginationSummary — only when summary enabled and totals known.
                if spec.show_pagination_summary {
                    if let (Some(total_items), Some(page_size)) =
                        (spec.total_items, spec.page_size)
                    {
                        let summary_spec = PaginationSummarySpec::new(
                            spec.current_page.max(1),
                            page_size.max(1),
                            total_items,
                        );
                        pager_region =
                            pager_region.child(js_pagination_summary(&summary_spec, theme));
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
                pager_region = pager_region.child(js_pagination(&pagination_spec, theme));

                container = container.child(pager_region);
            }
        }
        ListContainerState::Loading => {
            // Contract: Callout tone="pending", message={loadingMessage}.
            let msg = spec
                .loading_message
                .clone()
                .unwrap_or_else(|| "Loading items...".to_string());
            let callout = CallOutSpec::new()
                .with_tone(StatusTone::Pending)
                .with_content(msg);
            container = container.child(
                ui_element::div()
                    .flex_col()
                    .gap(region_gap)
                    .child(js_callout(&callout, theme)),
            );
        }
        ListContainerState::Error => {
            // Contract: Callout tone="danger", title={errorTitle},
            // message={errorMessage}, announceMode="assertive".
            let title = spec
                .error_title
                .clone()
                .unwrap_or_else(|| "Unable to load list".to_string());
            let mut callout = CallOutSpec::new()
                .with_tone(StatusTone::Danger)
                .with_title(title)
                .with_announce_mode(CalloutAnnounceMode::Assertive);
            if let Some(ref msg) = spec.error_message {
                callout = callout.with_content(msg.clone());
            }
            container = container.child(
                ui_element::div()
                    .flex_col()
                    .gap(region_gap)
                    .child(js_callout(&callout, theme)),
            );
        }
        ListContainerState::Empty => {
            // Contract: EmptyState title/message/variant.
            let title = spec
                .empty_title
                .clone()
                .unwrap_or_else(|| "Nothing here yet".to_string());
            let mut empty = EmptyStateSpec::new(title)
                .with_variant(empty_variant(spec));
            if let Some(ref msg) = spec.empty_message {
                empty = empty.with_message(msg.clone());
            }
            container = container.child(
                ui_element::div()
                    .flex_col()
                    .gap(region_gap)
                    .child(js_empty_state(&empty, theme)),
            );
        }
    }

    container
}

/// Resolve the EmptyState variant for the container. ListContainerSpec carries
/// no explicit variant field, so the default neutral variant is used; the
/// contract `emptyVariant` prop is host-driven through the EmptyState slot.
fn empty_variant(_spec: &ListContainerSpec) -> EmptyStateVariant {
    EmptyStateVariant::Neutral
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use jetstream_runtime::ui_element;
    use poodle_specs::ListContainerState;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    fn ready_spec() -> ListContainerSpec {
        ListContainerSpec::new("Projects")
            .with_subtitle("Manage your team's projects")
            .with_eyebrow("Settings")
            .with_current_page(2)
            .with_total_pages(5)
            .with_total_items(48)
            .with_page_size(10)
    }

    #[test]
    fn list_container_ready_header_content_and_pagination() {
        let th = theme();
        let content = ui_element::label("Row one");
        let el = js_list_container(&ready_spec(), &th, Some(content), None, None);
        let tree = probe(&el, 640.0, 480.0);

        assert!(!tree.is_empty(), "probe produced no nodes");
        // Header title + subtitle + eyebrow flow through PageHeader.
        assert!(tree.has_text("Projects"), "title missing: {:?}", tree.texts());
        assert!(
            tree.has_text("Manage your team's projects"),
            "subtitle missing: {:?}",
            tree.texts()
        );
        // Host content rendered in the Ready content region.
        assert!(tree.has_text("Row one"), "content missing: {:?}", tree.texts());
        // PaginationSummary "Showing 11 – 20 of 48" renders (page 2, size 10).
        assert!(
            tree.texts().iter().any(|t| t.contains("48") && t.contains("Showing")),
            "pagination summary missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn list_container_filters_and_batch_regions_render() {
        let th = theme();
        let filters = ui_element::label("Filter bar");
        let batch = ui_element::label("3 selected");
        let el = js_list_container(
            &ready_spec(),
            &th,
            Some(ui_element::label("Body")),
            Some(filters),
            Some(batch),
        );
        let tree = probe(&el, 640.0, 480.0);

        assert!(tree.has_text("Filter bar"), "filters slot missing: {:?}", tree.texts());
        assert!(tree.has_text("3 selected"), "batch slot missing: {:?}", tree.texts());
    }

    #[test]
    fn list_container_breadcrumbs_and_actions_slots_render() {
        // Contract §2: breadcrumbs + actions forward into the composed PageHeader.
        let th = theme();
        let breadcrumbs = ui_element::label("Home / Settings / Team");
        let actions = ui_element::label("New member");
        let el = js_list_container_with_slots(
            &ready_spec(),
            &th,
            Some(ui_element::label("Body")),
            None,
            None,
            Some(breadcrumbs),
            Some(actions),
        );
        let tree = probe(&el, 640.0, 480.0);

        assert!(
            tree.has_text("Home / Settings / Team"),
            "breadcrumbs slot missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("New member"),
            "actions slot missing: {:?}",
            tree.texts()
        );
        // Header title still renders alongside the forwarded slots.
        assert!(tree.has_text("Projects"), "title missing: {:?}", tree.texts());
    }

    #[test]
    fn list_container_loading_renders_callout() {
        let th = theme();
        let spec = ListContainerSpec::new("Projects").with_state(ListContainerState::Loading);
        let el = js_list_container(&spec, &th, None, None, None);
        let tree = probe(&el, 640.0, 480.0);

        // Default loading message flows through the pending Callout.
        assert!(
            tree.has_text("Loading items..."),
            "loading callout message missing: {:?}",
            tree.texts()
        );
        // No pagination in non-ready state.
        assert!(
            !tree.texts().iter().any(|t| t.contains("Showing")),
            "pagination summary should not render while loading"
        );
    }

    #[test]
    fn list_container_error_renders_danger_callout() {
        let th = theme();
        let spec = ListContainerSpec::new("Projects")
            .with_state(ListContainerState::Error)
            .with_error_message("A network error occurred.");
        let el = js_list_container(&spec, &th, None, None, None);
        let tree = probe(&el, 640.0, 480.0);

        assert!(
            tree.has_text("Unable to load list"),
            "error title missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("A network error occurred."),
            "error message missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn list_container_empty_renders_empty_state() {
        let th = theme();
        let spec = ListContainerSpec::new("Projects")
            .with_state(ListContainerState::Empty)
            .with_empty_message("Create your first project to get started.");
        let el = js_list_container(&spec, &th, None, None, None);
        let tree = probe(&el, 640.0, 480.0);

        assert!(
            tree.has_text("Nothing here yet"),
            "empty title missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Create your first project to get started."),
            "empty message missing: {:?}",
            tree.texts()
        );
    }
}
