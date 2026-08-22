//! g15.047 — fixture mode for the offscreen capture target.
//!
//! One invocation renders one accepted Button fixture from the canonical
//! inventory (`--fixture <exact-name> --out <png> --receipt <json>`) through
//! the production path — `ButtonSpec` → `poodle_render::button` →
//! `poodle_gpui_node_backend::to_gpui` into GPUI's `HeadlessAppContext` — and
//! writes the rasterized PNG plus a typed `poodle.button-visual-capture.v1`
//! receipt. The receipt carries the declared landmark bounds (read back from
//! the real paint pass, never recomputed from spec data) and the five visual
//! roles as resolved on the node tree.
//!
//! Determinism rules (the comparison README's scene contract): the fixture's
//! own theme supplies the canvas background, the repo's Inter TTFs are loaded
//! before the window opens, reduce motion is on so the loading spinner paints
//! its declared initial frame, and icons come from the real
//! `packages/render/assets/icons` files. Every input is validated before any
//! renderer is constructed; a missing icon file, a missing font, or a landmark
//! the paint pass never recorded is a hard failure, never a green skip.

use std::borrow::Cow;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context as _, Result, bail};
use gpui::{
    AnyElement, App, AssetSource, Context, HeadlessAppContext, IntoElement, ParentElement, Render,
    SharedString, Styled, TextRun, Window, div, px, size,
};
use poodle_adapter::ThemeProvider;
use poodle_gpui_node_backend::bounds_for;
use poodle_node::{ColorValue, Node, NodeKind};
use poodle_specs::ButtonSpec;
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::inventory::{self, ButtonFixture, FixtureContent, FixtureState};
use crate::presentation_axes::ControlSize;
use crate::{ACCEPTED_SCALE, GPUI_REVISION, publish_pair};

/// Versioned fixture receipt schema identity, shared with the TypeScript
/// verifier (`test/visual/button-comparison/receipt.ts`).
const FIXTURE_RECEIPT_SCHEMA: &str = "poodle.button-visual-capture.v1";

/// The scene's uniform padding: the Button's border-box origin lands at
/// logical (16, 16), the same placement the web fixture hosts use.
const SCENE_PADDING: f32 = 16.0;

/// Landmark element ids stamped on the converted node tree. Stamping an id is
/// capture-host observation, not a component change: it makes the node
/// backend record the element's real paint bounds for `bounds_for`.
const ROOT_ELEMENT_ID: &str = "fixture-root";
const CONTENT_ELEMENT_ID: &str = "fixture-content";
const ICON_ELEMENT_ID: &str = "fixture-icon";
const SPINNER_ELEMENT_ID: &str = "fixture-spinner";

/// Parsed and validated fixture-mode command line.
pub struct FixtureArgs {
    pub fixture: String,
    pub out_png: PathBuf,
    pub out_receipt: PathBuf,
}

/// Real asset source for fixture scenes: the same files the interactive
/// preview serves. A requested icon that is not on disk is an error, never a
/// silently empty frame.
pub(crate) struct FixtureAssets {
    pub(crate) base: PathBuf,
}

impl AssetSource for FixtureAssets {
    fn load(&self, path: &str) -> anyhow::Result<Option<Cow<'static, [u8]>>> {
        let Some(name) = path.strip_prefix("assets/icons/") else {
            return Ok(None);
        };
        let full_path = self.base.join("../../render/assets/icons").join(name);
        let data = std::fs::read(&full_path).with_context(|| {
            format!(
                "fixture icon asset is missing or unreadable: {}",
                full_path.display()
            )
        })?;
        Ok(Some(Cow::Owned(data)))
    }

    fn list(&self, _path: &str) -> anyhow::Result<Vec<SharedString>> {
        Ok(Vec::new())
    }
}

/// The icons a fixture scene will request, so a missing file fails before any
/// renderer is constructed rather than painting an empty glyph slot.
fn preflight_icon_assets(fixture: &ButtonFixture) -> Result<()> {
    let mut names: Vec<&str> = Vec::new();
    match &fixture.content {
        FixtureContent::LeadingIcon { icon, .. } | FixtureContent::IconOnly { icon, .. } => {
            names.push(icon);
        }
        FixtureContent::Label { .. } => {}
    }
    if fixture.state == FixtureState::Loading {
        names.push("spinner");
    }
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../render/assets/icons");
    for name in names {
        let path = dir.join(format!("{name}.svg"));
        if !path.is_file() {
            bail!(
                "fixture '{}' requires icon asset {}, which does not exist",
                fixture.name,
                path.display()
            );
        }
    }
    Ok(())
}

