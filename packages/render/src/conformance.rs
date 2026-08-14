//! Native conformance machinery (spec 066, architecture 009).
//!
//! Data-driven: the interface's part declarations carry resolution
//! descriptors and observation rules, and the observer walks the node tree
//! by those descriptors alone. No component identifier, part list, icon
//! name, class name, or component-specific tree branch lives in this module
//! — the same code serves every profile pilot.
//!
//! Verdicts are strict: every case assertion must be observable by the
//! runtime evaluating it. An expected field the runtime cannot observe is a
//! failure naming runtime, case, step, and field — never a silently
//! passable "vacuous".

use std::collections::BTreeMap;

use poodle_layout::LayoutSizing;
use poodle_node::{Node, NodeKind, NodeToggled};
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

// ── Interface document (parsed from the serialized interface JSON) ─────────

#[derive(Debug, Clone)]
pub enum NativeResolution {
    SelfNode,
    RootLabel,
    FirstText,
    /// Icons on one side of the label: leading icons sit before the first
    /// text child, trailing icons after it.
    IconSide { side: IconSide, except: Vec<String> },
    IconNamed { name: String },
}

#[derive(Debug, Clone, Copy)]
pub enum IconSide {
    Leading,
    Trailing,
}

#[derive(Debug, Clone)]
pub struct PartDecl {
    pub id: String,
    pub role: Option<String>,
    pub resolve: NativeResolution,
}

#[derive(Debug, Clone)]
pub enum NativeStateObservation {
    InteractionDisabled,
    PartPresent { part: String },
    A11yToggled,
    BackendFocus,
    FocusWithFocusStyle,
}

#[derive(Debug, Clone)]
pub struct StateDecl {
    pub name: String,
    pub observe: NativeStateObservation,
}

