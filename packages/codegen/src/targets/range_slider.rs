//! The range-slider-ts target — the authored RangeSlider definition as a
//! committed TypeScript artifact inside the consuming web component
//! packages (g13-b003 R1 "Generated output location": generated TypeScript
//! lands under a `generated/` directory in the consuming package).
//!
//! Output (per model, under the target's output root
//! `generated/range-slider`):
//!
//! - `index.ts` — one `rangeSliderDefinition` readonly constant per
//!   component model, carrying the **rendered vocabulary** (card 045 R2):
//!   - `parts` — the anatomy with the DOM class (or classes) each part
//!     renders under (the `poodle-range-slider__<part>` projection plus the
//!     `--negative`/`--positive`/`--lower`/`--upper` modifiers),
//!   - `attributes` — the eight state-derived `data-*` attributes with
//!     their names, forms, emission policies, and value domains,
//!   - `styleProps` — the seven computed custom properties
//!     (`--poodle-range-start/end/center/negative-*/positive-*`, RNG-17)
//!     the components emit as inline style, with the VisualState field that
//!     feeds each,
//!   - `recipeHooks` — the `--poodle-recipe-*` override chains.
//!
//! The generic attribute-domain projections (form names, emission names,
//! value domains) live in `super::button` (b041); they are component-
//! agnostic helpers and are reused here. When a third component arrives
//! they should be hoisted to a shared module — a g13.008 question, not
//! this card's.
//!
//! # R2 — the artifact drives the DOM, not just types
//!
//! `g13.006`'s acceptance is "one definition change is visible in all four
//! previews", and a generated props type cannot satisfy that. This target
//! emits the rendered vocabulary — the parts, the state attributes and
//! their value domains, the geometry custom properties, the recipe hooks —
//! and RangeSlider's Svelte and React read it instead of hard-coding the
//! eight attribute names, the seven style-property names, and the anatomy
//! classes inline. Renaming an attribute or a geometry hook in
//! `range_slider.rs` moves both web previews' DOM in one `ir:build`.
//!
//! The target is scoped like `button-ts`: NOT in [`super::all`], so a
//! plain `ir:build` over the synthetic fixture never writes into a web
//! package; reachable via `--target range-slider-ts`.

use poodle_ir::{ComponentDefinition, IrModel};

use crate::emit::{header, sort_by_id, EmitTarget, GeneratedFile};
use crate::error::Result;

use super::button::{
    attribute_values, emission_name, form_name, link_kind_name, value_visual_field,
};
use super::shell::camel_case;
use super::ts::ts_string_literal;

/// The range-slider-ts target. Scoped to the authored RangeSlider model:
/// not in [`super::all`], so a plain `ir:build` over the synthetic fixture
/// never writes into a web package; reachable via `--target
/// range-slider-ts`.
pub struct RangeSliderTarget;

impl EmitTarget for RangeSliderTarget {
    fn id(&self) -> &'static str {
        "range-slider-ts"
    }

    // Its own nested root inside the shared `generated/` directory of the
    // consuming web component packages (`components/src/generated/`), the
    // same disjoint-roots layout card 041 established for `button-ts`.
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
            // `index.ts` inside the target's nested root
            // (`generated/range-slider/`) so consumers import the
            // directory, not a doubled file name.
            GeneratedFile::new("index.ts", contents)
        })
        .collect()
}

/// The DOM class a part renders under (contract §2 anatomy classes plus the
/// rendered modifiers). The convention is `poodle-range-slider__<part-id>`
/// with the documented exceptions: the root part is `poodle-range-slider`,
/// and the fill/control parts carry a modifier class alongside the base
/// (`--negative`/`--positive`/`--center`/`--lower`/`--upper`,
/// `--embedded-control` base) so one definition names the full class list
/// the markup emits. The projection is authored exactly once here; the
/// components read the result from the artifact.
pub(crate) fn part_class_name(part_id: &str) -> String {
    match part_id {
        "root" => "poodle-range-slider".to_owned(),
        "fill-negative" => {
            "poodle-range-slider__fill poodle-range-slider__fill--negative".to_owned()
        }
        "fill-positive" => {
            "poodle-range-slider__fill poodle-range-slider__fill--positive".to_owned()
        }
        "control-lower" => {
            "poodle-range-slider__control poodle-range-slider__control--lower".to_owned()
        }
        "control-upper" => {
            "poodle-range-slider__control poodle-range-slider__control--upper".to_owned()
        }
        "embedded-lower" => "poodle-range-slider__embedded-control \
             poodle-range-slider__embedded-control--lower"
            .to_owned(),
        "embedded-upper" => "poodle-range-slider__embedded-control \
             poodle-range-slider__embedded-control--upper"
            .to_owned(),
        _ => format!("poodle-range-slider__{part_id}"),
    }
}

