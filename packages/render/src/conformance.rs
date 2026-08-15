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

/// Geometry fields, mirroring the web observer's field set. The relative
/// logical-bounds fields (gaps against a part's `relativeTo` anchor) are
/// computed from the two parts' rendered bounds when both resolve.
const GEOMETRY_FIELDS: &[&str] = &[
    "height",
    "minWidth",
    "paddingLeft",
    "paddingRight",
    "radius",
    "borderWidth",
    "topGap",
    "bottomGap",
    "leftGap",
    "rightGap",
    "hStart",
    "hEnd",
    "vStart",
    "widthGap",
];

// ── Interface document (parsed from the serialized interface JSON) ─────────

#[derive(Debug, Clone)]
pub enum NativeResolution {
    SelfNode,
    Id {
        id: String,
    },
    IdTemplate {
        template: String,
    },
    RootLabel,
    FirstText,
    /// Icons on one side of the label: leading icons sit before the first
    /// text child, trailing icons after it.
    IconSide {
        side: IconSide,
        except: Vec<String>,
    },
    IconNamed {
        name: String,
    },
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
    pub repeated: bool,
    /// The part this part is positioned relative to (an anchored surface's
    /// trigger). Relative logical-bounds gaps are computed against it.
    pub relative_to: Option<String>,
}

#[derive(Debug, Clone)]
pub enum NativeStateObservation {
    InteractionDisabled,
    PartPresent { part: String },
    A11yToggled,
    BackendFocus,
    FocusWithFocusStyle,
    PartInteractionDisabled { part: String },
    PartBackendFocus { part: String },
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
                "id" => NativeResolution::Id {
                    id: native
                        .get("id")
                        .and_then(Value::as_str)
                        .ok_or_else(|| format!("part '{id}' id resolution needs an id"))?
                        .to_owned(),
                },
                "id-template" => NativeResolution::IdTemplate {
                    template: native
                        .get("template")
                        .and_then(Value::as_str)
                        .ok_or_else(|| {
                            format!("part '{id}' id-template resolution needs a template")
                        })?
                        .to_owned(),
                },
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
                other => {
                    return Err(format!(
                        "part '{id}' has unknown native resolution '{other}'"
                    ))
                }
            };
            parts.push(PartDecl {
                id,
                role,
                resolve,
                repeated: part.get("repeat").is_some(),
                relative_to: part.get("relativeTo").and_then(Value::as_str).map(str::to_owned),
            });
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
                "part-interaction-disabled" => NativeStateObservation::PartInteractionDisabled {
                    part: state
                        .get("part")
                        .and_then(Value::as_str)
                        .ok_or_else(|| format!("state '{name}' needs a part"))?
                        .to_owned(),
                },
                "part-backend-focus" => NativeStateObservation::PartBackendFocus {
                    part: state
                        .get("part")
                        .and_then(Value::as_str)
                        .ok_or_else(|| format!("state '{name}' needs a part"))?
                        .to_owned(),
                },
                other => {
                    return Err(format!(
                        "state '{name}' has unknown native observation '{other}'"
                    ))
                }
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
        let base = id.split_once(':').map(|(base, _)| base).unwrap_or(id);
        self.parts.iter().find(|p| p.id == base)
    }
}

// ── Part resolution (data-driven) ──────────────────────────────────────────

/// True when another `self`-resolved part is this same node. Native
/// runtimes collapse wrapper chrome onto the control; identity then belongs
/// to the named part, not to `root`. Label/`root-label` sharing the root is
/// not collapse — that part reads text off the root on purpose.
fn collapsed_chrome_root(iface: &InterfaceDoc, part_id: &str, node: &Node, root: &Node) -> bool {
    part_id == "root"
        && iface.parts.iter().any(|decl| {
            decl.id != "root"
                && matches!(decl.resolve, NativeResolution::SelfNode)
                && std::ptr::eq(node, root)
        })
}

