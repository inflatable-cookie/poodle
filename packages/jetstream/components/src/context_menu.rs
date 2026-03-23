//! ContextMenu — Jetstream context menu backed by ContextMenuSpec.
//!
//! Contract: `docs/contracts/foundation/context-menu.md`
//! Uses overlay() for the menu panel.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::ContextMenuSpec;

use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_context_menu(spec: &ContextMenuSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, "semantic.color.background.elevated");
    let border = resolve_color(theme, "semantic.color.border.default");
    let radius = resolve_radius(theme, "semantic.radius.surface");
    let text_color = resolve_color(theme, "semantic.color.text.primary");

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .flex_col()
        .pt(4.0).pb(4.0)
        .min_w(160.0)
        .shadow_md()
        .overlay();

    for item in &spec.menu.items {
        // Menu items with icon if available
        let mut item_el = ui_element::div()
            .flex_row().items_center().gap(8.0)
            .pl(12.0).pr(12.0).pt(6.0).pb(6.0)
            .cursor_pointer()
            .focusable();

        item_el = item_el.child(
            ui_element::label(&item.label).text_color(text_color).text_size(13.0).grow()
        );

        el = el.child(item_el);
    }

    el
}
