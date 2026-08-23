//! UpdateStatus — the update mechanism as data in and commands out.
//!
//! Contract: `docs/contracts/components/update-status.md`
//!
//! Display copy resolves once through `poodle_headless::update::update_status_view`.
//! The host owns every authority read; this module never fetches, downloads,
//! or installs. `observe` is web-only — a native host rerenders with fresh
//! props.

use std::sync::Arc;

use poodle_headless::update::{
    update_status_view, UpdateStatusAction, UpdateStatusInput, UpdateStatusNoticeTone,
};
use poodle_node::{CrossAxisAlignment, LayoutDirection, Node, NodeRole};
use poodle_specs::{
    AlertDialogSpec, AlertDialogTone, ButtonSpec, ButtonVariant, ControlSize, ProgressSpec,
    SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant, UpdateStatusSpec,
};

use crate::alert_dialog::{alert_dialog, AlertDialogHandlers};
use crate::button::button;
use crate::context::RenderContext;
use crate::presentation::rem_to_px;
use crate::progress::progress;
use crate::spinner::spinner;

#[derive(Default)]
pub struct UpdateStatusHandlers {
    pub instance_id: Option<String>,
    pub on_check: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_install: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_defer: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_confirm_open_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
}

fn action_id(handlers: &UpdateStatusHandlers, suffix: &str) -> String {
    match &handlers.instance_id {
        Some(scope) => format!("{scope}-{suffix}"),
        None => format!("update-status-{suffix}"),
    }
}

fn dispatch_action(action: UpdateStatusAction, spec: &UpdateStatusSpec, handlers: &UpdateStatusHandlers) {
    match action {
        UpdateStatusAction::Install => {
            if spec.confirm_install {
                if let Some(handler) = &handlers.on_confirm_open_change {
                    handler(true);
                }
            } else if let Some(handler) = &handlers.on_install {
                handler();
            }
        }
        UpdateStatusAction::Check => {
            if let Some(handler) = &handlers.on_check {
                handler();
            }
        }
        UpdateStatusAction::Defer => {
            if let Some(handler) = &handlers.on_defer {
                handler();
            }
        }
    }
}

fn action_label(action: UpdateStatusAction, spec: &UpdateStatusSpec) -> &str {
    match action {
        UpdateStatusAction::Install => spec.install_label.as_str(),
        UpdateStatusAction::Defer => spec.defer_label.as_str(),
        UpdateStatusAction::Check => spec.check_label.as_str(),
    }
}

fn action_suffix(action: UpdateStatusAction) -> &'static str {
    match action {
        UpdateStatusAction::Install => "install",
        UpdateStatusAction::Defer => "defer",
        UpdateStatusAction::Check => "check",
    }
}

