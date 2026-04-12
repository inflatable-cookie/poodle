use crate::app_state::AppState;
use crate::specimens::specimen_layout::specimen_layout;
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, SelectionSummary};
use poodle_specs::{ControlSize, EyebrowSpec, SelectionSummaryItem, SelectionSummarySpec};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Multiple items selected"),
                    theme,
                ))
                .child(SelectionSummary::from_spec(
                    SelectionSummarySpec::new(vec![
                        SelectionSummaryItem::new("1", "Button"),
                        SelectionSummaryItem::new("2", "Card"),
                        SelectionSummaryItem::new("3", "Dialog"),
                        SelectionSummaryItem::new("4", "Table"),
                        SelectionSummaryItem::new("5", "Tabs"),
                    ]),
                    theme,
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Single item"),
                    theme,
                ))
                .child(SelectionSummary::from_spec(
                    SelectionSummarySpec::new(vec![SelectionSummaryItem::new(
                        "1",
                        "Primary button",
                    )]),
                    theme,
                )),
        )
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
                                ]),
                                theme,
                            )
                            .with_size(size)
                        }),
                    ),
                ),
        )
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
                    .with_max_visible_items(3),
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
        move |size, theme: &GpuiThemeProvider| {
            SelectionSummary::from_spec(SelectionSummarySpec::new(items()), theme)
                .with_size(size)
                .into_any_element()
        },
        move |density, theme: &GpuiThemeProvider| {
            SelectionSummary::from_spec(SelectionSummarySpec::new(items()), theme)
                .with_density(density)
                .into_any_element()
        },
    )
}
