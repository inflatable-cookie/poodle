//! Viewport containing block for overlay nodes that fill the mount.
//!
//! GPUI 0.2.2 has no `position: fixed`. `style.overlay` already defers paint,
//! but layout still uses the nearest positioned ancestor — every GPUI div is
//! positioned, so an Absolute inset-0 Dialog backdrop collapses to its
//! composition wrapper. ConfirmAction's still-mounted trigger sitting beside
//! the open overlay is the reproducing sibling.
//!
//! Nodes that declare overlay *and* Absolute inset 0 are laid out against the
//! window viewport and prepainted at the window origin. Anchored overlays
//! (Popover, Menu) keep parent-relative Absolute coordinates.

use gpui::{
    div, point, px, size, AnyElement, App, AvailableSpace, Bounds, Element, GlobalElementId,
    InspectorElementId, IntoElement, LayoutId, ParentElement, Pixels, Point, Position, Style,
    Styled, Window,
};
use poodle_node::{Node, NodePosition};

pub(super) fn fills_viewport(node: &Node) -> bool {
    node.style.overlay
        && matches!(
            node.position,
            NodePosition::Absolute {
                top: Some(0.0),
                left: Some(0.0),
                right: Some(0.0),
                bottom: Some(0.0),
            }
        )
}

pub(super) fn viewport_containing_block(child: AnyElement, defer_paint: bool) -> ViewportOverlay {
    ViewportOverlay {
        child: Some(child),
        defer_paint,
    }
}

pub(super) struct ViewportOverlay {
    child: Option<AnyElement>,
    defer_paint: bool,
}

impl IntoElement for ViewportOverlay {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for ViewportOverlay {
    type RequestLayoutState = ();
    type PrepaintState = Option<AnyElement>;

    fn id(&self) -> Option<gpui::ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        // Stay out of the parent's in-flow size so a sibling trigger does not
        // become this overlay's containing block.
        let mut style = Style::default();
        style.position = Position::Absolute;
        style.size.width = px(0.).into();
        style.size.height = px(0.).into();
        (window.request_layout(style, std::iter::empty(), cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let child = self
            .child
            .take()
            .expect("viewport overlay paints once per build");
        let viewport = window.viewport_size();
        // A layout-root Absolute node shrink-wraps its children. Give it a
        // relative host the size of the window so inset-0 fills the mount.
        let mut host = div()
            .w(viewport.width)
            .h(viewport.height)
            .child(child)
            .into_any_element();
        let space = size(
            AvailableSpace::Definite(viewport.width),
            AvailableSpace::Definite(viewport.height),
        );
        host.layout_as_root(space, window, cx);
        if self.defer_paint {
            window.defer_draw(host, Point::default(), 1);
            None
        } else {
            host.prepaint_at(point(px(0.), px(0.)), window, cx);
            Some(host)
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(child) = prepaint {
            child.paint(window, cx);
        }
    }
}
