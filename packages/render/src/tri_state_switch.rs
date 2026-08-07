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

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutOverflow, LayoutSizing,
    MainAxisAlignment, Node, NodePosition, NodeRole, NodeToggled, ShadowLayer,
};
use poodle_specs::{ControlSize, TriStateSwitchSpec, TriStateValue};

use crate::color::{hex_color, mix_srgb, BLACK};
use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};

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
    theme: &dyn ThemeProvider,
    token: &str,
    override_hex: &Option<String>,
) -> ColorValue {
    if let Some(hex) = override_hex {
        if let Some(c) = hex_color(hex) {
            return c;
        }
    }
    theme.resolve_color(token)
}

pub fn tri_state_switch(
    spec: &TriStateSwitchSpec,
    theme: &dyn ThemeProvider,
    on_change: Option<Arc<dyn Fn(TriStateValue) + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // ── Semantic color tokens ──
    let text_secondary = theme.resolve_color(spec.unselected_text_token());
    let border_default = theme.resolve_color(spec.border_token());

    // Per-state colors (with optional hex overrides).
    let excluded_color = override_or(theme, spec.excluded_color_token(), &spec.excluded_color);
    let default_color = override_or(theme, spec.default_color_token(), &spec.default_color);
    let included_color = override_or(theme, spec.included_color_token(), &spec.included_color);

    // ── Track base: color-mix(canvas 70%, black); root = canvas 75% black ──
    let canvas = theme.resolve_color(spec.track_base_token());
    let track_base = mix_srgb(canvas, BLACK, 0.70);
    let root_bg = mix_srgb(canvas, BLACK, 0.75);

    // ── Sizing ──
    let height = rem_to_px(control_height_rem(effective_size));
    let x = rem_to_px(control_space_x_rem(spec.density));
    let inset = rem_to_px(track_inset_rem(spec.density));
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

    // ── Per-state selection fill + border ──
    let value = spec.value();
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
        // Contract: three mutually exclusive states — each is a `radio`.
        segment.a11y.role = Some(NodeRole::RadioButton);
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
        segment.interaction.focusable = true;

        // The active segment stays clickable: re-picking the current state is
        // still a click a host asked to hear about.
        if let (false, Some(handler)) = (spec.is_disabled, &on_change) {
            let handler = Arc::clone(handler);
            segment.style.descriptor.cursor = CursorHint::Pointer;
            segment.interaction.on_activate = Some(Arc::new(move || handler(state)));
        }

        root = root.child(segment);
    }

    // ── Disabled state ──
    if spec.is_disabled {
        root.style.descriptor.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
        root.interaction.disabled = true;
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            root.a11y.label = Some(label.to_string());
        }
    }
    // Contract: three mutually exclusive options — a `radiogroup`.
    root.a11y.role = Some(NodeRole::RadioGroup);
    root
}
