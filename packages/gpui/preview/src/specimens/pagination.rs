use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{ControlDensity, ControlSize, EyebrowSpec, PaginationSpec};
use poodle_gpui_components::{Eyebrow, Pagination};
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");

    div().flex().flex_col().gap(px(24.0))
        // --- Default ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child(
                    div().flex().flex_col().gap(px(4.0))
                        .child(
                            Pagination::from_spec(
                                PaginationSpec::new()
                                    .with_current_page(1)
                                    .with_total_pages(10)
                                    .with_aria_label("Results pagination"),
                                theme,
                            )
                        )
                        .child(
                            div().text_sm().text_color(color_to_hsla(text_secondary))
                                .child("Page 1 of 10")
                        )
                )
        )
        // --- Middle of range ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Middle of range"), theme))
                .child(
                    div().flex().flex_col().gap(px(4.0))
                        .child(
                            Pagination::from_spec(
                                PaginationSpec::new()
                                    .with_current_page(5)
                                    .with_total_pages(20)
                                    .with_sibling_count(2)
                                    .with_aria_label("Extended pagination"),
                                theme,
                            )
                        )
                        .child(
                            div().text_sm().text_color(color_to_hsla(text_secondary))
                                .child("Page 5 of 20")
                        )
                )
        )
        // --- Few pages ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Few pages"), theme))
                .child(
                    Pagination::from_spec(
                        PaginationSpec::new()
                            .with_current_page(2)
                            .with_total_pages(3)
                            .with_aria_label("Short pagination"),
                        theme,
                    )
                )
        )
        // --- Sizes ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Sizes"), theme))
                .child({
                    let sizes: &[ControlSize] = &[
                        ControlSize::Xs,
                        ControlSize::Sm,
                        ControlSize::Md,
                        ControlSize::Lg,
                        ControlSize::Xl,
                    ];
                    let mut col = div().flex().flex_col().gap(px(8.0));
                    for &size in sizes {
                        col = col.child(
                            Pagination::from_spec(
                                PaginationSpec::new()
                                    .with_current_page(3)
                                    .with_total_pages(10)
                                    .with_aria_label("Pagination size variant"),
                                theme,
                            )
                            .size(size)
                        );
                    }
                    col
                })
        )
        // --- Densities ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Densities"), theme))
                .child({
                    let densities: &[ControlDensity] = &[
                        ControlDensity::Compact,
                        ControlDensity::Default,
                        ControlDensity::Comfortable,
                    ];
                    let mut col = div().flex().flex_col().gap(px(8.0));
                    for &density in densities {
                        col = col.child(
                            Pagination::from_spec(
                                PaginationSpec::new()
                                    .with_current_page(3)
                                    .with_total_pages(10)
                                    .with_aria_label("Pagination density variant"),
                                theme,
                            )
                            .with_density(density)
                        );
                    }
                    col
                })
        )
}
