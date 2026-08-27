use crate::app_state::AppState;
use crate::app_state::NodeSpecimenEvent;
use crate::node_compat::{Eyebrow, Rating};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::rating::trim_rating_fraction;
use poodle_node::ColorValue;
use poodle_specs::{EyebrowSpec, RatingSpec};
use std::sync::Arc;

fn rating_text(state: &AppState, key: &str, fallback: &str) -> String {
    state
        .specimens
        .text
        .get(key)
        .cloned()
        .unwrap_or_else(|| fallback.to_string())
}

fn parse_rating(raw: &str) -> Option<f64> {
    if raw.is_empty() || raw == "none" {
        None
    } else {
        raw.parse().ok()
    }
}

fn on_rating_text(node_events: Arc<std::sync::Mutex<Vec<NodeSpecimenEvent>>>, key: &str) -> Arc<dyn Fn(Option<f64>) + Send + Sync> {
    let key = key.to_string();
    Arc::new(move |value: Option<f64>| {
        let text = match value {
            Some(raw) => {
                if raw.fract() == 0.0 {
                    format!("{}", raw as i64)
                } else {
                    format!("{raw}")
                }
            }
            None => "none".to_string(),
        };
        node_events
            .lock()
            .unwrap()
            .push(NodeSpecimenEvent::SetText {
                key: key.clone(),
                value: text,
            });
    })
}

fn readout(text_primary: ColorValue, label: String) -> Div {
    div()
        .text_size(px(12.0))
        .text_color(color_to_hsla(text_primary))
        .child(label)
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_primary = theme.resolve_color("color.text.primary");
    let node_events = state.node_events.clone();

    let half_raw = rating_text(state, "rating-half", "3.5");
    let half_value = parse_rating(&half_raw);
    let whole_raw = rating_text(state, "rating-whole", "3");
    let whole_value = parse_rating(&whole_raw);
    let clear_raw = rating_text(state, "rating-clear", "4");
    let clear_value = parse_rating(&clear_raw);

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // Live default half-step
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Half-step (default)"),
                    theme,
                ))
                .child({
                    let mut spec = RatingSpec::new()
                        .with_allow_clear(true)
                        .with_aria_label("Half-step rating");
                    if let Some(value) = half_value {
                        spec = spec.with_value(value);
                    }
                    Rating::from_spec(spec, theme)
                        .with_instance_id("specimen-half")
                        .on_change(on_rating_text(Arc::clone(&node_events), "rating-half"))
                })
                .child(readout(
                    text_primary,
                    match half_value {
                        Some(value) => format!("{} / 5", trim_rating_fraction(value)),
                        None => "none / 5".to_string(),
                    },
                )),
        )
        // Whole-step
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Whole-step"),
                    theme,
                ))
                .child({
                    let mut spec = RatingSpec::new()
                        .with_step(1.0)
                        .with_aria_label("Whole-step rating");
                    if let Some(value) = whole_value {
                        spec = spec.with_value(value);
                    }
                    Rating::from_spec(spec, theme)
                        .with_instance_id("specimen-whole")
                        .on_change(on_rating_text(Arc::clone(&node_events), "rating-whole"))
                })
                .child(readout(
                    text_primary,
                    match whole_value {
                        Some(value) => format!("{value} / 5"),
                        None => "none / 5".to_string(),
                    },
                )),
        )
        // 10-star scale
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("10-star scale"),
                    theme,
                ))
                .child(Rating::from_spec(
                    RatingSpec::new()
                        .with_default_value(7.0)
                        .with_max(10)
                        .with_step(1.0)
                        .with_aria_label("Score out of 10"),
                    theme,
                )),
        )
        // Arbitrary fractional display
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Arbitrary fraction display"),
                    theme,
                ))
                .child(Rating::from_spec(
                    RatingSpec::new()
                        .with_value(3.7)
                        .with_aria_label("Display fraction"),
                    theme,
                ))
                .child(readout(text_primary, "3.7 / 5 (display only)".to_string())),
        )
        // Clearable whole-step
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Clearable"),
                    theme,
                ))
                .child({
                    let mut spec = RatingSpec::new()
                        .with_step(1.0)
                        .with_allow_clear(true)
                        .with_aria_label("Clearable rating");
                    if let Some(value) = clear_value {
                        spec = spec.with_value(value);
                    } else if clear_raw == "none" {
                        // keep empty
                    } else {
                        spec = spec.with_default_value(4.0);
                    }
                    Rating::from_spec(spec, theme)
                        .with_instance_id("specimen-clear")
                        .on_change(on_rating_text(Arc::clone(&node_events), "rating-clear"))
                })
                .child(readout(
                    text_primary,
                    match clear_value {
                        Some(value) => format!("{value} / 5"),
                        None => "none / 5".to_string(),
                    },
                )),
        )
        // Disabled
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Disabled"),
                    theme,
                ))
                .child(Rating::from_spec(
                    RatingSpec::new()
                        .with_default_value(2.0)
                        .with_step(1.0)
                        .with_disabled(true)
                        .with_aria_label("Disabled rating"),
                    theme,
                )),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "rating",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                Rating::from_spec(
                    RatingSpec::new().with_value(3.0).with_step(1.0).with_size(size),
                    theme,
                )
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                Rating::from_spec(
                    RatingSpec::new()
                        .with_value(3.0)
                        .with_step(1.0)
                        .with_density(density),
                    theme,
                )
                .into_any_element()
            }),
    )
}
