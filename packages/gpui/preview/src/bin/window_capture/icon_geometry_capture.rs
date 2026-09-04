//! Closed icon-geometry fixture kind for the non-activating capture binary.
//!
//! The fixture drives the internal runtime deterministically before the first
//! window exists. The resulting resolved node is then painted by the ordinary
//! GPUI backend; no SVG or alternate drawing path participates.

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{bail, Context as _, Result};
use gpui::{
    div, px, App, AppContext as _, Context, IntoElement, ParentElement, Render, Styled, Window,
};
use poodle_adapter::ThemeProvider;
use poodle_headless::motion_policy::MotionPolicy;
use poodle_node::{Node, NodeKind};
use poodle_render::icon_geometry::{
    activate_icon_geometry, create_icon_geometry_runtime, planned_candidate_fixture,
    sample_icon_geometry, set_icon_geometry_policy, teardown_icon_geometry, GeometryEndpoint,
    GeometryRuntimeIntent,
};
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::fixture_capture::{inter_fonts, FixtureAssets};
use crate::publish_pair;
use crate::transport::{self, TRANSPORT};

const SCHEMA: &str = "poodle.icon-geometry-visual-capture.v1";
const SIZE: f32 = 128.0;
const PADDING: f32 = 32.0;
const ICON_SIZE: f32 = 64.0;
const OWNER: &str = "window-capture-icon-geometry";
const USAGE: &str = "usage: poodle-window-capture --icon-geometry --pair <candidate-id> --direction <forward|reverse> --state <endpoint-from|endpoint-to|midpoint|reverse-midpoint|frozen|interruption|teardown> --out <png> --receipt <json>";

#[derive(Debug)]
pub struct IconGeometryArgs {
    pair: String,
    direction: Direction,
    state: State,
    out_png: PathBuf,
    out_receipt: PathBuf,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Direction {
    Forward,
    Reverse,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "kebab-case")]
enum State {
    EndpointFrom,
    EndpointTo,
    Midpoint,
    ReverseMidpoint,
    Frozen,
    Interruption,
    Teardown,
}

pub fn parse_args(argv: &[String]) -> Result<IconGeometryArgs> {
    if argv.first().map(String::as_str) != Some("--icon-geometry") {
        bail!("icon-geometry mode must begin with --icon-geometry\n{USAGE}");
    }
    let mut pair = None;
    let mut direction = None;
    let mut state = None;
    let mut out_png = None;
    let mut out_receipt = None;
    let mut i = 1;
    while i < argv.len() {
        let flag = argv[i].as_str();
        let value = argv
            .get(i + 1)
            .with_context(|| format!("missing value for {flag}\n{USAGE}"))?;
        i += 2;
        match flag {
            "--pair" => {
                if planned_candidate_fixture(value).is_none() {
                    bail!("unknown icon-geometry candidate pair '{value}'");
                }
                pair = Some(value.to_owned());
            }
            "--direction" => {
                direction = Some(match value.as_str() {
                    "forward" => Direction::Forward,
                    "reverse" => Direction::Reverse,
                    _ => bail!("unknown direction '{value}'"),
                })
            }
            "--state" => {
                state = Some(match value.as_str() {
                    "endpoint-from" => State::EndpointFrom,
                    "endpoint-to" => State::EndpointTo,
                    "midpoint" => State::Midpoint,
                    "reverse-midpoint" => State::ReverseMidpoint,
                    "frozen" => State::Frozen,
                    "interruption" => State::Interruption,
                    "teardown" => State::Teardown,
                    _ => bail!("unknown icon-geometry state '{value}'"),
                })
            }
            "--out" => out_png = Some(PathBuf::from(value)),
            "--receipt" => out_receipt = Some(PathBuf::from(value)),
            other => bail!("argument '{other}' is not accepted in icon-geometry mode\n{USAGE}"),
        }
    }
    let out_png = out_png.with_context(|| format!("--out is required\n{USAGE}"))?;
    let out_receipt = out_receipt.with_context(|| format!("--receipt is required\n{USAGE}"))?;
    if out_png == out_receipt {
        bail!("--out and --receipt must name different files");
    }
    Ok(IconGeometryArgs {
        pair: pair.with_context(|| format!("--pair is required\n{USAGE}"))?,
        direction: direction.with_context(|| format!("--direction is required\n{USAGE}"))?,
        state: state.with_context(|| format!("--state is required\n{USAGE}"))?,
        out_png,
        out_receipt,
    })
}

struct Root {
    node: Node,
    canvas: gpui::Hsla,
}
impl Render for Root {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        poodle_gpui_node_backend::reset_element_ids();
        div()
            .size_full()
            .p(px(PADDING))
            .bg(self.canvas)
            .child(poodle_gpui_node_backend::to_gpui(&self.node))
    }
}

