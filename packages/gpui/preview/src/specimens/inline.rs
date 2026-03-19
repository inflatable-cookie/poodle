use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{PaddingScale, SurfaceBorder, SurfaceSpec, SurfaceTone};
use pug_gpui_components::{PugInline, PugSurface};
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
        // --- Default (center-aligned) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("Default (center-aligned)", text_secondary))
                .child(
                    PugInline::new(theme)
                        .with_gap("semantic.space.inline.md")
                        .child(surface_item("Item A"))
                        .child(surface_item("Taller item B with more text"))
                        .child(surface_item("Item C"))
                )
        )
        // --- Justify: between ---
        // PugInline doesn't support justify, so use raw flex layout
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("Justify: between", text_secondary))
                .child(
                    div().flex().flex_row().items_center().justify_between()
                        .gap(px(12.0))
                        .child(surface_item("Left"))
                        .child(surface_item("Center"))
                        .child(surface_item("Right"))
                )
        )
        // --- Wrapping with small gap ---
        // PugInline doesn't support wrap, so use raw flex layout
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("Wrapping with small gap", text_secondary))
                .child(
                    div().flex().flex_row().flex_wrap().items_center()
                        .gap(px(8.0))
                        .child(surface_item("Tag 1"))
                        .child(surface_item("Tag 2"))
                        .child(surface_item("Tag 3"))
                        .child(surface_item("Tag 4"))
                        .child(surface_item("Tag 5"))
                        .child(surface_item("Tag 6"))
                        .child(surface_item("Tag 7"))
                        .child(surface_item("Tag 8"))
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
