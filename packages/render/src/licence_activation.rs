//! LicenceActivation — one host-selected activation model: licence-key entry,
//! or account activation with licence-file fallback.
//!
//! Contract: `docs/contracts/components/licence-activation.md`
//!
//! Poodle renders; the host supplies policy and async behaviour. The pure
//! submit decision lives in `poodle_headless::licence::resolve_licence_submit`
//! and the host executes it through [`LicenceActivationHandlers::on_submit`]
//! — a rejection updates the spec's local message fields, an accepted
//! credential is emitted exactly. No OS-dialog or file logic lives here:
//! the offline view composes the generic FileUpload browse seam, and account
//! mode composes optional host-owned account content beside the spec.

use std::sync::Arc;

use poodle_headless::licence::{LicenceActivationRoute, LicenceKeyResult};
use poodle_node::{
    CrossAxisAlignment, LayoutDirection, MainAxisAlignment, Node, NodeRole, TextChangeHandler,
};
use poodle_specs::{
    ButtonSpec, ButtonVariant, CodeInputCompletion, CodeInputSpec, EditableLabelActivation,
    EditableLabelSpec, EditableLabelVariant, FieldSpec, FileUploadItem, FileUploadSpec,
    LicenceActivationSpec, LicenceKeyCodeInputOptions, TextInputSpec, ValidationState,
};

use crate::button::button;
use crate::code_input::{code_input_with_handlers, CodeInputHandlers};
use crate::context::RenderContext;
use crate::editable_label::{editable_label_with_handlers, EditableLabelHandlers};
use crate::field::field;
use crate::file_upload::{file_upload_with_handlers, FileUploadHandlers};
use crate::presentation::rem_to_px;
use crate::text_input::text_input_with_handlers;

/// Host callbacks. Everything that decides or acts on a credential stays in
/// the handler set; the spec is cloneable data only.
#[derive(Default)]
pub struct LicenceActivationHandlers {
    /// The key draft changed (also clears the local key message — host rule).
    pub on_key_change: Option<TextChangeHandler>,
    /// The segmented key's caret moved (Rust targets own the caret).
    pub on_key_selection_change: Option<Arc<dyn Fn(usize, usize) + Send + Sync>>,
    /// The machine-name draft changed while editing (controlled edit).
    pub on_machine_label_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// A machine-name edit was committed (Enter/blur). The host closes the
    /// edit state and emits the trimmed label.
    pub on_machine_label_commit: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// A machine-name edit was cancelled (Escape). The host closes the edit
    /// state without committing.
    pub on_machine_label_cancel: Option<Arc<dyn Fn() + Send + Sync>>,
    /// A machine-name edit was started.
    pub on_machine_label_edit: Option<Arc<dyn Fn() + Send + Sync>>,
    /// Caret into the machine-name draft moved.
    pub on_machine_label_selection_change: Option<Arc<dyn Fn(usize, usize) + Send + Sync>>,
    /// The account/offline route switch was pressed.
    pub on_view_change: Option<Arc<dyn Fn(LicenceActivationRoute) + Send + Sync>>,
    /// The form's submit button was pressed. The host runs the shared submit
    /// resolution, updates spec messages, and emits the exact credential.
    pub on_submit: Option<Arc<dyn Fn() + Send + Sync>>,
    /// The offline FileUpload requested one file (generic browse seam).
    pub on_file_browse: Option<Arc<dyn Fn() + Send + Sync>>,
    /// The selected licence file was removed.
    pub on_file_remove: Option<Arc<dyn Fn() + Send + Sync>>,
    /// The injected key parser for the segmented entry's presentation
    /// feedback (tick/cross at full length; never an activation).
    pub on_key_check: Option<Arc<dyn Fn(&str) -> LicenceKeyResult + Send + Sync>>,
}

pub fn licence_activation(
    spec: &LicenceActivationSpec,
    ctx: &RenderContext<'_>,
    handlers: LicenceActivationHandlers,
) -> Node {
    licence_activation_with_slots(spec, ctx, None, handlers)
}

