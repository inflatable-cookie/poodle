//! g16.005 — inset-shadow evidence scenes.
//!
//! `inset_shadow.rs` paints inset (inner) shadow layers itself, because
//! crates.io `gpui::BoxShadow` has no `inset` flag. Headless tests prove the
//! band geometry reaches `paint_quad` with the right widths, colour, and
//! padding box — they cannot show how those quads RASTERISE, or whether a
//! deferred overlay surface paints its bands in the right order.
//!
//! The Button visual comparison cannot answer that either: its closed 18-case
//! inventory is Button-only, and `poodle_render::button` emits no shadow
//! layers at all. So this is the scene set that actually exercises the new
//! path, through real components:
//!
//! | Scene | Component | Shape under test |
//! | --- | --- | --- |
//! | `accordion` | `poodle_render::accordion` | offset edge band (top highlight) |
//! | `list-card` | `poodle_render::list_card` | spread ring AND leading edge bar, stacked |
//! | `tabs` | `poodle_render::tabs` | spread ring on a drop target |
//! | `popover` | `poodle_render::popover` | offset edge band on a DEFERRED overlay surface |
//!
//! Point-in-time operator review evidence, not a baseline: nothing reads
//! these files back, no fixture inventory or comparison policy is involved,
//! and the whole set renders in ONE non-activating process through the shared
//! transport.
//!
//! Every scene's receipt carries the bands the paint pass actually recorded
//! (`painted_inset_shadows_for`), so the PNG the operator looks at comes with
//! the geometry it should be showing. A scene where nothing painted is a hard
//! failure — a blank capture is not evidence that the projection works.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use anyhow::{bail, Context as _, Result};
use gpui::{
    div, px, AnyElement, App, AppContext as _, Context, IntoElement, ParentElement, Render, Styled,
    Window,
};
use poodle_adapter::ThemeProvider;
use poodle_node::Node;
use poodle_specs::{
    AccordionItemSpec, AccordionSpec, ListCardSpec, PopoverSpec, TabDefinition, TabsSpec,
};
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::fixture_capture::{inter_fonts, FixtureAssets};
use crate::presentation_axes::ThemePreset;
use crate::publish_pair;
use crate::transport::{self, GPUI_SOURCE, GPUI_VERSION, TRANSPORT};

/// Versioned evidence receipt schema identity.
const RECEIPT_SCHEMA: &str = "poodle.gpui-inset-shadow-evidence.v1";

/// The closed scene set. Closed because the output file names are derived
/// from it: `--out-dir` is a directory, not a free-form path pair.
pub const SCENES: &[&str] = &["accordion", "list-card", "tabs", "popover"];

/// Parsed and validated inset-evidence command line.
#[derive(Debug)]
pub struct InsetEvidenceArgs {
    /// Which scenes to render, in order. `all` expands to [`SCENES`].
    pub scenes: Vec<String>,
    pub out_dir: PathBuf,
}

const USAGE: &str = "usage: poodle-window-capture --inset-evidence <accordion|list-card|tabs|popover|all> --out-dir <dir>";

/// A closed contract: exactly `--inset-evidence` and `--out-dir`, both
/// required, every other flag rejected.
pub fn parse_args(argv: &[String]) -> Result<InsetEvidenceArgs> {
    let mut scenes: Option<Vec<String>> = None;
    let mut out_dir: Option<PathBuf> = None;

    let mut i = 0;
    while i < argv.len() {
        let flag = argv[i].as_str();
        let value = argv
            .get(i + 1)
            .with_context(|| format!("missing value for {flag}\n{USAGE}"))?;
        i += 2;
        match flag {
            "--inset-evidence" => {
                scenes = Some(if value == "all" {
                    SCENES.iter().map(|scene| (*scene).to_owned()).collect()
                } else if SCENES.contains(&value.as_str()) {
                    vec![value.to_owned()]
                } else {
                    bail!("unknown inset-evidence scene '{value}': expected one of {SCENES:?} or 'all'");
                });
            }
            "--out-dir" => out_dir = Some(PathBuf::from(value)),
            other => bail!("argument '{other}' is not accepted in inset-evidence mode\n{USAGE}"),
        }
    }

    let out_dir = out_dir.with_context(|| format!("--out-dir is required\n{USAGE}"))?;
    let out_dir = std::fs::canonicalize(&out_dir).with_context(|| {
        format!(
            "--out-dir must be an existing directory: {}",
            out_dir.display()
        )
    })?;
    if !out_dir.is_dir() {
        bail!("--out-dir is not a directory: {}", out_dir.display());
    }

    Ok(InsetEvidenceArgs {
        scenes: scenes.with_context(|| format!("--inset-evidence is required\n{USAGE}"))?,
        out_dir,
    })
}

