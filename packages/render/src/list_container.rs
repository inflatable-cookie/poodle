//! ListContainer — a page-level paginated list shell.
//!
//! Contract: `docs/contracts/components/list-container.md`
//! Ported from: `packages/jetstream/components/src/list_container.rs`.
//!
//! Composes the real `page_header`, `callout`, `empty_state`, `pagination`,
//! and `pagination_summary` rather than hand-rolling header/state/pager
//! chrome. Root gap is `space.stack.lg`; the filters/batch/content/state/
//! pagination regions each use `space.stack.md`, matching contract §7/§8.

use std::sync::Arc;

use poodle_node::{LayoutDirection, LayoutSizing, Node};
use poodle_specs::{
    CallOutSpec, CalloutAnnounceMode, EmptyStateSpec, EmptyStateVariant, ListContainerSpec,
    ListContainerState, PageHeaderSpec, PaginationSpec, PaginationSummarySpec, StatusTone,
};

use crate::callout::{callout, CalloutHandlers};
use crate::context::RenderContext;
use crate::empty_state::empty_state;
use crate::page_header::page_header;
use crate::pagination::pagination;
use crate::pagination_summary::pagination_summary;

/// Host-composed slots for the list container. All optional; each is a
/// pre-built `Node` cluster (mirrors the GPUI `with_*` channels).
#[derive(Default)]
pub struct ListContainerSlots {
    pub content: Option<Node>,
    pub filters: Option<Node>,
    pub batch: Option<Node>,
    pub breadcrumbs: Option<Node>,
    pub actions: Option<Node>,
}

fn region(gap: f32, child: Node) -> Node {
    let mut r = Node::container();
    {
        let s = &mut r.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = gap;
    }
    r.child(child)
}

/// `on_page_change` fires with the destination page, from the composed
/// `pagination`.
pub fn list_container(
    spec: &ListContainerSpec,
    ctx: &RenderContext<'_>,
    slots: ListContainerSlots,
    on_page_change: Option<Arc<dyn Fn(usize) + Send + Sync>>,
) -> Node {
    // Contract §8: root gap = space.stack.lg (between major regions);
    // region gap = space.stack.md (inside filters/batch/content/state).
    let root_gap = ctx.theme().resolve_space("space.stack.lg");
    let region_gap = ctx.theme().resolve_space("space.stack.md");

    let mut container = Node::container();
    {
        let s = &mut container.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = root_gap;
        s.descriptor.layout.width = LayoutSizing::Grow;
    }

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
    container = container.child(page_header(
        &header_spec,
        ctx,
        slots.breadcrumbs,
        slots.actions,
        None,
    ));

    // ── State-dependent body ──────────────────────────────────────────────────
    match spec.state {
        ListContainerState::Ready => {
            // Filters region (optional)
            if let Some(filters) = slots.filters {
                container = container.child(region(region_gap, filters));
            }
            // Batch region (optional)
            if let Some(batch) = slots.batch {
                container = container.child(region(region_gap, batch));
            }
            // Content region
            let mut content_region = Node::container();
            {
                let s = &mut content_region.style;
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.descriptor.layout.spacing.gap = region_gap;
            }
            if let Some(content) = slots.content {
                content_region = content_region.child(content);
            }
            container = container.child(content_region);

            // Pagination region — contract §4: shouldShowPagination =
            // show_pagination && state==ready && total_pages > 1.
            if spec.show_pagination && spec.total_pages > 1 {
                let mut pager_region = Node::container();
                {
                    let s = &mut pager_region.style;
                    s.descriptor.layout.direction = LayoutDirection::Column;
                    s.descriptor.layout.spacing.gap = region_gap;
                }

                // PaginationSummary — only when summary enabled and totals known.
                if spec.show_pagination_summary {
                    if let (Some(total_items), Some(page_size)) = (spec.total_items, spec.page_size)
                    {
                        let summary_spec = PaginationSummarySpec::new(
                            spec.current_page.max(1),
                            page_size.max(1),
                            total_items,
                        );
                        let mut summary = Node::container();
                        summary.style.descriptor.layout.direction = LayoutDirection::Row;
                        summary.style.fill_width = true;
                        pager_region = pager_region
                            .child(summary.child(pagination_summary(&summary_spec, ctx)));
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
                let mut controls = Node::container();
                controls.style.descriptor.layout.direction = LayoutDirection::Row;
                controls.style.descriptor.layout.alignment.main =
                    poodle_node::MainAxisAlignment::End;
                controls.style.fill_width = true;
                pager_region = pager_region.child(controls.child(pagination(
                    &pagination_spec,
                    ctx,
                    on_page_change,
                )));

                container = container.child(pager_region);
            }
        }
        ListContainerState::Loading => {
            // Contract: Callout tone="pending", message={loadingMessage}.
            let msg = spec
                .loading_message
                .clone()
                .unwrap_or_else(|| "Loading items...".to_string());
            let spec = CallOutSpec::new()
                .with_tone(StatusTone::Pending)
                .with_content(msg);
            container = container.child(region(
                region_gap,
                callout(&spec, ctx, CalloutHandlers::default()),
            ));
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
            container = container.child(region(
                region_gap,
                callout(&callout_spec, ctx, CalloutHandlers::default()),
            ));
        }
        ListContainerState::Empty => {
            // Contract: EmptyState title/message/variant. ListContainerSpec
            // carries no explicit variant field, so neutral; the contract's
            // `emptyVariant` prop is host-driven through the EmptyState slot.
            let title = spec
                .empty_title
                .clone()
                .unwrap_or_else(|| "Nothing here yet".to_string());
            let mut empty = EmptyStateSpec::new(title).with_variant(EmptyStateVariant::Neutral);
            if let Some(ref msg) = spec.empty_message {
                empty = empty.with_message(msg.clone());
            }
            container = container.child(region(region_gap, empty_state(&empty, ctx)));
        }
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            container.a11y.label = Some(label.to_string());
        }
    }
    container
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_node::MainAxisAlignment;

    #[test]
    fn ready_pager_preserves_full_width_summary_and_end_aligned_controls() {
        let theme =
            poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);
        let ctx = RenderContext::new(&theme);
        let spec = ListContainerSpec::new("Items")
            .with_total_pages(3)
            .with_total_items(24)
            .with_page_size(10);
        let node = list_container(&spec, &ctx, ListContainerSlots::default(), None);
        let header = &node.children[0];
        let pager = node.children.last().expect("ready list renders pager");

        assert!(header.style.fill_width);
        assert_eq!(
            header.style.descriptor.layout.spacing.padding.top,
            header.style.descriptor.layout.spacing.padding.bottom
        );
        assert!(pager.children[0].style.fill_width);
        assert_eq!(
            pager.children[0].children[0].style.text_size,
            Some(poodle_adapter::ThemeProvider::resolve_space(
                &theme,
                "typography.body.size"
            ))
        );
        assert!(pager.children[1].style.fill_width);
        assert_eq!(
            pager.children[1].style.descriptor.layout.alignment.main,
            MainAxisAlignment::End
        );
    }
}
