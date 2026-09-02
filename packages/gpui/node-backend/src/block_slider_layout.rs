//! Host-owned two-pass layout for block Slider and RangeSlider.
//!
//! Architecture 010 still constructs `Spec + RenderContext -> Node`. Parent
//! capsule span and shaped inline advance are layout facts, so this GPUI
//! element waits for allocated width, shapes the spec's visible strings
//! through the window text system, then constructs. It does not guess 160px
//! or a character-width heuristic.

use std::collections::HashMap;
use std::sync::Arc;

use gpui::{
    px, relative, size, AnyElement, App, AvailableSpace, Bounds, Element, ElementId,
    GlobalElementId, InspectorElementId, IntoElement, LayoutId, Pixels, SharedString, Style,
    TextRun, Window,
};
use poodle_adapter::ThemeProvider;
use poodle_headless::slider::{
    layout_range_slider_block, layout_slider_block, range_slider_visual_state, resolved_range_text,
    resolved_visible_text, slider_visual_state, RangeSliderControlContext, SliderControlContext,
};
use poodle_node::NodeRole;
use poodle_render::{
    presentation, range_slider, slider, slider_block, BlockTextMeasure, RangeSliderHandlers,
    RenderContext, SliderHandlers,
};
use poodle_specs::{RangeSliderSpec, SliderSpec};

const BLOCK_SLIDER_HOST_ID: &str = "block-slider-host";
const BLOCK_RANGE_SLIDER_HOST_ID: &str = "block-range-slider-host";

use crate::to_gpui;

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

/// Host-shaped inline advance used by block Slider/RangeSlider fit.
pub fn shaped_block_advance(window: &Window, text: &str, font_size_px: f32) -> f32 {
    shaped_advance(window, text, font_size_px)
}

fn measure_for_texts(
    window: &Window,
    texts: impl IntoIterator<Item = String>,
    font_size_px: f32,
) -> BlockTextMeasure {
    let mut advances = HashMap::new();
    for text in texts {
        if text.is_empty() {
            continue;
        }
        advances.insert(text.clone(), shaped_advance(window, &text, font_size_px));
    }
    Arc::new(move |text: &str, font_size| {
        advances.get(text).copied().unwrap_or_else(|| {
            panic!("block slider host has no shaped advance for {text:?} (font_size={font_size})")
        })
    })
}

fn slider_visible_texts(spec: &SliderSpec) -> Vec<String> {
    let mut texts = Vec::new();
    if let Some(label) = spec
        .visible_label
        .as_deref()
        .filter(|text| !text.is_empty())
    {
        texts.push(label.to_owned());
    }
    if let Some(value) = resolved_visible_text(spec.value, spec.visible_value_text.as_deref()) {
        texts.push(value);
    }
    texts
}

fn range_visible_texts(spec: &RangeSliderSpec) -> Vec<String> {
    let mut texts = Vec::new();
    if let Some(label) = spec
        .visible_label
        .as_deref()
        .filter(|text| !text.is_empty())
    {
        texts.push(label.to_owned());
    }
    let lower = resolved_visible_text(spec.low, spec.visible_lower_text.as_deref());
    let upper = resolved_visible_text(spec.high, spec.visible_upper_text.as_deref());
    let range = resolved_range_text(
        spec.low,
        spec.high,
        spec.visible_range_text.as_deref(),
        lower.as_deref(),
        upper.as_deref(),
    );
    for text in [lower, upper, range].into_iter().flatten() {
        if !text.is_empty() {
            texts.push(text);
        }
    }
    texts
}

fn fallback_line_height_px(window: &Window, font_size_px: f32) -> f32 {
    let style = window.text_style();
    f32::from(
        style
            .line_height
            .to_pixels(px(font_size_px).into(), window.rem_size())
            .round(),
    )
}