fn target(direction: Direction) -> GeometryEndpoint {
    match direction {
        Direction::Forward => GeometryEndpoint::To,
        Direction::Reverse => GeometryEndpoint::From,
    }
}
fn opposite(direction: Direction) -> GeometryEndpoint {
    match target(direction) {
        GeometryEndpoint::From => GeometryEndpoint::To,
        GeometryEndpoint::To => GeometryEndpoint::From,
    }
}

fn realise(args: &IconGeometryArgs) -> Result<(Node, gpui::Hsla, Option<f32>, &'static str)> {
    let mut runtime = create_icon_geometry_runtime(MotionPolicy::Full);
    let intent = |target, initial| GeometryRuntimeIntent {
        owner: OWNER.to_owned(),
        pair_id: args.pair.clone(),
        target,
        initial,
    };
    let (sample, policy) = match args.state {
        State::EndpointFrom => {
            activate_icon_geometry(&mut runtime, intent(GeometryEndpoint::From, true));
            (Some(0.0), "full")
        }
        State::EndpointTo => {
            activate_icon_geometry(&mut runtime, intent(GeometryEndpoint::To, true));
            (Some(1.0), "full")
        }
        state => {
            let initial = opposite(args.direction);
            activate_icon_geometry(&mut runtime, intent(initial, true));
            let first = activate_icon_geometry(&mut runtime, intent(target(args.direction), false));
            match state {
                State::Midpoint => {
                    sample_icon_geometry(&mut runtime, &first.key, 0.5);
                    (Some(0.5), "full")
                }
                State::ReverseMidpoint | State::Interruption => {
                    sample_icon_geometry(&mut runtime, &first.key, 0.5);
                    let second = activate_icon_geometry(&mut runtime, intent(initial, false));
                    sample_icon_geometry(&mut runtime, &second.key, 0.5);
                    (Some(0.5), "full")
                }
                State::Frozen => {
                    set_icon_geometry_policy(&mut runtime, MotionPolicy::Frozen);
                    (Some(1.0), "frozen")
                }
                State::Teardown => {
                    teardown_icon_geometry(&mut runtime, None);
                    (None, "full")
                }
                State::EndpointFrom | State::EndpointTo => {
                    unreachable!("endpoint states are handled before the directed clock")
                }
            }
        }
    };
    let theme = poodle_gpui::GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ECLIPSE);
    let ctx = poodle_render::RenderContext::new(&theme);
    let canvas = poodle_gpui_node_backend::color(theme.resolve_color("color.background.canvas"));
    let node = if matches!(args.state, State::Teardown) {
        Node::container()
    } else {
        poodle_render::icon_geometry::resolved_icon_geometry(&runtime, ICON_SIZE, &ctx)
    };
    Ok((node, canvas, sample, policy))
}

fn frame_sha256(node: &Node) -> String {
    let mut bytes = Vec::new();
    if let NodeKind::ResolvedIconGeometry { frame, .. } = &node.kind {
        for contour in &frame.contours {
            bytes.push(u8::from(contour.closed));
            for (x, y) in &contour.points {
                bytes.extend_from_slice(&x.to_le_bytes());
                bytes.extend_from_slice(&y.to_le_bytes());
            }
        }
    }
    format!("{:x}", Sha256::digest(bytes))
}

#[derive(Serialize)]
struct Receipt {
    schema: &'static str,
    pair: String,
    direction: Direction,
    state: State,
    policy: &'static str,
    sample: Option<f32>,
    #[serde(rename = "frameSha256")]
    frame_sha256: String,
    transport: &'static str,
    #[serde(rename = "logicalViewport")]
    logical_viewport: Viewport,
    scene: SceneContract,
    foreground: transport::ForegroundEvidence,
    permission: &'static str,
}
#[derive(Serialize)]
struct Viewport {
    width: u32,
    height: u32,
}
#[derive(Serialize)]
struct SceneContract {
    surface: &'static str,
    #[serde(rename = "paddingPx")]
    padding_px: u32,
    #[serde(rename = "iconSizePx")]
    icon_size_px: u32,
}

