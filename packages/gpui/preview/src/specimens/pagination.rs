use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{EyebrowSpec, PaginationSpec, PaginationVariant};
use poodle_gpui_components::{Eyebrow, Pagination};
use crate::app_state::AppState;
use crate::specimens::specimen_layout::specimen_layout;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let examples = div().flex().flex_col().gap(px(24.0))
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
        // --- Simple variant with info and page size ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Simple variant with info and page size"), theme))
                .child(
                    Pagination::from_spec(
                        PaginationSpec::new()
                            .with_current_page(4)
                            .with_total_pages(12)
                            .with_variant(PaginationVariant::Simple)
                            .with_info_text("Showing 31–40 of 112")
                            .with_page_size(10)
                            .with_aria_label("Simple pagination"),
                        theme,
                    )
                )
        )
        // --- Full variant ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Full variant"), theme))
                .child(
                    Pagination::from_spec(
                        PaginationSpec::new()
                            .with_current_page(6)
                            .with_total_pages(25)
                            .with_sibling_count(1)
                            .with_variant(PaginationVariant::Full)
                            .with_info_text("Showing 51–60 of 248")
                            .with_page_size(10)
                            .with_aria_label("Full pagination"),
                        theme,
                    )
                )
        )
        // --- Standalone (no container chrome) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Standalone (no container chrome)"), theme))
                .child(
                    Pagination::from_spec(
                        PaginationSpec::new()
                            .with_current_page(3)
                            .with_total_pages(8)
                            .with_standalone(true)
                            .with_aria_label("Standalone pagination"),
                        theme,
                    )
                )
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "pagination",
        examples,
        |size, theme: &GpuiThemeProvider| {
            Pagination::from_spec(
                PaginationSpec::new()
                    .with_current_page(3)
                    .with_total_pages(10)
                    .with_aria_label("Pagination"),
                theme,
            )
            .size(size)
            .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            Pagination::from_spec(
                PaginationSpec::new()
                    .with_current_page(3)
                    .with_total_pages(10)
                    .with_aria_label("Pagination"),
                theme,
            )
            .with_density(density)
            .into_any_element()
        },
    )
}
