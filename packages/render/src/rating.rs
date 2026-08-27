//! Rating — ordinal star selection.
//!
//! Contract: `docs/contracts/components/rating.md`
//!
//! Whole-step mode uses RadioGroup + RadioButton with roving focus that moves
//! without selecting. Fractional mode uses one Slider root; star targets scrub
//! pointer fractions through shared pure math. The host owns post-render state.

use std::sync::Arc;

use poodle_headless::rating::{
    normalize_rating_value, rating_keyboard_step, rating_pointer_value, rating_select_value,
    rating_value_text, resolve_rating_step,
};
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, FocusRing, LayoutDirection, LayoutOverflow,
    LayoutSizing, MainAxisAlignment, Node, NodeKey, NodeModifiers, NodePosition, NodeRole,
    NodeToggled, ScrubAxis, ScrubPhase,
};
use poodle_specs::{ControlDensity, ControlSize, RatingSpec};

use crate::color::with_alpha;
use crate::context::RenderContext;
use crate::presentation::{control_height_rem, rem_to_px};

/// Host-owned native interaction for one Rating instance.
///
/// `instance_id` is the lifetime-stable scope. It is not a web public prop, and
/// the renderer never invents one from render order or selected value.
#[derive(Clone)]
pub struct RatingHandlers {
    pub instance_id: String,
    pub on_change: Option<Arc<dyn Fn(Option<f64>) + Send + Sync>>,
}

impl RatingHandlers {
    pub fn new(instance_id: impl Into<String>) -> Self {
        Self {
            instance_id: instance_id.into(),
            on_change: None,
        }
    }

    pub fn on_change(mut self, handler: Arc<dyn Fn(Option<f64>) + Send + Sync>) -> Self {
        self.on_change = Some(handler);
        self
    }
}

/// Per-size glyph font-size in rem (contract §8 size table).
fn glyph_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm => 0.875,
        ControlSize::Md => 1.0,
        ControlSize::Lg => 1.125,
        ControlSize::Xl => 1.25,
    }
}

/// Per-density inter-item gap in rem (contract §8).
fn item_gap_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.0625,
        ControlDensity::Default => 0.125,
        ControlDensity::Comfortable => 0.25,
    }
}

fn item_focus_id(instance_scope: &str, value: u8) -> String {
    format!("rating:{instance_scope}:item:{value}")
}

fn item_test_id(value: u8) -> String {
    format!("rating-item-{value}")
}

fn emit_change(
    next_raw: f64,
    current: Option<f64>,
    allow_clear: bool,
    item_count: f64,
    step: f64,
    on_change: &Option<Arc<dyn Fn(Option<f64>) + Send + Sync>>,
) {
    let Some(handler) = on_change else {
        return;
    };

    if !allow_clear && current == Some(next_raw) {
        return;
    }

    let selected = rating_select_value(next_raw, current, allow_clear);
    let payload = match selected {
        None => None,
        Some(raw) => normalize_rating_value(Some(raw), item_count, step),
    };
    handler(payload);
}

fn whole_step_key_handler(
    value: u8,
    item_count: u8,
    instance_scope: String,
) -> Arc<dyn Fn(NodeKey, NodeModifiers) -> Option<String> + Send + Sync> {
    Arc::new(move |key, _modifiers| {
        let last = item_count;
        let focus_target = match key {
            NodeKey::ArrowRight | NodeKey::ArrowUp => {
                Some(if value == last { 1 } else { value + 1 })
            }
            NodeKey::ArrowLeft | NodeKey::ArrowDown => {
                Some(if value == 1 { last } else { value - 1 })
            }
            NodeKey::Home => Some(1),
            NodeKey::End => Some(last),
            _ => None,
        }?;
        if focus_target == value {
            return None;
        }
        Some(item_focus_id(&instance_scope, focus_target))
    })
}

