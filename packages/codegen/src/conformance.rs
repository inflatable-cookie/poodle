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

/** True for a portable dimension value: a rem length (`14rem`, `20.5rem`).
 * Anything else is rejected at codegen, so an unsupported value never
 * silently becomes a default (popover contract §12). */
fn is_portable_dimension(raw: &str) -> bool {
    let Some(value) = raw.strip_suffix("rem").map(str::trim) else {
        return false;
    };
    let mut digits = value.split('.');
    let whole = digits.next().unwrap_or_default();
    if whole.is_empty() || !whole.chars().all(|ch| ch.is_ascii_digit()) {
        return false;
    }
    match digits.next() {
        None => true,
        Some(frac) => {
            digits.next().is_none() && !frac.is_empty() && frac.chars().all(|ch| ch.is_ascii_digit())
        }
    }
}

const GEOMETRY_FIELDS: [&str; 14] = [
    "height",
    "minWidth",
    "paddingLeft",
    "paddingRight",
    "radius",
    "borderWidth",
    "topGap",
    "bottomGap",
    "leftGap",
    "rightGap",
    "hStart",
    "hEnd",
    "vStart",
    "widthGap",
];

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
#[serde(rename_all = "camelCase")]
pub struct ScalarType {
    pub kind: String,
    #[serde(default)]
    pub values: Option<Vec<String>>,
    #[serde(default)]
    pub fields: Option<Vec<ItemFieldDecl>>,
    #[serde(default)]
    pub rust_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ItemFieldDecl {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: ScalarType,
    #[serde(default)]
    pub optional: bool,
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
    pub events: Vec<Named>,
    pub regions: Vec<Named>,
    pub parts: Vec<PartDecl>,
    pub states: Vec<StateDecl>,
    pub token_roles: Vec<TokenRoleDecl>,
    pub axes: Vec<String>,
    pub capabilities: Vec<PortableCapability>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PortableCapability {
    pub name: String,
    #[allow(dead_code)]
    pub required: bool,
}

#[derive(Debug, Deserialize)]
struct PrimitiveCapabilityRoster {
    schema: String,
    capabilities: Vec<PrimitiveCapabilityRow>,
}

#[derive(Debug, Deserialize)]
struct PrimitiveCapabilityRow {
    id: String,
}

/// A named declaration (event or region).
#[derive(Debug, Clone, Deserialize)]
pub struct Named {
    pub name: String,
}

/// A stable part with its native resolution descriptor.
#[derive(Debug, Clone, Deserialize)]
pub struct PartDecl {
    pub id: String,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub resolve: Option<ResolveDecl>,
    #[serde(default)]
    pub repeat: Option<RepeatDecl>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RepeatDecl {
    pub prop: String,
    pub key: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResolveDecl {
    pub web: serde_json::Value,
    pub native: serde_json::Value,
}

/// An observable state with its native observation rule.
#[derive(Debug, Clone, Deserialize)]
pub struct StateDecl {
    pub name: String,
    pub native: String,
    #[serde(default)]
    pub part: Option<String>,
}

/// A semantic token role projecting a prop.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenRoleDecl {
    pub name: String,
    #[serde(default)]
    pub prop: String,
    #[serde(default)]
    pub default: Option<String>,
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
        #[serde(default)]
        key: Option<String>,
        #[serde(default)]
        target: Option<String>,
    },
    #[serde(rename = "expectPart")]
    ExpectPart {
        part: String,
        expect: serde_json::Value,
    },
    #[serde(rename = "expectEvents")]
    ExpectEvents { events: Vec<String> },
}

/// One serialized component case.
#[derive(Debug, Clone, Deserialize)]
pub struct ComponentCase {
    pub id: String,
    pub fixture: serde_json::Value,
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

/// Validates the case corpus against the interface: every fixture prop,
/// region, part, state, event, token role, axis, and enum value must be
/// closed over the interface. Unknown names are errors, never ignored.
pub fn validate_cases(interface: &ComponentInterface, cases: &ComponentCases) -> Result<()> {
    let mut findings = Vec::new();
    let props: std::collections::HashMap<&str, &PortableProp> = interface
        .props
        .iter()
        .map(|p| (p.name.as_str(), p))
        .collect();
    let regions: std::collections::HashSet<&str> =
        interface.regions.iter().map(|r| r.name.as_str()).collect();
    let parts: std::collections::HashSet<&str> = interface
        .parts
        .iter()
        .filter(|p| p.repeat.is_none())
        .map(|p| p.id.as_str())
        .collect();
    let states: std::collections::HashSet<&str> =
        interface.states.iter().map(|s| s.name.as_str()).collect();
    let events: std::collections::HashSet<&str> =
        interface.events.iter().map(|e| e.name.as_str()).collect();
    let token_roles: std::collections::HashSet<&str> = interface
        .token_roles
        .iter()
        .map(|t| t.name.as_str())
        .collect();
    let axes: std::collections::HashSet<&str> = interface.axes.iter().map(String::as_str).collect();

    for part in interface.parts.iter().filter(|part| part.repeat.is_some()) {
        let repeat = part.repeat.as_ref().expect("filtered repeated part");
        let key_is_string = props
            .get(repeat.prop.as_str())
            .filter(|prop| prop.kind.kind == "collection")
            .and_then(|prop| prop.kind.fields.as_deref())
            .and_then(|fields| fields.iter().find(|field| field.name == repeat.key))
            .is_some_and(|field| field.kind.kind == "string");
        if !key_is_string {
            findings.push(format!(
                "part '{}' repeated key '{}.{}' must be a string field",
                part.id, repeat.prop, repeat.key
            ));
        }
    }

    for case in &cases.cases {
        let at = |message: String| format!("case '{}': {message}", case.id);
        let mut repeated_part_keys: std::collections::HashMap<
            &str,
            std::collections::HashSet<String>,
        > = std::collections::HashMap::new();

        if let Some(fixture_props) = case
            .fixture
            .get("props")
            .and_then(serde_json::Value::as_object)
        {
            for (key, value) in fixture_props {
                let Some(prop) = props.get(key.as_str()) else {
                    findings.push(at(format!("unknown prop '{key}'")));
                    continue;
                };
                if value.is_null() {
                    if !prop.nullable {
                        findings.push(at(format!("prop '{key}' is not nullable")));
                    }
                    continue;
                }
                match prop.kind.kind.as_str() {
                    "boolean" => {
                        if !value.is_boolean() {
                            findings.push(at(format!("prop '{key}' expects a boolean")));
                        }
                    }
                    "number" => {
                        if !value.is_number() {
                            findings.push(at(format!("prop '{key}' expects a number")));
                        }
                    }
                    "numberPair" => {
                        let valid = value.as_array().is_some_and(|pair| {
                            pair.len() == 2 && pair.iter().all(serde_json::Value::is_number)
                        });
                        if !valid {
                            findings.push(at(format!("prop '{key}' expects a number pair")));
                        }
                    }
                    "collection" => {
                        let Some(items) = value.as_array() else {
                            findings.push(at(format!("prop '{key}' expects an object collection")));
                            continue;
                        };
                        let fields = prop.kind.fields.as_deref().unwrap_or_default();
                        for (index, item) in items.iter().enumerate() {
                            let Some(item) = item.as_object() else {
                                findings.push(at(format!(
                                    "prop '{key}' item {index} expects an object"
                                )));
                                continue;
                            };
                            for field in fields {
                                let Some(field_value) = item.get(&field.name) else {
                                    if !field.optional {
                                        findings.push(at(format!(
                                            "prop '{key}' item {index} misses field '{}'",
                                            field.name
                                        )));
                                    }
                                    continue;
                                };
                                let valid = match field.kind.kind.as_str() {
                                    "boolean" => field_value.is_boolean(),
                                    "number" => field_value.is_number(),
                                    "string" | "icon" => field_value.is_string(),
                                    _ => false,
                                };
                                if !valid {
                                    findings.push(at(format!(
                                        "prop '{key}' item {index} field '{}' has wrong type",
                                        field.name
                                    )));
                                }
                            }
                            for field_name in item.keys() {
                                if !fields.iter().any(|field| field.name == *field_name) {
                                    findings.push(at(format!("prop '{key}' item {index} uses unknown field '{field_name}'")));
                                }
                            }
                        }
                    }
                    "string" | "icon" => {
                        if !value.is_string() {
                            findings.push(at(format!("prop '{key}' expects a string")));
                        }
                    }
                    "dimension" => {
                        if !value
                            .as_str()
                            .is_some_and(|raw| is_portable_dimension(raw))
                        {
                            findings.push(at(format!(
                                "prop '{key}' value is not a portable rem length (the popover §12 subset)"
                            )));
                        }
                    }
                    "enum" => {
                        let Some(expected) = value.as_str() else {
                            findings.push(at(format!("prop '{key}' expects an enum string")));
                            continue;
                        };
                        let values = prop.kind.values.as_deref().unwrap_or_default();
                        if !values.iter().any(|v| v == expected) {
                            findings.push(at(format!(
                                "prop '{key}' value '{expected}' is not one of {values:?}"
                            )));
                        }
                    }
                    other => findings.push(at(format!("prop '{key}' has unknown type '{other}'"))),
                }
            }
        }

        for part in interface.parts.iter().filter(|part| part.repeat.is_some()) {
            let repeat = part.repeat.as_ref().expect("filtered repeated part");
            let mut keys = std::collections::HashSet::new();
            if let Some(items) = case
                .fixture
                .get("props")
                .and_then(|p| p.get(&repeat.prop))
                .and_then(serde_json::Value::as_array)
            {
                for (index, item) in items.iter().enumerate() {
                    let key = item.get(&repeat.key).and_then(serde_json::Value::as_str);
                    match key {
                        Some(key) if !key.is_empty() && keys.insert(key.to_owned()) => {}
                        Some(key) if !key.is_empty() => findings.push(at(format!(
                            "prop '{}' has duplicate key '{key}'",
                            repeat.prop
                        ))),
                        _ => findings.push(at(format!(
                            "prop '{}' item {index} key '{}' must be a non-empty string",
                            repeat.prop, repeat.key
                        ))),
                    }
                }
            }
            repeated_part_keys.insert(part.id.as_str(), keys);
        }

        let part_is_known = |part: &str| {
            parts.contains(part)
                || part.split_once(':').is_some_and(|(base, key)| {
                    repeated_part_keys
                        .get(base)
                        .is_some_and(|keys| keys.contains(key))
                })
        };

        if let Some(fixture_regions) = case
            .fixture
            .get("regions")
            .and_then(serde_json::Value::as_object)
        {
            for key in fixture_regions.keys() {
                if !regions.contains(key.as_str()) {
                    findings.push(at(format!("unknown region '{key}'")));
                }
            }
        }

        for step in &case.steps {
            match step {
                CaseStep::Action {
                    part,
                    name,
                    target,
                    ..
                } => {
                    if !part_is_known(part) {
                        findings.push(at(format!("action targets unknown part '{part}'")));
                    }
                    if !matches!(
                        name.as_str(),
                        "press" | "focus" | "key" | "scrub" | "dismiss" | "pointer"
                    ) {
                        findings.push(at(format!("unknown action '{name}'")));
                    }
                    if name == "pointer"
                        && !matches!(target.as_deref(), Some("inside") | Some("outside"))
                    {
                        findings.push(at(format!(
                            "pointer action needs target inside|outside, got {target:?}"
                        )));
                    }
                }
                CaseStep::ExpectPart { part, expect } => {
                    if !part_is_known(part) {
                        findings.push(at(format!("expects unknown part '{part}'")));
                    }
                    if let Some(expect_states) =
                        expect.get("states").and_then(serde_json::Value::as_object)
                    {
                        for key in expect_states.keys() {
                            if !states.contains(key.as_str()) {
                                findings.push(at(format!("expects unknown state '{key}'")));
                            }
                        }
                    }
                    if let Some(expect_roles) = expect
                        .get("tokenRoles")
                        .and_then(serde_json::Value::as_object)
                    {
                        for key in expect_roles.keys() {
                            if !token_roles.contains(key.as_str()) {
                                findings.push(at(format!("expects unknown token role '{key}'")));
                            }
                        }
                    }
                    validate_geometry_expectation(expect, &at, &mut findings);
                }
                CaseStep::ExpectEvents { events: expected } => {
                    for event in expected {
                        if !events.contains(event.as_str()) {
                            findings.push(at(format!("expects unknown event '{event}'")));
                        }
                    }
                }
            }
        }

        if let Some(specimen_axes) = case
            .specimen
            .get("axes")
            .and_then(serde_json::Value::as_array)
        {
            for axis in specimen_axes {
                let Some(axis) = axis.as_str() else {
                    findings.push(at("specimen axis is not a string".to_owned()));
                    continue;
                };
                if !axes.contains(axis) {
                    findings.push(at(format!("unknown specimen axis '{axis}'")));
                }
            }
        }
    }

    if findings.is_empty() {
        Ok(())
    } else {
        Err(CodegenError::Gate {
            message: format!(
                "case corpus {} does not validate against interface {}:\n{}",
                cases.component,
                interface.id,
                findings.join("\n")
            ),
        })
    }
}

fn validate_geometry_expectation(
    expect: &serde_json::Value,
    at: &impl Fn(String) -> String,
    findings: &mut Vec<String>,
) {
    let Some(geometry) = expect.get("geometry") else {
        return;
    };
    let Some(geometry) = geometry.as_object() else {
        findings.push(at("geometry expectation must be an object".to_owned()));
        return;
    };
    match geometry
        .get("tolerance")
        .and_then(serde_json::Value::as_f64)
    {
        Some(tolerance) if tolerance.is_finite() && tolerance >= 0.0 => {}
        _ => findings.push(at(
            "geometry tolerance must be an authored finite non-negative number".to_owned(),
        )),
    }
    let mut field_count = 0;
    for (field, value) in geometry {
        if field == "tolerance" {
            continue;
        }
        if !GEOMETRY_FIELDS.contains(&field.as_str()) {
            findings.push(at(format!("unknown geometry field '{field}'")));
            continue;
        }
        field_count += 1;
        if !value.as_f64().is_some_and(f64::is_finite) {
            findings.push(at(format!("geometry '{field}' must be a finite number")));
        }
    }
    if field_count == 0 {
        findings.push(at(
            "geometry expectation needs at least one asserted field".to_owned()
        ));
    }
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
    let roster_path = path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("primitive-capability-roster.json");
    let roster_document = fs::read_to_string(&roster_path).map_err(|error| CodegenError::Read {
        path: roster_path.clone(),
        source: error,
    })?;
    let roster: PrimitiveCapabilityRoster =
        serde_json::from_str(&roster_document).map_err(|error| CodegenError::Gate {
            message: format!(
                "primitive capability roster {} is not valid JSON: {error}",
                roster_path.display()
            ),
        })?;
    if roster.schema != "primitive-capability-roster.v1" {
        return Err(CodegenError::Gate {
            message: format!(
                "primitive capability roster {} has schema '{}', expected primitive-capability-roster.v1",
                roster_path.display(),
                roster.schema
            ),
        });
    }
    validate_interface_capabilities(&interface, &roster, path)?;
    Ok(interface)
}

fn validate_interface_capabilities(
    interface: &ComponentInterface,
    roster: &PrimitiveCapabilityRoster,
    interface_path: &Path,
) -> Result<()> {
    let known: std::collections::HashSet<&str> = roster
        .capabilities
        .iter()
        .map(|row| row.id.as_str())
        .collect();
    for capability in &interface.capabilities {
        if !known.contains(capability.name.as_str()) {
            return Err(CodegenError::Gate {
                message: format!(
                    "conformance interface {} names unknown primitive capability '{}'",
                    interface_path.display(),
                    capability.name
                ),
            });
        }
    }
    Ok(())
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
                fields: None,
                rust_type: None,
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

    #[test]
    fn geometry_requires_an_explicit_local_tolerance() {
        let expect = serde_json::json!({ "geometry": { "height": 36 } });
        let mut findings = Vec::new();
        validate_geometry_expectation(&expect, &|message| message, &mut findings);
        assert_eq!(
            findings,
            ["geometry tolerance must be an authored finite non-negative number"]
        );
    }

    #[test]
    fn rust_validator_rejects_non_rem_dimensions() {
        let interface: ComponentInterface = serde_json::from_value(serde_json::json!({
            "schemaVersion": 1, "id": "probe", "profile": "overlay",
            "props": [{ "name": "surfaceMinWidth", "type": { "kind": "dimension" }, "default": null, "nullable": true }],
            "events": [], "regions": [],
            "parts": [{ "id": "root", "resolve": { "web": { "kind": "self" }, "native": { "kind": "self" } } }],
            "states": [], "tokenRoles": [], "axes": [], "capabilities": []
        })).unwrap();
        let cases: ComponentCases = serde_json::from_value(serde_json::json!({
            "schemaVersion": 1, "component": "probe", "cases": [{
                "id": "probe/planted", "fixture": { "props": { "surfaceMinWidth": "320px" } },
                "specimen": { "axes": [] },
                "steps": []
            }]
        })).unwrap();
        let error = validate_cases(&interface, &cases).unwrap_err();
        assert!(error.to_string().contains("portable rem length"));
    }

    #[test]
    fn rust_validator_accepts_rem_dimensions() {
        let interface: ComponentInterface = serde_json::from_value(serde_json::json!({
            "schemaVersion": 1, "id": "probe", "profile": "overlay",
            "props": [{ "name": "surfaceMinWidth", "type": { "kind": "dimension" }, "default": null, "nullable": true }],
            "events": [], "regions": [],
            "parts": [{ "id": "root", "resolve": { "web": { "kind": "self" }, "native": { "kind": "self" } } }],
            "states": [], "tokenRoles": [], "axes": [], "capabilities": []
        })).unwrap();
        let cases: ComponentCases = serde_json::from_value(serde_json::json!({
            "schemaVersion": 1, "component": "probe", "cases": [{
                "id": "probe/ok", "fixture": { "props": { "surfaceMinWidth": "20.5rem" } },
                "specimen": { "axes": [] },
                "steps": []
            }]
        })).unwrap();
        validate_cases(&interface, &cases).expect("rem dimensions validate");
    }

    #[test]
    fn rust_loading_rejects_unknown_primitive_capability() {
        let interface = ComponentInterface {
            schema_version: 1,
            id: "probe".to_owned(),
            profile: "control".to_owned(),
            props: Vec::new(),
            events: Vec::new(),
            regions: Vec::new(),
            parts: Vec::new(),
            states: Vec::new(),
            token_roles: Vec::new(),
            axes: Vec::new(),
            capabilities: vec![PortableCapability {
                name: "not-a-capability".to_owned(),
                required: true,
            }],
        };
        let roster = PrimitiveCapabilityRoster {
            schema: "primitive-capability-roster.v1".to_owned(),
            capabilities: vec![PrimitiveCapabilityRow {
                id: "activate".to_owned(),
            }],
        };
        let error = validate_interface_capabilities(&interface, &roster, Path::new("probe.json"))
            .expect_err("unknown capability must fail Rust loading");
        assert!(error.to_string().contains("not-a-capability"));
    }

    fn tabs_documents(
        items: serde_json::Value,
        part: &str,
    ) -> (ComponentInterface, ComponentCases) {
        let interface: ComponentInterface = serde_json::from_value(serde_json::json!({
            "schemaVersion": 1, "id": "tabs", "profile": "collection",
            "props": [{ "name": "items", "type": { "kind": "collection", "rustType": "TabDefinition", "fields": [
                { "name": "value", "type": { "kind": "string" } },
                { "name": "label", "type": { "kind": "string" } }
            ]}, "default": [] }],
            "events": [], "regions": [],
            "parts": [{ "id": "trigger", "repeat": { "prop": "items", "key": "value" } }],
            "states": [], "tokenRoles": [], "axes": [], "capabilities": []
        })).unwrap();
        let cases: ComponentCases = serde_json::from_value(serde_json::json!({
            "schemaVersion": 1, "component": "tabs", "cases": [{
                "id": "tabs/planted", "fixture": { "props": { "items": items }, "regions": {} },
                "specimen": { "axes": [] },
                "steps": [{ "kind": "action", "name": "press", "part": part }]
            }]
        }))
        .unwrap();
        (interface, cases)
    }

    #[test]
    fn rust_validator_rejects_duplicate_collection_keys() {
        let (interface, cases) = tabs_documents(
            serde_json::json!([
                { "value": "overview", "label": "Overview" },
                { "value": "overview", "label": "Duplicate" }
            ]),
            "trigger:overview",
        );
        let error = validate_cases(&interface, &cases).unwrap_err();
        assert!(error.to_string().contains("duplicate key 'overview'"));
    }

    #[test]
    fn rust_validator_rejects_repeated_part_outside_collection() {
        let (interface, cases) = tabs_documents(
            serde_json::json!([
                { "value": "overview", "label": "Overview" }
            ]),
            "trigger:missing",
        );
        let error = validate_cases(&interface, &cases).unwrap_err();
        assert!(error.to_string().contains("unknown part 'trigger:missing'"));
    }
}
