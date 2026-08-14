//! GPUI windowed primitive substrate probes (spec 066, g14.002).

use std::sync::{Arc, Mutex};

use gpui::*;
use poodle_render::conformance::observe_tree;
use poodle_render::primitive_probes::{
    build_probe_fixture, probe_activate_gpui, probe_focus_gpui, probe_interface,
    run_neutral_probes, ProbeEvidence, PROBE_ELEMENT_ID,
};
use serde_json::Value;

use super::conformance_driver::{
    blur_element_focus, drain_event_queue, focus_element, mount_node, pointer_activate,
    wait_for_focus_handle, warmup_and_calibrate, ClickCalibration,
};

fn backend_expected(capability_id: &str) -> &'static [&'static str] {
    match capability_id {
        "structure.identity" => &["structure.identity.container"],
        "structure.part-resolution" => &["content.text-icon.text", "content.text-icon.icon"],
        "layout.intent" => &["layout.intent.direction", "layout.intent.gap"],
        "layout.geometry" => &[
            "layout.geometry.flex-grow",
            "layout.geometry.min-width",
            "layout.geometry.max-width",
        ],
        "layout.position" => &["layout.position.relative"],
        "surface.channels" => &[
            "surface.channels.background",
            "surface.channels.border",
            "surface.channels.text",
            "surface.channels.opacity",
        ],
        "surface.extended" => &[
            "surface.extended.side-border",
            "surface.extended.shadow",
            "surface.extended.cursor",
        ],
        "surface.state-patches" => &[
            "surface.state-patches.hover",
            "surface.state-patches.active",
            "surface.state-patches.focus",
        ],
        "surface.animation" => &["surface.animation.scheduled"],
        "content.text-icon" => &["content.text-icon.text", "content.text-icon.icon"],
        "content.typography" => &[
            "content.typography.size",
            "content.typography.weight",
            "content.typography.line-height",
        ],
        "semantic.token-roles" => &["semantic.token-roles.received"],
        "toggle" => &["toggle.received"],
        "semantic.disabled" => &["semantic.disabled.blocked"],
        "accessibility.projection" => &["accessibility.projection.received"],
        "focus" | "activate" => &[],
        _ => &[],
    }
}

fn require_backend_receipt(
    probe: ProbeEvidence,
    receipt: &std::collections::BTreeSet<&'static str>,
) -> ProbeEvidence {
    if probe.verdict == "fail" {
        return probe;
    }
    let expected = backend_expected(&probe.capability_id);
    let missing = expected
        .iter()
        .copied()
        .find(|marker| !receipt.contains(marker));
    let fields = serde_json::json!({
        "neutral": probe.fields,
        "backendMarkers": expected,
    });
    match missing {
        Some(marker) => ProbeEvidence::fail(
            probe.capability_id,
            format!("backend-{}", probe.probe_id),
            fields,
            format!("backend.{marker}"),
        ),
        None => ProbeEvidence::pass_observed(
            probe.capability_id,
            format!("backend-{}", probe.probe_id),
            fields,
            &probe.observations,
        ),
    }
}

pub async fn drive_primitive_probes(cx: &mut AsyncWindowContext) -> Vec<ProbeEvidence> {
    let calibration = warmup_and_calibrate(cx).await;
    let trace = Arc::new(Mutex::new(0usize));
    let trace_for_handler = Arc::clone(&trace);
    let handler: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
        *trace_for_handler.lock().expect("trace lock") += 1;
    });

    let node = Arc::new(Mutex::new(build_probe_fixture(Some(handler))));
    cx.update(|_window, _cx| poodle_gpui_node_backend::begin_probe_capture())
        .ok();
    mount_node(cx, Arc::clone(&node));
    blur_element_focus(cx, PROBE_ELEMENT_ID).await;
    wait_for_focus_handle(cx, PROBE_ELEMENT_ID).await;

    let receipt = cx
        .update(|_window, _cx| {
            poodle_gpui_node_backend::take_probe_capture()
                .into_iter()
                .collect::<std::collections::BTreeSet<_>>()
        })
        .unwrap_or_default();

    let mut probes: Vec<ProbeEvidence> = cx
        .update(|_window, _cx| {
            let node = node.lock().expect("node lock").clone();
            run_neutral_probes(&node)
                .into_iter()
                .filter(|probe| probe.capability_id != "focus" && probe.capability_id != "activate")
                .map(|probe| require_backend_receipt(probe, &receipt))
                .collect()
        })
        .unwrap_or_default();

    // focus — real backend focus registry after gpui FocusHandle.focus
    focus_element(cx, PROBE_ELEMENT_ID).await;
    let focus_probe = cx
        .update(|_window, _cx| {
            let node = node.lock().expect("node lock").clone();
            let iface = probe_interface();
            let backend_focus = poodle_gpui_node_backend::focus_state_for(PROBE_ELEMENT_ID);
            let observation = observe_tree("gpui", "primitive-probe", &iface, &node, backend_focus);
            let focus_visible = observation.pointer("/parts/root/focusVisible").cloned();
            probe_focus_gpui(backend_focus, focus_visible)
        })
        .unwrap_or_else(|_| {
            ProbeEvidence::fail("focus", "backend-focus-registry", Value::Null, "gpui.focus")
        });
    probes.push(focus_probe);

    // activate — real NSEvent pointer path through node-backend listener
    pointer_activate(cx, calibration).await;
    let activate_probe = cx
        .update(|_window, _cx| {
            let count = *trace.lock().expect("trace lock");
            probe_activate_gpui(count)
        })
        .unwrap_or_else(|_| {
            ProbeEvidence::fail("activate", "pointer-activate", Value::Null, "gpui.event")
        });
    probes.push(activate_probe);

    drain_event_queue(cx).await;
    probes
}

#[allow(dead_code)]
pub async fn drive_primitive_probes_with_calibration(
    cx: &mut AsyncWindowContext,
    calibration: ClickCalibration,
) -> Vec<ProbeEvidence> {
    let _ = calibration;
    drive_primitive_probes(cx).await
}