/// Inter static weights, loaded before the window opens so label shaping is
/// the same real text stack the interactive preview uses. The files are part
/// of the scene contract: a missing font is a hard failure.
pub(crate) fn inter_fonts() -> Result<Vec<Cow<'static, [u8]>>> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/fonts");
    ["Inter-Regular.ttf", "Inter-Medium.ttf"]
        .iter()
        .map(|name| {
            let path = dir.join(name);
            std::fs::read(&path)
                .map(Cow::Owned)
                .with_context(|| format!("fixture font is missing: {}", path.display()))
        })
        .collect()
}

/// Map the presentation-axis control size onto the spec's own enum. The two
/// enumerations are deliberately separate authorities; the inventory validated
/// the name against both.
fn spec_control_size(size: ControlSize) -> poodle_specs::ControlSize {
    match size {
        ControlSize::Xs => poodle_specs::ControlSize::Xs,
        ControlSize::Sm => poodle_specs::ControlSize::Sm,
        ControlSize::Md => poodle_specs::ControlSize::Md,
        ControlSize::Lg => poodle_specs::ControlSize::Lg,
        ControlSize::Xl => poodle_specs::ControlSize::Xl,
    }
}

/// Build the spec directly from the fixture's resolved values — the declared
/// end state, no input replay.
fn build_spec(fixture: &ButtonFixture) -> ButtonSpec {
    let spec = ButtonSpec::new()
        .with_variant(fixture.variant)
        .with_tone(fixture.tone)
        .with_size(spec_control_size(fixture.size))
        .with_density(fixture.density);
    let spec = match fixture.state {
        FixtureState::Rest => spec,
        FixtureState::Disabled => spec.with_disabled(true),
        FixtureState::Loading => spec.with_loading(true),
        FixtureState::Pressed => spec.with_pressed(true),
    };
    match &fixture.content {
        FixtureContent::Label { label } => spec.with_label(label.clone()),
        FixtureContent::LeadingIcon { label, icon } => {
            spec.with_label(label.clone()).with_leading_icon(icon.clone())
        }
        FixtureContent::IconOnly { icon, aria_label } => {
            spec.with_leading_icon(icon.clone()).with_aria_label(aria_label.clone())
        }
    }
}

/// Which content elements the rendered tree carries, found while stamping.
struct StampedContent {
    /// A `NodeKind::Text` label child exists (icon-bearing content shapes and
    /// the loading state carry one; a plain label rides on the root instead).
    label_element: bool,
}

/// Stamp observation ids on the rendered tree: root, label text, leading
/// icon, spinner — the elements the fixture's declared landmarks name.
fn stamp_landmark_ids(node: &mut Node) -> StampedContent {
    node.id = Some(ROOT_ELEMENT_ID.to_owned());
    let mut stamped = StampedContent {
        label_element: false,
    };
    for child in &mut node.children {
        stamp_landmark_descendants(child, &mut stamped);
    }
    stamped
}

/// Icons and the spinner ride in a fixed icon-md wrapper box (contract §8),
/// so the glyph nodes are grandchildren of the Button root — walk the whole
/// subtree rather than the direct children.
fn stamp_landmark_descendants(node: &mut Node, stamped: &mut StampedContent) {
    let target = match &node.kind {
        NodeKind::Text { .. } => Some(CONTENT_ELEMENT_ID),
        NodeKind::Icon { name, .. } if name == "spinner" => Some(SPINNER_ELEMENT_ID),
        NodeKind::Icon { .. } => Some(ICON_ELEMENT_ID),
        _ => None,
    };
    if let Some(id) = target {
        node.id = Some(id.to_owned());
        if id == CONTENT_ELEMENT_ID {
            stamped.label_element = true;
        }
    }
    for child in &mut node.children {
        stamp_landmark_descendants(child, stamped);
    }
}

/// The fixture scene root: the canvas-colored padded surface the Button sits
/// on. Identical placement to the smoke scene; only the background and font
/// family are fixture-driven.
struct FixtureRoot {
    node: Node,
    canvas: gpui::Hsla,
}

impl Render for FixtureRoot {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        poodle_gpui_node_backend::reset_element_ids();
        let element: AnyElement = poodle_gpui_node_backend::to_gpui(&self.node);
        div()
            .size_full()
            .p(px(SCENE_PADDING))
            // Keep the Button at its intrinsic size: a bare gpui `div()` is a
            // block container whose children fill its width, so the scene
            // must be an explicit non-stretching flex container — otherwise
            // the Button captures full-width where the web scene is
            // natural-width. Row direction, matching the web host: a column
            // container's main axis is vertical, and flex-shrink then clamped
            // the xl Button's 52px height to the 48px content box.
            .flex()
            .items_start()
            .bg(self.canvas)
            .font_family("Inter")
            .child(element)
    }
}

