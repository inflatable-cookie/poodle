//! The TypeScript target — types for a component's prop surface, emitted
//! from an `IrModel` instance.
//!
//! Output (per model, under the target's output root):
//!
//! - `shared-types.ts` — one union type per first-class shared enumerated
//!   type, members in authoring order (the canonical definition order),
//!   shared types themselves sorted by id.
//! - `<component-id>.ts` — one `export type <Name>Props = Readonly<{…}>`
//!   per component, props in declaration order (the contract's prop order),
//!   components sorted by id. The file imports the shared types it
//!   references.
//! - `index.ts` — re-exports, sorted.
//!
//! Emission rules (the emitter owns every byte, ruling R2 — no formatter
//! ever runs over this output):
//!
//! - prop type mapping: `String` → `string`, `Number` → `number`,
//!   `Bool` → `boolean`, `Pair(t)` → `[t, t]`, `List(t)` → `Array<t>`,
//!   `Opaque` → `unknown`.
//! - a `Shared` prop with a permitted subset emits the **subset union**
//!   inline — the R6.2 constraint survives into the artifact; without a
//!   subset it references the shared type by name.
//! - required props are non-optional; everything else is optional.
//! - defaults and web-only marks land in the prop's doc comment; the type
//!   surface stays declarative.
//! - string literals use JSON escaping and numbers a fixed, locale-
//!   independent formatter (the `formatF32` rule from the token emitter,
//!   applied to TypeScript).
//!
//! The output is framework-free by construction: primitives, unions,
//! tuples, and `unknown` only — no Svelte, React, or DOM types.

use poodle_ir::{ComponentDefinition, Identifier, IrModel, Prop, PropType, SharedType, Value};

use crate::emit::{header, sort_by_id, EmitTarget, GeneratedFile};
use crate::error::{CodegenError, Result};

/// The one target this card ships.
pub struct TypeScriptTarget;

impl EmitTarget for TypeScriptTarget {
    fn id(&self) -> &'static str {
        "typescript"
    }

    fn output_root(&self) -> &'static str {
        "ts"
    }

    fn render(&self, model: &IrModel, source_path: &str) -> Result<Vec<GeneratedFile>> {
        render_model(model, source_path)
    }
}

/// Renders a model. Public for tests; the bin goes through
/// [`EmitTarget::render`].
pub fn render_model(model: &IrModel, source_path: &str) -> Result<Vec<GeneratedFile>> {
    let header = header(source_path);

    let shared_types = {
        let mut items: Vec<&SharedType> = model.shared_types.iter().collect();
        sort_by_id(&mut items, |shared| shared.id.as_str());
        items
    };

    let components = {
        let mut items: Vec<&ComponentDefinition> = model.components.iter().collect();
        sort_by_id(&mut items, |component| component.id.as_str());
        items
    };

    let mut files = Vec::new();

    // Shared types: one union per type, authoring order preserved.
    let mut shared_ts = String::new();
    for shared in &shared_types {
        shared_ts.push_str(&render_shared_type(shared));
        shared_ts.push('\n');
    }
    files.push(GeneratedFile::new(
        "shared-types.ts",
        format!("{header}{shared_ts}"),
    ));

    // One file per component.
    for component in &components {
        files.push(GeneratedFile::new(
            format!("{}.ts", component.id),
            render_component_file(model, component, &header)?,
        ));
    }

    // Index: re-exports, sorted.
    let mut index = String::from("export * from \"./shared-types\";\n");
    for component in &components {
        index.push_str(&format!("export * from \"./{}\";\n", component.id));
    }
    files.push(GeneratedFile::new("index.ts", format!("{header}{index}")));

    Ok(files)
}

fn render_shared_type(shared: &SharedType) -> String {
    let mut out = String::from("export type ");
    out.push_str(&shared.name);
    out.push_str(" =\n");
    for member in &shared.members {
        out.push_str(&format!("  | {}\n", ts_string_literal(member.id.as_str())));
    }
    out.push(';');
    out
}

