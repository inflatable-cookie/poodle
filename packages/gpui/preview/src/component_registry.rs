//! GPUI preview catalogue overlay.
//!
//! Canonical identity, section/family/kind, and search fields come from the
//! generated catalogue. This file adds GPUI package naming and contract paths.
//! Unknown slugs return `None` — there is no Workstation fallback.

#[path = "generated/catalogue/catalogue.rs"]
mod generated;

pub use generated::{
    family_by_id, find_component, matches_search, search_components, CanonicalComponent,
    CanonicalComponent as ComponentEntry, CatalogueFamilyId, CatalogueSectionId,
    CANONICAL_COMPONENTS, CATALOGUE_FAMILIES, CATALOGUE_SECTIONS,
};

pub struct FamilyGroup {
    pub family: CatalogueFamilyId,
    pub label: &'static str,
    pub items: Vec<&'static CanonicalComponent>,
}

pub struct SectionGroup {
    pub section: CatalogueSectionId,
    pub label: &'static str,
    pub families: Vec<FamilyGroup>,
}

pub fn package_name() -> &'static str {
    "poodle-gpui-components"
}

pub fn contract_root() -> &'static str {
    "docs/contracts/components/"
}

pub fn contract_doc_path(slug: &str) -> String {
    format!("{}{}.md", contract_root(), slug)
}

pub fn all_components() -> Vec<&'static CanonicalComponent> {
    CANONICAL_COMPONENTS.iter().collect()
}

pub fn grouped_sections(search: &str) -> Vec<SectionGroup> {
    let query = search.trim();
    CATALOGUE_SECTIONS
        .iter()
        .copied()
        .filter_map(|section| {
            let families: Vec<FamilyGroup> = CATALOGUE_FAMILIES
                .iter()
                .filter(|family| family.section == section)
                .filter_map(|family| {
                    let items: Vec<&'static CanonicalComponent> = CANONICAL_COMPONENTS
                        .iter()
                        .filter(|component| component.family == family.id)
                        .filter(|component| matches_search(component, query))
                        .collect();
                    if items.is_empty() {
                        None
                    } else {
                        Some(FamilyGroup {
                            family: family.id,
                            label: family.label,
                            items,
                        })
                    }
                })
                .collect();
            if families.is_empty() {
                None
            } else {
                Some(SectionGroup {
                    section,
                    label: section.label(),
                    families,
                })
            }
        })
        .collect()
}

/// True when a family should render open. An explicit operator choice wins;
/// otherwise the active component's family is disclosed automatically.
pub fn family_is_disclosed(
    family: CatalogueFamilyId,
    active_slug: Option<&str>,
    user_disclosure: &std::collections::HashMap<String, bool>,
) -> bool {
    if let Some(disclosed) = user_disclosure.get(family.id()) {
        return *disclosed;
    }
    active_slug
        .and_then(find_component)
        .is_some_and(|component| component.family == family)
}