/// Resolves a stable part in the node tree by the interface's descriptor.
pub fn find_part<'a>(iface: &InterfaceDoc, root: &'a Node, part_id: &str) -> Option<&'a Node> {
    let decl = iface.part_decl(part_id)?;
    match &decl.resolve {
        NativeResolution::SelfNode => Some(root),
        NativeResolution::Id { id } => root.find(&|node| node.id.as_deref() == Some(id.as_str())),
        NativeResolution::IdTemplate { template } => {
            let key = part_id.split_once(':')?.1;
            let id = template.replace("{key}", key);
            root.find(&|node| node.id.as_deref() == Some(id.as_str()))
        }
        NativeResolution::RootLabel => label_text(root).map(|_| root),
        NativeResolution::FirstText => first_text(root),
        NativeResolution::IconSide { side, except } => {
            let is_content_icon = |child: &&Node| matches!(&child.kind, NodeKind::Icon { name, .. } if !except.contains(name));
            let label_index = root
                .children
                .iter()
                .position(|child| matches!(child.kind, NodeKind::Text { .. }));
            let children: Vec<&Node> = root.children.iter().collect();
            match side {
                IconSide::Leading => {
                    let end = label_index.unwrap_or(children.len());
                    children[..end]
                        .iter()
                        .rev()
                        .find(|child| is_content_icon(child))
                        .copied()
                }
                IconSide::Trailing => {
                    let start = label_index.map(|i| i + 1).unwrap_or(0);
                    children[start..]
                        .iter()
                        .find(|child| is_content_icon(child))
                        .copied()
                }
            }
        }
        NativeResolution::IconNamed { name } => root
            .children
            .iter()
            .find(|child| matches!(&child.kind, NodeKind::Icon { name: n, .. } if n == name)),
    }
}

fn first_text(root: &Node) -> Option<&Node> {
    root.children
        .iter()
        .find(|child| matches!(child.kind, NodeKind::Text { .. }))
}

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
    match node.a11y.role {
        Some(poodle_node::NodeRole::TextInput) => Some("textbox".to_owned()),
        Some(role) => Some(format!("{role:?}").to_ascii_lowercase()),
        None => None,
    }
}

fn rgba(color: &poodle_node::ColorValue) -> String {
    format!("rgba({},{},{},{})", color.0, color.1, color.2, color.3)
}

/// Runtime relationships carry real node ids. Normalize id-template targets
/// back to the corpus's semantic repeated-part ids before comparison; plain
/// id-resolved parts match their declared id or its instance-scoped suffix
/// (runtime ids read `{instance}:{semantic}`).
fn normalize_relationship(value: &str, iface: &InterfaceDoc) -> String {
    for part in &iface.parts {
        match &part.resolve {
            NativeResolution::IdTemplate { template } => {
                let Some((prefix, suffix)) = template.split_once("{key}") else {
                    continue;
                };
                let Some(key) = value
                    .strip_prefix(prefix)
                    .and_then(|tail| tail.strip_suffix(suffix))
                else {
                    continue;
                };
                return format!("{}:{key}", part.id);
            }
            NativeResolution::Id { id } => {
                if value == id || value.ends_with(&format!(":{id}")) {
                    return part.id.clone();
                }
            }
            _ => {}
        }
    }
    value.to_owned()
}

// ── Observation ────────────────────────────────────────────────────────────

fn runtime_identity(node: &Node) -> &str {
    node.runtime_id
        .as_deref()
        .or(node.id.as_deref())
        .unwrap_or("")
}

