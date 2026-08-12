//! The JSON surface target — one machine-readable document per component
//! describing its prop surface, shared-type references, permitted subsets,
//! events, and axes, plus a stable index.
//!
//! Output (per model, under the target's output root):
//!
//! - `<component-id>.json` — one document per component: identity,
//!   `props` (in declaration order, each with its tagged type, permitted
//!   subset when constrained, default/default-expr, required and web-only
//!   marks, and description), `shared_types` (the shared enumerated types
//!   the component's props reference, sorted by id, members in authoring
//!   order, `referenced_by` naming the referencing props), `events`
//!   (declaration order, with payload and timing), and `axes` (size,
//!   density, orientation).
//! - `index.json` — the stable index: every component document with its
//!   id, name, and output path, sorted by id.
//!
//! Emission rules (the emitter owns every byte, ruling R2):
//!
//! - the `IR-07` header is the `generated` object ([`json_common`]) — the
//!   `//` comment form is not valid JSON;
//! - prop types and values use tagged `kind` objects so string vs number
//!   vs member values are unambiguous to a non-Rust consumer (the
//!   externally-tagged serde form would leak Rust enum names);
//! - a `Shared` prop carries its permitted subset as the prop's own
//!   `permitted_subset` array (R6.2 — the constraint survives into the
//!   artifact) and the referenced shared type appears in `shared_types`;
//! - ordering is deterministic everywhere: components and shared types
//!   sort by id, props and events keep authoring order (the contract's own
//!   order), subset members are a `BTreeSet`, and serde_json maps sort
//!   their keys.

use poodle_ir::{
    Axes, ComponentDefinition, Event, Identifier, IrModel, Prop, PropType, SharedType, Value,
};

use crate::emit::{sort_by_id, EmitTarget, GeneratedFile};
use crate::error::Result;

use super::json_common::{generated_json, json_file};

/// The JSON surface target.
pub struct JsonSurfaceTarget;

impl EmitTarget for JsonSurfaceTarget {
    fn id(&self) -> &'static str {
        "json"
    }

    fn output_root(&self) -> &'static str {
        "json"
    }

    fn render(&self, model: &IrModel, source_path: &str) -> Result<Vec<GeneratedFile>> {
        render_model(model, source_path)
    }
}

/// Renders a model. Public for tests; the bin goes through
/// [`EmitTarget::render`].
pub fn render_model(model: &IrModel, source_path: &str) -> Result<Vec<GeneratedFile>> {
    let generated = generated_json(source_path);

    let components = {
        let mut items: Vec<&ComponentDefinition> = model.components.iter().collect();
        sort_by_id(&mut items, |component| component.id.as_str());
        items
    };

    let mut files = Vec::new();

    for component in &components {
        let document = component_document(model, component, &generated)?;
        files.push(json_file(format!("{}.json", component.id), document));
    }

    let mut index = serde_json::Map::new();
    index.insert("generated".to_owned(), generated);
    index.insert(
        "components".to_owned(),
        serde_json::Value::Array(
            components
                .iter()
                .map(|component| {
                    serde_json::json!({
                        "id": component.id.as_str(),
                        "name": component.name,
                        "document": format!("{}.json", component.id),
                    })
                })
                .collect(),
        ),
    );
    files.push(json_file("index.json", serde_json::Value::Object(index)));

    Ok(files)
}

fn component_document(
    model: &IrModel,
    component: &ComponentDefinition,
    generated: &serde_json::Value,
) -> Result<serde_json::Value> {
    let mut props = Vec::new();
    let mut shared_refs: Vec<(&SharedType, Vec<&str>)> = Vec::new();

    for prop in &component.props {
        props.push(prop_json(prop));
        if let PropType::Shared(id) = &prop.prop_type {
            let shared = shared_type(model, id)?;
            match shared_refs
                .iter_mut()
                .find(|(shared_ref, _)| shared_ref.id == shared.id)
            {
                Some((_, referenced_by)) => referenced_by.push(prop.id.as_str()),
                None => shared_refs.push((shared, vec![prop.id.as_str()])),
            }
        }
    }

    Ok(serde_json::json!({
        "generated": generated,
        "component": {
            "id": component.id.as_str(),
            "name": component.name,
            "layer": component.layer,
            "contract": {
                "path": component.contract.path,
                "section": component.contract.section,
            },
            "description": component.description,
        },
        "props": props,
        "shared_types": shared_refs
            .into_iter()
            .map(|(shared, referenced_by)| serde_json::json!({
                "id": shared.id.as_str(),
                "name": shared.name,
                "members": shared.members.iter().map(|member| member.id.as_str()).collect::<Vec<_>>(),
                "referenced_by": referenced_by,
            }))
            .collect::<Vec<_>>(),
        "events": component.events.iter().map(event_json).collect::<Vec<_>>(),
        "axes": axes_json(&component.axes),
    }))
}

/// Renders one prop: identity, tagged type, permitted subset (R6.2),
/// default and expression default, required and web-only marks, and the
/// description.
fn prop_json(prop: &Prop) -> serde_json::Value {
    serde_json::json!({
        "id": prop.id.as_str(),
        "name": prop.name,
        "type": prop_type_json(&prop.prop_type),
        "permitted_subset": prop.permitted_subset.as_ref().map(|subset| {
            subset
                .members
                .iter()
                .map(|member| member.as_str())
                .collect::<Vec<_>>()
        }),
        "default": prop.default.as_ref().map(value_json),
        "default_expr": prop.default_expr.as_ref().map(serialize),
        "required": prop.required,
        "web_only": prop.web_only,
        "description": prop.description,
    })
}

