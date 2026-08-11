use crate::app_state::AppState;
use crate::node_compat::{Eyebrow, ModelPicker};
use crate::specimens::specimen_layout::specimen_layout;
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    EyebrowSpec, ModelAxisBinding, ModelAxisControl, ModelAxisOption, ModelAxisValue,
    ModelCapabilityAxis, ModelImage, ModelOption, ModelPickerEmphasis, ModelPickerSpec,
    ModelPickerVariant, ModelSelection,
};

fn demo_models() -> Vec<ModelOption> {
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

fn demo_axes() -> Vec<ModelCapabilityAxis> {
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

fn demo_selection() -> ModelSelection {
    ModelSelection::new("atlas-pro")
        .with_axis("effort", ModelAxisValue::Text("high".into()))
        .with_axis("fast", ModelAxisValue::Flag(false))
        .with_axis("context", ModelAxisValue::Text("1m".into()))
}

fn demo_spec() -> ModelPickerSpec {
    ModelPickerSpec::new()
        .with_models(demo_models())
        .with_axes(demo_axes())
        .with_value(demo_selection())
}

fn section(title: &str, theme: &GpuiThemeProvider, content: AnyElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(title),
            theme,
        ))
        .child(content)
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(section(
            "Model + axes (open)",
            theme,
            ModelPicker::from_spec(demo_spec().with_open(true), theme).into_any_element(),
        ))
        .child(section(
            "Rebound axis (Corvid 1: own levels, forced to a list)",
            theme,
            ModelPicker::from_spec(
                demo_spec()
                    .with_value(ModelSelection::new("corvid-1"))
                    .with_open(true),
                theme,
            )
            .into_any_element(),
        ))
        .child(section(
            "Many-level axis (Corvid Ultra: 7 levels, relabelled key)",
            theme,
            ModelPicker::from_spec(
                demo_spec()
                    .with_value(ModelSelection::new("corvid-ultra"))
                    .with_open(true),
                theme,
            )
            .into_any_element(),
        ))
        .child(section(
            "Model with no axes at all (Corvid Mini)",
            theme,
            ModelPicker::from_spec(
                demo_spec()
                    .with_value(ModelSelection::new("corvid-mini"))
                    .with_open(true),
                theme,
            )
            .into_any_element(),
        ))
        .child(section(
            "Trigger only (collapsed)",
            theme,
            ModelPicker::from_spec(demo_spec(), theme).into_any_element(),
        ))
        .child(section(
            "Emphasis: subdued (recedes beside a louder control)",
            theme,
            ModelPicker::from_spec(
                demo_spec().with_emphasis(ModelPickerEmphasis::Subdued),
                theme,
            )
            .into_any_element(),
        ))
        .child(section(
            "Outlined trigger",
            theme,
            ModelPicker::from_spec(
                demo_spec().with_variant(ModelPickerVariant::Outlined),
                theme,
            )
            .into_any_element(),
        ))
        .child(section(
            "No model selected",
            theme,
            ModelPicker::from_spec(demo_spec().with_value(ModelSelection::default()), theme)
                .into_any_element(),
        ))
        .child(section(
            "Disabled",
            theme,
            ModelPicker::from_spec(demo_spec().with_disabled(true), theme).into_any_element(),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "model-picker",
        examples,
        |size, theme: &GpuiThemeProvider| {
            ModelPicker::from_spec(demo_spec(), theme)
                .size(size)
                .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            ModelPicker::from_spec(demo_spec(), theme)
                .with_density(density)
                .into_any_element()
        },
    )
}
