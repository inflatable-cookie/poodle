//! Preview catalogue manifest (g14.025).
//!
//! Parallel to [`crate::machine_interfaces`]: load and validate JSON, never an
//! `IrModel`. The manifest owns preview discovery metadata only — section,
//! family, kind, optional collections, and canonical identity. Package names
//! and specimen availability stay in runtime overlays.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::error::{CodegenError, Result};

/// Schema version this emitter accepts. Bump only with a matching emitter
/// change; generated headers stamp this number.
pub const SCHEMA_VERSION: u32 = 1;

/// Fixed section order from g14.025.
pub const REQUIRED_SECTIONS: &[&str] = &["foundations", "composition", "systems"];

/// Fixed family order from g14.025, grouped by section.
pub const REQUIRED_FAMILIES: &[(&str, &str)] = &[
    ("actions-selection", "foundations"),
    ("text-value-entry", "foundations"),
    ("date-time", "foundations"),
    ("layout", "foundations"),
    ("content-identity", "foundations"),
    ("status-progress", "foundations"),
    ("navigation", "composition"),
    ("overlays-disclosure", "composition"),
    ("forms-validation", "composition"),
    ("data-collections", "composition"),
    ("media", "composition"),
    ("application-shell", "systems"),
    ("agent-tools", "systems"),
    ("model-connections", "systems"),
    ("audio-music", "systems"),
    ("account-lifecycle", "systems"),
];

/// Closed kind vocabulary from g14.025.
pub const REQUIRED_KINDS: &[&str] = &[
    "control",
    "input",
    "layout",
    "display",
    "overlay",
    "navigation",
    "data",
    "media",
    "feedback",
    "form",
    "composite",
];

const FORBIDDEN_FAMILY_IDS: &[&str] = &["general", "miscellaneous", "other", "workstation"];

/// Authoritative preview-catalogue document.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Document {
    /// Schema version. Must equal [`SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Broad navigation tiers, required order.
    pub sections: Vec<NamedEntry>,
    /// Primary sidebar homes, required order, each bound to one section.
    pub families: Vec<FamilyEntry>,
    /// Anatomy/filter vocabulary.
    pub kinds: Vec<NamedEntry>,
    /// Optional secondary discovery groupings.
    #[serde(default)]
    pub collections: Vec<NamedEntry>,
    /// Canonical component identity and classification.
    pub components: Vec<ComponentEntry>,
}

/// A labelled vocabulary member (`section`, `kind`, or `collection`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NamedEntry {
    /// Stable kebab-case id.
    pub id: String,
    /// Sidebar / breadcrumb label.
    pub label: String,
}

/// A primary family bound to exactly one section.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FamilyEntry {
    /// Stable kebab-case id.
    pub id: String,
    /// Parent section id.
    pub section: String,
    /// Sidebar heading.
    pub label: String,
}

/// One canonical catalogue component. No package or specimen fields.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ComponentEntry {
    /// Route identity (`#components/<slug>`).
    pub slug: String,
    /// Adopter-facing name.
    pub display_name: String,
    /// One-line catalogue description.
    pub description: String,
    /// Broad navigation tier.
    pub section: String,
    /// Exactly one primary sidebar home.
    pub family: String,
    /// Anatomy/filter kind.
    pub kind: String,
    /// Optional secondary collections; never a second sidebar home.
    #[serde(default)]
    pub collections: Vec<String>,
}

/// Reads and validates a preview-catalogue manifest. Never panics on bad input.
pub fn load_and_validate(path: &Path) -> Result<Document> {
    let source = fs::read_to_string(path).map_err(|error| CodegenError::Read {
        path: path.to_path_buf(),
        source: error,
    })?;

    let document: Document = serde_json::from_str(&source).map_err(|error| CodegenError::Gate {
        message: format!("{} is not valid catalogue JSON: {error}", path.display()),
    })?;

    let findings = validate(&document);
    if !findings.is_empty() {
        let mut message = format!("{} failed catalogue validation:", path.display());
        for finding in findings {
            message.push_str("\n  - ");
            message.push_str(&finding);
        }
        return Err(CodegenError::Gate { message });
    }

    Ok(document)
}

