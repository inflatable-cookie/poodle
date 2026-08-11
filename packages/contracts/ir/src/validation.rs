//! Validation — one entry point, every finding at once.
//!
//! [`validate`] walks a whole [`IrModel`](crate::IrModel) and returns
//! **all** findings, never the first (batch card 011: "Validation must
//! report ALL findings at once, not the first"). Every finding carries the
//! offending identifier and an actionable message naming what to fix.
//!
//! Covered rules (card scope): duplicate IDs, invalid references, impossible
//! prop bindings, unsupported cycles, missing accessibility data,
//! undeclared capabilities, and a value outside a component's permitted
//! subset of a shared type (g13-b003 R6.2), plus schema-version and token
//! resolution checks.
//!
//! Validation is pure data inspection — no code generation, no emission,
//! no framework involvement (`NEG-01`).

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::{
    AttributeForm, Axes, Identifier, IrModel, PartKind, PermittedSubset, Prop, PropType,
    SceneAxisKind, Value,
};

/// A validation finding — one violation, with the offending identifier and
/// an actionable message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finding {
    /// Rule category of the finding.
    pub kind: FindingKind,
    /// The offending identifier — the component, prop, member, binding, or
    /// path that must be fixed. May be `"(model)"` for whole-model rules.
    pub identifier: String,
    /// Actionable message naming what to fix and why.
    pub message: String,
}

/// Category of a validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FindingKind {
    /// Serialization schema version mismatch (IR-07).
    #[serde(rename = "schema-version")]
    SchemaVersion,
    /// A duplicate identifier in a scope that requires uniqueness.
    #[serde(rename = "duplicate-id")]
    DuplicateId,
    /// A reference to an undefined id (shared type, member, prop, part,
    /// event, state, vector, component, scene, token path, theme name).
    #[serde(rename = "invalid-reference")]
    InvalidReference,
    /// A prop binding that is impossible: unknown prop, type mismatch, or a
    /// do-not-mix controlled pair (`CROSS-04`).
    #[serde(rename = "impossible-binding")]
    ImpossibleBinding,
    /// A parent/child cycle among anatomy parts (`CROSS-12`).
    #[serde(rename = "cycle")]
    Cycle,
    /// Required accessibility data missing (`CROSS-15`; `BTN-21`).
    #[serde(rename = "missing-accessibility")]
    MissingAccessibility,
    /// A capability a declared behavior needs but the component does not
    /// declare (`CROSS-17`, IR-08).
    #[serde(rename = "undeclared-capability")]
    UndeclaredCapability,
    /// A value outside a component's permitted subset of a shared type
    /// (g13-b003 R6.2).
    #[serde(rename = "permitted-subset-violation")]
    PermittedSubsetViolation,
}

impl Finding {
    /// Builds a finding.
    pub fn new(
        kind: FindingKind,
        identifier: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            identifier: identifier.into(),
            message: message.into(),
        }
    }
}

/// Validates the whole model, returning every finding at once.
///
/// Ordering is deterministic: model-level rules first (schema version,
/// duplicate ids, references), then per-component rules, then scene rules.
pub fn validate(model: &IrModel) -> Vec<Finding> {
    let mut findings = Vec::new();

    validate_schema_version(model, &mut findings);
    validate_shared_types(model, &mut findings);
    validate_conformance_vectors(model, &mut findings);
    for component in &model.components {
        validate_component(model, component, &mut findings);
    }
    validate_scenes(model, &mut findings);
    validate_registry(model, &mut findings);

    findings
}

fn validate_schema_version(model: &IrModel, findings: &mut Vec<Finding>) {
    if model.schema_version != crate::IR_SCHEMA_VERSION {
        findings.push(Finding::new(
            FindingKind::SchemaVersion,
            "(model)",
            format!(
                "schema_version is {}, expected {} (IR_SCHEMA_VERSION); re-encode the model or \
                 migrate it, breaking changes require a migration or a deliberate pre-1.0 \
                 regeneration (IR-07)",
                model.schema_version,
                crate::IR_SCHEMA_VERSION
            ),
        ));
    }
}

fn validate_shared_types(model: &IrModel, findings: &mut Vec<Finding>) {
    let mut seen = BTreeSet::new();
    for shared in &model.shared_types {
        if !seen.insert(shared.id.clone()) {
            findings.push(Finding::new(
                FindingKind::DuplicateId,
                shared.id.as_str(),
                "duplicate shared-type id; ids must be unique across the model so component \
                 references resolve unambiguously (CROSS-01)",
            ));
        }
        let mut members = BTreeSet::new();
        for member in &shared.members {
            if !members.insert(member.id.clone()) {
                findings.push(Finding::new(
                    FindingKind::DuplicateId,
                    member.id.as_str(),
                    format!(
                        "duplicate member id on shared type '{}'; each member id must be unique \
                         (g13-b003 R6.1)",
                        shared.id
                    ),
                ));
            }
        }
        if shared.members.is_empty() {
            findings.push(Finding::new(
                FindingKind::InvalidReference,
                shared.id.as_str(),
                "shared type declares no members; an enumerated shared type must define at \
                 least one member (g13-b003 R6.1)",
            ));
        }
    }
}

