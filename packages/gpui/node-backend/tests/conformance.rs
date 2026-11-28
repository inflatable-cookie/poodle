//! The pure pilot reads GPUI's laid-out bounds, AccessKit updates and scene.

#![cfg(any(target_os = "macos", target_os = "linux"))]

#[allow(dead_code)]
#[path = "../../preview/src/headless_driver.rs"]
mod headless_driver;
#[path = "support/window.rs"]
mod window;

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fs;
use std::process::Command;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use gpui::accesskit::{self, NodeId, TreeUpdate};
use gpui::{
    point, px, rgb, size, solid_background, AppContext, Application, ApplicationHandle,
    AssetSource, Bounds, FrameRequestSource, RequestFrameOptions, SharedString, WindowBounds,
    WindowHandle, WindowOptions,
};
use poodle_conformance::{
    cases, digest, Backend, Identity, Probe, Reading, Receipt, Sprite, Status,
};
use poodle_gpui_node_backend::{bounds_for, painted_text_for, reset_focus_registry};
use poodle_node::Node;
use serde_json::{json, Value};

use headless_driver::HeadlessRoot;
use window::{Output, PlatformTap};

struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> anyhow::Result<Option<Cow<'static, [u8]>>> {
        use poodle_conformance::{IMAGE_ASSET, IMAGE_SVG, NAMED_ICON, NAMED_ICON_SVG};
        let icon = format!("assets/icons/{NAMED_ICON}.svg");
        Ok(match path {
            IMAGE_ASSET => Some(Cow::Borrowed(IMAGE_SVG)),
            path if path == icon => Some(Cow::Borrowed(NAMED_ICON_SVG)),
            _ => None,
        })
    }

    fn list(&self, _: &str) -> anyhow::Result<Vec<SharedString>> {
        Ok(Vec::new())
    }
}

struct Gpui {
    platform: Rc<PlatformTap>,
    app: ApplicationHandle,
    window: Option<WindowHandle<HeadlessRoot>>,
    scale: f32,
    tab_stops: Value,
    ids: BTreeMap<NodeId, String>,
}

impl Gpui {
    fn new() -> Self {
        let platform = Rc::new(PlatformTap::new());
        let app = Application::with_platform(platform.clone())
            .with_assets(Assets)
            .run_embedded(|_| {});
        Self {
            platform,
            app,
            window: None,
            scale: 1.0,
            tab_stops: Value::Null,
            ids: BTreeMap::new(),
        }
    }

    fn frame(&self) {
        self.platform.run_until_idle();
        let mut request = self
            .platform
            .output
            .borrow_mut()
            .request
            .take()
            .expect("GPUI frame callback");
        request(RequestFrameOptions {
            require_presentation: true,
            force_render: true,
            signal_at: None,
            signal_source: FrameRequestSource::LocalSchedule,
        });
        self.platform.output.borrow_mut().request = Some(request);
    }

    fn a11y_node(&self, target: &str) -> Option<NodeId> {
        self.ids
            .iter()
            .find_map(|(id, name)| (name == target).then_some(*id))
    }

    fn a11y_property(&self, node: &accesskit::Node, property: &str) -> Reading {
        let related = |ids: &[NodeId]| ids.iter().map(|id| self.ids.get(id)).collect::<Vec<_>>();
        let value = match property {
            "role" => json!(match node.role() {
                accesskit::Role::SearchInput => "SearchBox".into(),
                role => format!("{role:?}"),
            }),
            "label" => json!(node.label()),
            "expanded" => json!(node.is_expanded()),
            "selected" => json!(node.is_selected()),
            "multiselectable" => json!(node.is_multiselectable()),
            "hidden" => json!(node.is_hidden()),
            "has_popup" => enum_value(node.has_popup()),
            "orientation" => enum_value(node.orientation()),
            "toggled" => enum_value(node.toggled()),
            "level" => json!(node.level()),
            "value" => json!(node.numeric_value()),
            "value_min" => json!(node.min_numeric_value()),
            "value_max" => json!(node.max_numeric_value()),
            "value_text" => json!(node.value()),
            "invalid" => json!(node
                .invalid()
                .map(|invalid| invalid == accesskit::Invalid::True)),
            "busy" => json!(node.is_busy()),
            "controls" => json!(related(node.controls())),
            "labelled_by" => json!(related(node.labelled_by())),
            "described_by" => json!(related(node.described_by())),
            "focus_action" => json!(node.supports_action(accesskit::Action::Focus)),
            _ => return Reading::Unsupported(format!("unknown accessibility property {property}")),
        };
        Reading::Observed(value)
    }

