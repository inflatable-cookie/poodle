//! Dialog — Jetstream dialog container backed by DialogSpec.
//!
//! Contract: `docs/contracts/components/dialog.md`
//! Uses overlay() for modal rendering with backdrop.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::DialogSpec;

use crate::presentation::{
    panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_dialog(spec: &DialogSpec, theme: &JetstreamThemeProvider, content: Option<JsEl>) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let title_font = rem_to_px(size_font_rem(effective_size) + 0.1875); // title is larger
    let body_font = rem_to_px(size_font_rem(effective_size));
    let space_x = rem_to_px(panel_space_x_rem(spec.density) + 0.5);
    let space_y = rem_to_px(panel_space_y_rem(spec.density) + 0.5);

    let fill = resolve_color(theme, spec.surface_fill_token());
    let backdrop_fill: Color = resolve_color(theme, spec.backdrop_fill_token()).into();
    let border = resolve_color(theme, "color.border.default");
    let radius = resolve_radius(theme, "radius.surface");
    let title_color = resolve_color(theme, "color.text.primary");
    let desc_color = resolve_color(theme, "color.text.secondary");

    // Dialog panel
    let mut panel = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .pl(space_x).pr(space_x).pt(space_y).pb(space_y)
        .flex_col().gap(rem_to_px(1.0))
        .min_w(rem_to_px(25.0))
        .shadow_lg();

    if let Some(ref title) = spec.title {
        panel = panel.child(
            ui_element::label(title).text_color(title_color).text_size(title_font).text_weight(600)
        );
    }

    if let Some(ref description) = spec.description {
        panel = panel.child(
            ui_element::label(description).text_color(desc_color).text_size(body_font)
        );
    }

    if let Some(content_el) = content {
        panel = panel.child(content_el);
    }

    // Backdrop + centered panel as overlay
    ui_element::div()
        .bg(backdrop_fill)
        .overlay()
        .items_center().justify_center()
        .child(panel)
}
