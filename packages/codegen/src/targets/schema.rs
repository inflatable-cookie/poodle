//! The JSON Schema target — a draft 2020-12 JSON Schema for the documents
//! the JSON surface target emits, so a consumer can validate one without
//! Rust. Hand-emitted from the model like everything else (ruling R4 — no
//! `schemars`, whose type-mirroring derive could not see instance facts
//! like a component's id set or a permitted subset anyway).
//!
//! Output (per model, under the target's output root):
//!
//! - `schema.json` — one schema validating **both** JSON-surface document
//!   kinds (the per-component documents and `index.json`): the root
//!   `anyOf` discriminates, and `$defs` carry the closed vocabularies
//!   (prop-type and value tag shapes, event kinds, axes, layer) plus
//!   model-derived enums — `component-id` lists exactly the components in
//!   the model and `shared-type-id` exactly the shared types, so the schema
//!   moves with the model like every other declared artifact.
//!
//! Emission rules (the emitter owns every byte, ruling R2):
//!
//! - the `IR-07` header is the `generated` object, mirroring the JSON
//!   surface documents themselves;
//! - every keyword the emitter produces is the minimal subset the emitted
//!   documents need: `type`, `required`, `properties`,
//!   `additionalProperties`, `items`, `oneOf`, `anyOf`, `enum`, `const`,
//!   `uniqueItems`, `minItems`, `maxItems`, `minimum`, `pattern`,
//!   `$ref`/`$defs` — nothing a validator would have to guess at;
//! - ordering is deterministic: `$defs` keys sort alphabetically through
//!   serde_json's map, model-derived enums are sorted by id, and the whole
//!   document pretty-prints byte-identically across runs.

use poodle_ir::{IrModel, IR_SCHEMA_VERSION};

use crate::emit::{EmitTarget, GeneratedFile};
use crate::error::Result;

use super::json_common::{generated_json, json_file};

/// The JSON Schema target.
pub struct JsonSchemaTarget;

impl EmitTarget for JsonSchemaTarget {
    fn id(&self) -> &'static str {
        "schema"
    }

    fn output_root(&self) -> &'static str {
        "schema"
    }

    fn render(&self, model: &IrModel, source_path: &str) -> Result<Vec<GeneratedFile>> {
        Ok(vec![json_file(
            "schema.json",
            schema_document(model, source_path),
        )])
    }
}

/// Builds the schema document. Public for tests; the bin goes through
/// [`EmitTarget::render`].
pub fn schema_document(model: &IrModel, source_path: &str) -> serde_json::Value {
    let mut defs = serde_json::Map::new();
    defs.insert("generated".to_owned(), generated_def());
    defs.insert(
        "component-id".to_owned(),
        sorted_id_enum(model.components.iter().map(|c| c.id.as_str())),
    );
    defs.insert(
        "shared-type-id".to_owned(),
        sorted_id_enum(model.shared_types.iter().map(|t| t.id.as_str())),
    );
    defs.insert("prop-type".to_owned(), prop_type_def());
    defs.insert("value".to_owned(), value_def());
    defs.insert("prop".to_owned(), prop_def());
    defs.insert("event".to_owned(), event_def());
    defs.insert("axes".to_owned(), axes_def());
    defs.insert("component-document".to_owned(), component_document_def());
    defs.insert("index-document".to_owned(), index_document_def());

    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "title": "poodle IR JSON surface schema",
        "description": "Validates the documents emitted by the poodle-codegen `json` target \
                        (per-component prop-surface documents and index.json). Hand-emitted \
                        from the validated IR model; the component-id and shared-type-id \
                        enums are exactly this model's ids.",
        "generated": generated_json(source_path),
        "anyOf": [
            { "$ref": "#/$defs/component-document" },
            { "$ref": "#/$defs/index-document" },
        ],
        "$defs": defs,
    })
}

/// The `IR-07` header shape every JSON-surface document carries.
fn generated_def() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "required": ["generator", "source", "ir_schema_version"],
        "properties": {
            "generator": { "type": "string", "pattern": "^poodle-codegen " },
            "source": { "type": "string" },
            "ir_schema_version": { "const": IR_SCHEMA_VERSION },
        },
        "additionalProperties": false,
    })
}

/// An enum over the given ids, sorted — model-array order is not trusted
/// for output; a reorder in the fixture must not reorder the artifact.
fn sorted_id_enum<'a>(ids: impl Iterator<Item = &'a str>) -> serde_json::Value {
    let mut values: Vec<&str> = ids.collect();
    values.sort_unstable();
    serde_json::json!({ "type": "string", "enum": values })
}