fn validate_conformance_vectors(model: &IrModel, findings: &mut Vec<Finding>) {
    let mut seen = BTreeSet::new();
    for vector in &model.conformance_vectors {
        if !seen.insert(vector.id.clone()) {
            findings.push(Finding::new(
                FindingKind::DuplicateId,
                vector.id.as_str(),
                "duplicate conformance-vector id; ids must be unique (CROSS-18)",
            ));
        }
        let mut steps = BTreeSet::new();
        for step in &vector.steps {
            if !steps.insert(step.id.clone()) {
                findings.push(Finding::new(
                    FindingKind::DuplicateId,
                    step.id.as_str(),
                    format!(
                        "duplicate step id on vector '{}'; step ids must be unique (CROSS-18)",
                        vector.id
                    ),
                ));
            }
        }
        if vector.applies_to.is_empty() {
            findings.push(Finding::new(
                FindingKind::InvalidReference,
                vector.id.as_str(),
                "conformance vector declares no runtime targets; add applies_to so each runtime \
                 machine knows it must implement the vector (CROSS-18)",
            ));
        }
    }
}

fn validate_component(
    model: &IrModel,
    component: &crate::ComponentDefinition,
    findings: &mut Vec<Finding>,
) {
    let scope = |_what: &str, id: &Identifier| format!("{}.{}", component.id, id);

    validate_component_props(model, component, findings, &scope);
    validate_component_controlled_state(component, findings, &scope);
    validate_component_events(component, findings, &scope);
    validate_component_parts(component, findings, &scope);
    validate_component_attributes(component, findings, &scope);
    validate_component_axes(component, findings, &scope);
    validate_component_tokens(component, findings, &scope);
    validate_component_accessibility(component, findings, &scope);
    validate_component_capabilities(component, findings, &scope);
    validate_component_keyboard(component, findings, &scope);
    validate_component_visual_state(component, findings, &scope);
    validate_component_conformance_refs(model, component, findings, &scope);
    validate_component_extensions(component, findings, &scope);
}

fn prop_by_id<'a>(component: &'a crate::ComponentDefinition, id: &str) -> Option<&'a Prop> {
    component.props.iter().find(|p| p.id.as_str() == id)
}

fn shared_member_ids(shared: &crate::SharedType) -> BTreeSet<String> {
    shared.members.iter().map(|m| m.id.to_string()).collect()
}

fn value_matches_type(value: &Value, prop_type: &PropType) -> bool {
    match (value, prop_type) {
        (Value::String(_), PropType::String | PropType::Opaque) => true,
        (Value::Number(_), PropType::Number) => true,
        (Value::Bool(_), PropType::Bool) => true,
        (Value::Member(_), PropType::Shared(_)) => true,
        (Value::Pair(a, b), PropType::Pair(inner)) => {
            value_matches_type(a, inner) && value_matches_type(b, inner)
        }
        (Value::List(items), PropType::List(inner)) => {
            items.iter().all(|v| value_matches_type(v, inner))
        }
        (Value::Null, PropType::Opaque) => true,
        // Null is a valid controlled empty state for any value-carrying prop
        // (TXT-02 "null is a valid controlled empty state").
        (Value::Null, PropType::String | PropType::Number | PropType::Bool) => true,
        _ => false,
    }
}

fn validate_component_props(
    model: &IrModel,
    component: &crate::ComponentDefinition,
    findings: &mut Vec<Finding>,
    scope: &dyn Fn(&str, &Identifier) -> String,
) {
    let mut seen = BTreeSet::new();
    for prop in &component.props {
        if !seen.insert(prop.id.clone()) {
            findings.push(Finding::new(
                FindingKind::DuplicateId,
                scope("prop", &prop.id),
                "duplicate prop id; each prop must be uniquely identifiable so bindings and \
                 defaults resolve (CROSS-02)",
            ));
        }
        // Shared-type references resolve (g13-b003 R6.1).
        let shared_id = match &prop.prop_type {
            PropType::Shared(id) => Some(id.to_string()),
            _ => None,
        };
        if let Some(ref_id) = &shared_id {
            if model.shared_type(ref_id).is_none() {
                findings.push(Finding::new(
                    FindingKind::InvalidReference,
                    scope("prop", &prop.id),
                    format!(
                        "prop references shared type '{ref_id}' which is not defined; define it \
                         in the model's shared_types (g13-b003 R6.1)"
                    ),
                ));
            }
        }
        // Permitted subset is only meaningful on shared-typed props and must
        // resolve (g13-b003 R6.2).
        if let Some(subset) = &prop.permitted_subset {
            if !prop.prop_type.is_shared() {
                findings.push(Finding::new(
                    FindingKind::InvalidReference,
                    scope("prop", &prop.id),
                    format!(
                        "permitted subset on non-shared prop '{}'; a subset constrains a shared \
                         enumerated type, so the prop type must be Shared (g13-b003 R6.2)",
                        prop.id
                    ),
                ));
            }
            validate_subset(model, subset, &scope("prop", &prop.id), findings);
        }
        // Default value checks: member defaults resolve against the shared
        // type and stay inside the permitted subset.
        if let Some(default) = &prop.default {
            let member = match default {
                Value::Member(id) => Some(id.to_string()),
                _ => None,
            };
            if let (Some(shared_id), Some(member_id)) = (&shared_id, &member) {
                let shared = model.shared_type(shared_id);
                match shared {
                    None => {
                        findings.push(Finding::new(
                            FindingKind::InvalidReference,
                            scope("prop", &prop.id),
                            format!(
                                "default member '{member_id}' references undefined shared type \
                                 '{shared_id}' (g13-b003 R6.1)"
                            ),
                        ));
                    }
                    Some(shared) => {
                        let ids = shared_member_ids(shared);
                        if !ids.contains(member_id) {
                            findings.push(Finding::new(
                                FindingKind::InvalidReference,
                                scope("prop", &prop.id),
                                format!(
                                    "default member '{member_id}' is not a member of shared type \
                                     '{shared_id}'; add it to the shared type or fix the default"
                                ),
                            ));
                        }
                        if let Some(subset) = &prop.permitted_subset {
                            if !subset.permits(member_id) {
                                findings.push(Finding::new(
                                    FindingKind::PermittedSubsetViolation,
                                    scope("prop", &prop.id),
                                    format!(
                                        "default member '{member_id}' is outside the permitted \
                                         subset of '{shared_id}' for prop '{}'; permitted \
                                         members are [{}] (g13-b003 R6.2)",
                                        prop.id,
                                        subset
                                            .members
                                            .iter()
                                            .map(|m| m.to_string())
                                            .collect::<Vec<_>>()
                                            .join(", ")
                                    ),
                                ));
                            }
                        }
                    }
                }
            } else if !value_matches_type(default, &prop.prop_type) {
                findings.push(Finding::new(
                    FindingKind::ImpossibleBinding,
                    scope("prop", &prop.id),
                    format!(
                        "default value does not match prop type; give '{}' a value of the \
                         declared type (CROSS-02)",
                        prop.id
                    ),
                ));
            }
        }
        // Web-only props mark the web surface excluded from the portable spec
        // (CROSS-03) — no validation rule needed beyond existence.
    }
}

