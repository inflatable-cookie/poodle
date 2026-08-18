use std::sync::Arc;

use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::Eyebrow;
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::update::{
    OfferReason, UpdateAvailabilityProjection, UpdateControllerStatus, UpdatePresence,
    UpdateProgressProjection,
};
use poodle_render::{update_center, UpdateCenterHandlers};
use poodle_specs::{EyebrowSpec, UpdateCenterSpec};

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

fn center_element(
    spec: UpdateCenterSpec,
    theme: &GpuiThemeProvider,
    handlers: UpdateCenterHandlers,
) -> AnyElement {
    poodle_gpui_node_backend::to_gpui(&update_center(&spec, theme, handlers))
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let open = state
        .specimens
        .toggles
        .get("update-center-open")
        .copied()
        .unwrap_or(true);
    let events = state.node_events.clone();
    let handlers = UpdateCenterHandlers {
        instance_id: Some("update-center-demo".to_string()),
        on_open_change: Some(Arc::new(move |next| {
            events.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
                key: "update-center-open".to_string(),
                value: next,
            });
        })),
        ..UpdateCenterHandlers::default()
    };
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(420.0))
        .child(group(
            theme,
            "Attention — offer",
            center_element(
                UpdateCenterSpec::new(UpdatePresence::Attention)
                    .with_status(UpdateControllerStatus::Ready)
                    .with_availability(offer())
                    .with_open(open),
                theme,
                handlers,
            ),
        ))
        .child(group(
            theme,
            "Quiet — downloading",
            center_element(
                UpdateCenterSpec::new(UpdatePresence::Quiet).with_progress(
                    UpdateProgressProjection::Downloading {
                        fraction: Some(0.42),
                    },
                ),
                theme,
                UpdateCenterHandlers::default(),
            ),
        ))
        .child(group(
            theme,
            "Hidden — withheld by rollout",
            center_element(
                UpdateCenterSpec::new(UpdatePresence::Hidden)
                    .with_status(UpdateControllerStatus::Ready)
                    .with_availability(UpdateAvailabilityProjection::WithheldByRollout {
                        version: "2.0.0".to_string(),
                    }),
                theme,
                UpdateCenterHandlers::default(),
            ),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "update-center",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                center_element(
                    UpdateCenterSpec::new(UpdatePresence::Attention)
                        .with_status(UpdateControllerStatus::Ready)
                        .with_availability(offer())
                        .with_size(size),
                    theme,
                    UpdateCenterHandlers::default(),
                )
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                center_element(
                    UpdateCenterSpec::new(UpdatePresence::Attention)
                        .with_status(UpdateControllerStatus::Ready)
                        .with_availability(offer())
                        .with_density(density),
                    theme,
                    UpdateCenterHandlers::default(),
                )
            }),
    )
}
