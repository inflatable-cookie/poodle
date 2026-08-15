//! GPUI headless primitive substrate probes (spec 066, g14.002 / g14.023).

use std::sync::{Arc, Mutex};
use poodle_render::conformance::{observe_tree, observe_tree_with_context, ObserveContext};
use poodle_render::primitive_probes::{
    build_probe_fixture, overlay_probe_fixture, probe_activate_gpui, probe_focus_gpui,
    probe_interface, run_neutral_probes, run_overlay_probes, ProbeEvidence, PROBE_ELEMENT_ID,
};
use super::conformance_driver::HeadlessDriver;

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

pub fn drive_primitive_probes(driver: &mut HeadlessDriver<'_>) -> Vec<ProbeEvidence> {
    let trace = Arc::new(Mutex::new(0usize));
    let trace_for_handler = Arc::clone(&trace);
    let handler: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
        *trace_for_handler.lock().expect("trace lock") += 1;
    });

    let node = Arc::new(Mutex::new(build_probe_fixture(Some(handler))));
    poodle_gpui_node_backend::begin_probe_capture();
    driver.mount_node(Arc::clone(&node));
    driver.blur_element_focus(PROBE_ELEMENT_ID);
    driver.wait_for_focus_handle(PROBE_ELEMENT_ID);

    let receipt = poodle_gpui_node_backend::take_probe_capture()
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>();

    let mut probes: Vec<ProbeEvidence> = {
        let node = node.lock().expect("node lock").clone();
        run_neutral_probes(&node)
            .into_iter()
            .filter(|probe| probe.capability_id != "focus" && probe.capability_id != "activate")
            .map(|probe| require_backend_receipt(probe, &receipt))
            .collect()
    };

    // focus — real backend focus registry after gpui FocusHandle.focus
    driver.focus_element(PROBE_ELEMENT_ID);
    let focus_probe = {
        let node = node.lock().expect("node lock").clone();
        let iface = probe_interface();
        let backend_focus = poodle_gpui_node_backend::focus_state_for(PROBE_ELEMENT_ID);
        let observation = observe_tree("gpui", "primitive-probe", &iface, &node, backend_focus);
        let focus_visible = observation.pointer("/parts/root/focusVisible").cloned();
        probe_focus_gpui(backend_focus, focus_visible)
    };
    probes.push(focus_probe);

    // activate — real pointer path through the node-backend listener
    driver.pointer_activate();
    let activate_probe = {
        let count = *trace.lock().expect("trace lock");
        probe_activate_gpui(count)
    };
    probes.push(activate_probe);

    // ── Overlay rows (g14.005): the overlay probe fixture mounts as a real
    // ── layer; Escape routes through the window dispatch tree to the
    // ── probe's dismiss handler, and the layer registry reports the open
    // ── layer count and containment bounds.
    let dismiss_reasons: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));
    let mut overlay_fixture = overlay_probe_fixture();
    {
        let recorded = Arc::clone(&dismiss_reasons);
        let handler: poodle_node::DismissHandler = Arc::new(move |reason| {
            recorded.lock().expect("reason lock").push(match reason {
                poodle_node::DismissReason::Escape => "escape",
                poodle_node::DismissReason::Outside => "outside",
            });
        });
        // The fixture's trigger is its root node.
        overlay_fixture.interaction.on_dismiss = Some(handler);
    }
    let overlay_node = Arc::new(Mutex::new(overlay_fixture));
    driver.mount_node(Arc::clone(&overlay_node));
    driver.draw_frame();
    let layer_count = poodle_gpui_node_backend::open_layer_count();
    // The real Escape route: the mount host's key handler runs the backend's
    // innermost dismissal through the dispatch tree.
    driver.dispatch_key("escape");
    let escape_reasons = dismiss_reasons.lock().expect("reason lock").clone();
    let overlay_observation = {
        let node = overlay_node.lock().expect("node lock").clone();
        let iface = poodle_render::primitive_probes::overlay_probe_interface();
        let focus_by_id = |id: &str| poodle_gpui_node_backend::focus_state_for(id);
        let bounds_by_id = |id: &str| {
            poodle_gpui_node_backend::bounds_for(id).map(|bounds| {
                (
                    f32::from(bounds.origin.y),
                    f32::from(bounds.origin.x),
                    f32::from(bounds.size.width),
                    f32::from(bounds.size.height),
                )
            })
        };
        observe_tree_with_context(
            "gpui",
            "overlay-probe",
            &iface,
            &node,
            &ObserveContext {
                focus_by_id: &focus_by_id,
                layer_count: &(|| Some(layer_count)),
                bounds_by_id: &bounds_by_id,
            },
        )
    };
    for probe in run_overlay_probes(&overlay_node.lock().expect("node lock").clone()) {
        let probe = overlay_backend_evidence(probe, layer_count, &escape_reasons, &overlay_observation);
        probes.push(probe);
    }

    // ── Input rows (g14.006): a real TextInput, one keystroke through
    // ── on_edit_key, and an IME mark that must not commit.
    let typed: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let spec = poodle_specs::TextInputSpec::new()
        .with_id("input-probe")
        .with_value("");
    let on_change = {
        let typed = Arc::clone(&typed);
        Arc::new(move |next: &str| {
            typed.lock().expect("typed lock").push(next.to_owned());
        })
    };
    let mut input_node = poodle_render::text_input_with_handlers(
        &spec,
        &poodle_gpui::GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ECLIPSE),
        poodle_render::TextInputHandlers {
            on_change: Some(on_change),
            ..poodle_render::TextInputHandlers::default()
        },
    );
    input_node.id = Some(poodle_render::primitive_probes::INPUT_PROBE_ELEMENT_ID.to_owned());
    let input_mounted = Arc::new(Mutex::new(input_node));
    driver.mount_node(Arc::clone(&input_mounted));
    driver.wait_for_focus_handle(poodle_render::primitive_probes::INPUT_PROBE_ELEMENT_ID);
    driver.focus_element(poodle_render::primitive_probes::INPUT_PROBE_ELEMENT_ID);
    driver.keyboard_key(poodle_render::primitive_probes::INPUT_PROBE_ELEMENT_ID, "a");
    let typed_values = typed.lock().expect("typed lock").clone();
    poodle_gpui_node_backend::mark_composing(
        "poodle-input-input-probe-value",
        (0, 0),
        "ñ",
    );
    let composing = poodle_gpui_node_backend::take_composing("poodle-input-input-probe-value");
    for probe in poodle_render::primitive_probes::run_input_probes(&poodle_render::primitive_probes::input_probe_fixture())
    {
        probes.push(input_backend_evidence(probe, &typed_values, composing.as_deref()));
    }

    driver.drain();
    probes
}

