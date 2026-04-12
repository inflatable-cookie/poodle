use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, Icon, IconProvider};
use poodle_specs::{EyebrowSpec, IconSpec};

use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(12.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content("Provider boundary"),
            theme,
        ))
        .child(
            IconProvider::new().with_child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(12.0))
                    .child(Icon::from_spec(IconSpec::new("search"), theme))
                    .child(Icon::from_spec(IconSpec::new("calendar"), theme))
                    .child(Icon::from_spec(IconSpec::new("clock"), theme)),
            ),
        )
        .child(
            div()
                .text_sm()
                .text_color(color_to_hsla(theme.resolve_color("color.text.secondary")))
                .child("GPUI uses a shared icon registry today, so this provider is a no-visual compatibility boundary."),
        )
}
