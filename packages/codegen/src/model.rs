//! Loading and validating a serialized `IrModel` for emission.

use std::fs;
use std::path::Path;

use poodle_ir::IrModel;

use crate::error::{CodegenError, Result};

/// Reads the fixture JSON, deserializes it as the current IR schema, and
/// runs whole-model validation. Any failure — unreadable file, malformed
/// JSON, or validation findings — is a clean [`CodegenError`], never a
/// panic. Invalid models are refused at the door: emission only ever sees
/// a validated model, so reference resolution during rendering cannot fail
/// on a well-formed input.
pub fn load_and_validate(path: &Path) -> Result<IrModel> {
    let source = fs::read_to_string(path).map_err(|error| CodegenError::Read {
        path: path.to_path_buf(),
        source: error,
    })?;

    let model: IrModel =
        serde_json::from_str(&source).map_err(|error| CodegenError::Malformed {
            path: path.to_path_buf(),
            detail: error.to_string(),
        })?;

    let findings = model.validate();
    if !findings.is_empty() {
        return Err(CodegenError::Invalid {
            path: path.to_path_buf(),
            findings,
        });
    }

    Ok(model)
}
