//! Internal resolved icon-geometry construction.
//!
//! Shared composition owns pair lookup, size, colour, identity, policy, and
//! frame. The emitted node carries only a compact validated frame — backends
//! cannot recover pair meaning.
//!
//! Default visibility is crate-private. The `icon-geometry-internal` feature is
//! a sealed host/test route; it is not a public consumer API.

#[allow(dead_code)]
#[path = "../../contracts/components/src/icon_geometry.rs"]
mod runtime;

#[allow(unused_imports)]
pub use runtime::{
    abort_icon_geometry, activate_icon_geometry, candidate_fixture_ids, complete_icon_geometry,
    create_icon_geometry_runtime, current_icon_geometry_frame, icon_geometry_clock_timing,
    live_geometry_clock_count, planned_candidate_fixture, sample_icon_geometry,
    set_icon_geometry_policy, teardown_icon_geometry, CompactGeometryFrame, GeometryEndpoint,
    GeometryRuntimeDecision, GeometryRuntimeIntent, IconGeometryRuntime, ICON_GEOMETRY_CHANNEL,
    ICON_GEOMETRY_DURATION_MS, ICON_GEOMETRY_ROLE,
};

use poodle_node::{
    CrossAxisAlignment, LayoutDirection, LayoutSizing, MainAxisAlignment, Node, NodeKind,
    ResolvedIconContour, ResolvedIconGeometryFrame,
};

use crate::context::RenderContext;

pub fn resolved_icon_geometry(
    runtime: &IconGeometryRuntime,
    size: f32,
    ctx: &RenderContext<'_>,
) -> Node {
    let color = ctx.theme().resolve_color("color.icon.primary");
    let mut frame = ResolvedIconGeometryFrame::default();
    write_compact_into(runtime, &mut frame);
    let mut el = Node::resolved_icon_geometry(size, frame);
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.descriptor.layout.width = LayoutSizing::Fixed(size);
        s.descriptor.layout.height = LayoutSizing::Fixed(size);
        s.descriptor.text_color = Some(color);
    }
    el
}

/// Mutate an existing resolved-geometry node in place after plan creation.
/// Interior samples reuse reserved contour/point capacity; no new rows.
pub fn write_resolved_frame(runtime: &IconGeometryRuntime, node: &mut Node) {
    let NodeKind::ResolvedIconGeometry { frame, .. } = &mut node.kind else {
        return;
    };
    write_compact_into(runtime, frame);
}

pub fn resolved_frame_point_caps(node: &Node) -> Vec<usize> {
    let NodeKind::ResolvedIconGeometry { frame, .. } = &node.kind else {
        return Vec::new();
    };
    frame
        .contours
        .iter()
        .map(|contour| contour.points.capacity())
        .collect()
}

pub fn compact_frame_point_caps(runtime: &IconGeometryRuntime) -> Vec<usize> {
    runtime::compact_frame_point_caps(runtime)
}

pub fn compact_frame_point_ptrs(runtime: &IconGeometryRuntime) -> Vec<*const (i32, i32)> {
    runtime::compact_frame_point_ptrs(runtime)
}

fn write_compact_into(runtime: &IconGeometryRuntime, dest: &mut ResolvedIconGeometryFrame) {
    let Some(src) = current_icon_geometry_frame(runtime) else {
        dest.contours.clear();
        return;
    };
    if dest.contours.len() < src.contours.len() {
        dest.contours.resize(
            src.contours.len(),
            ResolvedIconContour {
                closed: false,
                points: Vec::new(),
            },
        );
    }
    dest.contours.truncate(src.contours.len());
    for (dest_contour, src_contour) in dest.contours.iter_mut().zip(src.contours.iter()) {
        dest_contour.closed = src_contour.closed;
        let needed = src_contour.points.len().max(src_contour.points.capacity());
        if dest_contour.points.capacity() < needed {
            dest_contour
                .points
                .reserve(needed - dest_contour.points.len());
        }
        dest_contour.points.clear();
        dest_contour.points.extend_from_slice(&src_contour.points);
    }
}

