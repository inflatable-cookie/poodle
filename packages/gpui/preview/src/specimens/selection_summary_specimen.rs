use gpui::*;
use poodle_components::{SelectionSummarySpec, SelectionSummaryItem};
use poodle_components::EyebrowSpec;
use poodle_gpui_components::{SelectionSummary, Eyebrow};
use poodle_gpui::GpuiThemeProvider;
use crate::app_state::AppState;
use crate::specimens::specimen_layout::specimen_layout;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let examples = div().flex().flex_col().gap(px(24.0))
        // --- Multiple items selected ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Multiple items selected"), theme))
                .child(
                    SelectionSummary::from_spec(
                        SelectionSummarySpec::new(vec![
                            SelectionSummaryItem::new("1", "Button"),
                            SelectionSummaryItem::new("2", "Card"),
                            SelectionSummaryItem::new("3", "Dialog"),
                            SelectionSummaryItem::new("4", "Table"),
                            SelectionSummaryItem::new("5", "Tabs"),
                        ]),
                        theme,
                    )
                )
        )
        // --- Single item ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Single item"), theme))
                .child(
                    SelectionSummary::from_spec(
                        SelectionSummarySpec::new(vec![
                            SelectionSummaryItem::new("1", "Primary button"),
                        ]),
                        theme,
                    )
                )
        )
        // --- Many items ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Many items"), theme))
                .child(
                    SelectionSummary::from_spec(
                        SelectionSummarySpec::new(vec![
                            SelectionSummaryItem::new("a", "Alpha"),
                            SelectionSummaryItem::new("b", "Beta"),
                            SelectionSummaryItem::new("c", "Gamma"),
                            SelectionSummaryItem::new("d", "Delta"),
                            SelectionSummaryItem::new("e", "Epsilon"),
                            SelectionSummaryItem::new("f", "Zeta"),
                        ]),
                        theme,
                    )
                )
        )
        // --- Truncated (max visible = 3) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Truncated (max visible = 3)"), theme))
                .child(
                    SelectionSummary::from_spec(
                        SelectionSummarySpec::new(vec![
                            SelectionSummaryItem::new("1", "Button"),
                            SelectionSummaryItem::new("2", "Card"),
                            SelectionSummaryItem::new("3", "Dialog"),
                            SelectionSummaryItem::new("4", "Select"),
                            SelectionSummaryItem::new("5", "Switch"),
                            SelectionSummaryItem::new("6", "Table"),
                            SelectionSummaryItem::new("7", "Tabs"),
                        ])
                        .with_max_visible_items(3),
                        theme,
                    )
                )
        )
        .into_any_element();

    let items = || vec![
        SelectionSummaryItem::new("1", "Button"),
        SelectionSummaryItem::new("2", "Card"),
        SelectionSummaryItem::new("3", "Dialog"),
    ];

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
