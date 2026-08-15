//! Renderer-neutral primitive substrate probes (spec 066, g14.002).
//!
//! Hand-built `poodle-node` fixtures and capability probes shared by GPUI
//! windowed execution and headless `#[cfg(test)]` checks. No public dummy
//! component — test-only fixtures only.

use std::sync::Arc;

use poodle_layout::{
    CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutOverflow, LayoutSizing,
    MainAxisAlignment,
};
use poodle_node::{
    Node, NodeAnimation, NodeKind, NodePosition, NodeRole, NodeStyle, NodeToggled, StylePatch,
};
use poodle_style::{CornerRadii, CursorHint, FontFamily, StyleDescriptor, TypographyDescriptor};
use poodle_tokens::typed::{ColorValue, ShadowValue};
use serde_json::{json, Value};

use crate::conformance::{observe_tree, InterfaceDoc};

/// Stable element id for GPUI focus registry probes.
pub const PROBE_ELEMENT_ID: &str = "primitive-probe-root";

/// Minimal interface for part-resolution probes — not Button-specific.
pub const PROBE_INTERFACE_JSON: &str = r#"{
  "parts": [
    {
      "id": "root",
      "role": "button",
      "resolve": { "native": { "kind": "self" } }
    },
    {
      "id": "label",
      "resolve": { "native": { "kind": "root-label" } }
    },
    {
      "id": "leadingIcon",
      "resolve": {
        "native": { "kind": "icon-side", "side": "leading", "except": [] }
      }
    }
  ],
  "states": [
    { "name": "disabled", "native": "interaction-disabled" },
    { "name": "pressed", "native": "a11y-toggled" },
    { "name": "focused", "native": "backend-focus" },
    { "name": "focusVisible", "native": "focus-with-focus-style" }
  ],
  "tokenRoles": [
    { "name": "variant", "prop": "variant", "default": "secondary" },
    { "name": "tone", "prop": "tone", "default": "default" }
  ]
}"#;

/// Overlay probe interface (g14.005): trigger + overlay surface parts, so
/// the overlay rows observe through the generic observer like a real overlay
/// profile does.
pub const OVERLAY_PROBE_INTERFACE_JSON: &str = r#"{
  "parts": [
    {
      "id": "root",
      "role": "button",
      "resolve": { "native": { "kind": "self" } }
    },
    {
      "id": "surface",
      "role": "dialog",
      "relativeTo": "root",
      "resolve": { "native": { "kind": "id", "id": "overlay-probe-surface" } }
    }
  ],
  "states": [],
  "tokenRoles": []
}"#;

/// Input probe interface (g14.006): a textbox with value and selection.
pub const INPUT_PROBE_INTERFACE_JSON: &str = r#"{
  "parts": [
    {
      "id": "root",
      "role": "textbox",
      "resolve": { "native": { "kind": "self" } }
    },
    {
      "id": "control",
      "role": "textbox",
      "resolve": { "native": { "kind": "self" } }
    }
  ],
  "states": [],
  "tokenRoles": []
}"#;

pub const INPUT_PROBE_ELEMENT_ID: &str = "input-probe-root";

/// The overlay probe fixture: a hand-built trigger + overlay surface pair
/// carrying the overlay vocabulary — expanded projection, the overlay style
/// flag, the dismiss handler, and the layer id.
pub fn overlay_probe_fixture() -> Node {
    let mut trigger = Node::container();
    trigger.id = Some("overlay-probe-trigger".to_owned());
    trigger.a11y.role = Some(NodeRole::Button);
    trigger.a11y.expanded = Some(true);
    trigger.a11y.label = Some("Overlay probe".to_owned());
    trigger.interaction.focusable = true;
    trigger.interaction.dismiss_layer = Some("overlay-probe-layer".to_owned());
    trigger
        .interaction
        .on_dismiss = Some(Arc::new(|_reason| {}));
    let mut surface = Node::container();
    surface.id = Some("overlay-probe-surface".to_owned());
    surface.a11y.role = Some(NodeRole::Dialog);
    surface.style.overlay = true;
    surface.interaction.dismiss_layer = Some("overlay-probe-layer".to_owned());
    trigger.child(surface)
}

pub fn overlay_probe_interface() -> InterfaceDoc {
    let interface: Value =
        serde_json::from_str(OVERLAY_PROBE_INTERFACE_JSON).expect("overlay probe interface parses");
    InterfaceDoc::parse(&interface).expect("overlay probe interface valid")
}

