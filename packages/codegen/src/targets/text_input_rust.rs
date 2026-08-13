//! The text-input-rust target — the authored TextInput definition as a
//! committed, self-contained Rust artifact inside `poodle-render`, the
//! package that consumes it (card 049 R1/R1a: `poodle-render` may not
//! depend on `poodle-ir` or `poodle-codegen` during the pilot — the
//! artifact is plain data with no `use` of any Poodle crate, pulled in via
//! `#[path]`, the `poodle-tokens` mechanism g13-b003 R1 names).
//!
//! Output (per model, under the target's output root
//! `generated/text-input`):
//!
//! - `index.rs` — the definition as a `pub static` of plain data
//!   structs: the shared-type member lists the render discriminates on
//!   (types, validation states, sizes, densities), the ten-part anatomy,
//!   the four `data-*` state attributes with their names, forms, emission
//!   policies, and value domains, the five TXT-16 padding hooks as
//!   `style_props` with the VisualState field that feeds each, the
//!   recipe-hook chains, and the typed capability boundary (R2/R3 — the
//!   six environment capabilities plus the component-owned timers).
//!   One file per component model.
//!
//! # R2 — a sibling target, not a repurposed one
//!
//! Card 049 R1: `text-input-ts`'s output is byte-compared by b048's
//! tests, so it is frozen. This target renders the **same definition** in
//! Rust shape — the same vocabulary projections, shared via the
//! `pub(crate)` helpers in [`super::button`] (attributes, domains, chain
//! kinds, visual-field sources) and [`super::text_input`] (the part-class
//! projection) — so one authored change still moves every artifact in one
//! `ir:build`: the two web artifacts and this one.
//!
//! The capability boundary is rendered as the one vocabulary the natives
//! consume beyond the web channel: `poodle-render`'s text input wires its
//! edit handlers only while the definition declares the capabilities that
//! own them (`text-editing`, `measurement`, `focus`), which is the
//! component-scoped honouring of the boundary — the artifact carries the
//! same list for every runtime, which is exactly the R3 finding: the IR
//! has no per-runtime expression, so nothing can drop the wiring for
//! Jetstream alone.

use poodle_ir::{ComponentDefinition, Identifier, IrModel};

use crate::emit::{header, sort_by_id, EmitTarget, GeneratedFile};
use crate::error::Result;

use super::button::{
    attribute_values, emission_name, form_name, link_kind_name, part_instances,
    shared_member_names, value_visual_field,
};
use super::shell_rust::{rust_string_literal, static_name};
use super::text_input::part_class_name;

/// The text-input-rust target. Scoped to the authored TextInput model:
/// not in [`super::all`], so a plain `ir:build` over the synthetic fixture
/// never writes into a consumer package; reachable via `--target
/// text-input-rust`.
pub struct TextInputRustTarget;

impl EmitTarget for TextInputRustTarget {
    fn id(&self) -> &'static str {
        "text-input-rust"
    }

    // Its own nested root inside the shared `generated/` directory of
    // poodle-render — the same disjoint-roots layout card 041 established
    // for shared `generated/` directories (write.rs: a nested directory is
    // another target's root; the top level belongs to button-rust, whose
    // orphan sweep would delete a sibling's file).
    fn output_root(&self) -> &'static str {
        "generated/text-input"
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
            // (`generated/text-input/`) so the module include names the
            // directory, not a doubled file name — the same shape as the
            // `text-input-ts` artifact (`generated/text-input/index.ts`).
            GeneratedFile::new("index.rs", contents)
        })
        .collect()
}

/// The struct definitions every artifact carries. Emitted verbatim so each
/// artifact is self-contained — the consumer must not import anything to
/// read the definition. `dead_code` is allowed by design: the artifact is
/// the definition, and the render consumes the subset it implements.
const STRUCT_PRELUDE: &str = r#"#![allow(dead_code)]

//! The authored TextInput definition (spec 063 "Generated Artifact
//! Contract"): plain data, self-contained, no Poodle crate imports. Pulled
//! `poodle-render` via `#[path = "generated/text-input/index.rs"]` — the
//! `poodle-tokens` mechanism (g13-b003 R1). Regenerate with
//! `effigy ir:build`; drift is gated by `effigy ir:check`.
//!
//! Card 049 R3: the render takes its vocabulary — the ten-part anatomy,
//! the validation-state treatment, the size/density ladders, and the
//! typed capability boundary — from this definition instead of its own
//! literals; the web components read the same vocabulary from the
//! `text-input-ts` artifact (card 048 R2). One definition change moves
//! every artifact and every runtime in one `ir:build`.
//!
//! The capabilities are the boundary the milestone measures (g13.007):
//! declared for the component, never per-runtime — the same list every
//! runtime sees, which is the finding this card records (R3): the IR
//! cannot express that Jetstream lacks a declared capability.

