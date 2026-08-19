use std::sync::Arc;

use crate::app_state::{AppState, ModelConnectionEvent, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, ModelCatalogueEditor};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::model_connection::{ModelCatalogueState, ModelCatalogueVisibilityChange};
use poodle_node::Node;
use poodle_specs::{ButtonSpec, ButtonVariant, ControlSize, EyebrowSpec, ModelCatalogueEditorSpec};

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
    div().max_w(px(576.0)).child(child)
}

fn posture(
    theme: &GpuiThemeProvider,
    state: ModelCatalogueState,
    scope: &str,
) -> ModelCatalogueEditor {
    ModelCatalogueEditor::from_spec(ModelCatalogueEditorSpec::new().with_state(state), theme)
        .with_instance_id(scope)
}

/// The live editor: order, visibility, grab, drop target, hidden disclosure,
/// announcements and focus all round-trip through the preview's host loop.
fn interactive(
    state: &AppState,
    scope: &str,
    tweak: impl FnOnce(ModelCatalogueEditorSpec) -> ModelCatalogueEditorSpec,
) -> ModelCatalogueEditor {
    let theme = &state.theme;
    let queue = Arc::clone(&state.node_events);
    let host = &state.model_connection;

    let editor = ModelCatalogueEditor::from_spec(
        tweak(
            ModelCatalogueEditorSpec::new()
                .with_items(host.catalogue_items.clone())
                .with_grabbed(host.grabbed_id.clone())
                .with_drop_target(host.drop_target_id.clone())
                .with_hidden_open(host.hidden_open)
                .with_live_message(host.live_message.clone()),
        ),
        theme,
    )
    .on_order_change({
        let queue = Arc::clone(&queue);
        Arc::new(move |order: &[String]| {
            queue
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::ModelConnection(
                    ModelConnectionEvent::CatalogueOrder(order.to_vec()),
                ));
        })
    })
    .on_visibility_change({
        let queue = Arc::clone(&queue);
        Arc::new(move |change: &ModelCatalogueVisibilityChange| {
            queue
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::ModelConnection(
                    ModelConnectionEvent::CatalogueVisibility {
                        id: change.id.clone(),
                        visible: change.visible,
                    },
                ));
        })
    })
    .on_info({
        let queue = Arc::clone(&queue);
        Arc::new(move |id: &str| {
            queue
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::ModelConnection(
                    ModelConnectionEvent::CatalogueInfo(id.to_string()),
                ));
        })
    })
    .on_grab_change({
        let queue = Arc::clone(&queue);
        Arc::new(move |id: Option<&str>| {
            queue
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::ModelConnection(
                    ModelConnectionEvent::CatalogueGrab(id.map(str::to_string)),
                ));
        })
    })
    .on_drop_target_change({
        let queue = Arc::clone(&queue);
        Arc::new(move |id: Option<&str>| {
            queue
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::ModelConnection(
                    ModelConnectionEvent::CatalogueDropTarget(id.map(str::to_string)),
                ));
        })
    })
    .on_hidden_open_change({
        let queue = Arc::clone(&queue);
        Arc::new(move |open: bool| {
            queue
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::ModelConnection(
                    ModelConnectionEvent::CatalogueHiddenOpen(open),
                ));
        })
    })
    .on_announce({
        let queue = Arc::clone(&queue);
        Arc::new(move |message: &str| {
            queue
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::ModelConnection(
                    ModelConnectionEvent::CatalogueAnnounce(message.to_string()),
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
    // Four live editors share one host state; without a per-instance scope
    // they would share one focus handle per item id too.
    .with_instance_id(scope);

    editor
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let _ = cx;

    div()
        .flex()
        .flex_col()
        .gap(px(32.0))
        // Two rows share the label "Shared Label" and stay distinct: identity
        // is the opaque id, and a display label was never identity.
        .child(group(
            theme,
            "Shown and hidden models",
            panel(interactive(state, "catalogue-main", |spec| spec)),
        ))
        // Pointer drag, keyboard grab and explicit move buttons are three
        // routes to the same reorder; a host may switch either off.
        .child(group(
            theme,
            "Reorder and visibility controls",
            div()
                .flex()
                .flex_col()
                .gap(px(16.0))
                .child(panel(interactive(state, "catalogue-no-drag", |spec| {
                    spec.with_drag_enabled(false)
                })))
                .child(panel(interactive(state, "catalogue-no-buttons", |spec| {
                    spec.with_move_actions(false)
                }))),
        ))
        // Host composition: the leading mark and row metadata are keyed by
        // opaque id, and the custom action is the host's own.
        .child(group(
            theme,
            "Host mark, actions, and row metadata",
            panel(
                interactive(state, "catalogue-host-content", |spec| spec)
                    .with_leading("model-gamma", Node::icon("star", 16.0))
                    .with_row_meta("model-gamma", Node::text("128k context"))
                    .with_custom_action(poodle_render::button(
                        &ButtonSpec::new()
                            .with_label("Add custom model")
                            .with_variant(ButtonVariant::Secondary)
                            .with_size(ControlSize::Sm),
                        theme,
                        None,
                    )),
            ),
        ))
        .child(group(
            theme,
            "Loading and pending",
            div()
                .flex()
                .flex_col()
                .gap(px(16.0))
                .child(panel(posture(
                    theme,
                    ModelCatalogueState::Loading,
                    "catalogue-loading",
                )))
                // A mutation lock leaves the list readable and every control inert.
                .child(panel(
                    ModelCatalogueEditor::from_spec(
                        ModelCatalogueEditorSpec::new()
                            .with_items(state.model_connection.catalogue_items.clone())
                            .with_pending(true),
                        theme,
                    )
                    .with_instance_id("catalogue-pending"),
                )),
        ))
        .child(group(
            theme,
            "Empty catalogue",
            panel(posture(
                theme,
                ModelCatalogueState::Empty,
                "catalogue-empty",
            )),
        ))
        .child(group(
            theme,
            "Unavailable, error, and session-negotiated",
            div()
                .flex()
                .flex_col()
                .gap(px(16.0))
                .child(panel(posture(
                    theme,
                    ModelCatalogueState::Unavailable,
                    "catalogue-unavailable",
                )))
                .child(panel(posture(
                    theme,
                    ModelCatalogueState::Error,
                    "catalogue-error",
                )))
                .child(panel(posture(
                    theme,
                    ModelCatalogueState::SessionNegotiated,
                    "catalogue-session",
                ))),
        ))
}
