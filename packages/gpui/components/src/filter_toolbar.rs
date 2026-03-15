//! PugFilterToolbar — real GPUI component backed by FilterToolbarSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_composites::FilterToolbarSpec;

use crate::theme_ext::{resolve_color, resolve_px};

/// A real GPUI filter toolbar component backed by `FilterToolbarSpec`.
///
/// Renders a horizontal bar with search query display, active filter count,
/// result count, and an optional clear action.
pub struct PugFilterToolbar {
    spec: FilterToolbarSpec,
    theme: GpuiThemeProvider,
    search_slot: Option<AnyElement>,
    filter_slot: Option<AnyElement>,
    on_clear: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl PugFilterToolbar {
    pub fn new(spec: FilterToolbarSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            search_slot: None,
            filter_slot: None,
            on_clear: None,
        }
    }

    pub fn with_search(mut self, search: impl IntoElement) -> Self {
        self.search_slot = Some(search.into_any_element());
        self
    }

    pub fn with_filter(mut self, filter: impl IntoElement) -> Self {
        self.filter_slot = Some(filter.into_any_element());
        self
    }

    pub fn on_clear(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_clear = Some(Box::new(handler));
        self
    }
}

impl IntoElement for PugFilterToolbar {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let gap = resolve_px(theme, spec.gap_token());
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let accent = resolve_color(theme, "semantic.color.accent.base");
        let border = resolve_color(theme, "semantic.color.border.subtle");

        let mut toolbar = div()
            .w_full()
            .flex()
            .items_center()
            .gap(gap)
            .px(px(12.0))
            .py(px(8.0))
            .border_b_1()
            .border_color(border);

        // Search slot or query display
        if let Some(search) = self.search_slot {
            toolbar = toolbar.child(search);
        } else if let Some(ref query) = spec.query {
            if !query.is_empty() {
                toolbar = toolbar.child(
                    div()
                        .text_sm()
                        .text_color(text_primary)
                        .child(format!("\u{1F50D} {}", query)),
                );
            }
        }

        // Filter slot
        if let Some(filter) = self.filter_slot {
            toolbar = toolbar.child(filter);
        }

        // Spacer
        toolbar = toolbar.child(div().flex_grow());

        // Active filter count badge
        if spec.active_filter_count > 0 {
            toolbar = toolbar.child(
                div()
                    .px(px(8.0))
                    .py(px(2.0))
                    .rounded(px(10.0))
                    .bg(accent.opacity(0.12))
                    .text_xs()
                    .text_color(accent)
                    .child(format!(
                        "{} filter{}",
                        spec.active_filter_count,
                        if spec.active_filter_count == 1 {
                            ""
                        } else {
                            "s"
                        }
                    )),
            );
        }

        // Result count
        if let Some(count) = spec.result_count {
            toolbar = toolbar.child(
                div()
                    .text_xs()
                    .text_color(text_secondary)
                    .child(format!(
                        "{} result{}",
                        count,
                        if count == 1 { "" } else { "s" }
                    )),
            );
        }

        // Clear action
        if spec.show_clear_action && spec.has_active_filters() {
            let clear_id = SharedString::from("filter-toolbar-clear");
            let mut clear_btn = div()
                .id(clear_id)
                .cursor_pointer()
                .text_xs()
                .text_color(accent)
                .child("Clear");

            if let Some(handler) = self.on_clear {
                clear_btn =
                    clear_btn.on_click(move |event, window, cx| handler(event, window, cx));
            }

            toolbar = toolbar.child(clear_btn);
        }

        toolbar.into_any_element()
    }
}
