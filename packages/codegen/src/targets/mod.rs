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
//! directories — same scoping, same select-only reachability. `g13-b041`
//! adds the component-scoped `button-ts` target (card 041 R2), which
//! renders Button's rendered vocabulary into both web packages; `g13-b042`
//! adds its Rust sibling `button-rust` (card 042 R2), which renders the
//! same definition into `poodle-render`'s `generated/` directory — same
//! scoping, same select-only reachability. `g13-b045` adds the
//! component-scoped `range-slider-ts` target, which renders RangeSlider's
//! rendered vocabulary into both web packages; `g13-b046` adds its Rust
//! sibling `range-slider-rust` (card 046 R2), which renders the same
//! definition into `poodle-render`'s `generated/` directory — same
//! scoping, same select-only reachability.

use crate::emit::EmitTarget;

mod json_common;

pub mod button;
pub mod button_rust;
pub mod conformance;
pub mod docs;
pub mod json;
pub mod range_slider;
pub mod range_slider_rust;
pub mod registry;
pub mod schema;
pub mod shell;
pub mod shell_rust;
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
    targets.push(&button::ButtonTarget);
    targets.push(&button_rust::ButtonRustTarget);
    targets.push(&range_slider::RangeSliderTarget);
    targets.push(&range_slider_rust::RangeSliderRustTarget);
    targets
}

/// Looks up a target by id.
pub fn by_id(id: &str) -> Option<&'static dyn EmitTarget> {
    selectable().into_iter().find(|target| target.id() == id)
}
