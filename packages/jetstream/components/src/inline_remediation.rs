//! InlineRemediation — Jetstream inline fix suggestion backed by InlineRemediationSpec.
use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_components::InlineRemediationSpec;
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

pub fn js_inline_remediation(spec: &InlineRemediationSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let border = resolve_color(theme, spec.border_token());
    let tone_color: Color = resolve_color(theme, spec.border_token()).into();
    let panel: Color = resolve_color(theme, "color.background.panel").into();
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let accent_color = resolve_color(theme, "color.accent.base");

    let fill = tone_color.mix(panel, 0.08);
    let font_size = rem_to_px(0.8125);
    let small_size = rem_to_px(0.75);
    let icon_size = rem_to_px(1.0);
    let pad_x = rem_to_px(0.75);
    let pad_y = rem_to_px(0.5);
    let gap = rem_to_px(0.5);
    let content_gap = rem_to_px(0.25);

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(rem_to_px(0.375))
        .pl(pad_x).pr(pad_x).pt(pad_y).pb(pad_y)
        .flex_row().gap(gap);

    // Tone icon
    el = el.child(
        ui_element::icon(tone_icon(spec.tone))
            .w(icon_size).h(icon_size)
            .text_color(border)
    );

    // Content column
    let mut content = ui_element::div().flex_col().gap(content_gap).grow();

    if let Some(ref title) = spec.title {
        content = content.child(
            ui_element::label(title)
                .text_color(text_primary).text_size(font_size).text_weight(600)
        );
    }

    content = content.child(
        ui_element::label(&spec.message)
            .text_color(text_primary).text_size(font_size)
    );

    // Referenced field count hint
    if spec.reference_count() > 0 {
        let hint = format!("{} field{} affected", spec.reference_count(), if spec.reference_count() == 1 { "" } else { "s" });
        content = content.child(
            ui_element::label(&hint)
                .text_color(text_secondary).text_size(small_size)
        );
    }

    // Action button
    if let Some(ref action) = spec.action {
        content = content.child(
            ui_element::label(&action.label)
                .text_color(accent_color).text_size(small_size).text_weight(500)
                .cursor_pointer()
        );
    }

    el.child(content)
}
