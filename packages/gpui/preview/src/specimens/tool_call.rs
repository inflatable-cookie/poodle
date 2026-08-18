use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, ToolCall};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::agent_transcript::ToolCallStatus;
use poodle_specs::{EyebrowSpec, ToolCallSpec};
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

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let expanded = state.specimens.is_on("tool-call-with-output");
    let events = state.node_events.clone();

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            "Status",
            theme,
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(ToolCall::from_spec(
                    ToolCallSpec::new("ok", "Ran command")
                        .with_detail("effigy cp-api/test:latex")
                        .with_status(ToolCallStatus::Success),
                    theme,
                ))
                .child(ToolCall::from_spec(
                    ToolCallSpec::new("run", "Ran command")
                        .with_detail("cargo build --release")
                        .with_status(ToolCallStatus::Running),
                    theme,
                ))
                .child(ToolCall::from_spec(
                    ToolCallSpec::new("err", "Ran command")
                        .with_detail("effigy check:gpui")
                        .with_status(ToolCallStatus::Error),
                    theme,
                )),
        ))
        .child(group(
            "Kinds",
            theme,
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(ToolCall::from_spec(
                    ToolCallSpec::new("k1", "File change")
                        .with_detail("packages/styles/src/tool-call.css"),
                    theme,
                ))
                .child(ToolCall::from_spec(
                    ToolCallSpec::new("k2", "Searched").with_detail("ResizeObserver"),
                    theme,
                ))
                .child(ToolCall::from_spec(
                    ToolCallSpec::new("k4", "Something else")
                        .with_detail("with an explicit icon")
                        .with_icon("sparkles"),
                    theme,
                )),
        ))
        .child(group(
            "Output",
            theme,
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(ToolCall::from_spec(
                    ToolCallSpec::new("no-output", "Ran command")
                        .with_detail("no output, not interactive"),
                    theme,
                ))
                .child(
                    ToolCall::from_spec(
                        ToolCallSpec::new("with-output", "Ran command")
                            .with_detail("bun test")
                            .with_output("272 pass\n0 fail")
                            .with_expanded(expanded),
                        theme,
                    )
                    .on_toggle(Arc::new(move |id| {
                        events
                            .lock()
                            .unwrap()
                            .push(NodeSpecimenEvent::Toggle(format!("tool-call-{id}")));
                    }))
                    .with_instance_id("with-output"),
                ),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "tool-call",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                ToolCall::from_spec(
                    ToolCallSpec::new(format!("sz-{size:?}"), "Ran command")
                        .with_detail(format!("size {size:?}"))
                        .with_size(size),
                    theme,
                )
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                ToolCall::from_spec(
                    ToolCallSpec::new(format!("dn-{density:?}"), "Ran command")
                        .with_detail(format!("density {density:?}"))
                        .with_density(density),
                    theme,
                )
                .into_any_element()
            }),
    )
}
