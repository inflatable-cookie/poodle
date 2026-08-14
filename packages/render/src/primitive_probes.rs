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
        let probes = run_neutral_probes(&node);
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
        let probes = run_neutral_probes(&node);
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
