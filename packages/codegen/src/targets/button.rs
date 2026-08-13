//! The button-ts target — the authored Button definition as a committed
//! TypeScript artifact inside the consuming web packages (g13-b003 R1
//! "Generated output location": generated TypeScript lands under a
//! `generated/` directory in the consuming package, mirroring
//! `packages/core/src/tokens/generated/` and the `shell-scene` target).
//!
//! Output (per model, under the target's output root `generated/button`):
//!
//! - `index.ts` — one `buttonDefinition` readonly constant per component
//!   model, carrying the **rendered vocabulary** (card 041 R2):
//!   - `parts` — the anatomy with the DOM class each part renders under
//!     (the `poodle-button__<part>` projection),
//!   - `attributes` — the state-derived `data-*` attributes with their
//!     names, forms, emission policies, and **value domains**,
//!   - `recipeHooks` — the `--poodle-recipe-*` override chains.
//!
//! The nested root (`generated/button/`) keeps this target's files
//! disjoint from the `shell-scene` target's, which owns the top level of
//! the same physical `generated/` directory in the web packages; each
//! target's orphan sweep is scoped to its own root's top level (card 041).
//!
//! # R2 — the artifact drives the DOM, not just types
//!
//! `g13.005`'s acceptance is "one definition change is visible in all four
//! previews", and a generated `type ButtonProps` cannot satisfy that. This
//! target therefore emits the rendered vocabulary — the parts, the state
//! attributes and their value domains, the recipe hooks — and Button's
//! Svelte and React read it instead of hard-coding the eleven attribute
//! names and their values inline. Renaming an attribute in `button.rs`
//! moves both web previews' DOM in one `ir:build`.
//!
//! The target is scoped like `shell-scene`: NOT in [`super::all`], so a
//! plain `ir:build` over the synthetic fixture never writes into a web
//! package; reachable via `--target button-ts`.

use poodle_ir::{
    AttributeForm, ComponentDefinition, EmissionPolicy, Expr, ExprOperand, Identifier, IrModel,
    PropType, RecipeLinkKind, StateAttribute, Value, VisualFieldKind,
};

use crate::emit::{header, sort_by_id, EmitTarget, GeneratedFile};
use crate::error::Result;

use super::shell::camel_case;
use super::ts::ts_string_literal;

/// The button-ts target. Scoped to the authored Button model: not in
/// [`super::all`], so a plain `ir:build` over the synthetic fixture never
/// writes into a web package; reachable via `--target button-ts`.
pub struct ButtonTarget;

impl EmitTarget for ButtonTarget {
    fn id(&self) -> &'static str {
        "button-ts"
    }

    // Its own nested root inside the shared `generated/` directory: the
    // shell-scene target owns the top level of the same physical directory
    // (both web artifact families land in the consuming packages'
    // `preview/src/generated/`), and each target's orphan sweep is scoped
    // to its own root's top level.
    fn output_root(&self) -> &'static str {
        "generated/button"
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
            // `index.ts` inside the target's nested root (`generated/button/`)
            // so consumers import the directory, not a doubled file name.
            GeneratedFile::new("index.ts", contents)
        })
        .collect()
}

/// The DOM class a part renders under (contract §2 anatomy classes). The
/// convention is `poodle-button__<part-id>`; two documented exceptions:
/// the root part is `poodle-button`, and the Leading/Trailing Icon parts
/// share one DOM class (`poodle-button__icon`) because the anatomy renders
/// the same icon span for leading and trailing content (B §2). The
/// projection is authored exactly once here; the components read the
/// result from the artifact.
///
/// Shared with the `button-rust` sibling target (card 042 R2): one home
/// for the anatomy projection, like `shell.rs`'s label helpers in 036.
pub(crate) fn part_class_name(part_id: &str) -> String {
    match part_id {
        "root" => "poodle-button".to_owned(),
        "leading-icon" | "trailing-icon" => "poodle-button__icon".to_owned(),
        _ => format!("poodle-button__{part_id}"),
    }
}

/// The serialized emission-policy name (the IR's serde rename). Shared
/// with the `button-rust` sibling target (card 042 R2).
pub(crate) fn emission_name(policy: EmissionPolicy) -> &'static str {
    match policy {
        EmissionPolicy::OmitWhenDefault => "omit-when-default",
        EmissionPolicy::Always => "always",
    }
}

/// The serialized attribute-form name (the IR's serde rename). Shared
/// with the `button-rust` sibling target (card 042 R2).
pub(crate) fn form_name(form: AttributeForm) -> &'static str {
    match form {
        AttributeForm::PresenceOnly => "presence-only",
        AttributeForm::Valued => "valued",
    }
}

