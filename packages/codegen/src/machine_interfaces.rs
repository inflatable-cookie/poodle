//! Machine-interface schema (spec 064 mechanism 1).
//!
//! Parallel to [`crate::model`]: load and validate JSON, never an `IrModel`.
//! The schema holds types and names only — no transitions, guards, or
//! derivation. Emission lives in [`crate::targets::machine_ts`] and
//! [`crate::targets::machine_rust`].

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::error::{CodegenError, Result};

/// Schema version this emitter accepts. Bump only with a matching emitter
/// change; the header stamps this number.
pub const SCHEMA_VERSION: u32 = 1;

/// Authoritative machine-interface document.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Document {
    /// Schema version. Must equal [`SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Machines in authoring order; emitters sort by [`MachineInterface::id`].
    pub machines: Vec<MachineInterface>,
}

/// One machine's public type surface.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MachineInterface {
    /// Stable id and generated filename stem, e.g. `hover`.
    pub id: String,
    /// Type-name prefix, e.g. `Hover` → `HoverState`.
    pub prefix: String,
    /// Extra exported unions/enums used by context, events, or effects.
    #[serde(default)]
    pub named_types: Vec<NamedType>,
    /// State members, authoring order.
    pub states: Vec<StateMember>,
    /// Context struct / interface.
    pub context: Context,
    /// Event union / enum.
    pub events: VariantSet,
    /// Effect union / enum.
    pub effects: VariantSet,
}

/// A string-union member (TS) paired with its Rust enum variant.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StateMember {
    /// TypeScript string member, e.g. `closed`.
    pub ts: String,
    /// Rust variant, e.g. `Closed`.
    pub rs: String,
}

/// A named exported type (TS string union, Rust enum).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NamedType {
    /// Public type name in both languages, e.g. `PopoverInitialFocus`.
    pub name: String,
    /// Whether the Rust enum is `Copy`.
    pub copy: bool,
    /// Whether the Rust enum is `Eq`.
    pub eq: bool,
    /// Members, authoring order.
    pub variants: Vec<StateMember>,
}

/// Context fields plus the Rust trait set of the existing public struct.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Context {
    /// Whether the Rust struct is `Copy`.
    pub copy: bool,
    /// Whether the Rust struct is `Eq`.
    pub eq: bool,
    /// Whether the Rust struct derives `Default`.
    #[serde(default, rename = "default")]
    pub rs_default: bool,
    /// Fields, authoring order.
    pub fields: Vec<Field>,
}

/// Event or effect variants plus the Rust trait set of the existing enum.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VariantSet {
    /// Whether the Rust enum is `Copy`.
    pub copy: bool,
    /// Whether the Rust enum is `Eq`.
    pub eq: bool,
    /// Variants, authoring order.
    pub variants: Vec<Variant>,
}

/// One event or effect variant.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Variant {
    /// TypeScript discriminant, e.g. `ENTER` or `startTimer`.
    pub ts_type: String,
    /// Rust variant, e.g. `Enter` or `StartTimer`.
    pub rs_variant: String,
    /// Payload fields; empty for a unit variant.
    #[serde(default)]
    pub fields: Vec<Field>,
}

/// A context, event, or effect field.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Field {
    /// TypeScript field name.
    pub ts_name: String,
    /// Rust field name.
    pub rs_name: String,
    /// `boolean`, `number`, `string`, or a named type on the same machine.
    #[serde(rename = "type")]
    pub ty: TypeRef,
    /// TypeScript optional (`?:`). Rust stays a required field.
    #[serde(default)]
    pub ts_optional: bool,
}

/// Primitive or named field type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeRef {
    /// TS `boolean` / Rust `bool`.
    Boolean,
    /// TS `number` / Rust `f64`.
    Number,
    /// TS `string` / Rust `String`.
    String,
    /// A [`NamedType::name`] on the same machine.
    Named(String),
}

