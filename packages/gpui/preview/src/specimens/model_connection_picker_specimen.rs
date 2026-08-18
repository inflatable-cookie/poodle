use std::sync::Arc;

use crate::app_state::{
    AppState, ModelConnectionEvent, ModelConnectionPreviewState, NodeSpecimenEvent,
};
use crate::node_compat::{Eyebrow, ModelConnectionPicker};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::model_connection::ModelConnectionPickerState;
use poodle_specs::{EyebrowSpec, ModelConnectionPickerSpec};

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

fn spec() -> ModelConnectionPickerSpec {
    ModelConnectionPickerSpec::new().with_options(ModelConnectionPreviewState::options())
}

/// Every group on this page renders the same routes, so each instance needs
/// its own backend-state scope or they would share one focus handle per
/// option id.
fn scoped(
    spec: ModelConnectionPickerSpec,
    theme: &GpuiThemeProvider,
    scope: &str,
) -> ModelConnectionPicker {
    ModelConnectionPicker::from_spec(spec, theme).with_instance_id(scope)
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let queue = Arc::clone(&state.node_events);
    let _ = cx;

    // The grouped catalogue is live: selection and query run through the real
    // handlers and the preview's host loop, which owns both values.
    let interactive = ModelConnectionPicker::from_spec(
        spec()
            .with_value(
                state
                    .model_connection
                    .picker_value
                    .clone()
                    .or_else(|| Some("openai-responses".to_string())),
            )
            .with_query(state.model_connection.picker_query.clone()),
        theme,
    )
    .with_instance_id("picker-live")
    .on_value_change({
        let queue = Arc::clone(&queue);
        Arc::new(move |id: &str| {
            queue
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::ModelConnection(
                    ModelConnectionEvent::PickerValue(id.to_string()),
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
                    ModelConnectionEvent::PickerQuery(query.to_string()),
                ));
        })
    });

    div()
        .flex()
        .flex_col()
        .gap(px(32.0))
        .child(group(
            theme,
            "Grouped catalogue (many providers, one provider with several routes)",
            panel(interactive),
        ))
        // Availability is the host's classification. All four postures are on
        // their matching options above; nothing here probes for them.
        .child(group(
            theme,
            "Availability: available, checking, unavailable, unsupported",
            panel(scoped(
                spec().with_value(Some("openai-responses".to_string())),
                theme,
                "picker-availability",
            )),
        ))
        .child(group(
            theme,
            "Query with results",
            panel(scoped(
                spec()
                    .with_query("anthropic")
                    .with_value(Some("anthropic-messages".to_string())),
                theme,
                "picker-query-results",
            )),
        ))
        .child(group(
            theme,
            "Query with no results",
            panel(scoped(
                spec().with_query("zzzznothing"),
                theme,
                "picker-no-results",
            )),
        ))
        .child(group(
            theme,
            "Loading",
            panel(scoped(
                spec().with_state(ModelConnectionPickerState::Loading),
                theme,
                "picker-loading",
            )),
        ))
        .child(group(
            theme,
            "Error",
            panel(scoped(
                spec().with_state(ModelConnectionPickerState::Error),
                theme,
                "picker-error",
            )),
        ))
        .child(group(
            theme,
            "Empty catalogue",
            panel(scoped(
                ModelConnectionPickerSpec::new().with_state(ModelConnectionPickerState::Empty),
                theme,
                "picker-empty",
            )),
        ))
        // A host-supplied provider mark, keyed by option id, and a footer.
        // The generic mark stays on every option the host did not name.
        .child(group(
            theme,
            "Host provider marks and footer",
            panel(
                scoped(spec(), theme, "picker-host-content")
                    .with_leading("ollama-local", poodle_node::Node::icon("terminal", 16.0))
                    .with_footer(poodle_node::Node::text(
                        "Connections are managed by the host application.",
                    )),
            ),
        ))
        .child(group(
            theme,
            "Narrow layout",
            div().max_w(px(320.0)).child(scoped(
                spec().with_value(Some("ollama-local".to_string())),
                theme,
                "picker-narrow",
            )),
        ))
        .child(group(
            theme,
            "Disabled",
            panel(scoped(
                spec()
                    .with_value(Some("openai-responses".to_string()))
                    .with_disabled(true),
                theme,
                "picker-disabled",
            )),
        ))
}
