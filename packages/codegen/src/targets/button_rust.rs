//! The button-rust target — the authored Button definition as a committed,
//! self-contained Rust artifact inside `poodle-render`, the package that
//! consumes it (card 042 R1/R1a: `poodle-render` may not depend on
//! `poodle-ir` or `poodle-codegen` during the pilot — the artifact is plain
//! data with no `use` of any Poodle crate, pulled in via `#[path]`, the
//! `poodle-tokens` mechanism g13-b003 R1 names).
//!
//! Output (per model, under the target's output root `generated`):
//!
//! - `<component-id>.rs` — the definition as a `pub static` of plain data
//!   structs: the shared-type member lists the render discriminates on
//!   (variants, tones, densities), the anatomy parts, the state attributes
//!   with their names, forms, emission policies, and value domains, and the
//!   recipe-hook chains. Components sort by id; one file per component.
//!
//! # R2 — a sibling target, not a repurposed one
//!
//! Card 042 R2: `button-ts`'s output is byte-compared by b041's tests, so
//! it is frozen. This target renders the **same definition** in Rust shape
//! — the same vocabulary projections, shared via the `pub(crate)` helpers
//! in [`super::button`] — so one authored change still moves every
//! artifact in one `ir:build`: the two web artifacts and this one.

use poodle_ir::{ComponentDefinition, Identifier, IrModel};

use crate::emit::{header, sort_by_id, EmitTarget, GeneratedFile};
use crate::error::Result;

use super::button::{
    attribute_values, emission_name, form_name, link_kind_name, part_class_name, part_instances,
};
use super::shell_rust::{rust_string_literal, static_name};

/// The button-rust target. Scoped to the authored Button model: not in
/// [`super::all`], so a plain `ir:build` over the synthetic fixture never
/// writes into a consumer package; reachable via `--target button-rust`.
pub struct ButtonRustTarget;

impl EmitTarget for ButtonRustTarget {
    fn id(&self) -> &'static str {
        "button-rust"
    }

    fn output_root(&self) -> &'static str {
        "generated"
    }

    fn render(&self, model: &IrModel, source_path: &str) -> Result<Vec<GeneratedFile>> {
        Ok(render_components(model, source_path))
    }
}

/// Renders one file per component, sorted by id. Public for tests; the bin
/// goes through [`EmitTarget::render`].
pub fn render_components(model: &IrModel, source_path: &str) -> Vec<GeneratedFile> {
    let mut components = model.components.clone();
    sort_by_id(&mut components, |component| component.id.as_str());

    components
        .into_iter()
        .map(|component| {
            let contents = render_component_file(model, &component, source_path);
            GeneratedFile::new(format!("{}.rs", component.id.as_str()), contents)
        })
        .collect()
}

/// The struct definitions every artifact carries. Emitted verbatim so each
/// artifact is self-contained — the consumer must not import anything to
/// read the definition. `dead_code` is allowed by design: the artifact is
/// the definition, and the render consumes the subset it implements.
const STRUCT_PRELUDE: &str = r#"#![allow(dead_code)]

//! The authored Button definition (spec 063 "Generated Artifact Contract"):
//! plain data, self-contained, no Poodle crate imports. Pulled into
//! `poodle-render` via `#[path = "generated/button.rs"]` — the
//! `poodle-tokens` mechanism (g13-b003 R1). Regenerate with
//! `effigy ir:build`; drift is gated by `effigy ir:check`.
//!
//! Card 042 R3: the render takes its vocabulary — variants, tones, the
//! state-attribute names and their value domains — from this definition
//! instead of its own literals; the web components read the same
//! vocabulary from the `button-ts` artifact (card 041 R2). One definition
//! change moves every artifact and every runtime in one `ir:build`.

/// One anatomy part: id, display name, the DOM class the web markup
/// renders it under, its parent part, and — for an identified family
/// (g13.018 R5) — the ids of its instances. The class projection is
/// shared with the `button-ts` artifact (`part_class_name`).
pub struct ButtonPart {
    pub id: &'static str,
    pub name: &'static str,
    pub dom_class: &'static str,
    pub parent: Option<&'static str>,
    /// The instance ids of an identified family, or `None` for any other
    /// part kind. The count and the identities come from the definition.
    pub instances: Option<&'static [&'static str]>,
}

/// One state attribute: id, the `data-*` name the DOM carries, its form
/// and emission policy, and its value domain. The value domain is the
/// emitter's projection of the source prop's declared type, minus the
/// default member under omit-when-default (the DOM never carries it).
pub struct ButtonAttribute {
    pub id: &'static str,
    pub name: &'static str,
    pub form: &'static str,
    pub emission: &'static str,
    pub values: Option<&'static [&'static str]>,
}

/// One link in a recipe-hook override chain.
pub struct ButtonRecipeLink {
    pub kind: &'static str,
    pub target: &'static str,
}

/// One recipe hook: the `--poodle-recipe-*` name and its override chain.
pub struct ButtonRecipeHook {
    pub hook: &'static str,
    pub chain: &'static [ButtonRecipeLink],
}