fn fractional_key_handler(
    current: Option<f64>,
    allow_clear: bool,
    item_count: f64,
    step: f64,
    on_change: Option<Arc<dyn Fn(Option<f64>) + Send + Sync>>,
) -> Arc<dyn Fn(NodeKey, NodeModifiers) -> Option<String> + Send + Sync> {
    let min_selectable = if allow_clear { 0.0 } else { step };
    Arc::new(move |key, _modifiers| {
        let Some(handler) = &on_change else {
            return None;
        };
        match key {
            NodeKey::ArrowRight | NodeKey::ArrowUp => {
                let base = normalize_rating_value(current.or(Some(0.0)), item_count, step)
                    .unwrap_or(0.0);
                let next = rating_keyboard_step(base, 1, step, item_count, min_selectable);
                handler(normalize_rating_value(Some(next), item_count, step));
            }
            NodeKey::ArrowLeft | NodeKey::ArrowDown => {
                let base = normalize_rating_value(current.or(Some(0.0)), item_count, step)
                    .unwrap_or(0.0);
                let next = rating_keyboard_step(base, -1, step, item_count, min_selectable);
                handler(normalize_rating_value(Some(next), item_count, step));
            }
            NodeKey::Home => {
                handler(normalize_rating_value(Some(min_selectable), item_count, step));
            }
            NodeKey::End => {
                handler(normalize_rating_value(Some(item_count), item_count, step));
            }
            NodeKey::Space if allow_clear && current.is_some() => {
                handler(None);
            }
            _ => return None,
        }
        None
    })
}

fn star_glyph(
    spec: &RatingSpec,
    index: u8,
    glyph_px: f32,
    active: ColorValue,
    inactive: ColorValue,
) -> Node {
    let ratio = spec.fill_ratio(index) as f32;
    let fill_w = (glyph_px * ratio).clamp(0.0, glyph_px);

    let mut base = Node::icon("star", glyph_px);
    base.style.descriptor.text_color = Some(inactive);

    let mut glyph = Node::container();
    glyph.position = NodePosition::Relative;
    {
        let s = &mut glyph.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.width = LayoutSizing::Fixed(glyph_px);
        s.descriptor.layout.height = LayoutSizing::Fixed(glyph_px);
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
    }
    let mut glyph = glyph.child(base);

    if fill_w > 0.0 {
        let mut fill = Node::container();
        fill.position = NodePosition::Absolute {
            top: Some(0.0),
            left: Some(0.0),
            right: None,
            bottom: None,
        };
        {
            let s = &mut fill.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.height = LayoutSizing::Fixed(glyph_px);
            s.descriptor.layout.width = LayoutSizing::Fixed(fill_w);
            s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
            s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        }
        let mut filled = Node::icon("star", glyph_px);
        filled.style.descriptor.text_color = Some(active);
        glyph = glyph.child(fill.child(filled));
    }

    glyph
}