/// What one scene renders, and how big a window it needs.
struct EvidenceScene {
    node: Node,
    logical_width: f32,
    logical_height: f32,
}

fn build_scene(scene: &str, ctx: &poodle_render::RenderContext<'_>) -> EvidenceScene {
    match scene {
        // Offset edge band: the 1px top highlight on each item surface.
        "accordion" => EvidenceScene {
            node: poodle_render::accordion(
                &AccordionSpec::new(vec![
                    AccordionItemSpec::new("one", "First section"),
                    AccordionItemSpec::new("two", "Second section"),
                ]),
                ctx,
                poodle_render::AccordionHandlers::new("inset-evidence"),
            ),
            logical_width: 420.0,
            logical_height: 200.0,
        },
        // Spread ring AND leading edge bar on one surface: the stacking case,
        // and the only place two inset layers compose in production.
        "list-card" => EvidenceScene {
            node: poodle_render::list_card(
                &ListCardSpec::new()
                    .with_title("Highlighted and active")
                    .with_subtitle("Accent ring plus leading bar")
                    .with_highlighted(true)
                    .with_active(true),
                ctx,
                poodle_render::ListCardSlots::default(),
                None,
            ),
            logical_width: 420.0,
            logical_height: 140.0,
        },
        // Spread ring alone, on a drop target.
        "tabs" => EvidenceScene {
            node: poodle_render::tabs(
                &TabsSpec::new(vec![
                    TabDefinition::new("a", "One"),
                    TabDefinition::new("b", "Two"),
                    TabDefinition::new("c", "Drop here"),
                ])
                .with_drop_target_value(Some("c".to_owned())),
                ctx,
                None,
                None,
            ),
            logical_width: 420.0,
            logical_height: 120.0,
        },
        // The deferred case: the panel is an overlay surface, so its bands
        // paint inside GPUI's deferred pass rather than the ordinary one.
        "popover" => EvidenceScene {
            node: poodle_render::popover(
                &PopoverSpec::new().with_open(true),
                ctx,
                &poodle_render::PopoverHandlers {
                    on_activate: None,
                    on_dismiss: Some(Arc::new(|_| {})),
                    instance_id: Some("inset-evidence".to_owned()),
                },
                Some(Node::text("Trigger")),
                Some(Node::text("Panel content")),
            ),
            logical_width: 420.0,
            logical_height: 260.0,
        },
        other => unreachable!("scene '{other}' is not in the closed set"),
    }
}

/// Stamp an observation id on every node that declares an inset layer, so the
/// paint registry can be read back per surface. Real compositions put these
/// on inner surfaces, not on the composition root.
fn stamp_inset_nodes(node: &mut Node, scene: &str, next: &mut usize, ids: &mut Vec<String>) {
    if node.style.shadow_layers.iter().any(|layer| layer.inset) {
        let id = format!("inset-evidence:{scene}:{next}");
        node.id = Some(id.clone());
        // The backend keys painted observations by runtime identity when a
        // composition provides one. Popover scopes its surface that way, so
        // stamp both identities or the deferred panel paints under its old
        // runtime id while this evidence lane waits forever on the semantic
        // id above.
        node.runtime_id = Some(id.clone());
        ids.push(id);
        *next += 1;
    }
    for child in &mut node.children {
        stamp_inset_nodes(child, scene, next, ids);
    }
}

