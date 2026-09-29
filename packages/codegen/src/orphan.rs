//! Shared top-level scan and sibling protection for check and write.
//!
//! Two targets may share one output root (catalogue-ts / catalogue-rust both
//! write `generated/catalogue/`). Each target still sweeps files it owns —
//! plus unclaimed garbage — and leaves sibling artifacts alone. Exclusive
//! targets pass an empty sibling-extension list and keep the old sweep.

use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{CodegenError, Result};

/// True when `relative` is a sibling target's artifact, not this target's
/// orphan. Extension comparison is exact (`"rs"`, not `".rs"`).
pub fn is_protected_sibling(relative: &str, sibling_extensions: &[&str]) -> bool {
    let Some(ext) = Path::new(relative).extension().and_then(|e| e.to_str()) else {
        return false;
    };
    sibling_extensions.iter().any(|owned| *owned == ext)
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
    fn sibling_rs_is_protected_for_a_ts_target() {
        assert!(is_protected_sibling("catalogue.rs", &["rs"]));
        assert!(!is_protected_sibling("catalogue.ts", &["rs"]));
        assert!(!is_protected_sibling("orphan.json", &["rs"]));
        assert!(!is_protected_sibling("noext", &["rs"]));
    }
}
