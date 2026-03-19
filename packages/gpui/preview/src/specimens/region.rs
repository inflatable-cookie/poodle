use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::RegionSpec;
use pug_gpui_components::PugRegion;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(24.0))
        // Default
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("DEFAULT", text_secondary))
                .child(PugRegion::new(
                    RegionSpec::new().with_label("Content area"),
                    theme,
                ))
        )
        // Custom colors
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("CUSTOM COLORS", text_secondary))
                .child(PugRegion::new(
                    RegionSpec::new().with_label("Header").with_color("#5b9bd5").with_min_height(48.0),
                    theme,
                ))
                .child(PugRegion::new(
                    RegionSpec::new().with_label("Sidebar").with_color("#70ad47").with_min_height(96.0),
                    theme,
                ))
                .child(PugRegion::new(
                    RegionSpec::new().with_label("Main content").with_color("#ed7d31").with_min_height(128.0),
                    theme,
                ))
                .child(PugRegion::new(
                    RegionSpec::new().with_label("Footer").with_color("#a855f7").with_min_height(48.0),
                    theme,
                ))
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(crate::style_bridge::color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