/// The scene root. Unlike the fixture and focus-evidence roots, this one
/// mounts a real overlay host: the popover scene's panel is a deferred
/// surface, and without a host it never draws at all.
struct InsetEvidenceRoot {
    node: Node,
    canvas: gpui::Hsla,
    text: gpui::Hsla,
}

impl Render for InsetEvidenceRoot {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // The same frame boundary the preview and the headless driver use:
        // the layer, bounds, and painted-band registries are rebuilt once per
        // rendered frame. Tooltip prepare/sweep bind to this window's handle.
        let window_handle = window.window_handle();
        poodle_gpui_node_backend::overlay_frame_begin_for(window_handle, cx);
        cx.defer(move |_cx| {
            poodle_gpui_node_backend::overlay_frame_end_for(window_handle);
        });
        poodle_gpui_node_backend::reset_element_ids();
        let element: AnyElement = poodle_gpui_node_backend::to_gpui(&self.node);
        poodle_gpui_node_backend::attach_overlay_host(
            div()
                .size_full()
                .p(px(16.0))
                .flex()
                .flex_col()
                .items_start()
                .bg(self.canvas)
                .text_color(self.text)
                .font_family("Inter")
                .child(element),
            window_handle,
        )
    }
}

/// One painted band, as the receipt serializes it.
#[derive(Serialize, Clone)]
struct BandEvidence {
    element: String,
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
    color: [f32; 4],
    /// The padding box the band was clipped to: x, y, width, height.
    bounds: [f32; 4],
}

#[derive(Serialize)]
struct InsetEvidenceReceipt {
    schema: &'static str,
    scene: String,
    /// The bands the paint pass recorded, in stamped order. Never empty: a
    /// scene that painted nothing fails before publishing.
    bands: Vec<BandEvidence>,
    gpui_source: &'static str,
    gpui_version: &'static str,
    transport: &'static str,
    platform: &'static str,
    theme: &'static str,
    logical_viewport: [f32; 2],
    scale: f32,
    device_dimensions: [u32; 2],
    png_sha256: String,
    foreground: transport::ForegroundEvidence,
}

pub fn run(args: &InsetEvidenceArgs) -> ! {
    let prepared: Result<Vec<transport::Shot<InsetEvidenceRoot>>> = args
        .scenes
        .iter()
        .map(|scene| prepare(scene, &args.out_dir))
        .collect();
    let fonts = prepared.as_ref().ok().and(inter_fonts().ok());
    match (prepared, fonts) {
        (Ok(shots), Some(fonts)) => transport::capture_batch(
            FixtureAssets {
                base: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            },
            fonts,
            shots,
        ),
        (Err(error), _) => {
            eprintln!("poodle-window-capture: {error:#}");
            std::process::exit(1)
        }
        (Ok(_), None) => {
            eprintln!("poodle-window-capture: the Inter fonts could not be loaded");
            std::process::exit(1)
        }
    }
}

