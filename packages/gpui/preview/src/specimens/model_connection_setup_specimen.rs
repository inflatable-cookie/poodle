use std::sync::Arc;

use crate::app_state::{AppState, ModelConnectionEvent, ModelConnectionPreviewState,
    NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, ModelConnectionSetup};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::model_connection::{
    ModelConnectionAvailability, ModelConnectionOption, ModelConnectionSetupStage,
};
use poodle_node::Node;
use poodle_specs::{
    ButtonSpec, ButtonVariant, EyebrowSpec, FieldSpec, ModelConnectionSetupSpec, TextInputSpec,
};

fn group(theme: &GpuiThemeProvider, label: &str, specimen: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(specimen)
}

fn panel(child: impl IntoElement) -> Div {
    div().max_w(px(672.0)).child(child)
}

fn options() -> Vec<ModelConnectionOption> {
    ModelConnectionPreviewState::options()
}

/// The web specimen's interactive set: `codex-app` made available, so the
/// direct-add route can be exercised.
fn interactive_options() -> Vec<ModelConnectionOption> {
    options()
        .into_iter()
        .map(|option| {
            if option.id == "codex-app" {
                option
                    .with_availability(ModelConnectionAvailability::Available, "Available")
                    .with_disabled(false)
            } else {
                option
            }
        })
        .collect()
}

/// Host configuration content. Poodle never sees these values: the field and
/// its input are nodes the host built and handed over.
fn api_key_field(theme: &GpuiThemeProvider, id: &str, value: &str) -> Node {
    poodle_render::field(
        &FieldSpec::new(id, "API key"),
        theme,
        Some(poodle_render::text_input(
            &TextInputSpec::new()
                .with_value(value)
                .with_type("password")
                .with_placeholder("sk-demo-placeholder"),
            theme,
            None,
        )),
    )
}

fn endpoint_field(theme: &GpuiThemeProvider, id: &str) -> Node {
    poodle_render::field(
        &FieldSpec::new(id, "Endpoint URL"),
        theme,
        Some(poodle_render::text_input(
            &TextInputSpec::new()
                .with_value("http://127.0.0.1:11434")
                .with_placeholder("http://127.0.0.1:11434"),
            theme,
            None,
        )),
    )
}

