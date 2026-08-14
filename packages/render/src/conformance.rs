//! Native conformance machinery (spec 066, architecture 009).
//!
//! The shared, component-agnostic half of the native runners: node-tree
//! observation into `component-observation.v1`, stable-part resolution by
//! convention, and case-step/assertion evaluation over the serialized corpus.
//! The per-runtime runners (gpui / jetstream preview bins) supply the real
//! mount, the real backend event path, and the host toggle behaviour; nothing
//! here knows Button specifics beyond the part convention the interface
//! declares.

use std::collections::BTreeMap;

use poodle_layout::LayoutSizing;
use poodle_node::{Node, NodeKind, NodeRole, NodeToggled};
use serde_json::{json, Value};
/// Geometry fields, mirroring the web observer's field set.
const GEOMETRY_FIELDS: &[&str] = &[
    "height",
    "minWidth",
    "paddingLeft",
    "paddingRight",
    "radius",
    "borderWidth",
];

/// Part ids the interface declares; resolved by convention against the tree.
pub const PART_IDS: &[&str] = &[
    "root",
    "label",
    "leadingIcon",
    "trailingIcon",
    "spinner",
    "chevron",
];

/// Finds a stable part in the node tree by the interface's convention.
pub fn find_part<'a>(root: &'a Node, part_id: &str) -> Option<&'a Node> {
    match part_id {
        "root" => Some(root),
        // The label lives in the Button node itself when there are no icons
        // and in a Text child when there are; either way it is part of root.
        "label" => label_text(root).map(|_| root),
        "leadingIcon" | "trailingIcon" => content_icons(root)
            .filter(|icons| !icons.is_empty())
            .map(|icons| {
                if part_id == "leadingIcon" {
                    icons.first().copied()
                } else {
                    icons.last().copied()
                }
            })
            .flatten(),
        "spinner" => icon_named(root, "spinner"),
        "chevron" => icon_named(root, "chevron-down"),
        _ => None,
    }
}

/// Every Icon child except the spinner and chevron, in tree order.
fn content_icons(root: &Node) -> Option<Vec<&Node>> {
    let icons: Vec<&Node> = root
        .children
        .iter()
        .filter(|child| matches!(&child.kind, NodeKind::Icon { name, .. } if name != "spinner" && name != "chevron-down"))
        .collect();
    Some(icons)
}

fn icon_named<'a>(root: &'a Node, name: &str) -> Option<&'a Node> {
    root.children.iter().find(|child| {
        matches!(&child.kind, NodeKind::Icon { name: n, .. } if n == name)
    })
}

/// The visible label text: the button's own label or its first Text child.
fn label_text(root: &Node) -> Option<String> {
    match &root.kind {
        NodeKind::Button { label } if !label.is_empty() => Some(label.clone()),
        NodeKind::Button { .. } | _ => first_text(root).map(|node| match &node.kind {
            NodeKind::Text { content } => content.clone(),
            _ => unreachable!("first_text returns Text nodes"),
        }),
    }
}

fn first_text(root: &Node) -> Option<&Node> {
    root.children
        .iter()
        .find(|child| matches!(child.kind, NodeKind::Text { .. }))
}

fn role_of(node: &Node) -> Option<String> {
    if let Some(role) = &node.a11y.role {
        return Some(role_name(role));
    }
    match &node.kind {
        NodeKind::Button { .. } => Some("button".to_owned()),
        _ => None,
    }
}

fn role_name(role: &NodeRole) -> String {
    format!("{role:?}").to_ascii_lowercase()
}

fn rgba(color: &poodle_node::ColorValue) -> String {
    let [r, g, b, a] = [color.0, color.1, color.2, color.3];
    format!("rgba({r},{g},{b},{a})")
}

