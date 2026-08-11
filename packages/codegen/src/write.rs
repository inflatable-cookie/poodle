//! Write mode (`ir:build`) — the only place in the crate that writes to the
//! worktree.
//!
//! Write mode is deliberately a separate module from [`crate::check`]: the
//! gate never composes a write-mode generator (ruling R3 — the `45caae82`
//! failure was `docs:check` reaching `tokens:build` through
//! `report:parity`). `ir:build` is the write selector, `ir:check` the
//! read-only one; nothing calls both.

use std::fs;
use std::path::Path;

use crate::emit::GeneratedFile;
use crate::error::{CodegenError, Result};

/// Materializes the generated files under `output_root`, mirroring the
/// icons script's write mode: stale orphans are deleted, every expected
/// file is written. Orphan deletion walks the whole root recursively so the
/// write mode and the check mode agree on what "stale" means.
pub fn write_outputs(output_root: &Path, files: &[GeneratedFile]) -> Result<()> {
    let expected: std::collections::BTreeSet<&str> =
        files.iter().map(|file| file.path.as_str()).collect();

    // A fresh output root has no orphans to delete; the check mode treats
    // the same situation as Missing drift.
    if output_root.exists() {
        let on_disk = list_files(output_root)?;
        for path in on_disk {
            let relative = path
                .strip_prefix(output_root)
                .expect("walk result is under the output root")
                .to_str()
                .expect("generated paths are UTF-8")
                .replace(std::path::MAIN_SEPARATOR, "/");
            if !expected.contains(relative.as_str()) {
                fs::remove_file(&path).map_err(|error| CodegenError::Write {
                    path: path.clone(),
                    source: error,
                })?;
            }
        }
    }

    for file in files {
        let path = output_root.join(&file.path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| CodegenError::Write {
                path: parent.to_path_buf(),
                source: error,
            })?;
        }
        fs::write(&path, &file.contents).map_err(|error| CodegenError::Write {
            path: path.clone(),
            source: error,
        })?;
    }
    Ok(())
}

/// Recursively lists every file under `root`, sorted.
fn list_files(root: &Path) -> Result<Vec<std::path::PathBuf>> {
    let mut out = Vec::new();
    list_files_into(root, &mut out)?;
    out.sort();
    Ok(out)
}

fn list_files_into(dir: &Path, out: &mut Vec<std::path::PathBuf>) -> Result<()> {
    let entries = fs::read_dir(dir).map_err(|error| CodegenError::Read {
        path: dir.to_path_buf(),
        source: error,
    })?;
    for entry in entries {
        let entry = entry.map_err(|error| CodegenError::Read {
            path: dir.to_path_buf(),
            source: error,
        })?;
        let path = entry.path();
        if path.is_dir() {
            list_files_into(&path, out)?;
        } else {
            out.push(path);
        }
    }
    Ok(())
}
