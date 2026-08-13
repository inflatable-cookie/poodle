//! The conformance target — the corpus's `CV` rows (the model's
//! [`ConformanceVector`]s, serving `CROSS-18`) as one shared vector file
//! every runtime machine executes against, in the `machines.json`
//! convention this repo already uses
//! (`packages/contracts/headless/vectors/machines.json`: named vectors with
//! order-significant rows).
//!
//! Output (per model, under the target's output root):
//!
//! - `vectors.json` — `vectors` sorted by id. Each vector carries its
//!   `applies_to` runtime targets and `steps` in authoring order — step
//!   order is execution order, exactly as `machines.json` says ("Effects
//!   order, so it is never re-sorted. Each step pins the
//!   (input condition → expected machine behavior) pair the corpus `CV`
//!   rows describe: `kind` names the behavior class (transition, guard,
//!   effect-intent, invariant) and `description` what the step proves.
//!   Guard conditions were expressions and are gone (g13.017 R1 bucket 1:
//!   no vector step carried one). `declared_by` lists the components whose
//!   `conformance` declares reliance on the vector, sorted by id.
//!
//! Emission rules (the emitter owns every byte, ruling R2): the `IR-07`
//! header is the `generated` object; vectors sort by id, `declared_by`
//! sorts by id, and steps keep authoring order (order-significant).

use poodle_ir::{ConformanceVector, IrModel};

use crate::emit::{sort_by_id, EmitTarget, GeneratedFile};
use crate::error::Result;

use super::json_common::{generated_json, json_file};

/// The conformance target.
pub struct ConformanceTarget;

impl EmitTarget for ConformanceTarget {
    fn id(&self) -> &'static str {
        "conformance"
    }

    fn output_root(&self) -> &'static str {
        "conformance"
    }

    fn render(&self, model: &IrModel, source_path: &str) -> Result<Vec<GeneratedFile>> {
        Ok(vec![json_file(
            "vectors.json",
            vectors_document(model, source_path),
        )])
    }
}

/// Builds the vectors document. Public for tests; the bin goes through
/// [`EmitTarget::render`].
pub fn vectors_document(model: &IrModel, source_path: &str) -> serde_json::Value {
    let vectors = {
        let mut items: Vec<&ConformanceVector> = model.conformance_vectors.iter().collect();
        sort_by_id(&mut items, |vector| vector.id.as_str());
        items
    };

    serde_json::json!({
        "generated": generated_json(source_path),
        "vectors": vectors
            .iter()
            .map(|vector| vector_entry(model, vector))
            .collect::<Vec<_>>(),
    })
}

fn vector_entry(model: &IrModel, vector: &ConformanceVector) -> serde_json::Value {
    // Components declaring reliance on this vector, sorted by id — a
    // cross-reference the artifact carries so a runtime knows which
    // components' machines a vector pins.
    let mut declared_by: Vec<&str> = model
        .components
        .iter()
        .filter(|component| {
            component
                .conformance
                .iter()
                .any(|id| id.as_str() == vector.id.as_str())
        })
        .map(|component| component.id.as_str())
        .collect();
    declared_by.sort_unstable();

    serde_json::json!({
        "id": vector.id.as_str(),
        "name": vector.name,
        "applies_to": vector.applies_to,
        "description": vector.description,
        "declared_by": declared_by,
        "steps": vector.steps.iter().map(|step| serde_json::json!({
            "id": step.id.as_str(),
            "kind": step.kind,
            "description": step.description,
        })).collect::<Vec<_>>(),
    })
}
