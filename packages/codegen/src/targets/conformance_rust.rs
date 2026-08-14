//! Rust portable-declaration target (spec 066 mechanism 1): renders the
//! serialized portable interface into the `ButtonSpec` declaration surface,
//! replacing the hand-written struct/default/builders. The extension methods
//! (token recipes, derived queries) stay hand-written beside it.
//!
//! Select-only: reached via `--conformance <interface> --out <crate-src>`
//! with `--target conformance-rust`; not in [`super::all`].

use crate::conformance::{portable_props, rust_field_name, ComponentInterface, PortableProp};
use crate::emit::{header, GeneratedFile};
use crate::error::Result;

/// Target id accepted by `--target` in conformance mode.
pub const ID: &str = "conformance-rust";

/// Output root relative to `--out` (the consuming crate's `src/`).
pub const OUTPUT_ROOT: &str = "generated";

/// Renders one module per declared component: `<id>/mod.rs`.
pub fn render(interface: &ComponentInterface, source_path: &str) -> Result<Vec<GeneratedFile>> {
    Ok(vec![GeneratedFile::new(
        format!("{}/mod.rs", interface.id),
        render_declaration(interface, source_path),
    )])
}

fn render_declaration(interface: &ComponentInterface, source_path: &str) -> String {
    let mut out = header(source_path);
    out.push_str("#![cfg_attr(rustfmt, rustfmt::skip)]\n\n");
    out.push_str(&format!(
        "//! Portable declaration for `{}` (profile: {}), generated from the\n\
         //! portable interface module. Struct, defaults, and builders are the\n\
         //! generated surface; token recipes and derived queries live in the\n\
         //! hand-written extension beside this file.\n\n",
        interface.id, interface.profile
    ));

    let props = portable_props(interface);

    // Generated enums first (props whose values are not a poodle type).
    for prop in &props {
        if let Some(enum_decl) = generated_enum(prop) {
            out.push_str(&enum_decl);
            out.push('\n');
        }
    }

    let struct_name = struct_name(interface);
    out.push_str(&render_struct(&struct_name, &props));
    out.push('\n');
    out.push_str(&render_default(&struct_name, &props));
    out.push('\n');
    out.push_str(&render_builders(&struct_name, &props));

    out
}

fn pascal_case(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut upper = true;
    for ch in value.chars() {
        if ch == '-' || ch == '_' {
            upper = true;
            continue;
        }
        if upper {
            out.push(ch.to_ascii_uppercase());
            upper = false;
        } else {
            out.push(ch);
        }
    }
    out
}

fn variant_name(value: &str) -> String {
    pascal_case(value)
}

/// A generated enum (prop values that are not a poodle type), if any.
fn generated_enum(prop: &PortableProp) -> Option<String> {
    if prop.kind.kind != "enum" || prop.rust_type.is_some() {
        return None;
    }
    let name = prop
        .rust_enum_name
        .clone()
        .unwrap_or_else(|| pascal_case(&prop.name));
    let values = prop.kind.values.as_deref().unwrap_or_default();
    let mut out =
        format!("#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]\npub enum {name} {{\n");
    for value in values {
        let variant = variant_name(value);
        let default = if value == "default" {
            "#[default] "
        } else {
            ""
        };
        out.push_str(&format!("    {default}{variant},\n"));
    }
    out.push_str("}\n");
    Some(out)
}

/// The Rust field type of a prop. Nullable props keep `Option` so absent
/// (`null`) stays distinguishable from an explicit value — the TypeScript
/// projection and the Rust surface must have the same shape.
fn prop_type(prop: &PortableProp) -> String {
    let base = match prop.kind.kind.as_str() {
        "boolean" => "bool".to_owned(),
        "icon" => "String".to_owned(),
        "dimension" => "crate::types::Dimension".to_owned(),
        "string" => "String".to_owned(),
        "collection" => format!(
            "Vec<crate::types::{}>",
            prop.kind
                .rust_type
                .as_deref()
                .unwrap_or("serde_json::Value")
        ),
        "enum" => {
            if let Some(rust_type) = &prop.rust_type {
                rust_named_type(rust_type)
            } else {
                prop.rust_enum_name
                    .clone()
                    .unwrap_or_else(|| pascal_case(&prop.name))
            }
        }
        other => other.to_owned(),
    };
    if prop.nullable {
        format!("Option<{base}>")
    } else {
        base
    }
}

