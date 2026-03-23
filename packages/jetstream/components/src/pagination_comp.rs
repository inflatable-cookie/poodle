//! Pagination — Jetstream pagination backed by PaginationSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::PaginationSpec;

use crate::theme_ext::resolve_color;

pub fn js_pagination(spec: &PaginationSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let text_color = resolve_color(theme, "semantic.color.text.primary");
    let muted = resolve_color(theme, "semantic.color.text.secondary");
    let accent = resolve_color(theme, "semantic.color.accent.base");

    let mut el = ui_element::div().flex_row().items_center().gap(4.0);

    // Previous button
    el = el.child(
        ui_element::button("")
            .child(ui_element::icon("chevron-left").w(16.0).h(16.0).text_color(muted))
            .focusable()
    );

    // Page numbers
    let total = spec.total_pages;
    for page in 1..=total.min(7) {
        let is_current = page == spec.current_page;
        el = el.child(
            ui_element::button(&page.to_string())
                .text_color(if is_current { accent } else { text_color })
                .text_size(13.0)
                .text_weight(if is_current { 600 } else { 400 })
                .pl(6.0).pr(6.0)
                .focusable()
        );
    }

    // Next button
    el = el.child(
        ui_element::button("")
            .child(ui_element::icon("chevron-right").w(16.0).h(16.0).text_color(muted))
            .focusable()
    );

    el
}