/// One anatomy part: id, display name, the DOM class the web markup
/// renders it under, its parent part, and — for an identified family
/// (g13.018 R5) — the ids of its instances. The class projection is shared
/// with the `text-input-ts` artifact (`part_class_name`).
pub struct TextInputPart {
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
/// emitter's projection of the source's declared type (a shared prop or a
/// VisualState enum field); source-less attributes carry no domain.
pub struct TextInputAttribute {
    pub id: &'static str,
    pub name: &'static str,
    pub form: &'static str,
    pub emission: &'static str,
    pub values: Option<&'static [&'static str]>,
}

/// One TXT-16 padding hook: the computed custom property the web markup
/// emits as inline style, and the VisualState field that feeds it. The
/// values are runtime-derived strings (`calc()` arithmetic is not
/// vocabulary) — the names and their source fields are the declared
/// vocabulary.
pub struct TextInputStyleProp {
    pub id: &'static str,
    pub name: &'static str,
    pub source: &'static str,
}

/// One link in a recipe-hook override chain.
pub struct TextInputRecipeLink {
    pub kind: &'static str,
    pub target: &'static str,
}

/// One recipe hook: the `--poodle-recipe-*` name and its override chain.
pub struct TextInputRecipeHook {
    pub hook: &'static str,
    pub chain: &'static [TextInputRecipeLink],
}

/// The authored TextInput definition — plain data, no Poodle crate
/// imports.
pub struct TextInputDefinition {
    pub id: &'static str,
    pub name: &'static str,
    /// The `text-input-type` shared-type members — the `data-type` value
    /// domain.
    pub types: &'static [&'static str],
    /// The `validation-state` shared-type members — the `data-*` value
    /// domain for validation (the exact attribute name is carried in the
    /// `attributes` rows below, so a rename never leaves a stale name in
    /// this doc), and the validation treatments the render discriminates
    /// on.
    pub validation_states: &'static [&'static str],
    /// The `control-size` shared-type members — the `data-size` value
    /// domain, and the size ladder the render's metric treatments
    /// discriminate on.
    pub sizes: &'static [&'static str],
    /// The `control-density` shared-type members — the `data-density`
    /// value domain, and the density adjustments the render's padding
    /// treatment discriminates on.
    pub densities: &'static [&'static str],
    /// The declared capability boundary (R2/R3) — the serde names of the
    /// `Capability` requirements in declaration order. The render wires
    /// the edit handlers the capability names own while declared; the
    /// list is the same for every runtime.
    pub capabilities: &'static [&'static str],
    pub parts: &'static [TextInputPart],
    pub attributes: &'static [TextInputAttribute],
    pub style_props: &'static [TextInputStyleProp],
    pub recipe_hooks: &'static [TextInputRecipeHook],
}

"#;

/// The member ids of a shared type, or an empty slice when the definition
/// does not declare it (unreachable for the authored TextInput model; the
/// render falls back to its default arms).
fn shared_members(model: &IrModel, id: &str) -> Vec<String> {
    shared_member_names(model, &Identifier::from(id)).unwrap_or_default()
}

/// The serde names of the declared capabilities, in declaration order.
fn capability_names(component: &ComponentDefinition) -> Vec<&'static str> {
    component
        .capabilities
        .iter()
        .map(|requirement| match requirement.capability {
            poodle_ir::Capability::Focus => "focus",
            poodle_ir::Capability::Measurement => "measurement",
            poodle_ir::Capability::PointerCapture => "pointer-capture",
            poodle_ir::Capability::ScrubFraction => "scrub-fraction",
            poodle_ir::Capability::TextEditing => "text-editing",
            poodle_ir::Capability::Ime => "ime",
            poodle_ir::Capability::Clipboard => "clipboard",
            poodle_ir::Capability::PortalPlacement => "portal-placement",
            poodle_ir::Capability::Timers => "timers",
            poodle_ir::Capability::Announcements => "announcements",
        })
        .collect()
}

