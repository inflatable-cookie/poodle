//! PickerShell — real GPUI component backed by PickerShellSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_composites::{BrowseState, PickerShellSpec, PickerVariant, SelectionMode};
use poodle_primitives::{SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant};

use crate::primitives::Spinner;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI picker shell component backed by `PickerShellSpec`.
///
/// Renders a searchable selection panel with a header (title, description),
/// search input area, result list slot, and a footer showing selection state.
pub struct PickerShell {
    spec: PickerShellSpec,
    theme: GpuiThemeProvider,
    search_slot: Option<AnyElement>,
    results_slot: Option<AnyElement>,
    actions_slot: Option<AnyElement>,
}

impl std::ops::Deref for PickerShell {
    type Target = PickerShellSpec;
    fn deref(&self) -> &PickerShellSpec { &self.spec }
}

impl PickerShell {
    pub fn new(title: impl Into<String>, theme: &GpuiThemeProvider) -> Self {
        Self { spec: PickerShellSpec::new(title), theme: theme.clone(), search_slot: None, results_slot: None, actions_slot: None }
    }

    pub fn from_spec(spec: PickerShellSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            search_slot: None,
            results_slot: None,
            actions_slot: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn title(mut self, v: impl Into<String>) -> Self { self.spec.title = v.into(); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.spec.description = Some(v.into()); self }
    pub fn variant(mut self, v: PickerVariant) -> Self { self.spec.variant = v; self }
    pub fn selection_mode(mut self, v: SelectionMode) -> Self { self.spec.selection_mode = v; self }
    pub fn state(mut self, v: BrowseState) -> Self { self.spec.state = v; self }
    pub fn query(mut self, v: impl Into<String>) -> Self { self.spec.query = v.into(); self }
    pub fn result_count(mut self, v: usize) -> Self { self.spec.result_count = Some(v); self }
    pub fn selected_count(mut self, v: usize) -> Self { self.spec.selected_count = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }


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

impl IntoElement for PickerShell {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let inline_padding = resolve_px(theme, "space.inline.md");
        let gap_sm = resolve_px(theme, "space.inline.sm");
        let gap_md = resolve_px(theme, "space.inline.md");
        let gap_lg = resolve_px(theme, "space.inline.lg");
        let body_size = resolve_px(theme, "typography.body.size");
        let label_size = resolve_px(theme, "typography.label.size");
        let control_radius = resolve_radius(theme, "radius.control");
        let footer_gap = resolve_px(theme, spec.footer_gap_token());
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let border = resolve_color(theme, "color.border.subtle");
        let bg = resolve_color(theme, "color.background.elevated");
        let _accent = resolve_color(theme, "color.accent.base");

        let mut shell = div()
            .flex()
            .flex_col()
            .bg(bg)
            .border_1()
            .border_color(border)
            .rounded(control_radius)
            .overflow_hidden()
            .when(spec.is_modal_like(), |el| {
                el.shadow(vec![
                    gpui::BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.12),
                        offset: point(px(0.0), px(8.0)),
                        blur_radius: px(24.0),
                        spread_radius: px(0.0),
                    },
                    gpui::BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.08),
                        offset: point(px(0.0), px(2.0)),
                        blur_radius: px(8.0),
                        spread_radius: px(0.0),
                    },
                ]).min_w(px(320.0)).max_w(px(480.0))
            });

        // Header
        let mut header = div()
            .w_full()
            .px(inline_padding)
            .py(gap_md)
            .flex()
            .flex_col()
            .gap(px(2.0))
            .border_b_1()
            .border_color(border);

        header = header.child(
            div()
                .text_size(body_size)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(text_primary)
                .child(spec.title.clone()),
        );

        if let Some(ref description) = spec.description {
            header = header.child(
                div()
                    .text_size(label_size)
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
                    .px(inline_padding)
                    .py(gap_sm)
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
                .flex_col()
                .items_center()
                .justify_center()
                .gap(gap_sm)
                .py(gap_lg)
                .child(
                    Spinner::from_spec(
                        SpinnerSpec::new()
                            .with_variant(SpinnerVariant::Grid)
                            .with_size(SpinnerSize::Md)
                            .with_tone(SpinnerTone::Accent),
                        theme,
                    ),
                )
                .child(
                    div()
                        .text_size(body_size)
                        .text_color(text_secondary)
                        .child("Searching\u{2026}"),
                ),
            BrowseState::Error => div()
                .w_full()
                .flex()
                .items_center()
                .justify_center()
                .py(gap_lg)
                .child(
                    div()
                        .text_size(body_size)
                        .text_color(resolve_color(theme, "color.status.danger"))
                        .child("Search failed."),
                ),
            BrowseState::Empty | BrowseState::NoResults => div()
                .w_full()
                .flex()
                .items_center()
                .justify_center()
                .py(gap_lg)
                .child(
                    div()
                        .text_size(body_size)
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
            .px(inline_padding)
            .py(gap_sm)
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
                .text_size(label_size)
                .text_color(text_secondary)
                .child(selection_label),
        );

        // Result count
        if let Some(count) = spec.result_count {
            footer = footer.child(
                div()
                    .text_size(label_size)
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
