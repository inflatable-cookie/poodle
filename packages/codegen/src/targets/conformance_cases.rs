//! Conformance cases target: copies the serialized interface and case
//! corpus into a consuming crate's generated tree so the native runners and
//! specimen pages can `include_str!` them without reaching across packages.
//!
//! Select-only: reached via `--conformance <interface> --cases <corpus>
//! --out <crate-src>` with `--target conformance-cases`; not in
//! [`super::all`].

use std::fs;
use std::path::Path;

use crate::emit::GeneratedFile;
use crate::error::Result;
use crate::CodegenError;

/// Target id accepted by `--target` in conformance mode.
pub const ID: &str = "conformance-cases";

/// Output root relative to `--out` (the consuming crate's `src/`). Nested so
/// the shell/specimen targets' top-level orphan scans never flag it.
pub const OUTPUT_ROOT: &str = "generated/conformance";

/// Renders the JSON copies. Reads the committed fixtures (the source of
/// truth the serializer authored); the copies are byte-identical.
pub fn render(
    interface_path: &Path,
    cases_path: &Path,
) -> Result<Vec<GeneratedFile>> {
    let interface = fs::read_to_string(interface_path).map_err(|error| CodegenError::Read {
        path: interface_path.to_path_buf(),
        source: error,
    })?;
    let cases = fs::read_to_string(cases_path).map_err(|error| CodegenError::Read {
        path: cases_path.to_path_buf(),
        source: error,
    })?;
    Ok(vec![
        GeneratedFile::new("button-interface.json".to_owned(), interface),
        GeneratedFile::new("button-cases.json".to_owned(), cases),
    ])
}
