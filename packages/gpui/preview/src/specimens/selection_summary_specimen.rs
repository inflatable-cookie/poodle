use gpui::*;
use poodle_composites::{SelectionSummarySpec, SelectionSummaryItem};
use poodle_primitives::EyebrowSpec;
use poodle_gpui_components::{SelectionSummary, Eyebrow};
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
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
}