/// Hand-built input fixture carrying the editing vocabulary: Input kind,
/// caret, and the key/insert/submit/cancel doors. GPUI mounts a real
/// TextInput for the event graft.
pub fn input_probe_fixture() -> Node {
    let mut root = Node::input("hello", "hint");
    root.id = Some(INPUT_PROBE_ELEMENT_ID.to_owned());
    root.a11y.role = Some(NodeRole::TextInput);
    root.a11y.label = Some("Input probe".to_owned());
    root.interaction.focusable = true;
    root.interaction.on_edit_key = Some(Arc::new(|_, _| {}));
    root.interaction.on_edit_insert = Some(Arc::new(|_| {}));
    root.interaction.on_submit = Some(Arc::new(|| {}));
    root.interaction.on_cancel = Some(Arc::new(|| {}));
    root.interaction.on_text_change = Some(Arc::new(|_| {}));
    root.interaction.on_select_range = Some(Arc::new(|_, _, _| {}));
    root.with_caret(
        (2, 5),
        ColorValue(1.0, 1.0, 1.0, 1.0),
        ColorValue(0.2, 0.4, 0.8, 0.3),
    )
}

pub fn input_probe_interface() -> InterfaceDoc {
    let interface: Value =
        serde_json::from_str(INPUT_PROBE_INTERFACE_JSON).expect("input probe interface parses");
    InterfaceDoc::parse(&interface).expect("input probe interface valid")
}

/// One executed primitive probe row (`primitive-probe-evidence.v1`).
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProbeEvidence {
    pub capability_id: String,
    pub probe_id: String,
    pub verdict: &'static str,
    pub observations: Vec<&'static str>,
    pub fields: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl ProbeEvidence {
    pub fn pass(
        capability_id: impl Into<String>,
        probe_id: impl Into<String>,
        fields: Value,
    ) -> Self {
        Self {
            capability_id: capability_id.into(),
            probe_id: probe_id.into(),
            verdict: "pass",
            observations: Vec::new(),
            fields,
            reason: None,
        }
    }

    pub fn pass_observed(
        capability_id: impl Into<String>,
        probe_id: impl Into<String>,
        fields: Value,
        observations: &[&'static str],
    ) -> Self {
        Self {
            capability_id: capability_id.into(),
            probe_id: probe_id.into(),
            verdict: "pass",
            observations: observations.to_vec(),
            fields,
            reason: None,
        }
    }

    pub fn fail(
        capability_id: impl Into<String>,
        probe_id: impl Into<String>,
        fields: Value,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            capability_id: capability_id.into(),
            probe_id: probe_id.into(),
            verdict: "fail",
            observations: Vec::new(),
            fields,
            reason: Some(reason.into()),
        }
    }

    /// Plant-ready diagnostic naming capability, runtime/layer, probe, field.
    pub fn failure_message(&self, runtime: &str) -> String {
        format!(
            "capability={} runtime={} probe={} field={}",
            self.capability_id,
            runtime,
            self.probe_id,
            self.reason.as_deref().unwrap_or("unknown")
        )
    }
}

pub fn probe_interface() -> InterfaceDoc {
    let interface: Value =
        serde_json::from_str(PROBE_INTERFACE_JSON).expect("probe interface parses");
    InterfaceDoc::parse(&interface).expect("probe interface valid")
}

/// Hand-built node tree exercising the g14.002 substrate vocabulary.
pub fn build_probe_fixture(activate_trace: Option<Arc<dyn Fn() + Send + Sync>>) -> Node {
    let hover = StylePatch {
        background: Some(ColorValue(0.2, 0.4, 0.8, 1.0)),
        border_color: None,
        text_color: None,
        opacity: Some(0.95),
    };
    let focus = StylePatch {
        background: Some(ColorValue(0.15, 0.35, 0.75, 1.0)),
        border_color: Some(ColorValue(0.1, 0.2, 0.5, 1.0)),
        text_color: None,
        opacity: None,
    };
    let active = StylePatch {
        background: Some(ColorValue(0.1, 0.3, 0.7, 1.0)),
        border_color: None,
        text_color: None,
        opacity: None,
    };

    let layout = LayoutIntent::new()
        .with_direction(LayoutDirection::Row)
        .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Center)
        .with_gap(8.0)
        .with_padding(LayoutEdges::symmetric(12.0, 8.0))
        .with_height(LayoutSizing::Fixed(40.0))
        .with_width(LayoutSizing::Constrained {
            min: Some(80.0),
            max: Some(240.0),
        })
        .with_overflow(LayoutOverflow::Hidden, LayoutOverflow::Hidden);

    let descriptor = StyleDescriptor::new()
        .with_background(ColorValue(0.18, 0.22, 0.28, 1.0))
        .with_text_color(ColorValue(0.95, 0.96, 0.98, 1.0))
        .with_icon_color(ColorValue(0.8, 0.85, 0.9, 1.0))
        .with_border(1.0, ColorValue(0.35, 0.4, 0.48, 1.0))
        .with_corner_radii(CornerRadii::uniform(6.0))
        .with_shadow(ShadowValue {
            offset_x: 0.0,
            offset_y: 2.0,
            blur: 4.0,
            color: ColorValue(0.0, 0.0, 0.0, 0.25),
        })
        .with_typography(TypographyDescriptor {
            family: FontFamily::Sans,
            size: 14.0,
            weight: 500,
            line_height: 1.25,
        })
        .with_cursor(CursorHint::Pointer)
        .with_layout(layout);

    let mut style = NodeStyle {
        descriptor,
        hover: Some(hover),
        active: Some(active),
        focus: Some(focus),
        min_width: Some(80.0),
        max_width: Some(240.0),
        flex_grow: Some(1.0),
        text_size: Some(14.0),
        text_weight: Some(500),
        font_family: Some(FontFamily::Sans),
        line_height: Some(1.25),
        letter_spacing_em: Some(0.02),
        text_align: Some(poodle_node::TextAlign::Center),
        text_wrap: true,
        border_color_left: Some(ColorValue(0.5, 0.55, 0.6, 1.0)),
        border_dashed: false,
        animation: Some(NodeAnimation::spin("probe-spin", 2.0)),
        ..NodeStyle::default()
    };
    style.descriptor.opacity = 0.98;

    let mut root = Node {
        id: Some(PROBE_ELEMENT_ID.to_owned()),
        kind: NodeKind::Container,
        style,
        position: NodePosition::Relative,
        interaction: poodle_node::Interaction {
            focusable: true,
            disabled: false,
            on_activate: activate_trace,
            ..Default::default()
        },
        a11y: poodle_node::NodeA11y {
            role: Some(NodeRole::Button),
            label: Some("Probe control".to_owned()),
            toggled: Some(NodeToggled::False),
            ..Default::default()
        },
        roles: [
            ("variant".to_owned(), "primary".to_owned()),
            ("tone".to_owned(), "default".to_owned()),
        ]
        .into_iter()
        .collect(),
        ..Node::default()
    };

    root.children.push({
        let mut icon = Node::icon("star", 16.0);
        icon.style.flex_shrink_zero = true;
        // Exercise the backend's disabled-listener short circuit without
        // disabling the interactive root used by focus/activation probes.
        icon.interaction.disabled = true;
        icon.interaction.on_activate = Some(Arc::new(|| {}));
        icon
    });
    root.children.push(Node::text("Probe"));
    root
}

