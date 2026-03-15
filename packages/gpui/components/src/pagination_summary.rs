//! PugPaginationSummary — real GPUI component backed by PaginationSummarySpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_composites::PaginationSummarySpec;

use crate::theme_ext::{resolve_color, resolve_px};

/// A real GPUI pagination summary component backed by `PaginationSummarySpec`.
///
/// Renders page information like "Showing 26-50 of 67" with optional
/// navigation slots for prev/next.
pub struct PugPaginationSummary {
    spec: PaginationSummarySpec,
    theme: GpuiThemeProvider,
    prev_slot: Option<AnyElement>,
    next_slot: Option<AnyElement>,
}

impl PugPaginationSummary {
    pub fn new(spec: PaginationSummarySpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            prev_slot: None,
            next_slot: None,
        }
    }

    pub fn with_prev(mut self, prev: impl IntoElement) -> Self {
        self.prev_slot = Some(prev.into_any_element());
        self
    }

    pub fn with_next(mut self, next: impl IntoElement) -> Self {
        self.next_slot = Some(next.into_any_element());
        self
    }
}

impl IntoElement for PugPaginationSummary {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let gap = resolve_px(theme, spec.summary_gap_token());
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");

        let start = spec.start_index();
        let end = spec.end_index();
        let total = spec.total_items;

        let summary_text = if total == 0 {
            String::from("No items")
        } else {
            format!("Showing {}\u{2013}{} of {}", start, end, total)
        };

        let mut container = div()
            .flex()
            .items_center()
            .gap(gap);

        // Prev button slot
        if let Some(prev) = self.prev_slot {
            container = container.child(prev);
        }

        // Summary label
        container = container.child(
            div()
                .text_sm()
                .text_color(text_secondary)
                .child(summary_text),
        );

        // Next button slot
        if let Some(next) = self.next_slot {
            container = container.child(next);
        }

        container.into_any_element()
    }
}
