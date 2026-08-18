use std::sync::Arc;

use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::Eyebrow;
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::update::{
    Channel, DeferralCause, InstallManager, OfferReason, UpdateAvailabilityProjection,
    UpdateControllerStatus, UpdateDeferral, UpdateProgressProjection, UpdateRejectionCode,
};
use poodle_render::{update_status, UpdateStatusHandlers};
use poodle_specs::{EyebrowSpec, UpdateStatusSpec};

fn group(theme: &GpuiThemeProvider, label: &str, child: impl IntoElement) -> Div {
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

fn offer() -> UpdateAvailabilityProjection {
    UpdateAvailabilityProjection::Offer {
        version: "1.4.0".to_string(),
        reason: OfferReason::Staged,
        notes: Some("Bug fixes and improvements.".to_string()),
    }
}

fn status_element(
    spec: UpdateStatusSpec,
    theme: &GpuiThemeProvider,
    handlers: UpdateStatusHandlers,
) -> AnyElement {
    poodle_gpui_node_backend::to_gpui(&update_status(&spec, theme, handlers))
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let confirm_open = state
        .specimens
        .toggles
        .get("update-status-confirm")
        .copied()
        .unwrap_or(false);
    let events = state.node_events.clone();
    let interactive_handlers = UpdateStatusHandlers {
        instance_id: Some("update-status-demo".to_string()),
        on_confirm_open_change: Some(Arc::new(move |open| {
            events.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
                key: "update-status-confirm".to_string(),
                value: open,
            });
        })),
        ..UpdateStatusHandlers::default()
    };
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(420.0))
        .child(group(
            theme,
            "Offer",
            status_element(
                UpdateStatusSpec::new()
                    .with_status(UpdateControllerStatus::Ready)
                    .with_availability(offer())
                    .with_confirm_open(confirm_open),
                theme,
                interactive_handlers,
            ),
        ))
        .child(group(
            theme,
            "Up to date",
            status_element(
                UpdateStatusSpec::new()
                    .with_status(UpdateControllerStatus::Ready)
                    .with_availability(UpdateAvailabilityProjection::UpToDate)
                    .with_installed_version("1.3.0")
                    .with_channel(Channel::Production),
                theme,
                UpdateStatusHandlers::default(),
            ),
        ))
        .child(group(
            theme,
            "Ahead of channel",
            status_element(
                UpdateStatusSpec::new()
                    .with_status(UpdateControllerStatus::Ready)
                    .with_availability(UpdateAvailabilityProjection::AheadOfChannel {
                        installed: "1.3.0-nightly.4".to_string(),
                        channel: "1.2.9".to_string(),
                    }),
                theme,
                UpdateStatusHandlers::default(),
            ),
        ))
        .child(group(
            theme,
            "Downloading — indeterminate",
            status_element(
                UpdateStatusSpec::new()
                    .with_status(UpdateControllerStatus::Ready)
                    .with_progress(UpdateProgressProjection::Downloading { fraction: None }),
                theme,
                UpdateStatusHandlers::default(),
            ),
        ))
        .child(group(
            theme,
            "Downloading — zero bar",
            status_element(
                UpdateStatusSpec::new()
                    .with_status(UpdateControllerStatus::Ready)
                    .with_progress(UpdateProgressProjection::Downloading {
                        fraction: Some(0.0),
                    }),
                theme,
                UpdateStatusHandlers::default(),
            ),
        ))
        .child(group(
            theme,
            "Managed elsewhere",
            status_element(
                UpdateStatusSpec::new()
                    .with_status(UpdateControllerStatus::Ready)
                    .with_availability(UpdateAvailabilityProjection::ManagedElsewhere {
                        version: "1.4.0".to_string(),
                        manager: InstallManager::HomebrewCask,
                    })
                    .with_deferral(UpdateDeferral {
                        version: "1.4.0".to_string(),
                        cause: DeferralCause::ExternallyManaged {
                            manager: InstallManager::HomebrewCask,
                            command: Some("brew upgrade finch".to_string()),
                        },
                    }),
                theme,
                UpdateStatusHandlers::default(),
            ),
        ))
        .child(group(
            theme,
            "Signature rejected",
            status_element(
                UpdateStatusSpec::new()
                    .with_status(UpdateControllerStatus::Ready)
                    .with_last_rejection(UpdateRejectionCode::SignatureRejected),
                theme,
                UpdateStatusHandlers::default(),
            ),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "update-status",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                status_element(
                    UpdateStatusSpec::new()
                        .with_status(UpdateControllerStatus::Ready)
                        .with_availability(offer())
                        .with_size(size),
                    theme,
                    UpdateStatusHandlers::default(),
                )
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                status_element(
                    UpdateStatusSpec::new()
                        .with_status(UpdateControllerStatus::Ready)
                        .with_availability(offer())
                        .with_density(density),
                    theme,
                    UpdateStatusHandlers::default(),
                )
            }),
    )
}
