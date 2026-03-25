use gpui::*;
use poodle_composites::FilterToolbarSpec;
use poodle_gpui_components::{FilterToolbar, Eyebrow};
use poodle_primitives::EyebrowSpec;
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        // -- Default --
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(
                            FilterToolbar::from_spec(
                                FilterToolbarSpec::new()
                                    .with_query("button")
                                    .with_active_filter_count(2)
                                    .with_result_count(14)
                                    .with_show_clear_action(true),
                                theme,
                            )
                        )
                        .child(
                            FilterToolbar::from_spec(
                                FilterToolbarSpec::new()
                                    .with_result_count(156),
                                theme,
                            )
                        )
                )
        )
}
