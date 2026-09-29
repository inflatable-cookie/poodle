//! Shared output-root orphan sweep: exclusive production roots delete every
//! unclaimed file; exact-path protection keeps a sibling's owned artifacts
//! without hiding a stray file of the same extension.

use std::fs;
use std::path::PathBuf;

use poodle_codegen::{
    check_outputs, check_outputs_protecting, colliding_output_roots, targets, write_outputs,
    write_outputs_protecting, GeneratedFile,
};

fn scratch(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("scratch dir creates");
    dir
}

fn ts_files() -> Vec<GeneratedFile> {
    vec![GeneratedFile::new(
        "owned.ts",
        "export const owned = true;\n",
    )]
}

fn rs_files() -> Vec<GeneratedFile> {
    vec![GeneratedFile::new(
        "owned.rs",
        "pub const OWNED: bool = true;\n",
    )]
}

#[test]
fn exclusive_write_deletes_a_stale_rs_file_in_a_typescript_root() {
    let root = scratch("exclusive-stale-rs");
    write_outputs(&root, &ts_files()).expect("ts write");
    fs::write(root.join("stale.rs"), "pub const STALE: bool = true;\n").expect("plant stale.rs");
    write_outputs(&root, &ts_files()).expect("ts rewrite");
    assert!(root.join("owned.ts").exists());
    assert!(
        !root.join("stale.rs").exists(),
        "exclusive TypeScript sweep must delete a stray .rs orphan"
    );
}

#[test]
fn exclusive_check_reports_a_stale_rs_file_in_a_typescript_root() {
    let root = scratch("exclusive-check-stale-rs");
    write_outputs(&root, &ts_files()).expect("ts write");
    fs::write(root.join("stale.rs"), "pub const STALE: bool = true;\n").expect("plant stale.rs");
    let report = check_outputs(&root, &ts_files()).expect("check");
    assert!(
        report.stale.iter().any(|path| path.ends_with("stale.rs")),
        "exclusive TypeScript check must report stale.rs: {:?}",
        report.stale
    );
}

#[test]
fn exclusive_write_deletes_a_sibling_file_at_the_same_root() {
    let root = scratch("exclusive-clobber");
    write_outputs(&root, &ts_files()).expect("ts write");
    write_outputs(&root, &rs_files()).expect("rs write");
    assert!(
        !root.join("owned.ts").exists(),
        "exclusive sweep still treats a non-owned top-level file as an orphan"
    );
    assert!(root.join("owned.rs").exists());
}

#[test]
fn shared_root_write_keeps_exact_sibling_paths_and_deletes_orphans() {
    let root = scratch("shared-write");
    write_outputs_protecting(&root, &ts_files(), &["owned.rs"]).expect("ts write");
    write_outputs_protecting(&root, &rs_files(), &["owned.ts"]).expect("rs write");
    assert!(
        root.join("owned.ts").exists(),
        "ts artifact survives the rs sweep"
    );
    assert!(
        root.join("owned.rs").exists(),
        "rs artifact survives the ts sweep"
    );

    fs::write(root.join("orphan.json"), "{}\n").expect("plant unclaimed orphan");
    fs::write(root.join("stale.ts"), "export const stale = true;\n").expect("plant owned orphan");
    fs::write(root.join("stale.rs"), "pub const STALE: bool = true;\n")
        .expect("plant sibling-extension orphan");

    write_outputs_protecting(&root, &ts_files(), &["owned.rs"]).expect("ts rewrite");
    assert!(root.join("owned.ts").exists());
    assert!(
        root.join("owned.rs").exists(),
        "exact sibling path must survive"
    );
    assert!(
        !root.join("orphan.json").exists(),
        "unclaimed garbage is still an orphan"
    );
    assert!(
        !root.join("stale.ts").exists(),
        "a file this target used to emit is still removed"
    );
    assert!(
        !root.join("stale.rs").exists(),
        "a stray .rs that is not the sibling's exact path is still an orphan"
    );
}

#[test]
fn shared_root_check_reports_own_orphans_not_exact_siblings() {
    let root = scratch("shared-check");
    write_outputs_protecting(&root, &ts_files(), &["owned.rs"]).expect("ts write");
    write_outputs_protecting(&root, &rs_files(), &["owned.ts"]).expect("rs write");
    fs::write(root.join("orphan.json"), "{}\n").expect("plant unclaimed orphan");
    fs::write(root.join("stale.ts"), "export const stale = true;\n").expect("plant owned orphan");
    fs::write(root.join("stale.rs"), "pub const STALE: bool = true;\n")
        .expect("plant sibling-extension orphan");

    let report = check_outputs_protecting(&root, &ts_files(), &["owned.rs"]).expect("check");
    assert!(
        report
            .stale
            .iter()
            .any(|path| path.ends_with("orphan.json")),
        "unclaimed orphan is stale: {:?}",
        report.stale
    );
    assert!(
        report.stale.iter().any(|path| path.ends_with("stale.ts")),
        "owned orphan is stale: {:?}",
        report.stale
    );
    assert!(
        report.stale.iter().any(|path| path.ends_with("stale.rs")),
        "stray .rs that is not the sibling's exact path is stale: {:?}",
        report.stale
    );
    assert!(
        !report.stale.iter().any(|path| path.ends_with("owned.rs")),
        "exact sibling artifact is not stale: {:?}",
        report.stale
    );
    assert!(
        report.drifted.is_empty(),
        "expected files are clean: {:?}",
        report.drifted
    );

    let exclusive = check_outputs(&root, &ts_files()).expect("exclusive check");
    assert!(
        exclusive
            .stale
            .iter()
            .any(|path| path.ends_with("owned.rs")),
        "without protection the sibling looks stale: {:?}",
        exclusive.stale
    );
}

#[test]
fn default_registered_targets_have_exclusive_output_roots() {
    assert!(
        colliding_output_roots(targets::all()).is_empty(),
        "plain ir:build must keep exclusive per-target directories: {:?}",
        colliding_output_roots(targets::all())
    );
}

#[test]
fn shell_and_specimen_pairs_collide_on_their_shared_output_root_strings() {
    let shell = colliding_output_roots([
        targets::by_id("shell-scene").expect("shell-scene"),
        targets::by_id("shell-rust").expect("shell-rust"),
    ]);
    assert_eq!(
        shell,
        vec![("generated", vec!["shell-scene", "shell-rust"])],
        "selecting both shell targets in one run must refuse"
    );

    let specimens = colliding_output_roots([
        targets::by_id("specimen-ts").expect("specimen-ts"),
        targets::by_id("specimen-rust").expect("specimen-rust"),
    ]);
    assert_eq!(
        specimens,
        vec![("generated/specimens", vec!["specimen-ts", "specimen-rust"])],
        "selecting both specimen targets in one run must refuse"
    );
}
