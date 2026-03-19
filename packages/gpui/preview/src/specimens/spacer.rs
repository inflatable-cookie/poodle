use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{PaddingScale, SurfaceBorder, SurfaceSpec, SurfaceTone};
use pug_gpui_components::{PugInline, PugSpacer, PugSurface};
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let surface_item = |label: &str| {
        let spec = SurfaceSpec::new()
            .with_tone(SurfaceTone::Panel)
            .with_border(SurfaceBorder::Subtle)
            .with_padding(PaddingScale::Sm);
        PugSurface::new(spec, theme)
            .with_content(
                div().text_sm().text_color(color_to_hsla(text_secondary))
                    .child(label.to_string()),
            )
    };

    div().flex().flex_col().gap(px(24.0))
        // --- Push items apart ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("Push items apart", text_secondary))
                .child(
                    PugInline::new(theme)
                        .with_gap("semantic.space.inline.md")
                        .child(surface_item("Logo"))
                        .child(PugSpacer::new())
                        .child(surface_item("Sign in"))
                )
        )
        // --- Between three items ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("Between three items", text_secondary))
                .child(
                    PugInline::new(theme)
                        .with_gap("semantic.space.inline.md")
                        .child(surface_item("Left"))
                        .child(PugSpacer::new())
                        .child(surface_item("Center"))
                        .child(PugSpacer::new())
                        .child(surface_item("Right"))
                )
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