fn browser_sign_in(theme: &GpuiThemeProvider) -> Node {
    poodle_render::button(
        &ButtonSpec::new()
            .with_label("Sign in with browser")
            .with_variant(ButtonVariant::Secondary),
        theme,
        None,
    )
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let queue = Arc::clone(&state.node_events);
    let _ = cx;

    let host = &state.model_connection;
    let selected = host
        .setup_value
        .clone()
        .or_else(|| Some("openai-responses".to_string()));
    let configuration = match selected.as_deref() {
        Some("ollama-local") => Some(endpoint_field(theme, "mcs-choose-endpoint")),
        Some("anthropic-messages") => Some(browser_sign_in(theme)),
        Some("codex-app") => None,
        _ => Some(api_key_field(theme, "mcs-choose-api-key", "")),
    };

    // The choose stage is live: selection, query, Continue, Back, Add and
    // Cancel all run through the real handlers, and the preview's host loop
    // owns stage and value exactly as Nucleus will.
    let mut interactive = ModelConnectionSetup::from_spec(
        ModelConnectionSetupSpec::new()
            .with_options(interactive_options())
            .with_stage(host.setup_stage)
            .with_value(selected)
            .with_query(host.setup_query.clone())
            .with_can_submit(true),
        theme,
    )
    .on_stage_change({
        let queue = Arc::clone(&queue);
        Arc::new(move |stage: ModelConnectionSetupStage| {
            queue
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::ModelConnection(
                    ModelConnectionEvent::SetupStage(stage),
                ));
        })
    })
    .on_value_change({
        let queue = Arc::clone(&queue);
        Arc::new(move |id: &str| {
            queue
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::ModelConnection(
                    ModelConnectionEvent::SetupValue(id.to_string()),
                ));
        })
    })
    .on_query_change({
        let queue = Arc::clone(&queue);
        Arc::new(move |query: &str| {
            queue
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::ModelConnection(
                    ModelConnectionEvent::SetupQuery(query.to_string()),
                ));
        })
    })
    .on_submit({
        let queue = Arc::clone(&queue);
        Arc::new(move |id: &str| {
            queue
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::ModelConnection(
                    ModelConnectionEvent::SetupSubmit(id.to_string()),
                ));
        })
    })
    .on_cancel({
        let queue = Arc::clone(&queue);
        Arc::new(move || {
            queue
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::ModelConnection(
                    ModelConnectionEvent::SetupCancel,
                ));
        })
    })
    .on_focus_request({
        let queue = Arc::clone(&queue);
        Arc::new(move |id: &str| {
            queue
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::ModelConnection(
                    ModelConnectionEvent::FocusRequest(id.to_string()),
                ));
        })
    })
    .with_instance_id("setup-live");
    if let Some(content) = configuration {
        interactive = interactive.with_configuration(content);
    }
    // A host provider mark, keyed by option id, forwarded to both the picker
    // and the configure stage's selected summary.
    interactive = interactive.with_leading("ollama-local", Node::icon("terminal", 16.0));

    // Every group below renders the same routes, so each instance carries its
    // own backend-state scope.
    let configure = |value: &str| {
        ModelConnectionSetupSpec::new()
            .with_options(options())
            .with_stage(ModelConnectionSetupStage::Configure)
            .with_value(Some(value.to_string()))
    };

    div()
        .flex()
        .flex_col()
        .gap(px(32.0))
        .child(group(theme, "Choose stage", panel(interactive)))
        .child(group(
            theme,
            "Configure: API key",
            panel(
                ModelConnectionSetup::from_spec(
                    configure("openai-responses").with_can_submit(true),
                    theme,
                )
                .with_instance_id("setup-api-key")
                .with_configuration(api_key_field(theme, "mcs-api-key", "")),
            ),
        ))
        // Detection is the host's. Poodle renders the outcome it was given
        // and never probes for an install.
        .child(group(
            theme,
            "Auto-detect: found",
            panel(
                ModelConnectionSetup::from_spec(
                    configure("codex-app")
                        .with_can_submit(true)
                        .with_success("Local harness detected."),
                    theme,
                )
                .with_instance_id("setup-detect-found"),
            ),
        ))
        .child(group(
            theme,
            "Auto-detect: missing",
            panel(
                ModelConnectionSetup::from_spec(
                    configure("codex-app").with_error("Codex app not found on this machine."),
                    theme,
                )
                .with_instance_id("setup-detect-missing"),
            ),
        ))
        .child(group(
            theme,
            "OAuth pending",
            panel(
                ModelConnectionSetup::from_spec(
                    configure("anthropic-messages")
                        .with_pending(true)
                        .with_pending_label("Waiting for browser sign-in"),
                    theme,
                )
                .with_instance_id("setup-oauth-pending")
                .with_configuration(browser_sign_in(theme)),
            ),
        ))
        .child(group(
            theme,
            "Local endpoint",
            panel(
                ModelConnectionSetup::from_spec(
                    configure("ollama-local").with_can_submit(true),
                    theme,
                )
                .with_instance_id("setup-local-endpoint")
                .with_configuration(endpoint_field(theme, "mcs-endpoint"))
                .with_configure_aside(Node::text(
                    "The host checks this endpoint; Poodle never contacts it.",
                )),
            ),
        ))
        .child(group(
            theme,
            "Validation failure",
            panel(
                ModelConnectionSetup::from_spec(
                    configure("openai-responses").with_error("API key format is invalid."),
                    theme,
                )
                .with_instance_id("setup-invalid")
                .with_configuration(api_key_field(theme, "mcs-invalid-key", "••••••••")),
            ),
        ))
        .child(group(
            theme,
            "Pending submit",
            panel(
                ModelConnectionSetup::from_spec(
                    configure("openai-responses")
                        .with_can_submit(true)
                        .with_pending(true),
                    theme,
                )
                .with_instance_id("setup-pending")
                .with_configuration(api_key_field(theme, "mcs-pending-key", "••••••••")),
            ),
        ))
}