/// Render with optional host-owned account content beside the spec.
pub fn licence_activation_with_slots(
    spec: &LicenceActivationSpec,
    ctx: &RenderContext<'_>,
    account_content: Option<Node>,
    handlers: LicenceActivationHandlers,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let base_size = ctx.base_size(spec.size);
    let density = ctx.resolve_density(spec.density);
    let frozen = spec.interaction_frozen();

    let text_primary = ctx.theme().resolve_color("color.text.primary");
    let text_secondary = ctx.theme().resolve_color("color.text.secondary");
    let danger = ctx.theme().resolve_color("color.status.danger");

    let route = spec.effective_route();

    // ── Header: title + route switch ──
    let mut header = Node::container();
    {
        let s = &mut header.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    }
    let mut title = Node::text(&spec.title);
    title.style.text_size = Some(rem_to_px(1.0));
    title.style.text_weight = Some(600);
    title.style.descriptor.text_color = Some(text_primary);
    let mut header = header.child(title);
    if spec.shows_route_switch() {
        let offline = route == LicenceActivationRoute::AccountToken;
        let switch_spec = ButtonSpec::new()
            .with_variant(ButtonVariant::Ghost)
            .with_leading_icon(if offline { "cloud-off" } else { "user" })
            .with_label(if offline {
                "Activate offline"
            } else {
                "Use account activation"
            })
            .with_disabled(frozen)
            .with_size(base_size)
            .with_density(density);
        let switch = button(&switch_spec, ctx, {
            let on_view_change = handlers.on_view_change.clone();
            let target = if offline {
                LicenceActivationRoute::LicenceFile
            } else {
                LicenceActivationRoute::AccountToken
            };
            Some(Arc::new(move || {
                if let Some(handler) = &on_view_change {
                    handler(target);
                }
            }))
        });
        header = header.child(switch);
    }

    // ── Route view ──
    let view = match route {
        LicenceActivationRoute::Key => match &spec.key_code_input {
            Some(options) => key_code_input_view(spec, options, ctx, &handlers, frozen),
            None => free_form_key_view(spec, ctx, &handlers, frozen),
        },
        LicenceActivationRoute::AccountToken => {
            let mut view = Node::container();
            {
                let s = &mut view.style;
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.stack.sm");
            }
            match account_content {
                Some(content) => {
                    let mut slot = Node::container();
                    slot.style.descriptor.layout.direction = LayoutDirection::Column;
                    view = view.child(slot.child(content));
                }
                None => {
                    let mut explanation =
                        Node::text("Continue with your account to authorise this machine.");
                    explanation.style.descriptor.text_color = Some(text_secondary);
                    view = view.child(explanation);
                }
            }
            view
        }
        LicenceActivationRoute::LicenceFile => {
            let mut view = Node::container();
            {
                let s = &mut view.style;
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.stack.sm");
            }
            let mut upload = file_upload_with_handlers(
                &FileUploadSpec::new()
                    .with_accept(spec.file_accept.clone().unwrap_or_default())
                    .with_multiple(false)
                    .with_show_preview(false)
                    .with_disabled(frozen)
                    .with_size(base_size)
                    .with_density(density),
                ctx,
                FileUploadHandlers {
                    on_browse: handlers.on_file_browse.clone(),
                    on_remove: handlers.on_file_remove.as_ref().map(|handler| {
                        let handler = Arc::clone(handler);
                        Arc::new(move |_name: &str| handler()) as Arc<dyn Fn(&str) + Send + Sync>
                    }),
                },
            );
            if let (Some(name), Some(contents)) = (&spec.file_name, &spec.file_contents_base64) {
                let size = (contents.len() as u64) * 3 / 4;
                let item = FileUploadItem::new("licence-file", name.clone(), size)
                    .with_status(poodle_specs::FileUploadStatus::Complete);
                upload = crate::file_upload::file_upload_with_handlers(
                    &FileUploadSpec::new()
                        .with_accept(spec.file_accept.clone().unwrap_or_default())
                        .with_multiple(false)
                        .with_show_preview(false)
                        .with_disabled(frozen)
                        .with_file(item)
                        .with_size(base_size)
                        .with_density(density),
                    ctx,
                    FileUploadHandlers {
                        on_browse: handlers.on_file_browse.clone(),
                        on_remove: handlers.on_file_remove.as_ref().map(|handler| {
                            let handler = Arc::clone(handler);
                            Arc::new(move |_name: &str| handler())
                                as Arc<dyn Fn(&str) + Send + Sync>
                        }),
                    },
                );
            }
            view = view.child(upload);
            view
        }
    };

    // ── Route message (non-key routes) ──
    let with_message = match route {
        LicenceActivationRoute::Key => view,
        _ => {
            if let Some(message) = &spec.route_message {
                let mut status = Node::text(message);
                status.style.text_size = Some(ctx.theme().resolve_space("typography.label.size"));
                status.style.descriptor.text_color = Some(danger);
                status.a11y.role = Some(NodeRole::Status);
                view.child(status)
            } else {
                view
            }
        }
    };

    // ── Actions row: machine name (left) + submit (right) ──
    let mut actions = Node::container();
    {
        let s = &mut actions.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::End;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.inline.md");
    }
    if spec.machine_label.is_some() {
        actions = actions.child(machine_name(spec, ctx, &handlers, frozen));
    }
    let submit_spec = ButtonSpec::new()
        .with_label(spec.submit_label())
        .with_loading(spec.pending)
        .with_disabled(frozen)
        .with_size(base_size)
        .with_density(density);
    let submit = button(&submit_spec, ctx, handlers.on_submit.clone());
    let actions = actions.child(submit);

    // ── Root form ──
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.stack.md");
    }
    root = root.child(header).child(with_message).child(actions);

    // ── Data state (the native data-* counterpart) ──
    root.roles.insert(
        "mode".to_owned(),
        format!("{:?}", spec.mode).to_ascii_lowercase(),
    );
    root.roles.insert(
        "route".to_owned(),
        format!("{:?}", route).to_ascii_lowercase(),
    );
    root.roles
        .insert("busy".to_owned(), spec.pending.to_string());
    root.roles.insert(
        "size".to_owned(),
        format!("{effective_size:?}").to_ascii_lowercase(),
    );
    root.roles.insert(
        "density".to_owned(),
        format!("{density:?}").to_ascii_lowercase(),
    );
    root
}