pub fn run(args: &IconGeometryArgs) -> ! {
    let (node, canvas, sample, policy) = match realise(args) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("poodle-window-capture: {error:#}");
            std::process::exit(1)
        }
    };
    let hash = frame_sha256(&node);
    let args = Arc::new((
        args.pair.clone(),
        args.direction,
        args.state,
        policy,
        sample,
        args.out_png.clone(),
        args.out_receipt.clone(),
        hash,
    ));
    let shot = transport::Shot {
        label: "icon-geometry".to_owned(),
        logical_width: SIZE,
        logical_height: SIZE,
        build: Box::new(move |_window: &mut Window, cx: &mut App| {
            cx.new(|_| Root { node, canvas })
        }),
        on_frame: transport::settle_after(transport::FRAMES_BEFORE_CAPTURE),
        finish: Box::new(move |facts| {
            let (pair, direction, state, policy, sample, out_png, out_receipt, hash) = &*args;
            let receipt = Receipt {
                schema: SCHEMA,
                pair: pair.clone(),
                direction: *direction,
                state: *state,
                policy,
                sample: *sample,
                frame_sha256: hash.clone(),
                transport: TRANSPORT,
                logical_viewport: Viewport {
                    width: SIZE as u32,
                    height: SIZE as u32,
                },
                scene: SceneContract {
                    surface: "color.background.canvas",
                    padding_px: PADDING as u32,
                    icon_size_px: ICON_SIZE as u32,
                },
                foreground: facts.foreground.clone(),
                permission: "screen-recording-required",
            };
            publish_pair(
                out_png,
                &facts.png,
                out_receipt,
                &serde_json::to_vec_pretty(&receipt)?,
            )
        }),
    };
    transport::capture(
        FixtureAssets {
            base: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        },
        match inter_fonts() {
            Ok(fonts) => fonts,
            Err(error) => {
                eprintln!("poodle-window-capture: {error:#}");
                std::process::exit(1)
            }
        },
        shot,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    fn args(extra: &[&str]) -> Vec<String> {
        [
            "--icon-geometry",
            "--pair",
            "plus-to-x",
            "--direction",
            "forward",
            "--state",
            "midpoint",
            "--out",
            "a.png",
            "--receipt",
            "a.json",
        ]
        .into_iter()
        .chain(extra.iter().copied())
        .map(str::to_owned)
        .collect()
    }
    #[test]
    fn rejected_pair_fails_before_window_creation() {
        let mut a = args(&[]);
        a[2] = "menu-to-x".to_owned();
        assert!(parse_args(&a).is_err());
    }
    #[test]
    fn midpoint_and_reversal_are_distinct() {
        let midpoint = parse_args(&args(&[])).unwrap();
        let mut reverse = args(&[]);
        reverse[6] = "reverse-midpoint".to_owned();
        let reverse = parse_args(&reverse).unwrap();
        let (mid_node, _, sample, _) = realise(&midpoint).unwrap();
        let (reverse_node, _, reverse_sample, _) = realise(&reverse).unwrap();
        assert_eq!(sample, Some(0.5));
        assert_eq!(reverse_sample, Some(0.5));
        assert_ne!(frame_sha256(&mid_node), frame_sha256(&reverse_node));
    }
    #[test]
    fn teardown_is_empty_scene() {
        let mut a = args(&[]);
        a[6] = "teardown".to_owned();
        let (node, _, sample, _) = realise(&parse_args(&a).unwrap()).unwrap();
        assert_eq!(sample, None);
        assert!(matches!(node.kind, NodeKind::Container));
    }

    #[test]
    fn endpoints_ignore_direction_and_paint_the_named_target() {
        for (direction, state, endpoint) in [
            ("forward", "endpoint-from", GeometryEndpoint::From),
            ("forward", "endpoint-to", GeometryEndpoint::To),
            ("reverse", "endpoint-from", GeometryEndpoint::From),
            ("reverse", "endpoint-to", GeometryEndpoint::To),
        ] {
            let mut actual = args(&[]);
            actual[4] = direction.to_owned();
            actual[6] = state.to_owned();
            let actual = parse_args(&actual).unwrap();
            let (node, _, sample, _) = realise(&actual).unwrap();

            let mut runtime = create_icon_geometry_runtime(MotionPolicy::Full);
            activate_icon_geometry(
                &mut runtime,
                GeometryRuntimeIntent {
                    owner: OWNER.to_owned(),
                    pair_id: actual.pair.clone(),
                    target: endpoint,
                    initial: true,
                },
            );
            let theme =
                poodle_gpui::GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ECLIPSE);
            let expected = poodle_render::icon_geometry::resolved_icon_geometry(
                &runtime,
                ICON_SIZE,
                &poodle_render::RenderContext::new(&theme),
            );
            assert_eq!(
                frame_sha256(&node),
                frame_sha256(&expected),
                "{direction}/{state}"
            );
            assert_eq!(
                sample,
                Some(if endpoint == GeometryEndpoint::From {
                    0.0
                } else {
                    1.0
                })
            );
        }
    }
}
