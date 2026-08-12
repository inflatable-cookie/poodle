//! The shell-scene target — the authored shell scene as a committed
//! TypeScript artifact inside the consuming web packages (g13-b003 R1
//! "Generated output location": generated TypeScript lands under a
//! `generated/` directory in the consuming package, mirroring
//! `packages/core/src/tokens/generated/`).
//!
//! Output (per model, under the target's output root `generated`):
//!
//! - `<scene-id>.ts` — the scene as a typed readonly constant: the control
//!   surface (axes with display labels and values, search when configured),
//!   navigation layout, specimen tabs, preview-state defaults, and the
//!   parity-harness vocabulary. Scenes sort by id; one file per scene.
//!
//! # R4 — labels are a projection of the scene
//!
//! Card 035's R4: one scene supplies the label text; a shell may style it
//! (uppercase via CSS or the native equivalent) but must not author
//! different text. The scene IR carries no label field — the *capability*
//! is modeled (`SceneAxis.kind`, `search` presence), and the label text is
//! this emitter's deterministic projection, authored exactly once here:
//!
//! - an axis's label is the display name of its [`SceneAxisKind`];
//! - search's label and placeholder appear only when the scene configures
//!   search;
//! - a tab's label is the display form of its identifier.
//!
//! Both web shells consume the artifact verbatim, so they cannot drift on
//! capability set or label text — the failure this card exists to prevent.

use poodle_ir::{AxisValues, IrModel, Scene, SceneAxisKind, SearchField};

use crate::emit::{header, sort_by_id, EmitTarget, GeneratedFile};
use crate::error::Result;

use super::ts::{format_number, ts_string_literal};

/// The shell-scene target. Scoped to the shell model: not in
/// [`super::all`], so a plain `ir:build` over the synthetic fixture never
/// writes into a web package; reachable via `--target shell-scene`.
pub struct ShellSceneTarget;

impl EmitTarget for ShellSceneTarget {
    fn id(&self) -> &'static str {
        "shell-scene"
    }

    fn output_root(&self) -> &'static str {
        "generated"
    }

    fn render(&self, model: &IrModel, source_path: &str) -> Result<Vec<GeneratedFile>> {
        Ok(render_scenes(model, source_path))
    }
}

/// Renders one file per scene, sorted by id. Public for tests; the bin goes
/// through [`EmitTarget::render`].
pub fn render_scenes(model: &IrModel, source_path: &str) -> Vec<GeneratedFile> {
    let mut scenes = model.scenes.clone();
    sort_by_id(&mut scenes, |scene| scene.id.as_str());

    scenes
        .into_iter()
        .map(|scene| {
            let contents = render_scene_file(&scene, source_path);
            GeneratedFile::new(format!("{}.ts", scene.id.as_str()), contents)
        })
        .collect()
}

/// The display label of an axis kind — the scene's label text for the
/// control (card 035 R4). Casing here is content; a shell may restyle it.
fn axis_label(kind: SceneAxisKind) -> &'static str {
    match kind {
        SceneAxisKind::Theme => "Theme",
        SceneAxisKind::Size => "Size",
        SceneAxisKind::Density => "Density",
        SceneAxisKind::Orientation => "Orientation",
        SceneAxisKind::Contrast => "Contrast",
    }
}

