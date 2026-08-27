//! TriStateSwitch — ternary exclude/default/include switch.
//!
//! Contract: `docs/contracts/components/tri-state-switch.md`
//! Ported from: `packages/jetstream/components/src/tri_state_switch.rs`.
//!
//! The "Selection" capsule is realized by painting the active segment's own
//! fill + border + shadow stack (no abstractly-positioned slider). The
//! payload is `TriStateValue`, not a bool: three states, no toggle
//! semantics. Per-state hex overrides land in sRGB and linearise at the
//! adapter edge (the old tier fed raw bytes into the linear pipeline — the
//! established custom-hex divergence, fixed here).

use std::sync::Arc;

use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, FocusRing, LayoutDirection, LayoutOverflow,
    LayoutSizing, MainAxisAlignment, Node, NodeKey, NodePosition, NodeRole, NodeToggled,
    ShadowLayer,
};
use poodle_specs::{ControlSize, TriStateSwitchSpec, TriStateValue};

use crate::color::{hex_color, mix_srgb, BLACK};
use crate::context::RenderContext;
use crate::presentation::{control_height_rem, control_space_x_rem, rem_to_px, size_font_rem};

/// Host-owned native interaction for one TriStateSwitch instance.
///
/// `instance_id` is the lifetime-stable scope. It is not a form name, and the
/// renderer never invents one from render order or the selected value.
#[derive(Clone)]
pub struct TriStateSwitchHandlers {
    pub instance_id: String,
    pub on_value_change: Option<Arc<dyn Fn(TriStateValue) + Send + Sync>>,
}

impl TriStateSwitchHandlers {
    pub fn new(instance_id: impl Into<String>) -> Self {
        Self {
            instance_id: instance_id.into(),
            on_value_change: None,
        }
    }

    pub fn on_value_change(mut self, handler: Arc<dyn Fn(TriStateValue) + Send + Sync>) -> Self {
        self.on_value_change = Some(handler);
        self
    }
}

/// Track inset in rem, derived from density (contract §8).
fn track_inset_rem(density: poodle_specs::ControlDensity) -> f32 {
    match density {
        poodle_specs::ControlDensity::Compact => 0.0625,
        poodle_specs::ControlDensity::Default => 0.125,
        poodle_specs::ControlDensity::Comfortable => 0.1875,
    }
}

/// Per-size `--poodle-tri-state-min-content-width` (contract §8 size scale).
fn tri_state_min_content_width_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 2.5,
        ControlSize::Sm => 2.625,
        ControlSize::Md => 3.0,
        ControlSize::Lg => 3.375,
        ControlSize::Xl => 3.75,
    }
}

/// Resolve a per-state color, preferring an instance hex override (sRGB).
fn override_or(
    ctx: &RenderContext<'_>,
    token: &str,
    override_hex: &Option<String>,
) -> ColorValue {
    if let Some(hex) = override_hex {
        if let Some(c) = hex_color(hex) {
            return c;
        }
    }
    ctx.theme().resolve_color(token)
}

fn segment_id(value: TriStateValue) -> String {
    format!("tri-state:{}", value.as_str())
}

fn segment_focus_id(instance_scope: &str, value: TriStateValue) -> String {
    format!("tri-state:{instance_scope}:option:{}", value.as_str())
}

fn tab_stop_value(value: TriStateValue, enabled: bool) -> Option<TriStateValue> {
    if enabled {
        Some(value)
    } else {
        None
    }
}

fn roving_key_handler(
    value: TriStateValue,
    instance_scope: String,
    current: TriStateValue,
    on_value_change: Option<Arc<dyn Fn(TriStateValue) + Send + Sync>>,
) -> Option<Arc<dyn Fn(NodeKey, poodle_node::NodeModifiers) -> Option<String> + Send + Sync>> {
    let index = TriStateValue::ALL.iter().position(|candidate| *candidate == value)?;
    Some(Arc::new(move |key, _modifiers| {
        let last = TriStateValue::ALL.len() - 1;
        let next = match key {
            NodeKey::ArrowRight => Some(if index == last { 0 } else { index + 1 }),
            NodeKey::ArrowLeft => Some(if index == 0 { last } else { index - 1 }),
            _ => None,
        }?;
        let target = TriStateValue::ALL[next];
        if target != current {
            if let Some(handler) = &on_value_change {
                handler(target);
            }
        }
        Some(segment_focus_id(&instance_scope, target))
    }))
}

