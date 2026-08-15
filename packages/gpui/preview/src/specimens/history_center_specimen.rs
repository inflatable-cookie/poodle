//! HistoryCenter specimen (g14.007). The same composition the conformance
//! corpus executes, driven by the preview's own toggle state: disclose a fork,
//! pick between two, check one out, rename a branch.
//!
//! The history is fixture data, not a store. Every operation leaves as a
//! command and the specimen answers it the way a host would — which is the
//! whole point of the component.

use std::sync::Arc;

use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::PreviewRoot;
use gpui::*;
use poodle_headless::history_center::{
    history_center_visible_rows, HistoryCenterOpenFork, HistoryContinuation, HistoryEntry,
    HistoryEntryPosition, HistoryPathPage,
};
use poodle_render::{history_center, HistoryCenterHandlers, HistoryCenterRename, HistoryCenterView};
use poodle_specs::HistoryCenterSpec;

const ANCHOR: &str = "e2";

fn entry(id: &str, label: &str, position: HistoryEntryPosition, count: usize) -> HistoryEntry {
    HistoryEntry::new(id, label)
        .with_position(position)
        .with_continuation_count(count)
}

/// The spine, newest first — the order a page arrives in.
fn pages() -> Vec<HistoryPathPage> {
    vec![HistoryPathPage::new(vec![
        entry("e3", "Raise gain", HistoryEntryPosition::Future, 0),
        entry("e2", "Trim tail", HistoryEntryPosition::Current, 3),
        entry(
            "e1",
            "Import stems",
            HistoryEntryPosition::Past,
            1,
        )
        .with_checkpoint(true),
    ])]
}

/// The forks the host would answer `loadContinuations` with, the child already
/// on the list included — filtering it is the component's job.
fn continuations() -> Vec<HistoryContinuation> {
    vec![
        HistoryContinuation::new("e3", "Raise gain", "main").with_preferred(true),
        HistoryContinuation::new("f1", "Widen stereo", "branch-wide")
            .with_entry_count(2)
            .with_branch_name("Wide mix"),
        HistoryContinuation::new("f2", "Duck bass", "branch-duck").with_branch_name("Duck bass"),
    ]
}

