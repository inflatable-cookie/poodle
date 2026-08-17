use std::sync::Arc;

use crate::app_state::{AppState, ModelConnectionEvent, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, ModelCatalogueEditor};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::model_connection::{
    ModelCatalogueState, ModelCatalogueVisibilityChange,
};
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

fn posture(theme: &GpuiThemeProvider, state: ModelCatalogueState) -> ModelCatalogueEditor {
    ModelCatalogueEditor::from_spec(ModelCatalogueEditorSpec::new().with_state(state), theme)
}

/// The live editor: order, visibility, grab, drop target, hidden disclosure,
/// announcements and focus all round-trip through the preview's host loop.
fn interactive(state: &AppState, with_custom_action: bool) -> ModelCatalogueEditor {
    let theme = &state.theme;
    let queue = Arc::clone(&state.node_events);
    let host = &state.model_connection;

    let mut editor = ModelCatalogueEditor::from_spec(
        ModelCatalogueEditorSpec::new()
            .with_items(host.catalogue_items.clone())
            .with_grabbed(host.grabbed_id.clone())
            .with_drop_target(host.drop_target_id.clone())
            .with_hidden_open(host.hidden_open)
            .with_live_message(host.live_message.clone()),
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
    });

    if with_custom_action {
        editor = editor.with_custom_action(poodle_render::button(
            &ButtonSpec::new()
                .with_label("Add custom model")
                .with_variant(ButtonVariant::Secondary)
                .with_size(ControlSize::Sm),
            theme,
            None,
        ));
    }
    editor
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let _ = cx;

    div()
        .flex()
        .flex_col()
        .gap(px(32.0))
        .child(group(
            theme,
            "Shown and hidden models",
            panel(interactive(state, false)),
        ))
        // Same live editor with the host's leading marks and row metadata
        // keyed by opaque id.
        .child(group(
            theme,
            "Reorder-capable list with host content",
            panel(
                interactive(state, false)
                    .with_leading("model-gamma", Node::icon("star", 16.0))
                    .with_row_meta("model-gamma", Node::text("128k context")),
            ),
        ))
        // Two rows share the label "Shared Label" and stay distinct: identity
        // is the opaque id, and a display label was never identity.
        .child(group(
            theme,
            "Duplicate display labels",
            panel(interactive(state, false)),
        ))
        .child(group(
            theme,
            "Custom action",
            panel(interactive(state, true)),
        ))
        .child(group(
            theme,
            "Loading",
            panel(posture(theme, ModelCatalogueState::Loading)),
        ))
        .child(group(
            theme,
            "Unavailable",
            panel(posture(theme, ModelCatalogueState::Unavailable)),
        ))
        .child(group(
            theme,
            "Empty",
            panel(posture(theme, ModelCatalogueState::Empty)),
        ))
        .child(group(
            theme,
            "Error",
            panel(posture(theme, ModelCatalogueState::Error)),
        ))
        .child(group(
            theme,
            "Session negotiated",
            panel(posture(theme, ModelCatalogueState::SessionNegotiated)),
        ))
        .child(group(
            theme,
            "Pending mutation lock",
            panel(ModelCatalogueEditor::from_spec(
                ModelCatalogueEditorSpec::new()
                    .with_items(state.model_connection.catalogue_items.clone())
                    .with_pending(true),
                theme,
            )),
        ))
        .child(group(
            theme,
            "Drag disabled (keyboard and buttons remain)",
            panel(ModelCatalogueEditor::from_spec(
                ModelCatalogueEditorSpec::new()
                    .with_items(state.model_connection.catalogue_items.clone())
                    .with_drag_enabled(false),
                theme,
            )),
        ))
}
