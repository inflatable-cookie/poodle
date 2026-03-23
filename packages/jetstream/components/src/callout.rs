//! Callout — Jetstream callout/alert component backed by CallOutSpec.
//!
//! Contract: `docs/contracts/foundation/callout.md`
//! Uses Color::mix for tinted fill and SVG icon per tone.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use flint_jetstream::JetstreamThemeProvider;
use flint_primitives::{CallOutSpec, StatusTone};

use crate::theme_ext::{resolve_color, resolve_radius};

/// Map a status tone to its icon name (contract icon mapping).
fn tone_icon(tone: StatusTone) -> &'static str {
    match tone {
        StatusTone::Neutral | StatusTone::Info => "info",
        StatusTone::Success => "check-circle",
        StatusTone::Warning => "alert-triangle",
        StatusTone::Danger => "x-circle",
        StatusTone::Pending => "loader",
    }
}

pub fn js_callout(spec: &CallOutSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let tone_color: Color = resolve_color(theme, spec.fill_token()).into();
    let border_color = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, "semantic.radius.surface");
    let text_primary = resolve_color(theme, "semantic.color.text.primary");
    let panel: Color = resolve_color(theme, "semantic.color.background.panel").into();

    // Contract: fill = color-mix(tone ~10%, panel ~90%)
    let fill = tone_color.mix(panel, 0.10);

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0)
        .border_color(border_color)
        .rounded(radius)
        .pl(12.0).pr(12.0)
        .pt(12.0).pb(12.0)
        .flex_row()
        .gap(8.0);

    // Icon per tone
    el = el.child(
        ui_element::icon(tone_icon(spec.tone))
            .w(16.0).h(16.0)
            .text_color(tone_color)
    );

    // Content column
    let mut content = ui_element::div().flex_col().gap(4.0).grow();

    if let Some(ref title) = spec.title {
        content = content.child(
            ui_element::label(title)
                .text_color(text_primary)
                .text_size(13.0)
                .text_weight(600)
        );
    }

    if let Some(ref body) = spec.content {
        content = content.child(
            ui_element::label(body)
                .text_color(text_primary)
                .text_size(13.0)
        );
    }

    el.child(content)
}
