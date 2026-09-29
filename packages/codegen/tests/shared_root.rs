//! Shared output-root orphan sweep: exclusive production roots delete every
//! unclaimed file; exact-path protection keeps a sibling's owned artifacts
//! without hiding a stray file of the same extension.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

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
        "selecting both shell targets shares the generated/ root string"
    );

    let specimens = colliding_output_roots([
        targets::by_id("specimen-ts").expect("specimen-ts"),
        targets::by_id("specimen-rust").expect("specimen-rust"),
    ]);
    assert_eq!(
        specimens,
        vec![("generated/specimens", vec!["specimen-ts", "specimen-rust"])],
        "selecting both specimen targets shares the generated/specimens root string"
    );
}

const SHELL_FIXTURE: &str = "packages/codegen/fixtures/shell-model.json";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn run_cli(out: &std::path::Path, target: &str, check: bool) -> std::process::Output {
    let bin = env!("CARGO_BIN_EXE_poodle-codegen");
    let mut command = Command::new(bin);
    command
        .args([SHELL_FIXTURE, "--out"])
        .arg(out)
        .args(["--target", target])
        .current_dir(repo_root());
    if check {
        command.arg("--check");
    }
    command.output().expect("bin runs")
}

#[test]
fn cli_sequential_shell_targets_keep_siblings_and_remove_orphans() {
    let out = scratch("cli-shell-shared");
    let generated = out.join("generated");

    let first = run_cli(&out, "shell-scene", false);
    assert!(
        first.status.success(),
        "shell-scene write failed: {}",
        String::from_utf8_lossy(&first.stderr)
    );
    assert!(generated.join("preview-shell.ts").exists());

    let second = run_cli(&out, "shell-rust", false);
    assert!(
        second.status.success(),
        "shell-rust write failed: {}",
        String::from_utf8_lossy(&second.stderr)
    );
    assert!(
        generated.join("preview-shell.ts").exists(),
        "shell-scene artifact must survive a later shell-rust write to the same --out"
    );
    assert!(generated.join("preview-shell.rs").exists());

    let check_ts = run_cli(&out, "shell-scene", true);
    assert!(
        check_ts.status.success(),
        "shell-scene check must accept the sibling rust file: {}",
        String::from_utf8_lossy(&check_ts.stderr)
    );
    let check_rs = run_cli(&out, "shell-rust", true);
    assert!(
        check_rs.status.success(),
        "shell-rust check must accept the sibling ts file: {}",
        String::from_utf8_lossy(&check_rs.stderr)
    );

    fs::write(generated.join("orphan.json"), "{}\n").expect("plant unclaimed orphan");
    fs::write(
        generated.join("stale.rs"),
        "pub const STALE: bool = true;\n",
    )
    .expect("plant same-extension orphan");

    let rewrite = run_cli(&out, "shell-scene", false);
    assert!(
        rewrite.status.success(),
        "shell-scene rewrite failed: {}",
        String::from_utf8_lossy(&rewrite.stderr)
    );
    assert!(generated.join("preview-shell.ts").exists());
    assert!(generated.join("preview-shell.rs").exists());
    assert!(
        !generated.join("orphan.json").exists(),
        "unclaimed garbage is still an orphan"
    );
    assert!(
        !generated.join("stale.rs").exists(),
        "a stray .rs that is not the sibling's exact path is still an orphan"
    );

    fs::write(
        generated.join("stale.rs"),
        "pub const STALE: bool = true;\n",
    )
    .expect("replant same-extension orphan");
    let before = fs::read_to_string(generated.join("preview-shell.rs")).expect("sibling bytes");
    let check_orphan = run_cli(&out, "shell-scene", true);
    assert!(
        !check_orphan.status.success(),
        "stale.rs must fail shell-scene check"
    );
    let stderr = String::from_utf8_lossy(&check_orphan.stderr);
    assert!(
        stderr.contains("stale.rs"),
        "check must name the real orphan: {stderr}"
    );
    assert!(
        !stderr.contains("preview-shell.rs"),
        "check must not treat the sibling as stale: {stderr}"
    );
    assert_eq!(
        fs::read_to_string(generated.join("preview-shell.rs")).expect("sibling unchanged"),
        before,
        "check mode never writes"
    );
    assert!(
        generated.join("stale.rs").exists(),
        "check mode must not delete the planted orphan"
    );
}
