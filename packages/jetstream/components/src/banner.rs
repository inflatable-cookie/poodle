//! Banner — Jetstream status banner backed by BannerSpec.
//!
//! Uses Color::mix for tinted fill and SVG icon per tone.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::{BannerSpec, StatusTone};

use crate::theme_ext::resolve_color;

/// Map a status tone to its icon name.
fn tone_icon(tone: StatusTone) -> &'static str {
    match tone {
        StatusTone::Neutral | StatusTone::Info => "info",
        StatusTone::Success => "check-circle",
        StatusTone::Warning => "alert-triangle",
        StatusTone::Danger => "x-circle",
        StatusTone::Pending => "loader",
    }
}

pub fn js_banner(spec: &BannerSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let tone_color: Color = resolve_color(theme, spec.fill_token()).into();
    let icon_color = resolve_color(theme, spec.icon_color_token());
    let border_color = resolve_color(theme, spec.border_token());
    let text_primary = resolve_color(theme, "semantic.color.text.primary");
    let panel: Color = resolve_color(theme, "semantic.color.background.panel").into();

    let fill = tone_color.mix(panel, 0.12);

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0)
        .border_color(border_color)
        .pl(12.0).pr(12.0)
        .pt(8.0).pb(8.0)
        .flex_row()
        .items_center()
        .gap(8.0);

    // Icon (from tone)
    if spec.has_icon {
        el = el.child(
            ui_element::icon(tone_icon(spec.tone))
                .w(16.0).h(16.0)
                .text_color(icon_color)
        );
    }

    if let Some(ref title) = spec.title {
        el = el.child(
            ui_element::label(title)
                .text_color(text_primary)
                .text_size(13.0)
                .text_weight(600)
        );
    }

    if let Some(ref message) = spec.message {
        el = el.child(
            ui_element::label(message)
                .text_color(text_primary)
                .text_size(13.0)
                .grow()
        );
    }

    // Dismiss button
    if spec.is_dismissible {
        el = el.child(
            ui_element::icon("x")
                .w(16.0).h(16.0)
                .text_color(text_primary)
                .cursor_pointer()
        );
    }

    el
}