/// The tagged form of a prop type. `Shared` names the shared type the
/// prop's values must be members of; the permitted subset on the prop
/// constrains it further (R6.2).
fn prop_type_json(prop_type: &PropType) -> serde_json::Value {
    match prop_type {
        PropType::String => serde_json::json!({ "kind": "string" }),
        PropType::Number => serde_json::json!({ "kind": "number" }),
        PropType::Bool => serde_json::json!({ "kind": "boolean" }),
        PropType::Shared(id) => {
            serde_json::json!({ "kind": "shared", "shared_type": id.as_str() })
        }
        PropType::Pair(inner) => {
            serde_json::json!({ "kind": "pair", "of": prop_type_json(inner) })
        }
        PropType::List(inner) => {
            serde_json::json!({ "kind": "list", "of": prop_type_json(inner) })
        }
        PropType::Opaque => serde_json::json!({ "kind": "opaque" }),
    }
}

/// The tagged form of a default value — string vs number vs shared member
/// are unambiguous to a JSON consumer, unlike the externally-tagged serde
/// form.
fn value_json(value: &Value) -> serde_json::Value {
    match value {
        Value::String(s) => serde_json::json!({ "kind": "string", "value": s }),
        Value::Number(n) => serde_json::json!({ "kind": "number", "value": n }),
        Value::Bool(b) => serde_json::json!({ "kind": "boolean", "value": b }),
        Value::Member(id) => serde_json::json!({ "kind": "member", "value": id.as_str() }),
        Value::Pair(a, b) => serde_json::json!({
            "kind": "pair",
            "values": [value_json(a), value_json(b)],
        }),
        Value::List(items) => serde_json::json!({
            "kind": "list",
            "values": items.iter().map(value_json).collect::<Vec<_>>(),
        }),
        Value::Null => serde_json::json!({ "kind": "null" }),
    }
}

/// Renders an event with payload and timing. Enum variants serialize under
/// their serde names (the spec vocabulary), so the document uses the same
/// words the IR does.
fn event_json(event: &Event) -> serde_json::Value {
    serde_json::json!({
        "id": event.id.as_str(),
        "name": event.name,
        "kind": serialize(event.kind),
        "payload": event.payload.as_ref().map(|payload| serde_json::json!({
            "name": payload.name,
            "kind": serialize(payload.kind),
        })),
        "timing": {
            "phase": serialize(event.timing.phase),
            "debounce_ms": event.timing.debounce_ms,
            "flush_on_blur": event.timing.flush_on_blur,
            "ordering": event.timing.ordering.iter().map(|ordering| serde_json::json!({
                "before": ordering.before.as_str(),
                "after": ordering.after.as_str(),
                "reason": ordering.reason,
            })).collect::<Vec<_>>(),
        },
        "description": event.description,
    })
}

/// The axes a component participates in — size, density, orientation in
/// fixed struct order, each `null` when undeclared.
fn axes_json(axes: &Axes) -> serde_json::Value {
    let size = axes.size.as_ref().map(|size| {
        serde_json::json!({
            "explicit": size.explicit.map(serialize),
            "size_role": serialize(size.size_role),
            "fallback": size.fallback.as_ref().map(serialize),
            "ladder": size.ladder.iter().map(|step| serde_json::json!({
                "size": serialize(step.size),
                "metrics": serialize(&step.metrics),
                "description": step.description,
            })).collect::<Vec<_>>(),
        })
    });
    let density = axes.density.as_ref().map(|density| {
        serde_json::json!({
            "explicit": density.explicit.map(serialize),
            "adjustments": density.adjustments.iter().map(|adjustment| serde_json::json!({
                "density": serialize(adjustment.density),
                "applies_to": adjustment.applies_to.as_ref().map(|id| id.as_str()),
                "inline": adjustment.inline.as_ref().map(serialize),
                "block": adjustment.block.as_ref().map(serialize),
                "description": adjustment.description,
            })).collect::<Vec<_>>(),
        })
    });
    let orientation = axes.orientation.as_ref().map(|orientation| {
        serde_json::json!({
            "default": serialize(orientation.default),
            "values": orientation
                .values
                .iter()
                .map(serialize)
                .collect::<Vec<_>>(),
        })
    });
    serde_json::json!({ "size": size, "density": density, "orientation": orientation })
}

/// Serializes a serde value — deterministic for the enums, expressions,
/// and metric maps this target emits (serde_json maps sort their keys).
fn serialize<T: serde::Serialize>(value: T) -> serde_json::Value {
    serde_json::to_value(value).expect("IR values are always serializable")
}

/// Resolves a shared-type reference. A validated model always resolves; the
/// error path exists so emission never panics and never silently drops a
/// reference (mirrors `ts::shared_type_name`).
fn shared_type<'a>(model: &'a IrModel, id: &Identifier) -> Result<&'a SharedType> {
    model
        .shared_type(id.as_str())
        .ok_or_else(|| crate::error::CodegenError::UnresolvedReference {
            what: format!("shared type '{id}'"),
        })
}