/// The machine-name EditableLabel, only when naming is opted in.
fn machine_name(
    spec: &LicenceActivationSpec,
    ctx: &RenderContext<'_>,
    handlers: &LicenceActivationHandlers,
    frozen: bool,
) -> Node {
    let mut boxed = Node::container();
    {
        let s = &mut boxed.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.stack.xs");
    }
    let mut caption = Node::text("Machine name");
    caption.style.text_size = Some(ctx.theme().resolve_space("typography.label.size"));
    caption.style.descriptor.text_color = Some(ctx.theme().resolve_color("color.text.secondary"));
    let label = editable_label_with_handlers(
        &EditableLabelSpec::new()
            .with_value(spec.machine_label.clone().unwrap_or_default())
            .with_draft_value(spec.machine_label_draft.clone())
            .with_selection(
                spec.machine_label_selection.0,
                spec.machine_label_selection.1,
            )
            .with_editing(spec.machine_label_editing)
            .with_activation_mode(EditableLabelActivation::EnterOrSpace)
            .with_variant(EditableLabelVariant::Default)
            .with_empty_text("unnamed machine")
            .with_placeholder("unnamed machine")
            .with_show_edit_icon(true)
            .with_aria_label("Edit machine name")
            .with_disabled(frozen)
            .with_size(ctx.base_size(spec.size))
            .with_density(ctx.resolve_density(spec.density)),
        ctx,
        EditableLabelHandlers {
            on_edit_start: handlers.on_machine_label_edit.clone(),
            on_change: handlers.on_machine_label_change.clone(),
            on_selection_change: handlers.on_machine_label_selection_change.clone(),
            on_commit: crate::editable_label::adapt_commit(
                handlers.on_machine_label_commit.clone(),
            ),
            on_cancel: handlers.on_machine_label_cancel.clone(),
            ..EditableLabelHandlers::default()
        },
    );
    boxed.child(caption).child(label)
}