/// Observes one part into the observation JSON.
fn observe_part(part_id: &str, node: &Node, root: &Node) -> Value {
    let (geometry, channels) = match part_id {
        "root" => (geometry_of(node), channels_of(node)),
        _ => (json!({}), json!({})),
    };
    let (present, text, name, icon) = match part_id {
        "label" => {
            let text = label_text(root);
            (text.is_some(), text.clone(), text.clone(), None)
        }
        _ => (
            true,
            text_of(node),
            name_of(node, root),
            icon_of(node),
        ),
    };
    if !present {
        return absent_part();
    }
    json!({
        "present": true,
        "role": role_of(node),
        "name": name,
        "text": text,
        "icon": icon,
        "states": states_of(node, root),
        "tokenRoles": {},
        "focusable": node.interaction.focusable && !node.interaction.disabled,
        "focused": null,
        "focusVisible": null,
        "geometry": geometry,
        "channels": channels,
    })
}

fn absent_part() -> Value {
    json!({
        "present": false,
        "role": null,
        "name": null,
        "text": null,
        "icon": null,
        "states": {},
        "tokenRoles": {},
        "focusable": false,
        "focused": null,
        "focusVisible": null,
        "geometry": {},
        "channels": {},
    })
}

fn states_of(node: &Node, root: &Node) -> Value {
    let mut states = BTreeMap::new();
    states.insert("disabled".to_owned(), json!(node.interaction.disabled));
    states.insert(
        "loading".to_owned(),
        json!(icon_named(root, "spinner").is_some()),
    );
    states.insert(
        "pressed".to_owned(),
        json!(matches!(node.a11y.toggled, Some(NodeToggled::True))),
    );
    serde_json::to_value(states).expect("states serialize")
}

fn name_of(node: &Node, root: &Node) -> Option<String> {
    node.a11y
        .label
        .clone()
        .or_else(|| label_text(root))
}

fn text_of(node: &Node) -> Option<String> {
    match &node.kind {
        NodeKind::Text { content } => Some(content.clone()),
        _ => None,
    }
}

fn icon_of(node: &Node) -> Option<String> {
    match &node.kind {
        NodeKind::Icon { name, .. } => Some(name.clone()),
        _ => None,
    }
}

fn geometry_of(node: &Node) -> Value {
    let descriptor = &node.style.descriptor;
    let mut geometry = BTreeMap::new();
    if let LayoutSizing::Fixed(height) = descriptor.layout.height {
        geometry.insert("height".to_owned(), json!(height));
    }
    geometry.insert("minWidth".to_owned(), json!(node.style.min_width.unwrap_or(0.0)));
    geometry.insert(
        "paddingLeft".to_owned(),
        json!(descriptor.layout.spacing.padding.left),
    );
    geometry.insert(
        "paddingRight".to_owned(),
        json!(descriptor.layout.spacing.padding.right),
    );
    geometry.insert(
        "radius".to_owned(),
        json!(descriptor.corner_radii.top_left),
    );
    geometry.insert("borderWidth".to_owned(), json!(descriptor.border.width));
    serde_json::to_value(geometry).expect("geometry serializes")
}

fn channels_of(node: &Node) -> Value {
    let descriptor = &node.style.descriptor;
    let mut channels = BTreeMap::new();
    if let Some(color) = descriptor.background {
        channels.insert("background".to_owned(), json!(rgba(&color)));
    }
    channels.insert(
        "borderColor".to_owned(),
        json!(rgba(&descriptor.border.color)),
    );
    if let Some(color) = descriptor.text_color {
        channels.insert("color".to_owned(), json!(rgba(&color)));
    }
    channels.insert("opacity".to_owned(), json!(descriptor.opacity));
    serde_json::to_value(channels).expect("channels serialize")
}

/// Observes the whole tree into `component-observation.v1` shape.
pub fn observe_tree(runtime: &str, component: &str, root: &Node) -> Value {
    let mut parts = BTreeMap::new();
    for part_id in PART_IDS {
        let observed = find_part(root, part_id);
        parts.insert(
            (*part_id).to_owned(),
            match observed {
                Some(node) => observe_part(part_id, node, root),
                None => absent_part(),
            },
        );
    }
    let observation = json!({
        "runtime": runtime,
        "component": component,
        "parts": parts,
        "trace": [],
    });
    // The runner stamps the live trace; here we return the parts frame.
    observation
}