fn validate_subset(
    model: &IrModel,
    subset: &PermittedSubset,
    where_id: &str,
    findings: &mut Vec<Finding>,
) {
    let shared = model.shared_type(subset.shared_type.as_str());
    let Some(shared) = shared else {
        findings.push(Finding::new(
            FindingKind::InvalidReference,
            where_id.to_owned(),
            format!(
                "permitted subset references shared type '{}' which is not defined; define it \
                 first (g13-b003 R6.2)",
                subset.shared_type
            ),
        ));
        return;
    };
    if subset.members.is_empty() {
        findings.push(Finding::new(
            FindingKind::InvalidReference,
            where_id.to_owned(),
            format!(
                "permitted subset of '{}' is empty; an empty subset permits nothing — declare \
                 at least one member (g13-b003 R6.2)",
                subset.shared_type
            ),
        ));
    }
    let ids = shared_member_ids(shared);
    for member in &subset.members {
        if !ids.contains(&member.to_string()) {
            findings.push(Finding::new(
                FindingKind::InvalidReference,
                where_id.to_owned(),
                format!(
                    "permitted subset member '{member}' is not a member of shared type '{}'; \
                     members are [{}]",
                    subset.shared_type,
                    ids.iter().cloned().collect::<Vec<_>>().join(", ")
                ),
            ));
        }
    }
}

fn validate_component_controlled_state(
    component: &crate::ComponentDefinition,
    findings: &mut Vec<Finding>,
    scope: &dyn Fn(&str, &Identifier) -> String,
) {
    let mut seen = BTreeSet::new();
    for state in &component.controlled_state {
        if !seen.insert(state.id.clone()) {
            findings.push(Finding::new(
                FindingKind::DuplicateId,
                scope("controlled-state", &state.id),
                "duplicate controlled-state id (CROSS-04)",
            ));
        }
        for (field, label) in [(&state.controlled, "controlled"), (&state.seed, "seed")] {
            if prop_by_id(component, field.as_str()).is_none() {
                findings.push(Finding::new(
                    FindingKind::InvalidReference,
                    scope("controlled-state", &state.id),
                    format!(
                        "{label} prop '{}' does not exist; add the prop or fix the reference \
                         (CROSS-04)",
                        field
                    ),
                ));
            }
        }
        if state.controlled == state.seed {
            findings.push(Finding::new(
                FindingKind::ImpossibleBinding,
                scope("controlled-state", &state.id),
                format!(
                    "controlled and seed reference the same prop '{}'; a controlled/uncontrolled \
                     pair needs two distinct props (CROSS-04)",
                    state.controlled
                ),
            ));
        }
    }
}

fn validate_component_events(
    component: &crate::ComponentDefinition,
    findings: &mut Vec<Finding>,
    scope: &dyn Fn(&str, &Identifier) -> String,
) {
    let mut seen = BTreeSet::new();
    for event in &component.events {
        if !seen.insert(event.id.clone()) {
            findings.push(Finding::new(
                FindingKind::DuplicateId,
                scope("event", &event.id),
                "duplicate event id; each event must be uniquely identifiable (CROSS-05)",
            ));
        }
        for ordering in &event.timing.ordering {
            let events = component
                .events
                .iter()
                .map(|e| e.id.to_string())
                .collect::<BTreeSet<_>>();
            for (ref_id, label) in [(&ordering.before, "before"), (&ordering.after, "after")] {
                if !events.contains(&ref_id.to_string()) {
                    findings.push(Finding::new(
                        FindingKind::InvalidReference,
                        scope("event", &event.id),
                        format!(
                            "ordering constraint names '{ref_id}' as {label}, but no sibling \
                             event has that id; fix the ordering reference (CROSS-06)"
                        ),
                    ));
                }
            }
        }
    }
}