/// Segmented licence-key entry (opt-in `keyCodeInput`).
fn key_code_input_view(
    spec: &LicenceActivationSpec,
    options: &LicenceKeyCodeInputOptions,
    ctx: &RenderContext<'_>,
    handlers: &LicenceActivationHandlers,
    frozen: bool,
) -> Node {
    // Presentation feedback at full length: the injected parser decides the
    // tick/cross. Never an activation, and the result belongs to the exact
    // value shown — CodeInput drops it the moment the value edits away.
    let completion = match &handlers.on_key_check {
        Some(check) if spec.key_draft.chars().count() == options.length => {
            match check(&spec.key_draft) {
                LicenceKeyResult::Ok { .. } => {
                    Some(CodeInputCompletion::Passed(spec.key_draft.clone()))
                }
                LicenceKeyResult::Err(_) => {
                    Some(CodeInputCompletion::Failed(spec.key_draft.clone()))
                }
            }
        }
        _ => None,
    };

    let mut code_spec = CodeInputSpec::new()
        .with_length(options.length)
        .with_numbers_only(false)
        .with_autocomplete("off")
        .with_label("Licence key")
        .with_value(spec.key_draft.clone())
        .with_selection(spec.key_selection.0, spec.key_selection.1)
        .with_disabled(frozen)
        .with_size(ctx.base_size(spec.size))
        .with_density(ctx.resolve_density(spec.density));
    if let Some(groups) = &options.groups {
        code_spec = code_spec.with_groups(groups.iter().copied());
    }
    if let Some(separator) = &options.separator {
        code_spec = code_spec.with_separator(separator.clone());
    }
    if let Some(message) = &spec.key_message {
        code_spec = code_spec.with_error(message.clone());
    }
    if let Some(completion) = completion {
        code_spec = code_spec.with_completion_result(completion);
    }

    code_input_with_handlers(
        &code_spec,
        ctx,
        CodeInputHandlers {
            on_value_change: handlers.on_key_change.clone(),
            on_selection_change: handlers.on_key_selection_change.clone(),
            ..CodeInputHandlers::default()
        },
    )
}