impl<'de> Deserialize<'de> for TypeRef {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let raw = String::deserialize(deserializer)?;
        Ok(match raw.as_str() {
            "boolean" => TypeRef::Boolean,
            "number" => TypeRef::Number,
            "string" => TypeRef::String,
            other => TypeRef::Named(other.to_owned()),
        })
    }
}

impl TypeRef {
    /// TypeScript rendering.
    pub fn ts(&self) -> &str {
        match self {
            TypeRef::Boolean => "boolean",
            TypeRef::Number => "number",
            TypeRef::String => "string",
            TypeRef::Named(name) => name,
        }
    }

    /// Rust rendering.
    pub fn rs(&self) -> &str {
        match self {
            TypeRef::Boolean => "bool",
            TypeRef::Number => "f64",
            TypeRef::String => "String",
            TypeRef::Named(name) => name,
        }
    }

    fn is_string(&self) -> bool {
        matches!(self, TypeRef::String)
    }

    fn is_number(&self) -> bool {
        matches!(self, TypeRef::Number)
    }
}

/// Reads and validates a machine-interface schema. Never panics on bad input.
pub fn load_and_validate(path: &Path) -> Result<Document> {
    let source = fs::read_to_string(path).map_err(|error| CodegenError::Read {
        path: path.to_path_buf(),
        source: error,
    })?;

    let document: Document = serde_json::from_str(&source).map_err(|error| CodegenError::Gate {
        message: format!(
            "{} is not valid machine-interface JSON: {error}",
            path.display()
        ),
    })?;

    let findings = validate(&document);
    if !findings.is_empty() {
        let mut message = format!("{} failed machine-interface validation:", path.display());
        for finding in findings {
            message.push_str("\n  - ");
            message.push_str(&finding);
        }
        return Err(CodegenError::Gate { message });
    }

    Ok(document)
}

fn validate(document: &Document) -> Vec<String> {
    let mut findings = Vec::new();

    if document.schema_version != SCHEMA_VERSION {
        findings.push(format!(
            "schemaVersion: expected {SCHEMA_VERSION}, found {}",
            document.schema_version
        ));
    }
    if document.machines.is_empty() {
        findings.push("machines: at least one machine is required".to_owned());
    }

    let mut ids = BTreeSet::new();
    for machine in &document.machines {
        validate_machine(machine, &mut findings);
        if !ids.insert(machine.id.as_str()) {
            findings.push(format!("machines: duplicate id '{}'", machine.id));
        }
    }

    findings
}

fn validate_machine(machine: &MachineInterface, findings: &mut Vec<String>) {
    let id = machine.id.as_str();
    if machine.id.is_empty() {
        findings.push("machine id is empty".to_owned());
    }
    if machine.prefix.is_empty() {
        findings.push(format!("{id}: prefix is empty"));
    }
    if machine.states.is_empty() {
        findings.push(format!("{id}: states must not be empty"));
    }

    let mut named = BTreeSet::new();
    for named_type in &machine.named_types {
        if named_type.name.is_empty() {
            findings.push(format!("{id}: named type name is empty"));
        } else if !named.insert(named_type.name.as_str()) {
            findings.push(format!("{id}: duplicate named type '{}'", named_type.name));
        }
        if named_type.variants.is_empty() {
            findings.push(format!("{id}: named type '{}' has no variants", named_type.name));
        }
        validate_members(
            id,
            &format!("named type '{}'", named_type.name),
            &named_type.variants,
            findings,
        );
    }

    validate_members(id, "states", &machine.states, findings);

    let named_names: BTreeSet<&str> = machine.named_types.iter().map(|n| n.name.as_str()).collect();
    validate_fields(id, "context", &machine.context.fields, &named_names, findings);
    validate_copy_eq(
        id,
        "context",
        machine.context.copy,
        machine.context.eq,
        machine.context.fields.iter(),
        findings,
    );

    validate_variant_set(id, "events", &machine.events, &named_names, findings);
    validate_variant_set(id, "effects", &machine.effects, &named_names, findings);
}

