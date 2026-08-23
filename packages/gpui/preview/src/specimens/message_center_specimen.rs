use std::sync::Arc;

use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::Eyebrow;
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_render::{RenderContext, message_center, MessageCenterHandlers};
use poodle_specs::{
    EyebrowSpec, MessageCenterItem, MessageCenterItemProgress, MessageCenterSpec, OverlayPlacement,
    StatusTone,
};

const IDS: &[&str] = &["render", "mention", "maintenance"];

/// Two ordinary rows the axis representatives read from. The Examples pane
/// keeps the fuller, interactive set.
fn axis_items() -> Vec<MessageCenterItem> {
    vec![
        MessageCenterItem::new("axis-render", "Render complete")
            .with_message("Mix preview 42 is ready for review.")
            .with_meta("Render queue"),
        MessageCenterItem::new("axis-mention", "Maya mentioned you")
            .with_message("Can you check the limiter settings before export?")
            .with_meta("Studio chat"),
    ]
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let items = vec![
        MessageCenterItem::new("job-render", "Mix preview")
            .with_message("Rendering stems and automation…")
            .with_meta("Render queue · 60%")
            .with_progress(MessageCenterItemProgress::determinate(60.0))
            .as_live_row(),
        MessageCenterItem::new("job-upload", "Uploading stems")
            .with_message("Three of four stems transferred.")
            .with_meta("Cloud sync")
            .with_progress(MessageCenterItemProgress::indeterminate())
            .as_live_row(),
        MessageCenterItem::new("render", "Render complete")
            .with_message("Mix preview 42 is ready for review.")
            .with_meta("Render queue")
            .with_timestamp("2026-08-11T09:40:00Z")
            .with_tone(StatusTone::Success),
        MessageCenterItem::new("mention", "Maya mentioned you")
            .with_message("Can you check the limiter settings before export?")
            .with_meta("Studio chat")
            .with_timestamp("2026-08-11T09:20:00Z"),
        MessageCenterItem::new("maintenance", "Maintenance scheduled")
            .with_message("Workstation services restart tonight at 23:00.")
            .with_meta("Operations")
            .with_timestamp("2026-08-10T15:00:00Z")
            .with_tone(StatusTone::Warning)
            .with_read(true),
    ]
    .into_iter()
    .filter(|item| {
        !state
            .specimens
            .is_on(&format!("message-center-removed-{}", item.id))
    })
    .map(|mut item| {
        let key = format!("message-center-read-{}", item.id);
        if state.specimens.toggles.contains_key(&key) {
            item.read = state.specimens.is_on(&key);
        }
        item
    })
    .collect();

    let open = state.specimens.is_on("message-center-open");
    let queue = state.node_events.clone();
    let open_queue = queue.clone();
    let read_queue = queue.clone();
    let remove_queue = queue.clone();
    let mark_queue = queue.clone();
    let select_queue = queue;
    let handlers = MessageCenterHandlers {
        on_open_change: Some(Arc::new(move |value| {
            open_queue
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::SetToggle {
                    key: "message-center-open".into(),
                    value,
                });
        })),
        on_item_select: Some(Arc::new(move |id| {
            select_queue
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::SetText {
                    key: "message-center-selected".into(),
                    value: id.into(),
                });
        })),
        on_read_change: Some(Arc::new(move |id, read| {
            read_queue
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::SetToggle {
                    key: format!("message-center-read-{id}"),
                    value: read,
                });
        })),
        on_remove: Some(Arc::new(move |id| {
            remove_queue
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::SetToggle {
                    key: format!("message-center-removed-{id}"),
                    value: true,
                });
        })),
        on_mark_all_read: Some(Arc::new(move || {
            let mut events = mark_queue.lock().unwrap();
            for id in IDS {
                events.push(NodeSpecimenEvent::SetToggle {
                    key: format!("message-center-read-{id}"),
                    value: true,
                });
            }
        })),
    };

    let spec = MessageCenterSpec::new(items)
        .with_open(open)
        .with_placement(OverlayPlacement::BottomStart);
    let center = poodle_gpui_node_backend::to_gpui(&message_center(
        &spec,
        &RenderContext::new(&state.theme),
        handlers,
    ));
    let selected = state
        .specimens
        .text
        .get("message-center-selected")
        .cloned()
        .unwrap_or_else(|| "none".into());
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(12.0))
        .min_h(px(520.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new()
                .with_content("Live jobs, unread mentions, and a read maintenance notice"),
            &state.theme,
        ))
        .child(center)
        .child(format!("Selected message: {selected}"))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "message-center",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                poodle_gpui_node_backend::to_gpui(&message_center(
                    &MessageCenterSpec::new(axis_items())
                        .with_placement(OverlayPlacement::BottomStart)
                        .with_size(size),
                    &RenderContext::new(theme),
                    MessageCenterHandlers::default(),
                ))
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                poodle_gpui_node_backend::to_gpui(&message_center(
                    &MessageCenterSpec::new(axis_items())
                        .with_placement(OverlayPlacement::BottomStart)
                        .with_density(density),
                    &RenderContext::new(theme),
                    MessageCenterHandlers::default(),
                ))
            }),
    )
}
