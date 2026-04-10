use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{EyebrowSpec, PaddingScale, SurfaceBorder, SurfaceSpec, SurfaceTone};
use poodle_gpui_components::{Eyebrow, Spacer, Surface};
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");

    let surface_item = |label: &str| {
        let spec = SurfaceSpec::new()
            .with_tone(SurfaceTone::Panel)
            .with_border(SurfaceBorder::Subtle)
            .with_padding(PaddingScale::Sm);
        Surface::from_spec(spec, theme)
            .with_content(
                div().text_sm().text_color(color_to_hsla(text_secondary))
                    .child(label.to_string()),
            )
    };

    div().flex().flex_col().gap(px(24.0))
        // --- Push items apart ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Push items apart"), theme))
                .child(
                    div().flex().items_center().gap(px(8.0))
                        .child(surface_item("Logo"))
                        .child(Spacer::new())
                        .child(surface_item("Sign in"))
                )
        )
        // --- Between three items ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Between three items"), theme))
                .child(
                    div().flex().items_center().gap(px(8.0))
                        .child(surface_item("Left"))
                        .child(Spacer::new())
                        .child(surface_item("Center"))
                        .child(Spacer::new())
                        .child(surface_item("Right"))
                )
        )
}
