//! Drawer — Jetstream slide-out panel backed by DrawerSpec.
//!
//! Contract: `docs/contracts/foundation/drawer.md`
//! Uses overlay() with backdrop, anchored to viewport edge.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::DrawerSpec;

use crate::presentation::{
    panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size,
    resolve_supporting_visual_size, size_font_rem,
};
use crate::theme_ext::resolve_color;

pub fn js_drawer(spec: &DrawerSpec, theme: &JetstreamThemeProvider, content: Option<JsEl>) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let title_font = rem_to_px(size_font_rem(effective_size) + 0.1875);
    let icon_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(effective_size)));
    let space_x = rem_to_px(panel_space_x_rem(spec.density));
    let space_y = rem_to_px(panel_space_y_rem(spec.density));

    let fill = resolve_color(theme, spec.surface_fill_token());
    let backdrop: Color = resolve_color(theme, spec.backdrop_fill_token()).into();
    let border = resolve_color(theme, "color.border.default");
    let title_color = resolve_color(theme, "color.text.primary");

    // Close icon
    let close_icon = ui_element::icon("x")
        .w(icon_size).h(icon_size)
        .text_color(title_color)
        .cursor_pointer();

    // Drawer panel
    let mut panel = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .flex_col().gap(rem_to_px(0.75))
        .pl(space_x).pr(space_x).pt(space_y).pb(space_y)
        .h(rem_to_px(30.0)).w(rem_to_px(22.5))
        .shadow_lg();

    // Header with title and close
    let mut header = ui_element::div().flex_row().items_center().justify_between();
    if let Some(ref title) = spec.title {
        header = header.child(
            ui_element::label(title).text_color(title_color).text_size(title_font).text_weight(600)
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
