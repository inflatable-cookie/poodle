//! Shared top-level scan and sibling protection for check and write.
//!
//! Production invocations write one target per concrete directory (Svelte
//! catalogue-ts, GPUI catalogue-rust, …). Those sweeps are exclusive: every
//! top-level file not in `files` is an orphan, including a stray `.rs` in a
//! TypeScript root. Protection is exact relative paths, used only when two
//! targets actually share one directory.

use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{CodegenError, Result};

/// True when `relative` is an exact sibling-owned path, not this target's
/// orphan. Extension matching is intentionally not used: a stray `stale.rs`
/// in a TypeScript-only root is still an orphan.
pub fn is_protected_path(relative: &str, protected: &[&str]) -> bool {
    protected.iter().any(|path| *path == relative)
}

/// Lists the top-level files of `root`, sorted. Directories are skipped:
/// they are sibling targets' output roots, not this target's files (the
/// card 041 shared-`generated/` layout).
pub fn list_top_level_files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    for entry in fs::read_dir(root).map_err(|error| CodegenError::Read {
        path: root.to_path_buf(),
        source: error,
    })? {
        let entry = entry.map_err(|error| CodegenError::Read {
            path: root.to_path_buf(),
            source: error,
        })?;
        let path = entry.path();
        if !path.is_dir() {
            out.push(path);
        }
    }
    out.sort();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protection_is_exact_paths_not_extensions() {
        assert!(is_protected_path("owned.rs", &["owned.rs"]));
        assert!(!is_protected_path("stale.rs", &["owned.rs"]));
        assert!(!is_protected_path("orphan.json", &["owned.rs"]));
    }
}
