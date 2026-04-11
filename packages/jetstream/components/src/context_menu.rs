//! ContextMenu — Jetstream context menu backed by ContextMenuSpec.
//!
//! Contract: `docs/contracts/components/context-menu.md`
//! Uses overlay() for the menu panel.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_components::ContextMenuSpec;

use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_context_menu(spec: &ContextMenuSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let fill = resolve_color(theme, "color.background.elevated");
    let border = resolve_color(theme, "color.border.default");
    let radius = resolve_radius(theme, "radius.surface");
    let text_color = resolve_color(theme, "color.text.primary");

    let font_size = rem_to_px(size_font_rem(effective_size));
    // Contract: panel padding-y 0.25rem, min-width 10rem, item padding 0.375rem 0.75rem, gap 0.5rem
    let panel_py = rem_to_px(0.25);
    let min_width = rem_to_px(10.0);
    let item_px = rem_to_px(0.75);
    let item_py = rem_to_px(0.375);
    let item_gap = rem_to_px(0.5);

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .flex_col()
        .pt(panel_py).pb(panel_py)
        .min_w(min_width)
        .shadow_md()
        .overlay();

    for item in &spec.menu.items {
        let mut item_el = ui_element::div()
            .flex_row().items_center().gap(item_gap)
            .pl(item_px).pr(item_px).pt(item_py).pb(item_py)
            .cursor_pointer()
            .focusable();

        item_el = item_el.child(
            ui_element::label(&item.label).text_color(text_color).text_size(font_size).grow()
        );

        el = el.child(item_el);
    }

    el
}