    fn tree(&self) -> std::cell::Ref<'_, TreeUpdate> {
        std::cell::Ref::map(self.platform.output.borrow(), |output| {
            output.tree.as_ref().expect("GPUI sent an AccessKit update")
        })
    }
}

impl Backend for Gpui {
    fn mount(&mut self, root: &Node, viewport: [f32; 2]) {
        if let Some(previous) = self.window.take() {
            self.app
                .update(|cx| previous.update(cx, |_, window, _| window.remove_window()))
                .expect("close former headless window");
        }
        reset_focus_registry();
        *self.platform.output.borrow_mut() = Output::default();
        let mut root_ids = Vec::new();
        collect_ids(root, &mut root_ids);
        let root = Arc::new(Mutex::new(root.clone()));
        let window = self
            .app
            .update(|cx| {
                cx.open_window(
                    WindowOptions {
                        window_bounds: Some(WindowBounds::Windowed(Bounds {
                            origin: point(px(0.0), px(0.0)),
                            size: size(px(viewport[0] + 64.0), px(viewport[1] + 64.0)),
                        })),
                        show: false,
                        focus: false,
                        ..WindowOptions::default()
                    },
                    |_, cx| cx.new(|cx| HeadlessRoot::node(root, viewport, cx)),
                )
            })
            .expect("open the in-memory conformance window");
        self.window = Some(window);

        // The first frame allocates tracked focus handles. A refreshed frame
        // attaches them before GPUI reports focused AccessKit nodes.
        for _ in 0..3 {
            self.frame();
        }
        let (scale, debug) = self
            .app
            .update(|cx| {
                window.update(cx, |_, window, _| {
                    (window.scale_factor(), window.debug_a11y_tree_json())
                })
            })
            .expect("read the mounted window");
        self.scale = scale;
        let debug: Value = serde_json::from_str(&debug.expect("GPUI built its accessibility tree"))
            .expect("decode GPUI's debug metadata");
        self.tab_stops = debug["frame"]["tab_stop_count"].clone();
        let ids = attribute_nodes(&root_ids, &self.tree(), self.scale);
        self.ids = ids;
        assert!(
            !self.tree().nodes.is_empty(),
            "the platform received accessible output"
        );
    }

    fn read(&mut self, target: &str, probe: Probe) -> Reading {
        let value = match probe {
            Probe::A11y(property) => {
                let id = self.a11y_node(target);
                let tree = self.tree();
                if let Some((_, node)) = tree.nodes.iter().find(|(node, _)| Some(*node) == id) {
                    return self.a11y_property(node, property);
                }
                Value::Null
            }
            Probe::Focused => json!(self.a11y_node(target) == Some(self.tree().focus)),
            Probe::TabStops => self.tab_stops.clone(),
            Probe::Bounds => {
                let positioned = bounds_for(target).zip(bounds_for("root"));
                json!(positioned.map(|(bounds, root)| [
                    f32::from(bounds.origin.x - root.origin.x),
                    f32::from(bounds.origin.y - root.origin.y),
                    f32::from(bounds.size.width),
                    f32::from(bounds.size.height),
                ]))
            }
            Probe::ShapedText => json!(painted_text_for(target)),
            Probe::WiderThan(other) => json!(bounds_for(target)
                .zip(bounds_for(other))
                .map(|(target, other)| target.size.width > other.size.width)),
            Probe::Paint {
                point: [x, y],
                rgb: [r, g, b],
            } => {
                let color = solid_background(rgb(u32::from_be_bytes([0, r, g, b])));
                json!(bounds_for(target).map(|bounds| {
                    let p = (bounds.origin + point(px(x), px(y))).scale(self.scale);
                    let output = self.platform.output.borrow();
                    output.quads.iter().any(|quad| {
                        quad.background == color
                            && quad.content_mask.bounds.contains(&p)
                            && quad.bounds.contains(&p)
                    }) || output.path_covers(p, color)
                }))
            }
            Probe::Sprite(kind) => {
                let bounds = bounds_for(target).map(|bounds| bounds.scale(self.scale));
                let deadline = Instant::now() + Duration::from_millis(500);
                loop {
                    let visible = bounds.is_some_and(|bounds| {
                        let output = self.platform.output.borrow();
                        let sprites = match kind {
                            Sprite::Mask => &output.masks,
                            Sprite::Image => &output.images,
                        };
                        sprites.iter().any(|sprite| bounds.intersects(sprite))
                    });
                    if visible || Instant::now() >= deadline {
                        break json!(visible);
                    }
                    // The normal asset loader may complete on a worker between frames.
                    std::thread::sleep(Duration::from_millis(2));
                    self.frame();
                }
            }
        };
        Reading::Observed(value)
    }
}