fn validate_component_parts(
    component: &crate::ComponentDefinition,
    findings: &mut Vec<Finding>,
    scope: &dyn Fn(&str, &Identifier) -> String,
) {
    let mut seen = BTreeSet::new();
    let mut part_ids: BTreeSet<String> = BTreeSet::new();
    for part in &component.parts {
        if !seen.insert(part.id.clone()) {
            findings.push(Finding::new(
                FindingKind::DuplicateId,
                scope("part", &part.id),
                "duplicate part id; anatomy parts must be uniquely identifiable (CROSS-12)",
            ));
        }
        part_ids.insert(part.id.to_string());
    }
    for part in &component.parts {
        if let Some(parent) = &part.parent {
            if !part_ids.contains(&parent.to_string()) {
                findings.push(Finding::new(
                    FindingKind::InvalidReference,
                    scope("part", &part.id),
                    format!(
                        "part parent '{parent}' does not exist; add the parent part or fix the \
                         reference (CROSS-12)"
                    ),
                ));
            }
        }
        match &part.kind {
            PartKind::Conditional { when, .. } => {
                let prop = prop_by_id(component, when.as_str());
                match prop {
                    None => findings.push(Finding::new(
                        FindingKind::InvalidReference,
                        scope("part", &part.id),
                        format!(
                            "conditional part gates on prop '{when}' which does not exist; add \
                             the prop or fix the reference (CROSS-12)"
                        ),
                    )),
                    Some(prop) if prop.prop_type != PropType::Bool => findings.push(Finding::new(
                        FindingKind::ImpossibleBinding,
                        scope("part", &part.id),
                        format!(
                            "conditional part gates on prop '{when}' which is not boolean; a \
                             conditional node needs a boolean condition (CROSS-12)"
                        ),
                    )),
                    _ => {}
                }
            }
            PartKind::Repeated { over, .. } => {
                let prop = prop_by_id(component, over.as_str());
                match prop {
                    None => findings.push(Finding::new(
                        FindingKind::InvalidReference,
                        scope("part", &part.id),
                        format!(
                            "repeated part iterates over prop '{over}' which does not exist; add \
                             the prop or fix the reference (CROSS-12)"
                        ),
                    )),
                    Some(prop) if !matches!(prop.prop_type, PropType::List(_)) => {
                        findings.push(Finding::new(
                            FindingKind::ImpossibleBinding,
                            scope("part", &part.id),
                            format!(
                                "repeated part iterates over prop '{over}' which is not a list; \
                                 a repeated node needs a list source (CROSS-12)"
                            ),
                        ));
                    }
                    _ => {}
                }
            }
            PartKind::Static => {}
        }
    }
    // Parent/child cycles are unsupported (card scope "unsupported cycles").
    for part in &component.parts {
        let mut cursor = part.id.clone();
        let mut hops = 0usize;
        loop {
            let current = component.parts.iter().find(|p| p.id == cursor);
            let Some(current) = current else {
                break; // missing parent already reported above
            };
            let Some(parent) = &current.parent else {
                break;
            };
            if parent == &part.id {
                findings.push(Finding::new(
                    FindingKind::Cycle,
                    scope("part", &part.id),
                    format!(
                        "parent/child cycle detected: part '{}' is reachable from its own \
                         subtree; break the cycle by removing a parent reference (CROSS-12)",
                        part.id
                    ),
                ));
                break;
            }
            cursor = parent.clone();
            hops += 1;
            if hops > component.parts.len() {
                findings.push(Finding::new(
                    FindingKind::Cycle,
                    scope("part", &part.id),
                    format!(
                        "parent chain for part '{}' exceeds the number of parts, so a cycle is \
                         present; break the cycle by removing a parent reference (CROSS-12)",
                        part.id
                    ),
                ));
                break;
            }
        }
    }
}

fn validate_component_attributes(
    component: &crate::ComponentDefinition,
    findings: &mut Vec<Finding>,
    scope: &dyn Fn(&str, &Identifier) -> String,
) {
    let mut seen = BTreeSet::new();
    let mut known: BTreeSet<String> = component.props.iter().map(|p| p.id.to_string()).collect();
    known.extend(component.controlled_state.iter().map(|s| s.id.to_string()));
    // Attribute sources may also name VisualState projection fields, e.g.
    // the RangeSlider fill-geometry custom properties derived from
    // `lowerNorm` (RNG-17) and the TextInput adornment-padding reservation
    // (TXT-16).
    known.extend(
        component
            .visual_state
            .iter()
            .flat_map(|state| state.fields.iter())
            .map(|field| field.id.to_string()),
    );
    for attribute in &component.attributes {
        if !seen.insert(attribute.id.clone()) {
            findings.push(Finding::new(
                FindingKind::DuplicateId,
                scope("attribute", &attribute.id),
                "duplicate attribute id; state-derived attributes must be uniquely identifiable \
                 (CROSS-13)",
            ));
        }
        if let Some(source) = &attribute.source {
            if !known.contains(&source.to_string()) {
                findings.push(Finding::new(
                    FindingKind::InvalidReference,
                    scope("attribute", &attribute.id),
                    format!(
                        "attribute source '{source}' is neither a prop, a controlled state, nor \
                         a VisualState field; state-derived attributes must derive from declared \
                         state (CROSS-13)"
                    ),
                ));
            }
        }
        if attribute.form == AttributeForm::Valued && attribute.source.is_none() {
            findings.push(Finding::new(
                FindingKind::ImpossibleBinding,
                scope("attribute", &attribute.id),
                format!(
                    "valued attribute '{}' has no source; a valued attribute needs a source to \
                     derive its value from (CROSS-13)",
                    attribute.id
                ),
            ));
        }
    }
}

