use std::sync::Arc;

use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, LicenceActivation};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_node_backend::file_capability::SingleFilePickSpec;
use poodle_headless::licence::{
    LicenceActivationMode, LicenceActivationRoute, LicenceKeyFormat, LicenceKeyProblem,
    LicenceKeyResult, LicenceSubmitDraft, resolve_licence_submit,
};
use poodle_specs::{
    EyebrowSpec, FieldSpec, LicenceActivationSpec, TextInputSpec, ValidationState,
};

/// Stand-in for the host's key parser (the web specimen's, ported). The real
/// parser belongs to the authority — the specimen shows Poodle works against
/// any parser satisfying the interface, and imports neither.
struct SpecimenKeyFormat;

impl LicenceKeyFormat for SpecimenKeyFormat {
    fn parse(&self, input: &str) -> LicenceKeyResult {
        let stripped: String = input
            .chars()
            .filter(|c| *c != '-' && !c.is_whitespace())
            .collect();
        if let Some(symbol) = stripped.chars().find(|c| !c.is_ascii_alphanumeric()) {
            return LicenceKeyResult::Err(LicenceKeyProblem::UnexpectedSymbol {
                symbol: symbol.to_string(),
            });
        }
        let length = stripped.chars().count();
        if length < 20 {
            return LicenceKeyResult::Err(LicenceKeyProblem::TooShort {
                minimum: 20,
                actual: length,
            });
        }
        LicenceKeyResult::Ok {
            key: stripped.to_uppercase(),
            grouped: stripped.to_uppercase(),
        }
    }

    fn is_probably_a_typo(&self, problem: &LicenceKeyProblem) -> bool {
        matches!(
            problem,
            LicenceKeyProblem::CheckFailed
                | LicenceKeyProblem::UnexpectedSymbol { .. }
        )
    }
}

/// A caption for an emitted credential — the kind only, never the contents.
fn emitted_caption(credential: &poodle_headless::licence::LicenceCredential) -> String {
    match credential {
        poodle_headless::licence::LicenceCredential::Key { .. } => {
            "Emitted licence key".to_string()
        }
        poodle_headless::licence::LicenceCredential::AccountToken { .. } => {
            "Emitted account token".to_string()
        }
        poodle_headless::licence::LicenceCredential::LicenceFile { .. } => {
            "Emitted licence file".to_string()
        }
    }
}

fn group(theme: &GpuiThemeProvider, label: &str, specimen: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(EyebrowSpec::new().with_content(label), theme))
        .child(specimen)
}

