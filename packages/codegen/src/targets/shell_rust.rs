//! The shell-scene Rust target — the authored shell scene as a committed,
//! self-contained Rust artifact inside the consuming native previews
//! (card 036 R1: no `poodle-ir` / `poodle-codegen` dependency in either
//! native manifest — the artifact is plain data with no `use` of any Poodle
//! crate, pulled in by the hosts via `#[path = "generated/preview-shell.rs"]`,
//! the `poodle-tokens` mechanism g13-b003 R1 names).
//!
//! Output (per model, under the target's output root `generated`):
//!
//! - `<scene-id>.rs` — the scene as a `pub static`: the control surface
//!   (axes with display labels and values, search when configured),
//!   navigation layout, specimen tabs, preview-state defaults, and the
//!   parity-harness vocabulary. Scenes sort by id; one file per scene.
//!
//! # R2 — a sibling target, not a repurposed one
//!
//! Card 036 R2: `shell-scene` hard-codes `format!("{}.ts", …)` and b035's
//! tests byte-compare both committed web artifacts against its render, so
//! its output is frozen. This target renders the **same scene** in Rust
//! shape; one authored change still moves every shell in one `ir:build` —
//! four artifacts now, not two.
//!
//! The label projection is shared with `shell-scene` via
//! [`super::shell::axis_label`] / [`super::shell::kind_name`] /
//! [`super::shell::humanize`] — exactly one home for the label text.

use poodle_ir::{AxisValues, IrModel, Scene};

use crate::emit::{header, sort_by_id, EmitTarget, GeneratedFile};
use crate::error::Result;

use super::shell::{axis_label, humanize, kind_name};
use super::ts::format_number;

/// The shell-scene Rust target. Scoped like `shell-scene`: not in
/// [`super::all`], so a plain `ir:build` over the synthetic fixture never
/// writes into a native package; reachable via `--target shell-rust`.
pub struct ShellRustTarget;

impl EmitTarget for ShellRustTarget {
    fn id(&self) -> &'static str {
        "shell-rust"
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
            GeneratedFile::new(format!("{}.rs", scene.id.as_str()), contents)
        })
        .collect()
}

/// The struct definitions every artifact carries. Emitted verbatim so each
/// artifact is self-contained — the hosts must not import anything to read
/// the scene. `dead_code` is allowed by design: the artifact is the scene,
/// and a host consumes the subset it renders; the field set is the scene's,
/// not the consumer's.
const STRUCT_PRELUDE: &str = r#"#![allow(dead_code)]

//! The authored shell scene (spec 063 "Generated Artifact Contract"):
//! plain data, self-contained, no Poodle crate imports. Pulled into the
//! host previews via `#[path = "generated/preview-shell.rs"]` — the
//! `poodle-tokens` mechanism (g13-b003 R1). Regenerate with
//! `effigy ir:build`; drift is gated by `effigy ir:check`.

/// One control on the shared preview shell — an axis or the search field.
/// `kind` is the machine key shells discriminate on ("theme", "size",
/// "density", "contrast", "search"); the label is the scene's projection
/// (card 035 R4 — a shell may style it, not author different text).
pub struct ShellControl {
    pub kind: &'static str,
    pub label: &'static str,
    /// Named axis values; `None` on the continuous contrast axis and search.
    pub values: Option<&'static [&'static str]>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub default_value: Option<f64>,
    /// The search field's placeholder; `None` on axes.
    pub placeholder: Option<&'static str>,
}

/// A specimen tab: machine id and display label.
pub struct ShellTab {
    pub id: &'static str,
    pub label: &'static str,
}

/// One navigation section of the shell layout.
pub struct ShellLayoutSection {
    pub title: &'static str,
    pub kind: &'static str,
}

/// The shell layout: navigation sections and URL-persisted state keys.
pub struct ShellLayout {
    pub sections: &'static [ShellLayoutSection],
    pub persisted: &'static [&'static str],
}

/// Preview-state defaults the scene carries.
pub struct ShellPreviewState {
    pub theme: Option<&'static str>,
    pub density: Option<&'static str>,
    pub control_size: Option<&'static str>,
    pub contrast: Option<f64>,
}

/// One parity-harness visual gate.
pub struct ShellVisualGate {
    pub tier: &'static str,
    pub axes: &'static [&'static str],
}

/// The parity-harness vocabulary (SHELL-09).
pub struct ShellParity {
    pub defaults: ShellPreviewState,
    pub review_route_presets: &'static [&'static str],
    pub targets: &'static [&'static str],
    pub visual_gates: &'static [ShellVisualGate],
    pub native_visual_baseline: bool,
}