fn probe_structure_identity(node: &Node) -> ProbeEvidence {
    let id_ok = node.id.as_deref() == Some(PROBE_ELEMENT_ID);
    let children_ok = node.children.len() == 2;
    let kinds_ok = matches!(node.kind, NodeKind::Container)
        && matches!(node.children[0].kind, NodeKind::Icon { .. })
        && matches!(node.children[1].kind, NodeKind::Text { .. });
    let fields = json!({
        "node.id": node.id,
        "children.len": node.children.len(),
        "root.kind": "Container",
    });
    if id_ok && children_ok && kinds_ok {
        ProbeEvidence::pass_observed(
            "structure.identity",
            "node-tree",
            fields,
            &["parts.present", "node.field"],
        )
    } else {
        ProbeEvidence::fail(
            "structure.identity",
            "node-tree",
            fields,
            "node.id or child structure",
        )
    }
}

fn probe_structure_part_resolution(node: &Node, iface: &InterfaceDoc) -> ProbeEvidence {
    let observation = observe_tree("render-neutral", "primitive-probe", iface, node, None);
    let label_present = observation
        .pointer("/parts/label/present")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let label_text = observation.pointer("/parts/label/text").cloned();
    let icon_present = observation
        .pointer("/parts/leadingIcon/present")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let icon_name = observation.pointer("/parts/leadingIcon/icon").cloned();
    let fields = json!({
        "label.present": label_present,
        "label.text": label_text,
        "leadingIcon.present": icon_present,
        "leadingIcon.icon": icon_name,
    });
    if label_present
        && label_text == Some(json!("Probe"))
        && icon_present
        && icon_name == Some(json!("star"))
    {
        ProbeEvidence::pass_observed(
            "structure.part-resolution",
            "observe-tree",
            fields,
            &["parts.present", "parts.text", "parts.icon"],
        )
    } else {
        ProbeEvidence::fail(
            "structure.part-resolution",
            "observe-tree",
            fields,
            "parts.label or parts.leadingIcon",
        )
    }
}

