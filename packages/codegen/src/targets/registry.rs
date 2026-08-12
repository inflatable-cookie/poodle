//! The registry target — one document listing every component: id,
//! capabilities required, axes supported, shared types referenced. The
//! "what exists" index, consumed by tooling that must enumerate the
//! component inventory without parsing per-component documents.
//!
//! Output (per model, under the target's output root):
//!
//! - `registry.json` — `components` sorted by id; per component: `id` and
//!   `name`, `capabilities` (the declared adapter capability requirements,
//!   sorted by the inventory order of [`Capability`]), `axes` (the axes the
//!   component participates in — size, density, orientation in fixed struct
//!   order), and `shared_types` (the ids of the shared enumerated types
//!   its props reference, sorted).
//!
//! Emission rules (the emitter owns every byte, ruling R2): the `IR-07`
//! header is the `generated` object; every list is sorted by a stable key
//! — component ids, capability inventory order, axis struct order, shared
//! type ids — never model-array iteration order.

use poodle_ir::{ComponentDefinition, IrModel, PropType};

use crate::emit::{sort_by_id, EmitTarget, GeneratedFile};
use crate::error::Result;

use super::json_common::{generated_json, json_file};

/// The registry target.
pub struct RegistryTarget;

impl EmitTarget for RegistryTarget {
    fn id(&self) -> &'static str {
        "registry"
    }

    fn output_root(&self) -> &'static str {
        "registry"
    }

    fn render(&self, model: &IrModel, source_path: &str) -> Result<Vec<GeneratedFile>> {
        Ok(vec![json_file(
            "registry.json",
            registry_document(model, source_path),
        )])
    }
}

/// Builds the registry document. Public for tests; the bin goes through
/// [`EmitTarget::render`].
pub fn registry_document(model: &IrModel, source_path: &str) -> serde_json::Value {
    let components = {
        let mut items: Vec<&ComponentDefinition> = model.components.iter().collect();
        sort_by_id(&mut items, |component| component.id.as_str());
        items
    };

    serde_json::json!({
        "generated": generated_json(source_path),
        "components": components
            .iter()
            .map(|component| component_entry(component))
            .collect::<Vec<_>>(),
    })
}

fn component_entry(component: &ComponentDefinition) -> serde_json::Value {
    let mut capabilities: Vec<&poodle_ir::Capability> = component
        .capabilities
        .iter()
        .map(|requirement| &requirement.capability)
        .collect();
    // `Capability` derives Ord in the corpus's inventory order — a stable,
    // meaningful key, not declaration order.
    capabilities.sort_unstable();

    let mut shared_types: Vec<&str> = component
        .props
        .iter()
        .filter_map(|prop| match &prop.prop_type {
            PropType::Shared(id) => Some(id.as_str()),
            _ => None,
        })
        .collect();
    shared_types.sort_unstable();
    shared_types.dedup();

    // Axes in fixed struct order — size, density, orientation.
    let mut axes = Vec::new();
    if component.axes.size.is_some() {
        axes.push("size");
    }
    if component.axes.density.is_some() {
        axes.push("density");
    }
    if component.axes.orientation.is_some() {
        axes.push("orientation");
    }

    serde_json::json!({
        "id": component.id.as_str(),
        "name": component.name,
        "capabilities": capabilities,
        "axes": axes,
        "shared_types": shared_types,
    })
}
