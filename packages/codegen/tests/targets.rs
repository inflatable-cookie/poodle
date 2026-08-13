//! Integration tests for the targets this card adds — JSON surface, JSON
//! Schema, registry, conformance vectors, docs fragments.
//!
//! The shared-machinery tests (byte-identical double generation via CLI,
//! environment independence, whitespace classification, check-mode
//! read-only-ness, malformed/invalid input) live in `emission.rs` and cover
//! every registered target automatically — `targets::all()` drives the CLI
//! and `ir:check`. These tests are per-target: each registered target must
//! render the fixture without error, double-generate byte-identical, detect
//! drift and stale orphans in its own output root, and satisfy its content
//! contract. The milestone acceptance ("one fixture change updates every
//! declared artifact in one `ir:build`") and the schema round-trip test are
//! here too.

use std::fs;
use std::path::{Path, PathBuf};

use poodle_codegen::{
    check_outputs, generate, load_and_validate, targets, write_outputs, DriftKind, GeneratedFile,
};

/// Repo-relative fixture path, exactly as the Effigy selector passes it.
const FIXTURE: &str = "packages/codegen/fixtures/synthetic-model.json";

fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn repo_root() -> PathBuf {
    crate_dir()
        .parent()
        .expect("packages/codegen has a parent")
        .parent()
        .expect("packages has a parent")
        .to_path_buf()
}

fn fixture_path() -> PathBuf {
    repo_root().join(FIXTURE)
}

/// A scratch directory for one test, under the target dir cargo cleans.
fn scratch(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("scratch dir creates");
    dir
}

/// The committed-artifact root for one target under a scratch `--out`.
fn target_root(name: &str, target_id: &str) -> PathBuf {
    scratch(name).join(target_id)
}

fn render_target(target_id: &str) -> Vec<GeneratedFile> {
    let model = load_and_validate(&fixture_path()).expect("fixture loads and validates");
    let target = targets::by_id(target_id).expect("target registered");
    generate(&model, FIXTURE, target).expect("fixture renders")
}

fn write_target(target_id: &str, root: &Path) -> Vec<GeneratedFile> {
    let files = render_target(target_id);
    write_outputs(root, &files).expect("write mode succeeds");
    files
}

/// Recursively maps relative path → bytes under `root`.
fn snapshot(root: &Path) -> Vec<(String, Vec<u8>)> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir).expect("snapshot reads dir") {
            let entry = entry.expect("snapshot entry");
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                let relative = path
                    .strip_prefix(root)
                    .expect("under root")
                    .to_str()
                    .expect("utf8")
                    .to_owned();
                out.push((relative, fs::read(&path).expect("snapshot reads file")));
            }
        }
    }
    out.sort();
    out
}

/// Every registered target id, in registry order.
fn registered_target_ids() -> Vec<&'static str> {
    targets::all().iter().map(|target| target.id()).collect()
}

// ---------------------------------------------------------------------------
// Per-target rendering and determinism
// ---------------------------------------------------------------------------

#[test]
fn every_target_renders_the_fixture_without_error() {
    for target_id in registered_target_ids() {
        let files = render_target(target_id);
        assert!(
            !files.is_empty(),
            "target '{target_id}' emitted nothing for the fixture"
        );
        for file in &files {
            assert!(
                !file.contents.is_empty(),
                "target '{target_id}' emitted an empty file {}",
                file.path
            );
        }
    }
}

