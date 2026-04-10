//! CommandPalette — Jetstream command palette backed by CommandPaletteSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_composites::CommandPaletteSpec;

use crate::presentation::{
    control_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_command_palette(spec: &CommandPaletteSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let icon_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let pad_y = rem_to_px(panel_space_y_rem(spec.density) - 0.25);
    let item_py = rem_to_px(panel_space_y_rem(spec.density) - 0.375);
    let gap = rem_to_px(0.5);

    let fill = resolve_color(theme, "color.background.elevated");
    let border = resolve_color(theme, "color.border.default");
    let radius = resolve_radius(theme, "radius.surface");
    let text_color = resolve_color(theme, "color.text.primary");
    let text_muted = resolve_color(theme, "color.text.secondary");

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .flex_col()
        .min_w(rem_to_px(25.0)).min_h(rem_to_px(12.5))
        .shadow_lg()
        .overlay();

    // Search input area with icon
    el = el.child(
        ui_element::div()
            .flex_row().items_center().gap(gap)
            .pl(pad_x).pr(pad_x).pt(pad_y).pb(pad_y)
            .border_b_1().border_color(border)
            .child(ui_element::icon("search").w(icon_size).h(icon_size).text_color(text_muted))
            .child(ui_element::label("Type a command...").text_color(text_muted).text_size(font_size).grow())
    );

    // Action list
    for action in &spec.actions {
        el = el.child(
            ui_element::div()
                .flex_row().items_center().gap(gap)
                .pl(pad_x).pr(pad_x).pt(item_py).pb(item_py)
                .child(ui_element::label(&action.title).text_color(text_color).text_size(font_size).grow())
        );
    }

    el
}