fn slider_host_height(
    spec: &SliderSpec,
    surface_px: f32,
    capsule_span: f32,
    window: &Window,
    font_size_px: f32,
) -> f32 {
    let measure = measure_for_texts(window, slider_visible_texts(spec), font_size_px);
    let value_text = resolved_visible_text(spec.value, spec.visible_value_text.as_deref());
    let visual = slider_visual_state(SliderControlContext {
        value: spec.value,
        min: spec.min,
        max: spec.max,
        step: spec.step,
        disabled: spec.is_disabled,
        law: spec.law,
        polarity: spec.polarity,
        center_value: spec.center_value,
        pointer_active: false,
    });
    let layout = layout_slider_block(
        capsule_span,
        visual.value_norm as f32,
        spec.visible_label
            .as_deref()
            .filter(|text| !text.is_empty()),
        value_text.as_deref(),
        |text| measure(text, font_size_px),
    );
    if layout.fallback.is_some() {
        surface_px + fallback_line_height_px(window, font_size_px)
    } else {
        surface_px
    }
}

fn range_host_height(
    spec: &RangeSliderSpec,
    surface_px: f32,
    capsule_span: f32,
    window: &Window,
    font_size_px: f32,
) -> f32 {
    let measure = measure_for_texts(window, range_visible_texts(spec), font_size_px);
    let visual = range_slider_visual_state(RangeSliderControlContext {
        value: (spec.low, spec.high),
        min: spec.min,
        max: spec.max,
        step: spec.step,
        disabled: spec.is_disabled,
        law: spec.law,
        polarity: spec.polarity,
        center_value: spec.center_value,
        pointer_active: false,
        active_thumb: None,
    });
    let lower = resolved_visible_text(spec.low, spec.visible_lower_text.as_deref());
    let upper = resolved_visible_text(spec.high, spec.visible_upper_text.as_deref());
    let range = resolved_range_text(
        spec.low,
        spec.high,
        spec.visible_range_text.as_deref(),
        lower.as_deref(),
        upper.as_deref(),
    );
    let layout = layout_range_slider_block(
        capsule_span,
        visual.lower_norm as f32,
        visual.upper_norm.max(visual.lower_norm) as f32,
        spec.visible_label
            .as_deref()
            .filter(|text| !text.is_empty()),
        lower.as_deref(),
        upper.as_deref(),
        range.as_deref(),
        |text| measure(text, font_size_px),
    );
    if layout.fallback.is_some() {
        surface_px + fallback_line_height_px(window, font_size_px)
    } else {
        surface_px
    }
}

fn request_host_layout(
    window: &mut Window,
    spec_height: impl Fn(f32, &Window) -> f32 + 'static,
) -> LayoutId {
    let mut style = Style::default();
    style.size.width = relative(1.).into();
    window.request_measured_layout(style, move |known, available, window, _cx| {
        let width = known.width.unwrap_or_else(|| match available.width {
            AvailableSpace::Definite(w) => w,
            AvailableSpace::MinContent | AvailableSpace::MaxContent => px(1.0),
        });
        let height = spec_height(f32::from(width).max(1.0), window);
        size(width, px(height))
    })
}

fn stamp_host_ids(node: &mut poodle_node::Node, element_id: Option<&str>) {
    if node.a11y.role == Some(NodeRole::Slider) {
        if let Some(id) = element_id {
            if node.id.is_none() {
                node.id = Some(id.to_owned());
            }
        }
    }
    for child in &mut node.children {
        stamp_host_ids(child, element_id);
    }
}

struct BlockSliderLayoutHost {
    spec: SliderSpec,
    theme: Arc<dyn ThemeProvider + Send + Sync>,
    handlers: SliderHandlers,
    min_height: f32,
    element_id: Option<String>,
}

struct BlockRangeSliderLayoutHost {
    spec: RangeSliderSpec,
    theme: Arc<dyn ThemeProvider + Send + Sync>,
    handlers: RangeSliderHandlers,
    min_height: f32,
    element_id: Option<String>,
}

struct PrepaintState {
    child: AnyElement,
}

impl BlockSliderLayoutHost {
    fn build_child(&self, capsule_span: f32, window: &Window, font_size_px: f32) -> AnyElement {
        let root = RenderContext::new(self.theme.as_ref());
        let ctx = root.with_block_layout(
            capsule_span,
            measure_for_texts(window, slider_visible_texts(&self.spec), font_size_px),
        );
        let mut node = slider(&self.spec, &ctx, &self.handlers);
        if node.id.is_none() {
            node.id = Some(BLOCK_SLIDER_HOST_ID.to_owned());
        }
        stamp_host_ids(&mut node, self.element_id.as_deref());
        to_gpui(&node)
    }
}

