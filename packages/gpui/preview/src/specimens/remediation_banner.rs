use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, RemediationBanner};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ButtonVariant, EyebrowSpec, RemediationAction, RemediationBannerSpec, StatusTone, ToneFill,
};
use std::sync::Arc;

fn group(label: &str, theme: &GpuiThemeProvider, child: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(child)
}

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let dismissed = state.specimens.is_on("remediation-banner-dismissed");
    let last_action = state
        .specimens
        .text
        .get("remediation-banner-last-action")
        .cloned();

    let recovery = if dismissed {
        div().child("Dismissed.").into_any_element()
    } else {
        let action_events = state.node_events.clone();
        let dismiss_events = state.node_events.clone();
        let banner = RemediationBanner::from_spec(
            RemediationBannerSpec::new(
                "We could not save your changes",
                "Your edits are still local. Retry the save or inspect the error details.",
            )
            .with_tone(StatusTone::Danger)
            .with_primary_action(
                RemediationAction::new("retry", "Try again").with_variant(ButtonVariant::Primary),
            )
            .with_secondary_action(RemediationAction::new("details", "View details"))
            .with_dismissible(true),
            theme,
        )
        .on_action(Arc::new(move |id: &str| {
            action_events
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::SetText {
                    key: "remediation-banner-last-action".to_string(),
                    value: id.to_string(),
                });
        }))
        .on_dismiss(Arc::new(move || {
            dismiss_events
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::Toggle(
                    "remediation-banner-dismissed".to_string(),
                ));
        }))
        .with_instance_id("recovery");
        let mut stack = div().flex().flex_col().gap(px(8.0)).child(banner);
        if let Some(action) = last_action {
            stack = stack.child(div().child(format!("Last request: {action}")));
        }
        stack.into_any_element()
    };

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group("Recovery actions", theme, recovery))
        .child(group(
            "Recovery in progress",
            theme,
            RemediationBanner::from_spec(
                RemediationBannerSpec::new("Reconnecting", "This should only take a moment.")
                    .with_tone(StatusTone::Pending),
                theme,
            ),
        ))
        .child(group(
            "Solid fills",
            theme,
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(RemediationBanner::from_spec(
                    RemediationBannerSpec::new(
                        "Solid neutral recovery",
                        "Neutral solid surfaces keep the theme's primary foreground.",
                    )
                    .with_tone(StatusTone::Neutral)
                    .with_fill(ToneFill::Solid),
                    theme,
                ))
                .child(RemediationBanner::from_spec(
                    RemediationBannerSpec::new(
                        "Solid information",
                        "Tint-border colour promoted into the fill.",
                    )
                    .with_tone(StatusTone::Info)
                    .with_fill(ToneFill::Solid),
                    theme,
                ))
                .child(RemediationBanner::from_spec(
                    RemediationBannerSpec::new(
                        "Solid success",
                        "The operation completed successfully.",
                    )
                    .with_tone(StatusTone::Success)
                    .with_fill(ToneFill::Solid),
                    theme,
                ))
                .child(RemediationBanner::from_spec(
                    RemediationBannerSpec::new(
                        "Solid warning",
                        "Review this condition before continuing.",
                    )
                    .with_tone(StatusTone::Warning)
                    .with_fill(ToneFill::Solid),
                    theme,
                ))
                .child(RemediationBanner::from_spec(
                    RemediationBannerSpec::new(
                        "Solid danger recovery",
                        "Secondary and ghost actions stay readable on the solid surface.",
                    )
                    .with_tone(StatusTone::Danger)
                    .with_fill(ToneFill::Solid)
                    .with_secondary_action(RemediationAction::new("details", "View details")),
                    theme,
                ))
                .child(RemediationBanner::from_spec(
                    RemediationBannerSpec::new("Solid pending", "Recovery is still in progress.")
                        .with_tone(StatusTone::Pending)
                        .with_fill(ToneFill::Solid),
                    theme,
                )),
        ))
}
