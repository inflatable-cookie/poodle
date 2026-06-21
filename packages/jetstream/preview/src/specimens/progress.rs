//! Progress specimen — mirrors Svelte ProgressSpecimen.svelte sections.
//!
//! Contract: `docs/contracts/components/progress.md` §13 Specimen Definitions.
//! Real component (`js_progress`) only — fill is resolved from `ProgressSpec`
//! (`normalized_progress` = value/max), never hand-rolled. Values are on the
//! default 0..100 scale so the spec resolves the proportional fill.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::progress::js_progress;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ControlSize, ProgressSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // Determinate — Empty / 35% / 72% / Complete (default max=100)
        .child(group("Determinate", secondary,
            div().flex_col().gap(12.0).self_stretch()
                .child(js_progress(&ProgressSpec::new().with_value(0.0), theme))
                .child(js_progress(&ProgressSpec::new().with_value(35.0), theme))
                .child(js_progress(&ProgressSpec::new().with_value(72.0), theme))
                .child(js_progress(&ProgressSpec::new().with_value(100.0), theme))
        ))
        // Indeterminate — animated sliding indicator
        .child(group("Indeterminate", secondary,
            div().flex_col().gap(12.0).self_stretch()
                .child(js_progress(&ProgressSpec::new().with_indeterminate(true), theme))
        ))
        // Custom max — value=3, max=5 → 60% fill
        .child(group("Custom max", secondary,
            div().flex_col().gap(12.0).self_stretch()
                .child(js_progress(&custom_max_spec(), theme))
        ))
        // With label / value text — value=3 of 5
        .child(group("With label / value text", secondary,
            div().flex_col().gap(12.0).self_stretch()
                .child(js_progress(&labelled_spec(), theme))
        ))
        // Sizes — 60% at each ControlSize
        .child(group("Sizes", secondary,
            div().flex_col().gap(12.0).self_stretch()
                .child(js_progress(&ProgressSpec::new().with_value(60.0).with_size(ControlSize::Xs), theme))
                .child(js_progress(&ProgressSpec::new().with_value(60.0).with_size(ControlSize::Sm), theme))
                .child(js_progress(&ProgressSpec::new().with_value(60.0).with_size(ControlSize::Md), theme))
                .child(js_progress(&ProgressSpec::new().with_value(60.0).with_size(ControlSize::Lg), theme))
                .child(js_progress(&ProgressSpec::new().with_value(60.0).with_size(ControlSize::Xl), theme))
        ))
}

fn custom_max_spec() -> ProgressSpec {
    let mut spec = ProgressSpec::new().with_value(3.0);
    spec.max = 5.0;
    spec
}

fn labelled_spec() -> ProgressSpec {
    let mut spec = ProgressSpec::new().with_value(3.0);
    spec.max = 5.0;
    spec.aria_label = Some("3 of 5 steps complete".to_string());
    spec.value_text = Some("3 of 5 steps".to_string());
    spec
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
