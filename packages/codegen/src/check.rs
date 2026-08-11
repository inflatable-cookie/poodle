//! The read-only drift gate (`ir:check`, ruling R3).
//!
//! Structural property: this module contains no filesystem write call. It
//! reads committed files, compares byte-exact, classifies whitespace-only
//! differences, and scans the output root for stale orphans — then reports
//! everything at once. The write path lives in [`crate::write`] and is only
//! reachable from write mode; the two can never be confused.

use std::fs;
use std::path::{Path, PathBuf};

use crate::emit::GeneratedFile;

/// Why a committed file disagrees with the emitter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriftKind {
    /// Bytes differ beyond whitespace.
    Content,
    /// Bytes differ, but the difference is entirely whitespace (same
    /// non-whitespace character stream) — the `45caae82` failure class
    /// (b015 failure mode 3), classified so it is a one-line fix instead
    /// of an investigation.
    WhitespaceOnly,
    /// The emitter produces the file but nothing is committed at its path.
    Missing,
}

impl DriftKind {
    fn label(self) -> &'static str {
        match self {
            DriftKind::Content => "content drift",
            DriftKind::WhitespaceOnly => "whitespace-only difference",
            DriftKind::Missing => "missing",
        }
    }
}

/// The gate's verdict. `drifted` pairs each path with its classification;
/// `stale` lists committed files under the output root the emitter no
/// longer produces (b015 failure mode 5 — `build-tokens.ts`'s blind spot).
#[derive(Debug, Default, PartialEq, Eq)]
pub struct CheckReport {
    /// Drifted files, with their classification. Sorted for stable output.
    pub drifted: Vec<(PathBuf, DriftKind)>,
    /// Stale orphans: committed under the output root, not in the expected
    /// set. Sorted.
    pub stale: Vec<PathBuf>,
}

impl CheckReport {
    /// Whether the tree and the emitter agree.
    pub fn is_clean(&self) -> bool {
        self.drifted.is_empty() && self.stale.is_empty()
    }

    /// Renders the report exactly as the gate prints it: every finding at
    /// once, sorted, each classified. Empty when clean.
    pub fn message(&self) -> String {
        if self.is_clean() {
            return String::new();
        }
        let mut lines = Vec::new();
        for (path, kind) in &self.drifted {
            lines.push(format!("{} ({})", path.display(), kind.label()));
        }
        for path in &self.stale {
            lines.push(format!("{} (stale orphan)", path.display()));
        }
        lines.join("\n")
    }
}

/// Compares what would be written against what is committed.
///
/// - every generated file is compared byte-exact (a missing committed file
///   is `Missing` drift — b015 failure mode 6),
/// - every committed file under `output_root` that the emitter does not
///   produce is reported stale (b015 failure mode 5),
/// - all findings are reported at once, never the first.
///
/// Never writes, including on failure. Check mode is structurally incapable
/// of writing: this is the only entry point the `--check` branch of the bin
/// calls.
pub fn check_outputs(output_root: &Path, files: &[GeneratedFile]) -> crate::Result<CheckReport> {
    let mut report = CheckReport::default();

    for file in files {
        let committed = output_root.join(&file.path);
        match fs::read(&committed) {
            Ok(bytes) => {
                let existing = String::from_utf8_lossy(&bytes);
                if existing != file.contents {
                    let kind = if whitespace_equivalent(&existing, &file.contents) {
                        DriftKind::WhitespaceOnly
                    } else {
                        DriftKind::Content
                    };
                    report.drifted.push((file.path.clone().into(), kind));
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                report
                    .drifted
                    .push((file.path.clone().into(), DriftKind::Missing));
            }
            Err(error) => {
                return Err(crate::CodegenError::Read {
                    path: committed,
                    source: error,
                });
            }
        }
    }

    let expected: std::collections::BTreeSet<&str> =
        files.iter().map(|file| file.path.as_str()).collect();
    // A missing output root means every generated file is Missing (and
    // there are no orphans on disk); the walk is skipped so the report
    // stays all-findings rather than erroring on the missing directory.
    let on_disk = if output_root.exists() {
        walk_files(output_root).map_err(|error| crate::CodegenError::Read {
            path: output_root.to_path_buf(),
            source: error,
        })?
    } else {
        Vec::new()
    };
    for on_disk in on_disk {
        let relative = on_disk
            .strip_prefix(output_root)
            .expect("walk result is under the output root");
        let relative = relative
            .to_str()
            .expect("generated paths are UTF-8")
            .replace(std::path::MAIN_SEPARATOR, "/");
        if !expected.contains(relative.as_str()) {
            report.stale.push(relative.into());
        }
    }

    report.drifted.sort_by(|a, b| a.0.cmp(&b.0));
    report.stale.sort();
    Ok(report)
}

/// Whether two strings differ only in whitespace characters. Conservative
/// classification: a difference inside a string literal that only involves
/// whitespace would also classify as whitespace-only, but the file still
/// fails the gate either way — the classification only decides the label.
fn whitespace_equivalent(a: &str, b: &str) -> bool {
    let stripped = |s: &str| s.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    stripped(a) == stripped(b)
}

/// Recursively lists every file under `root`, sorted, as absolute paths.
fn walk_files(root: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    walk_files_into(root, &mut out)?;
    out.sort();
    Ok(out)
}

fn walk_files_into(dir: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            walk_files_into(&path, out)?;
        } else {
            out.push(path);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whitespace_classification_ignores_whitespace_only_diffs() {
        assert!(whitespace_equivalent(
            "export type A = \"x\";\n",
            "export   type A =\n  \"x\";"
        ));
    }

    #[test]
    fn whitespace_classification_keeps_content_diffs() {
        assert!(!whitespace_equivalent(
            "export type A = \"x\";",
            "export type A = \"y\";"
        ));
    }

    #[test]
    fn report_message_lists_every_finding_classified() {
        let report = CheckReport {
            drifted: vec![
                (PathBuf::from("badge.ts"), DriftKind::Content),
                (PathBuf::from("gauge.ts"), DriftKind::WhitespaceOnly),
            ],
            stale: vec![PathBuf::from("orphan.ts")],
        };
        let message = report.message();
        assert!(message.contains("badge.ts (content drift)"));
        assert!(message.contains("gauge.ts (whitespace-only difference)"));
        assert!(message.contains("orphan.ts (stale orphan)"));
    }
}