fn validate_component_axes(
    component: &crate::ComponentDefinition,
    findings: &mut Vec<Finding>,
    scope: &dyn Fn(&str, &Identifier) -> String,
) {
    let Axes {
        size,
        density,
        orientation,
    } = &component.axes;
    if let Some(size_axis) = size {
        let mut rungs = BTreeSet::new();
        for step in &size_axis.ladder {
            if !rungs.insert(step.size) {
                findings.push(Finding::new(
                    FindingKind::DuplicateId,
                    scope("size-step", &Identifier::new(format!("{:?}", step.size))),
                    format!(
                        "duplicate size rung '{:?}' in the size ladder; each ladder rung must \
                         be defined once (CROSS-07)",
                        step.size
                    ),
                ));
            }
        }
    }
    if let Some(density_axis) = density {
        let mut seen = BTreeSet::new();
        for adjustment in &density_axis.adjustments {
            let key = (
                adjustment.density,
                adjustment.applies_to.clone().map(|id| id.to_string()),
            );
            if !seen.insert(key.clone()) {
                findings.push(Finding::new(
                    FindingKind::DuplicateId,
                    scope("density-adjustment", &Identifier::new(format!("{:?}", key))),
                    format!(
                        "duplicate density adjustment for density '{:?}' and part '{}'; merge the \
                         adjustments (CROSS-08)",
                        adjustment.density,
                        adjustment
                            .applies_to
                            .as_ref()
                            .map(|id| id.to_string())
                            .unwrap_or_else(|| "(whole control)".to_owned())
                    ),
                ));
            }
        }
    }
    if let Some(orientation_axis) = orientation {
        if orientation_axis.values.is_empty() {
            findings.push(Finding::new(
                FindingKind::InvalidReference,
                scope("orientation", &Identifier::new("orientation")),
                "orientation axis declares no values; declare at least horizontal or vertical \
                 (CROSS-11)",
            ));
        }
    }
}

fn validate_component_tokens(
    component: &crate::ComponentDefinition,
    findings: &mut Vec<Finding>,
    scope: &dyn Fn(&str, &Identifier) -> String,
) {
    let semantic: BTreeSet<&str> = crate::tokens::semantic_token_paths().into_iter().collect();
    for token in &component.tokens {
        if !semantic.contains(token.path.as_str()) {
            findings.push(Finding::new(
                FindingKind::InvalidReference,
                scope("token", &Identifier::new(token.path.clone())),
                format!(
                    "token path '{}' does not resolve against poodle-tokens; use a path from the \
                     generated semantic registry, e.g. 'color.accent.base' (CROSS-09)",
                    token.path
                ),
            ));
        }
    }
    // Recipe hooks carry their own chain; the terminal token link resolves
    // against the same registry.
    for hook in &component.recipe_hooks {
        if !hook.hook.starts_with("--poodle-recipe-") {
            findings.push(Finding::new(
                FindingKind::InvalidReference,
                scope("recipe-hook", &Identifier::new(hook.hook.clone())),
                format!(
                    "recipe hook '{}' does not start with '--poodle-recipe-'; hooks follow the \
                     documented override-chain naming (CROSS-09)",
                    hook.hook
                ),
            ));
        }
    }
}

fn validate_component_accessibility(
    component: &crate::ComponentDefinition,
    findings: &mut Vec<Finding>,
    scope: &dyn Fn(&str, &Identifier) -> String,
) {
    use crate::{NameRule, NameSource};
    let a11y = &component.accessibility;
    let mut known: BTreeSet<String> = component.props.iter().map(|p| p.id.to_string()).collect();
    known.extend(component.controlled_state.iter().map(|s| s.id.to_string()));
    // Missing accessibility data: a required name with no source
    // (BTN-21 icon-only requires accessible name).
    if a11y.name_rule == NameRule::Required && a11y.name_source.is_none() {
        findings.push(Finding::new(
            FindingKind::MissingAccessibility,
            component.id.to_string(),
            format!(
                "component '{}' requires an accessible name (name_rule = required) but declares \
                 no name_source; add a content, prop, or external-label name source (BTN-21, \
                 CROSS-15)",
                component.id
            ),
        ));
    }
    if let NameRule::FromProp(prop) = &a11y.name_rule {
        if prop_by_id(component, prop.as_str()).is_none() {
            findings.push(Finding::new(
                FindingKind::InvalidReference,
                scope("name-rule", prop),
                format!(
                    "name rule references prop '{prop}' which does not exist; add the prop or \
                     fix the reference (CROSS-15)"
                ),
            ));
        }
    }
    if let Some(NameSource::Prop(prop)) = &a11y.name_source {
        if prop_by_id(component, prop.as_str()).is_none() {
            findings.push(Finding::new(
                FindingKind::InvalidReference,
                scope("name-source", prop),
                format!(
                    "name source references prop '{prop}' which does not exist; add the prop or \
                     fix the reference (CROSS-15)"
                ),
            ));
        }
    }
    for mapping in &a11y.aria {
        if !known.contains(&mapping.source.to_string()) {
            findings.push(Finding::new(
                FindingKind::InvalidReference,
                scope("aria-mapping", &Identifier::new(mapping.aria_attr.clone())),
                format!(
                    "aria mapping '{}' derives from '{}' which is neither a prop nor a \
                     controlled state; fix the source (CROSS-15)",
                    mapping.aria_attr, mapping.source
                ),
            ));
        }
    }
}

