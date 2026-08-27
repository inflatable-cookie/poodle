use crate::app_state::AppState;
use crate::app_state::NodeSpecimenEvent;
use crate::node_compat::{Eyebrow, Rating};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
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

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_primary = theme.resolve_color("color.text.primary");

    let interactive_raw = rating_text(state, "rating-interactive", "3");
    let interactive_rating = parse_rating(&interactive_raw);
    let node_events = state.node_events.clone();
    let on_change = Arc::new(move |value: Option<f64>| {
        let text = match value {
            Some(raw) => format!("{raw}"),
            None => "none".to_string(),
        };
        node_events
            .lock()
            .unwrap()
            .push(NodeSpecimenEvent::SetText {
                key: "rating-interactive".to_string(),
                value: text,
            });
    });

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Default (5 stars) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Default (5 stars)"),
                    theme,
                ))
                .child({
                    let mut spec = RatingSpec::new().with_step(1.0);
                    if let Some(value) = interactive_rating {
                        spec = spec.with_value(value);
                    }
                    Rating::from_spec(spec, theme).on_change(on_change.clone())
                })
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(color_to_hsla(text_primary))
                        .child(match interactive_rating {
                            Some(value) => format!("Rating: {value} / 5"),
                            None => "Rating: none / 5".to_string(),
                        }),
                ),
        )
        // --- 10-star scale ---
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
                        .with_step(1.0),
                    theme,
                )),
        )
        // --- Half-star steps (fractional fill) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Half-star steps"),
                    theme,
                ))
                .child(Rating::from_spec(
                    RatingSpec::new()
                        .with_value(3.5)
                        .with_step(0.5)
                        .with_allow_clear(true),
                    theme,
                )),
        )
        // --- Clearable ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Clearable"),
                    theme,
                ))
                .child(Rating::from_spec(
                    RatingSpec::new()
                        .with_default_value(4.0)
                        .with_step(1.0)
                        .with_allow_clear(true),
                    theme,
                )),
        )
        // --- Disabled ---
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
                        .with_disabled(true),
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
