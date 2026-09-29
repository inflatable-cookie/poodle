//! The specimen TypeScript target — the display-specimen scenes as
//! committed TypeScript artifacts inside the consuming web packages
//! (g13-b003 R1 "Generated output location": generated TypeScript lands
//! under a `generated/` directory in the consuming package).
//!
//! Output (per model, under the target's output root `generated`):
//!
//! - `<scene-id>.ts` — one scene as a typed readonly constant: the
//!   specimen's groups (the `group` field added by `g14-b005`), each with
//!   its instances' typed prop bindings, the matrix axes, and the tabs.
//! - `specimen-scenes.ts` — the slug registry mapping each scene's primary
//!   component (its first instance's component) to the scene constant, so
//!   the web specimen renderers look scenes up by slug.
//!
//! The content `content` binding is data like any other; the web renderer
//! projects it to children (see the SceneSpecimen consumers). No behaviour
//! is emitted.

use poodle_ir::{IrModel, Scene, Value};

use crate::emit::{header, sort_by_id, EmitTarget, GeneratedFile};
use crate::error::Result;

use super::shell::camel_case;
use super::ts::format_number;

/// The specimen target. Scoped like `shell-scene`: not in [`super::all`],
/// so a plain `ir:build` over the synthetic fixture never writes into a web
/// package; reachable via `--target specimen-ts`.
pub struct SpecimenTsTarget;

impl EmitTarget for SpecimenTsTarget {
    fn id(&self) -> &'static str {
        "specimen-ts"
    }

    fn output_root(&self) -> &'static str {
        "generated/specimens"
    }

    fn render(&self, model: &IrModel, source_path: &str) -> Result<Vec<GeneratedFile>> {
        Ok(render_scenes(model, source_path))
    }
}

/// The value serialized as the TS prop literal.
fn value_literal(value: &Value) -> String {
    match value {
        Value::String(s) => ts_string_literal(s),
        Value::Number(n) => format_number(*n),
        Value::Bool(b) => b.to_string(),
        Value::Member(id) => ts_string_literal(id.as_str()),
        Value::Null => "null".to_owned(),
        Value::Pair(a, b) => format!("[{}, {}]", value_literal(a), value_literal(b)),
        Value::List(items) => format!(
            "[{}]",
            items.iter().map(value_literal).collect::<Vec<_>>().join(", ")
        ),
    }
}

/// The scene's primary component — the slug the specimen renders under.
/// Every tranche-one scene binds one component; the first instance names it.
fn primary_component(scene: &Scene) -> &str {
    scene
        .instances
        .first()
        .map(|instance| instance.component.as_str())
        .unwrap_or("")
}

fn ts_string_literal(value: &str) -> String {
    super::ts::ts_string_literal(value)
}

/// The slug registry, sorted by slug so the artifact is deterministic.
fn render_registry(scenes: &[Scene], source_path: &str) -> String {
    let mut out = header(source_path);
    let mut entries = Vec::new();
    for scene in scenes {
        let slug = primary_component(scene);
        if slug.is_empty() {
            continue;
        }
        out.push_str(&format!(
            "import {{ {} }} from \"./{}\";\n",
            camel_case(scene.id.as_str()),
            scene.id.as_str()
        ));
        entries.push((slug, camel_case(scene.id.as_str())));
    }
    out.push_str("\nexport const specimenScenes = {\n");
    for (slug, export) in entries {
        out.push_str(&format!("  {}: {export},\n", js_key(slug)));
    }
    out.push_str("} as const;\n");
    out
}

/// Object key form for a slug (`empty-state` → `"empty-state"`).
fn js_key(slug: &str) -> String {
    if slug.chars().all(|c| c.is_alphanumeric() || c == '_') {
        slug.to_owned()
    } else {
        ts_string_literal(slug)
    }
}

