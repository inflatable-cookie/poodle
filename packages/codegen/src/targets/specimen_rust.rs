//! The specimen Rust target — the display-specimen scenes as a committed,
//! self-contained Rust artifact inside the consuming native previews
//! (card 036 R1: no `poodle-ir` / `poodle-codegen` dependency in either
//! native manifest — the artifact is plain data with no `use` of any Poodle
//! crate, pulled in by the hosts via `#[path = "generated/specimens.rs"]`,
//! the `poodle-tokens` mechanism g13-b003 R1 names).
//!
//! Output (per model, under the target's output root `generated`):
//!
//! - `specimens.rs` — one file for every display-specimen scene: the
//!   fixture structs (a static table, unlike `shell-rust`'s per-scene
//!   files, because the struct definitions must live in exactly one
//!   `#[path]`-included module) and `pub static SPECIMEN_SCENES`.
//!
//! Values are serialized as their string forms (`Member` ids, `String`
//! content, `"true"`/`"false"`, formatted numbers); each native renderer
//! parses them into its own spec types.

use poodle_ir::{IrModel, Value};

use crate::emit::{header, EmitTarget, GeneratedFile};
use crate::error::Result;

use super::ts::format_number;

/// The specimen Rust target. Scoped like `shell-rust`: not in
/// [`super::all`]; reachable via `--target specimen-rust`.
pub struct SpecimenRustTarget;

impl EmitTarget for SpecimenRustTarget {
    fn id(&self) -> &'static str {
        "specimen-rust"
    }

    fn output_root(&self) -> &'static str {
        "generated/specimens"
    }

    fn render(&self, model: &IrModel, source_path: &str) -> Result<Vec<GeneratedFile>> {
        Ok(vec![GeneratedFile::new(
            "specimens.rs".to_owned(),
            render_specimens(model, source_path),
        )])
    }
}

/// The value in its string form — always a quoted Rust string literal: the
/// fixture's value fields are `&'static str`, and each renderer parses the
/// string into its own spec types.
fn value_str(value: &Value) -> String {
    let form = match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => format_number(*n),
        Value::Bool(b) => b.to_string(),
        Value::Member(id) => id.to_string(),
        Value::Null => "null".to_owned(),
        Value::Pair(a, b) => format!("({}, {})", value_str(a), value_str(b)),
        Value::List(items) => format!(
            "[{}]",
            items.iter().map(value_str).collect::<Vec<_>>().join(", ")
        ),
    };
    ts_string_literal(&form)
}