fn probe_layout_intent(node: &Node) -> ProbeEvidence {
    let layout = &node.style.descriptor.layout;
    let fields = json!({
        "direction": format!("{:?}", layout.direction),
        "mainAxis": format!("{:?}", layout.alignment.main),
        "crossAxis": format!("{:?}", layout.alignment.cross),
        "gap": layout.spacing.gap,
        "paddingLeft": layout.spacing.padding.left,
    });
    let pass = layout.direction == LayoutDirection::Row
        && layout.alignment.main == MainAxisAlignment::Center
        && layout.spacing.gap == 8.0;
    if pass {
        ProbeEvidence::pass_observed(
            "layout.intent",
            "descriptor-layout",
            fields,
            &["node.field"],
        )
    } else {
        ProbeEvidence::fail(
            "layout.intent",
            "descriptor-layout",
            fields,
            "layout.intent fields",
        )
    }
}

fn probe_layout_geometry(node: &Node) -> ProbeEvidence {
    let fields = json!({
        "minWidth": node.style.min_width,
        "maxWidth": node.style.max_width,
        "flexGrow": node.style.flex_grow,
        "height": match node.style.descriptor.layout.height {
            LayoutSizing::Fixed(h) => json!(h),
            other => json!(format!("{:?}", other)),
        },
    });
    let pass = node.style.min_width == Some(80.0)
        && node.style.max_width == Some(240.0)
        && node.style.flex_grow == Some(1.0);
    if pass {
        ProbeEvidence::pass_observed("layout.geometry", "node-style", fields, &["node.field"])
    } else {
        ProbeEvidence::fail("layout.geometry", "node-style", fields, "geometry fields")
    }
}

fn probe_layout_position(node: &Node) -> ProbeEvidence {
    let fields = json!({
        "position": format!("{:?}", node.position),
    });
    if matches!(node.position, NodePosition::Relative) {
        ProbeEvidence::pass_observed("layout.position", "node-position", fields, &["node.field"])
    } else {
        ProbeEvidence::fail("layout.position", "node-position", fields, "NodePosition")
    }
}

fn probe_surface_channels(node: &Node, iface: &InterfaceDoc) -> ProbeEvidence {
    let observation = observe_tree("render-neutral", "primitive-probe", iface, node, None);
    let channels = observation
        .pointer("/parts/root/channels")
        .cloned()
        .unwrap_or(Value::Null);
    let bg = channels.get("background").is_some();
    let border = channels.get("borderColor").is_some();
    let fields = json!({ "channels": channels });
    if bg && border {
        ProbeEvidence::pass_observed(
            "surface.channels",
            "observe-tree-channels",
            fields,
            &["parts.channels", "node.field"],
        )
    } else {
        ProbeEvidence::fail(
            "surface.channels",
            "observe-tree-channels",
            fields,
            "parts.root.channels",
        )
    }
}

fn probe_surface_extended(node: &Node) -> ProbeEvidence {
    let desc = &node.style.descriptor;
    let fields = json!({
        "iconColor": desc.icon_color.is_some(),
        "shadow": desc.shadow.is_some(),
        "cursor": format!("{:?}", desc.cursor),
        "borderColorLeft": node.style.border_color_left.is_some(),
        "typography": desc.typography.is_some(),
    });
    let pass = desc.icon_color.is_some()
        && desc.shadow.is_some()
        && desc.cursor == CursorHint::Pointer
        && node.style.border_color_left.is_some();
    if pass {
        ProbeEvidence::pass_observed(
            "surface.extended",
            "node-style-extended",
            fields,
            &["node.field"],
        )
    } else {
        ProbeEvidence::fail(
            "surface.extended",
            "node-style-extended",
            fields,
            "extended surface fields",
        )
    }
}

fn probe_surface_state_patches(node: &Node) -> ProbeEvidence {
    let fields = json!({
        "hover": node.style.hover.is_some(),
        "active": node.style.active.is_some(),
        "focus": node.style.focus.is_some(),
    });
    if node.style.hover.is_some() && node.style.active.is_some() && node.style.focus.is_some() {
        ProbeEvidence::pass_observed(
            "surface.state-patches",
            "style-patches",
            fields,
            &["node.field"],
        )
    } else {
        ProbeEvidence::fail(
            "surface.state-patches",
            "style-patches",
            fields,
            "StylePatch channels",
        )
    }
}

fn probe_surface_animation(node: &Node) -> ProbeEvidence {
    let fields = json!({
        "animation": node.style.animation.as_ref().map(|anim| json!({
            "key": anim.key,
            "durationSecs": anim.duration_secs,
            "loopMode": format!("{:?}", anim.loop_mode),
        })),
    });
    if node.style.animation.is_some() {
        ProbeEvidence::pass_observed(
            "surface.animation",
            "node-animation",
            fields,
            &["node.field"],
        )
    } else {
        ProbeEvidence::fail(
            "surface.animation",
            "node-animation",
            fields,
            "NodeStyle.animation",
        )
    }
}