/// Display form of an identifier, the `theme-options.ts` rule: split on
/// `-`/`_`, capitalize each word, join with a space (`examples` →
/// `Examples`, `high-contrast` → `High Contrast`).
fn humanize(value: &str) -> String {
    value
        .split(['-', '_'])
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// One scene's control entries, in authored axis order, followed by the
/// search entry when the scene configures search. The shells iterate this
/// list; deleting an axis or search from the scene removes the control from
/// every consuming shell (card 035 R3).
fn render_controls(scene: &Scene) -> String {
    let mut entries: Vec<String> = scene
        .axes
        .iter()
        .map(|axis| {
            let kind = axis.kind;
            let label = ts_string_literal(axis_label(kind));
            match &axis.values {
                AxisValues::Named(values) => {
                    let values = values
                        .iter()
                        .map(|value| ts_string_literal(value.as_str()))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!(
                        "    {{ kind: {}, label: {}, values: [{values}] }}",
                        ts_string_literal(kind_name(kind)),
                        label
                    )
                }
                AxisValues::Continuous { min, max, default } => {
                    format!(
                        "    {{ kind: {}, label: {}, min: {}, max: {}, default: {} }}",
                        ts_string_literal(kind_name(kind)),
                        label,
                        format_number(*min),
                        format_number(*max),
                        format_number(*default)
                    )
                }
            }
        })
        .collect();

    if let Some(search) = &scene.search {
        let fields = search
            .fields
            .iter()
            .map(|field| {
                ts_string_literal(match field {
                    SearchField::DisplayName => "display-name",
                    SearchField::Description => "description",
                })
            })
            .collect::<Vec<_>>()
            .join(", ");
        entries.push(format!(
            "    {{ kind: \"search\", label: {}, placeholder: {}, caseInsensitive: {}, fields: [{fields}] }}",
            ts_string_literal("Search"),
            ts_string_literal("Find component..."),
            search.case_insensitive
        ));
    }

    entries.join(",\n")
}

/// The serialized scene-kind name (the IR's serde rename), used as the
/// control's machine key. The shell discriminates widgets on it.
fn kind_name(kind: SceneAxisKind) -> &'static str {
    match kind {
        SceneAxisKind::Theme => "theme",
        SceneAxisKind::Size => "size",
        SceneAxisKind::Density => "density",
        SceneAxisKind::Orientation => "orientation",
        SceneAxisKind::Contrast => "contrast",
    }
}

fn render_scene_file(scene: &Scene, source_path: &str) -> String {
    let mut out = header(source_path);

    let export_name = camel_case(scene.id.as_str());

    out.push_str(&format!("export const {export_name} = {{\n"));
    out.push_str(&format!(
        "  id: {},\n",
        ts_string_literal(scene.id.as_str())
    ));
    out.push_str(&format!("  name: {},\n", ts_string_literal(&scene.name)));
    out.push_str(&format!(
        "  description: {},\n",
        ts_string_literal(&scene.description)
    ));
    out.push_str("  controls: [\n");
    out.push_str(&render_controls(scene));
    out.push_str("\n  ],\n");

    out.push_str("  tabs: [\n");
    if let Some(tabs) = &scene.tabs {
        for tab in &tabs.tabs {
            out.push_str(&format!(
                "    {{ id: {}, label: {} }},\n",
                ts_string_literal(tab.as_str()),
                ts_string_literal(&humanize(tab.as_str()))
            ));
        }
    }
    out.push_str("  ],\n");

    out.push_str("  layout: ");
    match &scene.layout {
        Some(layout) => {
            out.push_str("{\n    sections: [\n");
            for section in &layout.sections {
                out.push_str(&format!(
                    "      {{ title: {}, kind: {} }},\n",
                    ts_string_literal(&section.title),
                    ts_string_literal(match section.kind {
                        poodle_ir::NavSectionKind::Components => "components",
                        poodle_ir::NavSectionKind::Tokens => "tokens",
                        poodle_ir::NavSectionKind::Demo => "demo",
                    })
                ));
            }
            out.push_str("    ],\n");
            out.push_str("    persisted: [");
            let persisted = layout
                .route_state
                .persisted
                .iter()
                .map(|key| ts_string_literal(key))
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&persisted);
            out.push_str("],\n  },\n");
        }
        None => out.push_str("null,\n"),
    }

    out.push_str("  previewState: ");
    match &scene.preview_state {
        Some(state) => {
            out.push_str("{\n");
            out.push_str(&format!(
                "    theme: {},\n",
                state
                    .theme
                    .as_ref()
                    .map(|value| ts_string_literal(value.as_str()))
                    .unwrap_or_else(|| "null".to_owned())
            ));
            out.push_str(&format!(
                "    density: {},\n",
                state
                    .density
                    .as_ref()
                    .map(|value| ts_string_literal(value.as_str()))
                    .unwrap_or_else(|| "null".to_owned())
            ));
            out.push_str(&format!(
                "    controlSize: {},\n",
                state
                    .control_size
                    .as_ref()
                    .map(|value| ts_string_literal(value.as_str()))
                    .unwrap_or_else(|| "null".to_owned())
            ));
            out.push_str(&format!(
                "    contrast: {},\n  }},\n",
                state
                    .contrast
                    .map(format_number)
                    .unwrap_or_else(|| "null".to_owned())
            ));
        }
        None => out.push_str("null,\n"),
    }

    out.push_str("  parity: ");
    match &scene.parity {
        Some(parity) => {
            out.push_str("{\n    defaults: ");
            out.push_str(&render_preview_state_literal(&parity.defaults));
            out.push_str("    reviewRoutePresets: [");
            let presets = parity
                .review_route_presets
                .iter()
                .map(|id| ts_string_literal(id.as_str()))
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&presets);
            out.push_str("],\n");
            out.push_str("    targets: [");
            let targets = parity
                .targets
                .iter()
                .map(|target| {
                    ts_string_literal(match target {
                        poodle_ir::RuntimeTarget::Svelte => "svelte",
                        poodle_ir::RuntimeTarget::React => "react",
                        poodle_ir::RuntimeTarget::Gpui => "gpui",
                        poodle_ir::RuntimeTarget::Jetstream => "jetstream",
                    })
                })
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&targets);
            out.push_str("],\n");
            out.push_str("    visualGates: [\n");
            for gate in &parity.visual_gates {
                let tier = ts_string_literal(match gate.tier {
                    poodle_ir::GateTier::Smoke => "smoke",
                    poodle_ir::GateTier::Axis => "axis",
                    poodle_ir::GateTier::Sweep => "sweep",
                });
                let axes = gate
                    .axes
                    .iter()
                    .map(|kind| ts_string_literal(kind_name(*kind)))
                    .collect::<Vec<_>>()
                    .join(", ");
                out.push_str(&format!("      {{ tier: {tier}, axes: [{axes}] }},\n"));
            }
            out.push_str("    ],\n");
            out.push_str(&format!(
                "    nativeVisualBaseline: {},\n  }},\n",
                parity.native_visual_baseline
            ));
        }
        None => out.push_str("null,\n"),
    }

    out.push_str("  captures: [");
    let captures = scene
        .captures
        .iter()
        .map(|id| ts_string_literal(id.as_str()))
        .collect::<Vec<_>>()
        .join(", ");
    out.push_str(&captures);
    out.push_str("],\n} as const;\n");

    out
}

