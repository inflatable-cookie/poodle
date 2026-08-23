use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, ResizeHandle};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_render::ResizePhase;
use poodle_specs::{EyebrowSpec, Orientation, ResizeHandleSpec};
use std::sync::{Arc, Mutex};

const HORIZONTAL_LEFT_KEY: &str = "resize-handle-h-left";
const VERTICAL_TOP_KEY: &str = "resize-handle-v-top";
const MIN_HORIZONTAL_PX: f32 = 48.0;
const MAX_HORIZONTAL_PX: f32 = 280.0;
const MIN_VERTICAL_PX: f32 = 40.0;
const MAX_VERTICAL_PX: f32 = 120.0;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let panel_bg = theme.resolve_color("color.background.panel");
    let border_subtle = theme.resolve_color("color.border.subtle");
    let left_px = stored_px(state, HORIZONTAL_LEFT_KEY, 120.0, MIN_HORIZONTAL_PX, MAX_HORIZONTAL_PX);
    let top_px = stored_px(state, VERTICAL_TOP_KEY, 80.0, MIN_VERTICAL_PX, MAX_VERTICAL_PX);
    let _ = cx;

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(example_group(
            theme,
            "Horizontal split (vertical handle \u{2014} drag left/right)",
            horizontal_split(
                theme,
                text_secondary,
                panel_bg,
                border_subtle,
                pane(text_secondary, panel_bg, "Left")
                    .flex_none()
                    .w(px(left_px)),
                ResizeHandleSpec::new(HORIZONTAL_LEFT_KEY)
                    .with_orientation(Orientation::Horizontal)
                    .with_aria_label("Resize horizontal")
                    .with_aria_value_now(left_px)
                    .with_aria_value_min(MIN_HORIZONTAL_PX)
                    .with_aria_value_max(MAX_HORIZONTAL_PX),
                Some(resize_delta_handler(
                    HORIZONTAL_LEFT_KEY,
                    Arc::clone(&state.node_events),
                    left_px,
                    MIN_HORIZONTAL_PX,
                    MAX_HORIZONTAL_PX,
                )),
                pane(text_secondary, panel_bg, "Right").flex_1(),
            ),
        ))
        .child(example_group(
            theme,
            "Vertical split (horizontal handle \u{2014} drag up/down)",
            vertical_split(
                theme,
                text_secondary,
                panel_bg,
                border_subtle,
                pane(text_secondary, panel_bg, "Top")
                    .flex_none()
                    .h(px(top_px)),
                ResizeHandleSpec::new(VERTICAL_TOP_KEY)
                    .with_orientation(Orientation::Vertical)
                    .with_aria_label("Resize vertical")
                    .with_aria_value_now(top_px)
                    .with_aria_value_min(MIN_VERTICAL_PX)
                    .with_aria_value_max(MAX_VERTICAL_PX),
                Some(resize_delta_handler(
                    VERTICAL_TOP_KEY,
                    Arc::clone(&state.node_events),
                    top_px,
                    MIN_VERTICAL_PX,
                    MAX_VERTICAL_PX,
                )),
                pane(text_secondary, panel_bg, "Bottom").flex_1(),
            ),
        ))
        .child(example_group(
            theme,
            "Disabled (horizontal split)",
            horizontal_split(
                theme,
                text_secondary,
                panel_bg,
                border_subtle,
                pane(text_secondary, panel_bg, "Left").flex_1(),
                ResizeHandleSpec::new("resize-handle:disabled-horizontal")
                    .with_orientation(Orientation::Horizontal)
                    .with_disabled(true)
                    .with_aria_label("Disabled resize"),
                None,
                pane(text_secondary, panel_bg, "Right").flex_1(),
            ),
        ))
        .child(example_group(
            theme,
            "Disabled (vertical split)",
            vertical_split(
                theme,
                text_secondary,
                panel_bg,
                border_subtle,
                pane(text_secondary, panel_bg, "Top").flex_1(),
                ResizeHandleSpec::new("resize-handle:disabled-vertical")
                    .with_orientation(Orientation::Vertical)
                    .with_disabled(true)
                    .with_aria_label("Disabled resize vertical"),
                None,
                pane(text_secondary, panel_bg, "Bottom").flex_1(),
            ),
        ))
}

fn example_group(theme: &GpuiThemeProvider, title: &str, content: Div) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(EyebrowSpec::new().with_content(title), theme))
        .child(content)
}

fn stored_px(state: &AppState, key: &str, default: f32, min: f32, max: f32) -> f32 {
    match state.specimens.selected(key) {
        0 => default,
        px => px as f32,
    }
    .clamp(min, max)
}

fn resize_delta_handler(
    key: &'static str,
    queue: Arc<Mutex<Vec<NodeSpecimenEvent>>>,
    current_px: f32,
    min: f32,
    max: f32,
) -> Arc<dyn Fn(ResizePhase, f32) + Send + Sync> {
    use std::sync::atomic::{AtomicU32, Ordering};

    let live = Arc::new(AtomicU32::new(current_px.to_bits()));
    Arc::new(move |phase, delta| match phase {
        ResizePhase::Start => {
            live.store(current_px.to_bits(), Ordering::SeqCst);
        }
        ResizePhase::Move => {
            let current = f32::from_bits(live.load(Ordering::SeqCst));
            let next = (current + delta).clamp(min, max);
            live.store(next.to_bits(), Ordering::SeqCst);
            queue.lock().unwrap().push(NodeSpecimenEvent::Select {
                key: key.to_string(),
                index: next.round() as usize,
            });
        }
        ResizePhase::End => {}
    })
}

