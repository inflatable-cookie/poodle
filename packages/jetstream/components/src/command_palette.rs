//! CommandPalette — Jetstream command palette backed by CommandPaletteSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_composites::CommandPaletteSpec;
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_command_palette(spec: &CommandPaletteSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, "semantic.color.background.elevated");
    let border = resolve_color(theme, "semantic.color.border.default");
    let radius = resolve_radius(theme, "semantic.radius.surface");
    let text_color = resolve_color(theme, "semantic.color.text.primary");
    let text_muted = resolve_color(theme, "semantic.color.text.secondary");

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .flex_col()
        .min_w(400.0).min_h(200.0)
        .shadow_lg()
        .overlay();

    // Search input area with icon
    el = el.child(
        ui_element::div()
            .flex_row().items_center().gap(8.0)
            .pl(12.0).pr(12.0).pt(8.0).pb(8.0)
            .border_b_1().border_color(border)
            .child(ui_element::icon("search").w(16.0).h(16.0).text_color(text_muted))
            .child(ui_element::label("Type a command...").text_color(text_muted).text_size(14.0).grow())
    );

    // Action list
    for action in &spec.actions {
        el = el.child(
            ui_element::div()
                .flex_row().items_center().gap(8.0)
                .pl(12.0).pr(12.0).pt(6.0).pb(6.0)
                .child(ui_element::label(&action.title).text_color(text_color).text_size(13.0).grow())
        );
    }

    el
}