fn ts_string_literal(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

/// Renders the full fixture: structs once, then every scene. Public for
/// tests; the bin goes through [`EmitTarget::render`].
pub fn render_specimens(model: &IrModel, source_path: &str) -> String {
    let mut scenes = model.scenes.clone();
    scenes.sort_by(|a, b| a.id.as_str().cmp(b.id.as_str()));

    let mut out = header(source_path);
    out.push_str("#![allow(dead_code)]\n\n");
    out.push_str(
        "//! The display-specimen scenes (tranche one, g14-b005): plain data,\n\
         //! self-contained, no Poodle crate imports. Pulled into the host\n\
         //! previews via `#[path = \"generated/specimens.rs\"]`. Values are\n\
         //! string forms; each renderer parses them into its own spec types.\n\n",
    );

    out.push_str(
        "/// One prop binding: the prop id and its string-form value.\n\
         #[derive(Clone)]\n\
         pub struct SpecimenProp<'a> {\n\
         \x20   pub prop: &'a str,\n\
         \x20   pub value: &'a str,\n\
         }\n\n\
         /// One component instance inside a specimen group.\n\
         pub struct SpecimenInstance<'a> {\n\
         \x20   pub component: &'a str,\n\
         \x20   pub caption: Option<&'a str>,\n\
         \x20   pub props: &'a [SpecimenProp<'a>],\n\
         }\n\n\
         /// One specimen section heading with its instances.\n\
         pub struct SpecimenGroup<'a> {\n\
         \x20   pub label: &'a str,\n\
         \x20   pub instances: &'a [SpecimenInstance<'a>],\n\
         }\n\n\
         /// A display-specimen scene.\n\
         pub struct SpecimenScene<'a> {\n\
         \x20   pub id: &'a str,\n\
         \x20   pub name: &'a str,\n\
         \x20   pub description: &'a str,\n\
         \x20   pub size_axis: &'a [&'a str],\n\
         \x20   pub density_axis: &'a [&'a str],\n\
         \x20   pub groups: &'a [SpecimenGroup<'a>],\n\
         }\n\n",
    );

    out.push_str("pub static SPECIMEN_SCENES: &[SpecimenScene<'static>] = &[\n");
    for scene in &scenes {
        out.push_str(&render_scene(scene));
    }
    out.push_str("];\n");

    out
}

fn render_axis(scene: &poodle_ir::Scene, kind: poodle_ir::SceneAxisKind) -> String {
    scene
        .axes
        .iter()
        .find(|axis| axis.kind == kind)
        .and_then(|axis| match &axis.values {
            poodle_ir::AxisValues::Named(values) => Some(values),
            poodle_ir::AxisValues::Continuous { .. } => None,
        })
        .map(|values| {
            values
                .iter()
                .map(|value| ts_string_literal(value.as_str()))
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_default()
}

fn render_scene(scene: &poodle_ir::Scene) -> String {
    let mut out = String::new();
    out.push_str("    SpecimenScene {\n");
    out.push_str(&format!("        id: {},\n", ts_string_literal(scene.id.as_str())));
    out.push_str(&format!("        name: {},\n", ts_string_literal(&scene.name)));
    out.push_str(&format!(
        "        description: {},\n",
        ts_string_literal(&scene.description)
    ));
    out.push_str(&format!(
        "        size_axis: &[{}],\n",
        render_axis(scene, poodle_ir::SceneAxisKind::Size)
    ));
    out.push_str(&format!(
        "        density_axis: &[{}],\n",
        render_axis(scene, poodle_ir::SceneAxisKind::Density)
    ));
    out.push_str("        groups: &[\n");

    let mut current_group: Option<&str> = None;
    let mut first = true;
    for instance in &scene.instances {
        let group = instance.group.as_deref().unwrap_or("");
        if current_group != Some(group) {
            if !first {
                out.push_str("        ],\n            },\n");
            }
            first = false;
            out.push_str(&format!(
                "            SpecimenGroup {{ label: {}, instances: &[\n",
                ts_string_literal(group)
            ));
            current_group = Some(group);
        }
        out.push_str("                SpecimenInstance {\n");
        out.push_str(&format!(
            "                    component: {},\n",
            ts_string_literal(instance.component.as_str())
        ));
        out.push_str(&format!(
            "                    caption: {},\n",
            instance
                .caption
                .as_deref()
                .map(|caption| format!("Some({})", ts_string_literal(caption)))
                .unwrap_or_else(|| "None".to_owned())
        ));
        out.push_str("                    props: &[\n");
        for binding in &instance.bindings {
            out.push_str(&format!(
                "                        SpecimenProp {{ prop: {}, value: {} }},\n",
                ts_string_literal(binding.prop.as_str()),
                value_str(&binding.value)
            ));
        }
        out.push_str("                    ],\n");
        out.push_str("                },\n");
    }
    if !first {
        out.push_str("        ],\n            },\n");
    }
    out.push_str("        ],\n");
    out.push_str("    },\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_literals_escape_quotes_and_backslashes() {
        assert_eq!(ts_string_literal("a\"b\\c"), "\"a\\\"b\\\\c\"");
    }

    #[test]
    fn value_str_forms_match_the_native_parse_surface() {
        assert_eq!(value_str(&Value::string("x")), "\"x\"");
        assert_eq!(value_str(&Value::boolean(true)), "\"true\"");
        assert_eq!(value_str(&Value::member("danger")), "\"danger\"");
        assert_eq!(value_str(&Value::number(1.5)), "\"1.5\"");
    }
}