fn horizontal_split(
    theme: &GpuiThemeProvider,
    text_secondary: poodle_tokens::typed::ColorValue,
    panel_bg: poodle_tokens::typed::ColorValue,
    border_subtle: poodle_tokens::typed::ColorValue,
    leading: Div,
    handle_spec: ResizeHandleSpec,
    on_resize: Option<Arc<dyn Fn(ResizePhase, f32) + Send + Sync>>,
    trailing: Div,
) -> Div {
    let _ = (text_secondary, panel_bg);
    let mut handle = ResizeHandle::from_spec(handle_spec, theme);
    if let Some(handler) = on_resize {
        handle = handle.on_resize(handler);
    }

    div()
        .flex()
        .items_start()
        .h(px(96.0))
        .border_1()
        .border_color(color_to_hsla(border_subtle))
        .rounded(px(6.0))
        .overflow_hidden()
        .child(leading)
        .child(handle)
        .child(trailing)
}

fn vertical_split(
    theme: &GpuiThemeProvider,
    text_secondary: poodle_tokens::typed::ColorValue,
    panel_bg: poodle_tokens::typed::ColorValue,
    border_subtle: poodle_tokens::typed::ColorValue,
    leading: Div,
    handle_spec: ResizeHandleSpec,
    on_resize: Option<Arc<dyn Fn(ResizePhase, f32) + Send + Sync>>,
    trailing: Div,
) -> Div {
    let _ = (text_secondary, panel_bg);
    let mut handle = ResizeHandle::from_spec(handle_spec, theme);
    if let Some(handler) = on_resize {
        handle = handle.on_resize(handler);
    }

    div()
        .flex()
        .flex_col()
        .h(px(160.0))
        .border_1()
        .border_color(color_to_hsla(border_subtle))
        .rounded(px(6.0))
        .overflow_hidden()
        .child(leading)
        .child(handle)
        .child(trailing)
}

fn pane(
    text_color: poodle_tokens::typed::ColorValue,
    bg_color: poodle_tokens::typed::ColorValue,
    label: &str,
) -> Div {
    div()
        .flex()
        .items_center()
        .justify_center()
        .text_sm()
        .text_color(color_to_hsla(text_color))
        .bg(color_to_hsla(bg_color).opacity(0.5))
        .child(label.to_string())
}

#[cfg(test)]
mod interaction_tests {
    use super::{
        resize_delta_handler, HORIZONTAL_LEFT_KEY, MAX_HORIZONTAL_PX, MIN_HORIZONTAL_PX,
        VERTICAL_TOP_KEY,
    };
    use crate::app_state::NodeSpecimenEvent;
    use poodle_gpui::GpuiThemeProvider;
    use poodle_node::{NodeDragPhase, NodeKey, NodeModifiers};
    use poodle_render::{context::RenderContext, resize_handle, ResizePhase};
    use poodle_specs::{Orientation, ResizeHandleSpec};
    use std::sync::{Arc, Mutex};

    fn drag_move(node: &poodle_node::Node, delta_x: f32) {
        let drag = node
            .find(&|n| n.interaction.on_drag.is_some())
            .and_then(|n| n.interaction.on_drag.as_ref())
            .expect("interactive handle exposes on_drag — without one the section is inert");
        drag(&poodle_node::NodeDragEvent {
            phase: NodeDragPhase::Start,
            delta_x: 0.0,
            delta_y: 0.0,
        });
        drag(&poodle_node::NodeDragEvent {
            phase: NodeDragPhase::Move,
            delta_x,
            delta_y: 0.0,
        });
    }

    #[test]
    fn move_emits_resize_delta() {
        let events: Arc<Mutex<Vec<NodeSpecimenEvent>>> = Arc::new(Mutex::new(Vec::new()));
        let handler = resize_delta_handler(
            HORIZONTAL_LEFT_KEY,
            Arc::clone(&events),
            120.0,
            48.0,
            280.0,
        );
        handler(ResizePhase::Start, 0.0);
        handler(ResizePhase::Move, 16.0);
        let queue = events.lock().unwrap();
        match &queue[0] {
            NodeSpecimenEvent::Select { key, index } => {
                assert_eq!(key.as_str(), HORIZONTAL_LEFT_KEY);
                assert_eq!(*index, 136);
            }
            _ => panic!("expected Select event"),
        }
    }