/// The authored shell scene — plain data, no Poodle crate imports.
pub struct ShellScene {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub controls: &'static [ShellControl],
    pub tabs: &'static [ShellTab],
    pub layout: Option<ShellLayout>,
    pub preview_state: Option<ShellPreviewState>,
    pub parity: Option<ShellParity>,
    pub captures: &'static [&'static str],
}

"#;

/// A Rust string literal. JSON escaping is not valid Rust (`\u00XX` and
/// `\b`/`\f` are JSON-isms), so this is its own function — not a reuse of
/// the TS target's.
fn rust_string_literal(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            ch if (ch as u32) < 0x20 => {
                out.push_str(&format!("\\u{{{:X}}}", ch as u32));
            }
            ch => out.push(ch),
        }
    }
    out.push('"');
    out
}

/// `preview-shell` → `PREVIEW_SHELL`. The artifact's static name; scenes
/// get one file each, so names cannot collide.
fn static_name(scene_id: &str) -> String {
    scene_id
        .split(['-', '_'])
        .filter(|part| !part.is_empty())
        .map(|part| part.to_uppercase())
        .collect::<Vec<_>>()
        .join("_")
}

/// One scene's control entries, in authored axis order, followed by the
/// search entry when the scene configures search. The hosts iterate this
/// list; deleting an axis or search from the scene removes the control from
/// every consuming shell (card 035 R3, repeated for the natives in 036 R4).
fn render_controls(scene: &Scene) -> String {
    let mut entries: Vec<String> = scene
        .axes
        .iter()
        .map(|axis| {
            let kind = axis.kind;
            let label = rust_string_literal(axis_label(kind));
            match &axis.values {
                AxisValues::Named(values) => {
                    let values = values
                        .iter()
                        .map(|value| rust_string_literal(value.as_str()))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!(
                        "        ShellControl {{ kind: {}, label: {}, values: Some(&[{values}]), min: None, max: None, default_value: None, placeholder: None }},",
                        rust_string_literal(kind_name(kind)),
                        label
                    )
                }
                AxisValues::Continuous { min, max, default } => {
                    format!(
                        "        ShellControl {{ kind: {}, label: {}, values: None, min: Some({}), max: Some({}), default_value: Some({}), placeholder: None }},",
                        rust_string_literal(kind_name(kind)),
                        label,
                        format_number(*min),
                        format_number(*max),
                        format_number(*default)
                    )
                }
            }
        })
        .collect();

    if scene.search.is_some() {
        entries.push(format!(
            "        ShellControl {{ kind: \"search\", label: {}, values: None, min: None, max: None, default_value: None, placeholder: Some({}) }},",
            rust_string_literal("Search"),
            rust_string_literal("Find component...")
        ));
    }

    entries.join("\n")
}

/// An optional `&str` field as `Some("…")` / `None`.
fn option_str(value: Option<&str>) -> String {
    match value {
        Some(value) => format!("Some({})", rust_string_literal(value)),
        None => "None".to_owned(),
    }
}

/// An optional `f64` field as `Some(0.5)` / `None`.
fn option_number(value: Option<f64>) -> String {
    match value {
        Some(value) => format!("Some({})", format_number(value)),
        None => "None".to_owned(),
    }
}

