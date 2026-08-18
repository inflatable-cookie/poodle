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

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_primary = theme.resolve_color("color.text.primary");

    let interactive_rating = state
        .specimens
        .selections
        .get("rating-interactive")
        .copied()
        .unwrap_or(3);
    let node_events = state.node_events.clone();
    let on_change = Arc::new(move |value: u32| {
        node_events.lock().unwrap().push(NodeSpecimenEvent::Select {
            key: "rating-interactive".to_string(),
            index: value as usize,
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
                .child(
                    Rating::from_spec(
                        RatingSpec::new().with_value(interactive_rating as f64),
                        theme,
                    )
                    .on_change(on_change.clone()),
                )
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(color_to_hsla(text_primary))
                        .child(format!("Rating: {} / 5", interactive_rating)),
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
                    RatingSpec::new().with_default_value(7.0).with_max(10),
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
                        .with_allow_clear(true),
                    theme,
                )),
        )
        // --- Readonly ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Readonly"),
                    theme,
                ))
                .child(Rating::from_spec(
                    RatingSpec::new().with_value(4.0).with_readonly(true),
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
                Rating::from_spec(RatingSpec::new().with_value(3.0).with_size(size), theme)
                    .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                Rating::from_spec(
                    RatingSpec::new().with_value(3.0).with_density(density),
                    theme,
                )
                .into_any_element()
            }),
    )
}
