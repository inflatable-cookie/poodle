use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::RegionSpec;
use pug_gpui_components::Region;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(24.0))
        // --- Default ---
        .child(section_label("DEFAULT", text_secondary))
        .child(Region::from_spec(
            RegionSpec::new().with_label("Content area"),
            theme,
        ))

        // --- Custom colors ---
        .child(section_label("CUSTOM COLORS", text_secondary))
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

        // --- App layout mockup ---
        .child(section_label("APP LAYOUT MOCKUP", text_secondary))
        .child(
            div().flex().flex_col().gap(px(2.0))
                // Header row
                .child(Region::from_spec(
                    RegionSpec::new().with_label("App Header").with_color("#3b82f6").with_min_height(40.0),
                    theme,
                ))
                // Body row: sidebar + main + panel
                .child(
                    div().flex().gap(px(2.0)).h(px(200.0))
                        .child(
                            div().w(px(180.0)).flex_shrink_0()
                                .child(Region::from_spec(
                                    RegionSpec::new().with_label("Sidebar").with_color("#8b5cf6").with_min_height(200.0),
                                    theme,
                                ))
                        )
                        .child(
                            div().flex_grow()
                                .child(Region::from_spec(
                                    RegionSpec::new().with_label("Main Content").with_color("#10b981").with_min_height(200.0),
                                    theme,
                                ))
                        )
                        .child(
                            div().w(px(160.0)).flex_shrink_0()
                                .child(Region::from_spec(
                                    RegionSpec::new().with_label("Panel").with_color("#f59e0b").with_min_height(200.0),
                                    theme,
                                ))
                        )
                )
                // Footer
                .child(Region::from_spec(
                    RegionSpec::new().with_label("Status Bar").with_color("#6b7280").with_min_height(24.0),
                    theme,
                ))
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
