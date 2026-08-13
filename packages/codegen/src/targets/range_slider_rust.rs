//! The range-slider-rust target — the authored RangeSlider definition as a
//! committed, self-contained Rust artifact inside `poodle-render`, the
//! package that consumes it (card 046 R1/R1a: `poodle-render` may not
//! depend on `poodle-ir` or `poodle-codegen` during the pilot — the
//! artifact is plain data with no `use` of any Poodle crate, pulled in via
//! `#[path]`, the `poodle-tokens` mechanism g13-b003 R1 names).
//!
//! Output (per model, under the target's output root
//! `generated/range-slider`):
//!
//! - `index.rs` — the definition as a `pub static` of plain data
//!   structs: the shared-type member lists the render discriminates on
//!   (variants, polarities, sizes, densities), the anatomy parts, the
//!   eight state attributes with their names, forms, emission policies,
//!   and value domains, the seven RNG-17 geometry hooks as `style_props`
//!   with the VisualState field that feeds each, and the recipe-hook
//!   chains. One file per component model.
//!
//! # R2 — a sibling target, not a repurposed one
//!
//! Card 046 R2: `range-slider-ts`'s output is byte-compared by b045's
//! tests, so it is frozen. This target renders the **same definition** in
//! Rust shape — the same vocabulary projections, shared via the
//! `pub(crate)` helpers in [`super::button`] (attributes, domains, chain
//! kinds, visual-field sources) and [`super::range_slider`] (the part-class
//! projection) — so one authored change still moves every artifact in one
//! `ir:build`: the two web artifacts and this one.

use poodle_ir::{ComponentDefinition, Identifier, IrModel};

use crate::emit::{header, sort_by_id, EmitTarget, GeneratedFile};
use crate::error::Result;

use super::button::{
    attribute_values, emission_name, form_name, link_kind_name, shared_member_names,
    value_visual_field,
};
use super::range_slider::part_class_name;
use super::shell_rust::{rust_string_literal, static_name};

/// The range-slider-rust target. Scoped to the authored RangeSlider model:
/// not in [`super::all`], so a plain `ir:build` over the synthetic fixture
/// never writes into a consumer package; reachable via `--target
/// range-slider-rust`.
pub struct RangeSliderRustTarget;

impl EmitTarget for RangeSliderRustTarget {
    fn id(&self) -> &'static str {
        "range-slider-rust"
    }

    // Its own nested root inside the shared `generated/` directory of
    // poodle-render — the same disjoint-roots layout card 041 established
    // for shared `generated/` directories (write.rs: a nested directory is
    // another target's root; the top level belongs to button-rust, whose
    // orphan sweep would delete a sibling's file).
    fn output_root(&self) -> &'static str {
        "generated/range-slider"
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
            // `index.rs` inside the target's nested root
            // (`generated/range-slider/`) so the module include names the
            // directory, not a doubled file name — the same shape as the
            // `range-slider-ts` artifact (`generated/range-slider/index.ts`).
            GeneratedFile::new("index.rs", contents)
        })
        .collect()
}

/// The struct definitions every artifact carries. Emitted verbatim so each
/// artifact is self-contained — the consumer must not import anything to
/// read the definition. `dead_code` is allowed by design: the artifact is
/// the definition, and the render consumes the subset it implements.
const STRUCT_PRELUDE: &str = r#"#![allow(dead_code)]

//! The authored RangeSlider definition (spec 063 "Generated Artifact
//! Contract"): plain data, self-contained, no Poodle crate imports. Pulled
//! `poodle-render` via `#[path = "generated/range-slider/index.rs"]` — the
//! `poodle-tokens` mechanism (g13-b003 R1). Regenerate with
//! `effigy ir:build`; drift is gated by `effigy ir:check`.
//!
//! Card 046 R3: the render takes its vocabulary — the variant treatments,
//! the two-thumb anatomy, the state-attribute names, and the fill-geometry
//! hooks — from this definition instead of its own literals; the web
//! components read the same vocabulary from the `range-slider-ts` artifact
//! (card 045 R2). One definition change moves every artifact and every
//! runtime in one `ir:build`.

/// One anatomy part: id, display name, the DOM class the web markup
/// renders it under, and its parent part. The class projection is shared
/// with the `range-slider-ts` artifact (`part_class_name`).
pub struct RangeSliderPart {
    pub id: &'static str,
    pub name: &'static str,
    pub dom_class: &'static str,
    pub parent: Option<&'static str>,
}

/// One state attribute: id, the `data-*` name the DOM carries, its form
/// and emission policy, and its value domain. The value domain is the
/// emitter's projection of the source's declared type (a shared prop, a
/// boolean, or a VisualState enum field); expression-valued attributes
/// (e.g. `data-state`) carry no domain.
pub struct RangeSliderAttribute {
    pub id: &'static str,
    pub name: &'static str,
    pub form: &'static str,
    pub emission: &'static str,
    pub values: Option<&'static [&'static str]>,
}

/// One RNG-17 geometry hook: the computed custom property the web markup
/// emits as inline style, and the VisualState field that feeds it. The
/// values are machine-computed arithmetic (`norm * 100%`) — the names and
/// their source fields are the declared vocabulary.
pub struct RangeSliderStyleProp {
    pub id: &'static str,
    pub name: &'static str,
    pub source: &'static str,
}

/// One link in a recipe-hook override chain.
pub struct RangeSliderRecipeLink {
    pub kind: &'static str,
    pub target: &'static str,
}

/// One recipe hook: the `--poodle-recipe-*` name and its override chain.
pub struct RangeSliderRecipeHook {
    pub hook: &'static str,
    pub chain: &'static [RangeSliderRecipeLink],
}