pub fn update_status(
    spec: &UpdateStatusSpec,
    ctx: &RenderContext<'_>,
    handlers: UpdateStatusHandlers,
) -> Node {
    let density = ctx.resolve_density(spec.density);
    let base_size = ctx.base_size(spec.size);

    let view = update_status_view(UpdateStatusInput {
        status: spec.status.clone(),
        availability: spec.availability.clone(),
        progress: spec.progress.clone(),
        deferral: spec.deferral.clone(),
        last_rejection: spec.last_rejection,
        ahead_of_channel: spec.ahead_of_channel.clone(),
        channel: spec.channel,
        installed_version: spec.installed_version.clone(),
    });

    let text_primary = ctx.theme().resolve_color("color.text.primary");
    let text_secondary = ctx.theme().resolve_color("color.text.secondary");
    let danger = ctx.theme().resolve_color("color.text.danger");
    let title_color = if matches!(view.tone, poodle_headless::update::UpdateStatusTone::Danger) {
        danger
    } else {
        text_primary
    };

    let mut head = Node::container();
    {
        let s = &mut head.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.inline.sm");
    }
    if view.busy {
        head = head.child(spinner(
            &SpinnerSpec::new()
                .with_variant(SpinnerVariant::Ring)
                .with_size(SpinnerSize::Sm)
                .with_tone(SpinnerTone::Muted),
            ctx,
        ));
    }
    let mut title = Node::text(&view.title);
    title.style.text_size = Some(rem_to_px(1.0));
    title.style.text_weight = Some(600);
    title.style.descriptor.text_color = Some(title_color);
    head = head.child(title);

    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.stack.sm");
    }
    root.roles
        .insert("state".to_owned(), view.state.as_str().to_string());
    root.roles
        .insert("tone".to_owned(), view.tone.as_str().to_string());
    root = root.child(head);

    if let Some(body) = &view.body {
        let mut body_node = Node::text(body);
        body_node.style.text_size = Some(ctx.theme().resolve_space("typography.body.size"));
        body_node.style.descriptor.text_color = Some(text_secondary);
        root = root.child(body_node);
    }

    if let Some(bar) = view.progress {
        let mut progress_spec = ProgressSpec::new()
            .with_size(ControlSize::Sm)
            .with_density(density);
        progress_spec.aria_label = Some("Download progress".to_string());
        if let Some(fraction) = bar.fraction {
            progress_spec = progress_spec.with_value((fraction * 100.0).round());
        } else {
            progress_spec = progress_spec.with_indeterminate(true);
        }
        root = root.child(progress(&progress_spec, ctx));
    }

    if let Some(notice) = &view.notice {
        let mut notice_row = Node::container();
        {
            let s = &mut notice_row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.inline.sm");
        }
        notice_row.a11y.role = Some(NodeRole::Status);
        notice_row.roles.insert(
            "tone".to_owned(),
            match notice.tone {
                UpdateStatusNoticeTone::Neutral => "neutral".to_string(),
                UpdateStatusNoticeTone::Danger => "danger".to_string(),
            },
        );
        let mut message = Node::text(&notice.message);
        message.style.text_size = Some(ctx.theme().resolve_space("typography.body.size"));
        message.style.descriptor.text_color = Some(match notice.tone {
            UpdateStatusNoticeTone::Danger => danger,
            UpdateStatusNoticeTone::Neutral => text_primary,
        });
        notice_row = notice_row.child(message);
        if let Some(retry) = notice.retry {
            let retry_action = retry;
            let retry_spec = ButtonSpec::new()
                .with_variant(ButtonVariant::Ghost)
                .with_size(ControlSize::Xs)
                .with_density(density)
                .with_label(spec.retry_label.clone())
                .with_disabled(spec.pending);
            let handler = {
                let spec = spec.clone();
                let on_check = handlers.on_check.clone();
                let on_install = handlers.on_install.clone();
                let on_defer = handlers.on_defer.clone();
                let on_confirm = handlers.on_confirm_open_change.clone();
                Some(Arc::new(move || {
                    dispatch_action(
                        retry_action,
                        &spec,
                        &UpdateStatusHandlers {
                            on_check: on_check.clone(),
                            on_install: on_install.clone(),
                            on_defer: on_defer.clone(),
                            on_confirm_open_change: on_confirm.clone(),
                            instance_id: None,
                        },
                    );
                }) as Arc<dyn Fn() + Send + Sync>)
            };
            let mut retry_btn = button(&retry_spec, ctx, handler);
            retry_btn.id = Some(action_id(&handlers, "retry"));
            notice_row = notice_row.child(retry_btn);
        }
        root = root.child(notice_row);
    }

    if !view.actions.is_empty() {
        let mut actions = Node::container();
        {
            let s = &mut actions.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.inline.sm");
        }
        for action in view.actions {
            let variant = if matches!(action, UpdateStatusAction::Install) {
                ButtonVariant::Primary
            } else {
                ButtonVariant::Secondary
            };
            let action_spec = ButtonSpec::new()
                .with_variant(variant)
                .with_size(ControlSize::Sm)
                .with_density(density)
                .with_label(action_label(action, spec).to_string())
                .with_disabled(spec.pending);
            let handler = {
                let spec = spec.clone();
                let on_check = handlers.on_check.clone();
                let on_install = handlers.on_install.clone();
                let on_defer = handlers.on_defer.clone();
                let on_confirm = handlers.on_confirm_open_change.clone();
                Some(Arc::new(move || {
                    dispatch_action(
                        action,
                        &spec,
                        &UpdateStatusHandlers {
                            on_check: on_check.clone(),
                            on_install: on_install.clone(),
                            on_defer: on_defer.clone(),
                            on_confirm_open_change: on_confirm.clone(),
                            instance_id: None,
                        },
                    );
                }) as Arc<dyn Fn() + Send + Sync>)
            };
            let mut btn = button(&action_spec, ctx, handler);
            btn.id = Some(action_id(&handlers, action_suffix(action)));
            actions = actions.child(btn);
        }
        root = root.child(actions);
    }

    if spec.confirm_open {
        let confirm_spec = AlertDialogSpec::new("Install and restart?")
            .with_description("The application will close and restart to finish the update.")
            .with_tone(AlertDialogTone::Warning)
            .with_confirm_label(spec.install_label.clone())
            .with_cancel_label("Cancel")
            .with_size(base_size)
            .with_size_role(spec.size_role)
            .with_density(density)
            .with_open(true);
        let on_install = handlers.on_install.clone();
        let on_confirm = handlers.on_confirm_open_change.clone();
        let on_cancel = handlers.on_confirm_open_change.clone();
        let dialog = alert_dialog(
            &confirm_spec,
            ctx,
            false,
            spec.install_label.as_str(),
            AlertDialogHandlers {
                confirm: Some(Arc::new(move || {
                    if let Some(handler) = &on_confirm {
                        handler(false);
                    }
                    if let Some(handler) = &on_install {
                        handler();
                    }
                })),
                cancel: Some(Arc::new(move || {
                    if let Some(handler) = &on_cancel {
                        handler(false);
                    }
                })),
            },
        );
        let mut stack = Node::container();
        stack.style.descriptor.layout.direction = LayoutDirection::Column;
        stack = stack.child(root).child(dialog);
        return stack;
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_headless::update::{
        Channel, DeferralCause, InstallManager, OfferReason, UpdateAvailabilityProjection,
        UpdateControllerStatus, UpdateDeferral, UpdateProgressProjection, UpdateRejectionCode,
    };
    use poodle_node::NodeKind;
    use std::sync::Mutex;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn texts(node: &Node) -> Vec<String> {
        node.texts()
            .into_iter()
            .map(str::to_string)
            .filter(|t| !t.is_empty())
            .collect()
    }

    fn walk<'a>(node: &'a Node, visit: &mut impl FnMut(&'a Node)) {
        visit(node);
        for child in &node.children {
            walk(child, visit);
        }
    }

    fn offer() -> UpdateAvailabilityProjection {
        UpdateAvailabilityProjection::Offer {
            version: "1.4.0".to_string(),
            reason: OfferReason::Staged,
            notes: Some("Bug fixes and improvements.".to_string()),
        }
    }

    fn ready() -> UpdateStatusSpec {
        UpdateStatusSpec::new()
            .with_status(UpdateControllerStatus::Ready)
            .with_availability(offer())
    }

    #[test]
    fn offer_renders_version_notes_and_both_actions() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = update_status(&ready(), &ctx, UpdateStatusHandlers::default());
        let rendered = texts(&node);
        assert!(rendered.iter().any(|t| t == "Version 1.4.0 is available"));
        assert!(rendered.iter().any(|t| t == "Bug fixes and improvements."));
        assert!(rendered.iter().any(|t| t == "Install and restart"));
        assert!(rendered.iter().any(|t| t == "Later"));
        assert_eq!(node.roles.get("state").map(String::as_str), Some("offer"));
    }

    #[test]
    fn up_to_date_has_no_actions() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = update_status(
            &UpdateStatusSpec::new()
                .with_status(UpdateControllerStatus::Ready)
                .with_availability(UpdateAvailabilityProjection::UpToDate)
                .with_installed_version("1.3.0")
                .with_channel(Channel::Production),
            &ctx,
            UpdateStatusHandlers::default(),
        );
        let rendered = texts(&node);
        assert!(rendered.iter().any(|t| t == "You're up to date"));
        assert!(rendered
            .iter()
            .any(|t| t == "Version 1.3.0 · production channel"));
        assert!(!rendered.iter().any(|t| t == "Check for updates"));
    }

    #[test]
    fn ahead_of_channel_is_not_up_to_date() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = update_status(
            &UpdateStatusSpec::new()
                .with_status(UpdateControllerStatus::Ready)
                .with_availability(UpdateAvailabilityProjection::AheadOfChannel {
                    installed: "1.3.0-nightly.4".to_string(),
                    channel: "1.2.9".to_string(),
                }),
            &ctx,
            UpdateStatusHandlers::default(),
        );
        let rendered = texts(&node);
        assert!(rendered.iter().any(|t| t == "You're ahead of your channel"));
        assert!(rendered
            .iter()
            .any(|t| t == "Installed 1.3.0-nightly.4 · channel 1.2.9"));
        assert!(!rendered.iter().any(|t| t == "You're up to date"));
    }

    #[test]
    fn managed_elsewhere_is_information() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = update_status(
            &UpdateStatusSpec::new()
                .with_status(UpdateControllerStatus::Ready)
                .with_availability(UpdateAvailabilityProjection::ManagedElsewhere {
                    version: "1.4.0".to_string(),
                    manager: InstallManager::HomebrewCask,
                }),
            &ctx,
            UpdateStatusHandlers::default(),
        );
        let rendered = texts(&node);
        assert!(rendered.iter().any(|t| t == "Managed by Homebrew."));
        assert_eq!(node.roles.get("tone").map(String::as_str), Some("info"));
    }

    #[test]
    fn deferral_notice_is_status_not_an_alert() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = update_status(
            &ready().with_deferral(UpdateDeferral {
                version: "1.4.0".to_string(),
                cause: DeferralCause::WorkInFlight {
                    detail: "A transfer is running.".to_string(),
                },
            }),
            &ctx,
            UpdateStatusHandlers::default(),
        );
        let mut found = false;
        walk(&node, &mut |n| {
            if n.a11y.role == Some(NodeRole::Status)
                && n.roles.get("tone").map(String::as_str) == Some("neutral")
            {
                found = true;
            }
        });
        assert!(found, "deferral is a polite status, not an error");
        assert!(texts(&node)
            .iter()
            .any(|t| t == "Install is on hold: A transfer is running."));
    }

    #[test]
    fn null_fraction_is_indeterminate_and_zero_is_a_zero_bar() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let null_node = update_status(
            &UpdateStatusSpec::new()
                .with_status(UpdateControllerStatus::Ready)
                .with_progress(UpdateProgressProjection::Downloading { fraction: None }),
            &ctx,
            UpdateStatusHandlers::default(),
        );
        let mut null_progress = None;
        walk(&null_node, &mut |n| {
            if n.a11y.role == Some(NodeRole::ProgressIndicator) {
                null_progress = Some(n.kind.clone());
            }
        });
        assert!(
            !matches!(null_progress, Some(NodeKind::Progress { .. })),
            "null fraction must not become a Progress widget (that would be a zero bar)"
        );

        let zero_node = update_status(
            &UpdateStatusSpec::new()
                .with_status(UpdateControllerStatus::Ready)
                .with_progress(UpdateProgressProjection::Downloading {
                    fraction: Some(0.0),
                }),
            &ctx,
            UpdateStatusHandlers::default(),
        );
        let mut zero_is_zero = false;
        walk(&zero_node, &mut |n| {
            if n.a11y.role == Some(NodeRole::ProgressIndicator) {
                zero_is_zero = matches!(n.kind, NodeKind::Progress { fraction } if fraction == 0.0);
            }
        });
        assert!(zero_is_zero, "fraction 0 must render a zero bar");
    }

    #[test]
    fn signature_rejection_offers_no_retry() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = update_status(
            &UpdateStatusSpec::new()
                .with_status(UpdateControllerStatus::Ready)
                .with_last_rejection(UpdateRejectionCode::SignatureRejected),
            &ctx,
            UpdateStatusHandlers::default(),
        );
        let rendered = texts(&node);
        assert!(rendered.iter().any(|t| t.contains("signature check")));
        assert!(!rendered.iter().any(|t| t == "Try again"));
    }

    #[test]
    fn pending_disables_actions() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = update_status(
            &ready().with_pending(true),
            &ctx,
            UpdateStatusHandlers::default(),
        );
        let mut disabled = 0;
        walk(&node, &mut |n| {
            if n.interaction.disabled {
                disabled += 1;
            }
        });
        assert!(disabled >= 2, "install and defer are disabled while pending");
    }

    #[test]
    fn confirm_install_opens_the_dialog_instead_of_emitting() {
        let installs = Arc::new(Mutex::new(0usize));
        let confirms = Arc::new(Mutex::new(Vec::new()));
        let install_sink = Arc::clone(&installs);
        let confirm_sink = Arc::clone(&confirms);
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = update_status(
            &ready(),
            &ctx,
            UpdateStatusHandlers {
                on_install: Some(Arc::new(move || *install_sink.lock().unwrap() += 1)),
                on_confirm_open_change: Some(Arc::new(move |open| {
                    confirm_sink.lock().unwrap().push(open)
                })),
                ..UpdateStatusHandlers::default()
            },
        );
        let mut install_btn = None;
        walk(&node, &mut |n| {
            if n.id.as_deref() == Some("update-status-install") {
                install_btn = n.interaction.on_activate.clone();
            }
        });
        install_btn.expect("install action")();
        assert_eq!(*installs.lock().unwrap(), 0);
        assert_eq!(*confirms.lock().unwrap(), vec![true]);

        let direct_sink = Arc::clone(&installs);
        let direct = update_status(
            &ready().with_confirm_install(false),
            &ctx,
            UpdateStatusHandlers {
                on_install: Some(Arc::new(move || *direct_sink.lock().unwrap() += 1)),
                ..UpdateStatusHandlers::default()
            },
        );
        let mut direct_btn = None;
        walk(&direct, &mut |n| {
            if n.id.as_deref() == Some("update-status-install") {
                direct_btn = n.interaction.on_activate.clone();
            }
        });
        direct_btn.expect("direct install")();
        assert_eq!(*installs.lock().unwrap(), 1);
    }

    #[test]
    fn confirmation_inherits_authored_presentation() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = update_status(
            &ready()
                .with_confirm_open(true)
                .with_size(ControlSize::Md)
                .with_size_role(poodle_specs::SemanticControlSizeRole::Prominent)
                .with_density(poodle_specs::ControlDensity::Compact),
            &ctx,
            UpdateStatusHandlers::default(),
        );
        let confirm = node
            .find(&|child| {
                matches!(&child.kind, NodeKind::Button { label } if label == "Install and restart")
                    && child.id.as_deref() != Some("update-status-install")
            })
            .expect("confirmation button");
        assert_eq!(confirm.roles.get("size").map(String::as_str), Some("lg"));
        assert_eq!(
            confirm.roles.get("density").map(String::as_str),
            Some("compact")
        );
    }
}
