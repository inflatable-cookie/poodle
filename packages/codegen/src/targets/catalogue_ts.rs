//! TypeScript preview-catalogue target (g14.025).
//!
//! Select-only: not in [`super::all`], reachable only via `--catalogue` +
//! `--target catalogue-ts`. Renders one module under `generated/catalogue/`.

use crate::catalogue::{components_in_order, Document};
use crate::emit::{catalogue_header, GeneratedFile};
use super::ts::ts_string_literal;

/// Target id accepted by `--target` in catalogue mode.
pub const ID: &str = "catalogue-ts";

/// Output root relative to `--out` (the consuming package's `src/`).
pub const OUTPUT_ROOT: &str = "generated/catalogue";

/// Renders the canonical TypeScript catalogue module.
pub fn render(document: &Document, source_path: &str) -> Vec<GeneratedFile> {
    vec![GeneratedFile::new(
        "catalogue.ts",
        render_module(document, source_path),
    )]
}

fn render_module(document: &Document, source_path: &str) -> String {
    let mut out = catalogue_header(source_path, document.schema_version);
    out.push('\n');

    out.push_str(&format!(
        "export type CatalogueSectionId = {};\n",
        union_of(document.sections.iter().map(|entry| entry.id.as_str()))
    ));
    out.push_str(&format!(
        "export type CatalogueFamilyId = {};\n",
        union_of(document.families.iter().map(|entry| entry.id.as_str()))
    ));
    out.push_str(&format!(
        "export type CatalogueKindId = {};\n",
        union_of(document.kinds.iter().map(|entry| entry.id.as_str()))
    ));
    out.push_str(&format!(
        "export type CatalogueCollectionId = {};\n\n",
        if document.collections.is_empty() {
            "never".to_owned()
        } else {
            union_of(document.collections.iter().map(|entry| entry.id.as_str()))
        }
    ));

    out.push_str(
        "export type CatalogueSection = {\n\
         \treadonly id: CatalogueSectionId;\n\
         \treadonly label: string;\n\
         };\n\n\
         export type CatalogueFamily = {\n\
         \treadonly id: CatalogueFamilyId;\n\
         \treadonly section: CatalogueSectionId;\n\
         \treadonly label: string;\n\
         };\n\n\
         export type CatalogueKind = {\n\
         \treadonly id: CatalogueKindId;\n\
         \treadonly label: string;\n\
         };\n\n\
         export type CatalogueCollection = {\n\
         \treadonly id: CatalogueCollectionId;\n\
         \treadonly label: string;\n\
         };\n\n\
         export type CanonicalComponent = {\n\
         \treadonly slug: string;\n\
         \treadonly displayName: string;\n\
         \treadonly description: string;\n\
         \treadonly section: CatalogueSectionId;\n\
         \treadonly family: CatalogueFamilyId;\n\
         \treadonly kind: CatalogueKindId;\n\
         \treadonly collections: readonly CatalogueCollectionId[];\n\
         };\n\n",
    );

    out.push_str("export const catalogueSections = [\n");
    for section in &document.sections {
        out.push_str(&format!(
            "\t{{ id: {}, label: {} }},\n",
            ts_string_literal(&section.id),
            ts_string_literal(&section.label)
        ));
    }
    out.push_str("] as const satisfies readonly CatalogueSection[];\n\n");

    out.push_str("export const catalogueFamilies = [\n");
    for family in &document.families {
        out.push_str(&format!(
            "\t{{ id: {}, section: {}, label: {} }},\n",
            ts_string_literal(&family.id),
            ts_string_literal(&family.section),
            ts_string_literal(&family.label)
        ));
    }
    out.push_str("] as const satisfies readonly CatalogueFamily[];\n\n");

    out.push_str("export const catalogueKinds = [\n");
    for kind in &document.kinds {
        out.push_str(&format!(
            "\t{{ id: {}, label: {} }},\n",
            ts_string_literal(&kind.id),
            ts_string_literal(&kind.label)
        ));
    }
    out.push_str("] as const satisfies readonly CatalogueKind[];\n\n");

    if document.collections.is_empty() {
        out.push_str("export const catalogueCollections = [] as const satisfies readonly CatalogueCollection[];\n\n");
    } else {
        out.push_str("export const catalogueCollections = [\n");
        for collection in &document.collections {
            out.push_str(&format!(
                "\t{{ id: {}, label: {} }},\n",
                ts_string_literal(&collection.id),
                ts_string_literal(&collection.label)
            ));
        }
        out.push_str("] as const satisfies readonly CatalogueCollection[];\n\n");
    }

    out.push_str("export const canonicalComponents: readonly CanonicalComponent[] = [\n");
    for component in components_in_order(document) {
        let collections = if component.collections.is_empty() {
            "[] as const satisfies readonly CatalogueCollectionId[]".to_owned()
        } else {
            format!(
                "[{}]",
                component
                    .collections
                    .iter()
                    .map(|id| ts_string_literal(id))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        out.push_str(&format!(
            "\t{{\n\
             \t\tslug: {},\n\
             \t\tdisplayName: {},\n\
             \t\tdescription: {},\n\
             \t\tsection: {},\n\
             \t\tfamily: {},\n\
             \t\tkind: {},\n\
             \t\tcollections: {},\n\
             \t}},\n",
            ts_string_literal(&component.slug),
            ts_string_literal(&component.display_name),
            ts_string_literal(&component.description),
            ts_string_literal(&component.section),
            ts_string_literal(&component.family),
            ts_string_literal(&component.kind),
            collections
        ));
    }
    out.push_str("];\n\n");

    out.push_str(
        "export function findCanonicalComponent(slug: string): CanonicalComponent | undefined {\n\
         \treturn canonicalComponents.find((component) => component.slug === slug);\n\
         }\n",
    );

    out
}

fn union_of<'a>(ids: impl Iterator<Item = &'a str>) -> String {
    ids.map(ts_string_literal).collect::<Vec<_>>().join(" | ")
}
