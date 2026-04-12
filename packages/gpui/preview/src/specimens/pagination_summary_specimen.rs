use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, PaginationSummary};
use poodle_specs::EyebrowSpec;
use poodle_specs::PaginationSummarySpec;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Default ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Default"),
                    theme,
                ))
                .child(PaginationSummary::from_spec(
                    PaginationSummarySpec::new(1, 20, 156),
                    theme,
                )),
        )
        // --- Single page ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Single page"),
                    theme,
                ))
                .child(PaginationSummary::from_spec(
                    PaginationSummarySpec::new(1, 20, 12),
                    theme,
                )),
        )
        // --- Large dataset ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Large dataset"),
                    theme,
                ))
                .child(PaginationSummary::from_spec(
                    PaginationSummarySpec::new(5, 20, 1000),
                    theme,
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Final partial page"),
                    theme,
                ))
                .child(PaginationSummary::from_spec(
                    PaginationSummarySpec::new(8, 25, 188),
                    theme,
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Empty dataset"),
                    theme,
                ))
                .child(PaginationSummary::from_spec(
                    PaginationSummarySpec::new(1, 20, 0),
                    theme,
                )),
        )
}
