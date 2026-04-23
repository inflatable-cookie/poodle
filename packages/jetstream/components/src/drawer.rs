//! Drawer — Jetstream slide-out panel backed by DrawerSpec.
//!
//! Contract: `docs/contracts/components/drawer.md`
//! Uses overlay() with backdrop, anchored to viewport edge.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ControlSize, DrawerEdge, DrawerSpec};

use crate::presentation::{
    panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size,
    resolve_supporting_visual_size, size_font_rem,
};
use crate::theme_ext::resolve_color;

pub fn js_drawer(spec: &DrawerSpec, theme: &JetstreamThemeProvider, content: Option<JsEl>) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let title_font = rem_to_px(size_font_rem(effective_size) + 0.1875);
    let body_font = rem_to_px(size_font_rem(effective_size));
    let icon_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(effective_size)));
    let space_x = rem_to_px(panel_space_x_rem(spec.density));
    let space_y = rem_to_px(panel_space_y_rem(spec.density));

    let fill = resolve_color(theme, spec.surface_fill_token());
    let backdrop: Color = resolve_color(theme, spec.backdrop_fill_token()).into();
    let border = resolve_color(theme, "color.border.default");
    let title_color = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");

    // Size-based panel dimensions — vertical drawers fix width, horizontal drawers fix height.
    let panel_fixed_dim = match spec.size {
        ControlSize::Sm => rem_to_px(18.0),
        ControlSize::Lg | ControlSize::Xl => rem_to_px(32.0),
        _ => rem_to_px(24.0),
    };

    // Close icon
    let close_icon = ui_element::icon("x")
        .w(icon_size).h(icon_size)
        .text_color(title_color)
        .cursor_pointer();

    // Panel — direction-specific sizing and border edge.
    let panel = {
        let mut p = ui_element::div()
            .bg(fill)
            .flex_col().gap(rem_to_px(0.75))
            .pl(space_x).pr(space_x).pt(space_y).pb(space_y)
            .shadow_lg();

        p = match spec.edge {
            DrawerEdge::Right => p
                .h_full()
                .w(panel_fixed_dim)
                .border_l_1()
                .border_color(border),
            DrawerEdge::Left => p
                .h_full()
                .w(panel_fixed_dim)
                .border_r_1()
                .border_color(border),
            DrawerEdge::Bottom => p
                .w_full()
                .h(panel_fixed_dim)
                .border_t_1()
                .border_color(border),
            DrawerEdge::Top => p
                .w_full()
                .h(panel_fixed_dim)
                .border_b_1()
                .border_color(border),
        };

        p
    };

    // Header: title + optional description, close button at end.
    let mut header_col = ui_element::div()
        .flex_col()
        .gap(rem_to_px(0.25))
        .grow();

    if let Some(ref title) = spec.title {
        header_col = header_col.child(
            ui_element::label(title)
                .text_color(title_color)
                .text_size(title_font)
                .text_weight(600),
        );
    }

    if let Some(ref description) = spec.description {
        header_col = header_col.child(
            ui_element::label(description)
                .text_color(text_secondary)
                .text_size(body_font),
        );
    }

    let header = ui_element::div()
        .flex_row()
        .items_center()
        .justify_between()
        .child(header_col)
        .child(close_icon);

    let mut panel = panel.child(header);

    if let Some(content_el) = content {
        panel = panel.child(content_el);
    }

    // Backdrop + panel as overlay — edge controls panel anchor position.
    let overlay = match spec.edge {
        DrawerEdge::Right => ui_element::div()
            .bg(backdrop)
            .overlay()
            .flex_row()
            .justify_end(),
        DrawerEdge::Left => ui_element::div()
            .bg(backdrop)
            .overlay()
            .flex_row()
            .justify_start(),
        DrawerEdge::Bottom => ui_element::div()
            .bg(backdrop)
            .overlay()
            .flex_col()
            .justify_end(),
        DrawerEdge::Top => ui_element::div()
            .bg(backdrop)
            .overlay()
            .flex_col()
            .justify_start(),
    };

    overlay.child(panel)
}
