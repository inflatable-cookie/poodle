//! Internal resolved icon-geometry construction.
//!
//! Shared composition owns pair lookup, size, colour, identity, policy, and
//! frame. The emitted node carries only a compact validated frame — backends
//! cannot recover pair meaning.

use poodle_node::{
    CrossAxisAlignment, LayoutDirection, LayoutSizing, MainAxisAlignment, Node,
    ResolvedIconContour, ResolvedIconGeometryFrame,
};
use poodle_specs::icon_geometry::IconGeometryRuntime;

use crate::context::RenderContext;

pub fn resolved_icon_geometry(
    runtime: &IconGeometryRuntime,
    size: f32,
    ctx: &RenderContext<'_>,
) -> Node {
    let color = ctx.theme().resolve_color("color.icon.primary");
    let frame = compact_to_node_frame(runtime);
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

fn compact_to_node_frame(runtime: &IconGeometryRuntime) -> ResolvedIconGeometryFrame {
    let Some(frame) = poodle_specs::icon_geometry::current_icon_geometry_frame(runtime) else {
        return ResolvedIconGeometryFrame::default();
    };
    ResolvedIconGeometryFrame {
        contours: frame
            .contours
            .iter()
            .map(|contour| ResolvedIconContour {
                closed: contour.closed,
                points: contour.points.clone(),
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use poodle_headless::motion_policy::MotionPolicy;
    use poodle_node::NodeKind;
    use poodle_specs::icon_geometry::{
        activate_icon_geometry, create_icon_geometry_runtime, GeometryEndpoint,
        GeometryRuntimeIntent,
    };
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
}
