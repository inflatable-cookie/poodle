//! StateTile specimen — compact label-value tile with trend.

use crate::compat::js_state_tile;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::StateTileSpec;

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        .child(group(
            "Default (no trend)",
            secondary,
            div()
                .flex_row()
                .gap(16.0)
                .child(js_state_tile(
                    &StateTileSpec::new("Total Records", "8,402"),
                    theme,
                ))
                .child(js_state_tile(
                    &StateTileSpec::new("Open Issues", "37"),
                    theme,
                )),
        ))
        .child(group(
            "With trend (up / down)",
            secondary,
            div()
                .flex_row()
                .gap(16.0)
                .child(js_state_tile(
                    &StateTileSpec::new("Active Users", "1,284")
                        .with_trend("up")
                        .with_trend_label("+12% this week"),
                    theme,
                ))
                .child(js_state_tile(
                    &StateTileSpec::new("Error Rate", "0.3%")
                        .with_trend("down")
                        .with_trend_label("-0.1% today"),
                    theme,
                )),
        ))
        .child(group(
            "Neutral trend (arbitrary string)",
            secondary,
            div()
                .flex_row()
                .gap(16.0)
                .child(js_state_tile(
                    &StateTileSpec::new("Latency", "42ms")
                        .with_trend("flat")
                        .with_trend_label("Stable"),
                    theme,
                ))
                .child(js_state_tile(
                    &StateTileSpec::new("Region", "EU-West")
                        .with_trend("steady")
                        .with_trend_label("No change"),
                    theme,
                )),
        ))
        .child(group(
            "With sparkline (host renders chart in slot)",
            secondary,
            js_state_tile(
                &StateTileSpec::new("Revenue", "$42,800")
                    .with_trend("up")
                    .with_trend_label("+8.5% vs last month")
                    .with_sparkline(true),
                theme,
            ),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
