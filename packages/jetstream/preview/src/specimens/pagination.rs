//! Pagination specimen — contract §13 specimen groups plus state coverage:
//! Default (numbered), Middle of range, Few pages, Simple+info+limit selector,
//! Full variant, chrome/standalone, boundary-disabled (first/last page),
//! compact, loading, Size ladder, Density ladder.
//!
//! Pagination renders statically here; click navigation lives in the preview
//! event loop. All visual properties resolve from PaginationSpec tokens.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::pagination_comp::js_pagination;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ControlDensity, ControlSize, PaginationSpec, PaginationVariant};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // Contract §13 Default — page 1 of 10, prev disabled, numbered + ellipsis.
        .child(group("Default", secondary,
            js_pagination(
                &PaginationSpec::new()
                    .with_current_page(1)
                    .with_total_pages(10)
                    .with_aria_label("Results pagination"),
                theme,
            )
        ))
        // Contract §13 Middle of range — siblingCount 2, both ends truncated.
        .child(group("Middle of range", secondary,
            js_pagination(
                &PaginationSpec::new()
                    .with_current_page(5)
                    .with_total_pages(20)
                    .with_sibling_count(2)
                    .with_aria_label("Extended pagination"),
                theme,
            )
        ))
        // Contract §13 Few pages — all three buttons, no ellipsis.
        .child(group("Few pages", secondary,
            js_pagination(
                &PaginationSpec::new()
                    .with_current_page(2)
                    .with_total_pages(3)
                    .with_aria_label("Short pagination"),
                theme,
            )
        ))
        // Boundary states — prev disabled at first page, next disabled at last.
        .child(group("First page (prev disabled)", secondary,
            js_pagination(
                &PaginationSpec::new()
                    .with_current_page(1)
                    .with_total_pages(8),
                theme,
            )
        ))
        .child(group("Last page (next disabled)", secondary,
            js_pagination(
                &PaginationSpec::new()
                    .with_current_page(8)
                    .with_total_pages(8),
                theme,
            )
        ))
        // Contract §13 Simple variant — item range + info + limit selector.
        .child(group("Simple variant with info and page size", secondary,
            js_pagination(
                &PaginationSpec::new()
                    .with_current_page(3)
                    .with_total_pages(10)
                    .with_variant(PaginationVariant::Simple)
                    .with_page_size(25)
                    .with_total_items(248)
                    .with_show_info(true)
                    .with_show_limit_selector(true)
                    .with_limit_options(vec![10, 25, 50, 100])
                    .with_aria_label("Simple pagination"),
                theme,
            )
        ))
        // Contract §13 Full variant — "Page X of Y" + first/last guillemets.
        .child(group("Full variant", secondary,
            js_pagination(
                &PaginationSpec::new()
                    .with_current_page(1)
                    .with_total_pages(7)
                    .with_variant(PaginationVariant::Full)
                    .with_page_size(20)
                    .with_total_items(140)
                    .with_aria_label("Full pagination"),
                theme,
            )
        ))
        // Container chrome — padding + top border + elevated bg (default here).
        .child(group("With container chrome", secondary,
            js_pagination(
                &PaginationSpec::new()
                    .with_current_page(1)
                    .with_total_pages(10)
                    .with_aria_label("Pagination with chrome"),
                theme,
            )
        ))
        // Standalone — chrome-free (no padding/border/bg).
        .child(group("Standalone (no chrome)", secondary,
            js_pagination(
                &PaginationSpec::new()
                    .with_current_page(3)
                    .with_total_pages(8)
                    .with_standalone(true)
                    .with_aria_label("Standalone pagination"),
                theme,
            )
        ))
        // Compact — tighter gap/padding.
        .child(group("Compact", secondary,
            js_pagination(
                &PaginationSpec::new()
                    .with_current_page(3)
                    .with_total_pages(10)
                    .with_compact(true)
                    .with_standalone(true),
                theme,
            )
        ))
        // Loading — whole control at reduced opacity, controls disabled.
        .child(group("Loading", secondary,
            js_pagination(
                &PaginationSpec::new()
                    .with_current_page(3)
                    .with_total_pages(10)
                    .with_loading(true)
                    .with_standalone(true),
                theme,
            )
        ))
        // Contract §13 Sizes — xs..xl button height + font ladder.
        .child(group("Sizes", secondary,
            div().flex_col().gap(12.0)
                .child(sized(ControlSize::Xs, theme))
                .child(sized(ControlSize::Sm, theme))
                .child(sized(ControlSize::Md, theme))
                .child(sized(ControlSize::Lg, theme))
                .child(sized(ControlSize::Xl, theme))
        ))
        // Contract §13 Densities — controls/pages gap ladder.
        .child(group("Densities", secondary,
            div().flex_col().gap(12.0)
                .child(densified(ControlDensity::Compact, theme))
                .child(densified(ControlDensity::Default, theme))
                .child(densified(ControlDensity::Comfortable, theme))
        ))
}

fn sized(size: ControlSize, theme: &JetstreamThemeProvider) -> JsEl {
    js_pagination(
        &PaginationSpec::new()
            .with_current_page(3)
            .with_total_pages(10)
            .with_standalone(true)
            .with_size(size),
        theme,
    )
}

fn densified(density: ControlDensity, theme: &JetstreamThemeProvider) -> JsEl {
    js_pagination(
        &PaginationSpec::new()
            .with_current_page(3)
            .with_total_pages(10)
            .with_standalone(true)
            .with_density(density),
        theme,
    )
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
