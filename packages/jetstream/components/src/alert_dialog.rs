//! AlertDialog — Jetstream alert dialog backed by AlertDialogSpec.
//!
//! Contract: `docs/contracts/foundation/alert-dialog.md`
//! Uses overlay() with backdrop. Alert dialogs are not dismissible by backdrop click.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::AlertDialogSpec;

use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_alert_dialog(spec: &AlertDialogSpec, theme: &JetstreamThemeProvider) -> JsEl {
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
        .pl(24.0).pr(24.0).pt(20.0).pb(20.0)
        .flex_col().gap(12.0)
        .min_w(320.0)
        .shadow_lg();

    if !spec.title.is_empty() {
        panel = panel.child(
            ui_element::label(&spec.title).text_color(title_color).text_size(16.0).text_weight(600)
        );
    }

    if let Some(ref desc) = spec.description {
        panel = panel.child(
            ui_element::label(desc).text_color(desc_color).text_size(13.0)
        );
    }

    // Backdrop + centered panel as overlay
    ui_element::div()
        .bg(backdrop)
        .overlay()
        .items_center().justify_center()
        .child(panel)
}
