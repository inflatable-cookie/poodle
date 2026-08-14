//! Conformance authority parsing (spec 066): the serialized portable
//! interface and component case corpus authored in TypeScript under
//! `packages/core/src/conformance/` and emitted as neutral JSON fixtures by
//! `conformance:build`'s serializer step. The Rust pipeline consumes these
//! through `serde` and never re-authors them.

use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::error::Result;
use crate::CodegenError;

/// One prop in the portable interface.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortableProp {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: ScalarType,
    pub default: serde_json::Value,
    #[serde(default)]
    pub nullable: bool,
    /// Rust field name when not the mechanical camelCase → snake_case form.
    #[serde(default)]
    pub rust_name: Option<String>,
    /// Rust type in `crate::types` for enum props.
    #[serde(default)]
    pub rust_type: Option<String>,
    /// Generated Rust enum name when the values are not a poodle type.
    #[serde(default)]
    pub rust_enum_name: Option<String>,
    /// Controlled-state pair: the event carrying the new value.
    #[serde(default)]
    pub controlled_by: Option<String>,
    /// Platform extension marker. Extensions are not portable.
    #[serde(default)]
    pub extension: Option<String>,
}

/// A scalar type in the portable interface.
#[derive(Debug, Clone, Deserialize)]
pub struct ScalarType {
    pub kind: String,
    #[serde(default)]
    pub values: Option<Vec<String>>,
}

/// The serialized portable interface (`button-interface.json`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentInterface {
    #[allow(dead_code)]
    pub schema_version: u32,
    pub id: String,
    pub profile: String,
    pub props: Vec<PortableProp>,
    #[allow(dead_code)]
    pub events: Vec<serde_json::Value>,
    #[allow(dead_code)]
    pub regions: Vec<serde_json::Value>,
    #[allow(dead_code)]
    pub parts: Vec<serde_json::Value>,
    #[allow(dead_code)]
    pub states: Vec<serde_json::Value>,
    #[allow(dead_code)]
    pub capabilities: Vec<serde_json::Value>,
}

/// A step in a component case.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind")]
pub enum CaseStep {
    #[serde(rename = "action")]
    Action {
        name: String,
        part: String,
        #[serde(default)]
        input: Option<String>,
    },
    #[serde(rename = "expectPart")]
    ExpectPart {
        part: String,
        #[allow(dead_code)]
        expect: serde_json::Value,
    },
    #[serde(rename = "expectEvents")]
    ExpectEvents { events: Vec<String> },
}

/// One serialized component case.
#[derive(Debug, Clone, Deserialize)]
pub struct ComponentCase {
    pub id: String,
    #[allow(dead_code)]
    pub fixture: serde_json::Value,
    #[allow(dead_code)]
    pub specimen: serde_json::Value,
    pub steps: Vec<CaseStep>,
}

/// The serialized case corpus (`button-cases.json`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentCases {
    #[allow(dead_code)]
    pub schema_version: u32,
    pub component: String,
    pub cases: Vec<ComponentCase>,
}

/// Loads and validates the interface JSON.
pub fn load_interface(path: &Path) -> Result<ComponentInterface> {
    let document = fs::read_to_string(path).map_err(|error| CodegenError::Read {
        path: path.to_path_buf(),
        source: error,
    })?;
    let interface: ComponentInterface =
        serde_json::from_str(&document).map_err(|error| CodegenError::Gate {
            message: format!(
                "conformance interface {} is not valid JSON: {error}",
                path.display()
            ),
        })?;
    if interface.schema_version != 1 {
        return Err(CodegenError::Gate {
            message: format!(
                "conformance interface {} has schema_version {}, expected 1",
                path.display(),
                interface.schema_version
            ),
        });
    }
    Ok(interface)
}

/// Loads and validates the case corpus JSON.
pub fn load_cases(path: &Path) -> Result<ComponentCases> {
    let document = fs::read_to_string(path).map_err(|error| CodegenError::Read {
        path: path.to_path_buf(),
        source: error,
    })?;
    let cases: ComponentCases =
        serde_json::from_str(&document).map_err(|error| CodegenError::Gate {
            message: format!(
                "conformance cases {} are not valid JSON: {error}",
                path.display()
            ),
        })?;
    if cases.schema_version != 1 {
        return Err(CodegenError::Gate {
            message: format!(
                "conformance cases {} have schema_version {}, expected 1",
                path.display(),
                cases.schema_version
            ),
        });
    }
    Ok(cases)
}

/// Portable props only: extensions are not generated into the Rust surface.
pub fn portable_props(interface: &ComponentInterface) -> Vec<&PortableProp> {
    interface
        .props
        .iter()
        .filter(|prop| prop.extension.is_none())
        .collect()
}

/// The Rust field name for a portable prop.
pub fn rust_field_name(prop: &PortableProp) -> String {
    if let Some(rust_name) = &prop.rust_name {
        return rust_name.clone();
    }
    to_snake_case(&prop.name)
}

/// Mechanical camelCase → snake_case.
pub fn to_snake_case(name: &str) -> String {
    let mut out = String::with_capacity(name.len() + 4);
    for (index, ch) in name.chars().enumerate() {
        if ch.is_uppercase() {
            if index > 0 {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snake_case_mechanical_transform() {
        assert_eq!(to_snake_case("leadingIcon"), "leading_icon");
        assert_eq!(to_snake_case("ariaExpanded"), "aria_expanded");
        assert_eq!(to_snake_case("defaultPressed"), "default_pressed");
        assert_eq!(to_snake_case("variant"), "variant");
    }

    #[test]
    fn rust_field_name_honors_explicit_override() {
        let mut prop = PortableProp {
            name: "disabled".to_owned(),
            kind: ScalarType {
                kind: "boolean".to_owned(),
                values: None,
            },
            default: serde_json::Value::Bool(false),
            nullable: false,
            rust_name: Some("is_disabled".to_owned()),
            rust_type: None,
            rust_enum_name: None,
            controlled_by: None,
            extension: None,
        };
        assert_eq!(rust_field_name(&prop), "is_disabled");
        prop.rust_name = None;
        assert_eq!(rust_field_name(&prop), "disabled");
    }
}