pub fn tri_state_switch(
    spec: &TriStateSwitchSpec,
    ctx: &RenderContext<'_>,
    handlers: TriStateSwitchHandlers,
) -> Node {
    let instance_scope = handlers.instance_id.as_str();
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);

    // ── Semantic color tokens ──
    let text_secondary = ctx.theme().resolve_color(spec.unselected_text_token());
    let border_default = ctx.theme().resolve_color(spec.border_token());

    // Per-state colors (with optional hex overrides).
    let excluded_color = override_or(ctx, spec.excluded_color_token(), &spec.excluded_color);
    let default_color = override_or(ctx, spec.default_color_token(), &spec.default_color);
    let included_color = override_or(ctx, spec.included_color_token(), &spec.included_color);

    // ── Track base: color-mix(canvas 70%, black); root = canvas 75% black ──
    let canvas = ctx.theme().resolve_color(spec.track_base_token());
    let track_base = mix_srgb(canvas, BLACK, 0.70);
    let root_bg = mix_srgb(canvas, BLACK, 0.75);

    // ── Sizing ──
    let height = rem_to_px(control_height_rem(effective_size));
    let x = rem_to_px(control_space_x_rem(density));
    let inset = rem_to_px(track_inset_rem(density));
    // Contract: segment min-width = min-content-width + x*2.
    let min_segment_width = rem_to_px(tri_state_min_content_width_rem(effective_size)) + x * 2.0;
    let track_width = min_segment_width * 3.0 + inset * 2.0;

    let border_width = rem_to_px(0.0625); // contract hairline

    // Contract §8: segment typography from the label tokens (fixed).
    let label_size = rem_to_px(size_font_rem(effective_size));
    // `typography.label.weight` is the contract's fixed medium weight. Weight
    // tokens are not dimensions, so they must not travel through
    // `ThemeProvider::resolve_space` (the GPUI provider correctly returns 0).
    let label_weight = 500;

    let focus_ring = FocusRing {
        color: ctx.theme().resolve_color(spec.focus_ring_color_token()),
        width: ctx.theme().resolve_border_width("border.width.focus"),
        offset: rem_to_px(0.125),
    };

    // ── Per-state selection fill + border ──
    let value = spec.value;
    let (selection_fill, selection_border) = match value {
        TriStateValue::Excluded => (
            mix_srgb(excluded_color, track_base, 0.14),
            mix_srgb(excluded_color, border_default, 0.58),
        ),
        TriStateValue::Default => (mix_srgb(default_color, track_base, 0.08), border_default),
        TriStateValue::Included => (
            mix_srgb(included_color, track_base, 0.14),
            mix_srgb(included_color, border_default, 0.58),
        ),
    };

    // ── Build segments ──
    let states = [
        (
            TriStateValue::Excluded,
            spec.excluded_label(),
            excluded_color,
        ),
        (TriStateValue::Default, spec.default_label(), default_color),
        (
            TriStateValue::Included,
            spec.included_label(),
            included_color,
        ),
    ];

    let segment_height = height - inset * 2.0;
    let segment_radius = segment_height / 2.0;
    let group_enabled = !spec.is_disabled;
    let tab_stop = tab_stop_value(value, group_enabled);

    let mut selection = Node::container();
    selection.position = NodePosition::Absolute {
        top: Some(inset),
        left: Some(inset + min_segment_width * value.index() as f32),
        right: None,
        bottom: None,
    };
    {
        let s = &mut selection.style;
        s.descriptor.layout.width = LayoutSizing::Fixed(min_segment_width);
        s.descriptor.layout.height = LayoutSizing::Fixed(segment_height);
        let c = &mut s.descriptor.corner_radii;
        c.top_left = segment_radius;
        c.top_right = segment_radius;
        c.bottom_right = segment_radius;
        c.bottom_left = segment_radius;
        s.descriptor.background = Some(selection_fill);
        s.descriptor.border.width = border_width;
        s.descriptor.border.color = selection_border;
        s.shadow_layers = vec![
            ShadowLayer {
                offset_x: 0.0,
                offset_y: rem_to_px(0.0625),
                blur: 0.0,
                spread: 0.0,
                color: ColorValue(1.0, 1.0, 1.0, 0.08),
                inset: false,
            },
            ShadowLayer {
                offset_x: 0.0,
                offset_y: rem_to_px(0.125),
                blur: rem_to_px(0.5),
                spread: 0.0,
                color: ColorValue(0.0, 0.0, 0.0, 0.18),
                inset: false,
            },
        ];
    }

    // ── Root ──
    let mut root = Node::container();
    root.position = NodePosition::Relative;
    {
        let s = &mut root.style;
        s.descriptor.layout.width = LayoutSizing::Fixed(track_width);
        s.descriptor.layout.height = LayoutSizing::Fixed(height);
        let c = &mut s.descriptor.corner_radii;
        let root_radius = height / 2.0;
        c.top_left = root_radius;
        c.top_right = root_radius;
        c.bottom_right = root_radius;
        c.bottom_left = root_radius;
        s.descriptor.background = Some(root_bg);
        s.descriptor.border.width = border_width;
        s.descriptor.border.color = border_default;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
    }
    root = root.child(selection);

    for &(state, label_text, state_color) in &states {
        let is_active = value == state;

        // Active uses per-state color; inactive uses text-secondary.
        let seg_text_color = if is_active {
            state_color
        } else {
            text_secondary
        };

        // The selection capsule paints behind three transparent segments.
        let transparent = ColorValue(0.0, 0.0, 0.0, 0.0);

        let mut segment = Node::button(label_text);
        segment.position = NodePosition::Relative;
        segment.id = Some(segment_id(state));
        segment.runtime_id = Some(segment_focus_id(instance_scope, state));
        segment.a11y.role = Some(NodeRole::RadioButton);
        segment.a11y.label = Some(label_text.to_string());
        segment.a11y.selected = Some(is_active);
        segment.a11y.toggled = Some(if is_active {
            NodeToggled::True
        } else {
            NodeToggled::False
        });

        {
            let s = &mut segment.style;
            s.descriptor.layout.width = LayoutSizing::Fixed(min_segment_width);
            s.descriptor.layout.height = LayoutSizing::Fixed(segment_height);
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = x;
            pad.right = x;
            let c = &mut s.descriptor.corner_radii;
            c.top_left = segment_radius;
            c.top_right = segment_radius;
            c.bottom_right = segment_radius;
            c.bottom_left = segment_radius;
            s.descriptor.background = Some(transparent);
            s.descriptor.border.width = border_width;
            s.descriptor.border.color = transparent;
            s.descriptor.text_color = Some(seg_text_color);
            s.text_size = Some(label_size);
            s.text_weight = Some(label_weight);
            s.no_wrap = true;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        }

        if group_enabled {
            segment.interaction.focusable = true;
            segment.style.focus_ring = Some(focus_ring);
            segment.a11y.tab_index = Some(if tab_stop == Some(state) { 0 } else { -1 });
            segment.style.descriptor.cursor = CursorHint::Pointer;
            if !is_active {
                if let Some(handler) = &handlers.on_value_change {
                    let handler = Arc::clone(handler);
                    segment.interaction.on_activate = Some(Arc::new(move || handler(state)));
                }
            }
            segment.interaction.on_key = roving_key_handler(
                state,
                instance_scope.to_string(),
                value,
                handlers.on_value_change.clone(),
            );
        } else {
            segment.interaction.disabled = true;
            segment.interaction.focusable = false;
            segment.a11y.tab_index = Some(-1);
        }

        root = root.child(segment);
    }

    // ── Disabled state ──
    if spec.is_disabled {
        root.style.descriptor.opacity = ctx.theme().resolve_opacity(spec.disabled_opacity_token());
        root.interaction.disabled = true;
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            root.a11y.label = Some(label.to_string());
        }
    }
    root.a11y.role = Some(NodeRole::RadioGroup);
    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_node::NodeModifiers;
    use std::sync::Mutex;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn render(spec: &TriStateSwitchSpec, handlers: TriStateSwitchHandlers) -> Node {
        tri_state_switch(spec, &RenderContext::new(&theme()), handlers)
    }

    fn find_segment<'a>(node: &'a Node, value: TriStateValue) -> &'a Node {
        let id = segment_id(value);
        node.find(&|n| n.id.as_deref() == Some(id.as_str()))
            .unwrap_or_else(|| panic!("segment {:?} exists", value))
    }

    #[test]
    fn selected_segment_is_the_roving_tab_stop() {
        let spec = TriStateSwitchSpec::new().with_value(TriStateValue::Included);
        let node = render(&spec, TriStateSwitchHandlers::new("filter"));
        assert_eq!(
            find_segment(&node, TriStateValue::Excluded).a11y.tab_index,
            Some(-1)
        );
        let included = find_segment(&node, TriStateValue::Included);
        assert_eq!(included.a11y.tab_index, Some(0));
        assert_eq!(included.a11y.selected, Some(true));
        assert_eq!(included.a11y.toggled, Some(NodeToggled::True));
    }

    #[test]
    fn disabled_group_suppresses_focus_and_handlers() {
        let spec = TriStateSwitchSpec::new()
            .with_value(TriStateValue::Default)
            .with_disabled(true);
        let node = render(
            &spec,
            TriStateSwitchHandlers::new("disabled-filter").on_value_change(Arc::new(|_| {})),
        );
        for value in TriStateValue::ALL {
            let segment = find_segment(&node, value);
            assert_eq!(segment.a11y.tab_index, Some(-1));
            assert!(!segment.interaction.focusable);
            assert!(segment.interaction.on_activate.is_none());
        }
    }

    #[test]
    fn same_value_activation_is_inert() {
        let payloads = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&payloads);
        let spec = TriStateSwitchSpec::new().with_value(TriStateValue::Default);
        let node = render(
            &spec,
            TriStateSwitchHandlers::new("filter").on_value_change(Arc::new(move |value| {
                sink.lock().unwrap().push(value);
            })),
        );
        let default = find_segment(&node, TriStateValue::Default);
        assert!(default.interaction.on_activate.is_none());
    }

    #[test]
    fn arrow_wrap_reports_changed_value_and_requests_focus() {
        let payloads = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&payloads);
        let spec = TriStateSwitchSpec::new().with_value(TriStateValue::Included);
        let node = render(
            &spec,
            TriStateSwitchHandlers::new("filter").on_value_change(Arc::new(move |value| {
                sink.lock().unwrap().push(value);
            })),
        );
        let included = find_segment(&node, TriStateValue::Included);
        let handler = included
            .interaction
            .on_key
            .as_ref()
            .expect("arrow handler exists");
        let focus = handler(NodeKey::ArrowRight, NodeModifiers::default());
        assert_eq!(focus, Some(segment_focus_id("filter", TriStateValue::Excluded)));
        assert_eq!(payloads.lock().unwrap().as_slice(), [TriStateValue::Excluded]);
    }

    #[test]
    fn two_instances_keep_independent_runtime_identity() {
        let left = render(
            &TriStateSwitchSpec::new().with_value(TriStateValue::Default),
            TriStateSwitchHandlers::new("left"),
        );
        let right = render(
            &TriStateSwitchSpec::new().with_value(TriStateValue::Default),
            TriStateSwitchHandlers::new("right"),
        );
        assert_eq!(
            find_segment(&left, TriStateValue::Default).runtime_id,
            Some(segment_focus_id("left", TriStateValue::Default))
        );
        assert_eq!(
            find_segment(&right, TriStateValue::Default).runtime_id,
            Some(segment_focus_id("right", TriStateValue::Default))
        );
    }
}
