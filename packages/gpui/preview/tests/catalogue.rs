//! Headless catalogue navigation invariants for the GPUI overlay.

#[path = "../src/component_registry.rs"]
mod component_registry;

use component_registry::{
    family_is_disclosed, find_component, grouped_sections, search_components, CatalogueFamilyId,
    CANONICAL_COMPONENTS,
};

#[test]
fn unknown_slug_has_no_workstation_fallback() {
    assert!(find_component("form-shell").is_none());
    assert!(find_component("tab-strip").is_none());
    assert!(find_component("inline-remediation").is_none());
    assert!(find_component("button").is_some());
}

#[test]
fn every_canonical_entry_has_a_family_and_section() {
    assert_eq!(CANONICAL_COMPONENTS.len(), 174);
    for component in CANONICAL_COMPONENTS {
        assert_eq!(component.section, component.family.section());
    }
}

#[test]
fn motivating_suites_share_one_family() {
    for slug in [
        "agent-plan",
        "tool-call",
        "changed-files",
        "agent-chat-input",
    ] {
        assert_eq!(find_component(slug).unwrap().family, CatalogueFamilyId::AgentTools);
    }
    for slug in [
        "model-picker",
        "model-catalogue-editor",
        "model-connection-setup",
    ] {
        assert_eq!(
            find_component(slug).unwrap().family,
            CatalogueFamilyId::ModelConnections
        );
    }
    for slug in ["knob", "audio-meter", "waveform-display"] {
        assert_eq!(find_component(slug).unwrap().family, CatalogueFamilyId::AudioMusic);
    }
    for slug in ["licence-status", "update-center"] {
        assert_eq!(
            find_component(slug).unwrap().family,
            CatalogueFamilyId::AccountLifecycle
        );
    }
    for slug in ["app-header", "message-center", "history-center", "settings-shell"] {
        assert_eq!(
            find_component(slug).unwrap().family,
            CatalogueFamilyId::ApplicationShell
        );
    }
}

#[test]
fn landing_starts_with_no_family_forced_open() {
    let expanded = std::collections::HashSet::new();
    assert!(!family_is_disclosed(
        CatalogueFamilyId::ActionsSelection,
        None,
        &expanded
    ));
    assert!(family_is_disclosed(
        CatalogueFamilyId::AgentTools,
        Some("agent-plan"),
        &expanded
    ));
}

#[test]
fn search_matches_family_and_kind_labels() {
    let hits = search_components("agent");
    assert!(hits.iter().any(|component| component.slug == "agent-plan"));
    let crumbs = search_components("model connections");
    assert!(crumbs
        .iter()
        .any(|component| component.slug == "model-connection-picker"));
}

#[test]
fn grouped_sections_preserve_counts_and_order() {
    let sections = grouped_sections("");
    assert_eq!(sections.len(), 3);
    assert_eq!(sections[0].label, "Foundations");
    assert_eq!(sections[2].label, "Systems");
    let total: usize = sections
        .iter()
        .flat_map(|section| section.families.iter())
        .map(|family| family.items.len())
        .sum();
    assert_eq!(total, 174);
    let agent = sections[2]
        .families
        .iter()
        .find(|family| family.family == CatalogueFamilyId::AgentTools)
        .expect("agent family");
    assert_eq!(agent.items.len(), 11);
}

#[test]
fn href_identity_is_the_canonical_slug() {
    for component in CANONICAL_COMPONENTS {
        assert!(!component.slug.is_empty());
        assert_eq!(find_component(component.slug).unwrap().slug, component.slug);
    }
}