fn collect_ids(node: &Node, ids: &mut Vec<String>) {
    if let Some(id) = &node.id {
        ids.push(id.clone());
    }
    for child in &node.children {
        collect_ids(child, ids);
    }
}

// The small fixtures place each accessible target in its own rectangle.
// Match the emitted AccessKit node to the independently measured layout,
// which also works when GPUI is compiled without debug provenance.
fn attribute_nodes(ids: &[String], tree: &TreeUpdate, scale: f32) -> BTreeMap<NodeId, String> {
    let mut attributed = BTreeMap::new();
    for id in ids {
        let Some(bounds) = bounds_for(id) else {
            continue;
        };
        let rect = accesskit::Rect {
            x0: f64::from(f32::from(bounds.left()) * scale),
            y0: f64::from(f32::from(bounds.top()) * scale),
            x1: f64::from(f32::from(bounds.right()) * scale),
            y1: f64::from(f32::from(bounds.bottom()) * scale),
        };
        let mut matches = tree.nodes.iter().filter(|(_, node)| {
            node.role() != accesskit::Role::Window && node.bounds() == Some(rect)
        });
        if let Some((node, _)) = matches.next() {
            assert!(matches.next().is_none(), "ambiguous AccessKit target {id}");
            assert!(
                attributed.insert(*node, id.clone()).is_none(),
                "overlapping targets for AccessKit node {node:?}"
            );
        }
    }
    attributed
}

fn enum_value(value: Option<impl std::fmt::Debug>) -> Value {
    json!(value.map(|value| format!("{value:?}")))
}

#[test]
fn missing_path_tiles_remove_observed_ring_paint() {
    let _deadline = headless_driver::arm_test_deadline();
    let mut backend = Gpui::new();
    let case = cases()
        .find(|case| case.id == "kind.ProgressRing")
        .expect("ring fixture");
    backend.mount(&case.root, case.viewport);
    let probe = case
        .checks
        .iter()
        .find(|check| matches!(check.probe, Probe::Paint { .. }) && check.expected == json!(true))
        .expect("ring fixture samples covered paint");
    assert!(matches!(
        backend.read(probe.target, probe.probe),
        Reading::Observed(Value::Bool(true))
    ));
    backend.platform.output.borrow_mut().path_tiles.clear();
    assert!(matches!(
        backend.read(probe.target, probe.probe),
        Reading::Observed(Value::Bool(false))
    ));
}

#[test]
fn missing_accesskit_label_is_observed_as_null() {
    let _deadline = headless_driver::arm_test_deadline();
    let mut backend = Gpui::new();
    let case = cases()
        .find(|case| case.id == "a11y.label")
        .expect("label fixture");
    assert_eq!(case.run(&mut backend).status, Status::Pass);

    let id = backend.a11y_node("target").expect("accessible target");
    let mut output = backend.platform.output.borrow_mut();
    let (_, node) = output
        .tree
        .as_mut()
        .expect("platform tree")
        .nodes
        .iter_mut()
        .find(|(node_id, _)| *node_id == id)
        .expect("target in the platform tree");
    node.clear_label();
    drop(output);
    assert!(matches!(
        backend.read("target", Probe::A11y("label")),
        Reading::Observed(Value::Null)
    ));
}

#[test]
fn node_vocabulary_pilot() {
    let _deadline = headless_driver::arm_test_deadline();
    let mut backend = Gpui::new();
    let results = cases()
        .map(|case| case.run(&mut backend))
        .collect::<Vec<_>>();

    // Keep the pilot observational while preventing an empty or disconnected
    // runner from succeeding or producing a receipt.
    for id in [
        "kind.Container",
        "a11y.label",
        "kind.ProgressRing",
        "kind.ResolvedIconGeometry",
    ] {
        let row = results
            .iter()
            .find(|row| row.id == id)
            .expect("pilot canary");
        assert_eq!(row.status, Status::Pass, "{row:?}");
    }

    if let Some(path) = std::env::var_os("POODLE_GPUI_CONFORMANCE_RECEIPT") {
        let git = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
            .expect("read source commit");
        assert!(git.status.success(), "git rev-parse HEAD failed");
        let commit = String::from_utf8(git.stdout)
            .expect("git commit is UTF-8")
            .trim()
            .to_owned();
        let identity = Identity {
            poodle_commit: commit.clone(),
            backend_commit: commit,
            lock_sha256: digest(include_bytes!("../Cargo.lock")),
            platform: format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH),
        };
        let receipt = Receipt::new(identity, results).expect("complete GPUI case results");
        let json = receipt.to_json().expect("serialize GPUI receipt");
        Receipt::from_json(json.as_bytes()).expect("validate the serialized GPUI receipt");
        fs::write(path, json + "\n").expect("write GPUI receipt");
    }
}