/// All-findings validation. Public for planted-failure tests.
pub fn validate(document: &Document) -> Vec<String> {
    let mut findings = Vec::new();

    if document.schema_version != SCHEMA_VERSION {
        findings.push(format!(
            "schemaVersion: expected {SCHEMA_VERSION}, found {}",
            document.schema_version
        ));
    }

    validate_named_list("sections", REQUIRED_SECTIONS, &document.sections, &mut findings);
    validate_kinds(&document.kinds, &mut findings);
    validate_families(&document.families, &mut findings);
    validate_named_unique("collections", &document.collections, &mut findings);

    let sections: BTreeSet<&str> = document.sections.iter().map(|e| e.id.as_str()).collect();
    let families: BTreeMap<&str, &FamilyEntry> = document
        .families
        .iter()
        .map(|family| (family.id.as_str(), family))
        .collect();
    let kinds: BTreeSet<&str> = document.kinds.iter().map(|e| e.id.as_str()).collect();
    let collections: BTreeSet<&str> = document.collections.iter().map(|e| e.id.as_str()).collect();

    if document.components.is_empty() {
        findings.push("components: at least one component is required".to_owned());
    }

    let mut slugs = BTreeSet::new();
    let mut family_counts: BTreeMap<&str, usize> = document
        .families
        .iter()
        .map(|family| (family.id.as_str(), 0usize))
        .collect();

    for component in &document.components {
        validate_component(
            component,
            &sections,
            &families,
            &kinds,
            &collections,
            &mut slugs,
            &mut family_counts,
            &mut findings,
        );
    }

    for (id, count) in family_counts {
        if count == 0 {
            findings.push(format!("family '{id}' is empty"));
        }
    }

    findings
}

fn validate_named_list(
    what: &str,
    required: &[&str],
    entries: &[NamedEntry],
    findings: &mut Vec<String>,
) {
    let ids: Vec<&str> = entries.iter().map(|entry| entry.id.as_str()).collect();
    if ids != required {
        findings.push(format!(
            "{what}: expected [{}], found [{}]",
            required.join(", "),
            ids.join(", ")
        ));
    }
    validate_named_unique(what, entries, findings);
}

