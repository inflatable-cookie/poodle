//! Checkbox — boolean toggle with indicator and label.
//!
//! Contract: `docs/contracts/components/checkbox.md`
//! Ported from: `packages/jetstream/components/src/checkbox.rs`.

use std::sync::Arc;

use poodle_node::{
    CrossAxisAlignment, LayoutDirection, LayoutSizing, MainAxisAlignment, Node, NodeRole,
    NodeToggled,
};
use poodle_specs::{CheckState, CheckboxSpec, ControlDensity, ControlSize};

use crate::color::hex_color;
use crate::context::RenderContext;
use crate::presentation::{icon_token, rem_to_px, size_font_rem};

/// Indicator size = per-size icon token + 0.125rem (the Svelte formula).
fn indicator_size_px(icon_px: f32) -> f32 {
    icon_px + rem_to_px(0.125)
}

/// Mark size = per-size icon token − 0.125rem (xs/sm/md) or − 0.25rem (lg/xl).
fn mark_size_px(size: ControlSize, icon_px: f32) -> f32 {
    let offset = match size {
        ControlSize::Xs | ControlSize::Sm | ControlSize::Md => -0.125,
        ControlSize::Lg | ControlSize::Xl => -0.25,
    };
    icon_px + rem_to_px(offset)
}

/// Indicator radius ladder — contract-exact rem literals.
fn indicator_radius_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.1875,
        ControlSize::Sm => 0.25,
        ControlSize::Md => 0.3125,
        ControlSize::Lg => 0.375,
        ControlSize::Xl => 0.4375,
    }
}

/// Root gap by density: compact literal, else inline-spacing tokens.
fn root_gap_px(density: ControlDensity, ctx: &RenderContext<'_>) -> f32 {
    match density {
        ControlDensity::Compact => rem_to_px(0.375),
        ControlDensity::Default => ctx.theme().resolve_space("space.inline.sm"),
        ControlDensity::Comfortable => ctx.theme().resolve_space("space.inline.md"),
    }
}

/// Build a checkbox node. `on_change` fires with the state moving **to**
/// (mixed resolves to checked) unless disabled or read-only — read-only stays
/// focusable and full strength but must not report a change.
pub fn checkbox(
    spec: &CheckboxSpec,
    ctx: &RenderContext<'_>,
    on_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
) -> Node {
    let theme = ctx.theme();
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);

    // Custom `selected_color` hex wins over the spec's accent token.
    //
    // Colour-space note: the old tier fed this hex into its linear pipeline
    // unconverted while every token colour was converted — custom-coloured
    // indicators rendered visibly brighter than the same colour anywhere else.
    // Here the hex is sRGB like everything in the vocabulary and converts at
    // the backend edge, which fixes that; the parity suite records the
    // divergence as intentional.
    let selected_fill = spec
        .selected_color
        .as_deref()
        .and_then(hex_color)
        .unwrap_or_else(|| theme.resolve_color(spec.indicator_fill_token()));
    let border_default = theme.resolve_color("color.border.default");
    let text_primary = theme.resolve_color("color.text.primary");
    let text_inverse = theme.resolve_color("color.text.inverse");
    let gap = root_gap_px(density, ctx);
    let label_size = rem_to_px(size_font_rem(effective_size));

    let state = spec.current_state();
    let is_checked = matches!(state, CheckState::Checked | CheckState::Mixed);

    let mark_color = if is_checked {
        text_inverse
    } else {
        text_primary
    };
    let indicator_border = if is_checked {
        selected_fill
    } else {
        border_default
    };
    let surface = theme.resolve_color("color.background.surface");
    let indicator_bg = if is_checked { selected_fill } else { surface };

    let icon_px = theme.resolve_space(icon_token(effective_size));
    let indicator_size = indicator_size_px(icon_px);
    let indicator_radius = rem_to_px(indicator_radius_rem(effective_size));
    let border_width = theme.resolve_border_width("border.width.default");
    let mark_size = mark_size_px(effective_size, icon_px);

    let mut indicator = Node::container();
    {
        let s = &mut indicator.style;
        s.descriptor.layout.width = LayoutSizing::Fixed(indicator_size);
        s.descriptor.layout.height = LayoutSizing::Fixed(indicator_size);
        s.descriptor.corner_radii.top_left = indicator_radius;
        s.descriptor.corner_radii.top_right = indicator_radius;
        s.descriptor.corner_radii.bottom_right = indicator_radius;
        s.descriptor.corner_radii.bottom_left = indicator_radius;
        s.descriptor.background = Some(indicator_bg);
        s.descriptor.border.width = border_width;
        s.descriptor.border.color = indicator_border;
        // Explicit Row (see switch.rs): the old tier got taffy's Row default.
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
    }

    match state {
        CheckState::Checked => {
            let mut mark = Node::icon("check", mark_size);
            mark.style.descriptor.text_color = Some(mark_color);
            indicator = indicator.child(mark);
        }
        CheckState::Mixed => {
            let mut mark = Node::icon("minus", mark_size);
            mark.style.descriptor.text_color = Some(mark_color);
            indicator = indicator.child(mark);
        }
        CheckState::Unchecked => {}
    }

    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.spacing.gap = gap;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    }
    root.interaction.focusable = true;
    root = root.child(indicator);

    if let Some(ref label) = spec.label {
        let mut text = Node::text(label);
        text.style.descriptor.text_color = Some(text_primary);
        text.style.text_size = Some(label_size);
        text.style.text_weight = Some(500);
        root = root.child(text);
    }

    if spec.is_disabled {
        root.style.descriptor.opacity = theme.resolve_opacity("state.opacity.disabled");
        root.interaction.disabled = true;
    }

    if !(spec.is_disabled || spec.is_read_only) {
        if let Some(handler) = on_change {
            let next = !matches!(spec.current_state(), CheckState::Checked);
            root.interaction.on_activate = Some(Arc::new(move || handler(next)));
        }
    }

    if let Some(label) = spec.aria_label.as_deref() {
        root.a11y.label = Some(label.to_string());
    }
    root.a11y.role = Some(NodeRole::CheckBox);
    root.a11y.toggled = Some(match spec.checked {
        Some(true) => NodeToggled::True,
        Some(false) => NodeToggled::False,
        // None is mixed, not unchecked — a different and true claim.
        None => NodeToggled::Mixed,
    });
    root
}
