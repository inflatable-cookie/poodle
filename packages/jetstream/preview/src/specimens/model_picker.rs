//! ModelPicker specimen — combined model + capability-axis picker.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::model_picker::js_model_picker;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{
    ControlDensity, ControlSize, ModelAxisBinding, ModelAxisControl, ModelAxisOption,
    ModelAxisValue, ModelCapabilityAxis, ModelImage, ModelOption, ModelPickerEmphasis,
    ModelPickerSpec, ModelPickerVariant, ModelSelection,
};

fn models() -> Vec<ModelOption> {
    vec![
        ModelOption::new("atlas-pro", "Atlas Pro")
            .with_description("Deepest reasoning, slowest responses")
            .with_badge("1M")
            .with_icon("sparkles")
            .with_group("Atlas")
            .with_axes(vec!["effort".into(), "fast".into(), "context".into()]),
        // No long-context option on this tier.
        ModelOption::new("atlas", "Atlas")
            .with_description("Balanced quality and latency")
            .with_icon("sparkles")
            .with_group("Atlas")
            .with_axes(vec!["effort".into(), "fast".into()]),
        // Same `effort` key, this provider's vocabulary, forced to a list.
        ModelOption::new("corvid-1", "Corvid 1")
            .with_description("Other provider, its own effort levels")
            // An arbitrary image (a provider logo) instead of a registry icon.
            .with_image(ModelImage::new("assets/logos/corvid.png").with_alt("Corvid"))
            .with_group("Corvid")
            .with_axes(vec![
                ModelAxisBinding::new("effort")
                    .with_control(ModelAxisControl::List)
                    .with_options(vec![
                        ModelAxisOption::new("minimal", "Minimal"),
                        ModelAxisOption::new("balanced", "Balanced"),
                        ModelAxisOption::new("deep", "Deep"),
                    ])
                    .with_default_value(ModelAxisValue::Text("balanced".into()))
                    .into(),
                "verbosity".into(),
            ]),
        // Seven levels render as a list on their own; the binding also relabels
        // the shared key for this provider's vocabulary.
        ModelOption::new("corvid-ultra", "Corvid Ultra")
            .with_description("Seven-level scale plus a thinking toggle")
            .with_badge("Preview")
            .with_image(ModelImage::new("assets/logos/corvid.png").with_alt("Corvid"))
            .with_group("Corvid")
            .with_axes(vec![
                ModelAxisBinding::new("effort")
                    .with_label("Thinking budget")
                    .with_options(vec![
                        ModelAxisOption::new("minimal", "Minimal"),
                        ModelAxisOption::new("very-low", "Very low"),
                        ModelAxisOption::new("low", "Low"),
                        ModelAxisOption::new("medium", "Medium"),
                        ModelAxisOption::new("high", "High"),
                        ModelAxisOption::new("very-high", "Very high"),
                        ModelAxisOption::new("max", "Maximum"),
                    ])
                    .with_default_value(ModelAxisValue::Text("high".into()))
                    .into(),
                "thinking".into(),
            ]),
        ModelOption::new("corvid-mini", "Corvid Mini")
            .with_description("No knobs at all")
            .with_icon("zap")
            .with_group("Corvid")
            .with_axes(vec![]),
        ModelOption::new("legacy-1", "Legacy 1")
            .with_description("Retired — kept for reproducibility")
            .with_group("Archive")
            .with_disabled(true),
    ]
}

fn axes() -> Vec<ModelCapabilityAxis> {
    vec![
        ModelCapabilityAxis::select(
            "effort",
            "Effort",
            vec![
                ModelAxisOption::new("low", "Low"),
                ModelAxisOption::new("medium", "Medium"),
                ModelAxisOption::new("high", "High"),
            ],
        )
        .with_default_value(ModelAxisValue::Text("medium".into())),
        ModelCapabilityAxis::toggle("fast", "Fast mode")
            .with_description("Trades a little depth for latency")
            .with_labels("Fast", "Normal"),
        ModelCapabilityAxis::select(
            "context",
            "Context window",
            vec![
                ModelAxisOption::new("200k", "200K"),
                ModelAxisOption::new("1m", "1M"),
            ],
        )
        .with_default_value(ModelAxisValue::Text("200k".into())),
        ModelCapabilityAxis::select(
            "verbosity",
            "Verbosity",
            vec![
                ModelAxisOption::new("terse", "Terse"),
                ModelAxisOption::new("normal", "Normal"),
                ModelAxisOption::new("chatty", "Chatty"),
            ],
        )
        .with_default_value(ModelAxisValue::Text("normal".into())),
        ModelCapabilityAxis::toggle("thinking", "Extended thinking")
            .with_labels("Thinking", "Direct"),
    ]
}

fn selection() -> ModelSelection {
    ModelSelection::new("atlas-pro")
        .with_axis("effort", ModelAxisValue::Text("high".into()))
        .with_axis("fast", ModelAxisValue::Flag(false))
        .with_axis("context", ModelAxisValue::Text("1m".into()))
}

fn base() -> ModelPickerSpec {
    ModelPickerSpec::new()
        .with_models(models())
        .with_axes(axes())
        .with_value(selection())
}

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        .child(group(
            "Model + axes (open)",
            secondary,
            js_model_picker(&base().with_open(true), theme),
        ))
        .child(group(
            "Rebound axis (Corvid 1: own levels, forced to a list)",
            secondary,
            js_model_picker(
                &base()
                    .with_value(ModelSelection::new("corvid-1"))
                    .with_open(true),
                theme,
            ),
        ))
        .child(group(
            "Many-level axis (Corvid Ultra: 7 levels, relabelled key)",
            secondary,
            js_model_picker(
                &base()
                    .with_value(ModelSelection::new("corvid-ultra"))
                    .with_open(true),
                theme,
            ),
        ))
        .child(group(
            "Model with no axes at all (Corvid Mini)",
            secondary,
            js_model_picker(
                &base()
                    .with_value(ModelSelection::new("corvid-mini"))
                    .with_open(true),
                theme,
            ),
        ))
        .child(group(
            "Trigger only (collapsed)",
            secondary,
            js_model_picker(&base(), theme),
        ))
        .child(group(
            "Emphasis: subdued (recedes beside a louder control)",
            secondary,
            js_model_picker(&base().with_emphasis(ModelPickerEmphasis::Subdued), theme),
        ))
        .child(group(
            "Outlined trigger",
            secondary,
            js_model_picker(&base().with_variant(ModelPickerVariant::Outlined), theme),
        ))

        .child(group(
            "No model selected",
            secondary,
            js_model_picker(&base().with_value(ModelSelection::default()), theme),
        ))
        .child(group(
            "Disabled",
            secondary,
            js_model_picker(&base().with_disabled(true), theme),
        ))
        .child(group(
            "Sizes",
            secondary,
            div().flex_col().gap(8.0).children(
                [
                    ControlSize::Xs,
                    ControlSize::Sm,
                    ControlSize::Md,
                    ControlSize::Lg,
                    ControlSize::Xl,
                ]
                .into_iter()
                .map(|size| js_model_picker(&base().with_size(size), theme)),
            ),
        ))
        .child(group(
            "Densities",
            secondary,
            div().flex_col().gap(8.0).children(
                [
                    ControlDensity::Compact,
                    ControlDensity::Default,
                    ControlDensity::Comfortable,
                ]
                .into_iter()
                .map(|density| js_model_picker(&base().with_density(density), theme)),
            ),
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