    #[test]
    fn rendered_handle_carries_drag_handler() {
        let events: Arc<Mutex<Vec<NodeSpecimenEvent>>> = Arc::new(Mutex::new(Vec::new()));
        let theme = GpuiThemeProvider::new();
        let handler = resize_delta_handler(
            HORIZONTAL_LEFT_KEY,
            Arc::clone(&events),
            120.0,
            48.0,
            280.0,
        );
        let node = resize_handle(
            &ResizeHandleSpec::new(HORIZONTAL_LEFT_KEY).with_orientation(Orientation::Horizontal),
            &RenderContext::new(&theme),
            Some(handler),
        );
        drag_move(&node, 8.0);
        let queue = events.lock().unwrap();
        match &queue[0] {
            NodeSpecimenEvent::Select { key, index } => {
                assert_eq!(key.as_str(), HORIZONTAL_LEFT_KEY);
                assert_eq!(*index, 128);
            }
            _ => panic!("expected Select event"),
        }
    }

    /// The page's own handler, wired to the page's own spec, driven through
    /// the node's key seam: an axis arrow moves the pane by the contract's
    /// step, and Home lands on the specimen's own minimum. Without this the
    /// native page teaches drag and stays silent about the keyboard.
    #[test]
    fn keyboard_steps_move_the_specimen_pane() {
        let events: Arc<Mutex<Vec<NodeSpecimenEvent>>> = Arc::new(Mutex::new(Vec::new()));
        let node = resize_handle(
            &ResizeHandleSpec::new(HORIZONTAL_LEFT_KEY)
                .with_orientation(Orientation::Horizontal)
                .with_aria_label("Resize horizontal")
                .with_aria_value_now(120.0)
                .with_aria_value_min(MIN_HORIZONTAL_PX)
                .with_aria_value_max(MAX_HORIZONTAL_PX),
            &RenderContext::new(&GpuiThemeProvider::new()),
            Some(resize_delta_handler(
                HORIZONTAL_LEFT_KEY,
                Arc::clone(&events),
                120.0,
                MIN_HORIZONTAL_PX,
                MAX_HORIZONTAL_PX,
            )),
        );
        let keys = node
            .interaction
            .on_key
            .as_ref()
            .expect("the enabled specimen handle routes keys");
        keys(NodeKey::ArrowRight, NodeModifiers::default());
        keys(NodeKey::Home, NodeModifiers::default());

        let queue = events.lock().unwrap();
        let widths: Vec<usize> = queue
            .iter()
            .map(|event| match event {
                NodeSpecimenEvent::Select { key, index } => {
                    assert_eq!(key.as_str(), HORIZONTAL_LEFT_KEY);
                    *index
                }
                _ => panic!("expected Select event"),
            })
            .collect();
        assert_eq!(widths, [128, MIN_HORIZONTAL_PX as usize]);
    }

    /// The specimen's value declaration is the pane it actually draws, not a
    /// default range the current value falls outside of.
    #[test]
    fn the_specimen_declares_the_pane_it_draws() {
        let node = resize_handle(
            &ResizeHandleSpec::new(HORIZONTAL_LEFT_KEY)
                .with_orientation(Orientation::Horizontal)
                .with_aria_label("Resize horizontal")
                .with_aria_value_now(120.0)
                .with_aria_value_min(MIN_HORIZONTAL_PX)
                .with_aria_value_max(MAX_HORIZONTAL_PX),
            &RenderContext::new(&GpuiThemeProvider::new()),
            None,
        );
        assert_eq!(node.a11y.value, Some(120.0));
        assert_eq!(node.a11y.value_min, Some(48.0));
        assert_eq!(node.a11y.value_max, Some(280.0));
        assert!(node.interaction.focusable);
    }

    /// The page's four sections are four instances. Each scope is the key the
    /// page already stores its pane under, so backend focus state and specimen
    /// state cannot drift apart.
    #[test]
    fn every_section_carries_its_own_backend_identity() {
        let build = |scope: &str, orientation| {
            resize_handle(
                &ResizeHandleSpec::new(scope).with_orientation(orientation),
                &RenderContext::new(&GpuiThemeProvider::new()),
                None,
            )
            .runtime_id
            .expect("the handle identifies itself")
        };
        let ids = [
            build(HORIZONTAL_LEFT_KEY, Orientation::Horizontal),
            build(VERTICAL_TOP_KEY, Orientation::Vertical),
            build("resize-handle:disabled-horizontal", Orientation::Horizontal),
            build("resize-handle:disabled-vertical", Orientation::Vertical),
        ];
        let unique: std::collections::BTreeSet<&String> = ids.iter().collect();
        assert_eq!(unique.len(), ids.len(), "four sections, four identities");
        assert!(ids[0].ends_with(HORIZONTAL_LEFT_KEY));
    }

    /// A disabled section stays static: no keys, no drag, no focus stop.
    #[test]
    fn a_disabled_specimen_section_stays_inert() {
        let node = resize_handle(
            &ResizeHandleSpec::new("resize-handle:disabled-horizontal")
                .with_orientation(Orientation::Horizontal)
                .with_disabled(true)
                .with_aria_label("Disabled resize"),
            &RenderContext::new(&GpuiThemeProvider::new()),
            None,
        );
        assert!(node.interaction.on_key.is_none());
        assert!(node.interaction.on_drag.is_none());
        assert!(!node.interaction.focusable);
        assert!(node.interaction.disabled);
    }
}
