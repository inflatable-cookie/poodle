//! RemediationBanner — Jetstream dismissible fix suggestion banner backed by RemediationBannerSpec.
use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_components::RemediationBannerSpec;
use poodle_components::StatusTone;

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

pub fn js_remediation_banner(spec: &RemediationBannerSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let border = resolve_color(theme, spec.border_token());
    let tone_color: Color = resolve_color(theme, spec.border_token()).into();
    let panel: Color = resolve_color(theme, spec.background_token()).into();
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let accent = resolve_color(theme, "color.accent.base");

    let fill = tone_color.mix(panel, 0.08);

    let font_size = rem_to_px(0.8125);
    let small_size = rem_to_px(0.75);
    let icon_size = rem_to_px(1.25);
    let pad_x = rem_to_px(1.0);
    let pad_y = rem_to_px(0.75);
    let gap = rem_to_px(0.75);
    let content_gap = rem_to_px(0.25);
    let action_gap = rem_to_px(0.5);

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(rem_to_px(0.5))
        .pl(pad_x).pr(pad_x).pt(pad_y).pb(pad_y)
        .flex_row().gap(gap);

    // Note: ARIA role from spec.accessibility_role() applied by host runtime.

    // Tone icon
    el = el.child(
        ui_element::icon(tone_icon(spec.tone))
            .w(icon_size).h(icon_size)
            .text_color(border)
    );

    // Content column
    let mut content = ui_element::div().flex_col().gap(content_gap).grow();

    content = content.child(
        ui_element::label(&spec.title)
            .text_color(text_primary).text_size(font_size).text_weight(600)
    );

    content = content.child(
        ui_element::label(&spec.message)
            .text_color(text_secondary).text_size(font_size)
    );

    // Action buttons
    if spec.action_count() > 0 {
        let mut actions_row = ui_element::div().flex_row().items_center().gap(action_gap)
            .pt(rem_to_px(0.25));

        if let Some(ref primary) = spec.primary_action {
            actions_row = actions_row.child(
                ui_element::label(&primary.label)
                    .text_color(accent).text_size(small_size).text_weight(600)
                    .cursor_pointer()
            );
        }

        if let Some(ref secondary) = spec.secondary_action {
            actions_row = actions_row.child(
                ui_element::label(&secondary.label)
                    .text_color(text_secondary).text_size(small_size).text_weight(500)
                    .cursor_pointer()
            );
        }

        content = content.child(actions_row);
    }

    el = el.child(content);

    // Dismiss button
    if spec.is_dismissible {
        el = el.child(
            ui_element::icon("x")
                .w(rem_to_px(1.0)).h(rem_to_px(1.0))
                .text_color(text_secondary)
                .cursor_pointer()
        );
    }

    el
}