fn render_component_file(
    model: &IrModel,
    component: &ComponentDefinition,
    header: &str,
) -> Result<String> {
    let mut out = String::from(header);

    // Import every shared type the component's props reference (without a
    // permitted subset, which inlines the union). Sorted for determinism.
    let mut imports: Vec<&str> = component
        .props
        .iter()
        .filter_map(|prop| match &prop.prop_type {
            PropType::Shared(id) if prop.permitted_subset.is_none() => {
                Some(shared_type_name(model, id).ok()?)
            }
            _ => None,
        })
        .collect();
    imports.sort_unstable();
    imports.dedup();
    if !imports.is_empty() {
        out.push_str("import type { ");
        out.push_str(&imports.join(", "));
        out.push_str(" } from \"./shared-types\";\n\n");
    }

    out.push_str(&format!(
        "export type {}Props = Readonly<{{\n",
        component.name
    ));
    for prop in &component.props {
        let rendered = render_prop(model, prop)?;
        out.push_str("  /**\n");
        out.push_str(&format!(
            "   * {}\n",
            collapse_whitespace(&prop.description)
        ));
        if let Some(default) = &prop.default {
            out.push_str(&format!("   * Default: {}\n", ts_value_literal(default)));
        } else if prop.default_expr.is_some() {
            out.push_str("   * Default: derived from a bounded expression (spec 063).\n");
        }
        if prop.web_only {
            out.push_str("   * Web-only (CROSS-03): excluded from the portable spec surface.\n");
        }
        out.push_str("   */\n");
        out.push_str(&format!("  {rendered};\n"));
    }
    out.push_str("}>\n");
    Ok(out)
}

/// Renders one prop's name (with optionality) + type, without the doc
/// comment.
fn render_prop(model: &IrModel, prop: &Prop) -> Result<String> {
    let name = prop.name.as_str();
    let ty = render_prop_type(model, &prop.prop_type, prop.permitted_subset.as_ref())?;
    Ok(format!(
        "{name}{}: {ty}",
        if prop.required { "" } else { "?" }
    ))
}

fn render_prop_type(
    model: &IrModel,
    prop_type: &PropType,
    subset: Option<&poodle_ir::PermittedSubset>,
) -> Result<String> {
    match prop_type {
        PropType::String => Ok("string".to_owned()),
        PropType::Number => Ok("number".to_owned()),
        PropType::Bool => Ok("boolean".to_owned()),
        PropType::Shared(id) => {
            if let Some(subset) = subset {
                // R6.2: the component's permitted subset survives into the
                // artifact as the prop's actual type.
                let members: Vec<String> = subset
                    .members
                    .iter()
                    .map(|member| ts_string_literal(member.as_str()))
                    .collect();
                Ok(members.join(" | "))
            } else {
                Ok(shared_type_name(model, id)?.to_owned())
            }
        }
        PropType::Pair(inner) => {
            let inner = render_prop_type(model, inner, None)?;
            Ok(format!("[{inner}, {inner}]"))
        }
        PropType::List(inner) => {
            let inner = render_prop_type(model, inner, None)?;
            Ok(format!("Array<{inner}>"))
        }
        PropType::Opaque => Ok("unknown".to_owned()),
    }
}

/// Resolves a shared-type id to its TS type name. A validated model always
/// resolves; the error path exists so emission never panics.
fn shared_type_name<'a>(model: &'a IrModel, id: &Identifier) -> Result<&'a str> {
    model
        .shared_type(id.as_str())
        .map(|shared| shared.name.as_str())
        .ok_or_else(|| CodegenError::UnresolvedReference {
            what: format!("shared type '{id}'"),
        })
}

/// A JSON-escaped TS string literal — locale-independent, the same escaping
/// the token emitter uses (`JSON.stringify`).
fn ts_string_literal(value: &str) -> String {
    serde_json::to_string(value).expect("string serialization cannot fail")
}

/// A TS literal for a default value. `f64` formatting is Rust's Display —
/// locale-independent, no exponent notation, shortest round-trip.
fn ts_value_literal(value: &Value) -> String {
    match value {
        Value::String(s) => ts_string_literal(s),
        Value::Number(n) => format_number(*n),
        Value::Bool(b) => b.to_string(),
        Value::Member(id) => {
            // A member default renders as the member id (the value props
            // cite), not the Rust-style name.
            ts_string_literal(id.as_str())
        }
        Value::Pair(a, b) => {
            format!("[{}, {}]", ts_value_literal(a), ts_value_literal(b))
        }
        Value::List(items) => {
            let items: Vec<String> = items.iter().map(ts_value_literal).collect();
            format!("[{}]", items.join(", "))
        }
        Value::Null => "null".to_owned(),
    }
}

/// Fixed number formatting: integral values print without a fractional
/// part, everything else shortest round-trip — never exponent notation,
/// never locale-dependent.
fn format_number(n: f64) -> String {
    if n.fract() == 0.0 && n.abs() < 1e15 {
        format!("{}", n as i64)
    } else {
        format!("{n}")
    }
}

/// Collapses runs of whitespace (including newlines in contract prose) to a
/// single space so doc comments stay one line.
fn collapse_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}