/// Renders one file per scene (sorted by id) plus the slug registry.
/// Public for tests; the bin goes through [`EmitTarget::render`].
pub fn render_scenes(model: &IrModel, source_path: &str) -> Vec<GeneratedFile> {
    let mut scenes = model.scenes.clone();
    sort_by_id(&mut scenes, |scene| scene.id.as_str());

    let mut files: Vec<GeneratedFile> = scenes
        .iter()
        .map(|scene| {
            let contents = render_scene_file(scene, source_path);
            GeneratedFile::new(format!("{}.ts", scene.id.as_str()), contents)
        })
        .collect();

    let registry = render_registry(&scenes, source_path);
    files.push(GeneratedFile::new("specimen-scenes.ts".to_owned(), registry));
    files
}

fn render_axis(values: &[poodle_ir::Identifier]) -> String {
    values
        .iter()
        .map(|value| ts_string_literal(value.as_str()))
        .collect::<Vec<_>>()
        .join(", ")
}

fn render_scene_file(scene: &Scene, source_path: &str) -> String {
    let mut out = header(source_path);

    let size_axis = scene
        .axes
        .iter()
        .find(|axis| axis.kind == poodle_ir::SceneAxisKind::Size);
    let density_axis = scene
        .axes
        .iter()
        .find(|axis| axis.kind == poodle_ir::SceneAxisKind::Density);

    out.push_str(&format!("export const {} = {{\n", camel_case(scene.id.as_str())));
    out.push_str(&format!("  id: {},\n", ts_string_literal(scene.id.as_str())));
    out.push_str(&format!("  name: {},\n", ts_string_literal(&scene.name)));
    out.push_str(&format!(
        "  description: {},\n",
        ts_string_literal(&scene.description)
    ));

    out.push_str("  tabs: [");
    let tabs = scene
        .tabs
        .as_ref()
        .map(|tabs| {
            tabs.tabs
                .iter()
                .map(|tab| ts_string_literal(tab.as_str()))
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_default();
    out.push_str(&tabs);
    out.push_str("],\n");

    out.push_str("  sizeAxis: [");
    if let Some(axis) = size_axis {
        if let poodle_ir::AxisValues::Named(values) = &axis.values {
            out.push_str(&render_axis(values));
        }
    }
    out.push_str("],\n");

    out.push_str("  densityAxis: [");
    if let Some(axis) = density_axis {
        if let poodle_ir::AxisValues::Named(values) = &axis.values {
            out.push_str(&render_axis(values));
        }
    }
    out.push_str("],\n");

    out.push_str("  groups: [\n");
    let mut current_group: Option<&str> = None;
    let mut first = true;
    for instance in &scene.instances {
        let group = instance.group.as_deref().unwrap_or("");
        if current_group != Some(group) {
            if !first {
                out.push_str("    ],\n    },\n");
            }
            first = false;
            out.push_str(&format!(
                "    {{ label: {}, instances: [\n",
                ts_string_literal(group)
            ));
            current_group = Some(group);
        }
        out.push_str(&render_instance(instance));
    }
    if !first {
        out.push_str("    ],\n    },\n");
    }
    out.push_str("  ],\n} as const;\n");

    out
}

fn render_instance(instance: &poodle_ir::ComponentInstance) -> String {
    let mut out = String::new();
    out.push_str("      {\n");
    out.push_str(&format!(
        "        component: {},\n",
        ts_string_literal(instance.component.as_str())
    ));
    if let Some(caption) = &instance.caption {
        out.push_str(&format!("        caption: {},\n", ts_string_literal(caption)));
    }
    out.push_str("        props: {\n");
    for binding in &instance.bindings {
        out.push_str(&format!(
            "          {}: {},\n",
            js_key(binding.prop.as_str()),
            value_literal(&binding.value)
        ));
    }
    out.push_str("        },\n");
    out.push_str("      },\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn js_key_quotes_hyphenated_slugs() {
        assert_eq!(js_key("callout"), "callout");
        assert_eq!(js_key("empty-state"), "\"empty-state\"");
    }

    #[test]
    fn value_literals_match_the_ts_shapes() {
        assert_eq!(value_literal(&Value::string("x")), "\"x\"");
        assert_eq!(value_literal(&Value::boolean(true)), "true");
        assert_eq!(value_literal(&Value::number(1.5)), "1.5");
        assert_eq!(value_literal(&Value::member("danger")), "\"danger\"");
        assert_eq!(value_literal(&Value::Null), "null");
    }
}
