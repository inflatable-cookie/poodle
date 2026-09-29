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
use crate::orphan::{is_protected_path, list_top_level_files};

/// Materializes the generated files under `output_root`, mirroring the
/// icons script's write mode: stale orphans are deleted, every expected
/// file is written. Orphan deletion walks the output root's **top level**
/// only: each target owns the top level of its own output root, and a
/// nested directory is another target's root (card 041: `shell-scene`
/// owns the top level of `generated/` inside the web preview packages,
/// so a recursive sweep would delete a sibling target's artifact). Write
/// mode and check mode agree on what "stale" means.
///
/// Exclusive ownership (the default, and every production `--out`): every
/// top-level file not in `files` is an orphan. Two targets that actually
/// share one directory pass [`write_outputs_protecting`] with the sibling's
/// exact relative paths so those artifacts survive and unclaimed garbage
/// is still removed.
pub fn write_outputs(output_root: &Path, files: &[GeneratedFile]) -> Result<()> {
    write_outputs_protecting(output_root, files, &[])
}

/// [`write_outputs`] when two targets share one concrete directory.
/// `protected` is exact relative paths the sibling currently emits, not
/// an extension glob.
pub fn write_outputs_protecting(
    output_root: &Path,
    files: &[GeneratedFile],
    protected: &[&str],
) -> Result<()> {
    let expected: std::collections::BTreeSet<&str> =
        files.iter().map(|file| file.path.as_str()).collect();

    // A fresh output root has no orphans to delete; the check mode treats
    // the same situation as Missing drift.
    if output_root.exists() {
        let on_disk = list_top_level_files(output_root)?;
        for path in on_disk {
            let relative = path
                .strip_prefix(output_root)
                .expect("walk result is under the output root")
                .to_str()
                .expect("generated paths are UTF-8")
                .replace(std::path::MAIN_SEPARATOR, "/");
            if expected.contains(relative.as_str()) {
                continue;
            }
            if is_protected_path(&relative, protected) {
                continue;
            }
            fs::remove_file(&path).map_err(|error| CodegenError::Write {
                path: path.clone(),
                source: error,
            })?;
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