fn validate_component_capabilities(
    component: &crate::ComponentDefinition,
    findings: &mut Vec<Finding>,
    scope: &dyn Fn(&str, &Identifier) -> String,
) {
    let mut declared = BTreeSet::new();
    for requirement in &component.capabilities {
        if !declared.insert(requirement.capability) {
            findings.push(Finding::new(
                FindingKind::DuplicateId,
                scope(
                    "capability",
                    &Identifier::new(format!("{:?}", requirement.capability)),
                ),
                format!(
                    "capability '{:?}' is declared more than once; merge the requirements \
                     (CROSS-17)",
                    requirement.capability
                ),
            ));
        }
    }
    // Undeclared capabilities: a keyboard command whose delivery needs a
    // capability the component does not declare (IR-08).
    for command in &component.keyboard {
        if let Some(required) = command.requires {
            if !declared.contains(&required) {
                findings.push(Finding::new(
                    FindingKind::UndeclaredCapability,
                    scope("keyboard", &command.id),
                    format!(
                        "keyboard command '{}' requires capability '{:?}' (delivery is \
                         adapter-owned, CROSS-16) but the component does not declare it; add a \
                         CapabilityRequirement or drop the requires (IR-08)",
                        command.id, required
                    ),
                ));
            }
        }
    }
}

fn validate_component_keyboard(
    component: &crate::ComponentDefinition,
    findings: &mut Vec<Finding>,
    scope: &dyn Fn(&str, &Identifier) -> String,
) {
    let mut seen = BTreeSet::new();
    for command in &component.keyboard {
        if !seen.insert(command.id.clone()) {
            findings.push(Finding::new(
                FindingKind::DuplicateId,
                scope("keyboard", &command.id),
                "duplicate keyboard command id (CROSS-16)",
            ));
        }
        if command.keys.is_empty() {
            findings.push(Finding::new(
                FindingKind::InvalidReference,
                scope("keyboard", &command.id),
                format!(
                    "keyboard command '{}' declares no key chords; add at least one chord \
                     (CROSS-16)",
                    command.id
                ),
            ));
        }
    }
}

fn validate_component_visual_state(
    component: &crate::ComponentDefinition,
    findings: &mut Vec<Finding>,
    scope: &dyn Fn(&str, &Identifier) -> String,
) {
    let mut seen = BTreeSet::new();
    for state in &component.visual_state {
        if !seen.insert(state.id.clone()) {
            findings.push(Finding::new(
                FindingKind::DuplicateId,
                scope("visual-state", &state.id),
                "duplicate visual-state id (CROSS-14)",
            ));
        }
        let mut fields = BTreeSet::new();
        for field in &state.fields {
            if !fields.insert(field.id.clone()) {
                findings.push(Finding::new(
                    FindingKind::DuplicateId,
                    scope("visual-state-field", &field.id),
                    format!(
                        "duplicate field id '{}' in visual state '{}'; projection fields must be \
                         unique (CROSS-14)",
                        field.id, state.id
                    ),
                ));
            }
        }
    }
}

fn validate_component_conformance_refs(
    model: &IrModel,
    component: &crate::ComponentDefinition,
    findings: &mut Vec<Finding>,
    scope: &dyn Fn(&str, &Identifier) -> String,
) {
    for vector_id in &component.conformance {
        if model.conformance_vector(vector_id.as_str()).is_none() {
            findings.push(Finding::new(
                FindingKind::InvalidReference,
                scope("conformance", vector_id),
                format!(
                    "component references conformance vector '{vector_id}' which is not defined; \
                     add it to conformance_vectors (CROSS-18)"
                ),
            ));
        }
    }
}

fn validate_component_extensions(
    component: &crate::ComponentDefinition,
    findings: &mut Vec<Finding>,
    scope: &dyn Fn(&str, &Identifier) -> String,
) {
    let mut seen = BTreeSet::new();
    for extension in &component.extensions {
        if !seen.insert(extension.id.clone()) {
            findings.push(Finding::new(
                FindingKind::DuplicateId,
                scope("extension", &extension.id),
                "duplicate extension id (spec 063 capability/escape-hatch rules)",
            ));
        }
    }
}

fn validate_scenes(model: &IrModel, findings: &mut Vec<Finding>) {
    let mut seen_scenes = BTreeSet::new();
    for scene in &model.scenes {
        if !seen_scenes.insert(scene.id.clone()) {
            findings.push(Finding::new(
                FindingKind::DuplicateId,
                scene.id.to_string(),
                "duplicate scene id; scene ids must be unique so shells can cite them \
                 (CROSS-21)",
            ));
        }
        for instance in &scene.instances {
            let instance_id = format!("{}.{}", scene.id, instance.component);
            let Some(component) = model.component(instance.component.as_str()) else {
                findings.push(Finding::new(
                    FindingKind::InvalidReference,
                    instance_id.clone(),
                    format!(
                        "scene '{}' references component '{}' which is not defined; add the \
                         component or fix the reference (CROSS-21)",
                        scene.id, instance.component
                    ),
                ));
                continue;
            };
            validate_instance_bindings(model, component, instance, &instance_id, scene, findings);
        }
        validate_scene_axes(scene, findings);
        validate_scene_layout(model, scene, findings);
        if let Some(preview) = &scene.preview_state {
            validate_preview_state(scene, preview, findings);
        }
    }
}

