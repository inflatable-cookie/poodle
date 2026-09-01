//! MotionPolicyProvider — GPUI specimen.
//!
//! Contract: `docs/contracts/components/motion-policy-provider.md`
//! Architecture: `docs/architecture/012-semantic-motion-policy.md`

use crate::app_state::AppState;
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_render::{motion_policy_provider, spinner, MotionPolicy, RenderContext};
use poodle_specs::{MotionPolicyProviderSpec, SpinnerSpec};

fn scoped_spinner(policy: MotionPolicy, ctx: &RenderContext<'_>) -> AnyElement {
    motion_policy_provider(
        &MotionPolicyProviderSpec::new().with_policy(policy),
        ctx,
        |scoped| poodle_gpui_node_backend::to_gpui(&spinner(&SpinnerSpec::new(), scoped)),
    )
}

pub(crate) fn render(
    state: &AppState,
    cx: &mut Context<PreviewRoot>,
    ctx: &RenderContext<'_>,
) -> Div {
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(12.0))
        .child(
            div()
                .child("full")
                .child(scoped_spinner(MotionPolicy::Full, &ctx)),
        )
        .child(
            div()
                .child("reduced")
                .child(scoped_spinner(MotionPolicy::Reduced, &ctx)),
        )
        .child(
            div()
                .child("frozen")
                .child(scoped_spinner(MotionPolicy::Frozen, &ctx)),
        )
        .into_any_element();
    specimen_layout(
        state,
        cx,
        "motion-policy-provider",
        examples,
        SpecimenAxes::examples_only(),
    )
}