fn validate_kinds(entries: &[NamedEntry], findings: &mut Vec<String>) {
    let ids: BTreeSet<&str> = entries.iter().map(|entry| entry.id.as_str()).collect();
    let required: BTreeSet<&str> = REQUIRED_KINDS.iter().copied().collect();
    if ids != required {
        findings.push(format!(
            "kinds: expected closed vocabulary [{}], found [{}]",
            REQUIRED_KINDS.join(", "),
            entries
                .iter()
                .map(|entry| entry.id.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    validate_named_unique("kinds", entries, findings);
}

fn validate_families(entries: &[FamilyEntry], findings: &mut Vec<String>) {
    let found: Vec<(&str, &str)> = entries
        .iter()
        .map(|family| (family.id.as_str(), family.section.as_str()))
        .collect();
    if found != REQUIRED_FAMILIES {
        findings.push(format!(
            "families: expected [{}], found [{}]",
            REQUIRED_FAMILIES
                .iter()
                .map(|(id, section)| format!("{id}@{section}"))
                .collect::<Vec<_>>()
                .join(", "),
            found
                .iter()
                .map(|(id, section)| format!("{id}@{section}"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    let mut ids = BTreeSet::new();
    for family in entries {
        if !is_kebab(&family.id) {
            findings.push(format!("family id '{}' is not kebab-case", family.id));
        }
        if family.label.trim().is_empty() {
            findings.push(format!("family '{}' has an empty label", family.id));
        }
        if FORBIDDEN_FAMILY_IDS.contains(&family.id.as_str()) {
            findings.push(format!(
                "family '{}' is a forbidden fallback bucket",
                family.id
            ));
        }
        if !ids.insert(family.id.as_str()) {
            findings.push(format!("families: duplicate id '{}'", family.id));
        }
    }
}

fn validate_named_unique(what: &str, entries: &[NamedEntry], findings: &mut Vec<String>) {
    let mut ids = BTreeSet::new();
    for entry in entries {
        if !is_kebab(&entry.id) {
            findings.push(format!("{what} id '{}' is not kebab-case", entry.id));
        }
        if entry.label.trim().is_empty() {
            findings.push(format!("{what} '{}' has an empty label", entry.id));
        }
        if !ids.insert(entry.id.as_str()) {
            findings.push(format!("{what}: duplicate id '{}'", entry.id));
        }
    }
}

    #[allow(clippy::too_many_arguments)]
    fn validate_component(
    component: &ComponentEntry,
    sections: &BTreeSet<&str>,
    families: &BTreeMap<&str, &FamilyEntry>,
    kinds: &BTreeSet<&str>,
    collections: &BTreeSet<&str>,
    slugs: &mut BTreeSet<String>,
    family_counts: &mut BTreeMap<&str, usize>,
    findings: &mut Vec<String>,
) {
    let ident = if component.slug.is_empty() {
        component.display_name.as_str()
    } else {
        component.slug.as_str()
    };

    if !is_kebab(&component.slug) {
        findings.push(format!("{ident}: slug is missing or not kebab-case"));
    }
    if component.display_name.trim().is_empty() {
        findings.push(format!("{ident}: displayName is empty"));
    }
    if component.description.trim().is_empty() {
        findings.push(format!("{ident}: description is empty"));
    }
    if component.section.trim().is_empty()
        || component.family.trim().is_empty()
        || component.kind.trim().is_empty()
    {
        findings.push(format!("{ident}: missing classification"));
    }
    if !slugs.insert(component.slug.clone()) {
        findings.push(format!("duplicate slug '{}'", component.slug));
    }

    match families.get(component.family.as_str()) {
        None => findings.push(format!(
            "{ident}: unknown family '{}'",
            component.family
        )),
        Some(family) => {
            if family.section != component.section {
                findings.push(format!(
                    "{ident}: section '{}' does not match family '{}' (section '{}')",
                    component.section, family.id, family.section
                ));
            }
            if let Some(count) = family_counts.get_mut(family.id.as_str()) {
                *count += 1;
            }
        }
    }

    if !component.section.is_empty() && !sections.contains(component.section.as_str()) {
        findings.push(format!("{ident}: unknown section '{}'", component.section));
    }
    if !component.kind.is_empty() && !kinds.contains(component.kind.as_str()) {
        findings.push(format!("{ident}: unknown kind '{}'", component.kind));
    }
    for collection in &component.collections {
        if !collections.contains(collection.as_str()) {
            findings.push(format!(
                "{ident}: unknown collection '{collection}'"
            ));
        }
    }
}

fn is_kebab(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !first.is_ascii_lowercase() {
        return false;
    }
    let mut prev_hyphen = false;
    for ch in chars {
        match ch {
            'a'..='z' | '0'..='9' => prev_hyphen = false,
            '-' if !prev_hyphen => prev_hyphen = true,
            _ => return false,
        }
    }
    !prev_hyphen
}

/// Components in sidebar order: family order, then display name.
pub fn components_in_order(document: &Document) -> Vec<&ComponentEntry> {
    let family_index: BTreeMap<&str, usize> = document
        .families
        .iter()
        .enumerate()
        .map(|(index, family)| (family.id.as_str(), index))
        .collect();
    let mut components: Vec<&ComponentEntry> = document.components.iter().collect();
    components.sort_by(|a, b| {
        family_index
            .get(a.family.as_str())
            .cmp(&family_index.get(b.family.as_str()))
            .then_with(|| a.display_name.cmp(&b.display_name))
            .then_with(|| a.slug.cmp(&b.slug))
    });
    components
}

/// PascalCase identifier for a kebab-case id.
pub fn pascal_case(id: &str) -> String {
    id.split('-')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
            }
        })
        .collect()
}
