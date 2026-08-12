//! Shared helpers for the JSON-emitting targets (surface, schema, registry,
//! conformance).
//!
//! The `//`-comment header of [`crate::emit::header`] is not valid JSON, so
//! the JSON targets carry the same `IR-07` content — generator name and
//! version, authored source path, IR schema version — as a `generated`
//! object field. No timestamp, no absolute path, no machine or user
//! identifier; a pure function of the source path, byte-identical across
//! runs and machines.

use poodle_ir::IR_SCHEMA_VERSION;

use crate::emit::GeneratedFile;
use crate::GENERATOR_VERSION;

/// The `IR-07` header content as a JSON object. Exact field names are part
/// of the emitted contract: consumers and the emitted JSON Schema read
/// `generator`, `source`, and `ir_schema_version` here.
pub(crate) fn generated_json(source_path: &str) -> serde_json::Value {
    serde_json::json!({
        "generator": format!("poodle-codegen {GENERATOR_VERSION}"),
        "source": source_path,
        "ir_schema_version": IR_SCHEMA_VERSION,
    })
}

/// Builds a generated JSON document: pretty-printed, trailing newline, the
/// header object first (deterministic key order — serde_json maps sort
/// keys, so two runs of the same inputs land byte-identical).
pub(crate) fn json_file(path: impl Into<String>, document: serde_json::Value) -> GeneratedFile {
    let mut contents =
        serde_json::to_string_pretty(&document).expect("JSON serialization cannot fail");
    contents.push('\n');
    GeneratedFile::new(path, contents)
}
