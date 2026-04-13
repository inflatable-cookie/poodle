use crate::app_state::AppState;
use crate::specimens::specimen_layout::specimen_layout;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, Pagination};
use poodle_specs::{EyebrowSpec, PaginationSpec, PaginationVariant};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let text_primary = theme.resolve_color("color.text.primary");

    let full_page = state
        .specimens
        .selections
        .get("pagination-full-page")
        .copied()
        .unwrap_or(6);
    let full_ps = state
        .specimens
        .selections
        .get("pagination-full-ps")
        .copied()
        .unwrap_or(10);
    let full_goto = state
        .specimens
        .text
        .get("pagination-full-goto")
        .cloned()
        .unwrap_or_else(|| full_page.to_string());
    let full_limit_open = state.specimens.is_on("pagination-full-limit-open");
    let total_items = 248usize;
    let showing_from = (full_page.saturating_sub(1)).saturating_mul(full_ps) + 1;
    let showing_to = (full_page * full_ps).min(total_items);
    let full_info = format!(
        "Showing {}–{} of {}",
        showing_from, showing_to, total_items
    );

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Default ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Default"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child(Pagination::from_spec(
                            PaginationSpec::new()
                                .with_current_page(1)
                                .with_total_pages(10)
                                .with_aria_label("Results pagination"),
                            theme,
                        ))
                        .child(
                            div()
                                .text_sm()
                                .text_color(color_to_hsla(text_secondary))
                                .child("Page 1 of 10"),
                        ),
                ),
        )
        // --- Middle of range ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Middle of range"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child(Pagination::from_spec(
                            PaginationSpec::new()
                                .with_current_page(5)
                                .with_total_pages(20)
                                .with_sibling_count(2)
                                .with_aria_label("Extended pagination"),
                            theme,
                        ))
                        .child(
                            div()
                                .text_sm()
                                .text_color(color_to_hsla(text_secondary))
                                .child("Page 5 of 20"),
                        ),
                ),
        )
        // --- Few pages ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Few pages"),
                    theme,
                ))
                .child(Pagination::from_spec(
                    PaginationSpec::new()
                        .with_current_page(2)
                        .with_total_pages(3)
                        .with_aria_label("Short pagination"),
                    theme,
                )),
        )
        // --- Simple variant with info and page size ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Simple variant with info and page size"),
                    theme,
                ))
                .child(Pagination::from_spec(
                    PaginationSpec::new()
                        .with_current_page(4)
                        .with_total_pages(12)
                        .with_variant(PaginationVariant::Simple)
                        .with_info_text("Showing 31–40 of 112")
                        .with_page_size(10)
                        .with_aria_label("Simple pagination"),
                    theme,
                )),
        )
        // --- Full variant + limit (interactive) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Full variant + page-size limit (interactive)"),
                    theme,
                ))
                .child(
                    Pagination::from_spec(
                        PaginationSpec::new()
                            .with_current_page(full_page)
                            .with_total_pages(25)
                            .with_sibling_count(1)
                            .with_variant(PaginationVariant::Full)
                            .with_info_text(full_info.clone())
                            .with_page_size(full_ps)
                            .with_show_limit_selector(true)
                            .with_limit_options(vec![10, 25, 50])
                            .with_aria_label("Full pagination"),
                        theme,
                    )
                    .on_page_change(cx.listener(|this, page: &usize, _w, cx| {
                        this.state
                            .specimens
                            .selections
                            .insert("pagination-full-page".to_string(), *page);
                        this.state
                            .specimens
                            .text
                            .insert("pagination-full-goto".to_string(), page.to_string());
                        cx.notify();
                    }))
                    .with_goto_page_input(full_goto.clone())
                    .on_goto_input_change(cx.listener(|this, s: &str, _w, cx| {
                        this.state
                            .specimens
                            .text
                            .insert("pagination-full-goto".to_string(), s.to_string());
                        cx.notify();
                    }))
                    .limit_selector_open(full_limit_open)
                    .on_limit_open_change(cx.listener(|this, open: &bool, _w, cx| {
                        this.state
                            .specimens
                            .set_toggle("pagination-full-limit-open", *open);
                        cx.notify();
                    }))
                    .on_page_size_change(cx.listener(|this, n: &usize, _w, cx| {
                        this.state
                            .specimens
                            .selections
                            .insert("pagination-full-ps".to_string(), *n);
                        cx.notify();
                    })),
                )
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(color_to_hsla(text_primary))
                        .child(format!(
                            "Live: page {full_page}, {full_ps} / page, limit menu {}, goto draft “{full_goto}”",
                            if full_limit_open { "open" } else { "closed" }
                        )),
                ),
        )
        // --- Standalone (no container chrome) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Standalone (no container chrome)"),
                    theme,
                ))
                .child(Pagination::from_spec(
                    PaginationSpec::new()
                        .with_current_page(3)
                        .with_total_pages(8)
                        .with_standalone(true)
                        .with_aria_label("Standalone pagination"),
                    theme,
                )),
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