fn validate_members(id: &str, what: &str, members: &[StateMember], findings: &mut Vec<String>) {
    let mut ts = BTreeSet::new();
    let mut rs = BTreeSet::new();
    for member in members {
        if member.ts.is_empty() || member.rs.is_empty() {
            findings.push(format!("{id}: {what} has an empty ts/rs name"));
        }
        if !ts.insert(member.ts.as_str()) {
            findings.push(format!("{id}: {what} duplicate ts '{}'", member.ts));
        }
        if !rs.insert(member.rs.as_str()) {
            findings.push(format!("{id}: {what} duplicate rs '{}'", member.rs));
        }
    }
}

fn validate_variant_set(
    id: &str,
    what: &str,
    set: &VariantSet,
    named: &BTreeSet<&str>,
    findings: &mut Vec<String>,
) {
    if set.variants.is_empty() {
        findings.push(format!("{id}: {what} must not be empty"));
    }
    let mut ts = BTreeSet::new();
    let mut rs = BTreeSet::new();
    let mut fields = Vec::new();
    for variant in &set.variants {
        if variant.ts_type.is_empty() || variant.rs_variant.is_empty() {
            findings.push(format!("{id}: {what} has an empty tsType/rsVariant"));
        }
        if !ts.insert(variant.ts_type.as_str()) {
            findings.push(format!("{id}: {what} duplicate tsType '{}'", variant.ts_type));
        }
        if !rs.insert(variant.rs_variant.as_str()) {
            findings.push(format!("{id}: {what} duplicate rsVariant '{}'", variant.rs_variant));
        }
        validate_fields(id, &format!("{what}.{}", variant.ts_type), &variant.fields, named, findings);
        fields.extend(&variant.fields);
    }
    validate_copy_eq(id, what, set.copy, set.eq, fields, findings);
}

fn validate_fields(
    id: &str,
    what: &str,
    fields: &[Field],
    named: &BTreeSet<&str>,
    findings: &mut Vec<String>,
) {
    let mut ts = BTreeSet::new();
    let mut rs = BTreeSet::new();
    for field in fields {
        if field.ts_name.is_empty() || field.rs_name.is_empty() {
            findings.push(format!("{id}: {what} has an empty field name"));
        }
        if !ts.insert(field.ts_name.as_str()) {
            findings.push(format!("{id}: {what} duplicate tsName '{}'", field.ts_name));
        }
        if !rs.insert(field.rs_name.as_str()) {
            findings.push(format!("{id}: {what} duplicate rsName '{}'", field.rs_name));
        }
        if let TypeRef::Named(name) = &field.ty {
            if !named.contains(name.as_str()) {
                findings.push(format!(
                    "{id}: {what} field '{}' references unknown type '{name}'",
                    field.ts_name
                ));
            }
        }
    }
}

fn validate_copy_eq<'a>(
    id: &str,
    what: &str,
    copy: bool,
    eq: bool,
    fields: impl IntoIterator<Item = &'a Field>,
    findings: &mut Vec<String>,
) {
    let fields: Vec<&Field> = fields.into_iter().collect();
    if copy && fields.iter().any(|field| field.ty.is_string()) {
        findings.push(format!(
            "{id}: {what} is Copy but has a string field (String is not Copy)"
        ));
    }
    if eq && fields.iter().any(|field| field.ty.is_number()) {
        findings.push(format!(
            "{id}: {what} is Eq but has a number field (f64 is not Eq)"
        ));
    }
}

/// Machines in stable id order. Emitters use this so authoring order cannot
/// reorder artifacts.
pub fn machines_sorted(document: &Document) -> Vec<&MachineInterface> {
    let mut machines: Vec<&MachineInterface> = document.machines.iter().collect();
    machines.sort_by(|a, b| a.id.cmp(&b.id));
    machines
}
