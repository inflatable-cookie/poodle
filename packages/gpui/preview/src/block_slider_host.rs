//! GPUI composition for block Slider and RangeSlider.
//!
//! Construction stays `Spec + RenderContext -> poodle-render -> Node`. The
//! node backend only waits for parent width, shapes the visible strings, and
//! interprets the rebuilt tree.

use std::sync::Arc;

use gpui::AnyElement;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_node_backend::{measured_node_element, ShapedAdvance};
use poodle_headless::slider::{resolved_range_text, resolved_visible_text};
use poodle_node::{Node, NodeRole};
use poodle_render::{
    presentation, range_slider, slider, slider_block, BlockTextMeasure, RangeSliderHandlers,
    RenderContext, SliderHandlers,
};
use poodle_specs::{ControlSize, RangeSliderSpec, SliderSpec};

const BLOCK_SLIDER_HOST_ID: &str = "block-slider-host";
const BLOCK_RANGE_SLIDER_HOST_ID: &str = "block-range-slider-host";

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

fn font_size_px(size: Option<ControlSize>) -> f32 {
    presentation::rem_to_px(slider_block::font_size_rem(size.unwrap_or(ControlSize::Md)))
}

#[allow(dead_code)]
pub fn block_slider_surface_height(spec: &SliderSpec) -> f32 {
    let capsule = presentation::rem_to_px(slider_block::capsule_height_rem(
        spec.size.unwrap_or(ControlSize::Md),
    ));
    poodle_headless::slider::SLIDER_BLOCK_HIT_PX.max(capsule)
}

#[allow(dead_code)]
pub fn block_range_slider_surface_height(spec: &RangeSliderSpec) -> f32 {
    let capsule = presentation::rem_to_px(slider_block::capsule_height_rem(
        spec.size.unwrap_or(ControlSize::Md),
    ));
    poodle_headless::slider::SLIDER_BLOCK_HIT_PX.max(capsule)
}

fn as_block_measure(advance: ShapedAdvance) -> BlockTextMeasure {
    Arc::new(move |text: &str, _font| advance(text))
}

fn stamp_slider_ids(node: &mut Node, element_id: Option<&str>) {
    if node.a11y.role == Some(NodeRole::Slider) {
        if let Some(id) = element_id {
            if node.id.is_none() {
                node.id = Some(id.to_owned());
            }
        }
    }
    for child in &mut node.children {
        stamp_slider_ids(child, element_id);
    }
}

pub fn slider_element(
    spec: SliderSpec,
    theme: GpuiThemeProvider,
    handlers: SliderHandlers,
    element_id: Option<String>,
) -> AnyElement {
    let texts = slider_visible_texts(&spec);
    let font_size_px = font_size_px(spec.size);
    let theme: Arc<dyn ThemeProvider + Send + Sync> = Arc::new(theme);
    measured_node_element(
        texts,
        font_size_px,
        Arc::new(move |span, advance| {
            let root = RenderContext::new(theme.as_ref());
            let ctx = root.with_block_layout(span, as_block_measure(advance));
            let mut node = slider(&spec, &ctx, &handlers);
            if node.id.is_none() {
                node.id = Some(BLOCK_SLIDER_HOST_ID.to_owned());
            }
            stamp_slider_ids(&mut node, element_id.as_deref());
            node
        }),
    )
}

pub fn range_slider_element(
    spec: RangeSliderSpec,
    theme: GpuiThemeProvider,
    handlers: RangeSliderHandlers,
    element_id: Option<String>,
) -> AnyElement {
    let texts = range_visible_texts(&spec);
    let font_size_px = font_size_px(spec.size);
    let theme: Arc<dyn ThemeProvider + Send + Sync> = Arc::new(theme);
    measured_node_element(
        texts,
        font_size_px,
        Arc::new(move |span, advance| {
            let root = RenderContext::new(theme.as_ref());
            let ctx = root.with_block_layout(span, as_block_measure(advance));
            let mut node = range_slider(
                &spec,
                &ctx,
                RangeSliderHandlers {
                    on_change: handlers.on_change.clone(),
                    on_value_commit: handlers.on_value_commit.clone(),
                },
            );
            if let Some(id) = &element_id {
                node.id = Some(id.clone());
            } else if node.id.is_none() {
                node.id = Some(BLOCK_RANGE_SLIDER_HOST_ID.to_owned());
            }
            node
        }),
    )
}