/// Free-form licence-key entry (default when `keyCodeInput` is omitted).
fn free_form_key_view(
    spec: &LicenceActivationSpec,
    ctx: &RenderContext<'_>,
    handlers: &LicenceActivationHandlers,
    frozen: bool,
) -> Node {
    let invalid = spec.key_message.is_some();
    let mut field_spec = FieldSpec::new("licence-key", "Licence key")
        .with_validation_state(if invalid {
            ValidationState::Invalid
        } else {
            ValidationState::None
        })
        .with_size(ctx.base_size(spec.size))
        .with_density(ctx.resolve_density(spec.density));
    if let Some(message) = &spec.key_message {
        field_spec = field_spec.with_error(message.clone());
    }

    let text_spec = TextInputSpec {
        value: Some(spec.key_draft.clone()),
        is_disabled: frozen,
        validation_state: if invalid {
            ValidationState::Invalid
        } else {
            ValidationState::None
        },
        size: Some(ctx.base_size(spec.size)),
        density: Some(ctx.resolve_density(spec.density)),
        ..TextInputSpec::default()
    };
    // The control builds inside the field's presentation scope (Field wraps
    // its control slot in a provider on the web), so it arrives as a builder.
    let control: crate::context::SlotBuilder = Box::new(move |scoped| {
        text_input_with_handlers(
            &text_spec,
            scoped,
            crate::text_input::TextInputHandlers {
                on_change: handlers.on_key_change.clone(),
                on_selection_change: handlers.on_key_selection_change.clone(),
                ..crate::text_input::TextInputHandlers::default()
            },
        )
    });
    field(&field_spec, ctx, Some(control))
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_headless::licence::{LicenceActivationMode, LicenceKeyProblem};

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    /// Key mode requires only the key path: no account or offline route
    /// renders, and the route switch is absent.
    #[test]
    fn key_mode_renders_no_account_or_offline_route() {
        let spec = LicenceActivationSpec::new()
            .with_mode(LicenceActivationMode::Key)
            .with_key_draft("abc");
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = licence_activation(&spec, &ctx, LicenceActivationHandlers::default());
        assert!(node.has_text("Licence key"));
        assert!(!node.has_text("Activate offline"));
        assert!(!node.has_text("Use account activation"));
        assert_eq!(node.roles.get("route").map(String::as_str), Some("key"));
    }

    /// Account mode opens on account activation, shows the switch, and has
    /// no key route.
    #[test]
    fn account_mode_opens_on_account_and_switches_offline() {
        let spec = LicenceActivationSpec::new();
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = licence_activation(&spec, &ctx, LicenceActivationHandlers::default());
        assert!(node.has_text("Activate offline"));
        assert!(node.has_text("Continue with your account to authorise this machine."));
        assert_eq!(
            node.roles.get("route").map(String::as_str),
            Some("accounttoken")
        );

        let offline = LicenceActivationSpec::new()
            .with_route(LicenceActivationRoute::LicenceFile)
            .with_file_accept(".lic");
        let node = licence_activation(&offline, &ctx, LicenceActivationHandlers::default());
        assert!(node.has_text("Use account activation"));
        assert_eq!(
            node.roles.get("route").map(String::as_str),
            Some("licencefile")
        );
    }

    /// The opt-in machine name renders only when supplied; a blank commit
    /// emits `unnamed machine` copy but never the copy as the label.
    #[test]
    fn machine_naming_is_opt_in_and_blank_emits_null() {
        let spec = LicenceActivationSpec::new().with_mode(LicenceActivationMode::Key);
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = licence_activation(&spec, &ctx, LicenceActivationHandlers::default());
        assert!(!node.has_text("Machine name"));
        assert_eq!(spec.committed_label(), None);

        let spec = LicenceActivationSpec::new()
            .with_mode(LicenceActivationMode::Key)
            .with_machine_label(Some(String::new()));
        let node = licence_activation(&spec, &ctx, LicenceActivationHandlers::default());
        assert!(node.has_text("Machine name"));
        assert!(node.has_text("unnamed machine"));
        assert_eq!(spec.committed_label(), None);
    }

    /// The segmented key entry composes CodeInput with the licence specifics:
    /// alphanumeric, explicit groups/separator, and a completion result that
    /// resolves through the injected parser at full length.
    #[test]
    fn segmented_key_entry_composes_grouped_code_input() {
        let spec = LicenceActivationSpec::new()
            .with_mode(LicenceActivationMode::Key)
            .with_key_code_input(
                LicenceKeyCodeInputOptions::new(20)
                    .with_groups([5, 5, 5, 5])
                    .with_separator("-"),
            )
            .with_key_draft("ABCDEFGHIJKLMNOPQRST");
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = licence_activation(
            &spec,
            &ctx,
            LicenceActivationHandlers {
                on_key_check: Some(Arc::new(|key: &str| {
                    if key == "ABCDEFGHIJKLMNOPQRST" {
                        LicenceKeyResult::Ok {
                            key: key.to_string(),
                            grouped: "ABCDE-FGHIJ-KLMNO-PQRST".to_string(),
                        }
                    } else {
                        LicenceKeyResult::Err(LicenceKeyProblem::CheckFailed)
                    }
                })),
                ..LicenceActivationHandlers::default()
            },
        );
        // The accepted full key shows its tick.
        assert!(node
            .find(&|n| { n.a11y.label.as_deref() == Some("Code check passed") })
            .is_some());

        // A different full value resolves through the same parser and shows
        // the danger cross — the indicator always belongs to the value the
        // check ran against.
        let edited = spec.clone().with_key_draft("ABCDEFGHIJKLMNOPQRSU");
        let node = licence_activation(
            &edited,
            &ctx,
            LicenceActivationHandlers {
                on_key_check: Some(Arc::new(|_key: &str| {
                    LicenceKeyResult::Err(LicenceKeyProblem::CheckFailed)
                })),
                ..LicenceActivationHandlers::default()
            },
        );
        assert!(node
            .find(&|n| n.a11y.label.as_deref() == Some("Code check failed"))
            .is_some());
    }

    /// The offline view composes the generic FileUpload browse seam and its
    /// remove handler; the submit button is blocked while pending/disabled.
    #[test]
    fn offline_view_browses_through_the_generic_seam() {
        let browsed = Arc::new(std::sync::Mutex::new(0usize));
        let sink = Arc::clone(&browsed);
        let spec = LicenceActivationSpec::new()
            .with_route(LicenceActivationRoute::LicenceFile)
            .with_file_accept(".lic");
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = licence_activation(
            &spec,
            &ctx,
            LicenceActivationHandlers {
                on_file_browse: Some(Arc::new(move || {
                    *sink.lock().unwrap() += 1;
                })),
                ..LicenceActivationHandlers::default()
            },
        );
        // The dropzone is the container that browses (the header switch is a
        // button; the submit button is only wired when a handler is given).
        let browse = node
            .find(&|n| {
                n.interaction.on_activate.is_some()
                    && matches!(n.kind, poodle_node::NodeKind::Container)
            })
            .expect("the dropzone browses");
        (browse.interaction.on_activate.as_ref().unwrap())();
        assert_eq!(*browsed.lock().unwrap(), 1);
    }

    #[test]
    fn submit_is_blocked_while_pending_or_disabled() {
        let spec = LicenceActivationSpec::new()
            .with_mode(LicenceActivationMode::Key)
            .with_pending(true);
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = licence_activation(
            &spec,
            &ctx,
            LicenceActivationHandlers {
                on_submit: Some(Arc::new(|| {})),
                ..LicenceActivationHandlers::default()
            },
        );
        assert_eq!(node.roles.get("busy").map(String::as_str), Some("true"));
        // A pending submit is loading: the button renders but carries no
        // activation (the button contract freezes a loading control).
        let submit = node
            .find(&|n| matches!(&n.kind, poodle_node::NodeKind::Button { .. }))
            .expect("the submit button renders");
        assert!(
            submit.interaction.on_activate.is_none(),
            "loading freezes the submit action"
        );
        assert!(submit.interaction.disabled);
    }

    /// The submit button is the defining action in every route: account
    /// mode fires the host-owned acquisition request, and the offline route
    /// fires the submit that the host resolves into the file credential.
    #[test]
    fn submit_fires_in_account_and_offline_routes() {
        let account_submits = Arc::new(std::sync::Mutex::new(0usize));
        let sink = Arc::clone(&account_submits);
        let account = LicenceActivationSpec::new()
            .with_mode(LicenceActivationMode::Account)
            .with_machine_label(Some("Studio Mac".to_string()));
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = licence_activation_with_slots(
            &account,
            &ctx,
            Some(Node::text("host login form")),
            LicenceActivationHandlers {
                on_submit: Some(Arc::new(move || {
                    *sink.lock().unwrap() += 1;
                })),
                ..LicenceActivationHandlers::default()
            },
        );
        let submit = node
            .find(&|n| {
                matches!(&n.kind, poodle_node::NodeKind::Button { label } if label == "Continue with account")
            })
            .expect("the account submit button");
        (submit.interaction.on_activate.as_ref().unwrap())();
        assert_eq!(*account_submits.lock().unwrap(), 1, "account submit fires");

        let offline_submits = Arc::new(std::sync::Mutex::new(0usize));
        let sink = Arc::clone(&offline_submits);
        let offline = LicenceActivationSpec::new()
            .with_mode(LicenceActivationMode::Account)
            .with_route(LicenceActivationRoute::LicenceFile)
            .with_file("machine.lic", "c3R1ZmY=");
        let node = licence_activation(
            &offline,
            &ctx,
            LicenceActivationHandlers {
                on_submit: Some(Arc::new(move || {
                    *sink.lock().unwrap() += 1;
                })),
                ..LicenceActivationHandlers::default()
            },
        );
        let submit = node
            .find(&|n| {
                matches!(&n.kind, poodle_node::NodeKind::Button { label } if label == "Activate")
            })
            .expect("the offline submit button");
        (submit.interaction.on_activate.as_ref().unwrap())();
        assert_eq!(*offline_submits.lock().unwrap(), 1, "offline submit fires");
    }

    /// Machine-name typing updates the controlled draft; commit and cancel
    /// are distinct so the host can close the edit state only on commit or
    /// escape, never on a keystroke.
    #[test]
    fn machine_label_change_commit_and_cancel_are_distinct() {
        let events = Arc::new(std::sync::Mutex::new(Vec::new()));
        let spec = LicenceActivationSpec::new()
            .with_mode(LicenceActivationMode::Key)
            .with_machine_label(Some("rig".to_string()))
            .with_machine_label_editing(true);
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = licence_activation(
            &spec,
            &ctx,
            LicenceActivationHandlers {
                on_machine_label_change: Some({
                    let sink = Arc::clone(&events);
                    Arc::new(move |value: &str| {
                        sink.lock().unwrap().push(format!("change:{value}"))
                    })
                }),
                on_machine_label_commit: Some({
                    let sink = Arc::clone(&events);
                    Arc::new(move |value: &str| {
                        sink.lock().unwrap().push(format!("commit:{value}"))
                    })
                }),
                on_machine_label_cancel: Some({
                    let sink = Arc::clone(&events);
                    Arc::new(move || sink.lock().unwrap().push("cancel".to_string()))
                }),
                ..LicenceActivationHandlers::default()
            },
        );
        // The editing input carries the three channels separately.
        let input = node
            .find(&|n| n.interaction.on_text_change.is_some())
            .expect("the editing input");
        (input.interaction.on_text_change.as_ref().unwrap())("rig-2");
        assert_eq!(
            events.lock().unwrap().as_slice(),
            ["change:rig-2"],
            "typing is a change"
        );
        // The host re-renders with the committed draft, then commit fires
        // with that value — distinct from the change channel.
        let node = licence_activation(
            &spec
                .clone()
                .with_machine_label(Some("rig".to_string()))
                .with_machine_label_draft(Some("rig-2".to_string())),
            &ctx,
            LicenceActivationHandlers {
                on_machine_label_change: Some({
                    let sink = Arc::clone(&events);
                    Arc::new(move |value: &str| {
                        sink.lock().unwrap().push(format!("change:{value}"))
                    })
                }),
                on_machine_label_commit: Some({
                    let sink = Arc::clone(&events);
                    Arc::new(move |value: &str| {
                        sink.lock().unwrap().push(format!("commit:{value}"))
                    })
                }),
                ..LicenceActivationHandlers::default()
            },
        );
        let input = node
            .find(&|n| n.interaction.on_text_change.is_some())
            .expect("the editing input");
        (input
            .interaction
            .on_submit
            .as_ref()
            .expect("commit via submit"))();
        assert_eq!(
            events.lock().unwrap().as_slice(),
            ["change:rig-2", "commit:rig-2"],
            "typing and commit are distinct, in order"
        );

        // Cancel is its own channel and closes the edit without committing.
        let events = Arc::new(std::sync::Mutex::new(Vec::new()));
        let sink = Arc::clone(&events);
        let node = licence_activation(
            &spec,
            &ctx,
            LicenceActivationHandlers {
                on_machine_label_cancel: Some(Arc::new(move || {
                    sink.lock().unwrap().push("cancel".to_string())
                })),
                ..LicenceActivationHandlers::default()
            },
        );
        let input = node
            .find(&|n| n.interaction.on_cancel.is_some())
            .expect("the editing input");
        (input.interaction.on_cancel.as_ref().unwrap())();
        assert_eq!(events.lock().unwrap().as_slice(), ["cancel"]);
    }
}