fn struct_name(interface: &ComponentInterface) -> String {
    let suffix = if interface.profile == "collection" {
        "PortableSpec"
    } else {
        "Spec"
    };
    format!("{}{suffix}", pascal_case(&interface.id))
}

fn rust_named_type(name: &str) -> String {
    match name {
        "ActiveEdge" | "ActiveFill" => format!("crate::tabs::{name}"),
        _ => format!("crate::types::{name}"),
    }
}

fn render_struct(struct_name: &str, props: &[&PortableProp]) -> String {
    let mut out = format!("#[derive(Clone, Debug, Eq, PartialEq)]\npub struct {struct_name} {{\n");
    for prop in props {
        out.push_str(&format!(
            "    pub {}: {},\n",
            rust_field_name(prop),
            prop_type(prop)
        ));
    }
    out.push_str("}\n");
    out
}

fn render_default(struct_name: &str, props: &[&PortableProp]) -> String {
    let mut out = format!(
        "impl Default for {struct_name} {{\n    fn default() -> Self {{\n        Self {{\n"
    );
    for prop in props {
        out.push_str(&format!(
            "            {}: {},\n",
            rust_field_name(prop),
            rust_default(prop)
        ));
    }
    out.push_str("        }\n    }\n}\n");
    out
}

fn rust_default(prop: &PortableProp) -> String {
    let ty = prop_type(prop);
    if ty.starts_with("Option<") {
        return "None".to_owned();
    }
    if ty == "bool" {
        return prop.default.as_bool().unwrap_or(false).to_string();
    }
    if prop.kind.kind == "enum" {
        let name = if let Some(rust_type) = &prop.rust_type {
            rust_named_type(rust_type)
        } else {
            prop.rust_enum_name
                .clone()
                .unwrap_or_else(|| pascal_case(&prop.name))
        };
        let value = prop.default.as_str().unwrap_or("default");
        return format!("{name}::{}", variant_name(value));
    }
    if ty == "String" {
        let value = prop.default.as_str().unwrap_or_default();
        return format!("{value:?}.to_owned()");
    }
    "Default::default()".to_owned()
}

fn render_builders(struct_name: &str, props: &[&PortableProp]) -> String {
    let mut out = format!(
        "impl {struct_name} {{\n    pub fn new() -> Self {{\n        Self::default()\n    }}\n\n"
    );
    for prop in props {
        out.push_str(&render_builder(prop));
        out.push('\n');
    }
    out.push_str("}\n");
    out
}

fn render_builder(prop: &PortableProp) -> String {
    let field = rust_field_name(prop);
    let method = format!("with_{}", crate::conformance::to_snake_case(&prop.name));
    let ty = prop_type(prop);
    let (param, assign) = if ty.starts_with("Option<") {
        match ty.as_str() {
            "Option<String>" => (
                "impl Into<String>".to_owned(),
                format!("self.{field} = Some(value.into());"),
            ),
            "Option<crate::types::Dimension>" => (
                "impl Into<crate::types::Dimension>".to_owned(),
                format!("self.{field} = Some(value.into());"),
            ),
            "Option<bool>" => ("bool".to_owned(), format!("self.{field} = Some(value);")),
            // Nullable enums: the builder takes the plain value and wraps
            // it, so an explicit value stays distinguishable from absence.
            other => (
                other
                    .strip_prefix("Option<")
                    .and_then(|rest| rest.strip_suffix('>'))
                    .unwrap_or(other)
                    .to_owned(),
                format!("self.{field} = Some(value);"),
            ),
        }
    } else {
        (ty.clone(), format!("self.{field} = value;"))
    };
    format!(
        "    pub fn {method}(mut self, value: {param}) -> Self {{\n        {assign}\n        self\n    }}"
    )
}
