use std::sync::Arc;

use crate::app_state::{AppState, LicenceSeatsEvent, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, LicenceSeats};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::licence::LicenceSeat;
use poodle_specs::{EyebrowSpec, LicenceSeatsSpec};

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

fn seat(machine_id: &str, label: Option<&str>, this_machine: bool) -> LicenceSeat {
    LicenceSeat {
        machine_id: machine_id.to_string(),
        label: label.map(str::to_string),
        this_machine,
    }
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let queue = Arc::clone(&state.node_events);
    let _ = cx;

    let restore_machine = state.licence_seats.request_focus_machine_id.clone();
    if restore_machine.is_some() {
        queue.lock().unwrap().push(NodeSpecimenEvent::LicenceSeats(
            LicenceSeatsEvent::ClearRestoreFocus,
        ));
    }

    // The mixed group is interactive: rename, edit, and release run through
    // the real handlers and the preview's host loop.
    let mixed =
        LicenceSeats::from_spec(
            LicenceSeatsSpec::new()
                .with_seats(state.licence_seats.seats.clone())
                .with_editing_machine(state.licence_seats.editing_machine_id.clone())
                .with_editing_draft(state.licence_seats.editing_draft.clone())
                .with_editing_selection(
                    state.licence_seats.editing_selection.0,
                    state.licence_seats.editing_selection.1,
                )
                .with_request_focus_machine(restore_machine)
                .with_open_confirm(state.licence_seats.open_confirm_machine_id.clone()),
            theme,
        )
        .on_rename_edit({
            let queue = Arc::clone(&queue);
            Arc::new(move |machine_id: &str| {
                queue.lock().unwrap().push(NodeSpecimenEvent::LicenceSeats(
                    LicenceSeatsEvent::Edit {
                        machine_id: machine_id.to_string(),
                    },
                ));
            })
        })
        .on_rename({
            let queue = Arc::clone(&queue);
            Arc::new(move |machine_id: &str, label: Option<&str>| {
                queue.lock().unwrap().push(NodeSpecimenEvent::LicenceSeats(
                    LicenceSeatsEvent::Rename {
                        machine_id: machine_id.to_string(),
                        label: label.map(str::to_string),
                    },
                ));
            })
        })
        .on_rename_change({
            let queue = Arc::clone(&queue);
            Arc::new(move |machine_id: &str, draft: &str| {
                queue.lock().unwrap().push(NodeSpecimenEvent::LicenceSeats(
                    LicenceSeatsEvent::Draft {
                        machine_id: machine_id.to_string(),
                        draft: draft.to_string(),
                    },
                ));
            })
        })
        .on_rename_selection_change({
            let queue = Arc::clone(&queue);
            Arc::new(move |machine_id: &str, start: usize, end: usize| {
                queue.lock().unwrap().push(NodeSpecimenEvent::LicenceSeats(
                    LicenceSeatsEvent::Selection {
                        machine_id: machine_id.to_string(),
                        start,
                        end,
                    },
                ));
            })
        })
        .on_rename_cancel({
            let queue = Arc::clone(&queue);
            Arc::new(move |_machine_id: &str| {
                queue.lock().unwrap().push(NodeSpecimenEvent::LicenceSeats(
                    LicenceSeatsEvent::CancelEdit,
                ));
            })
        })
        .on_rename_restore_display_focus({
            let queue = Arc::clone(&queue);
            Arc::new(move |machine_id: &str| {
                queue.lock().unwrap().push(NodeSpecimenEvent::LicenceSeats(
                    LicenceSeatsEvent::RestoreFocus {
                        machine_id: machine_id.to_string(),
                    },
                ));
            })
        })
        .on_release_trigger({
            let queue = Arc::clone(&queue);
            Arc::new(move |machine_id: &str| {
                queue.lock().unwrap().push(NodeSpecimenEvent::LicenceSeats(
                    LicenceSeatsEvent::ReleaseTrigger {
                        machine_id: machine_id.to_string(),
                    },
                ));
            })
        })
        .on_release({
            let queue = Arc::clone(&queue);
            Arc::new(move |machine_id: &str| {
                queue.lock().unwrap().push(NodeSpecimenEvent::LicenceSeats(
                    LicenceSeatsEvent::ReleaseConfirm {
                        machine_id: machine_id.to_string(),
                    },
                ));
            })
        })
        .on_release_cancel({
            let queue = Arc::clone(&queue);
            Arc::new(move |_machine_id: &str| {
                queue.lock().unwrap().push(NodeSpecimenEvent::LicenceSeats(
                    LicenceSeatsEvent::ReleaseCancel,
                ));
            })
        });

    let unnamed = vec![
        seat("cmd-2b90fe14", None, true),
        seat("cmd-6d17c3aa", None, false),
        seat("cmd-b04f9e51", None, false),
    ];
    let single = vec![seat("cmd-9f3a2b7c", Some("Studio Mac"), true)];
    let pending = vec![
        seat("cmd-9f3a2b7c", Some("Studio Mac"), true),
        seat("cmd-41ee80d2", Some("Tour laptop"), false),
        seat("cmd-77c1a5be", None, false),
    ];
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(420.0))
        .child(group(theme, "Mixed labels", mixed))
        // Two unnamed rows look alike, and stay that way. Inventing a
        // hostname to tell them apart would be claiming identity Poodle was
        // never given.
        .child(group(
            theme,
            "Unnamed machines",
            LicenceSeats::from_spec(LicenceSeatsSpec::new().with_seats(unnamed), theme),
        ))
        // This machine only: a marker, and no release action anywhere.
        .child(group(
            theme,
            "This machine only",
            LicenceSeats::from_spec(LicenceSeatsSpec::new().with_seats(single), theme),
        ))
        .child(group(
            theme,
            "Pending release",
            LicenceSeats::from_spec(
                LicenceSeatsSpec::new()
                    .with_seats(pending)
                    .with_pending_machine_id(Some("cmd-41ee80d2".to_string())),
                theme,
            ),
        ))
        .child(group(
            theme,
            "Direct release",
            LicenceSeats::from_spec(
                LicenceSeatsSpec::new()
                    .with_seats(vec![
                        seat("cmd-9f3a2b7c", Some("Studio Mac"), true),
                        seat("cmd-41ee80d2", Some("Tour laptop"), false),
                        seat("cmd-77c1a5be", None, false),
                    ])
                    .with_confirm_release(false),
                theme,
            ),
        ))
        // Renders nothing: no heading, no list, no invented seat count.
        .child(group(
            theme,
            "Empty authority",
            LicenceSeats::from_spec(LicenceSeatsSpec::new(), theme),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "licence-seats",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                LicenceSeats::from_spec(
                    LicenceSeatsSpec::new()
                        .with_seats(vec![
                            seat("machine-a", Some("Studio desktop"), true),
                            seat("machine-b", Some("Laptop"), false),
                        ])
                        .with_size(size),
                    theme,
                )
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                LicenceSeats::from_spec(
                    LicenceSeatsSpec::new()
                        .with_seats(vec![
                            seat("machine-a", Some("Studio desktop"), true),
                            seat("machine-b", Some("Laptop"), false),
                        ])
                        .with_density(density),
                    theme,
                )
                .into_any_element()
            }),
    )
}
