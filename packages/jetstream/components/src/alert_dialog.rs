//! AlertDialog — Jetstream alert dialog backed by AlertDialogSpec.
//!
//! Contract: `docs/contracts/foundation/alert-dialog.md`
//! Uses overlay() with backdrop. Alert dialogs are not dismissible by backdrop click.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::AlertDialogSpec;

use crate::presentation::{
    panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_alert_dialog(spec: &AlertDialogSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let title_font = rem_to_px(size_font_rem(effective_size) + 0.1875);
    let body_font = rem_to_px(size_font_rem(effective_size));
    let space_x = rem_to_px(panel_space_x_rem(spec.density) + 0.5);
    let space_y = rem_to_px(panel_space_y_rem(spec.density) + 0.5);

    let fill = resolve_color(theme, "semantic.color.background.elevated");
    let backdrop: Color = resolve_color(theme, spec.backdrop_fill_token()).into();
    let border = resolve_color(theme, "semantic.color.border.default");
    let radius = resolve_radius(theme, "semantic.radius.surface");
    let title_color = resolve_color(theme, "semantic.color.text.primary");
    let desc_color = resolve_color(theme, "semantic.color.text.secondary");

    let mut panel = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .pl(space_x).pr(space_x).pt(space_y).pb(space_y)
        .flex_col().gap(rem_to_px(0.75))
        .min_w(rem_to_px(20.0))
        .shadow_lg();

    if !spec.title.is_empty() {
        panel = panel.child(
            ui_element::label(&spec.title).text_color(title_color).text_size(title_font).text_weight(600)
        );
    }

    if let Some(ref desc) = spec.description {
        panel = panel.child(
            ui_element::label(desc).text_color(desc_color).text_size(body_font)
        );
    }

    // Backdrop + centered panel as overlay
    ui_element::div()
        .bg(backdrop)
        .overlay()
        .items_center().justify_center()
        .child(panel)
}