/// Build the four machine-name handlers shared by the key and embedded
/// instances. Edit start snaps the committed value; typing updates the
/// controlled draft; commit keeps the draft and clears the snapshot; Escape
/// restores the snapshot (the web EditableLabel reverts the draft).
fn label_handlers(
    queue: &Arc<std::sync::Mutex<Vec<NodeSpecimenEvent>>>,
    committed: &str,
) -> (
    Arc<dyn Fn() + Send + Sync>,
    Arc<dyn Fn(&str) + Send + Sync>,
    Arc<dyn Fn(&str) + Send + Sync>,
    Arc<dyn Fn() + Send + Sync>,
) {
    let committed = committed.to_string();

    let edit = {
        let queue = Arc::clone(queue);
        let committed = committed.clone();
        Arc::new(move || {
            queue
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::SetOptionalText {
                    key: "la-machine-label-original".to_string(),
                    value: Some(committed.clone()),
                });
            queue
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::SetToggle {
                    key: "la-machine-editing".to_string(),
                    value: true,
                });
        })
    };
    let change = {
        let queue = Arc::clone(queue);
        Arc::new(move |value: &str| {
            queue.lock().unwrap().push(NodeSpecimenEvent::SetText {
                key: "la-machine-label".to_string(),
                value: value.to_string(),
            });
        })
    };
    let commit = {
        let queue = Arc::clone(queue);
        Arc::new(move |_value: &str| {
            queue
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::SetOptionalText {
                    key: "la-machine-label-original".to_string(),
                    value: None,
                });
            queue
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::SetToggle {
                    key: "la-machine-editing".to_string(),
                    value: false,
                });
        })
    };
    let cancel = {
        let queue = Arc::clone(queue);
        Arc::new(move || {
            queue
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::MachineLabelCancel);
        })
    };
    (edit, change, commit, cancel)
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let queue = Arc::clone(&state.node_events);
    let _ = cx;

    let key_draft = state
        .specimens
        .text
        .get("la-key")
        .cloned()
        .unwrap_or_default();
    let key_message = state.specimens.text.get("la-key-message").cloned();
    let machine_label = state
        .specimens
        .text
        .get("la-machine-label")
        .cloned()
        .unwrap_or_default();
    let machine_editing = state.specimens.is_on("la-machine-editing");
    let offline = state.specimens.is_on("la-offline");
    let route = if offline {
        LicenceActivationRoute::LicenceFile
    } else {
        LicenceActivationRoute::AccountToken
    };
    let file_name = state.specimens.text.get("la-file-name").cloned();
    let file_contents_base64 = state.specimens.text.get("la-file-base64").cloned();
    let file_error = state.specimens.text.get("la-file-error").cloned();
    let emitted = state.specimens.text.get("la-emitted").cloned();

    let key_caption = match (&key_message, &emitted) {
        (Some(message), _) => message.clone(),
        (_, Some(caption)) => caption.clone(),
        _ => format!("Entered: {}", key_draft),
    };

    // The interactive grouped key activation: type into the CodeInput, watch
    // the parser's tick/cross, submit through the shared resolver.
    let key_submit = {
        let queue = Arc::clone(&queue);
        let key_draft = key_draft.clone();
        let machine_label = machine_label.clone();
        Arc::new(move || {
            let draft = LicenceSubmitDraft {
                route: LicenceActivationRoute::Key,
                key: key_draft.clone(),
                token: None,
                file_contents_base64: None,
                label: machine_label.clone(),
            };
            match resolve_licence_submit(&draft, Some(&SpecimenKeyFormat)) {
                poodle_headless::licence::LicenceSubmitResolution::Emit {
                    credential, ..
                } => {
                    queue.lock().unwrap().push(NodeSpecimenEvent::SetText {
                        key: "la-key-message".to_string(),
                        value: String::new(),
                    });
                    queue.lock().unwrap().push(NodeSpecimenEvent::SetText {
                        key: "la-emitted".to_string(),
                        value: emitted_caption(&credential),
                    });
                }
                poodle_headless::licence::LicenceSubmitResolution::Reject { message } => {
                    queue.lock().unwrap().push(NodeSpecimenEvent::SetText {
                        key: "la-key-message".to_string(),
                        value: message,
                    });
                }
                poodle_headless::licence::LicenceSubmitResolution::Quiet => {}
            }
        })
    };

    let interactive_key = LicenceActivation::from_spec(
        LicenceActivationSpec::new()
            .with_mode(LicenceActivationMode::Key)
            .with_key_code_input(
                poodle_specs::LicenceKeyCodeInputOptions::new(20)
                    .with_groups([5, 5, 5, 5])
                    .with_separator("-"),
            )
            .with_key_draft(key_draft.clone())
            .with_key_message(key_message.clone())
            .with_machine_label(Some(machine_label.clone()))
            .with_machine_label_editing(machine_editing)
            .with_key_selection(
                state.specimens.carets.get("la-key").map(|c| c.0).unwrap_or(0),
                state.specimens.carets.get("la-key").map(|c| c.1).unwrap_or(0),
            ),
        theme,
    )
    .on_key_change({
        let queue = Arc::clone(&queue);
        Arc::new(move |value: &str| {
            queue.lock().unwrap().push(NodeSpecimenEvent::SetText {
                key: "la-key".to_string(),
                value: value.to_string(),
            });
            // Editing the key clears the local validation copy — the web pair
            // clears keyMessage on every change, so a stale rejection must
            // never survive against a new key.
            queue.lock().unwrap().push(NodeSpecimenEvent::SetText {
                key: "la-key-message".to_string(),
                value: String::new(),
            });
        })
    })
    .on_key_selection_change({
        let queue = Arc::clone(&queue);
        Arc::new(move |start: usize, end: usize| {
            queue.lock().unwrap().push(NodeSpecimenEvent::SetCaret {
                key: "la-key".to_string(),
                start,
                end,
            });
        })
    })
    .on_key_check(Arc::new(|input: &str| SpecimenKeyFormat.parse(input)))
    .on_machine_label_edit({
        let (edit, _change, _commit, _cancel) =
            label_handlers(&queue, machine_label.as_str());
        edit
    })
    .on_machine_label_change({
        let (_edit, change, _commit, _cancel) = label_handlers(&queue, machine_label.as_str());
        change
    })
    .on_machine_label_commit({
        let (_edit, _change, commit, _cancel) = label_handlers(&queue, machine_label.as_str());
        commit
    })
    .on_machine_label_cancel({
        let (_edit, _change, _commit, cancel) = label_handlers(&queue, machine_label.as_str());
        cancel
    })
    .on_submit(key_submit);

    // Embedded account activation: host-owned account content beside the
    // spec, driven by Poodle's submit.
    let email = state.specimens.text.get("la-email").cloned().unwrap_or_default();
    let password = state.specimens.text.get("la-password").cloned().unwrap_or_default();
    let email_field = poodle_render::field(
        &FieldSpec::new("la-email", "Email address"),
        theme,
        Some(poodle_render::text_input_with_handlers(
            &TextInputSpec {
                value: Some(email.clone()),
                validation_state: ValidationState::None,
                size: Some(poodle_specs::ControlSize::Md),
                ..TextInputSpec::default()
            },
            theme,
            poodle_render::TextInputHandlers {
                on_change: Some({
                    let queue = Arc::clone(&queue);
                    Arc::new(move |value: &str| {
                        queue.lock().unwrap().push(NodeSpecimenEvent::SetText {
                            key: "la-email".to_string(),
                            value: value.to_string(),
                        });
                    })
                }),
                ..poodle_render::TextInputHandlers::default()
            },
        )),
    );
    let password_field = poodle_render::field(
        &FieldSpec::new("la-password", "Password"),
        theme,
        Some(poodle_render::text_input_with_handlers(
            &TextInputSpec {
                value: Some(password.clone()),
                validation_state: ValidationState::None,
                size: Some(poodle_specs::ControlSize::Md),
                ..TextInputSpec::default()
            },
            theme,
            poodle_render::TextInputHandlers {
                on_change: Some({
                    let queue = Arc::clone(&queue);
                    Arc::new(move |value: &str| {
                        queue.lock().unwrap().push(NodeSpecimenEvent::SetText {
                            key: "la-password".to_string(),
                            value: value.to_string(),
                        });
                    })
                }),
                ..poodle_render::TextInputHandlers::default()
            },
        )),
    );
    let account_content = poodle_node::Node::container()
        .child(email_field)
        .child(password_field);

    let account_message = state.specimens.text.get("la-account-message").cloned();
    let embedded_route_message = file_error.clone().or(account_message.clone());

    // The embedded specimen's Activate is driven by Poodle's submit: account
    // mode fires a host-owned acquisition request against the host form
    // state (the stand-in provider cancels, so no token is emitted), and the
    // offline route runs the shared resolver against the selected file.
    let embedded_submit = {
        let queue = Arc::clone(&queue);
        let file_b64 = file_contents_base64.clone();
        let machine_label = machine_label.clone();
        Arc::new(move || {
            if route == LicenceActivationRoute::AccountToken {
                queue.lock().unwrap().push(NodeSpecimenEvent::SetText {
                    key: "la-account-message".to_string(),
                    value: "Account activation requested".to_string(),
                });
                return;
            }
            let draft = LicenceSubmitDraft {
                route: LicenceActivationRoute::LicenceFile,
                key: String::new(),
                token: None,
                file_contents_base64: file_b64.clone(),
                label: machine_label.clone(),
            };
            match resolve_licence_submit(&draft, None) {
                poodle_headless::licence::LicenceSubmitResolution::Emit {
                    credential, ..
                } => {
                    queue.lock().unwrap().push(NodeSpecimenEvent::SetText {
                        key: "la-emitted".to_string(),
                        value: emitted_caption(&credential),
                    });
                    queue
                        .lock()
                        .unwrap()
                        .push(NodeSpecimenEvent::SetOptionalText {
                            key: "la-file-error".to_string(),
                            value: None,
                        });
                }
                poodle_headless::licence::LicenceSubmitResolution::Reject { message } => {
                    queue.lock().unwrap().push(NodeSpecimenEvent::SetText {
                        key: "la-file-error".to_string(),
                        value: message,
                    });
                }
                poodle_headless::licence::LicenceSubmitResolution::Quiet => {}
            }
        })
    };

    let embedded = LicenceActivation::from_spec(
        LicenceActivationSpec::new()
            .with_mode(LicenceActivationMode::Account)
            .with_activate_label("Activate")
            .with_file_accept(".licence")
            .with_route(route)
            .with_machine_label(Some(machine_label.clone()))
            .with_machine_label_editing(machine_editing)
            .with_file_name(file_name.clone().unwrap_or_default())
            .with_file_contents_base64(file_contents_base64.clone().unwrap_or_default())
            .with_route_message(embedded_route_message.clone()),
        theme,
    )
    .with_account_content(account_content)
    .on_view_change({
        let queue = Arc::clone(&queue);
        Arc::new(move |target: LicenceActivationRoute| {
            queue.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
                key: "la-offline".to_string(),
                value: target == LicenceActivationRoute::LicenceFile,
            });
            // Switching routes invalidates a selected or pending file read:
            // returning offline requires a new file.
            queue
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::FileInvalidate);
            queue
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::SetOptionalText {
                    key: "la-account-message".to_string(),
                    value: None,
                });
        })
    })
    .on_machine_label_edit({
        let (edit, _change, _commit, _cancel) =
            label_handlers(&queue, machine_label.as_str());
        edit
    })
    .on_machine_label_change({
        let (_edit, change, _commit, _cancel) = label_handlers(&queue, machine_label.as_str());
        change
    })
    .on_machine_label_commit({
        let (_edit, _change, commit, _cancel) = label_handlers(&queue, machine_label.as_str());
        commit
    })
    .on_machine_label_cancel({
        let (_edit, _change, _commit, cancel) = label_handlers(&queue, machine_label.as_str());
        cancel
    })
    .on_submit(embedded_submit)
    .on_file_browse({
        let queue = Arc::clone(&queue);
        Arc::new(move || {
            queue.lock().unwrap().push(NodeSpecimenEvent::FileBrowse {
                key: "la-file".to_string(),
                spec: SingleFilePickSpec {
                    prompt: "Choose a licence file".to_string(),
                    accept: Some(".licence".to_string()),
                    max_size: None,
                },
                // A read failure is a local polite error on this component
                // surface — the approved web copy, never the OS text.
                failed_message: Some(
                    poodle_headless::licence::LICENCE_FILE_UNREADABLE_MESSAGE.to_string()
                ),
            });
        })
    })
    .on_file_remove({
        let queue = Arc::clone(&queue);
        Arc::new(move || {
            queue.lock().unwrap().push(NodeSpecimenEvent::SetOptionalText {
                key: "la-file-name".to_string(),
                value: None,
            });
            queue.lock().unwrap().push(NodeSpecimenEvent::SetOptionalText {
                key: "la-file-base64".to_string(),
                value: None,
            });
            queue.lock().unwrap().push(NodeSpecimenEvent::SetOptionalText {
                key: "la-file-error".to_string(),
                value: None,
            });
        })
    });

    // External account activation: default explanation, no host content.
    let external = LicenceActivation::from_spec(
        LicenceActivationSpec::new()
            .with_mode(LicenceActivationMode::Account)
            .with_file_accept(".licence"),
        theme,
    );

    let pending_spec = LicenceActivationSpec::new()
        .with_mode(LicenceActivationMode::Account)
        .with_pending(true);
    let disabled_spec = LicenceActivationSpec::new()
        .with_mode(LicenceActivationMode::Key)
        .with_key_code_input(
            poodle_specs::LicenceKeyCodeInputOptions::new(20)
                .with_groups([5, 5, 5, 5])
                .with_separator("-"),
        )
        .with_disabled(true);

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(420.0))
        .child(group(
            theme,
            "Embedded account activation",
            embedded,
        ))
        .child(group(
            theme,
            "External account activation",
            external,
        ))
        .child(group(
            theme,
            "Key activation",
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(interactive_key)
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(gpui::white())
                        .child(key_caption),
                ),
        ))
        .child(group(
            theme,
            "Pending and disabled",
            div()
                .flex()
                .flex_col()
                .gap(px(16.0))
                .child(LicenceActivation::from_spec(pending_spec, theme))
                .child(LicenceActivation::from_spec(disabled_spec, theme)),
        ))
        .child(group(
            theme,
            "Host copy",
            LicenceActivation::from_spec(
                LicenceActivationSpec::new()
                    .with_mode(LicenceActivationMode::Account)
                    .with_title("Activate Finch")
                    .with_activate_label("Activate Finch")
                    .with_machine_label(Some(String::new())),
                theme,
            ),
        ))
        }