/// The serialized recipe-link-kind name (the IR's serde rename). Shared
/// with the `button-rust` sibling target (card 042 R2).
pub(crate) fn link_kind_name(kind: RecipeLinkKind) -> &'static str {
    match kind {
        RecipeLinkKind::RecipeHook => "recipe-hook",
        RecipeLinkKind::ComponentVariable => "component-variable",
        RecipeLinkKind::Token => "token",
    }
}

/// The prop an attribute's source references, if any. Shared with the
/// `button-rust` sibling target (card 042 R2).
pub(crate) fn source_prop<'a>(
    component: &'a ComponentDefinition,
    attribute: &StateAttribute,
) -> Option<&'a poodle_ir::Prop> {
    attribute
        .source
        .as_ref()
        .and_then(|source| component.props.iter().find(|p| &p.id == source))
}

/// The visual field an attribute's value expression references, if any.
/// Shared with the `button-rust` sibling target (card 042 R2).
pub(crate) fn value_visual_field<'a>(
    component: &'a ComponentDefinition,
    attribute: &StateAttribute,
) -> Option<&'a poodle_ir::VisualStateField> {
    match attribute.value.as_ref() {
        Some(Expr::Operand(ExprOperand::Visual(field_id))) => component
            .visual_state
            .iter()
            .flat_map(|state| state.fields.iter())
            .find(|field| &field.id == field_id),
        _ => None,
    }
}

/// The value domain of an attribute — the emitter's deterministic
/// projection of the source's declared type (R2 "state attributes and their
/// value domains"):
///
/// - a `Shared` prop projects the shared type's members;
/// - a `Bool` source projects `true`/`false`;
/// - an `OmitWhenDefault` emission drops the prop's default member from the
///   domain (the DOM never carries it, e.g. `data-tone` is never
///   `"default"`);
/// - presence-only attributes carry no domain.
pub(crate) fn attribute_values(
    model: &IrModel,
    component: &ComponentDefinition,
    attribute: &StateAttribute,
) -> Option<Vec<String>> {
    let domain: Vec<String> = source_prop(component, attribute)
        .and_then(|prop| prop_domain(model, &prop.prop_type))
        .or_else(|| {
            value_visual_field(component, attribute)
                .and_then(|field| field_domain(model, &field.kind))
        })?;

    // Omit-when-default: the default member never reaches the DOM, so it is
    // not part of the emitted domain (B §9, BTN-18).
    if attribute.emission == EmissionPolicy::OmitWhenDefault {
        if let Some(Value::Member(default_member)) =
            source_prop(component, attribute).and_then(|prop| prop.default.as_ref())
        {
            return Some(
                domain
                    .into_iter()
                    .filter(|value| value != default_member.as_str())
                    .collect(),
            );
        }
    }
    Some(domain)
}

/// The domain a declared prop type carries. Shared with the `button-rust`
/// sibling target (card 042 R2).
pub(crate) fn prop_domain(model: &IrModel, prop_type: &PropType) -> Option<Vec<String>> {
    match prop_type {
        PropType::Shared(shared_id) => shared_member_names(model, shared_id),
        PropType::Bool => Some(vec!["true".to_owned(), "false".to_owned()]),
        _ => None,
    }
}

/// The domain a VisualState field kind carries. Shared with the
/// `button-rust` sibling target (card 042 R2).
pub(crate) fn field_domain(model: &IrModel, kind: &VisualFieldKind) -> Option<Vec<String>> {
    match kind {
        VisualFieldKind::Enum(shared_id) => shared_member_names(model, shared_id),
        VisualFieldKind::Bool => Some(vec!["true".to_owned(), "false".to_owned()]),
        _ => None,
    }
}

/// The member ids of a shared type, in authoring order. Shared with the
/// `button-rust` sibling target (card 042 R2).
pub(crate) fn shared_member_names(model: &IrModel, shared_id: &Identifier) -> Option<Vec<String>> {
    let shared = model.shared_type(shared_id.as_str())?;
    Some(
        shared
            .members
            .iter()
            .map(|member| member.id.to_string())
            .collect(),
    )
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

    // Parts — the anatomy with the DOM class each part renders under.
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

    // State attributes — names, forms, emission policies, and value
    // domains. The components read the names; the domains record what the
    // DOM can carry (R2).
    out.push_str("  attributes: [\n");
    for attribute in &component.attributes {
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

    // Recipe hooks — the --poodle-recipe-* override chains (CROSS-09;
    // BTN-22). The styling seam button.css already consumes; the artifact
    // carries the declared chains so the definition is the single record.
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
        assert_eq!(part_class_name("root"), "poodle-button");
        assert_eq!(part_class_name("spinner"), "poodle-button__spinner");
        assert_eq!(part_class_name("label"), "poodle-button__label");
        assert_eq!(part_class_name("chevron"), "poodle-button__chevron");
        // The two icon parts collapse onto the shared icon span class.
        assert_eq!(part_class_name("leading-icon"), "poodle-button__icon");
        assert_eq!(part_class_name("trailing-icon"), "poodle-button__icon");
    }
}
