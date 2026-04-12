use std::collections::HashMap;
use std::sync::OnceLock;

use serde_json::Value;

pub struct ComponentEvidence {
    pub status: String,
    pub note: String,
    pub section_ids: Vec<String>,
    pub section_titles: Vec<String>,
}

struct EvidenceDb {
    by_export_name: HashMap<String, ComponentEvidence>,
}

static EVIDENCE_DB: OnceLock<Option<EvidenceDb>> = OnceLock::new();

pub fn component_evidence(export_name: &str) -> Option<&'static ComponentEvidence> {
    EVIDENCE_DB
        .get_or_init(load_evidence_db)
        .as_ref()
        .and_then(|db| db.by_export_name.get(export_name))
}

fn load_evidence_db() -> Option<EvidenceDb> {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../svelte/preview/artifacts/parity-report.json"
    ));
    let value: Value = serde_json::from_str(raw).ok()?;

    let mut targets_by_id = HashMap::new();
    for target in value.get("targets")?.as_array()? {
        let Some(section_id) = target.get("sectionId").and_then(Value::as_str) else {
            continue;
        };
        let Some(title) = target.get("title").and_then(Value::as_str) else {
            continue;
        };
        targets_by_id.insert(section_id.to_string(), title.to_string());
    }

    let mut by_export_name = HashMap::new();
    for entry in value.get("packageSurfaceCoverage")?.as_array()? {
        let package_name = entry.get("packageName").and_then(Value::as_str)?;
        if package_name != "@poodle/svelte" {
            continue;
        }

        let export_name = entry.get("exportName").and_then(Value::as_str)?.to_string();
        let status = entry.get("status").and_then(Value::as_str)?.to_string();
        let note = entry
            .get("note")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();

        let section_ids: Vec<String> = entry
            .get("sectionIds")
            .and_then(Value::as_array)
            .map(|items| {
                items
                    .iter()
                    .filter_map(Value::as_str)
                    .map(ToString::to_string)
                    .collect()
            })
            .unwrap_or_default();

        let section_titles = section_ids
            .iter()
            .filter_map(|section_id| targets_by_id.get(section_id).cloned())
            .collect();

        by_export_name.insert(
            export_name,
            ComponentEvidence {
                status,
                note,
                section_ids,
                section_titles,
            },
        );
    }

    Some(EvidenceDb { by_export_name })
}
