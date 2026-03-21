//! AlertDialog — Jetstream alert dialog backed by AlertDialogSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::AlertDialogSpec;

use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_alert_dialog(spec: &AlertDialogSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, "semantic.color.background.elevated");
    let border = resolve_color(theme, "semantic.color.border.default");
    let radius = resolve_radius(theme, "semantic.radius.surface");
    let title_color = resolve_color(theme, "semantic.color.text.primary");
    let desc_color = resolve_color(theme, "semantic.color.text.secondary");

    let mut content = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .pl(24.0).pr(24.0).pt(20.0).pb(20.0)
        .flex_col().gap(12.0)
        .min_w(320.0);

    if !spec.title.is_empty() {
        content = content.child(
            ui_element::label(&spec.title).text_color(title_color).text_size(16.0).text_weight(600)
        );
    }

    if let Some(ref desc) = spec.description {
        content = content.child(
            ui_element::label(desc).text_color(desc_color).text_size(13.0)
        );
    }

    content
}
