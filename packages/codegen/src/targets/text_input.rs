//! The text-input-ts target — the authored TextInput definition as a
//! committed TypeScript artifact inside the consuming web component
//! packages (g13-b003 R1 "Generated output location": generated TypeScript
//! lands under a `generated/` directory in the consuming package).
//!
//! Output (per model, under the target's output root
//! `generated/text-input`):
//!
//! - `index.ts` — one `textInputDefinition` readonly constant per component
//!   model, carrying the **rendered vocabulary** (card 048 R2):
//!   - `parts` — the anatomy with the DOM class (or classes) each part
//!     renders under (the `poodle-text-input__<part>` projection plus the
//!     affix/affordance base+modifier pairs),
//!   - `attributes` — the four state-derived `data-*` attributes with
//!     their names, forms, emission policies, and value domains,
//!   - `styleProps` — the five TXT-16 custom properties
//!     (`--poodle-text-input-control-padding-start/end`,
//!     `--poodle-text-input-multiline-padding-end`,
//!     `--poodle-text-input-clear-inset-inline-end`,
//!     `--poodle-text-input-trailing-inset-inline-end`) the components
//!     emit as inline style, with the VisualState field that feeds each,
//!   - `recipeHooks` — the `--poodle-recipe-*` override chains.
//!
//! The generic attribute-domain projections (form names, emission names,
//! value domains) live in `super::button` (b041); they are component-
//! agnostic helpers and are reused here, exactly as the `range-slider-ts`
//! target reuses them (b045).
//!
//! # R2 — the artifact drives the DOM, not just types
//!
//! `g13.007`'s acceptance is "one definition change is visible in all four
//! previews", and a generated props type cannot satisfy that. This target
//! emits the rendered vocabulary — the parts, the state attributes and
//! their value domains, the padding custom properties, the recipe hooks —
//! and TextInput's Svelte and React read it instead of hard-coding the
//! four attribute names, the style-property names, and the anatomy classes
//! inline. Renaming an attribute or a style property in
//! `text_input.rs` moves both web previews' DOM in one `ir:build`.
//!
//! The target is scoped like `button-ts`/`range-slider-ts`: NOT in
//! [`super::all`], so a plain `ir:build` over the synthetic fixture never
//! writes into a web package; reachable via `--target text-input-ts`.
//!
//! # R1 — sibling targets, byte-frozen outputs
//!
//! The `button-*`, `range-slider-*`, and `shell-*` outputs are untouched
//! (their tests byte-compare them); this target owns a disjoint nested
//! root (`generated/text-input/`) under the same physical `generated/`
//! directory, the card 041 disjoint-roots layout.

use poodle_ir::{ComponentDefinition, IrModel};

use crate::emit::{header, sort_by_id, EmitTarget, GeneratedFile};
use crate::error::Result;

use super::button::{
    attribute_values, emission_name, form_name, link_kind_name, part_instances_ts,
    value_visual_field,
};
use super::shell::camel_case;
use super::ts::ts_string_literal;

/// The text-input-ts target. Scoped to the authored TextInput model:
/// not in [`super::all`], so a plain `ir:build` over the synthetic fixture
/// never writes into a web package; reachable via `--target text-input-ts`.
pub struct TextInputTarget;

impl EmitTarget for TextInputTarget {
    fn id(&self) -> &'static str {
        "text-input-ts"
    }

    // Its own nested root inside the shared `generated/` directory of the
    // consuming web component packages (`components/src/generated/`), the
    // same disjoint-roots layout card 041 established for `button-ts` and
    // card 045 for `range-slider-ts`.
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
            // `index.ts` inside the target's nested root
            // (`generated/text-input/`) so consumers import the
            // directory, not a doubled file name.
            GeneratedFile::new("index.ts", contents)
        })
        .collect()
}

/// The DOM class a part renders under (contract §2 anatomy classes plus the
/// rendered modifiers). The convention is `poodle-text-input__<part-id>`
/// with the documented exceptions: the root part is `poodle-text-input`,
/// and the affix/affordance parts carry a modifier class alongside the
/// base (`--prefix`/`--suffix`, `--leading`/`--trailing`) so one definition
/// names the full class list the markup emits. The projection is authored
/// exactly once here; the components read the result from the artifact.
pub(crate) fn part_class_name(part_id: &str) -> String {
    match part_id {
        "root" => "poodle-text-input".to_owned(),
        "prefix" => "poodle-text-input__affix poodle-text-input__affix--prefix".to_owned(),
        "field" => "poodle-text-input__field".to_owned(),
        "leading-affordance" => {
            "poodle-text-input__affordance poodle-text-input__affordance--leading".to_owned()
        }
        "input-control" => "poodle-text-input__control".to_owned(),
        "trailing-affordance" => {
            "poodle-text-input__affordance poodle-text-input__affordance--trailing".to_owned()
        }
        "clear-button" => "poodle-text-input__clear".to_owned(),
        "validation-indicator" => "poodle-text-input__validation-indicator".to_owned(),
        "suffix" => "poodle-text-input__affix poodle-text-input__affix--suffix".to_owned(),
        "char-count" => "poodle-text-input__char-count".to_owned(),
        _ => format!("poodle-text-input__{part_id}"),
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
    // renders under. Identified families (g13.018 R5) also carry their
    // instance list, so the count and the identities come from the
    // definition.
    out.push_str("  parts: [\n");
    for part in &component.parts {
        out.push_str(&format!(
            "    {{ id: {}, name: {}, className: {}, parent: {}, instances: {} }},\n",
            ts_string_literal(part.id.as_str()),
            ts_string_literal(&part.name),
            ts_string_literal(&part_class_name(part.id.as_str())),
            part.parent
                .as_ref()
                .map(|parent| ts_string_literal(parent.as_str()))
                .unwrap_or_else(|| "null".to_owned()),
            part_instances_ts(part)
        ));
    }
    out.push_str("  ],\n");

    // State attributes — the data-* names, forms, emission policies, and
    // value domains (R2). The TXT-16 custom properties are split into
    // `styleProps` below: they emit as inline style, not DOM attributes.
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

    // Style props — the TXT-16 computed custom properties with the
    // VisualState field that feeds each. The components read the names and
    // emit them as inline style with the runtime-computed values (Svelte
    // emits all five; React emits the three shared ones — recorded in the
    // batch log's R7 inventory).
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
    // TXT-27). The styling seam text-input.css already consumes; the
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
        assert_eq!(part_class_name("root"), "poodle-text-input");
        assert_eq!(part_class_name("field"), "poodle-text-input__field");
        assert_eq!(
            part_class_name("input-control"),
            "poodle-text-input__control"
        );
        assert_eq!(part_class_name("clear-button"), "poodle-text-input__clear");
        assert_eq!(
            part_class_name("validation-indicator"),
            "poodle-text-input__validation-indicator"
        );
        assert_eq!(
            part_class_name("char-count"),
            "poodle-text-input__char-count"
        );
        // The affix and affordance parts carry base + modifier classes.
        assert_eq!(
            part_class_name("prefix"),
            "poodle-text-input__affix poodle-text-input__affix--prefix"
        );
        assert_eq!(
            part_class_name("suffix"),
            "poodle-text-input__affix poodle-text-input__affix--suffix"
        );
        assert_eq!(
            part_class_name("leading-affordance"),
            "poodle-text-input__affordance poodle-text-input__affordance--leading"
        );
        assert_eq!(
            part_class_name("trailing-affordance"),
            "poodle-text-input__affordance poodle-text-input__affordance--trailing"
        );
    }
}