fn run_pages(fork: &str) -> Vec<HistoryPathPage> {
    let entries = match fork {
        "f1" => vec![
            entry("f1b", "Add shimmer", HistoryEntryPosition::Future, 0),
            entry("f1", "Widen stereo", HistoryEntryPosition::Future, 1),
        ],
        "f2" => vec![entry("f2", "Duck bass", HistoryEntryPosition::Future, 0)],
        _ => Vec::new(),
    };
    if entries.is_empty() {
        Vec::new()
    } else {
        vec![HistoryPathPage::new(entries)]
    }
}

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let is_open = state.specimens.is_on("history-center-open");
    let is_disclosed = state.specimens.is_on("history-center-disclosed");
    let is_select_open = state.specimens.is_on("history-center-select-open");
    let is_actions_open = state.specimens.is_on("history-center-actions-open");
    let picked = state
        .specimens
        .text
        .get("history-center-pick")
        .cloned()
        .unwrap_or_else(|| "f1".into());
    let last_command = state
        .specimens
        .text
        .get("history-center-command")
        .cloned()
        .unwrap_or_else(|| "none".into());

    let open_levels = if is_disclosed {
        let shown = continuations()
            .into_iter()
            .find(|fork| fork.entry_id == picked);
        vec![HistoryCenterOpenFork {
            anchor_entry_id: ANCHOR.to_owned(),
            continuations: Some(continuations()),
            pick: shown.clone(),
            chosen: None,
            run_pages: shown
                .map(|fork| run_pages(&fork.entry_id))
                .unwrap_or_default(),
            inner: Vec::new(),
        }]
    } else {
        Vec::new()
    };

    let spec = HistoryCenterSpec::new()
        .with_can_undo(true)
        .with_can_redo(true);
    let view = HistoryCenterView {
        is_open,
        rows: history_center_visible_rows(Some(&pages()), &open_levels),
        open_anchors: open_levels
            .iter()
            .map(|level| level.anchor_entry_id.clone())
            .collect(),
        open_select_anchor: is_select_open.then(|| ANCHOR.to_owned()),
        open_actions_anchor: is_actions_open.then(|| ANCHOR.to_owned()),
        rename: state.specimens.is_on("history-center-renaming").then(|| {
            HistoryCenterRename {
                anchor_entry_id: ANCHOR.to_owned(),
                branch_id: "branch-wide".to_owned(),
                value: state
                    .specimens
                    .text
                    .get("history-center-rename")
                    .cloned()
                    .unwrap_or_else(|| "Wide mix".into()),
            }
        }),
        ..HistoryCenterView::default()
    };

    let queue = state.node_events.clone();
    let toggle = |queue: &Arc<std::sync::Mutex<Vec<NodeSpecimenEvent>>>, key: &str, value: bool| {
        queue.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
            key: key.into(),
            value,
        });
    };

    let open_queue = queue.clone();
    let disclose_queue = queue.clone();
    let select_queue = queue.clone();
    let pick_queue = queue.clone();
    let actions_queue = queue.clone();
    let rename_queue = queue.clone();
    let rename_key_queue = queue.clone();
    let checkout_queue = queue.clone();
    let navigate_queue = queue.clone();
    let undo_queue = queue.clone();
    let redo_queue = queue;

    let handlers = HistoryCenterHandlers {
        on_undo: Some(Arc::new(move || {
            undo_queue.lock().unwrap().push(NodeSpecimenEvent::SetText {
                key: "history-center-command".into(),
                value: "undo".into(),
            });
        })),
        on_redo: Some(Arc::new(move || {
            redo_queue.lock().unwrap().push(NodeSpecimenEvent::SetText {
                key: "history-center-command".into(),
                value: "redo".into(),
            });
        })),
        on_open_change: Some(Arc::new(move |value| {
            toggle(&open_queue, "history-center-open", value);
        })),
        on_activate_row: Some(Arc::new(move |row| {
            navigate_queue
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::SetText {
                    key: "history-center-command".into(),
                    value: format!("navigate {}", row.entry_id),
                });
        })),
        on_disclose: Some(Arc::new(move |_entry| {
            toggle(&disclose_queue, "history-center-disclosed", !is_disclosed);
        })),
        on_select_toggle: Some(Arc::new(move |_anchor| {
            toggle(&select_queue, "history-center-select-open", !is_select_open);
        })),
        on_pick: Some(Arc::new(move |entry_id| {
            let mut events = pick_queue.lock().unwrap();
            events.push(NodeSpecimenEvent::SetText {
                key: "history-center-pick".into(),
                value: entry_id.into(),
            });
            events.push(NodeSpecimenEvent::SetToggle {
                key: "history-center-select-open".into(),
                value: false,
            });
        })),
        on_actions_toggle: Some(Arc::new(move |_anchor| {
            toggle(&actions_queue, "history-center-actions-open", !is_actions_open);
        })),
        on_checkout: Some(Arc::new(move |_anchor| {
            let mut events = checkout_queue.lock().unwrap();
            events.push(NodeSpecimenEvent::SetText {
                key: "history-center-command".into(),
                value: "checkout".into(),
            });
            events.push(NodeSpecimenEvent::SetToggle {
                key: "history-center-actions-open".into(),
                value: false,
            });
        })),
        on_rename_open: Some(Arc::new(move |_anchor| {
            let mut events = rename_queue.lock().unwrap();
            events.push(NodeSpecimenEvent::SetToggle {
                key: "history-center-renaming".into(),
                value: true,
            });
            events.push(NodeSpecimenEvent::SetToggle {
                key: "history-center-actions-open".into(),
                value: false,
            });
        })),
        on_rename_key: Some(Arc::new(move |key| {
            if key == "enter" || key == "escape" {
                rename_key_queue
                    .lock()
                    .unwrap()
                    .push(NodeSpecimenEvent::SetToggle {
                        key: "history-center-renaming".into(),
                        value: false,
                    });
            }
        })),
        ..HistoryCenterHandlers::default()
    };

    let center = poodle_gpui_node_backend::to_gpui(&history_center(
        &spec,
        &state.theme,
        &view,
        &handlers,
    ));

    div()
        .flex()
        .flex_col()
        .gap(px(12.0))
        .min_h(px(560.0))
        .child(center)
        .child(format!("Last command: {last_command}"))
}