/// The authored Button definition — plain data, no Poodle crate imports.
pub struct ButtonDefinition {
    pub id: &'static str,
    pub name: &'static str,
    /// The `button-variant` shared-type members — the `data-variant` value
    /// domain, and the variant treatments the render discriminates on.
    pub variants: &'static [&'static str],
    /// The `button-tone` shared-type members — the tone vocabulary the
    /// render's status treatment discriminates on.
    pub tones: &'static [&'static str],
    /// The `control-density` shared-type members — the density vocabulary
    /// the render's metric treatments discriminate on.
    pub densities: &'static [&'static str],
    pub parts: &'static [ButtonPart],
    pub attributes: &'static [ButtonAttribute],
    pub recipe_hooks: &'static [ButtonRecipeHook],
}

"#;

/// The member ids of a shared type, or an empty slice when the definition
/// does not declare it (unreachable for the authored Button model; the
/// render falls back to its default arms).
fn shared_members(model: &IrModel, id: &str) -> Vec<String> {
    super::button::shared_member_names(model, &Identifier::from(id)).unwrap_or_default()
}

fn render_component_file(
    model: &IrModel,
    component: &ComponentDefinition,
    source_path: &str,
) -> String {
    let mut out = header(source_path);
    out.push_str(STRUCT_PRELUDE);

    let static_name = format!("{}_DEFINITION", static_name(component.id.as_str()));

    out.push_str(&format!(
        "pub static {static_name}: ButtonDefinition = ButtonDefinition {{\n"
    ));
    out.push_str(&format!(
        "    id: {},\n",
        rust_string_literal(component.id.as_str())
    ));
    out.push_str(&format!(
        "    name: {},\n",
        rust_string_literal(&component.name)
    ));

    // The shared-type member lists (card 042 R3): the variants, tones, and
    // densities the render discriminates on. Authoring order, like the
    // value domains of the corresponding attributes.
    for (field, shared_id) in [
        ("variants", "button-variant"),
        ("tones", "button-tone"),
        ("densities", "control-density"),
    ] {
        let members = shared_members(model, shared_id);
        let entries = members
            .iter()
            .map(|member| rust_string_literal(member))
            .collect::<Vec<_>>()
            .join(", ");
        out.push_str(&format!("    {field}: &[{entries}],\n"));
    }

    // Parts — the anatomy with the DOM class projection, shared with the
    // `button-ts` artifact (R2). Each entry is multi-line so the emitted
    // artifact is rustfmt-clean (the shell-rust precedent). Identified
    // families (g13.018 R5) also carry their instance list.
    out.push_str("    parts: &[\n");
    for part in &component.parts {
        out.push_str(&format!(
            "        ButtonPart {{\n            id: {},\n            name: {},\n            dom_class: {},\n            parent: {},\n            instances: {},\n        }},\n",
            rust_string_literal(part.id.as_str()),
            rust_string_literal(&part.name),
            rust_string_literal(&part_class_name(part.id.as_str())),
            part.parent
                .as_ref()
                .map(|parent| format!("Some({})", rust_string_literal(parent.as_str())))
                .unwrap_or_else(|| "None".to_owned()),
            match part_instances(part) {
                Some(instances) => {
                    let entries = instances
                        .iter()
                        .map(|instance| rust_string_literal(instance))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("Some(&[{entries}])")
                }
                None => "None".to_owned(),
            }
        ));
    }
    out.push_str("    ],\n");

    // State attributes — names, forms, emission policies, value domains;
    // the same entries the web components read (R2/R3).
    out.push_str("    attributes: &[\n");
    for attribute in &component.attributes {
        out.push_str("        ButtonAttribute {\n");
        out.push_str(&format!(
            "            id: {},\n            name: {},\n            form: {},\n            emission: {},\n",
            rust_string_literal(attribute.id.as_str()),
            rust_string_literal(&attribute.name),
            rust_string_literal(form_name(attribute.form)),
            rust_string_literal(emission_name(attribute.emission))
        ));
        if let Some(values) = attribute_values(model, component, attribute) {
            let values = values
                .iter()
                .map(|value| rust_string_literal(value))
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!("            values: Some(&[{values}]),\n"));
        } else {
            out.push_str("            values: None,\n");
        }
        out.push_str("        },\n");
    }
    out.push_str("    ],\n");

    // Recipe hooks — the `--poodle-recipe-*` override chains (BTN-22),
    // carried so the definition is the single record; the web styling seam
    // `button.css` consumes them (card 041).
    out.push_str("    recipe_hooks: &[\n");
    for hook in &component.recipe_hooks {
        out.push_str(&format!(
            "        ButtonRecipeHook {{\n            hook: {},\n            chain: &[\n",
            rust_string_literal(&hook.hook)
        ));
        for link in &hook.chain {
            out.push_str(&format!(
                "                ButtonRecipeLink {{\n                    kind: {},\n                    target: {},\n                }},\n",
                rust_string_literal(link_kind_name(link.kind)),
                rust_string_literal(&link.target)
            ));
        }
        out.push_str("            ],\n        },\n");
    }
    out.push_str("    ],\n");

    out.push_str("};\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definition_static_name_upper_snakes_with_suffix() {
        assert_eq!(
            format!("{}_DEFINITION", static_name("button")),
            "BUTTON_DEFINITION"
        );
    }
}