/// Logical-pixel bounds of one landmark, as the receipt serializes them.
#[derive(Serialize, Clone, Copy, Debug)]
struct LandmarkBounds {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl LandmarkBounds {
    fn center(self) -> (f32, f32) {
        (self.x + self.width / 2.0, self.y + self.height / 2.0)
    }
}

/// Read one landmark's painted bounds. A landmark the paint pass never
/// recorded is a hard failure — the receipt must never invent geometry.
fn painted_bounds(element_id: &str, landmark: &str) -> Result<LandmarkBounds> {
    let bounds = bounds_for(element_id).with_context(|| {
        format!("landmark '{landmark}' ('{element_id}') was not painted in the captured frame")
    })?;
    let bounds = LandmarkBounds {
        x: f32::from(bounds.origin.x),
        y: f32::from(bounds.origin.y),
        width: f32::from(bounds.size.width),
        height: f32::from(bounds.size.height),
    };
    if bounds.width <= 0.0 || bounds.height <= 0.0 {
        bail!("landmark '{landmark}' painted with non-positive bounds: {bounds:?}");
    }
    Ok(bounds)
}

/// The label's LAYOUT content box, centered on `center`: the real shaped
/// advance width by the declared line box (text_size × line_height — the same
/// box the web host's label element measurement reports). Not the glyph
/// ascent/descent ink box: the cross-runtime definition of the `content`
/// landmark is the layout box.
fn label_content_bounds(
    cx: &HeadlessAppContext,
    label: &str,
    label_size: f32,
    line_height: f32,
    center: (f32, f32),
) -> Result<LandmarkBounds> {
    let run = TextRun {
        len: label.len(),
        font: gpui::Font {
            family: "Inter".into(),
            features: Default::default(),
            fallbacks: None,
            weight: gpui::FontWeight(500.0),
            style: gpui::FontStyle::Normal,
        },
        ..Default::default()
    };
    // Shaping goes through a WindowTextSystem (the line-layout-cached view of
    // the shared text system); a throwaway cache measures identically.
    let text_system = gpui::WindowTextSystem::new(cx.text_system().clone());
    let line = text_system.shape_line(
        SharedString::from(label.to_string()),
        px(label_size),
        &[run],
        None,
    );
    let width = f32::from(line.width());
    let height = label_size * line_height;
    if width <= 0.0 || height <= 0.0 {
        bail!("label '{label}' shaped to non-positive metrics ({width}x{height})");
    }
    Ok(LandmarkBounds {
        x: center.0 - width / 2.0,
        y: center.1 - height / 2.0,
        width,
        height,
    })
}

/// The root landmark is the Button's border box (what the web host's
/// `getBoundingClientRect` reports). The paint-bounds canvas sits inside the
/// element as an absolutely positioned, full-size child: taffy places it at
/// the justify-center static position, so what `bounds_for` records is the
/// *padding* box, shifted (pad_left − pad_right)/2 horizontally when the
/// padding is asymmetric (icon fixtures). Reconstruct the border box from the
/// descriptor's own border width and padding. Vertical padding is symmetric
/// (zero) for Button, so no vertical shift applies.
fn root_border_box(node: &Node) -> Result<LandmarkBounds> {
    let recorded = painted_bounds(ROOT_ELEMENT_ID, "root")?;
    let border = node.style.descriptor.border.width;
    let padding = &node.style.descriptor.layout.spacing.padding;
    let shift = (padding.left - padding.right) / 2.0;
    let bounds = LandmarkBounds {
        x: recorded.x - shift - border,
        y: recorded.y - border,
        width: recorded.width + 2.0 * border,
        height: recorded.height + 2.0 * border,
    };
    if bounds.width <= 0.0 || bounds.height <= 0.0 {
        bail!("reconstructed root border box is non-positive: {bounds:?}");
    }
    Ok(bounds)
}

/// Resolve every landmark the fixture declares, in declared order, to real
/// painted geometry. The key set in the receipt is exactly the declared set.
fn collect_landmarks(
    fixture: &ButtonFixture,
    node: &Node,
    cx: &HeadlessAppContext,
    label_size: f32,
    label_line_height: f32,
    stamped: &StampedContent,
) -> Result<serde_json::Map<String, serde_json::Value>> {
    let root = root_border_box(node)?;
    let mut landmarks = serde_json::Map::new();
    for landmark in &fixture.landmarks {
        let bounds = match landmark.as_str() {
            "root" => root,
            "icon" => painted_bounds(ICON_ELEMENT_ID, "icon")?,
            "spinner" => painted_bounds(SPINNER_ELEMENT_ID, "spinner")?,
            "content" => match &fixture.content {
                // Icon-only has no label element: the content landmark reuses
                // the icon element's bounds, recorded under both keys.
                FixtureContent::IconOnly { .. } => painted_bounds(ICON_ELEMENT_ID, "content")?,
                FixtureContent::Label { label } | FixtureContent::LeadingIcon { label, .. } => {
                    let center = if stamped.label_element {
                        painted_bounds(CONTENT_ELEMENT_ID, "content")?.center()
                    } else {
                        root.center()
                    };
                    label_content_bounds(cx, label, label_size, label_line_height, center)?
                }
            },
            other => bail!("inventory declared an unknown landmark '{other}'"),
        };
        landmarks.insert(
            landmark.clone(),
            serde_json::to_value(bounds).expect("landmark bounds serialize"),
        );
    }
    Ok(landmarks)
}

fn srgb(color: ColorValue) -> [f32; 4] {
    [color.0, color.1, color.2, color.3]
}

/// The five fixed visual roles, read from the rendered node tree (post-render,
/// pre-conversion). The GPUI Button paints no shadow, ever, so the layer list
/// is honestly empty. The focus ring is dormant declared evidence (no fixture
/// captures a focused frame): it is read from the node's dedicated focus-ring
/// channel — the g15.052 native counterpart of the web `outline` — so the
/// recorded width is the declared ring width (2px), not the resting border's.
/// Absent ring (disabled/loading) means no focus treatment exists in this
/// state — an honest null pair.
#[derive(Serialize)]
struct RolesEvidence {
    fill: FillRole,
    border: BorderRole,
    text: FillRole,
    shadow: ShadowRole,
    #[serde(rename = "focus-ring")]
    focus_ring: FocusRingRole,
}

#[derive(Serialize)]
struct FillRole {
    color: [f32; 4],
}

#[derive(Serialize)]
struct BorderRole {
    color: [f32; 4],
    width: f32,
}

#[derive(Serialize)]
struct ShadowRole {
    layers: Vec<serde_json::Value>,
}

#[derive(Serialize)]
struct FocusRingRole {
    color: Option<[f32; 4]>,
    width: Option<f32>,
    status: &'static str,
}

fn roles_evidence(node: &Node) -> Result<RolesEvidence> {
    let descriptor = &node.style.descriptor;
    let fill = descriptor
        .background
        .with_context(|| "rendered button declares no fill color")?;
    let text = descriptor
        .text_color
        .with_context(|| "rendered button declares no text color")?;
    Ok(RolesEvidence {
        fill: FillRole { color: srgb(fill) },
        border: BorderRole {
            color: srgb(descriptor.border.color),
            width: descriptor.border.width,
        },
        text: FillRole { color: srgb(text) },
        shadow: ShadowRole { layers: Vec::new() },
        focus_ring: FocusRingRole {
            color: node.style.focus_ring.map(|ring| srgb(ring.color)),
            width: node.style.focus_ring.map(|ring| ring.width),
            status: "dormant",
        },
    })
}

#[derive(Serialize)]
struct LogicalViewport {
    width: u64,
    height: u64,
}

#[derive(Serialize)]
struct DeviceDimensions {
    width: u32,
    height: u32,
}

#[derive(Serialize)]
struct CaptureEnvironment {
    kind: &'static str,
    os: &'static str,
    arch: &'static str,
    #[serde(rename = "gpuiRevision")]
    gpui_revision: &'static str,
}

/// The typed fixture receipt. CamelCase, closed key sets — the TypeScript
/// verifier rejects unknown keys, so this struct is the whole shape.
#[derive(Serialize)]
struct FixtureReceipt {
    schema: &'static str,
    fixture: String,
    runtime: &'static str,
    #[serde(rename = "logicalViewport")]
    logical_viewport: LogicalViewport,
    scale: u64,
    #[serde(rename = "deviceDimensions")]
    device_dimensions: DeviceDimensions,
    #[serde(rename = "pngSha256")]
    png_sha256: String,
    environment: CaptureEnvironment,
    landmarks: serde_json::Map<String, serde_json::Value>,
    roles: RolesEvidence,
}

/// Render one accepted fixture and write its PNG + typed receipt.
pub fn run(args: &FixtureArgs) -> Result<()> {
    if !cfg!(target_os = "macos") {
        bail!("offscreen capture requires macOS: the Metal headless renderer exists nowhere else");
    }
    // A missing headless renderer (no Metal device) is an explicit failure,
    // checked before any capture is attempted — never a green skip.
    if gpui_platform::current_headless_renderer().is_none() {
        bail!("no GPUI headless renderer available: this machine exposes no compatible Metal device");
    }

    let fixtures = inventory::load_inventory()?;
    let fixture = fixtures
        .iter()
        .find(|fixture| fixture.name == args.fixture)
        .with_context(|| format!("fixture '{}' is not in the canonical inventory", args.fixture))?;
    preflight_icon_assets(fixture)?;

    let theme = fixture
        .theme
        .build_theme()
        .with_control_size(fixture.size.token_definition());
    let canvas = theme.resolve_color("color.background.canvas");

    let spec = build_spec(fixture);
    let mut node = poodle_render::button(&spec, &theme, None);
    let roles = roles_evidence(&node)?;
    let label_size = node
        .style
        .text_size
        .with_context(|| "rendered button declares no label size")?;
    let label_line_height = node
        .style
        .line_height
        .with_context(|| "rendered button declares no label line height")?;
    let stamped = stamp_landmark_ids(&mut node);
    let view_node = node.clone();

    let platform = gpui_platform::current_platform(true);
    let text_system = platform.text_system();
    let assets = FixtureAssets {
        base: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
    };
    let mut cx = HeadlessAppContext::with_platform(text_system, Arc::new(assets), || {
        gpui_platform::current_headless_renderer()
    });
    cx.text_system()
        .add_fonts(inter_fonts()?)
        .with_context(|| "load the fixture Inter fonts")?;
    // Reduce motion freezes the loading spinner at its declared initial frame:
    // the animation paints its start state statically and schedules no frames.
    cx.update(|cx: &mut App| cx.set_reduce_motion(true));

    let logical_width = fixture.viewport.width as f32;
    let logical_height = fixture.viewport.height as f32;
    let window = cx.open_window(
        size(px(logical_width), px(logical_height)),
        |_, cx: &mut App| {
            <App as gpui::AppContext>::new(cx, |_| FixtureRoot {
                node: view_node,
                canvas: poodle_gpui_node_backend::color(canvas),
            })
        },
    )?;
    cx.run_until_parked();

    // Force one full explicit frame so every bounds canvas has painted before
    // the landmark readback; the test platform never requests frames on its
    // own schedule.
    cx.update_window(window.into(), |_, window, cx| {
        window.refresh();
        let _ = window.draw(cx);
    })?;
    cx.run_until_parked();

    let landmarks = collect_landmarks(fixture, &node, &cx, label_size, label_line_height, &stamped)?;

    // Real offscreen readback. A blank image, a synthetic PNG, or a node-tree
    // serialization would not be evidence; this is the rasterized frame.
    let image = cx.capture_screenshot(window.into())?;

    let expected_w = (logical_width * ACCEPTED_SCALE).round() as u32;
    let expected_h = (logical_height * ACCEPTED_SCALE).round() as u32;
    if image.width() != expected_w || image.height() != expected_h {
        bail!(
            "device dimensions {}x{} do not equal logical {}x{} × {ACCEPTED_SCALE}",
            image.width(),
            image.height(),
            logical_width,
            logical_height
        );
    }

    let mut png_bytes: Vec<u8> = Vec::new();
    image.write_to(
        &mut std::io::Cursor::new(&mut png_bytes),
        image::ImageFormat::Png,
    )?;
    if png_bytes.is_empty() {
        bail!("capture produced an empty PNG encoding");
    }
    let png_sha256 = format!("{:x}", Sha256::digest(&png_bytes));

    let receipt = FixtureReceipt {
        schema: FIXTURE_RECEIPT_SCHEMA,
        fixture: fixture.name.clone(),
        runtime: "gpui",
        logical_viewport: LogicalViewport {
            width: fixture.viewport.width,
            height: fixture.viewport.height,
        },
        scale: fixture.scale,
        device_dimensions: DeviceDimensions {
            width: image.width(),
            height: image.height(),
        },
        png_sha256,
        environment: CaptureEnvironment {
            kind: "metal-headless",
            os: std::env::consts::OS,
            arch: std::env::consts::ARCH,
            gpui_revision: GPUI_REVISION,
        },
        landmarks,
        roles,
    };
    let receipt_json = serde_json::to_vec_pretty(&receipt)?;
    publish_pair(&args.out_png, &png_bytes, &args.out_receipt, &receipt_json)?;

    eprintln!(
        "captured {} ({}x{} logical @ {}x) sha256={}",
        fixture.name,
        fixture.viewport.width,
        fixture.viewport.height,
        fixture.scale,
        receipt.png_sha256
    );
    Ok(())
}
