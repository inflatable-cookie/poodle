//! Two-pass measured node host.
//!
//! Architecture 010 still constructs `Spec + RenderContext -> Node` in the
//! composition layer. Parent-owned width and shaped advance are layout facts,
//! so this element waits for allocated width, shapes the strings the caller
//! named, then interprets the Node the caller rebuilds. It does not own
//! component specs or call `poodle-render`.

use std::collections::HashMap;
use std::sync::Arc;

use gpui::{
    px, relative, size, AnyElement, App, AvailableSpace, Bounds, Element, ElementId,
    GlobalElementId, InspectorElementId, IntoElement, LayoutId, Pixels, SharedString, Style,
    TextRun, Window,
};
use poodle_node::{LayoutSizing, Node, NodeKind};

use crate::to_gpui;

/// Shaped inline advance for strings this host was asked to measure.
pub type ShapedAdvance = Arc<dyn Fn(&str) -> f32 + Send + Sync>;

fn shaped_advance(window: &Window, text: &str, font_size_px: f32) -> f32 {
    if text.is_empty() {
        return 0.0;
    }
    let style = window.text_style();
    let font_size = px(font_size_px);
    let run = TextRun {
        len: text.len(),
        font: style.font(),
        color: gpui::white(),
        background_color: None,
        underline: None,
        strikethrough: None,
    };
    let line = window.text_system().shape_line(
        SharedString::from(text.to_owned()),
        font_size,
        &[run],
        None,
    );
    f32::from(line.width)
}

/// Host-shaped inline advance used by block Slider/RangeSlider fit proofs.
pub fn shaped_block_advance(window: &Window, text: &str, font_size_px: f32) -> f32 {
    shaped_advance(window, text, font_size_px)
}

fn shape_texts(window: &Window, texts: &[String], font_size_px: f32) -> ShapedAdvance {
    let mut advances = HashMap::new();
    for text in texts {
        if text.is_empty() {
            continue;
        }
        advances.insert(text.clone(), shaped_advance(window, text, font_size_px));
    }
    Arc::new(move |text: &str| {
        advances
            .get(text)
            .copied()
            .unwrap_or_else(|| panic!("measured node host has no shaped advance for {text:?}"))
    })
}

fn line_height_px(window: &Window, font_size_px: f32) -> f32 {
    let style = window.text_style();
    f32::from(
        style
            .line_height
            .to_pixels(px(font_size_px).into(), window.rem_size())
            .round(),
    )
}

fn child_height(node: &Node, line_height_px: f32) -> f32 {
    match node.style.descriptor.layout.height {
        LayoutSizing::Fixed(h) => h,
        _ => match &node.kind {
            NodeKind::Text { .. } => line_height_px,
            _ => node.style.min_height.unwrap_or(0.0),
        },
    }
}

fn intrinsic_column_height(node: &Node, line_height_px: f32) -> f32 {
    if node.children.is_empty() {
        return child_height(node, line_height_px);
    }
    node.children
        .iter()
        .map(|child| child_height(child, line_height_px))
        .sum()
}

struct MeasuredNodeHost {
    texts: Vec<String>,
    font_size_px: f32,
    rebuild: Arc<dyn Fn(f32, ShapedAdvance) -> Node + Send + Sync>,
}

struct PrepaintState {
    child: AnyElement,
}

impl MeasuredNodeHost {
    fn rebuild_child(&self, capsule_span: f32, window: &Window) -> AnyElement {
        let advance = shape_texts(window, &self.texts, self.font_size_px);
        let node = (self.rebuild)(capsule_span, advance);
        to_gpui(&node)
    }
}

impl IntoElement for MeasuredNodeHost {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for MeasuredNodeHost {
    type RequestLayoutState = ();
    type PrepaintState = PrepaintState;

    fn id(&self) -> Option<ElementId> {
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
        _cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let texts = self.texts.clone();
        let font_size_px = self.font_size_px;
        let rebuild = Arc::clone(&self.rebuild);
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        let layout_id =
            window.request_measured_layout(style, move |known, available, window, _cx| {
                let width = known.width.unwrap_or_else(|| match available.width {
                    AvailableSpace::Definite(w) => w,
                    AvailableSpace::MinContent | AvailableSpace::MaxContent => px(1.0),
                });
                let span = f32::from(width).max(1.0);
                let advance = shape_texts(window, &texts, font_size_px);
                let node = rebuild(span, advance);
                let height = intrinsic_column_height(&node, line_height_px(window, font_size_px));
                size(width, px(height))
            });
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let capsule_span = f32::from(bounds.size.width).max(1.0);
        let mut child = self.rebuild_child(capsule_span, window);
        child.layout_as_root(
            size(
                AvailableSpace::Definite(bounds.size.width),
                AvailableSpace::Definite(bounds.size.height),
            ),
            window,
            cx,
        );
        child.prepaint_at(bounds.origin, window, cx);
        PrepaintState { child }
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
        prepaint.child.paint(window, cx);
    }
}

/// Interpret a Node that must be rebuilt after GPUI knows the allocated width.
///
/// `texts` are the strings the rebuild closure will measure. `rebuild` is
/// owned by the composition layer that already constructs from
/// `Spec + RenderContext`.
pub fn measured_node_element(
    texts: Vec<String>,
    font_size_px: f32,
    rebuild: Arc<dyn Fn(f32, ShapedAdvance) -> Node + Send + Sync>,
) -> AnyElement {
    MeasuredNodeHost {
        texts,
        font_size_px,
        rebuild,
    }
    .into_any_element()
}