pub fn rating(spec: &RatingSpec, ctx: &RenderContext<'_>, handlers: RatingHandlers) -> Node {
    let active = ctx.theme().resolve_color(spec.active_color_token());
    let inactive_base = ctx.theme().resolve_color(spec.inactive_color_token());
    let inactive = with_alpha(inactive_base, inactive_base.3 * spec.inactive_color_alpha());

    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let glyph_px = rem_to_px(glyph_font_rem(effective_size)) * 1.125;
    let item_px = rem_to_px(control_height_rem(effective_size));
    let gap = rem_to_px(item_gap_rem(density));
    let item_count = spec.item_count();
    let step = resolve_rating_step(spec.step);
    let fractional = step < 1.0;
    let current = spec.current_value();
    let instance_scope = handlers.instance_id.as_str();
    let focus_ring = FocusRing {
        color: ctx.theme().resolve_color(spec.focus_ring_token()),
        width: ctx.theme().resolve_border_width("border.width.focus"),
        offset: rem_to_px(0.125),
    };
    let disabled_opacity = ctx.theme().resolve_opacity("state.opacity.disabled");

    let tab_stop: Option<u8> = if spec.is_disabled || fractional {
        None
    } else {
        let selected = (1..=item_count).find(|&value| current == Some(value as f64));
        Some(selected.unwrap_or(1))
    };

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = gap;
    }

    for i in 0..item_count {
        let value = i + 1;
        let glyph = star_glyph(spec, i, glyph_px, active, inactive);

        let mut target = Node::container();
        {
            let s = &mut target.style;
            s.descriptor.layout.width = LayoutSizing::Fixed(item_px);
            s.descriptor.layout.height = LayoutSizing::Fixed(item_px);
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        }
        let mut target = target.child(glyph);
        target.id = Some(item_test_id(value));
        target.runtime_id = Some(item_focus_id(instance_scope, value));

        if fractional {
            // Accessibility-hidden pointer targets: no role, no tab stop.
            target.interaction.focusable = false;
            target.a11y.tab_index = Some(-1);
            if !spec.is_disabled {
                if handlers.on_change.is_some() {
                    target.style.descriptor.cursor = CursorHint::Pointer;
                    target.interaction.scrub_axis = ScrubAxis::Horizontal;
                    let on_change = handlers.on_change.clone();
                    let allow_clear = spec.allow_clear;
                    let count = item_count as f64;
                    let index = i as f64;
                    target.interaction.on_scrub = Some(Arc::new(move |fraction: f32, phase| {
                        let next =
                            rating_pointer_value(fraction as f64, index, step, count);
                        match phase {
                            ScrubPhase::Release => {}
                            ScrubPhase::Press => {
                                emit_change(next, current, allow_clear, count, step, &on_change);
                            }
                            ScrubPhase::Drag => {
                                // Live drag never clears; only moves to a new snapped value.
                                if let Some(handler) = &on_change {
                                    let payload =
                                        normalize_rating_value(Some(next), count, step);
                                    if payload != current {
                                        handler(payload);
                                    }
                                }
                            }
                        }
                    }));
                }
            }
        } else {
            let is_selected = current == Some(value as f64);
            target.a11y.role = Some(NodeRole::RadioButton);
            target.a11y.label = Some(format!("{value} of {item_count}"));
            target.a11y.selected = Some(is_selected);
            target.a11y.toggled = Some(if is_selected {
                NodeToggled::True
            } else {
                NodeToggled::False
            });

            if spec.is_disabled {
                target.interaction.disabled = true;
                target.interaction.focusable = false;
                target.a11y.tab_index = Some(-1);
            } else {
                target.interaction.focusable = true;
                target.a11y.tab_index = Some(if tab_stop == Some(value) { 0 } else { -1 });
                target.style.focus_ring = Some(focus_ring);
                target.style.descriptor.cursor = CursorHint::Pointer;

                if let Some(handler) = &handlers.on_change {
                    let handler = Arc::clone(handler);
                    let allow_clear = spec.allow_clear;
                    let count = item_count as f64;
                    target.interaction.on_activate = Some(Arc::new(move || {
                        emit_change(
                            value as f64,
                            current,
                            allow_clear,
                            count,
                            step,
                            &Some(handler.clone()),
                        );
                    }));
                }

                target.interaction.on_key = Some(whole_step_key_handler(
                    value,
                    item_count,
                    instance_scope.to_string(),
                ));
            }
        }

        el = el.child(target);
    }

    if spec.is_disabled {
        el.style.descriptor.opacity = disabled_opacity;
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            el.a11y.label = Some(label.to_string());
        }
    }

    if fractional {
        el.a11y.role = Some(NodeRole::Slider);
        el.a11y.value_min = Some(0.0);
        el.a11y.value_max = Some(item_count as f64);
        el.a11y.value = Some(current.unwrap_or(0.0));
        el.a11y.value_text = Some(rating_value_text(current, item_count));
        el.a11y.orientation = Some("horizontal".to_owned());
        if spec.is_disabled {
            el.interaction.disabled = true;
            el.interaction.focusable = false;
            el.a11y.tab_index = Some(-1);
        } else {
            el.interaction.focusable = true;
            el.a11y.tab_index = Some(0);
            el.style.focus_ring = Some(focus_ring);
            el.runtime_id = Some(format!("rating:{}:root", instance_scope));
            el.id = Some("rating-root".to_owned());
            el.interaction.on_key = Some(fractional_key_handler(
                current,
                spec.allow_clear,
                item_count as f64,
                step,
                handlers.on_change.clone(),
            ));
            // Enter clears via on_submit (NodeKey has no Enter). Do not use
            // on_activate: a focused slider root would also clear after star
            // scrub when Press rebuilds mid-gesture and the click lands.
            if spec.allow_clear {
                if let Some(handler) = handlers.on_change.clone() {
                    el.interaction.on_submit = Some(Arc::new(move || {
                        if current.is_some() {
                            handler(None);
                        }
                    }));
                }
            }
        }
    } else {
        el.a11y.role = Some(NodeRole::RadioGroup);
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn render(spec: &RatingSpec, handlers: RatingHandlers) -> Node {
        rating(spec, &RenderContext::new(&theme()), handlers)
    }

    fn item<'a>(node: &'a Node, value: u8) -> &'a Node {
        let id = item_test_id(value);
        node.find(&|n| n.id.as_deref() == Some(id.as_str()))
            .unwrap_or_else(|| panic!("item {value}"))
    }

    #[test]
    fn default_step_is_fractional_slider() {
        let node = render(&RatingSpec::new().with_value(3.5), RatingHandlers::new("r"));
        assert_eq!(node.a11y.role, Some(NodeRole::Slider));
        assert_eq!(node.a11y.value, Some(3.5));
        assert_eq!(
            node.a11y.value_text.as_deref(),
            Some("3.5 out of 5")
        );
        assert!(item(&node, 1).a11y.role.is_none());
        assert!(!item(&node, 1).interaction.focusable);
    }

    #[test]
    fn whole_step_selects_exact_radio_only() {
        let node = render(
            &RatingSpec::new().with_value(3.0).with_step(1.0),
            RatingHandlers::new("r"),
        );
        assert_eq!(node.a11y.role, Some(NodeRole::RadioGroup));
        assert_eq!(item(&node, 3).a11y.selected, Some(true));
        assert_eq!(item(&node, 3).a11y.tab_index, Some(0));
        assert_eq!(item(&node, 2).a11y.selected, Some(false));
        assert_eq!(item(&node, 2).a11y.tab_index, Some(-1));
        assert_eq!(item(&node, 1).a11y.toggled, Some(NodeToggled::False));
    }

    #[test]
    fn whole_step_arrows_move_focus_without_selecting() {
        let seen: Arc<Mutex<Vec<Option<f64>>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let node = render(
            &RatingSpec::new().with_value(2.0).with_step(1.0),
            RatingHandlers::new("r").on_change(Arc::new(move |v| {
                sink.lock().unwrap().push(v);
            })),
        );
        let keys = item(&node, 2).interaction.on_key.as_ref().unwrap();
        let mods = NodeModifiers::default();
        assert_eq!(
            keys(NodeKey::ArrowRight, mods),
            Some(item_focus_id("r", 3))
        );
        assert!(seen.lock().unwrap().is_empty());
        (item(&node, 3).interaction.on_activate.as_ref().unwrap())();
        assert_eq!(seen.lock().unwrap().as_slice(), &[Some(3.0)]);
        seen.lock().unwrap().clear();
        (item(&node, 2).interaction.on_activate.as_ref().unwrap())();
        assert!(
            seen.lock().unwrap().is_empty(),
            "re-selecting the current value without clear is inert"
        );
    }

    #[test]
    fn clear_on_reselect_and_disabled_inertia() {
        let seen: Arc<Mutex<Vec<Option<f64>>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let node = render(
            &RatingSpec::new()
                .with_value(3.0)
                .with_step(1.0)
                .with_allow_clear(true),
            RatingHandlers::new("r").on_change(Arc::new(move |v| {
                sink.lock().unwrap().push(v);
            })),
        );
        (item(&node, 3).interaction.on_activate.as_ref().unwrap())();
        assert_eq!(seen.lock().unwrap().as_slice(), &[None]);

        let seen: Arc<Mutex<Vec<Option<f64>>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let disabled = render(
            &RatingSpec::new()
                .with_value(3.0)
                .with_step(1.0)
                .with_disabled(true),
            RatingHandlers::new("d").on_change(Arc::new(move |v| {
                sink.lock().unwrap().push(v);
            })),
        );
        assert!(item(&disabled, 3).interaction.on_activate.is_none());
        assert!(item(&disabled, 3).interaction.on_key.is_none());
        assert!(seen.lock().unwrap().is_empty());
    }

    #[test]
    fn fractional_scrub_and_keys_use_pure_math() {
        let seen: Arc<Mutex<Vec<Option<f64>>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let node = render(
            &RatingSpec::new().with_value(2.0).with_allow_clear(true),
            RatingHandlers::new("f").on_change(Arc::new(move |v| {
                sink.lock().unwrap().push(v);
            })),
        );
        let scrub = item(&node, 3).interaction.on_scrub.as_ref().unwrap();
        scrub(0.3, ScrubPhase::Press);
        assert_eq!(seen.lock().unwrap().as_slice(), &[Some(2.5)]);

        seen.lock().unwrap().clear();
        let keys = node.interaction.on_key.as_ref().unwrap();
        keys(NodeKey::ArrowRight, NodeModifiers::default());
        assert_eq!(seen.lock().unwrap().as_slice(), &[Some(2.5)]);
        keys(NodeKey::Home, NodeModifiers::default());
        assert_eq!(seen.lock().unwrap().as_slice(), &[Some(2.5), Some(0.0)]);
        keys(NodeKey::Space, NodeModifiers::default());
        assert_eq!(
            seen.lock().unwrap().as_slice(),
            &[Some(2.5), Some(0.0), None]
        );
        seen.lock().unwrap().clear();
        // Rebuild with a value present so on_submit clear is exercised.
        let sink = Arc::clone(&seen);
        let node = render(
            &RatingSpec::new().with_value(2.0).with_allow_clear(true),
            RatingHandlers::new("f2").on_change(Arc::new(move |v| {
                sink.lock().unwrap().push(v);
            })),
        );
        (node.interaction.on_submit.as_ref().unwrap())();
        assert_eq!(seen.lock().unwrap().as_slice(), &[None]);
    }

    #[test]
    fn empty_state_stays_none_and_display_fraction_is_not_quantized() {
        let node = render(&RatingSpec::new(), RatingHandlers::new("e"));
        assert_eq!(node.a11y.value, Some(0.0));
        assert_eq!(
            node.a11y.value_text.as_deref(),
            Some("No rating selected out of 5")
        );

        let spec = RatingSpec::new().with_value(3.7);
        assert!((spec.fill_ratio(3) - 0.7).abs() < 1e-9);
        let display = render(&spec, RatingHandlers::new("d"));
        assert_eq!(display.a11y.value, Some(3.7));
    }

    #[test]
    fn instance_scopes_do_not_collide() {
        let left = render(
            &RatingSpec::new().with_value(1.0).with_step(1.0),
            RatingHandlers::new("left"),
        );
        let right = render(
            &RatingSpec::new().with_value(1.0).with_step(1.0),
            RatingHandlers::new("right"),
        );
        assert_eq!(
            item(&left, 1).runtime_id.as_deref(),
            Some("rating:left:item:1")
        );
        assert_eq!(
            item(&right, 1).runtime_id.as_deref(),
            Some("rating:right:item:1")
        );
    }
}