fn validate_instance_bindings(
    model: &IrModel,
    component: &crate::ComponentDefinition,
    instance: &crate::ComponentInstance,
    instance_id: &str,
    scene: &crate::Scene,
    findings: &mut Vec<Finding>,
) {
    let mut bound = BTreeSet::new();
    for binding in &instance.bindings {
        let binding_id = format!("{instance_id}.{}", binding.prop);
        let Some(prop) = prop_by_id(component, binding.prop.as_str()) else {
            findings.push(Finding::new(
                FindingKind::ImpossibleBinding,
                binding_id.clone(),
                format!(
                    "binding targets prop '{}' which does not exist on component '{}'; add the \
                     prop or fix the binding (CROSS-02)",
                    binding.prop, component.id
                ),
            ));
            continue;
        };
        if !bound.insert(binding.prop.clone()) {
            findings.push(Finding::new(
                FindingKind::ImpossibleBinding,
                binding_id.clone(),
                format!(
                    "prop '{}' is bound more than once in scene '{}'; bind each prop at most \
                     once (CROSS-21)",
                    binding.prop, scene.id
                ),
            ));
        }
        // Value/type match (impossible prop bindings).
        if !value_matches_type(&binding.value, &prop.prop_type) {
            findings.push(Finding::new(
                FindingKind::ImpossibleBinding,
                binding_id.clone(),
                format!(
                    "binding value for prop '{}' does not match its declared type '{:?}'; give \
                     the binding a value of the declared type (CROSS-02)",
                    prop.id, prop.prop_type
                ),
            ));
            continue;
        }
        // Permitted-subset check on member values — the R6.2 rule.
        if let (Some(subset), Some(member)) = (&prop.permitted_subset, binding.value.as_member()) {
            if !subset.permits(member) {
                let shared = model.shared_type(subset.shared_type.as_str());
                let expected = shared
                    .map(|s| {
                        s.members
                            .iter()
                            .filter(|m| subset.permits(m.id.as_str()))
                            .map(|m| m.id.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    })
                    .unwrap_or_default();
                findings.push(Finding::new(
                    FindingKind::PermittedSubsetViolation,
                    binding_id.clone(),
                    format!(
                        "value '{member}' is outside the permitted subset of '{}' for prop '{}' \
                         in scene '{}'; permitted members are [{}] (g13-b003 R6.2)",
                        subset.shared_type,
                        prop.id,
                        scene.id,
                        if expected.is_empty() {
                            subset
                                .members
                                .iter()
                                .map(|m| m.to_string())
                                .collect::<Vec<_>>()
                                .join(", ")
                        } else {
                            expected
                        }
                    ),
                ));
            }
        }
        // Do-not-mix rule: binding both halves of a controlled pair
        // (CROSS-04, T §3).
        for state in &component.controlled_state {
            let both = bound.contains(&state.controlled) && bound.contains(&state.seed);
            if both {
                findings.push(Finding::new(
                    FindingKind::ImpossibleBinding,
                    binding_id.clone(),
                    format!(
                        "scene '{}' binds both '{}' and '{}' of controlled state '{}'; these are \
                         mutually exclusive (do-not-mix modes, CROSS-04; T §3)",
                        scene.id, state.controlled, state.seed, state.id
                    ),
                ));
            }
        }
    }
}

fn validate_scene_axes(scene: &crate::Scene, findings: &mut Vec<Finding>) {
    let mut seen = BTreeSet::new();
    for axis in &scene.axes {
        if !seen.insert(axis.kind) {
            findings.push(Finding::new(
                FindingKind::DuplicateId,
                format!("{}.axis.{:?}", scene.id, axis.kind),
                format!(
                    "scene '{}' declares axis '{:?}' more than once; declare each axis at most \
                     once (spec 063 Scene IR)",
                    scene.id, axis.kind
                ),
            ));
        }
        match (&axis.kind, &axis.values) {
            (SceneAxisKind::Contrast, crate::AxisValues::Continuous { min, max, default }) => {
                if min > max {
                    findings.push(Finding::new(
                        FindingKind::ImpossibleBinding,
                        format!("{}.axis.contrast", scene.id),
                        format!(
                            "contrast axis range is inverted (min {min} > max {max}); min must \
                             be ≤ max (CROSS-10, SHELL-04)"
                        ),
                    ));
                }
                if !(min <= default && default <= max) {
                    findings.push(Finding::new(
                        FindingKind::ImpossibleBinding,
                        format!("{}.axis.contrast", scene.id),
                        format!(
                            "contrast axis default {default} is outside [{min}, {max}]; move the \
                             default inside the range (CROSS-10, SHELL-04)"
                        ),
                    ));
                }
            }
            (SceneAxisKind::Contrast, crate::AxisValues::Named(_)) => {
                findings.push(Finding::new(
                    FindingKind::ImpossibleBinding,
                    format!("{}.axis.contrast", scene.id),
                    "contrast is a continuous axis (neutral-contrast override, CROSS-10); use \
                     continuous values, not named values (SHELL-04)",
                ));
            }
            (SceneAxisKind::Theme, crate::AxisValues::Named(values)) => {
                let themes: BTreeSet<&str> = crate::tokens::theme_names().into_iter().collect();
                for value in values {
                    if !themes.contains(value.as_str()) {
                        findings.push(Finding::new(
                            FindingKind::InvalidReference,
                            format!("{}.axis.theme.{}", scene.id, value),
                            format!(
                                "theme '{}' is not a poodle-tokens theme preset; available \
                                 presets are [{}] (SHELL-01, CROSS-09)",
                                value,
                                themes.iter().copied().collect::<Vec<_>>().join(", ")
                            ),
                        ));
                    }
                }
            }
            (SceneAxisKind::Size, crate::AxisValues::Named(values)) => {
                let sizes: BTreeSet<&str> =
                    crate::tokens::control_size_names().into_iter().collect();
                for value in values {
                    if !sizes.contains(value.as_str()) {
                        findings.push(Finding::new(
                            FindingKind::InvalidReference,
                            format!("{}.axis.size.{}", scene.id, value),
                            format!(
                                "control size '{}' is not a poodle-tokens control size; \
                                 available sizes are [{}] (SHELL-02, CROSS-07)",
                                value,
                                sizes.iter().copied().collect::<Vec<_>>().join(", ")
                            ),
                        ));
                    }
                }
            }
            (SceneAxisKind::Density, crate::AxisValues::Named(values)) => {
                let densities: BTreeSet<&str> =
                    crate::tokens::density_names().into_iter().collect();
                for value in values {
                    if !densities.contains(value.as_str()) {
                        findings.push(Finding::new(
                            FindingKind::InvalidReference,
                            format!("{}.axis.density.{}", scene.id, value),
                            format!(
                                "density '{}' is not a poodle-tokens density preset; available \
                                 presets are [{}] (SHELL-03, CROSS-08)",
                                value,
                                densities.iter().copied().collect::<Vec<_>>().join(", ")
                            ),
                        ));
                    }
                }
            }
            _ => {}
        }
    }
}

fn validate_scene_layout(model: &IrModel, scene: &crate::Scene, findings: &mut Vec<Finding>) {
    let Some(layout) = &scene.layout else {
        return;
    };
    for group in layout.sections.iter().flat_map(|section| &section.groups) {
        for component_id in &group.components {
            if model.component(component_id.as_str()).is_none() {
                findings.push(Finding::new(
                    FindingKind::InvalidReference,
                    format!("{}.group.{}", scene.id, group.title),
                    format!(
                        "navigation group '{}' references component '{component_id}' which is \
                         not defined (SHELL-05)",
                        group.title
                    ),
                ));
            }
        }
    }
}

fn validate_preview_state(
    scene: &crate::Scene,
    preview: &crate::PreviewState,
    findings: &mut Vec<Finding>,
) {
    let id = |name: &str| format!("{}.preview-state.{}", scene.id, name);
    if let Some(theme) = &preview.theme {
        let themes: BTreeSet<&str> = crate::tokens::theme_names().into_iter().collect();
        if !themes.contains(theme.as_str()) {
            findings.push(Finding::new(
                FindingKind::InvalidReference,
                id("theme"),
                format!("preview state theme '{theme}' is not a poodle-tokens preset (SHELL-08)"),
            ));
        }
    }
    if let Some(density) = &preview.density {
        let densities: BTreeSet<&str> = crate::tokens::density_names().into_iter().collect();
        if !densities.contains(density.as_str()) {
            findings.push(Finding::new(
                FindingKind::InvalidReference,
                id("density"),
                format!(
                    "preview state density '{density}' is not a poodle-tokens preset (SHELL-08)"
                ),
            ));
        }
    }
    if let Some(size) = &preview.control_size {
        let sizes: BTreeSet<&str> = crate::tokens::control_size_names().into_iter().collect();
        if !sizes.contains(size.as_str()) {
            findings.push(Finding::new(
                FindingKind::InvalidReference,
                id("control-size"),
                format!(
                    "preview state control size '{size}' is not a poodle-tokens control size \
                     (SHELL-08)"
                ),
            ));
        }
    }
}

fn validate_registry(model: &IrModel, findings: &mut Vec<Finding>) {
    let Some(registry) = &model.specimen_registry else {
        return;
    };
    let mut seen = BTreeSet::new();
    for entry in &registry.entries {
        if !seen.insert(entry.id.clone()) {
            findings.push(Finding::new(
                FindingKind::DuplicateId,
                entry.id.to_string(),
                "duplicate specimen-registry entry id (SHELL-10)",
            ));
        }
        if model.component(entry.component.as_str()).is_none() {
            findings.push(Finding::new(
                FindingKind::InvalidReference,
                entry.id.to_string(),
                format!(
                    "specimen-registry entry references component '{}' which is not defined \
                     (SHELL-10)",
                    entry.component
                ),
            ));
        }
        for scene_id in &entry.scenes {
            if model.scene(scene_id.as_str()).is_none() {
                findings.push(Finding::new(
                    FindingKind::InvalidReference,
                    entry.id.to_string(),
                    format!(
                        "specimen-registry entry references scene '{scene_id}' which is not \
                         defined (SHELL-10)"
                    ),
                ));
            }
        }
    }
}
