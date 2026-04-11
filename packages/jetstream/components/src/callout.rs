//! Callout — Jetstream callout/alert component backed by CallOutSpec.
//!
//! Contract: `docs/contracts/components/callout.md`
//! Uses Color::mix for tinted fill and SVG icon per tone.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_components::{CallOutSpec, StatusTone};

use crate::presentation::{
    control_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size,
    resolve_supporting_visual_size, size_font_rem,
};
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
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let icon_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(effective_size)));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let pad_y = rem_to_px(panel_space_y_rem(spec.density));
    let gap = rem_to_px(0.5);
    let content_gap = rem_to_px(0.25);

    let tone_color: Color = resolve_color(theme, spec.fill_token()).into();
    let border_color = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, "radius.surface");
    let text_primary = resolve_color(theme, "color.text.primary");
    let panel: Color = resolve_color(theme, "color.background.panel").into();

    // Contract: fill = color-mix(tone ~10%, panel ~90%)
    let fill = tone_color.mix(panel, 0.10);

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0)
        .border_color(border_color)
        .rounded(radius)
        .pl(pad_x).pr(pad_x)
        .pt(pad_y).pb(pad_y)
        .flex_row()
        .gap(gap);

    // Icon per tone
    el = el.child(
        ui_element::icon(tone_icon(spec.tone))
            .w(icon_size).h(icon_size)
            .text_color(tone_color)
    );

    // Content column
    let mut content = ui_element::div().flex_col().gap(content_gap).grow();

    if let Some(ref title) = spec.title {
        content = content.child(
            ui_element::label(title)
                .text_color(text_primary)
                .text_size(font_size)
                .text_weight(600)
        );
    }

    if let Some(ref body) = spec.content {
        content = content.child(
            ui_element::label(body)
                .text_color(text_primary)
                .text_size(font_size)
        );
    }

    el.child(content)
}
