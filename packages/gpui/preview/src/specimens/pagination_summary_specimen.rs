use gpui::*;
use poodle_composites::PaginationSummarySpec;
use poodle_primitives::EyebrowSpec;
use poodle_gpui_components::{PaginationSummary, Eyebrow};
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        // --- Default ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(
                            PaginationSummary::from_spec(
                                PaginationSummarySpec::new(1, 20, 156),
                                theme,
                            )
                        )
                        .child(
                            PaginationSummary::from_spec(
                                PaginationSummarySpec::new(5, 20, 1000),
                                theme,
                            )
                        )
                )
        )
}