#[cfg(test)]
mod tests {
    use poodle_headless::motion_policy::MotionPolicy;
    use poodle_node::NodeKind;
    use poodle_specs::IconSpec;

    use super::*;
    use crate::icon::icon;

    #[test]
    fn named_icon_path_stays_a_named_icon_node() {
        let theme =
            poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);
        let ctx = RenderContext::new(&theme);
        let node = icon(&IconSpec::new("plus"), &ctx);
        assert!(matches!(
            &node.kind,
            NodeKind::Icon { name, .. } if name == "plus"
        ));
        assert!(node.has_text("plus"));
    }

    #[test]
    fn resolved_frame_carries_no_pair_identity() {
        let theme =
            poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);
        let ctx = RenderContext::new(&theme);
        let mut runtime = create_icon_geometry_runtime(MotionPolicy::Full);
        activate_icon_geometry(
            &mut runtime,
            GeometryRuntimeIntent {
                owner: String::from("fixture-owner"),
                pair_id: String::from("chevron-left-to-chevron-right"),
                target: GeometryEndpoint::To,
                initial: true,
            },
        );
        let node = resolved_icon_geometry(&runtime, 16.0, &ctx);
        match &node.kind {
            NodeKind::ResolvedIconGeometry { size, frame } => {
                assert_eq!(*size, 16.0);
                assert!(!frame.contours.is_empty());
            }
            _ => panic!("expected resolved geometry, got {node:?}"),
        }
        assert!(!node.has_text("chevron-left-to-chevron-right"));
        assert!(!node.has_text("chevron-left"));
        assert!(!node.has_text("chevron-right"));
        assert!(node.texts().is_empty());
    }

    #[test]
    fn construction_is_sealed_from_the_crate_root() {
        let render_lib = include_str!("lib.rs");
        assert!(
            !render_lib.contains("pub use icon_geometry::resolved_icon_geometry"),
            "resolved_icon_geometry must not be a crate-root consumer path"
        );
        assert!(
            render_lib.contains("icon-geometry-internal"),
            "construction stays behind the sealed internal feature"
        );
        let specs_lib = include_str!("../../contracts/components/src/lib.rs");
        assert!(
            specs_lib.contains("pub(crate) mod icon_geometry"),
            "poodle-specs must not export icon_geometry as a public module"
        );
        assert!(
            !specs_lib
                .lines()
                .any(|line| line.trim() == "pub mod icon_geometry"),
            "poodle-specs icon_geometry leaked as pub mod"
        );
    }

    #[test]
    fn interior_samples_reuse_compact_and_node_capacity() {
        let theme =
            poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);
        let ctx = RenderContext::new(&theme);
        let mut runtime = create_icon_geometry_runtime(MotionPolicy::Full);
        let start = activate_icon_geometry(
            &mut runtime,
            GeometryRuntimeIntent {
                owner: String::from("fixture-owner"),
                pair_id: String::from("chevron-left-to-chevron-right"),
                target: GeometryEndpoint::To,
                initial: false,
            },
        );
        let mut node = resolved_icon_geometry(&runtime, 16.0, &ctx);
        sample_icon_geometry(&mut runtime, &start.key, 0.2);
        write_resolved_frame(&runtime, &mut node);
        let compact_caps = compact_frame_point_caps(&runtime);
        let compact_ptrs = compact_frame_point_ptrs(&runtime);
        let node_caps = resolved_frame_point_caps(&node);
        sample_icon_geometry(&mut runtime, &start.key, 0.8);
        write_resolved_frame(&runtime, &mut node);
        assert_eq!(compact_frame_point_caps(&runtime), compact_caps);
        assert_eq!(compact_frame_point_ptrs(&runtime), compact_ptrs);
        assert_eq!(resolved_frame_point_caps(&node), node_caps);
    }
}