impl BlockRangeSliderLayoutHost {
    fn build_child(&self, capsule_span: f32, window: &Window, font_size_px: f32) -> AnyElement {
        let root = RenderContext::new(self.theme.as_ref());
        let ctx = root.with_block_layout(
            capsule_span,
            measure_for_texts(window, range_visible_texts(&self.spec), font_size_px),
        );
        let mut node = range_slider(
            &self.spec,
            &ctx,
            RangeSliderHandlers {
                on_change: self.handlers.on_change.clone(),
                on_value_commit: self.handlers.on_value_commit.clone(),
            },
        );
        if let Some(id) = &self.element_id {
            node.id = Some(id.clone());
        } else if node.id.is_none() {
            node.id = Some(BLOCK_RANGE_SLIDER_HOST_ID.to_owned());
        }
        to_gpui(&node)
    }
}

impl IntoElement for BlockSliderLayoutHost {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl IntoElement for BlockRangeSliderLayoutHost {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for BlockSliderLayoutHost {
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
        let spec = self.spec.clone();
        let surface = self.min_height;
        (
            request_host_layout(window, move |span, window| {
                let font_size_px = presentation::rem_to_px(slider_block::font_size_rem(
                    spec.size.unwrap_or(poodle_specs::ControlSize::Md),
                ));
                slider_host_height(&spec, surface, span, window, font_size_px)
            }),
            (),
        )
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
        let font_size_px = presentation::rem_to_px(slider_block::font_size_rem(
            self.spec.size.unwrap_or(poodle_specs::ControlSize::Md),
        ));
        let mut child = self.build_child(capsule_span, window, font_size_px);
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

impl Element for BlockRangeSliderLayoutHost {
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
        let spec = self.spec.clone();
        let surface = self.min_height;
        (
            request_host_layout(window, move |span, window| {
                let font_size_px = presentation::rem_to_px(slider_block::font_size_rem(
                    spec.size.unwrap_or(poodle_specs::ControlSize::Md),
                ));
                range_host_height(&spec, surface, span, window, font_size_px)
            }),
            (),
        )
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
        let font_size_px = presentation::rem_to_px(slider_block::font_size_rem(
            self.spec.size.unwrap_or(poodle_specs::ControlSize::Md),
        ));
        let mut child = self.build_child(capsule_span, window, font_size_px);
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

pub fn block_slider_element(
    spec: SliderSpec,
    theme: Arc<dyn ThemeProvider + Send + Sync>,
    handlers: SliderHandlers,
    min_height: f32,
    element_id: Option<String>,
) -> AnyElement {
    BlockSliderLayoutHost {
        spec,
        theme,
        handlers,
        min_height,
        element_id,
    }
    .into_any_element()
}

pub fn block_range_slider_element(
    spec: RangeSliderSpec,
    theme: Arc<dyn ThemeProvider + Send + Sync>,
    handlers: RangeSliderHandlers,
    min_height: f32,
    element_id: Option<String>,
) -> AnyElement {
    BlockRangeSliderLayoutHost {
        spec,
        theme,
        handlers,
        min_height,
        element_id,
    }
    .into_any_element()
}

pub fn block_slider_min_height(spec: &SliderSpec) -> f32 {
    let size = spec.size.unwrap_or(poodle_specs::ControlSize::Md);
    let capsule = presentation::rem_to_px(slider_block::capsule_height_rem(size));
    poodle_headless::slider::SLIDER_BLOCK_HIT_PX.max(capsule)
}

pub fn block_range_slider_min_height(spec: &RangeSliderSpec) -> f32 {
    let size = spec.size.unwrap_or(poodle_specs::ControlSize::Md);
    let capsule = presentation::rem_to_px(slider_block::capsule_height_rem(size));
    poodle_headless::slider::SLIDER_BLOCK_HIT_PX.max(capsule)
}
