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

pub async fn drive_primitive_probes(cx: &mut AsyncWindowContext) -> Vec<ProbeEvidence> {
    let calibration = warmup_and_calibrate(cx).await;
    let trace = Arc::new(Mutex::new(0usize));
    let trace_for_handler = Arc::clone(&trace);
    let handler: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
        *trace_for_handler.lock().expect("trace lock") += 1;
    });

    let node = Arc::new(Mutex::new(build_probe_fixture(Some(handler))));
    mount_node(cx, Arc::clone(&node));
    blur_element_focus(cx, PROBE_ELEMENT_ID).await;
    wait_for_focus_handle(cx, PROBE_ELEMENT_ID).await;

    let mut probes = cx
        .update(|_window, _cx| {
            let node = node.lock().expect("node lock").clone();
            run_neutral_probes(&node)
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
        .unwrap_or_else(|_| ProbeEvidence::fail("activate", "pointer-activate", Value::Null, "gpui.event"));
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