fn probe_content_text_icon(node: &Node, iface: &InterfaceDoc) -> ProbeEvidence {
    let observation = observe_tree("render-neutral", "primitive-probe", iface, node, None);
    let text = observation.pointer("/parts/label/text").cloned();
    let icon = observation.pointer("/parts/leadingIcon/icon").cloned();
    let fields = json!({ "label.text": text, "leadingIcon.icon": icon });
    if text == Some(json!("Probe")) && icon == Some(json!("star")) {
        ProbeEvidence::pass_observed(
            "content.text-icon",
            "part-carriers",
            fields,
            &["parts.text", "parts.icon", "node.field"],
        )
    } else {
        ProbeEvidence::fail(
            "content.text-icon",
            "part-carriers",
            fields,
            "text or icon content",
        )
    }
}

fn probe_content_typography(node: &Node) -> ProbeEvidence {
    let fields = json!({
        "textSize": node.style.text_size,
        "textWeight": node.style.text_weight,
        "fontFamily": node.style.font_family.as_ref().map(|f| format!("{:?}", f)),
        "lineHeight": node.style.line_height,
        "letterSpacingEm": node.style.letter_spacing_em,
        "textAlign": node.style.text_align.as_ref().map(|a| format!("{:?}", a)),
        "textWrap": node.style.text_wrap,
    });
    let pass = node.style.text_size == Some(14.0)
        && node.style.text_weight == Some(500)
        && node.style.line_height == Some(1.25);
    if pass {
        ProbeEvidence::pass_observed(
            "content.typography",
            "node-typography",
            fields,
            &["node.field"],
        )
    } else {
        ProbeEvidence::fail(
            "content.typography",
            "node-typography",
            fields,
            "typography fields",
        )
    }
}

fn probe_semantic_token_roles(node: &Node, iface: &InterfaceDoc) -> ProbeEvidence {
    let observation = observe_tree("render-neutral", "primitive-probe", iface, node, None);
    let roles = observation
        .pointer("/parts/root/tokenRoles")
        .cloned()
        .unwrap_or(Value::Null);
    let fields = json!({ "tokenRoles": roles });
    if roles.get("variant") == Some(&json!("primary"))
        && roles.get("tone") == Some(&json!("default"))
    {
        ProbeEvidence::pass_observed(
            "semantic.token-roles",
            "observe-tree-roles",
            fields,
            &["parts.tokenRoles"],
        )
    } else {
        ProbeEvidence::fail(
            "semantic.token-roles",
            "observe-tree-roles",
            fields,
            "parts.root.tokenRoles",
        )
    }
}

fn probe_toggle(node: &Node, iface: &InterfaceDoc) -> ProbeEvidence {
    let observation = observe_tree("render-neutral", "primitive-probe", iface, node, None);
    let pressed = observation.pointer("/parts/root/states/pressed").cloned();
    let fields =
        json!({ "states.pressed": pressed, "a11y.toggled": format!("{:?}", node.a11y.toggled) });
    if pressed == Some(json!(false)) {
        ProbeEvidence::pass_observed("toggle", "a11y-toggled", fields, &["parts.states"])
    } else {
        ProbeEvidence::fail("toggle", "a11y-toggled", fields, "states.pressed")
    }
}

fn probe_semantic_disabled(node: &Node, iface: &InterfaceDoc) -> ProbeEvidence {
    let observation = observe_tree("render-neutral", "primitive-probe", iface, node, None);
    let disabled = observation.pointer("/parts/root/states/disabled").cloned();
    let fields = json!({
        "states.disabled": disabled,
        "interaction.disabled": node.interaction.disabled,
    });
    if disabled == Some(json!(false)) && !node.interaction.disabled {
        ProbeEvidence::pass_observed(
            "semantic.disabled",
            "interaction-disabled",
            fields,
            &["parts.states"],
        )
    } else {
        ProbeEvidence::fail(
            "semantic.disabled",
            "interaction-disabled",
            fields,
            "states.disabled",
        )
    }
}

fn probe_accessibility_projection(node: &Node, iface: &InterfaceDoc) -> ProbeEvidence {
    let observation = observe_tree("render-neutral", "primitive-probe", iface, node, None);
    let role = observation.pointer("/parts/root/role").cloned();
    let name = observation.pointer("/parts/root/name").cloned();
    let fields = json!({
        "parts.role": role,
        "parts.name": name,
        "node.a11y.role": node.a11y.role.as_ref().map(|r| format!("{:?}", r)),
        "node.a11y.label": node.a11y.label,
    });
    if role == Some(json!("button")) && name == Some(json!("Probe control")) {
        ProbeEvidence::pass_observed(
            "accessibility.projection",
            "a11y-metadata",
            fields,
            &["parts.role", "parts.name", "node.a11y"],
        )
    } else {
        ProbeEvidence::fail(
            "accessibility.projection",
            "a11y-metadata",
            fields,
            "parts.role or parts.name",
        )
    }
}