#[test]
fn every_target_double_generation_is_byte_identical() {
    for target_id in registered_target_ids() {
        let first = render_target(target_id);
        let second = render_target(target_id);
        assert_eq!(
            first.len(),
            second.len(),
            "target '{target_id}' changed its file count between renders"
        );
        for (a, b) in first.iter().zip(&second) {
            assert_eq!(
                a, b,
                "target '{target_id}' is not deterministic: {} drifted between renders",
                a.path
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Per-target drift gate behavior (drift, orphans, read-only check)
// ---------------------------------------------------------------------------

#[test]
fn every_target_detects_content_drift_and_stale_orphans() {
    for target_id in registered_target_ids() {
        let root = target_root("drift", target_id);
        let files = write_target(target_id, &root);

        // Corrupt the first generated file with real content change.
        let drifted_path = files.first().expect("at least one file").path.clone();
        let drifted_file = root.join(&drifted_path);
        let mut contents = fs::read_to_string(&drifted_file).expect("generated file exists");
        contents.push_str("\n// planted drift");
        assert_ne!(
            contents,
            fs::read_to_string(&drifted_file).expect("generated file"),
            "the drift edit is real"
        );
        fs::write(&drifted_file, contents).expect("plant content drift");
        fs::write(root.join("orphan.json"), "{}\n").expect("plant stale orphan");

        let report = check_outputs(&root, &files).expect("check runs");
        assert!(
            !report.is_clean(),
            "target '{target_id}' did not detect the planted drift"
        );
        assert!(
            report
                .drifted
                .iter()
                .any(|(path, kind)| path.ends_with(&drifted_path) && *kind == DriftKind::Content),
            "target '{target_id}' did not report {} as content drift",
            drifted_path
        );
        assert!(
            report
                .stale
                .iter()
                .any(|path| path.ends_with("orphan.json")),
            "target '{target_id}' did not report the stale orphan: {:?}",
            report.stale
        );
    }
}

#[test]
fn every_target_write_mode_deletes_stale_orphans() {
    for target_id in registered_target_ids() {
        let root = target_root("write-orphan", target_id);
        let files = write_target(target_id, &root);
        fs::write(root.join("orphan.json"), "{}\n").expect("plant orphan");

        write_outputs(&root, &files).expect("write mode succeeds");
        assert!(
            !root.join("orphan.json").exists(),
            "target '{target_id}' write mode deletes stale orphans"
        );
    }
}

#[test]
fn every_target_check_leaves_the_tree_unchanged() {
    for target_id in registered_target_ids() {
        let root = target_root("readonly", target_id);
        let files = write_target(target_id, &root);
        fs::write(root.join("orphan.json"), "{}\n").expect("plant orphan");
        let drifted = snapshot(&root);

        let report = check_outputs(&root, &files).expect("check runs");
        assert!(!report.is_clean());
        assert_eq!(
            snapshot(&root),
            drifted,
            "target '{target_id}' check mode never mutates the tree"
        );
    }
}

// ---------------------------------------------------------------------------
// JSON surface content contract
// ---------------------------------------------------------------------------

#[test]
fn json_surface_index_lists_every_component_document_sorted() {
    let files = render_target("json");
    let index = files
        .iter()
        .find(|file| file.path == "index.json")
        .expect("json target emits index.json");
    let doc: serde_json::Value = serde_json::from_str(&index.contents).expect("index is JSON");
    let components = doc["components"].as_array().expect("index has components");
    let ids: Vec<&str> = components
        .iter()
        .map(|entry| entry["id"].as_str().expect("component id"))
        .collect();
    assert_eq!(
        ids,
        vec!["badge", "gauge", "search-field"],
        "index is id-sorted"
    );
    for (id, entry) in components.iter().enumerate() {
        assert_eq!(
            entry["document"].as_str(),
            Some(format!("{}.json", ids[id]).as_str()),
            "index names the per-component document path"
        );
    }
}

#[test]
fn json_surface_document_carries_prop_subsets_events_and_axes() {
    let files = render_target("json");

    let badge = files
        .iter()
        .find(|file| file.path == "badge.json")
        .expect("badge document");
    let badge: serde_json::Value = serde_json::from_str(&badge.contents).expect("badge is JSON");
    let tone = badge["props"]
        .as_array()
        .expect("props")
        .iter()
        .find(|prop| prop["id"] == "tone")
        .expect("tone prop");
    assert_eq!(tone["type"]["kind"], "shared");
    assert_eq!(tone["type"]["shared_type"], "tone");
    assert_eq!(
        tone["permitted_subset"]
            .as_array()
            .expect("subset")
            .iter()
            .map(|member| member.as_str().expect("member"))
            .collect::<Vec<_>>(),
        vec!["danger", "default", "success"],
        "the R6.2 permitted subset survives into the document"
    );
    assert_eq!(tone["default"]["kind"], "member");
    assert_eq!(tone["default"]["value"], "default");

    let gauge = files
        .iter()
        .find(|file| file.path == "gauge.json")
        .expect("gauge document");
    let gauge: serde_json::Value = serde_json::from_str(&gauge.contents).expect("gauge is JSON");
    let events = gauge["events"].as_array().expect("events");
    assert_eq!(events.len(), 2);
    assert_eq!(events[0]["id"], "value-change");
    assert_eq!(events[0]["kind"], "value-change");
    assert_eq!(events[0]["payload"]["kind"], "pair");
    assert_eq!(events[0]["timing"]["phase"], "during-interaction");
    assert_eq!(
        events[0]["timing"]["ordering"][0]["after"], "value-commit",
        "the ordering constraint survives"
    );

    let axes = &gauge["axes"];
    assert_eq!(axes["size"]["explicit"], "sm");
    assert_eq!(
        axes["size"]["ladder"][0]["metrics"]["min-height"]["rem"],
        1.5
    );
    assert_eq!(axes["density"]["explicit"], "default");
    assert_eq!(axes["orientation"]["default"], "horizontal");
    assert_eq!(
        axes["orientation"]["values"]
            .as_array()
            .expect("values")
            .iter()
            .map(|value| value.as_str().expect("value"))
            .collect::<Vec<_>>(),
        vec!["horizontal", "vertical"]
    );

    let shared = gauge["shared_types"]
        .as_array()
        .expect("shared_types")
        .iter()
        .find(|entry| entry["id"] == "orientation")
        .expect("orientation shared ref");
    assert_eq!(shared["referenced_by"][0], "orientation");
}

#[test]
fn json_surface_documents_carry_the_ir07_generated_object() {
    for file in render_target("json") {
        let doc: serde_json::Value = serde_json::from_str(&file.contents).expect("is JSON");
        assert_eq!(
            doc["generated"]["source"], FIXTURE,
            "{} carries the authored source path",
            file.path
        );
        assert_eq!(doc["generated"]["ir_schema_version"], 1);
        assert!(
            doc["generated"]["generator"]
                .as_str()
                .expect("generator")
                .starts_with("poodle-codegen "),
            "{} names the generator and version",
            file.path
        );
    }
}

// ---------------------------------------------------------------------------
// Registry content contract
// ---------------------------------------------------------------------------

#[test]
fn registry_lists_every_component_with_capabilities_axes_and_shared_types() {
    let files = render_target("registry");
    let registry = files
        .iter()
        .find(|file| file.path == "registry.json")
        .expect("registry target emits registry.json");
    let doc: serde_json::Value =
        serde_json::from_str(&registry.contents).expect("registry is JSON");
    let components = doc["components"].as_array().expect("components");

    let ids: Vec<&str> = components
        .iter()
        .map(|entry| entry["id"].as_str().expect("component id"))
        .collect();
    assert_eq!(
        ids,
        vec!["badge", "gauge", "search-field"],
        "registry is id-sorted"
    );

    let gauge = components
        .iter()
        .find(|entry| entry["id"] == "gauge")
        .expect("gauge entry");
    assert_eq!(
        gauge["capabilities"]
            .as_array()
            .expect("capabilities")
            .iter()
            .map(|capability| capability.as_str().expect("capability"))
            .collect::<Vec<_>>(),
        vec!["pointer-capture", "scrub-fraction"],
        "capabilities sort by the inventory order, not declaration order"
    );
    assert_eq!(
        gauge["axes"]
            .as_array()
            .expect("axes")
            .iter()
            .map(|axis| axis.as_str().expect("axis"))
            .collect::<Vec<_>>(),
        vec!["size", "density", "orientation"],
        "axes list the declared axes in fixed struct order"
    );
    assert_eq!(
        gauge["shared_types"][0].as_str(),
        Some("orientation"),
        "shared-type references resolve to ids"
    );

    let badge = components
        .iter()
        .find(|entry| entry["id"] == "badge")
        .expect("badge entry");
    assert_eq!(badge["shared_types"][0].as_str(), Some("tone"));
    assert_eq!(
        badge["capabilities"]
            .as_array()
            .expect("capabilities")
            .len(),
        0
    );

    let search = components
        .iter()
        .find(|entry| entry["id"] == "search-field")
        .expect("search-field entry");
    assert_eq!(
        search["shared_types"]
            .as_array()
            .expect("shared types")
            .iter()
            .map(|entry| entry.as_str().expect("shared type id"))
            .collect::<Vec<_>>(),
        vec!["tone", "validation-state"],
        "shared types sort by id"
    );
}

// ---------------------------------------------------------------------------
// Conformance vector content contract
// ---------------------------------------------------------------------------

#[test]
fn conformance_vectors_carry_steps_kinds_and_declared_by() {
    let files = render_target("conformance");
    let vectors = files
        .iter()
        .find(|file| file.path == "vectors.json")
        .expect("conformance target emits vectors.json");
    let doc: serde_json::Value = serde_json::from_str(&vectors.contents).expect("vectors is JSON");
    let vectors = doc["vectors"].as_array().expect("vectors");

    let vector = vectors
        .iter()
        .find(|vector| vector["id"] == "gauge-bounds")
        .expect("gauge-bounds vector");
    assert_eq!(
        vector["applies_to"]
            .as_array()
            .expect("applies_to")
            .iter()
            .map(|target| target.as_str().expect("runtime target"))
            .collect::<Vec<_>>(),
        vec!["svelte", "react", "gpui", "jetstream"]
    );
    assert_eq!(
        vector["declared_by"][0].as_str(),
        Some("gauge"),
        "declared_by lists the component whose conformance names the vector"
    );

    let steps = vector["steps"].as_array().expect("steps");
    assert_eq!(steps.len(), 3, "the three fixture steps survive in order");
    assert_eq!(steps[0]["kind"], "invariant");
    assert_eq!(steps[1]["kind"], "transition");
    assert_eq!(steps[2]["kind"], "effect-intent");
    assert!(
        steps
            .iter()
            .all(|step| step.get("guard").is_none()),
        "guard expressions are gone (g13.017)"
    );
}

// ---------------------------------------------------------------------------
// Docs-fragment content contract
// ---------------------------------------------------------------------------

#[test]
fn docs_fragments_render_contract_style_props_tables() {
    let files = render_target("docs");
    let badge = files
        .iter()
        .find(|file| file.path == "badge.md")
        .expect("badge fragment");
    assert!(
        badge.contents.contains("### Public Props"),
        "fragment opens the contract section"
    );
    assert!(
        badge.contents.contains(
            "| `tone` | `\"danger\" \\| \"default\" \\| \"success\"` | `\"default\"` | no |"
        ),
        "the R6.2 subset union survives into the fragment: {}",
        badge.contents
    );
    assert!(
        badge
            .contents
            .contains("| `label` | `string` | `—` | yes |"),
        "required props render yes, no default renders an em dash"
    );
    assert!(
        badge
            .contents
            .contains("| `maxWidth` | `number` | `120.5` | no |"),
        "numbers use the fixed formatter, never locale output"
    );

    let gauge = files
        .iter()
        .find(|file| file.path == "gauge.md")
        .expect("gauge fragment");
    assert!(
        gauge
            .contents
            .contains("| `orientation` | `Orientation` | `\"horizontal\"` | no |"),
        "an unreserved shared prop references the shared type by name"
    );
    assert!(
        gauge
            .contents
            .contains("| `value` | `[number, number]` | `[0, 100]` | no |"),
        "pairs render inline"
    );

    let search = files
        .iter()
        .find(|file| file.path == "search-field.md")
        .expect("search-field fragment");
    assert!(
        search.contents.contains("web-only (CROSS-03)"),
        "web-only props are included and marked (the TypeScript target's include-and-mark decision)"
    );
    assert!(
        search
            .contents
            .contains("| `hint` | `string` | `—` | no |"),
        "a prop without a default renders the em dash; expression defaults are gone (g13.017)"
    );
    assert!(
        search
            .contents
            .contains("| `accent` | `\"danger\"` | `\"danger\"` | no |"),
        "a one-member subset renders as a single-member union"
    );
}

// ---------------------------------------------------------------------------
// Milestone acceptance (spec 063 "One Rust definition change must update
// every expected target in one build"; g13.003 acceptance "one fixture
// change updates every declared artifact in one command")
// ---------------------------------------------------------------------------

/// One added component definition — a single fixture change. It exercises
/// every surface the targets emit: a shared-type prop with a permitted
/// subset, an event, an axis-free layout (the registry's axes list must
/// still reflect it), a capability, and a conformance-vector reference.
fn added_status_light() -> serde_json::Value {
    serde_json::json!({
        "id": "status-light",
        "name": "StatusLight",
        "layer": "foundation",
        "contract": { "path": "docs/contracts/components/status-light.md", "section": "§3" },
        "description": "Synthetic status light exercising the full emit surface added by g13-025.",
        "props": [
            {
                "id": "tone",
                "name": "tone",
                "prop_type": { "Shared": "tone" },
                "default": { "Member": "success" },
                "required": false,
                "web_only": false,
                "description": "Semantic status tone.",
                "permitted_subset": { "shared_type": "tone", "members": ["success", "warning"] }
            },
            {
                "id": "label",
                "name": "label",
                "prop_type": "String",
                "default": null,
                "required": true,
                "web_only": false,
                "description": "Accessible label for the light.",
                "permitted_subset": null
            }
        ],
        "controlled_state": [],
        "events": [
            {
                "id": "status-change",
                "name": "onStatusChange",
                "kind": "value-change",
                "payload": { "name": "status", "kind": "string" },
                "timing": { "phase": "during-interaction", "debounce_ms": null, "flush_on_blur": false, "ordering": [] },
                "description": "Reports the status string as it changes."
            }
        ],
        "parts": [],
        "attributes": [],
        "axes": { "size": null, "density": null, "orientation": null },
        "tokens": [],
        "recipe_hooks": [],
        "accessibility": {
            "role": "group",
            "name_rule": "from-content",
            "name_source": null,
            "aria": [],
            "native": [],
            "description": "Group role; the label names it."
        },
        "capabilities": [
            { "capability": "announcements", "purpose": "Live-region announcements for status changes (CROSS-17)." }
        ],
        "keyboard": [],
        "visual_state": [],
        "conformance": ["gauge-bounds"],
        "extensions": []
    })
}

/// The fixture with exactly one change: `status-light` appended to
/// `components`. Validates clean against the IR.
fn fixture_with_added_component() -> poodle_ir::IrModel {
    let source = fs::read_to_string(fixture_path()).expect("fixture reads");
    let mut doc: serde_json::Value = serde_json::from_str(&source).expect("fixture is JSON");
    doc["components"]
        .as_array_mut()
        .expect("components is an array")
        .push(added_status_light());
    let model: poodle_ir::IrModel =
        serde_json::from_value(doc).expect("changed model deserializes");
    let findings = model.validate();
    assert!(
        findings.is_empty(),
        "the one added component validates: {:?}",
        findings
    );
    model
}

#[test]
fn one_fixture_change_updates_every_declared_artifact() {
    let base_model = load_and_validate(&fixture_path()).expect("fixture loads and validates");
    let baseline: Vec<(&str, Vec<GeneratedFile>)> = targets::all()
        .iter()
        .map(|target| {
            (
                target.id(),
                generate(&base_model, FIXTURE, *target).expect("baseline renders"),
            )
        })
        .collect();

    let changed = fixture_with_added_component();

    // One `ir:build`-equivalent pass: every registered target rendered from
    // the one changed model.
    for (target_id, before) in &baseline {
        let target = targets::by_id(target_id).expect("registered target");
        let after = generate(&changed, FIXTURE, target).expect("changed model renders");
        assert_ne!(
            after, *before,
            "the one fixture change did not update target '{target_id}' — \
             every declared artifact must move with the model"
        );
    }

    // The update is real, not incidental: the new component lands in the
    // per-component artifacts and its references land in the cross-references.
    let json_files =
        generate(&changed, FIXTURE, targets::by_id("json").expect("json")).expect("json renders");
    assert!(
        json_files
            .iter()
            .any(|file| file.path == "status-light.json"),
        "the JSON surface gains the new component document"
    );
    let conformance = generate(
        &changed,
        FIXTURE,
        targets::by_id("conformance").expect("conformance"),
    )
    .expect("conformance renders");
    let vectors: serde_json::Value = serde_json::from_str(
        &conformance
            .iter()
            .find(|file| file.path == "vectors.json")
            .expect("vectors.json")
            .contents,
    )
    .expect("vectors is JSON");
    assert!(
        vectors["vectors"][0]["declared_by"]
            .as_array()
            .expect("declared_by")
            .iter()
            .any(|id| id == "status-light"),
        "the new component's conformance reference lands in the vector's declared_by"
    );
}

// ---------------------------------------------------------------------------
// JSON Schema round trip (acceptance "emitted JSON validates against the
// emitted JSON Schema, proven by test")
// ---------------------------------------------------------------------------

/// The emitted schema, compiled. `jsonschema` is a test-only oracle (not a
/// type-mirroring emitter — ruling R4); it renders nothing.
fn emitted_schema_validator() -> jsonschema::Validator {
    let schema_files = render_target("schema");
    let schema_file = schema_files
        .iter()
        .find(|file| file.path == "schema.json")
        .expect("schema target emits schema.json");
    let schema: serde_json::Value =
        serde_json::from_str(&schema_file.contents).expect("schema is JSON");
    jsonschema::options()
        .with_draft(jsonschema::Draft::Draft202012)
        .build(&schema)
        .expect("the emitted schema compiles")
}

#[test]
fn every_emitted_json_document_validates_against_the_emitted_schema() {
    let validator = emitted_schema_validator();
    for file in render_target("json") {
        let doc: serde_json::Value =
            serde_json::from_str(&file.contents).expect("document is JSON");
        let result = validator.validate(&doc);
        assert!(
            result.is_ok(),
            "{} does not validate against the emitted schema: {}",
            file.path,
            result.expect_err("err on failure")
        );
    }
}

#[test]
fn emitted_schema_rejects_tampered_documents() {
    let validator = emitted_schema_validator();
    let files = render_target("json");

    // Unknown component id — the model-derived enum has teeth.
    let badge = files
        .iter()
        .find(|file| file.path == "badge.json")
        .expect("badge document");
    let mut badge_doc: serde_json::Value =
        serde_json::from_str(&badge.contents).expect("badge is JSON");
    badge_doc["component"]["id"] = serde_json::json!("not-a-component");
    assert!(
        validator.validate(&badge_doc).is_err(),
        "an unknown component id must fail the schema"
    );

    // Unknown prop-type kind — the closed tag vocabulary has teeth.
    let gauge = files
        .iter()
        .find(|file| file.path == "gauge.json")
        .expect("gauge document");
    let mut gauge_doc: serde_json::Value =
        serde_json::from_str(&gauge.contents).expect("gauge is JSON");
    gauge_doc["props"][0]["type"]["kind"] = serde_json::json!("decimal");
    assert!(
        validator.validate(&gauge_doc).is_err(),
        "an unknown prop-type kind must fail the schema"
    );

    // A missing required field — required has teeth.
    let index = files
        .iter()
        .find(|file| file.path == "index.json")
        .expect("index document");
    let mut index_doc: serde_json::Value =
        serde_json::from_str(&index.contents).expect("index is JSON");
    index_doc
        .as_object_mut()
        .expect("index is an object")
        .remove("components");
    assert!(
        validator.validate(&index_doc).is_err(),
        "a document missing 'components' must fail the schema"
    );
}