/// A Rust string literal that stays within rustfmt's 100-column line: short
/// values emit as one literal, long ones (the affix/affordance class
/// pairs) as a single backslash-continuation literal — one string, no
/// adjacent literals (which do not parse in field position), rustfmt-clean
/// (the shell-rust/button-rust/range-slider-rust precedent).
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
        "pub static {static_name}: TextInputDefinition = TextInputDefinition {{\n"
    ));
    out.push_str(&format!(
        "    id: {},\n",
        rust_string_literal(component.id.as_str())
    ));
    out.push_str(&format!(
        "    name: {},\n",
        rust_string_literal(&component.name)
    ));

    // The shared-type member lists (card 049 R3): the types, validation
    // states, sizes, and densities the render discriminates on. Authoring
    // order, like the value domains of the corresponding attributes.
    for (field, shared_id) in [
        ("types", "text-input-type"),
        ("validation_states", "validation-state"),
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

    // The capability boundary (R2/R3) — the serde names in declaration
    // order. The one vocabulary the natives consume beyond the web
    // channel: the render gates its edit-handler wiring on these names.
    // One entry per line: six entries exceed rustfmt's one-line array
    // threshold, so a single-line emission is not rustfmt-clean.
    out.push_str("    capabilities: &[\n");
    for name in capability_names(component) {
        out.push_str(&format!("        {},\n", rust_string_literal(name)));
    }
    out.push_str("    ],\n");

    // Parts — the anatomy with the DOM class projection, shared with the
    // `text-input-ts` artifact (R2). Each entry is multi-line so the
    // emitted artifact is rustfmt-clean (the shell-rust precedent).
    // Identified families (g13.018 R5) also carry their instance list.
    out.push_str("    parts: &[\n");
    for part in &component.parts {
        out.push_str(&format!(
            "        TextInputPart {{\n            id: {},\n            name: {},\n            dom_class: {},\n            parent: {},\n            instances: {},\n        }},\n",
            rust_string_literal(part.id.as_str()),
            rust_wrapped_literal(&part.name),
            rust_wrapped_literal(&part_class_name(part.id.as_str())),
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

    // State attributes — the four data-* names, forms, emission policies,
    // value domains; the same entries the web components read (R2/R3).
    // The TXT-16 computed custom properties are split into `style_props`
    // below: they emit as inline style, not DOM attributes.
    out.push_str("    attributes: &[\n");
    for attribute in component
        .attributes
        .iter()
        .filter(|attribute| !attribute.name.starts_with("--"))
    {
        out.push_str("        TextInputAttribute {\n");
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

    // Style props — the five TXT-16 padding hooks with the VisualState
    // field that feeds each (R2/R3). The render carries them; the web
    // components emit them as inline style.
    out.push_str("    style_props: &[\n");
    for attribute in component
        .attributes
        .iter()
        .filter(|attribute| attribute.name.starts_with("--"))
    {
        let field = value_visual_field(component, attribute);
        out.push_str(&format!(
            "        TextInputStyleProp {{\n            id: {},\n            name: {},\n            source: {},\n        }},\n",
            rust_string_literal(attribute.id.as_str()),
            rust_string_literal(&attribute.name),
            field
                .map(|field| rust_string_literal(field.id.as_str()))
                .unwrap_or_else(|| "none".to_owned())
        ));
    }
    out.push_str("    ],\n");

    // Recipe hooks — the `--poodle-recipe-*` override chains (TXT-27),
    // carried so the definition is the single record; the web styling seam
    // `text-input.css` consumes them (card 048).
    out.push_str("    recipe_hooks: &[\n");
    for hook in &component.recipe_hooks {
        out.push_str(&format!(
            "        TextInputRecipeHook {{\n            hook: {},\n            chain: ",
            rust_string_literal(&hook.hook)
        ));
        if hook.chain.len() == 1 {
            // rustfmt collapses a single-link slice containing a multi-line
            // struct literal: emit its canonical shape so the artifact stays
            // rustfmt-clean (the sibling artifacts never hit this — every
            // button/range-slider chain has two or more links).
            let link = &hook.chain[0];
            out.push_str(&format!(
                "&[TextInputRecipeLink {{\n                kind: {},\n                target: {},\n            }}],\n",
                rust_string_literal(link_kind_name(link.kind)),
                rust_string_literal(&link.target)
            ));
        } else {
            out.push_str("&[\n");
            for link in &hook.chain {
                out.push_str(&format!(
                    "                TextInputRecipeLink {{\n                    kind: {},\n                    target: {},\n                }},\n",
                    rust_string_literal(link_kind_name(link.kind)),
                    rust_string_literal(&link.target)
                ));
            }
            out.push_str("            ],\n");
        }
        out.push_str("        },\n");
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
            format!("{}_DEFINITION", static_name("text-input")),
            "TEXT_INPUT_DEFINITION"
        );
    }

    #[test]
    fn capability_serde_names_match_the_ir_renames() {
        use poodle_ir::Capability;
        let expected = [
            (Capability::Focus, "focus"),
            (Capability::TextEditing, "text-editing"),
            (Capability::Ime, "ime"),
            (Capability::Clipboard, "clipboard"),
            (Capability::Measurement, "measurement"),
            (Capability::Timers, "timers"),
        ];
        let model = crate::models::text_input::text_input_model();
        let component = model
            .components
            .iter()
            .find(|component| component.id.as_str() == "text-input")
            .expect("the one component");
        let names = capability_names(component);
        assert_eq!(
            names,
            expected.iter().map(|(_, name)| *name).collect::<Vec<_>>(),
            "the artifact's capability names match the IR serde renames, in declaration order"
        );
        for (capability, name) in expected {
            assert!(
                component
                    .capabilities
                    .iter()
                    .any(|requirement| requirement.capability == capability),
                "the authored definition declares {name}"
            );
        }
    }
}
