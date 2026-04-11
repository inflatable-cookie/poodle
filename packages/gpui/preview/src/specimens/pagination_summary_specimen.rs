use gpui::*;
use poodle_components::PaginationSummarySpec;
use poodle_components::EyebrowSpec;
use poodle_gpui_components::{PaginationSummary, Eyebrow};
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        // --- Default ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child(
                    PaginationSummary::from_spec(
                        PaginationSummarySpec::new(1, 20, 156),
                        theme,
                    )
                )
        )
        // --- Single page ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Single page"), theme))
                .child(
                    PaginationSummary::from_spec(
                        PaginationSummarySpec::new(1, 20, 12),
                        theme,
                    )
                )
        )
        // --- Large dataset ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Large dataset"), theme))
                .child(
                    PaginationSummary::from_spec(
                        PaginationSummarySpec::new(5, 20, 1000),
                        theme,
                    )
                )
        )
}
