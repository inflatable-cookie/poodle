use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{EyebrowSpec, RegionSpec};
use poodle_gpui_components::{Eyebrow, Region};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {

    div().flex().flex_col().gap(px(24.0))
        // --- Default ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child(Region::from_spec(
                    RegionSpec::new().with_label("Content area"),
                    theme,
                ))
        )
        // --- Custom colors ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Custom colors"), theme))
                .child(
                    div().flex().flex_col().gap(px(4.0))
                        .child(Region::from_spec(
                            RegionSpec::new().with_label("Header").with_color("#5b9bd5").with_min_height(48.0),
                            theme,
                        ))
                        .child(Region::from_spec(
                            RegionSpec::new().with_label("Sidebar").with_color("#70ad47").with_min_height(96.0),
                            theme,
                        ))
                        .child(Region::from_spec(
                            RegionSpec::new().with_label("Main content").with_color("#ed7d31").with_min_height(128.0),
                            theme,
                        ))
                        .child(Region::from_spec(
                            RegionSpec::new().with_label("Footer").with_color("#a855f7").with_min_height(48.0),
                            theme,
                        ))
                )
        )
        // --- Layout composition ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Layout composition"), theme))
                .child(
                    div().flex().gap(px(8.0)).h(px(224.0))
                        .child(
                            div().w(px(160.0)).flex_shrink_0()
                                .child(Region::from_spec(
                                    RegionSpec::new().with_label("Nav").with_color("#5b9bd5").with_min_height(224.0),
                                    theme,
                                ))
                        )
                        .child(
                            div().flex_grow().flex().flex_col().gap(px(8.0))
                                .child(Region::from_spec(
                                    RegionSpec::new().with_label("Toolbar").with_color("#70ad47").with_min_height(40.0),
                                    theme,
                                ))
                                .child(Region::from_spec(
                                    RegionSpec::new().with_label("Content").with_color("#ed7d31").with_min_height(160.0),
                                    theme,
                                ))
                        )
                )
        )
}