/// The authored RangeSlider definition — plain data, no Poodle crate
/// imports.
pub struct RangeSliderDefinition {
    pub id: &'static str,
    pub name: &'static str,
    /// The `slider-variant` shared-type members — the `data-variant` value
    /// domain, and the standard/embedded treatments the render
    /// discriminates on.
    pub variants: &'static [&'static str],
    /// The `slider-polarity` shared-type members — the `data-polarity`
    /// value domain, and the bipolar fill-split treatment.
    pub polarities: &'static [&'static str],
    /// The `control-size` shared-type members — the `data-size` value
    /// domain, and the size ladder the render's metric treatments
    /// discriminate on.
    pub sizes: &'static [&'static str],
    /// The `control-density` shared-type members — the `data-density`
    /// value domain.
    pub densities: &'static [&'static str],
    pub parts: &'static [RangeSliderPart],
    pub attributes: &'static [RangeSliderAttribute],
    pub style_props: &'static [RangeSliderStyleProp],
    pub recipe_hooks: &'static [RangeSliderRecipeHook],
}

"#;

/// The member ids of a shared type, or an empty slice when the definition
/// does not declare it (unreachable for the authored RangeSlider model;
/// the render falls back to its default arms).
fn shared_members(model: &IrModel, id: &str) -> Vec<String> {
    shared_member_names(model, &Identifier::from(id)).unwrap_or_default()
}

/// A Rust string literal that stays within rustfmt's 100-column line: short
/// values emit as one literal, long ones (the embedded-control class pairs)
/// as a single backslash-continuation literal — one string, no adjacent
/// literals (which do not parse in field position), rustfmt-clean (the
/// shell-rust/button-rust precedent).
fn rust_wrapped_literal(value: &str) -> String {
    let literal = rust_string_literal(value);
    if literal.len() <= 72 {
        return literal;
    }
    let mut out = String::from("\"");
    for (i, part) in value.split(' ').enumerate() {
        if i > 0 {
            out.push_str(" \\\n         ");
        }
        out.push_str(part);
    }
    out.push('"');
    out
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
        "pub static {static_name}: RangeSliderDefinition = RangeSliderDefinition {{\n"
    ));
    out.push_str(&format!(
        "    id: {},\n",
        rust_string_literal(component.id.as_str())
    ));
    out.push_str(&format!(
        "    name: {},\n",
        rust_string_literal(&component.name)
    ));

    // The shared-type member lists (card 046 R3): the variants, polarities,
    // sizes, and densities the render discriminates on. Authoring order,
    // like the value domains of the corresponding attributes.
    for (field, shared_id) in [
        ("variants", "slider-variant"),
        ("polarities", "slider-polarity"),
        ("sizes", "control-size"),
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
    // `range-slider-ts` artifact (R2). Each entry is multi-line so the
    // emitted artifact is rustfmt-clean (the shell-rust precedent).
    out.push_str("    parts: &[\n");
    for part in &component.parts {
        out.push_str(&format!(
            "        RangeSliderPart {{\n            id: {},\n            name: {},\n            dom_class: {},\n            parent: {},\n        }},\n",
            rust_string_literal(part.id.as_str()),
            rust_wrapped_literal(&part.name),
            rust_wrapped_literal(&part_class_name(part.id.as_str())),
            part.parent
                .as_ref()
                .map(|parent| format!("Some({})", rust_string_literal(parent.as_str())))
                .unwrap_or_else(|| "None".to_owned())
        ));
    }
    out.push_str("    ],\n");

    // State attributes — the eight data-* names, forms, emission policies,
    // value domains; the same entries the web components read (R2/R3). The
    // RNG-17 computed custom properties are split into `style_props`
    // below: they emit as inline style, not DOM attributes.
    out.push_str("    attributes: &[\n");
    for attribute in component
        .attributes
        .iter()
        .filter(|attribute| !attribute.name.starts_with("--"))
    {
        out.push_str("        RangeSliderAttribute {\n");
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

    // Style props — the seven RNG-17 geometry hooks with the VisualState
    // field that feeds each (R2/R3). The render gates its fill treatments
    // on these names; the web components emit them as inline style.
    out.push_str("    style_props: &[\n");
    for attribute in component
        .attributes
        .iter()
        .filter(|attribute| attribute.name.starts_with("--"))
    {
        let field = value_visual_field(component, attribute);
        out.push_str(&format!(
            "        RangeSliderStyleProp {{\n            id: {},\n            name: {},\n            source: {},\n        }},\n",
            rust_string_literal(attribute.id.as_str()),
            rust_string_literal(&attribute.name),
            field
                .map(|field| rust_string_literal(field.id.as_str()))
                .unwrap_or_else(|| "none".to_owned())
        ));
    }
    out.push_str("    ],\n");

    // Recipe hooks — the `--poodle-recipe-*` override chains (RNG-21),
    // carried so the definition is the single record; the web styling seam
    // `range-slider.css` consumes them (card 045).
    out.push_str("    recipe_hooks: &[\n");
    for hook in &component.recipe_hooks {
        out.push_str(&format!(
            "        RangeSliderRecipeHook {{\n            hook: {},\n            chain: &[\n",
            rust_string_literal(&hook.hook)
        ));
        for link in &hook.chain {
            out.push_str(&format!(
                "                RangeSliderRecipeLink {{\n                    kind: {},\n                    target: {},\n                }},\n",
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
            format!("{}_DEFINITION", static_name("range-slider")),
            "RANGE_SLIDER_DEFINITION"
        );
    }
}
