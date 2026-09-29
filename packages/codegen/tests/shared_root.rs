//! Shared output-root orphan sweep: two targets in one directory keep each
//! other's artifacts, and a real orphan is still removed.

use std::fs;
use std::path::PathBuf;

use poodle_codegen::{
    check_outputs, check_outputs_protecting, write_outputs, write_outputs_protecting, GeneratedFile,
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
fn shared_root_write_keeps_sibling_artifacts_and_deletes_orphans() {
    let root = scratch("shared-write");
    write_outputs_protecting(&root, &ts_files(), &["rs"]).expect("ts write");
    write_outputs_protecting(&root, &rs_files(), &["ts"]).expect("rs write");
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

    write_outputs_protecting(&root, &ts_files(), &["rs"]).expect("ts rewrite");
    assert!(root.join("owned.ts").exists());
    assert!(root.join("owned.rs").exists(), "sibling rs must survive");
    assert!(
        !root.join("orphan.json").exists(),
        "unclaimed garbage is still an orphan"
    );
    assert!(
        !root.join("stale.ts").exists(),
        "a file this target used to emit is still removed"
    );
}

#[test]
fn shared_root_check_reports_own_orphans_not_siblings() {
    let root = scratch("shared-check");
    write_outputs_protecting(&root, &ts_files(), &["rs"]).expect("ts write");
    write_outputs_protecting(&root, &rs_files(), &["ts"]).expect("rs write");
    fs::write(root.join("orphan.json"), "{}\n").expect("plant unclaimed orphan");
    fs::write(root.join("stale.ts"), "export const stale = true;\n").expect("plant owned orphan");

    let report = check_outputs_protecting(&root, &ts_files(), &["rs"]).expect("check");
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
        !report.stale.iter().any(|path| path.ends_with("owned.rs")),
        "sibling artifact is not stale: {:?}",
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
