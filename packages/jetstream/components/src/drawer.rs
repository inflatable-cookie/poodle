//! Drawer — Jetstream slide-out panel backed by DrawerSpec.
//!
//! Contract: `docs/contracts/foundation/drawer.md`
//! Uses overlay() with backdrop, anchored to viewport edge.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::DrawerSpec;

use crate::theme_ext::resolve_color;

pub fn js_drawer(spec: &DrawerSpec, theme: &JetstreamThemeProvider, content: Option<JsEl>) -> JsEl {
    let fill = resolve_color(theme, spec.surface_fill_token());
    let backdrop: Color = resolve_color(theme, spec.backdrop_fill_token()).into();
    let border = resolve_color(theme, "semantic.color.border.default");
    let title_color = resolve_color(theme, "semantic.color.text.primary");

    // Close icon
    let close_icon = ui_element::icon("x")
        .w(16.0).h(16.0)
        .text_color(title_color)
        .cursor_pointer();

    // Drawer panel
    let mut panel = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .flex_col().gap(12.0)
        .pl(16.0).pr(16.0).pt(16.0).pb(16.0)
        .h(480.0).w(360.0)
        .shadow_lg();

    // Header with title and close
    let mut header = ui_element::div().flex_row().items_center().justify_between();
    if let Some(ref title) = spec.title {
        header = header.child(
            ui_element::label(title).text_color(title_color).text_size(16.0).text_weight(600)
        );
    }
    header = header.child(close_icon);
    panel = panel.child(header);

    if let Some(content_el) = content {
        panel = panel.child(content_el);
    }

    // Backdrop + panel as overlay
    ui_element::div()
        .bg(backdrop)
        .overlay()
        .flex_row()
        .justify_end() // drawer slides from right
        .child(panel)
}
