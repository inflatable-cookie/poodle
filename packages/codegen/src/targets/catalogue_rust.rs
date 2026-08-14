//! Rust preview-catalogue target (g14.025).
//!
//! Select-only sibling of [`super::catalogue_ts`]. Hosts pull the file in with
//! `#[path = "generated/catalogue/catalogue.rs"]`.

use crate::catalogue::{components_in_order, pascal_case, Document};
use crate::emit::{catalogue_header, GeneratedFile};

/// Target id accepted by `--target` in catalogue mode.
pub const ID: &str = "catalogue-rust";

/// Output root relative to `--out` (the consuming crate's `src/`).
pub const OUTPUT_ROOT: &str = "generated/catalogue";

/// Renders the canonical Rust catalogue module.
pub fn render(document: &Document, source_path: &str) -> Vec<GeneratedFile> {
    vec![GeneratedFile::new(
        "catalogue.rs",
        render_module(document, source_path),
    )]
}

fn render_module(document: &Document, source_path: &str) -> String {
    let mut out = catalogue_header(source_path, document.schema_version);
    out.push_str("#![allow(dead_code)]\n");
    out.push_str("#![cfg_attr(rustfmt, rustfmt::skip)]\n\n");
    out.push_str(
        "//! Canonical preview catalogue. Plain data, no Poodle crate imports.\n\
         //! Pulled into native previews via `#[path = \"generated/catalogue/catalogue.rs\"]`.\n\
         //! Regenerate with `effigy catalogue:build`; drift is gated by `effigy catalogue:check`.\n\n",
    );

    render_enum(
        &mut out,
        "CatalogueSectionId",
        document.sections.iter().map(|entry| entry.id.as_str()),
    );
    render_id_label_impl(
        &mut out,
        "CatalogueSectionId",
        &document
            .sections
            .iter()
            .map(|entry| (entry.id.as_str(), entry.label.as_str()))
            .collect::<Vec<_>>(),
    );

    render_enum(
        &mut out,
        "CatalogueFamilyId",
        document.families.iter().map(|entry| entry.id.as_str()),
    );
    render_id_label_impl(
        &mut out,
        "CatalogueFamilyId",
        &document
            .families
            .iter()
            .map(|entry| (entry.id.as_str(), entry.label.as_str()))
            .collect::<Vec<_>>(),
    );

    out.push_str("impl CatalogueFamilyId {\n");
    out.push_str("    pub fn section(self) -> CatalogueSectionId {\n");
    out.push_str("        match self {\n");
    for family in &document.families {
        out.push_str(&format!(
            "            Self::{} => CatalogueSectionId::{},\n",
            pascal_case(&family.id),
            pascal_case(&family.section)
        ));
    }
    out.push_str("        }\n    }\n}\n\n");

    render_enum(
        &mut out,
        "CatalogueKindId",
        document.kinds.iter().map(|entry| entry.id.as_str()),
    );
    render_id_label_impl(
        &mut out,
        "CatalogueKindId",
        &document
            .kinds
            .iter()
            .map(|entry| (entry.id.as_str(), entry.label.as_str()))
            .collect::<Vec<_>>(),
    );

    out.push_str(
        "#[derive(Clone, Copy, Debug, PartialEq, Eq)]\n\
         pub struct CatalogueFamily {\n\
         \tpub id: CatalogueFamilyId,\n\
         \tpub section: CatalogueSectionId,\n\
         \tpub label: &'static str,\n\
         }\n\n\
         #[derive(Clone, Copy, Debug, PartialEq, Eq)]\n\
         pub struct CanonicalComponent {\n\
         \tpub slug: &'static str,\n\
         \tpub display_name: &'static str,\n\
         \tpub description: &'static str,\n\
         \tpub section: CatalogueSectionId,\n\
         \tpub family: CatalogueFamilyId,\n\
         \tpub kind: CatalogueKindId,\n\
         \tpub collections: &'static [&'static str],\n\
         }\n\n",
    );

    out.push_str("pub const CATALOGUE_SECTIONS: &[CatalogueSectionId] = CatalogueSectionId::ALL;\n\n");
    out.push_str("pub const CATALOGUE_FAMILIES: &[CatalogueFamily] = &[\n");
    for family in &document.families {
        out.push_str(&format!(
            "    CatalogueFamily {{ id: CatalogueFamilyId::{}, section: CatalogueSectionId::{}, label: {} }},\n",
            pascal_case(&family.id),
            pascal_case(&family.section),
            rust_string(&family.label)
        ));
    }
    out.push_str("];\n\n");

    out.push_str("pub const CANONICAL_COMPONENTS: &[CanonicalComponent] = &[\n");
    for component in components_in_order(document) {
        let collections = if component.collections.is_empty() {
            "&[]".to_owned()
        } else {
            format!(
                "&[{}]",
                component
                    .collections
                    .iter()
                    .map(|id| rust_string(id))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        out.push_str(&format!(
            "    CanonicalComponent {{\n\
             \t\tslug: {},\n\
             \t\tdisplay_name: {},\n\
             \t\tdescription: {},\n\
             \t\tsection: CatalogueSectionId::{},\n\
             \t\tfamily: CatalogueFamilyId::{},\n\
             \t\tkind: CatalogueKindId::{},\n\
             \t\tcollections: {collections},\n\
             \t}},\n",
            rust_string(&component.slug),
            rust_string(&component.display_name),
            rust_string(&component.description),
            pascal_case(&component.section),
            pascal_case(&component.family),
            pascal_case(&component.kind)
        ));
    }
    out.push_str("];\n\n");

    out.push_str(
        "pub fn find_component(slug: &str) -> Option<&'static CanonicalComponent> {\n\
         \tCANONICAL_COMPONENTS.iter().find(|component| component.slug == slug)\n\
         }\n\n\
         pub fn family_by_id(id: CatalogueFamilyId) -> &'static CatalogueFamily {\n\
         \tCATALOGUE_FAMILIES\n\
         \t\t.iter()\n\
         \t\t.find(|family| family.id == id)\n\
         \t\t.expect(\"generated families cover every CatalogueFamilyId\")\n\
         }\n\n\
         pub fn matches_search(component: &CanonicalComponent, query: &str) -> bool {\n\
         \tlet query = query.trim().to_ascii_lowercase();\n\
         \tif query.is_empty() {\n\
         \t\treturn true;\n\
         \t}\n\
         \tlet family = family_by_id(component.family);\n\
         \tcomponent.display_name.to_ascii_lowercase().contains(&query)\n\
         \t\t|| component.description.to_ascii_lowercase().contains(&query)\n\
         \t\t|| family.label.to_ascii_lowercase().contains(&query)\n\
         \t\t|| component.kind.label().to_ascii_lowercase().contains(&query)\n\
         \t\t|| component.collections.iter().any(|label| label.to_ascii_lowercase().contains(&query))\n\
         }\n\n\
         pub fn search_components(query: &str) -> Vec<&'static CanonicalComponent> {\n\
         \tCANONICAL_COMPONENTS\n\
         \t\t.iter()\n\
         \t\t.filter(|component| matches_search(component, query))\n\
         \t\t.collect()\n\
         }\n",
    );

    out
}

fn render_enum<'a>(out: &mut String, name: &str, ids: impl Iterator<Item = &'a str>) {
    let ids: Vec<&str> = ids.collect();
    out.push_str("#[derive(Clone, Copy, Debug, PartialEq, Eq)]\n");
    out.push_str(&format!("pub enum {name} {{\n"));
    for id in &ids {
        out.push_str(&format!("    {},\n", pascal_case(id)));
    }
    out.push_str("}\n\n");
    out.push_str(&format!("impl {name} {{\n"));
    out.push_str("    pub const ALL: &[Self] = &[\n");
    for id in &ids {
        out.push_str(&format!("        Self::{},\n", pascal_case(id)));
    }
    out.push_str("    ];\n}\n\n");
}

fn render_id_label_impl(out: &mut String, name: &str, pairs: &[(&str, &str)]) {
    out.push_str(&format!("impl {name} {{\n"));
    out.push_str("    pub fn id(self) -> &'static str {\n        match self {\n");
    for (id, _) in pairs {
        out.push_str(&format!(
            "            Self::{} => {},\n",
            pascal_case(id),
            rust_string(id)
        ));
    }
    out.push_str("        }\n    }\n\n");
    out.push_str("    pub fn label(self) -> &'static str {\n        match self {\n");
    for (id, label) in pairs {
        out.push_str(&format!(
            "            Self::{} => {},\n",
            pascal_case(id),
            rust_string(label)
        ));
    }
    out.push_str("        }\n    }\n\n");
    out.push_str("    pub fn from_id(id: &str) -> Option<Self> {\n        match id {\n");
    for (id, _) in pairs {
        out.push_str(&format!(
            "            {} => Some(Self::{}),\n",
            rust_string(id),
            pascal_case(id)
        ));
    }
    out.push_str("            _ => None,\n        }\n    }\n}\n\n");
}

fn rust_string(value: &str) -> String {
    format!("{value:?}")
}
