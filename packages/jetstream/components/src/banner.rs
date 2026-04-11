//! Banner — Jetstream status banner backed by BannerSpec.
//!
//! Uses Color::mix for tinted fill and SVG icon per tone.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{BannerSpec, StatusTone};

use crate::presentation::rem_to_px;
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
    let text_primary = resolve_color(theme, "color.text.primary");
    let panel: Color = resolve_color(theme, "color.background.panel").into();

    let fill = tone_color.mix(panel, 0.12);

    // Contract: padding 0.75rem 0.5rem, gap 0.5rem, font-size 0.8125rem (13px),
    // icon-size 1rem (16px)
    let pad_x = rem_to_px(0.75);
    let pad_y = rem_to_px(0.5);
    let gap = rem_to_px(0.5);
    let font_size = rem_to_px(0.8125);
    let icon_size = rem_to_px(1.0);

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0)
        .border_color(border_color)
        .pl(pad_x).pr(pad_x)
        .pt(pad_y).pb(pad_y)
        .flex_row()
        .items_center()
        .gap(gap);

    // Icon (from tone)
    if spec.has_icon {
        el = el.child(
            ui_element::icon(tone_icon(spec.tone))
                .w(icon_size).h(icon_size)
                .text_color(icon_color)
        );
    }

    if let Some(ref title) = spec.title {
        el = el.child(
            ui_element::label(title)
                .text_color(text_primary)
                .text_size(font_size)
                .text_weight(600)
        );
    }

    if let Some(ref message) = spec.message {
        el = el.child(
            ui_element::label(message)
                .text_color(text_primary)
                .text_size(font_size)
                .grow()
        );
    }

    // Dismiss button
    if spec.is_dismissible {
        el = el.child(
            ui_element::icon("x")
                .w(icon_size).h(icon_size)
                .text_color(text_primary)
                .cursor_pointer()
        );
    }

    el
}