fn prop_type_def() -> serde_json::Value {
    serde_json::json!({
        "oneOf": [
            { "type": "object", "required": ["kind"], "properties": { "kind": { "const": "string" } }, "additionalProperties": false },
            { "type": "object", "required": ["kind"], "properties": { "kind": { "const": "number" } }, "additionalProperties": false },
            { "type": "object", "required": ["kind"], "properties": { "kind": { "const": "boolean" } }, "additionalProperties": false },
            {
                "type": "object",
                "required": ["kind", "shared_type"],
                "properties": {
                    "kind": { "const": "shared" },
                    "shared_type": { "$ref": "#/$defs/shared-type-id" },
                },
                "additionalProperties": false,
            },
            {
                "type": "object",
                "required": ["kind", "of"],
                "properties": {
                    "kind": { "const": "pair" },
                    "of": { "$ref": "#/$defs/prop-type" },
                },
                "additionalProperties": false,
            },
            {
                "type": "object",
                "required": ["kind", "of"],
                "properties": {
                    "kind": { "const": "list" },
                    "of": { "$ref": "#/$defs/prop-type" },
                },
                "additionalProperties": false,
            },
            { "type": "object", "required": ["kind"], "properties": { "kind": { "const": "opaque" } }, "additionalProperties": false },
        ]
    })
}

fn value_def() -> serde_json::Value {
    serde_json::json!({
        "oneOf": [
            { "type": "object", "required": ["kind", "value"], "properties": { "kind": { "const": "string" }, "value": { "type": "string" } }, "additionalProperties": false },
            { "type": "object", "required": ["kind", "value"], "properties": { "kind": { "const": "number" }, "value": { "type": "number" } }, "additionalProperties": false },
            { "type": "object", "required": ["kind", "value"], "properties": { "kind": { "const": "boolean" }, "value": { "type": "boolean" } }, "additionalProperties": false },
            { "type": "object", "required": ["kind", "value"], "properties": { "kind": { "const": "member" }, "value": { "type": "string" } }, "additionalProperties": false },
            {
                "type": "object",
                "required": ["kind", "values"],
                "properties": {
                    "kind": { "const": "pair" },
                    "values": { "type": "array", "items": { "$ref": "#/$defs/value" }, "minItems": 2, "maxItems": 2 },
                },
                "additionalProperties": false,
            },
            {
                "type": "object",
                "required": ["kind", "values"],
                "properties": {
                    "kind": { "const": "list" },
                    "values": { "type": "array", "items": { "$ref": "#/$defs/value" } },
                },
                "additionalProperties": false,
            },
            { "type": "object", "required": ["kind"], "properties": { "kind": { "const": "null" } }, "additionalProperties": false },
        ]
    })
}

fn prop_def() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "required": ["id", "name", "type", "required", "web_only", "description"],
        "properties": {
            "id": { "type": "string" },
            "name": { "type": "string" },
            "type": { "$ref": "#/$defs/prop-type" },
            "permitted_subset": {
                "oneOf": [
                    { "type": "null" },
                    { "type": "array", "items": { "type": "string" }, "uniqueItems": true },
                ]
            },
            "default": { "oneOf": [{ "type": "null" }, { "$ref": "#/$defs/value" }] },
            "required": { "type": "boolean" },
            "web_only": { "type": "boolean" },
            "description": { "type": "string" },
        },
        "additionalProperties": false,
    })
}

fn event_def() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "required": ["id", "name", "kind", "payload", "timing", "description"],
        "properties": {
            "id": { "type": "string" },
            "name": { "type": "string" },
            "kind": {
                "enum": ["activation", "value-change", "value-commit", "focus-change",
                         "pressed-change", "submit", "cancel", "clear", "selection-change"]
            },
            "payload": {
                "oneOf": [
                    { "type": "null" },
                    {
                        "type": "object",
                        "required": ["name", "kind"],
                        "properties": {
                            "name": { "type": "string" },
                            "kind": {
                                "enum": ["none", "string", "number", "boolean", "pair",
                                         "validation-status"]
                            },
                        },
                        "additionalProperties": false,
                    },
                ]
            },
            "timing": {
                "type": "object",
                "required": ["phase", "debounce_ms", "flush_on_blur", "ordering"],
                "properties": {
                    "phase": {
                        "enum": ["during-interaction", "on-release", "on-blur", "immediate"]
                    },
                    "debounce_ms": { "oneOf": [{ "type": "null" }, { "type": "integer", "minimum": 0 }] },
                    "flush_on_blur": { "type": "boolean" },
                    "ordering": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "required": ["before", "after", "reason"],
                            "properties": {
                                "before": { "type": "string" },
                                "after": { "type": "string" },
                                "reason": { "type": "string" },
                            },
                            "additionalProperties": false,
                        },
                    },
                },
                "additionalProperties": false,
            },
            "description": { "type": "string" },
        },
        "additionalProperties": false,
    })
}

