//! Rating specimen — mirrors Svelte RatingSpecimen.svelte sections.
//!
//! Contract: `docs/contracts/components/rating.md` §13 Specimen Definitions.
//! Real component (`js_rating`) only — fill is resolved from `RatingSpec`
//! (`fill_ratio` = value/max), never hand-rolled.

use crate::compat::js_rating;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{ControlSize, RatingSpec};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // Default (5 stars) — value=3
        .child(group(
            "Default (5 stars)",
            secondary,
            js_rating(
                &RatingSpec::new().with_value(3.0).with_step(1.0),
                theme,
            ),
        ))
        // 10-star scale — defaultValue=7, max=10
        .child(group(
            "10-star scale",
            secondary,
            js_rating(
                &RatingSpec::new()
                    .with_default_value(7.0)
                    .with_max(10)
                    .with_step(1.0),
                theme,
            ),
        ))
        // Half-star steps (fractional fill) — value=3.5, step=0.5, allowClear
        .child(group(
            "Half-star steps",
            secondary,
            js_rating(
                &RatingSpec::new()
                    .with_value(3.5)
                    .with_step(0.5)
                    .with_allow_clear(true),
                theme,
            ),
        ))
        // Clearable — defaultValue=4, allowClear
        .child(group(
            "Clearable",
            secondary,
            js_rating(
                &RatingSpec::new()
                    .with_default_value(4.0)
                    .with_step(1.0)
                    .with_allow_clear(true),
                theme,
            ),
        ))
        // Disabled — defaultValue=2, disabled
        .child(group(
            "Disabled",
            secondary,
            js_rating(
                &RatingSpec::new()
                    .with_default_value(2.0)
                    .with_step(1.0)
                    .with_disabled(true),
                theme,
            ),
        ))
        // Sizes — value=3 at each ControlSize
        .child(group(
            "Sizes",
            secondary,
            div()
                .flex_row()
                .items_center()
                .gap(16.0)
                .child(js_rating(
                    &RatingSpec::new()
                        .with_value(3.0)
                        .with_step(1.0)
                        .with_size(ControlSize::Xs),
                    theme,
                ))
                .child(js_rating(
                    &RatingSpec::new()
                        .with_value(3.0)
                        .with_step(1.0)
                        .with_size(ControlSize::Sm),
                    theme,
                ))
                .child(js_rating(
                    &RatingSpec::new()
                        .with_value(3.0)
                        .with_step(1.0)
                        .with_size(ControlSize::Md),
                    theme,
                ))
                .child(js_rating(
                    &RatingSpec::new()
                        .with_value(3.0)
                        .with_step(1.0)
                        .with_size(ControlSize::Lg),
                    theme,
                ))
                .child(js_rating(
                    &RatingSpec::new()
                        .with_value(3.0)
                        .with_step(1.0)
                        .with_size(ControlSize::Xl),
                    theme,
                )),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
