//! StateTile — Jetstream metric tile with sparkline backed by StateTileSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_composites::StateTileSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_state_tile(spec: &StateTileSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, spec.fill_token());
    let border = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, "semantic.radius.surface");
    let text_primary = resolve_color(theme, "semantic.color.text.primary");
    let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
    let trend_color = resolve_color(theme, spec.trend_color_token());

    let pad_x = rem_to_px(1.0);
    let pad_y = rem_to_px(0.75);
    let gap = rem_to_px(0.5);
    let label_size = rem_to_px(0.75);
    let value_size = rem_to_px(1.5);
    let trend_size = rem_to_px(0.75);

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .pl(pad_x).pr(pad_x).pt(pad_y).pb(pad_y)
        .flex_col().gap(gap);

    // Label
    el = el.child(
        ui_element::label(&spec.label)
            .text_color(text_secondary).text_size(label_size)
    );

    // Value
    el = el.child(
        ui_element::label(&spec.value)
            .text_color(text_primary).text_size(value_size).text_weight(700)
    );

    // Trend row
    if spec.trend.is_some() || spec.trend_label.is_some() {
        let mut trend_row = ui_element::div().flex_row().items_center().gap(rem_to_px(0.25));

        if let Some(ref trend) = spec.trend {
            let arrow = match trend.as_str() {
                "up" => "\u{2191}",    // ↑
                "down" => "\u{2193}",  // ↓
                _ => "\u{2192}",       // →
            };
            trend_row = trend_row.child(
                ui_element::label(arrow)
                    .text_color(trend_color).text_size(trend_size).text_weight(600)
            );
        }

        if let Some(ref label) = spec.trend_label {
            trend_row = trend_row.child(
                ui_element::label(label)
                    .text_color(trend_color).text_size(trend_size)
            );
        }

        el = el.child(trend_row);
    }

    // Sparkline placeholder
    if spec.has_sparkline {
        let sparkline_h = rem_to_px(2.0);
        let sparkline_bg = resolve_color(theme, "semantic.color.background.subtle");

        el = el.child(
            ui_element::div()
                .bg(sparkline_bg)
                .rounded(rem_to_px(0.25))
                .h(sparkline_h).grow()
        );
    }

    el
}
