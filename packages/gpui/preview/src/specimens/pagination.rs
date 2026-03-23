use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::PaginationSpec;
use pug_gpui_components::Pagination;
use pug_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(24.0))
        // --- Default (page 1 of 10) ---
        .child(section_label("DEFAULT", text_secondary))
        .child(
            div().flex().flex_col().gap(px(4.0))
                .child(
                    Pagination::from_spec(
                        PaginationSpec::new()
                            .with_current_page(1)
                            .with_total_pages(10)
                            .with_aria_label("Page navigation"),
                        theme,
                    )
                )
                .child(
                    div().text_sm().text_color(color_to_hsla(text_secondary))
                        .child("Page 1 of 10")
                )
        )
        // --- Few pages ---
        .child(section_label("FEW PAGES (3)", text_secondary))
        .child(
            Pagination::from_spec(
                PaginationSpec::new()
                    .with_current_page(2)
                    .with_total_pages(3)
                    .with_aria_label("Short pagination"),
                theme,
            )
        )
        // --- Many pages ---
        .child(section_label("MANY PAGES (50)", text_secondary))
        .child(
            Pagination::from_spec(
                PaginationSpec::new()
                    .with_current_page(25)
                    .with_total_pages(50)
                    .with_sibling_count(2)
                    .with_aria_label("Large pagination"),
                theme,
            )
        )
        // --- First page ---
        .child(section_label("FIRST PAGE", text_secondary))
        .child(
            Pagination::from_spec(
                PaginationSpec::new()
                    .with_current_page(1)
                    .with_total_pages(20)
                    .with_aria_label("First page pagination"),
                theme,
            )
        )
        // --- Last page ---
        .child(section_label("LAST PAGE", text_secondary))
        .child(
            Pagination::from_spec(
                PaginationSpec::new()
                    .with_current_page(20)
                    .with_total_pages(20)
                    .with_aria_label("Last page pagination"),
                theme,
            )
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