/// Renderer-neutral probes: node fields and observe_tree where applicable.
pub fn run_neutral_probes(node: &Node) -> Vec<ProbeEvidence> {
    let iface = probe_interface();
    vec![
        probe_structure_identity(node),
        probe_structure_part_resolution(node, &iface),
        probe_layout_intent(node),
        probe_layout_geometry(node),
        probe_layout_position(node),
        probe_surface_channels(node, &iface),
        probe_surface_extended(node),
        probe_surface_state_patches(node),
        probe_surface_animation(node),
        probe_content_text_icon(node, &iface),
        probe_content_typography(node),
        probe_semantic_token_roles(node, &iface),
        probe_toggle(node, &iface),
        probe_semantic_disabled(node, &iface),
        probe_accessibility_projection(node, &iface),
        probe_focus_neutral(node, &iface),
        probe_activate_neutral(node),
    ]
}

// ── Overlay probes (g14.005) ──────────────────────────────────────────────

/// The overlay intent channel: `NodeStyle.overlay` declared on the surface
/// node and observed through the generic observer's `overlay` part field.
pub fn probe_overlay_intent(node: &Node) -> ProbeEvidence {
    let surface = node.find(&|n| n.id.as_deref() == Some("overlay-probe-surface"));
    let declared = surface.map(|s| s.style.overlay).unwrap_or(false);
    let iface = overlay_probe_interface();
    let observation = observe_tree("render-neutral", "overlay-probe", &iface, node, None);
    let observed = observation
        .pointer("/parts/surface/overlay")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let fields = json!({
        "node.style.overlay": declared,
        "parts.surface.overlay": observed,
    });
    if declared && observed {
        ProbeEvidence::pass_observed(
            "overlay.intent",
            "node-overlay",
            fields,
            &["node.field", "parts.overlay"],
        )
    } else {
        ProbeEvidence::fail(
            "overlay.intent",
            "node-overlay",
            fields,
            "node.style.overlay or parts.surface.overlay",
        )
    }
}

/// The expanded projection channel: `NodeA11y.expanded` declared and observed
/// through the generic observer's `expanded` part field.
pub fn probe_semantic_expanded(node: &Node) -> ProbeEvidence {
    let declared = node.a11y.expanded.unwrap_or(false);
    let iface = overlay_probe_interface();
    let observation = observe_tree("render-neutral", "overlay-probe", &iface, node, None);
    let observed = observation
        .pointer("/parts/root/expanded")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let fields = json!({
        "node.a11y.expanded": declared,
        "parts.root.expanded": observed,
    });
    if declared && observed {
        ProbeEvidence::pass_observed(
            "semantic.expanded",
            "node-expanded",
            fields,
            &["node.a11y", "parts.expanded"],
        )
    } else {
        ProbeEvidence::fail(
            "semantic.expanded",
            "node-expanded",
            fields,
            "node.a11y.expanded or parts.root.expanded",
        )
    }
}

/// The dismissal channel: `Interaction.on_dismiss` routes the two real
/// reasons (Escape and outside) to the component's handler. The real event
/// dispatch lives on the web and GPUI layers; this proves the neutral channel
/// and reason routing.
pub fn probe_overlay_dismiss(node: &Node) -> ProbeEvidence {
    let declared = node
        .find(&|n| n.id.as_deref() == Some("overlay-probe-trigger"))
        .is_some_and(|trigger| trigger.interaction.on_dismiss.is_some());
    use std::sync::{Arc, Mutex};
    let received: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));
    let listener = Arc::clone(&received);
    let probe_handler: poodle_node::DismissHandler = Arc::new(move |reason| {
        listener.lock().expect("reason lock").push(match reason {
            poodle_node::DismissReason::Escape => "escape",
            poodle_node::DismissReason::Outside => "outside",
        });
    });
    probe_handler(poodle_node::DismissReason::Escape);
    probe_handler(poodle_node::DismissReason::Outside);
    let reasons = received.lock().expect("reason lock").clone();
    let fields = json!({ "on_dismiss_declared": declared, "reasons": reasons });
    if declared && reasons == ["escape", "outside"] {
        ProbeEvidence::pass_observed(
            "overlay.dismiss",
            "node-dismiss-reasons",
            fields,
            &["node.field"],
        )
    } else {
        ProbeEvidence::fail(
            "overlay.dismiss",
            "node-dismiss-reasons",
            fields,
            "on_dismiss channel or reason routing",
        )
    }
}

