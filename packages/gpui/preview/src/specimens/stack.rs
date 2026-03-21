use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{Alignment, PaddingScale, StackSpec, SurfaceBorder, SurfaceSpec, SurfaceTone};
use pug_gpui_components::{Stack, Surface};
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

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
        // --- Default (md gap) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("Default (md gap)", text_secondary))
                .child(
                    Stack::from_spec(StackSpec::new().with_gap(PaddingScale::Md), theme)
                        .with_child(surface_item("First item"))
                        .with_child(surface_item("Second item"))
                        .with_child(surface_item("Third item"))
                )
        )
        // --- Large gap with center alignment ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("Large gap with center alignment", text_secondary))
                .child(
                    Stack::from_spec(
                        StackSpec::new()
                            .with_gap(PaddingScale::Lg)
                            .with_align(Alignment::Center),
                        theme,
                    )
                    .with_child(surface_item("Centered A"))
                    .with_child(surface_item("Centered B"))
                )
        )
        // --- Small gap, compact ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("Small gap, compact", text_secondary))
                .child(
                    Stack::from_spec(StackSpec::new().with_gap(PaddingScale::Sm), theme)
                        .with_child(surface_item("Line 1"))
                        .with_child(surface_item("Line 2"))
                        .with_child(surface_item("Line 3"))
                        .with_child(surface_item("Line 4"))
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
