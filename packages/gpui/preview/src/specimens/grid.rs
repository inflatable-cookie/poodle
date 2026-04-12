use crate::style_bridge::color_to_hsla;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, Grid, Surface};
use poodle_specs::{EyebrowSpec, GridSpec, PaddingScale, SurfaceBorder, SurfaceSpec, SurfaceTone};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");

    let surface_md = |label: &str| {
        let spec = SurfaceSpec::new()
            .with_tone(SurfaceTone::Panel)
            .with_border(SurfaceBorder::Subtle)
            .with_padding(PaddingScale::Md);
        Surface::from_spec(spec, theme).with_content(
            div()
                .text_sm()
                .text_color(color_to_hsla(text_secondary))
                .child(label.to_string()),
        )
    };

    let surface_sm = |label: &str| {
        let spec = SurfaceSpec::new()
            .with_tone(SurfaceTone::Panel)
            .with_border(SurfaceBorder::Subtle)
            .with_padding(PaddingScale::Sm);
        Surface::from_spec(spec, theme).with_content(
            div()
                .text_sm()
                .text_color(color_to_hsla(text_secondary))
                .child(label.to_string()),
        )
    };

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Three columns ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Three columns"),
                    theme,
                ))
                .child(
                    Grid::from_spec(
                        GridSpec::new()
                            .with_columns("1fr 1fr 1fr")
                            .with_gap(PaddingScale::Md),
                        theme,
                    )
                    .with_child(surface_md("Column 1"))
                    .with_child(surface_md("Column 2"))
                    .with_child(surface_md("Column 3")),
                ),
        )
        // --- Mixed column widths ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Mixed column widths"),
                    theme,
                ))
                .child(
                    Grid::from_spec(
                        GridSpec::new()
                            .with_columns("1fr 2fr")
                            .with_gap(PaddingScale::Md),
                        theme,
                    )
                    .with_child(surface_md("Sidebar (1fr)"))
                    .with_child(surface_md("Main content (2fr)")),
                ),
        )
        // --- Auto-fit responsive ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Auto-fit responsive"),
                    theme,
                ))
                .child(
                    Grid::from_spec(
                        GridSpec::new()
                            .with_columns("repeat(auto-fit, minmax(8rem, 1fr))")
                            .with_gap(PaddingScale::Sm),
                        theme,
                    )
                    .with_child(surface_sm("A"))
                    .with_child(surface_sm("B"))
                    .with_child(surface_sm("C"))
                    .with_child(surface_sm("D"))
                    .with_child(surface_sm("E")),
                ),
        )
}