fn axes_def() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "required": ["size", "density", "orientation"],
        "properties": {
            "size": {
                "oneOf": [
                    { "type": "null" },
                    {
                        "type": "object",
                        "required": ["explicit", "size_role", "ladder"],
                        "properties": {
                            "explicit": { "oneOf": [{ "type": "null" }, { "enum": ["xs", "sm", "md", "lg", "xl"] }] },
                            "size_role": { "enum": ["chrome", "control", "prominent"] },
                            "ladder": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "required": ["size", "metrics", "description"],
                                    "properties": {
                                        "size": { "enum": ["xs", "sm", "md", "lg", "xl"] },
                                        "metrics": { "type": "object" },
                                        "description": { "type": "string" },
                                    },
                                    "additionalProperties": false,
                                },
                            },
                        },
                        "additionalProperties": false,
                    },
                ]
            },
            "density": {
                "oneOf": [
                    { "type": "null" },
                    {
                        "type": "object",
                        "required": ["explicit", "adjustments"],
                        "properties": {
                            "explicit": { "oneOf": [{ "type": "null" }, { "enum": ["compact", "default", "comfortable"] }] },
                            "adjustments": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "required": ["density", "applies_to", "inline", "block", "description"],
                                    "properties": {
                                        "density": { "enum": ["compact", "default", "comfortable"] },
                                        "applies_to": { "oneOf": [{ "type": "null" }, { "type": "string" }] },
                                        "inline": { "oneOf": [{ "type": "null" }, { "type": "object" }] },
                                        "block": { "oneOf": [{ "type": "null" }, { "type": "object" }] },
                                        "description": { "type": "string" },
                                    },
                                    "additionalProperties": false,
                                },
                            },
                        },
                        "additionalProperties": false,
                    },
                ]
            },
            "orientation": {
                "oneOf": [
                    { "type": "null" },
                    {
                        "type": "object",
                        "required": ["default", "values"],
                        "properties": {
                            "default": { "enum": ["horizontal", "vertical"] },
                            "values": { "type": "array", "items": { "enum": ["horizontal", "vertical"] } },
                        },
                        "additionalProperties": false,
                    },
                ]
            },
        },
        "additionalProperties": false,
    })
}

fn component_document_def() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "required": ["generated", "component", "props", "shared_types", "events", "axes"],
        "properties": {
            "generated": { "$ref": "#/$defs/generated" },
            "component": {
                "type": "object",
                "required": ["id", "name", "layer", "contract", "description"],
                "properties": {
                    "id": { "$ref": "#/$defs/component-id" },
                    "name": { "type": "string" },
                    "layer": { "enum": ["foundation"] },
                    "contract": {
                        "type": "object",
                        "required": ["path", "section"],
                        "properties": {
                            "path": { "type": "string" },
                            "section": { "oneOf": [{ "type": "null" }, { "type": "string" }] },
                        },
                        "additionalProperties": false,
                    },
                    "description": { "type": "string" },
                },
                "additionalProperties": false,
            },
            "props": { "type": "array", "items": { "$ref": "#/$defs/prop" } },
            "shared_types": {
                "type": "array",
                "items": {
                    "type": "object",
                    "required": ["id", "name", "members", "referenced_by"],
                    "properties": {
                        "id": { "$ref": "#/$defs/shared-type-id" },
                        "name": { "type": "string" },
                        "members": { "type": "array", "items": { "type": "string" }, "uniqueItems": true },
                        "referenced_by": { "type": "array", "items": { "type": "string" } },
                    },
                    "additionalProperties": false,
                },
            },
            "events": { "type": "array", "items": { "$ref": "#/$defs/event" } },
            "axes": { "$ref": "#/$defs/axes" },
        },
        "additionalProperties": false,
    })
}

fn index_document_def() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "required": ["generated", "components"],
        "properties": {
            "generated": { "$ref": "#/$defs/generated" },
            "components": {
                "type": "array",
                "items": {
                    "type": "object",
                    "required": ["id", "name", "document"],
                    "properties": {
                        "id": { "$ref": "#/$defs/component-id" },
                        "name": { "type": "string" },
                        "document": { "type": "string" },
                    },
                    "additionalProperties": false,
                },
            },
        },
        "additionalProperties": false,
    })
}