/// Observes one part into the observation JSON.
fn observe_part(
    part_id: &str,
    decl: &PartDecl,
    node: &Node,
    root: &Node,
    iface: &InterfaceDoc,
    focus_by_id: &dyn Fn(&str) -> Option<bool>,
    bounds_by_id: &dyn Fn(&str) -> Option<(f32, f32, f32, f32)>,
) -> Value {
    let is_root = part_id == "root";
    // Collapsed native chrome: root and another identity part are the same
    // node. Keep wrapper shape on root (present/states/tokenRoles, not the
    // tab stop); the named part carries role/value/focusable.
    let chrome_root = collapsed_chrome_root(iface, part_id, node, root);
    let mut geometry = geometry_of(node);
    if decl.relative_to.is_some() {
        let relative = relative_bounds_of(part_id, iface, root, node, bounds_by_id);
        if let (Some(geometry_obj), Some(relative)) = (geometry.as_object_mut(), relative.as_object())
        {
            for (field, value) in relative {
                geometry_obj.insert(field.clone(), value.clone());
            }
        }
    }
    let channels = channels_of(node);
    let states = if is_root {
        states_of(iface, root, focus_by_id)
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
    let identity_part = is_root
        || decl.role.is_some()
        || matches!(
            decl.resolve,
            NativeResolution::Id { .. }
                | NativeResolution::IdTemplate { .. }
                | NativeResolution::SelfNode
        );
    let report_identity = identity_part && !chrome_root;
    let focused = if identity_part {
        // Disabled controls are not focusable; report unfocused even if the
        // harness's pointer pressed them (browser semantics, and the same
        // rule the web observer applies).
        if !node.interaction.focusable || node.interaction.disabled {
            Some(false)
        } else {
            Some(
                focus_by_id(runtime_identity(node))
                    .or_else(|| if is_root { focus_by_id("") } else { None })
                    .unwrap_or(false),
            )
        }
    } else {
        None
    };
    json!({
        "present": true,
        "role": if report_identity {
            role_of(node).map(|r| json!(r)).unwrap_or(Value::Null)
        } else {
            Value::Null
        },
        "name": if report_identity && (!is_root || decl.role.is_some()) {
            node.a11y
                .label
                .clone()
                .or_else(|| if is_root { name_of(node, root) } else { None })
                .filter(|s| !s.is_empty())
                .map(Value::String)
                .unwrap_or(Value::Null)
        } else {
            Value::Null
        },
        "text": if is_root { Value::Null } else { text.map(|t| json!(t)).unwrap_or(Value::Null) },
        "icon": icon_of(node),
        "value": if report_identity {
            match &node.kind {
                NodeKind::Input { value, .. } => json!(value),
                _ => node.a11y.value.map(|v| json!(v)).unwrap_or(Value::Null),
            }
        } else {
            Value::Null
        },
        "selectionStart": if chrome_root {
            Value::Null
        } else {
            node.caret.map(|c| json!(c.selection.0)).unwrap_or(Value::Null)
        },
        "selectionEnd": if chrome_root {
            Value::Null
        } else {
            node.caret.map(|c| json!(c.selection.1)).unwrap_or(Value::Null)
        },
        "states": states,
        "tokenRoles": token_roles,
        "expanded": node.a11y.expanded.map(Value::Bool).unwrap_or(Value::Null),
        "overlay": json!(node.style.overlay),
        "parent": parent_part_of(part_id, iface, root, node)
            .map(Value::String)
            .unwrap_or(Value::Null),
        "focusable": if chrome_root {
            json!(false)
        } else if identity_part {
            json!(node.interaction.focusable && !node.interaction.disabled)
        } else {
            Value::Null
        },
        "focused": focused.map(|f| json!(f)).unwrap_or(Value::Null),
        "focusVisible": if chrome_root {
            json!(false)
        } else if identity_part {
            focus_visible_of(node, focused)
        } else {
            Value::Null
        },
        "tabbable": if chrome_root {
            json!(false)
        } else if identity_part {
            json!(node.interaction.focusable && !node.interaction.disabled && node.a11y.tab_index.unwrap_or(0) == 0)
        } else {
            Value::Null
        },
        "selected": node.a11y.selected.map(|value| json!(value)).unwrap_or(Value::Null),
        "orientation": node.a11y.orientation.clone().map(Value::String).unwrap_or(Value::Null),
        "controls": node.a11y.controls.as_deref().map(|value| Value::String(normalize_relationship(value, iface))).unwrap_or(Value::Null),
        "labelledBy": node.a11y.labelled_by.as_deref().map(|value| Value::String(normalize_relationship(value, iface))).unwrap_or(Value::Null),
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
        "value": null,
        "selectionStart": null,
        "selectionEnd": null,
        "states": {},
        "tokenRoles": {},
        "expanded": null,
        "overlay": null,
        "parent": null,
        "focusable": null,
        "focused": null,
        "focusVisible": null,
        "tabbable": null,
        "selected": null,
        "orientation": null,
        "controls": null,
        "labelledBy": null,
        "geometry": {},
        "channels": {},
    })
}

fn states_of(
    iface: &InterfaceDoc,
    root: &Node,
    focus_by_id: &dyn Fn(&str) -> Option<bool>,
) -> Value {
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
            NativeStateObservation::BackendFocus => {
                match focus_by_id(runtime_identity(root)) {
                    Some(focused) => json!(focused),
                    None => Value::Null,
                }
            }
            NativeStateObservation::FocusWithFocusStyle => {
                focus_visible_of(root, focus_by_id(runtime_identity(root)))
            }
            NativeStateObservation::PartInteractionDisabled { part } => {
                match find_part(iface, root, part) {
                    Some(node) => json!(node.interaction.disabled),
                    None => Value::Null,
                }
            }
            NativeStateObservation::PartBackendFocus { part } => {
                match find_part(iface, root, part) {
                    Some(node) if !node.interaction.focusable || node.interaction.disabled => {
                        json!(false)
                    }
                    Some(node) => match focus_by_id(runtime_identity(node)) {
                        Some(focused) => json!(focused),
                        None => json!(false),
                    },
                    None => Value::Null,
                }
            }
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
    geometry.insert(
        "minWidth".to_owned(),
        json!(node.style.min_width.unwrap_or(0.0)),
    );
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

/// Relative logical bounds of a part against its `relativeTo` anchor, from
/// the two parts' rendered bounds: the gaps between facing edges (positive
/// when the part is past the anchor in that direction), the start/end
/// alignment deltas, and the width difference. Rendered bounds come from the
/// runtime (the GPUI backend records them at paint); the computation itself
/// is renderer-neutral.
fn relative_bounds_of(
    part_id: &str,
    iface: &InterfaceDoc,
    root: &Node,
    part_node: &Node,
    bounds_by_id: &dyn Fn(&str) -> Option<(f32, f32, f32, f32)>,
) -> Value {
    let Some(anchor) = iface.part_decl(part_id).and_then(|decl| decl.relative_to.clone()) else {
        return Value::Null;
    };
    let Some(anchor_node) = find_part(iface, root, &anchor) else {
        return Value::Null;
    };
    let Some((at, al, aw, ah)) = bounds_by_id(runtime_identity(anchor_node)) else {
        return Value::Null;
    };
    let Some((pt, pl, pw, ph)) = bounds_by_id(runtime_identity(part_node)) else {
        return Value::Null;
    };
    json!({
        "topGap": pt - (at + ah),
        "bottomGap": at - (pt + ph),
        "leftGap": pl - (al + aw),
        "rightGap": al - (pl + pw),
        "hStart": pl - al,
        "hEnd": (al + aw) - (pl + pw),
        "vStart": pt - at,
        "widthGap": pw - aw,
    })
}

/// The part that hosts this part's layer: the nearest part node above the
/// target in the tree. The root part has none; a non-root part that resolves
/// to the root's own node (root-label etc.) is hosted by the root part, like
/// its web counterpart which lives inside the root element.
fn parent_part_of(part_id: &str, iface: &InterfaceDoc, root: &Node, target: &Node) -> Option<String> {
    if part_id == "root" {
        return None;
    }
    if std::ptr::eq(root, target) {
        return Some("root".to_owned());
    }
    let part_nodes: Vec<(&Node, String)> = iface
        .parts
        .iter()
        .filter_map(|decl| Some((find_part(iface, root, &decl.id)?, decl.id.clone())))
        .collect();
    fn walk<'a>(
        node: &'a Node,
        target: &'a Node,
        part_nodes: &[(&'a Node, String)],
        current: Option<&str>,
    ) -> Option<String> {
        if std::ptr::eq(node, target) {
            return current.map(str::to_owned);
        }
        let entered = part_nodes
            .iter()
            .find(|(part_node, _)| std::ptr::eq(*part_node, node))
            .map(|(_, id)| id.as_str())
            .or(current);
        for child in &node.children {
            if let Some(found) = walk(child, target, part_nodes, entered) {
                return Some(found);
            }
        }
        None
    }
    walk(root, target, &part_nodes, None)
}

/// The subtree text of the first node the runtime reports focused — the
/// focused-part progression's identity channel, mirroring the web observer's
/// `document.activeElement.textContent`.
fn focused_text_of(root: &Node, focus_by_id: &dyn Fn(&str) -> Option<bool>) -> Option<String> {
    fn text_of_focused(node: &Node, focus_by_id: &dyn Fn(&str) -> Option<bool>) -> Option<String> {
        if focus_by_id(runtime_identity(node)) == Some(true)
            && node.interaction.focusable
            && !node.interaction.disabled
        {
            // Web `<input>`/`<textarea>` textContent is empty; value lives on
            // the value channel, not focusedText.
            if matches!(node.kind, NodeKind::Input { .. }) {
                return None;
            }
            let joined = node.texts().join("");
            // The web observer reports null for a focus target without
            // readable text (empty textContent); mirror that.
            return if joined.is_empty() { None } else { Some(joined) };
        }
        for child in &node.children {
            if let Some(text) = text_of_focused(child, focus_by_id) {
                return Some(text);
            }
        }
        None
    }
    text_of_focused(root, focus_by_id)
}

/// Observes the whole tree into `component-observation.v1` shape.
pub fn observe_tree(
    runtime: &str,
    component: &str,
    iface: &InterfaceDoc,
    root: &Node,
    backend_focus: Option<bool>,
) -> Value {
    let root_id = root.id.clone();
    let focus_by_id = move |id: &str| -> Option<bool> {
        if id.is_empty() || root_id.as_deref() == Some(id) {
            backend_focus
        } else {
            None
        }
    };
    observe_tree_with_focus(runtime, component, iface, root, &focus_by_id)
}

/// Observes with an explicit per-element focus lookup (GPUI runner).
pub fn observe_tree_with_focus(
    runtime: &str,
    component: &str,
    iface: &InterfaceDoc,
    root: &Node,
    focus_by_id: &dyn Fn(&str) -> Option<bool>,
) -> Value {
    observe_tree_with_context(runtime, component, iface, root, &ObserveContext {
        focus_by_id,
        // No overlay layers are open in a bare tree; the web side always
        // reports a number, so the plain observer reports zero.
        layer_count: &|| Some(0),
        bounds_by_id: &|_| None,
    })
}

/// Extra runtime channels the overlay profile reads: the open layer count
/// (layer order / overlay state) and rendered element bounds (relative
/// logical bounds). Supplied by the runtime; the computation stays neutral.
pub struct ObserveContext<'a> {
    pub focus_by_id: &'a dyn Fn(&str) -> Option<bool>,
    pub layer_count: &'a dyn Fn() -> Option<usize>,
    pub bounds_by_id: &'a dyn Fn(&str) -> Option<(f32, f32, f32, f32)>,
}

/// Observes with the full runtime context (focus, layers, rendered bounds).
pub fn observe_tree_with_context(
    runtime: &str,
    component: &str,
    iface: &InterfaceDoc,
    root: &Node,
    context: &ObserveContext<'_>,
) -> Value {
    let mut parts = BTreeMap::new();
    for decl in &iface.parts {
        let part_ids = expanded_native_part_ids(decl, root);
        for part_id in part_ids {
            let observed = find_part(iface, root, &part_id);
            parts.insert(
                part_id.clone(),
                match observed {
                    Some(node) => observe_part(
                        &part_id,
                        decl,
                        node,
                        root,
                        iface,
                        context.focus_by_id,
                        context.bounds_by_id,
                    ),
                    None => absent_part(),
                },
            );
        }
    }
    if let (Some(lower), Some(upper)) = (
        parts
            .get("lower")
            .and_then(|p| p.get("value").and_then(Value::as_f64)),
        parts
            .get("upper")
            .and_then(|p| p.get("value").and_then(Value::as_f64)),
    ) {
        if let Some(root_part) = parts.get_mut("root") {
            if let Some(obj) = root_part.as_object_mut() {
                obj.insert("value".to_owned(), json!([lower, upper]));
            }
        }
    }
    if let Some(root_part) = parts.get_mut("root").and_then(|p| p.as_object_mut()) {
        root_part.insert(
            "focusedText".to_owned(),
            focused_text_of(root, context.focus_by_id)
                .map(Value::String)
                .unwrap_or(Value::Null),
        );
        root_part.insert(
            "layerCount".to_owned(),
            match (context.layer_count)() {
                Some(count) => json!(count),
                None => Value::Null,
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

fn expanded_native_part_ids(decl: &PartDecl, root: &Node) -> Vec<String> {
    if !decl.repeated {
        return vec![decl.id.clone()];
    }
    let NativeResolution::IdTemplate { template } = &decl.resolve else {
        return Vec::new();
    };
    let Some((prefix, suffix)) = template.split_once("{key}") else {
        return Vec::new();
    };
    fn visit(node: &Node, prefix: &str, suffix: &str, out: &mut Vec<String>) {
        if let Some(id) = node.id.as_deref() {
            if let Some(rest) = id.strip_prefix(prefix) {
                if let Some(key) = rest.strip_suffix(suffix) {
                    if !key.is_empty() {
                        out.push(key.to_owned());
                    }
                }
            }
        }
        for child in &node.children {
            visit(child, prefix, suffix, out);
        }
    }
    let mut keys = Vec::new();
    visit(root, prefix, suffix, &mut keys);
    keys.sort();
    keys.dedup();
    keys.into_iter()
        .map(|key| format!("{}:{key}", decl.id))
        .collect()
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
                assert_part(
                    iface,
                    part,
                    &expect,
                    index,
                    harness.observe(),
                    harness.runtime(),
                    &mut out,
                );
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
    let matches = values_match(expected, actual, tolerance);
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

/// Numeric-aware equality: case JSON often stores integers while a11y values
/// are f64, and serde_json's Number PartialEq treats those as unequal.
fn values_match(expected: &Value, actual: &Value, tolerance: Option<f64>) -> bool {
    let tol = tolerance.unwrap_or(1e-9);
    match (expected, actual) {
        (Value::Number(e), Value::Number(a)) => match (e.as_f64(), a.as_f64()) {
            (Some(e), Some(a)) => (e - a).abs() <= tol,
            _ => expected == actual,
        },
        (Value::Array(e), Value::Array(a)) if e.len() == a.len() => e
            .iter()
            .zip(a.iter())
            .all(|(e, a)| values_match(e, a, Some(tol))),
        _ => match tolerance {
            Some(tol) => match (expected.as_f64(), actual.as_f64()) {
                (Some(e), Some(a)) => (e - a).abs() <= tol,
                _ => expected == actual,
            },
            None => expected == actual,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_observation_prefers_runtime_identity_without_changing_semantic_id() {
        let mut node = Node::container();
        node.id = Some("tabs:overview".to_owned());
        node.runtime_id = Some("tabs:first:tab:overview".to_owned());

        assert_eq!(runtime_identity(&node), "tabs:first:tab:overview");
        assert_eq!(node.id.as_deref(), Some("tabs:overview"));
    }

    #[test]
    fn collapsed_root_keeps_wrapper_identity() {
        let mut node = Node::input("hello", "");
        node.interaction.focusable = true;
        node.a11y.role = Some(poodle_node::NodeRole::TextInput);
        let iface = InterfaceDoc::parse(&json!({
            "parts": [
                { "id": "root", "resolve": { "native": { "kind": "self" } } },
                { "id": "control", "role": "textbox", "resolve": { "native": { "kind": "self" } } }
            ],
            "states": [],
            "tokenRoles": []
        }))
        .expect("iface");
        let observation = observe_tree("test", "collapsed", &iface, &node, Some(true));
        let root = &observation["parts"]["root"];
        let control = &observation["parts"]["control"];
        assert_eq!(root["role"], Value::Null);
        assert_eq!(root["value"], Value::Null);
        assert_eq!(root["focusable"], json!(false));
        assert_eq!(root["tabbable"], json!(false));
        assert_eq!(root["focused"], json!(true));
        assert_eq!(root["focusVisible"], json!(false));
        assert_eq!(root["focusedText"], Value::Null);
        assert_eq!(control["role"], json!("textbox"));
        assert_eq!(control["value"], json!("hello"));
        assert_eq!(control["focusable"], json!(true));
        assert_eq!(control["parent"], json!("root"));
    }

    #[test]
    fn sole_root_keeps_control_identity() {
        let mut node = Node::container();
        node.interaction.focusable = true;
        node.a11y.role = Some(poodle_node::NodeRole::Button);
        let iface = InterfaceDoc::parse(&json!({
            "parts": [
                { "id": "root", "role": "button", "resolve": { "native": { "kind": "self" } } }
            ],
            "states": [],
            "tokenRoles": []
        }))
        .expect("iface");
        let observation = observe_tree("test", "button", &iface, &node, Some(true));
        let root = &observation["parts"]["root"];
        assert_eq!(root["focusable"], json!(true));
        assert_eq!(root["role"], json!("button"));
        assert_eq!(root["focused"], json!(true));
    }

    #[test]
    fn root_label_does_not_collapse_chrome() {
        let mut node = Node::container().child(Node::text("Save"));
        node.interaction.focusable = true;
        node.a11y.role = Some(poodle_node::NodeRole::Button);
        let iface = InterfaceDoc::parse(&json!({
            "parts": [
                { "id": "root", "role": "button", "resolve": { "native": { "kind": "self" } } },
                { "id": "label", "resolve": { "native": { "kind": "root-label" } } }
            ],
            "states": [],
            "tokenRoles": []
        }))
        .expect("iface");
        let observation = observe_tree("test", "button", &iface, &node, Some(true));
        let root = &observation["parts"]["root"];
        assert_eq!(root["focusable"], json!(true));
        assert_eq!(root["role"], json!("button"));
        assert_eq!(root["name"], json!("Save"));
    }
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

    let present = observed
        .get("present")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let expected_present = expect
        .get("present")
        .and_then(Value::as_bool)
        .unwrap_or(true);
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
        check(
            out,
            runtime,
            step_index,
            part,
            "present".to_owned(),
            expected,
            Some(&json!(present)),
            None,
        );
    }
    for field in [
        "role",
        "name",
        "text",
        "icon",
        "focusable",
        "focused",
        "focusVisible",
        "tabbable",
        "selected",
        "expanded",
        "parent",
        "overlay",
        "focusedText",
        "layerCount",
        "orientation",
        "controls",
        "labelledBy",
        "selectionStart",
        "selectionEnd",
    ] {
        if let Some(expected) = expect.get(field) {
            let actual = observed.get(field);
            check(
                out,
                runtime,
                step_index,
                part,
                field.to_owned(),
                expected,
                actual,
                None,
            );
        }
    }
    if let Some(expected) = expect.get("value") {
        let actual = observed.get("value");
        check(
            out,
            runtime,
            step_index,
            part,
            "value".to_owned(),
            expected,
            actual,
            None,
        );
    }

    if let Some(states) = expect.get("states").and_then(Value::as_object) {
        let observed_states = observed.get("states").cloned().unwrap_or(Value::Null);
        for (state, expected) in states {
            let actual = observed_states.as_object().and_then(|s| s.get(state));
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
            let actual = observed_roles.as_object().and_then(|r| r.get(token));
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
            .expect("validated geometry expectation requires an authored tolerance");
        for field in GEOMETRY_FIELDS {
            let Some(expected) = geometry.get(*field) else {
                continue;
            };
            let actual = observed.get("geometry").and_then(|g| g.get(*field));
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
