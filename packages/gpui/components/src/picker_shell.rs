//! PugPickerShell — real GPUI component backed by PickerShellSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_composites::{PickerShellSpec, BrowseState, SelectionMode};

use crate::theme_ext::{resolve_color, resolve_px};

/// A real GPUI picker shell component backed by `PickerShellSpec`.
///
/// Renders a searchable selection panel with a header (title, description),
/// search input area, result list slot, and a footer showing selection state.
pub struct PugPickerShell {
    spec: PickerShellSpec,
    theme: GpuiThemeProvider,
    search_slot: Option<AnyElement>,
    results_slot: Option<AnyElement>,
    actions_slot: Option<AnyElement>,
}

impl PugPickerShell {
    pub fn new(spec: PickerShellSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            search_slot: None,
            results_slot: None,
            actions_slot: None,
        }
    }

    pub fn with_search(mut self, search: impl IntoElement) -> Self {
        self.search_slot = Some(search.into_any_element());
        self
    }

    pub fn with_results(mut self, results: impl IntoElement) -> Self {
        self.results_slot = Some(results.into_any_element());
        self
    }

    pub fn with_actions(mut self, actions: impl IntoElement) -> Self {
        self.actions_slot = Some(actions.into_any_element());
        self
    }
}

impl IntoElement for PugPickerShell {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let footer_gap = resolve_px(theme, spec.footer_gap_token());
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let border = resolve_color(theme, "semantic.color.border.subtle");
        let bg = resolve_color(theme, "semantic.color.background.elevated");
        let accent = resolve_color(theme, "semantic.color.accent.base");

        let mut shell = div()
            .flex()
            .flex_col()
            .bg(bg)
            .border_1()
            .border_color(border)
            .rounded(px(6.0))
            .overflow_hidden()
            .when(spec.is_modal_like(), |el| {
                el.shadow_lg().min_w(px(320.0)).max_w(px(480.0))
            });

        // Header
        let mut header = div()
            .w_full()
            .px(px(12.0))
            .py(px(10.0))
            .flex()
            .flex_col()
            .gap(px(2.0))
            .border_b_1()
            .border_color(border);

        header = header.child(
            div()
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(text_primary)
                .child(spec.title.clone()),
        );

        if let Some(ref description) = spec.description {
            header = header.child(
                div()
                    .text_xs()
                    .text_color(text_secondary)
                    .child(description.clone()),
            );
        }

        shell = shell.child(header);

        // Search area
        if let Some(search) = self.search_slot {
            shell = shell.child(
                div()
                    .w_full()
                    .px(px(12.0))
                    .py(px(8.0))
                    .border_b_1()
                    .border_color(border)
                    .child(search),
            );
        }

        // Results area
        let results_area = match spec.state {
            BrowseState::Loading => div()
                .w_full()
                .flex()
                .items_center()
                .justify_center()
                .py(px(24.0))
                .child(
                    div()
                        .text_sm()
                        .text_color(text_secondary)
                        .child("Searching\u{2026}"),
                ),
            BrowseState::Error => div()
                .w_full()
                .flex()
                .items_center()
                .justify_center()
                .py(px(24.0))
                .child(
                    div()
                        .text_sm()
                        .text_color(resolve_color(theme, "semantic.color.status.danger"))
                        .child("Search failed."),
                ),
            BrowseState::Empty | BrowseState::NoResults => div()
                .w_full()
                .flex()
                .items_center()
                .justify_center()
                .py(px(24.0))
                .child(
                    div()
                        .text_sm()
                        .text_color(text_secondary)
                        .child("No results found."),
                ),
            BrowseState::Ready => {
                let mut area = div()
                    .w_full()
                    .max_h(px(240.0))
                    .overflow_hidden();
                if let Some(results) = self.results_slot {
                    area = area.child(results);
                }
                area
            }
        };

        shell = shell.child(results_area);

        // Footer with selection info and actions
        let mut footer = div()
            .w_full()
            .flex()
            .items_center()
            .gap(footer_gap)
            .px(px(12.0))
            .py(px(8.0))
            .border_t_1()
            .border_color(border);

        // Selection count
        let selection_label = match spec.selection_mode {
            SelectionMode::Single => {
                if spec.selected_count > 0 {
                    String::from("1 selected")
                } else {
                    String::from("None selected")
                }
            }
            SelectionMode::Multiple => {
                format!("{} selected", spec.selected_count)
            }
        };

        footer = footer.child(
            div()
                .text_xs()
                .text_color(text_secondary)
                .child(selection_label),
        );

        // Result count
        if let Some(count) = spec.result_count {
            footer = footer.child(
                div()
                    .text_xs()
                    .text_color(text_secondary.opacity(0.7))
                    .child(format!("{} result{}", count, if count == 1 { "" } else { "s" })),
            );
        }

        // Spacer
        footer = footer.child(div().flex_grow());

        // Actions slot
        if let Some(actions) = self.actions_slot {
            footer = footer.child(actions);
        }

        shell = shell.child(footer);

        shell.into_any_element()
    }
}
