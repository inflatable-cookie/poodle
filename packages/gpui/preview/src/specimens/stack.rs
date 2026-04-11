use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_components::{
    Alignment, EyebrowSpec, LayoutJustify, PaddingScale, StackDirection, StackSpec,
    SurfaceBorder, SurfaceSpec, SurfaceTone,
};
use poodle_gpui_components::{Eyebrow, Stack, Surface};
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
        // --- Column (default) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Column (default)"), theme))
                .child(
                    Stack::from_spec(StackSpec::new().with_gap(PaddingScale::Md), theme)
                        .with_child(surface_item("First item"))
                        .with_child(surface_item("Second item"))
                        .with_child(surface_item("Third item"))
                )
        )
        // --- Column — large gap, center aligned ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Column \u{2014} large gap, center aligned"), theme))
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
        // --- Row ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Row"), theme))
                .child(
                    Stack::from_spec(
                        StackSpec::new()
                            .with_direction(StackDirection::Row)
                            .with_gap(PaddingScale::Md),
                        theme,
                    )
                    .with_child(surface_item("Item A"))
                    .with_child(surface_item("Taller item B with more text"))
                    .with_child(surface_item("Item C"))
                )
        )
        // --- Row — justify: between ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Row \u{2014} justify: between"), theme))
                .child(
                    Stack::from_spec(
                        StackSpec::new()
                            .with_direction(StackDirection::Row)
                            .with_gap(PaddingScale::Md)
                            .with_justify(LayoutJustify::SpaceBetween),
                        theme,
                    )
                    .with_child(surface_item("Left"))
                    .with_child(surface_item("Center"))
                    .with_child(surface_item("Right"))
                )
        )
        // --- Row — wrapping ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Row \u{2014} wrapping"), theme))
                .child({
                    let mut stack = Stack::from_spec(
                        StackSpec::new()
                            .with_direction(StackDirection::Row)
                            .with_gap(PaddingScale::Sm)
                            .with_wrap(true),
                        theme,
                    );
                    for i in 1..=8 {
                        stack = stack.with_child(surface_item(&format!("Tag {}", i)));
                    }
                    stack
                })
        )
}
