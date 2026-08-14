//! Target registry. `g13-b022` shipped the TypeScript target; `g13-b025`
//! added the four the milestone names — JSON surface, JSON Schema,
//! registry, conformance vectors, docs fragments — in stable (id-sorted)
//! order. `g13-b035` adds the scene-scoped `shell-scene` target, which is
//! NOT in [`all`]: it renders the authored shell model into the consuming
//! web packages' `generated/` directories (g13-b003 R1 "Generated output
//! location"), and a plain `ir:build` over the synthetic fixture must never
//! write into a web package. It is reachable only via `--target
//! shell-scene`. `g13-b036` adds its Rust sibling `shell-rust` (card 036
//! R2), which renders the same scene into the native previews' `generated/`
//! directories — same scoping, same select-only reachability. The three
//! component targets were unwound in g13-053. `g14-b004` adds `machine-ts` /
//! `machine-rust`, also select-only, but they consume the machine-interface
//! schema rather than an `IrModel` and are not registered here — the CLI
//! reaches them via `--machine-interfaces`.

use crate::emit::EmitTarget;

mod json_common;

pub mod catalogue_rust;
pub mod catalogue_ts;
pub mod conformance;
pub mod conformance_rust;
pub mod docs;
pub mod json;
pub mod machine_rust;
pub mod machine_ts;
pub mod registry;
pub mod schema;
pub mod shell;
pub mod shell_rust;
pub mod specimen_rust;
pub mod specimen_ts;
pub mod ts;

/// Every registered target, in stable order — the default set a plain
/// `poodle-codegen <FIXTURE> --out <DIR>` run emits.
pub fn all() -> Vec<&'static dyn EmitTarget> {
    vec![
        &conformance::ConformanceTarget,
        &docs::DocsFragmentsTarget,
        &json::JsonSurfaceTarget,
        &registry::RegistryTarget,
        &schema::JsonSchemaTarget,
        &ts::TypeScriptTarget,
    ]
}

/// Every target selectable via `--target`, including scoped targets.
pub fn selectable() -> Vec<&'static dyn EmitTarget> {
    let mut targets = all();
    targets.push(&shell::ShellSceneTarget);
    targets.push(&shell_rust::ShellRustTarget);
    targets.push(&specimen_ts::SpecimenTsTarget);
    targets.push(&specimen_rust::SpecimenRustTarget);
    targets
}

/// Looks up a target by id.
pub fn by_id(id: &str) -> Option<&'static dyn EmitTarget> {
    selectable().into_iter().find(|target| target.id() == id)
}