#[derive(Debug, Clone)]
pub struct TokenRoleDecl {
    pub name: String,
    #[allow(dead_code)]
    pub prop: String,
    #[allow(dead_code)]
    pub default: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InterfaceDoc {
    pub parts: Vec<PartDecl>,
    pub states: Vec<StateDecl>,
    pub token_roles: Vec<TokenRoleDecl>,
}

impl InterfaceDoc {
    /// Parses the serialized interface JSON (the neutral artifact the
    /// TypeScript authority emits).
    pub fn parse(interface: &Value) -> Result<Self, String> {
        let mut parts = Vec::new();
        for part in interface
            .get("parts")
            .and_then(Value::as_array)
            .ok_or("interface needs a parts array")?
        {
            let id = part
                .get("id")
                .and_then(Value::as_str)
                .ok_or("part needs an id")?
                .to_owned();
            let role = part.get("role").and_then(Value::as_str).map(str::to_owned);
            let native = part
                .get("resolve")
                .and_then(|r| r.get("native"))
                .ok_or_else(|| format!("part '{id}' needs a native resolution"))?;
            let kind = native.get("kind").and_then(Value::as_str).unwrap_or("");
            let resolve = match kind {
                "self" => NativeResolution::SelfNode,
                "root-label" => NativeResolution::RootLabel,
                "first-text" => NativeResolution::FirstText,
                "icon-side" => NativeResolution::IconSide {
                    side: match native.get("side").and_then(Value::as_str) {
                        Some("trailing") => IconSide::Trailing,
                        _ => IconSide::Leading,
                    },
                    except: native
                        .get("except")
                        .and_then(Value::as_array)
                        .map(|items| {
                            items
                                .iter()
                                .filter_map(Value::as_str)
                                .map(str::to_owned)
                                .collect()
                        })
                        .unwrap_or_default(),
                },
                "icon-named" => NativeResolution::IconNamed {
                    name: native
                        .get("name")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_owned(),
                },
                other => return Err(format!("part '{id}' has unknown native resolution '{other}'")),
            };
            parts.push(PartDecl { id, role, resolve });
        }

        let mut states = Vec::new();
        for state in interface
            .get("states")
            .and_then(Value::as_array)
            .ok_or("interface needs a states array")?
        {
            let name = state
                .get("name")
                .and_then(Value::as_str)
                .ok_or("state needs a name")?
                .to_owned();
            let observe = match state.get("native").and_then(Value::as_str).unwrap_or("") {
                "interaction-disabled" => NativeStateObservation::InteractionDisabled,
                "part-present" => NativeStateObservation::PartPresent {
                    part: state
                        .get("part")
                        .and_then(Value::as_str)
                        .ok_or_else(|| format!("state '{name}' needs a part"))?
                        .to_owned(),
                },
                "a11y-toggled" => NativeStateObservation::A11yToggled,
                "backend-focus" => NativeStateObservation::BackendFocus,
                "focus-with-focus-style" => NativeStateObservation::FocusWithFocusStyle,
                other => return Err(format!("state '{name}' has unknown native observation '{other}'")),
            };
            states.push(StateDecl { name, observe });
        }

        let token_roles = interface
            .get("tokenRoles")
            .and_then(Value::as_array)
            .map(|roles| {
                roles
                    .iter()
                    .map(|role| TokenRoleDecl {
                        name: role
                            .get("name")
                            .and_then(Value::as_str)
                            .unwrap_or("")
                            .to_owned(),
                        prop: role
                            .get("prop")
                            .and_then(Value::as_str)
                            .unwrap_or("")
                            .to_owned(),
                        default: role
                            .get("default")
                            .and_then(Value::as_str)
                            .map(str::to_owned),
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(Self {
            parts,
            states,
            token_roles,
        })
    }

    pub fn part_decl(&self, id: &str) -> Option<&PartDecl> {
        self.parts.iter().find(|p| p.id == id)
    }
}

// ── Part resolution (data-driven) ──────────────────────────────────────────

/// Resolves a stable part in the node tree by the interface's descriptor.
pub fn find_part<'a>(iface: &InterfaceDoc, root: &'a Node, part_id: &str) -> Option<&'a Node> {
    let decl = iface.part_decl(part_id)?;
    match &decl.resolve {
        NativeResolution::SelfNode => Some(root),
        NativeResolution::RootLabel => label_text(root).map(|_| root),
        NativeResolution::FirstText => first_text(root),
        NativeResolution::IconSide { side, except } => {
            let is_content_icon = |child: &&Node| {
                matches!(&child.kind, NodeKind::Icon { name, .. } if !except.contains(name))
            };
            let label_index = root
                .children
                .iter()
                .position(|child| matches!(child.kind, NodeKind::Text { .. }));
            let children: Vec<&Node> = root.children.iter().collect();
            match side {
                IconSide::Leading => {
                    let end = label_index.unwrap_or(children.len());
                    children[..end].iter().rev().find(|child| is_content_icon(child)).copied()
                }
                IconSide::Trailing => {
                    let start = label_index.map(|i| i + 1).unwrap_or(0);
                    children[start..].iter().find(|child| is_content_icon(child)).copied()
                }
            }
        }
        NativeResolution::IconNamed { name } => root.children.iter().find(|child| {
            matches!(&child.kind, NodeKind::Icon { name: n, .. } if n == name)
        }),
    }
}

fn first_text(root: &Node) -> Option<&Node> {
    root.children
        .iter()
        .find(|child| matches!(child.kind, NodeKind::Text { .. }))
}

/// The visible label text: the root's intrinsic label or its first Text
/// child.
/// The visible label text: the root's first Text child, or the text the
/// root intrinsically carries (the vocabulary's own accessor — no kind
/// branch in observer code).
fn label_text(root: &Node) -> Option<String> {
    first_text(root)
        .map(|node| match &node.kind {
            NodeKind::Text { content } => content.clone(),
            _ => String::new(),
        })
        .or_else(|| root.intrinsic_text().map(str::to_owned))
}

/// The renderer declares roles on `a11y`; the observer never infers a
/// role from a node kind. Removing the renderer's declaration makes the
/// role observation `null` and the owning assertion fails — never a
/// silent fallback.
fn role_of(node: &Node) -> Option<String> {
    node.a11y
        .role
        .as_ref()
        .map(|role| format!("{role:?}").to_ascii_lowercase())
}

fn rgba(color: &poodle_node::ColorValue) -> String {
    format!("rgba({},{},{},{})", color.0, color.1, color.2, color.3)
}

// ── Observation ────────────────────────────────────────────────────────────

/// Observes one part into the observation JSON.
fn observe_part(
    decl: &PartDecl,
    node: &Node,
    root: &Node,
    iface: &InterfaceDoc,
    backend_focus: Option<bool>,
) -> Value {
    let part_id = decl.id.as_str();
    let is_root = part_id == "root";
    let (geometry, channels) = if is_root {
        (geometry_of(node), channels_of(node))
    } else {
        (json!({}), json!({}))
    };
    let states = if is_root {
        states_of(iface, root, backend_focus)
    } else {
        json!({})
    };
    let token_roles = if is_root {
        token_roles_of(iface, node)
    } else {
        json!({})
    };
    let is_root_label = matches!(decl.resolve, NativeResolution::RootLabel);
    let text = if is_root_label {
        label_text(root)
    } else {
        text_of(node)
    };
    json!({
        "present": true,
        // Only root carries role, name, and interactivity; other parts are
        // carriers (text/icon). The label part normalizes to those carrier
        // semantics on every runtime.
        "role": if is_root { role_of(node).map(|r| json!(r)).unwrap_or(Value::Null) } else { Value::Null },
        "name": if is_root { name_of(node, root).map(|n| json!(n)).unwrap_or(Value::Null) } else { Value::Null },
        "text": if is_root { Value::Null } else { text.map(|t| json!(t)).unwrap_or(Value::Null) },
        "icon": icon_of(node),
        "states": states,
        "tokenRoles": token_roles,
        "focusable": if is_root { json!(node.interaction.focusable && !node.interaction.disabled) } else { Value::Null },
        "focused": if is_root { backend_focus.map(|f| json!(f)).unwrap_or(Value::Null) } else { Value::Null },
        "focusVisible": if is_root { focus_visible_of(node, backend_focus) } else { Value::Null },
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
        "focusable": null,
        "focused": null,
        "focusVisible": null,
        "geometry": {},
        "channels": {},
    })
}

fn states_of(iface: &InterfaceDoc, root: &Node, backend_focus: Option<bool>) -> Value {
    let mut states = BTreeMap::new();
    for decl in &iface.states {
        let value = match &decl.observe {
            NativeStateObservation::InteractionDisabled => json!(root.interaction.disabled),
            NativeStateObservation::PartPresent { part } => {
                json!(find_part(iface, root, part).is_some())
            }
            NativeStateObservation::A11yToggled => {
                json!(matches!(root.a11y.toggled, Some(NodeToggled::True)))
            }
            NativeStateObservation::BackendFocus => match backend_focus {
                Some(focused) => json!(focused),
                None => Value::Null,
            },
            NativeStateObservation::FocusWithFocusStyle => focus_visible_of(root, backend_focus),
        };
        states.insert(decl.name.clone(), value);
    }
    serde_json::to_value(states).expect("states serialize")
}

fn focus_visible_of(node: &Node, backend_focus: Option<bool>) -> Value {
    match backend_focus {
        Some(true) => json!(node.style.focus.is_some()),
        Some(false) => json!(false),
        None => Value::Null,
    }
}

fn token_roles_of(iface: &InterfaceDoc, node: &Node) -> Value {
    let mut roles = BTreeMap::new();
    for decl in &iface.token_roles {
        let value = node.roles.get(&decl.name).cloned();
        roles.insert(
            decl.name.clone(),
            value.map(Value::String).unwrap_or(Value::Null),
        );
    }
    serde_json::to_value(roles).expect("roles serialize")
}

fn name_of(node: &Node, root: &Node) -> Option<String> {
    node.a11y.label.clone().or_else(|| label_text(root))
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
    geometry.insert("radius".to_owned(), json!(descriptor.corner_radii.top_left));
    geometry.insert("borderWidth".to_owned(), json!(descriptor.border.width));
    serde_json::to_value(geometry).expect("geometry serializes")
}

fn channels_of(node: &Node) -> Value {
    let descriptor = &node.style.descriptor;
    let mut channels = BTreeMap::new();
    if let Some(color) = descriptor.background {
        channels.insert("background".to_owned(), json!(rgba(&color)));
    }
    channels.insert("borderColor".to_owned(), json!(rgba(&descriptor.border.color)));
    if let Some(color) = descriptor.text_color {
        channels.insert("color".to_owned(), json!(rgba(&color)));
    }
    channels.insert("opacity".to_owned(), json!(descriptor.opacity));
    serde_json::to_value(channels).expect("channels serialize")
}

/// Observes the whole tree into `component-observation.v1` shape.
pub fn observe_tree(
    runtime: &str,
    component: &str,
    iface: &InterfaceDoc,
    root: &Node,
    backend_focus: Option<bool>,
) -> Value {
    let mut parts = BTreeMap::new();
    for decl in &iface.parts {
        let observed = find_part(iface, root, &decl.id);
        parts.insert(
            decl.id.clone(),
            match observed {
                Some(node) => observe_part(decl, node, root, iface, backend_focus),
                None => absent_part(),
            },
        );
    }
    json!({
        "runtime": runtime,
        "component": component,
        "parts": parts,
        "trace": [],
    })
}

// ── Evaluation (strict verdicts) ───────────────────────────────────────────

/// One assertion result, serialized like the web runner's.
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
    /// Why the runtime could not observe a required field (verdict "fail").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

fn result(
    step_index: usize,
    part: Option<&str>,
    field: String,
    verdict: &'static str,
    expected: Option<Value>,
    actual: Option<Value>,
    reason: Option<String>,
) -> AssertionResult {
    AssertionResult {
        step_index,
        part: part.map(str::to_owned),
        field,
        verdict,
        expected,
        actual,
        reason,
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

/// Evaluates one case's steps against the harness. Steps are the raw
/// serialized step objects from the corpus.
pub fn evaluate_steps(
    iface: &InterfaceDoc,
    steps: &[Value],
    harness: &mut dyn NativeHarness,
) -> Vec<AssertionResult> {
    let mut out = Vec::new();
    for (index, step) in steps.iter().enumerate() {
        let kind = step.get("kind").and_then(Value::as_str).unwrap_or_default();
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
                    other => out.push(result(
                        index,
                        Some(part),
                        format!("action.{other}"),
                        "fail",
                        None,
                        None,
                        None,
                    )),
                }
            }
            "expectPart" => {
                let part = step.get("part").and_then(Value::as_str).unwrap_or("");
                let expect = step.get("expect").cloned().unwrap_or(Value::Null);
                assert_part(iface, part, &expect, index, harness.observe(), harness.runtime(), &mut out);
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
                assert_events(&expected, &actual, index, &mut out);
            }
            other => out.push(result(
                index,
                None,
                format!("step.{other}"),
                "fail",
                None,
                None,
                None,
            )),
        }
    }
    out
}

/// Strict event-order comparison.
pub fn assert_events(
    expected: &[String],
    actual: &[String],
    step_index: usize,
    out: &mut Vec<AssertionResult>,
) {
    let pass = expected == actual;
    out.push(result(
        step_index,
        None,
        "events".to_owned(),
        if pass { "pass" } else { "fail" },
        Some(json!(expected)),
        Some(json!(actual)),
        None,
    ));
}

/// Strict compare: an expected field the runtime cannot observe is a failure.
fn check(
    out: &mut Vec<AssertionResult>,
    runtime: &str,
    index: usize,
    part: &str,
    field: String,
    expected: &Value,
    actual: Option<&Value>,
    tolerance: Option<f64>,
) {
    let Some(actual) = actual else {
        out.push(result(
            index,
            Some(part),
            field,
            "fail",
            Some(expected.clone()),
            None,
            Some(format!("not observed by {runtime}")),
        ));
        return;
    };
    if actual.is_null() {
        out.push(result(
            index,
            Some(part),
            field,
            "fail",
            Some(expected.clone()),
            None,
            Some(format!("not observed by {runtime}")),
        ));
        return;
    }
    let matches = match tolerance {
        Some(tol) => match (expected.as_f64(), actual.as_f64()) {
            (Some(e), Some(a)) => (e - a).abs() <= tol,
            _ => expected == actual,
        },
        None => expected == actual,
    };
    out.push(result(
        index,
        Some(part),
        field,
        if matches { "pass" } else { "fail" },
        Some(expected.clone()),
        Some(actual.clone()),
        None,
    ));
}

pub fn assert_part(
    _iface: &InterfaceDoc,
    part: &str,
    expect: &Value,
    step_index: usize,
    observation: Value,
    runtime: &str,
    out: &mut Vec<AssertionResult>,
) {
    let observed = observation
        .get("parts")
        .and_then(|parts| parts.get(part))
        .cloned()
        .unwrap_or(Value::Null);

    if observed.is_null() {
        out.push(result(
            step_index,
            Some(part),
            "present".to_owned(),
            "fail",
            Some(json!(true)),
            None,
            Some(format!("not observed by {runtime}")),
        ));
        return;
    }

    let present = observed.get("present").and_then(Value::as_bool).unwrap_or(false);
    let expected_present = expect.get("present").and_then(Value::as_bool).unwrap_or(true);
    if !present && expected_present {
        out.push(result(
            step_index,
            Some(part),
            "present".to_owned(),
            "fail",
            Some(json!(true)),
            Some(json!(false)),
            None,
        ));
        return;
    }

    if let Some(expected) = expect.get("present") {
        check(out, runtime, step_index, part, "present".to_owned(), expected, Some(&json!(present)), None);
    }
    for field in ["role", "name", "text", "icon", "focusable"] {
        if let Some(expected) = expect.get(field) {
            let actual = observed.get(field);
            check(out, runtime, step_index, part, field.to_owned(), expected, actual, None);
        }
    }

    if let Some(states) = expect.get("states").and_then(Value::as_object) {
        let observed_states = observed.get("states").cloned().unwrap_or(Value::Null);
        for (state, expected) in states {
            let actual = observed_states
                .as_object()
                .and_then(|s| s.get(state));
            check(
                out,
                runtime,
                step_index,
                part,
                format!("state.{state}"),
                expected,
                actual,
                None,
            );
        }
    }

    if let Some(token_roles) = expect.get("tokenRoles").and_then(Value::as_object) {
        let observed_roles = observed.get("tokenRoles").cloned().unwrap_or(Value::Null);
        for (token, expected) in token_roles {
            let actual = observed_roles
                .as_object()
                .and_then(|r| r.get(token));
            check(
                out,
                runtime,
                step_index,
                part,
                format!("tokenRole.{token}"),
                expected,
                actual,
                None,
            );
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
                .and_then(|g| g.get(*field));
            check(
                out,
                runtime,
                step_index,
                part,
                format!("geometry.{field}"),
                expected,
                actual,
                Some(tolerance),
            );
        }
    }

}

/// A host-owned activation handler that mirrors the web toggle path:
/// `pressedChange` is emitted before `press`, matching the reference
/// (Svelte) order, with the same payload the web pair records.
pub fn host_activate(toggle_mode: bool, pressed: &mut Option<bool>, trace: &mut Vec<Value>) {
    if toggle_mode {
        let next = !pressed.unwrap_or(false);
        *pressed = Some(next);
        trace.push(json!({ "event": "pressedChange", "payload": { "pressed": next } }));
    }
    trace.push(json!({ "event": "press" }));
}