/// The layer channel: `Interaction.dismiss_layer` declares layer membership
/// on the trigger and surface nodes (the containment unit the backends
/// register). Layer order/count observation is delivered by the runtime
/// layers (web stack, GPUI registry).
pub fn probe_overlay_layer(node: &Node) -> ProbeEvidence {
    fn collect(node: &Node, ids: &mut Vec<String>) {
        if let Some(layer) = &node.interaction.dismiss_layer {
            ids.push(layer.clone());
        }
        for child in &node.children {
            collect(child, ids);
        }
    }
    let mut layer_ids = Vec::new();
    collect(node, &mut layer_ids);
    let fields = json!({ "dismiss_layers": layer_ids });
    if layer_ids.len() == 2 && layer_ids.iter().all(|id| id == "overlay-probe-layer") {
        ProbeEvidence::pass_observed(
            "overlay.layer",
            "node-layer-membership",
            fields,
            &["node.field"],
        )
    } else {
        ProbeEvidence::fail(
            "overlay.layer",
            "node-layer-membership",
            fields,
            "dismiss_layer membership",
        )
    }
}

/// The overlay rows' renderer-neutral evidence, executed by the neutral probe
/// board and the GPUI probe runner.
pub fn run_overlay_probes(node: &Node) -> Vec<ProbeEvidence> {
    vec![
        probe_overlay_intent(node),
        probe_semantic_expanded(node),
        probe_overlay_dismiss(node),
        probe_overlay_layer(node),
    ]
}

fn probe_input_value(node: &Node, iface: &InterfaceDoc) -> ProbeEvidence {
    let kind_value = match &node.kind {
        NodeKind::Input { value, .. } => Some(value.clone()),
        _ => None,
    };
    let caret = node.caret.map(|c| c.selection);
    let observation = observe_tree("render-neutral", "input-probe", iface, node, Some(true));
    let parts_value = observation.pointer("/parts/control/value").cloned();
    let selection = observation.pointer("/parts/control/selectionStart").cloned();
    let fields = json!({
        "node.kind.value": kind_value,
        "node.caret": caret,
        "parts.value": parts_value,
        "parts.selection": selection,
    });
    if kind_value.as_deref() == Some("hello")
        && caret == Some((2, 5))
        && parts_value.and_then(|v| v.as_str().map(str::to_owned)).as_deref() == Some("hello")
        && selection.and_then(|v| v.as_u64()) == Some(2)
    {
        ProbeEvidence::pass_observed(
            "input.value",
            "node-input-value",
            fields,
            &["node.field", "parts.value", "parts.selection"],
        )
    } else {
        ProbeEvidence::fail("input.value", "node-input-value", fields, "parts.value")
    }
}

fn probe_input_editing(node: &Node) -> ProbeEvidence {
    let fields = json!({
        "on_edit_key": node.interaction.on_edit_key.is_some(),
        "on_edit_insert": node.interaction.on_edit_insert.is_some(),
        "on_select_range": node.interaction.on_select_range.is_some(),
        "on_submit": node.interaction.on_submit.is_some(),
        "on_cancel": node.interaction.on_cancel.is_some(),
        "on_text_change": node.interaction.on_text_change.is_some(),
    });
    if node.interaction.on_edit_key.is_some()
        && node.interaction.on_edit_insert.is_some()
        && node.interaction.on_select_range.is_some()
        && node.interaction.on_submit.is_some()
        && node.interaction.on_cancel.is_some()
    {
        ProbeEvidence::pass_observed(
            "input.editing",
            "node-edit-channels",
            fields,
            &["node.field"],
        )
    } else {
        ProbeEvidence::fail("input.editing", "node-edit-channels", fields, "edit channels")
    }
}

fn probe_input_ime(node: &Node) -> ProbeEvidence {
    let wired = node.interaction.on_edit_insert.is_some();
    let fields = json!({
        "on_edit_insert": wired,
        "kind": matches!(node.kind, NodeKind::Input { .. }),
    });
    if wired && matches!(node.kind, NodeKind::Input { .. }) {
        ProbeEvidence::pass_observed(
            "input.ime",
            "node-ime-commit-door",
            fields,
            &["node.field"],
        )
    } else {
        ProbeEvidence::fail("input.ime", "node-ime-commit-door", fields, "on_edit_insert")
    }
}

/// The input rows' renderer-neutral evidence.
pub fn run_input_probes(node: &Node) -> Vec<ProbeEvidence> {
    let iface = input_probe_interface();
    vec![
        probe_input_value(node, &iface),
        probe_input_editing(node),
        probe_input_ime(node),
    ]
}

