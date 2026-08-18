use crate::app_state::AppState;
use crate::node_compat::{Eyebrow, SelectionSummary};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ControlSize, EyebrowSpec, RemediationAction, SelectionSummaryItem, SelectionSummarySpec,
};
use std::sync::Arc;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Multiple items selected: count via chip row, clear control + remove/clear wired ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Multiple items selected"),
                    theme,
                ))
                .child(
                    SelectionSummary::from_spec(
                        SelectionSummarySpec::new(vec![
                            SelectionSummaryItem::new("1", "Button"),
                            SelectionSummaryItem::new("2", "Card"),
                            SelectionSummaryItem::new("3", "Dialog"),
                            SelectionSummaryItem::new("4", "Table"),
                            SelectionSummaryItem::new("5", "Tabs"),
                        ])
                        .with_clear_action(RemediationAction::new("clear", "Clear")),
                        theme,
                    )
                    .on_remove(Arc::new(|_id| {}))
                    .on_clear(Arc::new(|| {})),
                ),
        )
        // --- Empty state: reserved-height container, "No selection" placeholder ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Empty state"),
                    theme,
                ))
                .child(SelectionSummary::from_spec(
                    SelectionSummarySpec::new(vec![]),
                    theme,
                )),
        )
        // --- Single item ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Single item"),
                    theme,
                ))
                .child(
                    SelectionSummary::from_spec(
                        SelectionSummarySpec::new(vec![SelectionSummaryItem::new(
                            "1",
                            "Primary button",
                        )])
                        .with_clear_action(RemediationAction::new("clear", "Clear")),
                        theme,
                    )
                    .on_clear(Arc::new(|| {})),
                ),
        )
        // --- Sizes (all 5 of the contract size ladder) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Sizes"),
                    theme,
                ))
                .child(
                    div().flex().flex_col().gap(px(16.0)).children(
                        [
                            ControlSize::Xs,
                            ControlSize::Sm,
                            ControlSize::Md,
                            ControlSize::Lg,
                            ControlSize::Xl,
                        ]
                        .into_iter()
                        .map(|size| {
                            SelectionSummary::from_spec(
                                SelectionSummarySpec::new(vec![
                                    SelectionSummaryItem::new("1", "Button"),
                                    SelectionSummaryItem::new("2", "Card"),
                                    SelectionSummaryItem::new("3", "Dialog"),
                                ])
                                .with_clear_action(RemediationAction::new("clear", "Clear")),
                                theme,
                            )
                            .with_size(size)
                        }),
                    ),
                ),
        )
        // --- Truncated (max 3 visible): "+3 more" overflow + clear link ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Truncated (max 3 visible)"),
                    theme,
                ))
                .child(SelectionSummary::from_spec(
                    SelectionSummarySpec::new(vec![
                        SelectionSummaryItem::new("a", "Alpha"),
                        SelectionSummaryItem::new("b", "Beta"),
                        SelectionSummaryItem::new("c", "Gamma"),
                        SelectionSummaryItem::new("d", "Delta"),
                        SelectionSummaryItem::new("e", "Epsilon"),
                        SelectionSummaryItem::new("f", "Zeta"),
                    ])
                    .with_max_visible_items(3)
                    .with_clear_action(RemediationAction::new("clear", "Clear")),
                    theme,
                )),
        )
        .into_any_element();

    let items = || {
        vec![
            SelectionSummaryItem::new("1", "Button"),
            SelectionSummaryItem::new("2", "Card"),
            SelectionSummaryItem::new("3", "Dialog"),
        ]
    };

    specimen_layout(
        state,
        cx,
        "selection-summary",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(move |size, theme: &GpuiThemeProvider| {
                SelectionSummary::from_spec(SelectionSummarySpec::new(items()), theme)
                    .with_size(size)
                    .into_any_element()
            })
            .with_densities(move |density, theme: &GpuiThemeProvider| {
                SelectionSummary::from_spec(SelectionSummarySpec::new(items()), theme)
                    .with_density(density)
                    .into_any_element()
            }),
    )
}
