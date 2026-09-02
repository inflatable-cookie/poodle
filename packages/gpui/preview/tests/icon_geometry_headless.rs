//! Focused headless probes for the private icon-geometry GPUI path.
//!
//! No native pixels, no windowed capture. Candidate geometry is fixture input.

use std::sync::{Arc, Mutex};
use std::time::Instant;

use gpui::TestAppContext;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::motion_policy::MotionPolicy;
use poodle_node::NodeKind;
use poodle_render::context::RenderContext;
use poodle_specs::icon_geometry::{
    activate_icon_geometry, create_icon_geometry_runtime, sample_icon_geometry,
    teardown_icon_geometry, GeometryEndpoint, GeometryRuntimeIntent,
};
use poodle_specs::IconSpec;

#[path = "../src/headless_driver.rs"]
mod headless_driver;

use headless_driver::HeadlessDriver;

fn run_headless(body: impl FnOnce(&mut TestAppContext)) {
    poodle_gpui_node_backend::reset_focus_registry();
    let mut cx = TestAppContext::single();
    body(&mut cx);
    cx.dispatcher.run_until_parked();
    cx.background_executor.forbid_parking();
    cx.quit();
    cx.dispatcher.run_until_parked();
}

fn theme() -> GpuiThemeProvider {
    GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ECLIPSE)
}

#[test]
fn resolved_geometry_paints_without_pair_lookup_and_tears_down() {
    run_headless(|cx| {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let static_icon = poodle_render::icon(&IconSpec::new("plus"), &ctx);
        assert!(matches!(&static_icon.kind, NodeKind::Icon { name, .. } if name == "plus"));

        let mut runtime = create_icon_geometry_runtime(MotionPolicy::Full);
        let start = activate_icon_geometry(
            &mut runtime,
            GeometryRuntimeIntent {
                owner: String::from("geometry-owner"),
                pair_id: String::from("chevron-left-to-chevron-right"),
                target: GeometryEndpoint::To,
                initial: false,
            },
        );
        let mut node = poodle_render::resolved_icon_geometry(&runtime, 16.0, &ctx);
        node.id = Some("geometry-fixture".into());
        match &node.kind {
            NodeKind::ResolvedIconGeometry { frame, .. } => {
                assert!(!frame.contours.is_empty());
            }
            _ => panic!("expected resolved geometry, got {node:?}"),
        }
        assert!(!node.has_text("chevron-left-to-chevron-right"));

        let live = Arc::new(Mutex::new(node));
        poodle_gpui_node_backend::begin_probe_capture();
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&live));
        driver.draw_frame();
        let started = Instant::now();
        sample_icon_geometry(&mut runtime, &start.key, 0.5);
        {
            let mut guard = live.lock().expect("node lock");
            *guard = poodle_render::resolved_icon_geometry(&runtime, 16.0, &ctx);
            guard.id = Some("geometry-fixture".into());
        }
        driver.draw_frame();
        let elapsed = started.elapsed();
        assert!(
            elapsed.as_millis() <= 4,
            "geometry update exceeded the 1ms/instance budget: {elapsed:?}"
        );
        let channels = poodle_gpui_node_backend::take_probe_capture();
        assert!(
            channels
                .iter()
                .any(|channel| *channel == "content.text-icon.resolved-geometry"),
            "missing resolved-geometry probe, got {channels:?}"
        );
        assert!(
            !channels.iter().any(|channel| channel.contains("pair")),
            "backend must not record pair lookup, got {channels:?}"
        );

        teardown_icon_geometry(&mut runtime, None);
        {
            let mut guard = live.lock().expect("node lock");
            *guard = poodle_node::Node::container();
        }
        driver.draw_frame();
        assert!(matches!(
            live.lock().expect("node lock").kind,
            NodeKind::Container
        ));
        drop(driver);
    });
}

#[test]
fn missing_shared_lookup_cannot_recover_pair_meaning() {
    run_headless(|cx| {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let mut runtime = create_icon_geometry_runtime(MotionPolicy::Full);
        activate_icon_geometry(
            &mut runtime,
            GeometryRuntimeIntent {
                owner: String::from("geometry-owner"),
                pair_id: String::from("menu-to-ellipsis"),
                target: GeometryEndpoint::To,
                initial: false,
            },
        );
        let node = poodle_render::resolved_icon_geometry(&runtime, 16.0, &ctx);
        match &node.kind {
            NodeKind::ResolvedIconGeometry { frame, .. } => {
                assert!(frame.contours.is_empty());
            }
            _ => panic!("expected empty resolved geometry, got {node:?}"),
        }
        let live = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new(cx, live);
        driver.draw_frame();
        drop(driver);
    });
}