/// One assertion result, serialized exactly like the web runner's.
#[derive(Debug, serde::Serialize)]
pub struct AssertionResult {
    #[serde(rename = "stepIndex")]
    pub step_index: usize,
    pub part: Option<String>,
    pub field: String,
    pub verdict: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual: Option<Value>,
}

fn verdict_result(
    step_index: usize,
    part: Option<&str>,
    field: String,
    verdict: &'static str,
    expected: Option<Value>,
    actual: Option<Value>,
) -> AssertionResult {
    AssertionResult {
        step_index,
        part: part.map(str::to_owned),
        field,
        verdict,
        expected,
        actual,
    }
}

fn numbers_match(expected: &Value, actual: &Value, tolerance: f64) -> bool {
    match (expected.as_f64(), actual.as_f64()) {
        (Some(e), Some(a)) => (e - a).abs() <= tolerance,
        _ => expected == actual,
    }
}

/// What the runner must provide per runtime.
pub trait NativeHarness {
    fn runtime(&self) -> &'static str;
    fn component(&self) -> &'static str;
    fn observe(&self) -> Value;
    fn press(&mut self, part: &str, input: &str);
    fn focus(&mut self, part: &str);
    fn trace(&self) -> Vec<String>;
}

/// Evaluates one case's steps against the harness. `steps` are the raw
/// serialized step objects from the corpus.
pub fn evaluate_steps(
    component: &str,
    steps: &[Value],
    harness: &mut dyn NativeHarness,
) -> Vec<AssertionResult> {
    let mut out = Vec::new();
    for (index, step) in steps.iter().enumerate() {
        let kind = step
            .get("kind")
            .and_then(Value::as_str)
            .unwrap_or_default();
        match kind {
            "action" => {
                let part = step.get("part").and_then(Value::as_str).unwrap_or("");
                let input = step
                    .get("input")
                    .and_then(Value::as_str)
                    .unwrap_or("pointer");
                match step.get("name").and_then(Value::as_str).unwrap_or("") {
                    "press" => harness.press(part, input),
                    "focus" => harness.focus(part),
                    other => out.push(verdict_result(
                        index,
                        Some(part),
                        format!("action.{other}"),
                        "fail",
                        None,
                        None,
                    )),
                }
            }
            "expectPart" => {
                let part = step.get("part").and_then(Value::as_str).unwrap_or("");
                let expect = step.get("expect").cloned().unwrap_or(Value::Null);
                assert_part(component, part, &expect, index, harness, &mut out);
            }
            "expectEvents" => {
                let expected = step
                    .get("events")
                    .and_then(Value::as_array)
                    .map(|events| {
                        events
                            .iter()
                            .filter_map(Value::as_str)
                            .map(str::to_owned)
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();
                let actual = harness.trace();
                let pass = actual == expected;
                out.push(verdict_result(
                    index,
                    None,
                    "events".to_owned(),
                    if pass { "pass" } else { "fail" },
                    Some(json!(expected)),
                    Some(json!(actual)),
                ));
            }
            other => out.push(verdict_result(
                index,
                None,
                format!("step.{other}"),
                "fail",
                None,
                None,
            )),
        }
    }
    out
}

fn assert_part(
    _component: &str,
    part: &str,
    expect: &Value,
    step_index: usize,
    harness: &dyn NativeHarness,
    out: &mut Vec<AssertionResult>,
) {
    let observation = harness.observe();
    let observed = observation
        .get("parts")
        .and_then(|parts| parts.get(part));
    let Some(observed) = observed else {
        out.push(verdict_result(
            step_index,
            Some(part),
            "present".to_owned(),
            "fail",
            Some(json!(true)),
            None,
        ));
        return;
    };

    let present = observed.get("present").and_then(Value::as_bool).unwrap_or(false);
    if !present && expect.get("present").and_then(Value::as_bool).unwrap_or(true) {
        out.push(verdict_result(
            step_index,
            Some(part),
            "present".to_owned(),
            "fail",
            Some(json!(true)),
            Some(json!(false)),
        ));
        return;
    }

    for field in ["role", "name", "text", "icon"] {
        if let Some(expected) = expect.get(field) {
            let actual = observed.get(field).cloned().unwrap_or(Value::Null);
            if actual.is_null() {
                out.push(verdict_result(
                    step_index,
                    Some(part),
                    field.to_owned(),
                    "vacuous",
                    Some(expected.clone()),
                    None,
                ));
            } else if actual == *expected {
                out.push(verdict_result(
                    step_index,
                    Some(part),
                    field.to_owned(),
                    "pass",
                    Some(expected.clone()),
                    Some(actual),
                ));
            } else {
                out.push(verdict_result(
                    step_index,
                    Some(part),
                    field.to_owned(),
                    "fail",
                    Some(expected.clone()),
                    Some(actual),
                ));
            }
        }
    }

    if let Some(expected) = expect.get("focusable") {
        let actual = observed.get("focusable").cloned().unwrap_or(Value::Null);
        if actual.is_null() {
            out.push(verdict_result(
                step_index,
                Some(part),
                "focusable".to_owned(),
                "vacuous",
                Some(expected.clone()),
                None,
            ));
        } else if actual == *expected {
            out.push(verdict_result(
                step_index,
                Some(part),
                "focusable".to_owned(),
                "pass",
                Some(expected.clone()),
                Some(actual),
            ));
        } else {
            out.push(verdict_result(
                step_index,
                Some(part),
                "focusable".to_owned(),
                "fail",
                Some(expected.clone()),
                Some(actual),
            ));
        }
    }

    if let Some(states) = expect.get("states").and_then(Value::as_object) {
        for (state, expected) in states {
            let observed_states = observed.get("states").unwrap_or(&Value::Null);
            let actual = observed_states.get(state).cloned();
            match actual {
                None => out.push(verdict_result(
                    step_index,
                    Some(part),
                    format!("state.{state}"),
                    "vacuous",
                    Some(expected.clone()),
                    None,
                )),
                Some(actual) if actual == *expected => out.push(verdict_result(
                    step_index,
                    Some(part),
                    format!("state.{state}"),
                    "pass",
                    Some(expected.clone()),
                    Some(actual),
                )),
                Some(actual) => out.push(verdict_result(
                    step_index,
                    Some(part),
                    format!("state.{state}"),
                    "fail",
                    Some(expected.clone()),
                    Some(actual),
                )),
            }
        }
    }

    if let Some(token_roles) = expect.get("tokenRoles").and_then(Value::as_object) {
        for (token, expected) in token_roles {
            out.push(verdict_result(
                step_index,
                Some(part),
                format!("tokenRole.{token}"),
                "vacuous",
                Some(expected.clone()),
                None,
            ));
        }
    }

    if let Some(geometry) = expect.get("geometry").and_then(Value::as_object) {
        let tolerance = geometry
            .get("tolerance")
            .and_then(Value::as_f64)
            .unwrap_or(0.5);
        for field in GEOMETRY_FIELDS {
            let Some(expected) = geometry.get(*field) else {
                continue;
            };
            let actual = observed
                .get("geometry")
                .and_then(|g| g.get(*field))
                .cloned()
                .unwrap_or(Value::Null);
            if actual.is_null() {
                out.push(verdict_result(
                    step_index,
                    Some(part),
                    format!("geometry.{field}"),
                    "vacuous",
                    Some(expected.clone()),
                    None,
                ));
            } else if numbers_match(expected, &actual, tolerance) {
                out.push(verdict_result(
                    step_index,
                    Some(part),
                    format!("geometry.{field}"),
                    "pass",
                    Some(expected.clone()),
                    Some(actual),
                ));
            } else {
                out.push(verdict_result(
                    step_index,
                    Some(part),
                    format!("geometry.{field}"),
                    "fail",
                    Some(expected.clone()),
                    Some(actual),
                ));
            }
        }
    }
}

/// A host-owned activation handler that mirrors the web toggle path:
/// `pressedChange` is emitted before `press`, matching the reference
/// (Svelte) order.
pub fn host_activate(
    toggle_mode: bool,
    pressed: &mut Option<bool>,
    trace: &mut Vec<String>,
) {
    if toggle_mode {
        let next = !pressed.unwrap_or(false);
        *pressed = Some(next);
        trace.push("pressedChange".to_owned());
    }
    trace.push("press".to_owned());
}