/// Graft the real GPUI evidence onto the neutral overlay probes: the painted
/// overlay channel, the live layer registry, the real Escape route, and the
/// observed layer-count field.
fn overlay_backend_evidence(
    probe: ProbeEvidence,
    layer_count: usize,
    escape_reasons: &[&'static str],
    observation: &serde_json::Value,
) -> ProbeEvidence {
    if probe.verdict == "fail" {
        return probe;
    }
    let observed_layer_count = observation
        .pointer("/parts/root/layerCount")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0) as usize;
    let overlay_observed = observation
        .pointer("/parts/surface/overlay")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let expanded_observed = observation
        .pointer("/parts/root/expanded")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    match probe.capability_id.as_str() {
        "overlay.intent" => {
            if overlay_observed {
                ProbeEvidence::pass_observed(
                    "overlay.intent",
                    "backend-overlay",
                    serde_json::json!({ "neutral": probe.fields, "gpui.overlay": overlay_observed }),
                    &["node.field", "parts.overlay", "gpui.layout"],
                )
            } else {
                ProbeEvidence::fail(
                    "overlay.intent",
                    "backend-overlay",
                    serde_json::json!({ "gpui.overlay": overlay_observed }),
                    "parts.surface.overlay not observed by gpui",
                )
            }
        }
        "semantic.expanded" => {
            if expanded_observed {
                ProbeEvidence::pass_observed(
                    "semantic.expanded",
                    "backend-expanded",
                    serde_json::json!({ "neutral": probe.fields, "gpui.expanded": expanded_observed }),
                    &["node.a11y", "parts.expanded"],
                )
            } else {
                ProbeEvidence::fail(
                    "semantic.expanded",
                    "backend-expanded",
                    serde_json::json!({ "gpui.expanded": expanded_observed }),
                    "parts.root.expanded not observed by gpui",
                )
            }
        }
        "overlay.dismiss" => {
            if escape_reasons.contains(&"escape") {
                ProbeEvidence::pass_observed(
                    "overlay.dismiss",
                    "backend-escape",
                    serde_json::json!({ "neutral": probe.fields, "escape": escape_reasons }),
                    &["node.field", "trace", "gpui.dismiss"],
                )
            } else {
                ProbeEvidence::fail(
                    "overlay.dismiss",
                    "backend-escape",
                    serde_json::json!({ "escape": escape_reasons }),
                    "real Escape route never reached the dismiss handler",
                )
            }
        }
        "overlay.layer" => {
            if layer_count == 1 && observed_layer_count == 1 {
                ProbeEvidence::pass_observed(
                    "overlay.layer",
                    "backend-layer",
                    serde_json::json!({
                        "neutral": probe.fields,
                        "gpui.layerCount": layer_count,
                        "parts.layerCount": observed_layer_count,
                    }),
                    &["node.field", "parts.layerCount", "gpui.layer"],
                )
            } else {
                ProbeEvidence::fail(
                    "overlay.layer",
                    "backend-layer",
                    serde_json::json!({ "layerCount": layer_count }),
                    "layer registry or observed layerCount",
                )
            }
        }
        _ => probe,
    }
}

fn input_backend_evidence(
    probe: ProbeEvidence,
    typed: &[String],
    composing: Option<&str>,
) -> ProbeEvidence {
    if probe.verdict == "fail" {
        return probe;
    }
    match probe.capability_id.as_str() {
        "input.value" => ProbeEvidence::pass_observed(
            "input.value",
            "backend-input-value",
            serde_json::json!({ "neutral": probe.fields, "typed": typed }),
            &["node.field", "parts.value", "parts.selection"],
        ),
        "input.editing" => {
            if typed.iter().any(|value| value == "a") {
                ProbeEvidence::pass_observed(
                    "input.editing",
                    "backend-key-insert",
                    serde_json::json!({ "neutral": probe.fields, "typed": typed }),
                    &["node.field", "gpui.event", "trace"],
                )
            } else {
                ProbeEvidence::fail(
                    "input.editing",
                    "backend-key-insert",
                    serde_json::json!({ "typed": typed }),
                    "keystroke did not produce a valueChange",
                )
            }
        }
        "input.ime" => {
            if composing == Some("ñ") && typed.iter().all(|value| value != "ñ") {
                ProbeEvidence::pass_observed(
                    "input.ime",
                    "backend-ime-buffer",
                    serde_json::json!({
                        "neutral": probe.fields,
                        "composing": composing,
                        "typed": typed,
                    }),
                    &["node.field", "trace", "parts.value"],
                )
            } else {
                ProbeEvidence::fail(
                    "input.ime",
                    "backend-ime-buffer",
                    serde_json::json!({ "composing": composing, "typed": typed }),
                    "IME mark committed or dropped the composing buffer",
                )
            }
        }
        _ => probe,
    }
}
