//! The authored shell scene — the `g13.004` preview shell defined once in
//! Rust (spec 063 "Authoring Form": ordinary Rust types and constructor
//! helpers, no macros), serialized to the JSON fixture the existing
//! pipeline consumes (`ir:build` / `ir:check` via `load_and_validate`), and
//! emitted to both web shells through the `shell-scene` target.
//!
//! # Placement — pilot-scoped
//!
//! `g13-b003` R1 fixed `poodle-ir` as **lib only, no `[[bin]]`**, pure
//! serializable data plus validation — an authored *instance* is content,
//! not schema. This module therefore lives in `poodle-codegen`, reachable
//! from the existing bin (`src/bin/poodle-codegen.rs`), and no new crate
//! exists. Where production models are authored is a `g13.008` question;
//! do not mistake this boundary for settled.
//!
//! # R2 — values, not schema
//!
//! Every capability this scene names already has a `poodle-ir` field
//! (the `SHELL-01`–`SHELL-10` rows of `packages/contracts/ir/src/scenes.rs`).
//! Named axis values are derived from the `poodle-tokens` registries
//! ([`poodle_ir::theme_names`] etc.), never hand-listed, so this module and
//! validation cannot disagree.

use poodle_ir::{
    AxisValues, GateTier, Identifier, IrModel, NavSection, NavSectionKind, ParityHarness,
    PreviewState, RouteState, RuntimeTarget, Scene, SceneAxis, SceneAxisKind, SceneLayout,
    SearchConfig, SearchField, SpecimenTabs, VisualGate,
};

/// Builds [`Identifier`]s from preset names. Kept local so the authored
/// values stay plain data; uniqueness is enforced by validation, not by
/// construction (`poodle-ir`'s own rule).
fn ids(values: impl IntoIterator<Item = impl Into<String>>) -> Vec<Identifier> {
    values
        .into_iter()
        .map(Into::into)
        .map(Identifier::new)
        .collect()
}

/// The one scene this card authors: the shared preview shell of `g13.004`.
///
/// The control surface is `SHELL-01`–`SHELL-04` (theme, size, density,
/// contrast) plus `SHELL-06` search; navigation is `SHELL-05`, specimen
/// tabs `SHELL-07`, preview state `SHELL-08`, parity vocabulary `SHELL-09`.
/// No component instances or sidebar groups yet — `R5` locks component
/// migration until `g13.008` records **adopt**, and the specimen registry
/// (`SHELL-10`) is a later-card generated target.
pub fn shell_scene() -> Scene {
    Scene {
        id: Identifier::new("preview-shell"),
        name: "Preview shell".to_owned(),
        description: "The shared preview shell of g13.004: theme/size/density/contrast axes \
                      (SHELL-01-04), navigation (SHELL-05), search (SHELL-06), specimen tabs \
                      (SHELL-07), preview state (SHELL-08), and parity vocabulary (SHELL-09). \
                      Rendered by all four runtimes."
            .to_owned(),
        instances: Vec::new(),
        axes: vec![
            SceneAxis {
                kind: SceneAxisKind::Theme,
                values: AxisValues::Named(ids(poodle_ir::theme_names())),
                description: "Theme preset selection (SHELL-01; CROSS-09 theme axis).".to_owned(),
            },
            SceneAxis {
                kind: SceneAxisKind::Size,
                values: AxisValues::Named(ids(poodle_ir::control_size_names())),
                description: "Control-size selection xs-xl (SHELL-02; CROSS-07).".to_owned(),
            },
            SceneAxis {
                kind: SceneAxisKind::Density,
                values: AxisValues::Named(ids(poodle_ir::density_names())),
                description: "Density selection compact/default/comfortable (SHELL-03; CROSS-08)."
                    .to_owned(),
            },
            SceneAxis {
                kind: SceneAxisKind::Contrast,
                values: AxisValues::Continuous {
                    min: 0.4,
                    max: 1.6,
                    default: 0.5,
                },
                description: "Continuous neutral-contrast override (SHELL-04; CROSS-10; T §7)."
                    .to_owned(),
            },
        ],
        layout: Some(SceneLayout {
            sections: vec![
                NavSection {
                    title: "Components".to_owned(),
                    kind: NavSectionKind::Components,
                    groups: Vec::new(),
                },
                NavSection {
                    title: "Tokens".to_owned(),
                    kind: NavSectionKind::Tokens,
                    groups: Vec::new(),
                },
            ],
            route_state: RouteState {
                // The web shells persist theme/density/controlSize in the
                // URL query (SHELL-08); hash routing is host mechanics.
                persisted: vec![
                    "theme".to_owned(),
                    "density".to_owned(),
                    "controlSize".to_owned(),
                ],
            },
        }),
        tabs: Some(SpecimenTabs {
            tabs: ids(["examples", "sizes", "densities"]),
        }),
        search: Some(SearchConfig {
            case_insensitive: true,
            fields: vec![SearchField::DisplayName, SearchField::Description],
        }),
        preview_state: Some(PreviewState {
            theme: Some(Identifier::new("eclipse")),
            density: Some(Identifier::new("compact")),
            control_size: Some(Identifier::new("sm")),
            contrast: Some(0.5),
        }),
        parity: Some(ParityHarness {
            defaults: PreviewState {
                theme: Some(Identifier::new("eclipse")),
                density: Some(Identifier::new("compact")),
                control_size: Some(Identifier::new("sm")),
                contrast: Some(0.5),
            },
            review_route_presets: ids(["components", "tokens"]),
            targets: vec![
                RuntimeTarget::Svelte,
                RuntimeTarget::React,
                RuntimeTarget::Gpui,
                RuntimeTarget::Jetstream,
            ],
            visual_gates: vec![
                VisualGate {
                    tier: GateTier::Smoke,
                    axes: Vec::new(),
                },
                VisualGate {
                    tier: GateTier::Axis,
                    axes: vec![
                        SceneAxisKind::Size,
                        SceneAxisKind::Density,
                        SceneAxisKind::Contrast,
                    ],
                },
                VisualGate {
                    tier: GateTier::Sweep,
                    axes: vec![SceneAxisKind::Theme],
                },
            ],
            native_visual_baseline: true,
        }),
        captures: ids([
            "shell-theme",
            "shell-size",
            "shell-density",
            "shell-contrast",
            "shell-search",
        ]),
    }
}

/// The shell model — the scene alone, with no shared types, components,
/// or conformance vectors (`R5`: no component migration this card).
pub fn shell_model() -> IrModel {
    IrModel {
        schema_version: poodle_ir::IR_SCHEMA_VERSION,
        shared_types: Vec::new(),
        components: Vec::new(),
        conformance_vectors: Vec::new(),
        scenes: vec![shell_scene()],
        specimen_registry: None,
    }
}