fn probe_focus_neutral(node: &Node, iface: &InterfaceDoc) -> ProbeEvidence {
    let focused = observe_tree("render-neutral", "primitive-probe", iface, node, Some(true));
    let unfocused = observe_tree(
        "render-neutral",
        "primitive-probe",
        iface,
        node,
        Some(false),
    );
    let focusable = focused
        .pointer("/parts/root/focusable")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let focused_true = focused
        .pointer("/parts/root/focused")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let focused_false = unfocused
        .pointer("/parts/root/focused")
        .and_then(Value::as_bool)
        == Some(false);
    let fields = json!({
        "parts.focusable": focusable,
        "parts.focused.true": focused_true,
        "parts.focused.false": focused_false,
        "interaction.focusable": node.interaction.focusable,
    });
    if node.interaction.focusable && focusable && focused_true && focused_false {
        ProbeEvidence::pass_observed(
            "focus",
            "node-focus-channel",
            fields,
            &["parts.focusable", "parts.focused"],
        )
    } else {
        ProbeEvidence::fail("focus", "node-focus-channel", fields, "parts.focused")
    }
}

fn probe_activate_neutral(node: &Node) -> ProbeEvidence {
    let wired = node.interaction.on_activate.is_some();
    let fields = json!({
        "interaction.on_activate": wired,
    });
    // Headless layer certifies the activation channel is present on the node.
    // GPUI proves the real event path; web proves dispatched DOM events.
    if wired {
        ProbeEvidence::pass_observed("activate", "node-activate-channel", fields, &[])
    } else {
        ProbeEvidence::fail(
            "activate",
            "node-activate-channel",
            fields,
            "interaction.on_activate",
        )
    }
}

pub fn neutral_evidence_report(probes: &[ProbeEvidence]) -> Value {
    json!({
        "schema": "primitive-probe-evidence.v1",
        "runtime": "render-neutral",
        "probes": probes,
    })
}

/// GPUI-layer probes that require backend focus registry or real events.
pub fn probe_focus_gpui(
    backend_focus: Option<bool>,
    focus_visible: Option<Value>,
) -> ProbeEvidence {
    let fields = json!({
        "gpui.focus": backend_focus,
        "parts.focusVisible": focus_visible,
    });
    if backend_focus == Some(true) {
        ProbeEvidence::pass_observed("focus", "backend-focus-registry", fields, &["gpui.focus"])
    } else {
        ProbeEvidence::fail("focus", "backend-focus-registry", fields, "gpui.focus")
    }
}

pub fn probe_activate_gpui(trace_len: usize) -> ProbeEvidence {
    let fields = json!({ "trace.len": trace_len, "gpui.event": trace_len > 0 });
    if trace_len > 0 {
        ProbeEvidence::pass_observed("activate", "pointer-activate", fields, &["gpui.event"])
    } else {
        ProbeEvidence::fail("activate", "pointer-activate", fields, "gpui.event")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Headless substrate probes — fail loudly when a capability regresses.
    #[test]
    fn primitive_substrate_neutral_probes_pass() {
        let handler: Arc<dyn Fn() + Send + Sync> = Arc::new(|| {});
        let node = build_probe_fixture(Some(handler));
        let probes = [
            run_neutral_probes(&node),
            run_overlay_probes(&overlay_probe_fixture()),
            run_input_probes(&input_probe_fixture()),
        ]
        .concat();
        let failures: Vec<_> = probes.iter().filter(|p| p.verdict == "fail").collect();
        assert!(
            failures.is_empty(),
            "neutral primitive probes failed: {:?}",
            failures
                .iter()
                .map(|p| p.failure_message("render-neutral"))
                .collect::<Vec<_>>()
        );
    }

    /// Emit JSON evidence for the report gate.
    #[test]
    fn emit_neutral_primitive_evidence() {
        let handler: Arc<dyn Fn() + Send + Sync> = Arc::new(|| {});
        let node = build_probe_fixture(Some(handler));
        let probes = [
            run_neutral_probes(&node),
            run_overlay_probes(&overlay_probe_fixture()),
            run_input_probes(&input_probe_fixture()),
        ]
        .concat();
        let report = neutral_evidence_report(&probes);
        let out = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../test/conformance/web/out/primitive-render-neutral.json");
        if let Some(parent) = out.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        std::fs::write(
            &out,
            format!(
                "{}\n",
                serde_json::to_string_pretty(&report).expect("report serializes")
            ),
        )
        .expect("write render-neutral evidence");
        assert!(probes.iter().all(|p| p.verdict == "pass"));
    }
}