fn render_component_file(
    model: &IrModel,
    component: &ComponentDefinition,
    source_path: &str,
) -> String {
    let mut out = header(source_path);

    out.push_str(&format!(
        "export const {} = {{\n",
        camel_case(&format!("{}-definition", component.id.as_str()))
    ));
    out.push_str(&format!(
        "  id: {},\n",
        ts_string_literal(component.id.as_str())
    ));
    out.push_str(&format!(
        "  name: {},\n",
        ts_string_literal(&component.name)
    ));

    // Parts — the anatomy with the DOM class (or classes) each part
    // renders under.
    out.push_str("  parts: [\n");
    for part in &component.parts {
        out.push_str(&format!(
            "    {{ id: {}, name: {}, className: {}, parent: {} }},\n",
            ts_string_literal(part.id.as_str()),
            ts_string_literal(&part.name),
            ts_string_literal(&part_class_name(part.id.as_str())),
            part.parent
                .as_ref()
                .map(|parent| ts_string_literal(parent.as_str()))
                .unwrap_or_else(|| "null".to_owned())
        ));
    }
    out.push_str("  ],\n");

    // State attributes — the data-* names, forms, emission policies, and
    // value domains (R2). The RNG-17 computed custom properties are split
    // into `styleProps` below: they emit as inline style, not DOM
    // attributes.
    out.push_str("  attributes: [\n");
    for attribute in component
        .attributes
        .iter()
        .filter(|attribute| !attribute.name.starts_with("--"))
    {
        let mut entry = format!(
            "    {{ id: {}, name: {}, form: {}, emission: {}",
            ts_string_literal(attribute.id.as_str()),
            ts_string_literal(&attribute.name),
            ts_string_literal(form_name(attribute.form)),
            ts_string_literal(emission_name(attribute.emission))
        );
        if let Some(values) = attribute_values(model, component, attribute) {
            let values = values
                .iter()
                .map(|value| ts_string_literal(value))
                .collect::<Vec<_>>()
                .join(", ");
            entry.push_str(&format!(", values: [{values}]"));
        }
        entry.push_str(" },\n");
        out.push_str(&entry);
    }
    out.push_str("  ],\n");

    // Style props — the computed custom properties (RNG-17) with the
    // VisualState field that feeds each. The components read the names and
    // emit them as inline style with the machine-computed values.
    out.push_str("  styleProps: [\n");
    for attribute in component
        .attributes
        .iter()
        .filter(|attribute| attribute.name.starts_with("--"))
    {
        let field = value_visual_field(component, attribute);
        out.push_str(&format!(
            "    {{ id: {}, name: {}, source: {} }},\n",
            ts_string_literal(attribute.id.as_str()),
            ts_string_literal(&attribute.name),
            field
                .map(|field| ts_string_literal(field.id.as_str()))
                .unwrap_or_else(|| "null".to_owned())
        ));
    }
    out.push_str("  ],\n");

    // Recipe hooks — the --poodle-recipe-* override chains (CROSS-09;
    // RNG-21). The styling seam range-slider.css already consumes; the
    // artifact carries the declared chains so the definition is the single
    // record.
    out.push_str("  recipeHooks: [\n");
    for hook in &component.recipe_hooks {
        out.push_str(&format!(
            "    {{ hook: {}, chain: [",
            ts_string_literal(&hook.hook)
        ));
        let chain = hook
            .chain
            .iter()
            .map(|link| {
                format!(
                    "{{ kind: {}, target: {} }}",
                    ts_string_literal(link_kind_name(link.kind)),
                    ts_string_literal(&link.target)
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        out.push_str(&chain);
        out.push_str("] },\n");
    }
    out.push_str("  ],\n");

    out.push_str("} as const;\n");
    out
}

#[cfg(test)]
mod tests {
    use super::part_class_name;

    #[test]
    fn part_classes_follow_the_anatomy_convention() {
        assert_eq!(part_class_name("root"), "poodle-range-slider");
        assert_eq!(part_class_name("track"), "poodle-range-slider__track");
        assert_eq!(part_class_name("center"), "poodle-range-slider__center");
        // The fill and control parts carry base + modifier classes.
        assert_eq!(
            part_class_name("fill-negative"),
            "poodle-range-slider__fill poodle-range-slider__fill--negative"
        );
        assert_eq!(
            part_class_name("fill-positive"),
            "poodle-range-slider__fill poodle-range-slider__fill--positive"
        );
        assert_eq!(
            part_class_name("control-lower"),
            "poodle-range-slider__control poodle-range-slider__control--lower"
        );
        assert_eq!(
            part_class_name("control-upper"),
            "poodle-range-slider__control poodle-range-slider__control--upper"
        );
        assert_eq!(
            part_class_name("embedded-lower"),
            "poodle-range-slider__embedded-control poodle-range-slider__embedded-control--lower"
        );
        assert_eq!(
            part_class_name("embedded-upper"),
            "poodle-range-slider__embedded-control poodle-range-slider__embedded-control--upper"
        );
    }
}
