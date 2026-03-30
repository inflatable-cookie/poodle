//! StateTile specimen — compact label-value tile with trend.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::state_tile::js_state_tile;
use poodle_jetstream_components::theme_ext::*;
use poodle_composites::StateTileSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("With trend", secondary,
            div().flex_row().gap(16.0)
                .child(
                    js_state_tile(
                        &StateTileSpec::new("Active Users", "1,284")
                            .with_trend("up")
                            .with_trend_label("+12% this week"),
                        theme,
                    )
                )
                .child(
                    js_state_tile(
                        &StateTileSpec::new("Error Rate", "0.3%")
                            .with_trend("down")
                            .with_trend_label("-0.1% today"),
                        theme,
                    )
                )
        ))
        .child(group("With sparkline", secondary,
            js_state_tile(
                &StateTileSpec::new("Revenue", "$42,800")
                    .with_trend("up")
                    .with_trend_label("+8.5% vs last month")
                    .with_sparkline(true),
                theme,
            )
        ))
        .child(group("Flat trend", secondary,
            js_state_tile(
                &StateTileSpec::new("Latency", "42ms")
                    .with_trend("flat")
                    .with_trend_label("Stable"),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
