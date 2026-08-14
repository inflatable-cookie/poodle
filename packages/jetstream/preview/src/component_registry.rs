//! Jetstream preview catalogue overlay.
//!
//! Canonical identity, descriptions, sections, families, and order come from
//! the generated catalogue. `has_specimen` stays Jetstream-local. Interactive
//! shell navigation (collapse, search breadcrumbs) is deferred.

use std::sync::OnceLock;

#[path = "generated/catalogue/catalogue.rs"]
mod generated;

pub use generated::{CanonicalComponent, CatalogueFamilyId, CatalogueKindId, CatalogueSectionId};

pub struct ComponentEntry {
    pub slug: &'static str,
    pub display_name: &'static str,
    pub description: &'static str,
    pub family: CatalogueFamilyId,
    pub has_specimen: bool,
}

impl ComponentEntry {
    pub fn tag_label(&self) -> &'static str {
        generated::family_by_id(self.family).label
    }
}

static ALL: OnceLock<Vec<ComponentEntry>> = OnceLock::new();

pub fn all_components() -> &'static [ComponentEntry] {
    ALL.get_or_init(|| {
        generated::CANONICAL_COMPONENTS
            .iter()
            .map(|component| ComponentEntry {
                slug: component.slug,
                display_name: component.display_name,
                description: component.description,
                family: component.family,
                has_specimen: jetstream_has_specimen(component.slug),
            })
            .collect()
    })
}

pub fn specimen_count() -> usize {
    all_components().iter().filter(|entry| entry.has_specimen).count()
}

fn jetstream_has_specimen(slug: &str) -> bool {
    !matches!(
        slug,
        "radio"
            | "time-input"
            | "status-bar"
            | "agent-message"
            | "agent-plan"
            | "agent-plan-record"
            | "agent-question-record"
            | "agent-subagent"
            | "changed-files"
            | "history-center"
            | "licence-activation"
            | "licence-seats"
            | "licence-status"
            | "model-catalogue-editor"
            | "model-connection-card"
            | "model-connection-picker"
            | "model-connection-setup"
            | "remediation-banner"
            | "settings-shell"
            | "state-tile"
            | "theme-select"
            | "tool-call"
            | "tool-call-group"
            | "update-center"
            | "update-status"
            | "validation-summary"
    )
}