/// The preview state as an inline object literal under a `defaults:` key,
/// self-contained: opens `{`, closes `},\n` so the caller's next key line
/// continues the enclosing `parity` object.
fn render_preview_state_literal(state: &poodle_ir::PreviewState) -> String {
    format!(
        "{{\n      theme: {},\n      density: {},\n      controlSize: {},\n      contrast: {},\n    }},\n",
        state
            .theme
            .as_ref()
            .map(|value| ts_string_literal(value.as_str()))
            .unwrap_or_else(|| "null".to_owned()),
        state
            .density
            .as_ref()
            .map(|value| ts_string_literal(value.as_str()))
            .unwrap_or_else(|| "null".to_owned()),
        state
            .control_size
            .as_ref()
            .map(|value| ts_string_literal(value.as_str()))
            .unwrap_or_else(|| "null".to_owned()),
        state
            .contrast
            .map(format_number)
            .unwrap_or_else(|| "null".to_owned())
    )
}

/// `preview-shell` → `previewShell`. The artifact's export name; scenes get
/// one file each, so names cannot collide.
fn camel_case(value: &str) -> String {
    let mut out = String::new();
    let mut capitalize = false;
    for ch in value.chars() {
        if ch == '-' || ch == '_' {
            capitalize = true;
        } else if capitalize {
            out.extend(ch.to_uppercase());
            capitalize = false;
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
    fn humanize_matches_the_web_rule() {
        assert_eq!(humanize("examples"), "Examples");
        assert_eq!(humanize("high-contrast"), "High Contrast");
        assert_eq!(humanize("sizes"), "Sizes");
    }

    #[test]
    fn camel_case_joins_dash_segments() {
        assert_eq!(camel_case("preview-shell"), "previewShell");
        assert_eq!(camel_case("shell"), "shell");
    }
}