fn prepare(scene: &str, out_dir: &Path) -> Result<transport::Shot<InsetEvidenceRoot>> {
    let theme = ThemePreset::Eclipse.build_theme();
    let canvas = theme.resolve_color("color.background.canvas");
    let text = theme.resolve_color("color.text.primary");
    let ctx = poodle_render::RenderContext::new(&theme);
    let built = build_scene(scene, &ctx);

    let mut node = built.node;
    let mut ids = Vec::new();
    stamp_inset_nodes(&mut node, scene, &mut 0, &mut ids);
    if ids.is_empty() {
        bail!(
            "scene '{scene}' declares no inset shadow layer, so it cannot be evidence for the \
             inset painter"
        );
    }

    let logical_width = built.logical_width;
    let logical_height = built.logical_height;
    let scene_name = scene.to_owned();
    let out_png = out_dir.join(format!("{scene}.png"));
    let out_receipt = out_dir.join(format!("{scene}.json"));

    let recorded: Arc<Mutex<Vec<BandEvidence>>> = Arc::new(Mutex::new(Vec::new()));
    let frame_recorded = Arc::clone(&recorded);
    let frame_ids = ids.clone();

    Ok(transport::Shot {
        label: format!("inset-evidence/{scene}"),
        logical_width,
        logical_height,
        build: Box::new(move |_window, cx: &mut App| {
            cx.new(|_| InsetEvidenceRoot {
                node,
                canvas: poodle_gpui_node_backend::color(canvas),
                text: poodle_gpui_node_backend::color(text),
            })
        }),
        on_frame: Box::new(move |_window, _cx, _frame| {
            // Wait until EVERY stamped surface has painted its bands. The
            // deferred popover panel lands a frame or two after the ordinary
            // ones, and capturing before it does would produce a PNG missing
            // exactly the case this scene exists to show.
            let mut bands = Vec::new();
            for id in &frame_ids {
                let painted = poodle_gpui_node_backend::painted_inset_shadows_for(id);
                if painted.is_empty() {
                    return Ok(transport::Settled::Wait);
                }
                for band in painted {
                    bands.push(BandEvidence {
                        element: id.clone(),
                        left: band.left,
                        right: band.right,
                        top: band.top,
                        bottom: band.bottom,
                        color: [band.color.0, band.color.1, band.color.2, band.color.3],
                        bounds: band.bounds,
                    });
                }
            }
            *frame_recorded.lock().expect("band slot") = bands;
            Ok(transport::Settled::Ready)
        }),
        finish: Box::new(move |facts: &transport::CaptureFacts| {
            let bands = std::mem::take(&mut *recorded.lock().expect("band slot"));
            if bands.is_empty() {
                bail!("scene '{scene_name}' painted no inset bands — the capture is not evidence");
            }
            let png_sha256 = format!("{:x}", Sha256::digest(&facts.png));
            let receipt = InsetEvidenceReceipt {
                schema: RECEIPT_SCHEMA,
                scene: scene_name.clone(),
                bands,
                gpui_source: GPUI_SOURCE,
                gpui_version: GPUI_VERSION,
                transport: TRANSPORT,
                platform: "macos",
                theme: ThemePreset::Eclipse.label(),
                logical_viewport: [logical_width, logical_height],
                scale: facts.scale,
                device_dimensions: [facts.device_width, facts.device_height],
                png_sha256,
                foreground: facts.foreground.clone(),
            };
            let receipt_json = serde_json::to_vec_pretty(&receipt)?;
            publish_pair(&out_png, &facts.png, &out_receipt, &receipt_json)?;
            eprintln!(
                "inset-evidence {} — {} band(s) painted, sha256={}",
                receipt.scene,
                receipt.bands.len(),
                receipt.png_sha256
            );
            Ok(())
        }),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn argv(args: &[&str]) -> Vec<String> {
        args.iter().map(|s| s.to_string()).collect()
    }

    fn temp_dir() -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("poodle-inset-evidence-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("create test dir");
        dir
    }

    #[test]
    fn a_single_scene_parses() {
        let dir = temp_dir();
        let args = parse_args(&argv(&[
            "--inset-evidence",
            "accordion",
            "--out-dir",
            dir.to_str().expect("utf-8"),
        ]))
        .expect("the canonical invocation parses");
        assert_eq!(args.scenes, vec!["accordion".to_owned()]);
    }

    #[test]
    fn all_expands_to_the_closed_scene_set() {
        let dir = temp_dir();
        let args = parse_args(&argv(&[
            "--inset-evidence",
            "all",
            "--out-dir",
            dir.to_str().expect("utf-8"),
        ]))
        .expect("'all' parses");
        assert_eq!(args.scenes, SCENES.to_vec());
    }

    #[test]
    fn an_unknown_scene_is_rejected() {
        let dir = temp_dir();
        let error = parse_args(&argv(&[
            "--inset-evidence",
            "button",
            "--out-dir",
            dir.to_str().expect("utf-8"),
        ]))
        .expect_err("an unknown scene must not parse");
        assert!(
            format!("{error:#}").contains("unknown inset-evidence scene 'button'"),
            "the error names the offender: {error:#}"
        );
    }

    #[test]
    fn a_missing_output_directory_is_rejected() {
        assert!(
            parse_args(&argv(&[
                "--inset-evidence",
                "accordion",
                "--out-dir",
                "/nonexistent/poodle/inset-evidence",
            ]))
            .is_err(),
            "--out-dir must already exist"
        );
    }

    #[test]
    fn every_flag_is_required_and_extras_are_rejected() {
        let dir = temp_dir();
        let path = dir.to_str().expect("utf-8").to_owned();
        assert!(parse_args(&argv(&["--out-dir", &path])).is_err());
        assert!(parse_args(&argv(&["--inset-evidence", "accordion"])).is_err());
        assert!(parse_args(&argv(&[
            "--inset-evidence",
            "accordion",
            "--out-dir",
            &path,
            "--scale",
            "2.0"
        ]))
        .is_err());
    }

    /// The scenes exist to exercise the inset painter, so each must actually
    /// declare an inset layer through its real component. This is the check
    /// that would have caught the Button-only evidence gap: it fails if a
    /// scene stops producing the thing it is evidence for.
    #[test]
    fn every_scene_declares_at_least_one_inset_layer() {
        let theme = ThemePreset::Eclipse.build_theme();
        let ctx = poodle_render::RenderContext::new(&theme);
        for scene in SCENES {
            let mut node = build_scene(scene, &ctx).node;
            let mut ids = Vec::new();
            stamp_inset_nodes(&mut node, scene, &mut 0, &mut ids);
            assert!(
                !ids.is_empty(),
                "scene '{scene}' declares no inset layer, so it proves nothing"
            );
        }
    }

    /// The two real shapes and the stacking case must all be covered, or the
    /// scene set has a hole in it.
    #[test]
    fn the_scene_set_covers_both_band_shapes_and_the_stacked_case() {
        let theme = ThemePreset::Eclipse.build_theme();
        let ctx = poodle_render::RenderContext::new(&theme);

        fn layers(node: &Node, out: &mut Vec<poodle_node::ShadowLayer>) {
            out.extend(node.style.shadow_layers.iter().filter(|l| l.inset).copied());
            for child in &node.children {
                layers(child, out);
            }
        }

        let mut spread_ring = false;
        let mut edge_band = false;
        for scene in SCENES {
            let mut found = Vec::new();
            layers(&build_scene(scene, &ctx).node, &mut found);
            for layer in &found {
                if layer.spread > 0.0 && layer.offset_x == 0.0 && layer.offset_y == 0.0 {
                    spread_ring = true;
                }
                if layer.spread == 0.0 && (layer.offset_x != 0.0 || layer.offset_y != 0.0) {
                    edge_band = true;
                }
            }
            if *scene == "list-card" {
                assert!(
                    found.len() >= 2,
                    "the list-card scene must stack two inset layers, found {}",
                    found.len()
                );
            }
        }
        assert!(spread_ring, "no scene exercises a spread ring");
        assert!(edge_band, "no scene exercises an offset edge band");
    }

    /// Every declared inset layer in the set has zero blur, which is the
    /// premise the exact projection rests on. If a component ever declares a
    /// blurred one, this fails and the approximation has to be revisited.
    #[test]
    fn every_scene_layer_has_zero_blur() {
        let theme = ThemePreset::Eclipse.build_theme();
        let ctx = poodle_render::RenderContext::new(&theme);

        fn walk(node: &Node, out: &mut Vec<f32>) {
            out.extend(
                node.style
                    .shadow_layers
                    .iter()
                    .filter(|l| l.inset)
                    .map(|l| l.blur),
            );
            for child in &node.children {
                walk(child, out);
            }
        }

        for scene in SCENES {
            let mut blurs = Vec::new();
            walk(&build_scene(scene, &ctx).node, &mut blurs);
            assert!(
                blurs.iter().all(|blur| *blur == 0.0),
                "scene '{scene}' declares a blurred inset layer: {blurs:?}"
            );
        }
    }
}
