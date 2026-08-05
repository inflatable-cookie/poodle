//! SegmentedControl — an inline choice between exclusive options.
//!
//! Contract: `docs/contracts/components/segmented-control.md`
//! Ported from: `packages/jetstream/components/src/segmented_control.rs`.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, Node, NodeRole,
    ShadowLayer,
};
use poodle_specs::SegmentedControlSpec;

use crate::color::{mix_srgb, with_alpha};
use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
};

pub fn segmented_control(
    spec: &SegmentedControlSpec,
    theme: &dyn ThemeProvider,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let height = rem_to_px(control_height_rem(effective_size));
    // Contract §8: label font fixed at 0.75rem for all sizes.
    let font_size = rem_to_px(0.75);
    let seg_px = rem_to_px(control_space_x_rem(spec.density));
    let inner = rem_to_px(0.125);
    let seg_py = inner;

    let selected_fill = theme.resolve_color(spec.selected_fill_token());
    let surface = theme.resolve_color("color.background.surface");
    let text_primary = theme.resolve_color("color.text.primary");
    let border_subtle = theme.resolve_color("color.border.subtle");
    let text_inverse = theme.resolve_color("color.text.inverse");
    let text_muted = theme.resolve_color("color.text.secondary");
    let control_radius = theme.resolve_radius("radius.control");
    let disabled_opacity = theme.resolve_opacity("state.opacity.disabled");

    let root_bg = mix_srgb(surface, text_primary, 0.93);
    let root_border = with_alpha(border_subtle, border_subtle.3 * 0.84);
    let inner_radius = (control_radius - inner).max(0.0);

    let selected = spec.current_value();

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.background = Some(root_bg);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = root_border;
        s.descriptor.corner_radii.top_left = control_radius;
        s.descriptor.corner_radii.top_right = control_radius;
        s.descriptor.corner_radii.bottom_right = control_radius;
        s.descriptor.corner_radii.bottom_left = control_radius;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.padding.left = inner;
        s.descriptor.layout.spacing.padding.right = inner;
        s.descriptor.layout.spacing.padding.top = inner;
        s.descriptor.layout.spacing.padding.bottom = inner;
        s.descriptor.layout.spacing.gap = inner;
        s.descriptor.layout.height = LayoutSizing::Fixed(height);
        // equal_width=false: content-sized, left-aligned — Start is taffy's
        // default main alignment, so silence is the faithful emission.
    }

    for option in &spec.options {
        let is_selected = selected == Some(option.value.as_str());
        let is_option_disabled = !spec.is_disabled && option.is_disabled;

        let text_color = if is_selected { text_inverse } else { text_muted };

        let mut seg = Node::button(&option.label);
        {
            let s = &mut seg.style;
            s.text_size = Some(font_size);
            s.text_weight = Some(600);
            s.descriptor.text_color = Some(text_color);
            s.descriptor.layout.spacing.padding.left = seg_px;
            s.descriptor.layout.spacing.padding.right = seg_px;
            s.descriptor.layout.spacing.padding.top = seg_py;
            s.descriptor.layout.spacing.padding.bottom = seg_py;
            s.descriptor.corner_radii.top_left = inner_radius;
            s.descriptor.corner_radii.top_right = inner_radius;
            s.descriptor.corner_radii.bottom_right = inner_radius;
            s.descriptor.corner_radii.bottom_left = inner_radius;
            if is_selected {
                s.descriptor.background = Some(selected_fill);
                s.shadow_layers = vec![ShadowLayer {
                    offset_x: 0.0,
                    offset_y: rem_to_px(0.0625),
                    blur: 0.0,
                    spread: 0.0,
                    color: ColorValue(1.0, 1.0, 1.0, 0.12),
                    inset: true,
                }];
            }
            if spec.equal_width {
                s.descriptor.layout.width = LayoutSizing::Grow;
            }
            if is_option_disabled {
                s.descriptor.opacity = disabled_opacity;
            }
        }
        seg.interaction.focusable = true;
        if is_option_disabled {
            seg.interaction.disabled = true;
        }

        // Re-picking the current segment still fires: the host asked to be
        // told about clicks, and swallowing one would hide a "confirm".
        if let (false, false, Some(handler)) = (spec.is_disabled, option.is_disabled, &on_change) {
            let handler = Arc::clone(handler);
            let value = option.value.clone();
            seg.style.descriptor.cursor = CursorHint::Pointer;
            seg.interaction.on_activate = Some(Arc::new(move || handler(&value)));
        }

        el = el.child(seg);
    }

    if spec.is_disabled {
        el.style.descriptor.opacity = disabled_opacity;
    }

    if let Some(label) = spec.aria_label.as_deref() {
        el.a11y.label = Some(label.to_string());
    }
    el.a11y.role = Some(NodeRole::RadioGroup);
    el
}