fn render_scene_file(scene: &Scene, source_path: &str) -> String {
    let mut out = header(source_path);
    out.push_str(STRUCT_PRELUDE);

    let static_name = static_name(scene.id.as_str());
    out.push_str(&format!(
        "pub static {static_name}: ShellScene = ShellScene {{\n"
    ));
    out.push_str(&format!(
        "    id: {},\n",
        rust_string_literal(scene.id.as_str())
    ));
    out.push_str(&format!(
        "    name: {},\n",
        rust_string_literal(&scene.name)
    ));
    out.push_str(&format!(
        "    description: {},\n",
        rust_string_literal(&scene.description)
    ));
    out.push_str("    controls: &[\n");
    out.push_str(&render_controls(scene));
    out.push_str("\n    ],\n");

    out.push_str("    tabs: &[\n");
    if let Some(tabs) = &scene.tabs {
        for tab in &tabs.tabs {
            out.push_str(&format!(
                "        ShellTab {{ id: {}, label: {} }},\n",
                rust_string_literal(tab.as_str()),
                rust_string_literal(&humanize(tab.as_str()))
            ));
        }
    }
    out.push_str("    ],\n");

    out.push_str("    layout: ");
    match &scene.layout {
        Some(layout) => {
            out.push_str("Some(ShellLayout {\n        sections: &[\n");
            for section in &layout.sections {
                out.push_str(&format!(
                    "            ShellLayoutSection {{ title: {}, kind: {} }},\n",
                    rust_string_literal(&section.title),
                    rust_string_literal(match section.kind {
                        poodle_ir::NavSectionKind::Components => "components",
                        poodle_ir::NavSectionKind::Tokens => "tokens",
                        poodle_ir::NavSectionKind::Demo => "demo",
                    })
                ));
            }
            out.push_str("        ],\n        persisted: &[");
            let persisted = layout
                .route_state
                .persisted
                .iter()
                .map(|key| rust_string_literal(key))
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&persisted);
            out.push_str("],\n    }),\n");
        }
        None => out.push_str("None,\n"),
    }

    out.push_str("    preview_state: ");
    match &scene.preview_state {
        Some(state) => {
            out.push_str("Some(ShellPreviewState {\n");
            out.push_str(&format!(
                "        theme: {},\n",
                option_str(state.theme.as_ref().map(|value| value.as_str()))
            ));
            out.push_str(&format!(
                "        density: {},\n",
                option_str(state.density.as_ref().map(|value| value.as_str()))
            ));
            out.push_str(&format!(
                "        control_size: {},\n",
                option_str(state.control_size.as_ref().map(|value| value.as_str()))
            ));
            out.push_str(&format!(
                "        contrast: {},\n    }}),\n",
                option_number(state.contrast)
            ));
        }
        None => out.push_str("None,\n"),
    }

    out.push_str("    parity: ");
    match &scene.parity {
        Some(parity) => {
            out.push_str("Some(ShellParity {\n");
            out.push_str("        defaults: ShellPreviewState {\n");
            out.push_str(&format!(
                "            theme: {},\n",
                option_str(parity.defaults.theme.as_ref().map(|value| value.as_str()))
            ));
            out.push_str(&format!(
                "            density: {},\n",
                option_str(parity.defaults.density.as_ref().map(|value| value.as_str()))
            ));
            out.push_str(&format!(
                "            control_size: {},\n",
                option_str(
                    parity
                        .defaults
                        .control_size
                        .as_ref()
                        .map(|value| value.as_str())
                )
            ));
            out.push_str(&format!(
                "            contrast: {},\n        }},\n",
                option_number(parity.defaults.contrast)
            ));
            out.push_str("        review_route_presets: &[");
            let presets = parity
                .review_route_presets
                .iter()
                .map(|id| rust_string_literal(id.as_str()))
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&presets);
            out.push_str("],\n");
            out.push_str("        targets: &[");
            let targets = parity
                .targets
                .iter()
                .map(|target| {
                    rust_string_literal(match target {
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
            out.push_str("        visual_gates: &[\n");
            for gate in &parity.visual_gates {
                let tier = rust_string_literal(match gate.tier {
                    poodle_ir::GateTier::Smoke => "smoke",
                    poodle_ir::GateTier::Axis => "axis",
                    poodle_ir::GateTier::Sweep => "sweep",
                });
                let axes = gate
                    .axes
                    .iter()
                    .map(|kind| rust_string_literal(kind_name(*kind)))
                    .collect::<Vec<_>>()
                    .join(", ");
                out.push_str(&format!(
                    "            ShellVisualGate {{ tier: {tier}, axes: &[{axes}] }},\n"
                ));
            }
            out.push_str("        ],\n");
            out.push_str(&format!(
                "        native_visual_baseline: {},\n    }}),\n",
                parity.native_visual_baseline
            ));
        }
        None => out.push_str("None,\n"),
    }

    out.push_str("    captures: &[");
    let captures = scene
        .captures
        .iter()
        .map(|id| rust_string_literal(id.as_str()))
        .collect::<Vec<_>>()
        .join(", ");
    out.push_str(&captures);
    out.push_str("],\n};\n");

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_name_upper_snakes_the_scene_id() {
        assert_eq!(static_name("preview-shell"), "PREVIEW_SHELL");
        assert_eq!(static_name("shell"), "SHELL");
    }

    #[test]
    fn rust_string_literal_escapes_rust_style() {
        assert_eq!(rust_string_literal("plain"), "\"plain\"");
        assert_eq!(rust_string_literal("a\"b\\c"), "\"a\\\"b\\\\c\"");
        assert_eq!(rust_string_literal("line\nbreak"), "\"line\\nbreak\"");
        // A control character escapes as `\u{..}`, which Rust accepts —
        // JSON's `\u00XX` form would not compile.
        assert_eq!(rust_string_literal("bell\u{7}"), "\"bell\\u{7}\"");
    }
}
