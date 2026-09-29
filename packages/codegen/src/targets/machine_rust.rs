//! Rust machine-interface target (spec 064 mechanism 1).
//!
//! Select-only sibling of [`super::machine_ts`]: not in [`super::all`],
//! reachable only via `--machine-interfaces` + `--target machine-rust`.
//! Hosts pull each file in with `#[path = "generated/machines/<id>.rs"]`.

use crate::emit::{machine_header, GeneratedFile};
use crate::machine_interfaces::{
    machines_sorted, Context, Document, Field, MachineInterface, NamedType, Variant, VariantSet,
};

/// Target id accepted by `--target` in machine-interface mode.
pub const ID: &str = "machine-rust";

/// Output root relative to `--out` (the consuming crate's `src/`).
pub const OUTPUT_ROOT: &str = "generated/machines";

/// Renders one Rust file per machine, sorted by id.
pub fn render(document: &Document, source_path: &str) -> Vec<GeneratedFile> {
    machines_sorted(document)
        .into_iter()
        .map(|machine| {
            GeneratedFile::new(
                format!("{}.rs", machine.id),
                render_machine(machine, source_path, document.schema_version),
            )
        })
        .collect()
}

fn render_machine(machine: &MachineInterface, source_path: &str, schema_version: u32) -> String {
    let mut out = machine_header(source_path, schema_version);
    out.push_str("#![cfg_attr(rustfmt, rustfmt::skip)]\n\n");

    for named in &machine.named_types {
        out.push_str(&render_named(named));
        out.push('\n');
    }

    out.push_str(&render_state_enum(machine));
    out.push('\n');
    out.push_str(&render_context(&machine.prefix, &machine.context));
    out.push('\n');
    out.push_str(&render_variant_enum(&format!("{}Event", machine.prefix), &machine.events));
    out.push('\n');
    out.push_str(&render_variant_enum(
        &format!("{}Effect", machine.prefix),
        &machine.effects,
    ));

    out
}

fn render_named(named: &NamedType) -> String {
    let mut out = format!("{}\n", derive_line(named.copy, named.eq, false));
    out.push_str(&format!("pub enum {} {{\n", named.name));
    for member in &named.variants {
        out.push_str(&format!("    {},\n", member.rs));
    }
    out.push_str("}\n");
    out
}

fn render_state_enum(machine: &MachineInterface) -> String {
    let mut out = format!("{}\n", derive_line(true, true, false));
    out.push_str(&format!("pub enum {}State {{\n", machine.prefix));
    for member in &machine.states {
        out.push_str(&format!("    {},\n", member.rs));
    }
    out.push_str("}\n");
    out
}

fn render_context(prefix: &str, context: &Context) -> String {
    let mut out = format!(
        "{}\n",
        derive_line(context.copy, context.eq, context.rs_default)
    );
    out.push_str(&format!("pub struct {}Context {{\n", prefix));
    for field in &context.fields {
        out.push_str(&format!("    pub {}: {},\n", field.rs_name, field.ty.rs()));
    }
    out.push_str("}\n");
    out
}

fn render_variant_enum(name: &str, set: &VariantSet) -> String {
    let mut out = format!("{}\n", derive_line(set.copy, set.eq, false));
    out.push_str(&format!("pub enum {name} {{\n"));
    for variant in &set.variants {
        out.push_str(&render_variant(variant));
    }
    out.push_str("}\n");
    out
}

fn render_variant(variant: &Variant) -> String {
    if variant.fields.is_empty() {
        format!("    {},\n", variant.rs_variant)
    } else {
        let fields = variant
            .fields
            .iter()
            .map(rs_payload_field)
            .collect::<Vec<_>>()
            .join(", ");
        format!("    {} {{ {fields} }},\n", variant.rs_variant)
    }
}

fn rs_payload_field(field: &Field) -> String {
    format!("{}: {}", field.rs_name, field.ty.rs())
}

fn derive_line(copy: bool, eq: bool, rs_default: bool) -> String {
    let mut parts = vec!["Debug", "Clone"];
    if copy {
        parts.push("Copy");
    }
    parts.push("PartialEq");
    if eq {
        parts.push("Eq");
    }
    if rs_default {
        parts.push("Default");
    }
    format!("#[derive({})]", parts.join(", "))
}
