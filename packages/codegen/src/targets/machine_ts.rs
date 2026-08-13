//! TypeScript machine-interface target (spec 064 mechanism 1).
//!
//! Select-only, like `shell-scene`: not in [`super::all`], reachable only
//! via `--machine-interfaces` + `--target machine-ts`. Renders one file per
//! machine under `generated/machines/`. Not an [`crate::emit::EmitTarget`] —
//! the model is [`crate::machine_interfaces::Document`], not `IrModel`.

use crate::emit::{machine_header, GeneratedFile};
use crate::machine_interfaces::{machines_sorted, Document, Field, MachineInterface, Variant};

/// Target id accepted by `--target` in machine-interface mode.
pub const ID: &str = "machine-ts";

/// Output root relative to `--out` (the consuming package's `src/`).
pub const OUTPUT_ROOT: &str = "generated/machines";

/// Renders one TypeScript file per machine, sorted by id.
pub fn render(document: &Document, source_path: &str) -> Vec<GeneratedFile> {
    machines_sorted(document)
        .into_iter()
        .map(|machine| {
            GeneratedFile::new(
                format!("{}.ts", machine.id),
                render_machine(machine, source_path, document.schema_version),
            )
        })
        .collect()
}

fn render_machine(machine: &MachineInterface, source_path: &str, schema_version: u32) -> String {
    let mut out = machine_header(source_path, schema_version);
    out.push('\n');

    for named in &machine.named_types {
        out.push_str(&format!(
            "export type {} = {};\n\n",
            named.name,
            ts_union(&named.variants.iter().map(|m| m.ts.as_str()).collect::<Vec<_>>())
        ));
    }

    out.push_str(&format!(
        "export type {}State = {};\n\n",
        machine.prefix,
        ts_union(&machine.states.iter().map(|m| m.ts.as_str()).collect::<Vec<_>>())
    ));

    out.push_str(&format!("export interface {}Context {{\n", machine.prefix));
    for field in &machine.context.fields {
        out.push_str(&ts_field(field));
    }
    out.push_str("}\n\n");

    out.push_str(&format!(
        "export type {}Event =\n{};\n\n",
        machine.prefix,
        ts_variants(&machine.events.variants)
    ));

    out.push_str(&format!(
        "export type {}Effect =\n{};\n",
        machine.prefix,
        ts_variants(&machine.effects.variants)
    ));

    out
}

fn ts_union(members: &[&str]) -> String {
    members
        .iter()
        .map(|member| serde_json::to_string(member).expect("string serialization cannot fail"))
        .collect::<Vec<_>>()
        .join(" | ")
}

fn ts_field(field: &Field) -> String {
    let optional = if field.ts_optional { "?" } else { "" };
    format!("  {}{}: {};\n", field.ts_name, optional, field.ty.ts())
}

fn ts_variants(variants: &[Variant]) -> String {
    variants
        .iter()
        .map(|variant| {
            let disc = serde_json::to_string(&variant.ts_type).expect("string serialization cannot fail");
            if variant.fields.is_empty() {
                format!("  | {{ type: {disc} }}")
            } else {
                let mut body = format!("  | {{ type: {disc}");
                for field in &variant.fields {
                    body.push_str("; ");
                    body.push_str(&field.ts_name);
                    body.push_str(": ");
                    body.push_str(field.ty.ts());
                }
                body.push_str(" }");
                body
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
