//! Dialog — Jetstream dialog container backed by DialogSpec.
//!
//! Contract: `docs/contracts/foundation/dialog.md`
//! Uses overlay() for modal rendering with backdrop.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::DialogSpec;

use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_dialog(spec: &DialogSpec, theme: &JetstreamThemeProvider, content: Option<JsEl>) -> JsEl {
    let fill = resolve_color(theme, spec.surface_fill_token());
    let backdrop_fill: Color = resolve_color(theme, spec.backdrop_fill_token()).into();
    let border = resolve_color(theme, "semantic.color.border.default");
    let radius = resolve_radius(theme, "semantic.radius.surface");
    let title_color = resolve_color(theme, "semantic.color.text.primary");
    let desc_color = resolve_color(theme, "semantic.color.text.secondary");

    // Dialog panel
    let mut panel = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .pl(24.0).pr(24.0).pt(20.0).pb(20.0)
        .flex_col().gap(16.0)
        .min_w(400.0)
        .shadow_lg();

    if let Some(ref title) = spec.title {
        panel = panel.child(
            ui_element::label(title).text_color(title_color).text_size(16.0).text_weight(600)
        );
    }

    if let Some(ref description) = spec.description {
        panel = panel.child(
            ui_element::label(description).text_color(desc_color).text_size(13.0)
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
