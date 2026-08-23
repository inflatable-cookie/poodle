//! PageLoading — loading card (overlay or inline).
//!
//! Contract: `docs/contracts/components/page-loading.md`
//! Ported from: `packages/jetstream/components/src/page_loading.rs`.

use std::sync::Arc;

use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment, Node,
    NodeRole,
};
use poodle_specs::{
    PageLoadingSpec, ProgressSpec, SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant,
};

use crate::context::RenderContext;
use crate::presentation::{
    control_space_x_rem, panel_space_x_rem, panel_space_y_rem, rem_to_px, size_font_rem,
};
use crate::progress::progress;
use crate::spinner::spinner;

pub fn page_loading(
    spec: &PageLoadingSpec,
    ctx: &RenderContext<'_>,
    on_cancel: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    if !spec.is_visible {
        let mut empty = Node::container();
        // Explicit Row (see switch.rs).
        empty.style.descriptor.layout.direction = LayoutDirection::Row;
        return empty;
    }

    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(panel_space_x_rem(density));
    let pad_y = rem_to_px(panel_space_y_rem(density));
    let item_gap = rem_to_px(control_space_x_rem(density));

    let backdrop = ctx.theme().resolve_color(spec.backdrop_fill_token());
    let text_color = ctx.theme().resolve_color("color.text.primary");
    let text_secondary = ctx.theme().resolve_color("color.text.secondary");
    let surface_bg = ctx.theme().resolve_color("color.background.elevated");
    let border = ctx.theme().resolve_color("color.border.default");
    let border_width = ctx.theme().resolve_space("border.width.default");
    let radius = ctx.theme().resolve_radius("radius.surface");
    let control_radius = ctx.theme().resolve_radius("radius.control");

    let is_inline = spec.presentation.is_inline();

    // Card container. Overlay mode is an elevated card with chrome; inline
    // mode drops chrome + padding and caps at max-width 24rem.
    let mut card = Node::container();
    {
        let s = &mut card.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = item_gap * 2.0;
        if is_inline {
            s.max_width = Some(rem_to_px(24.0));
        } else {
            s.descriptor.background = Some(surface_bg);
            s.descriptor.border.width = border_width;
            s.descriptor.border.color = border;
            let c = &mut s.descriptor.corner_radii;
            c.top_left = radius;
            c.top_right = radius;
            c.bottom_right = radius;
            c.bottom_left = radius;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = pad_x;
            pad.right = pad_x;
            pad.top = pad_y;
            pad.bottom = pad_y;
        }
    }

    // Spinner — shared ring Spinner primitive (contract §2).
    let mut card = card.child(spinner(
        &SpinnerSpec::new()
            .with_variant(SpinnerVariant::Ring)
            .with_size(SpinnerSize::Lg)
            .with_tone(SpinnerTone::Accent),
        ctx,
    ));

    // Progress bar (determinate) — shared Progress primitive, full card width.
    if let Some(value) = spec.value {
        let mut progress_spec = ProgressSpec::new().with_value(value);
        progress_spec.max = spec.max;
        let mut wrap = Node::container();
        {
            let s = &mut wrap.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.self_stretch = true;
        }
        card = card.child(wrap.child(progress(&progress_spec, ctx)));
    }

    // Message
    if let Some(ref msg) = spec.message {
        let mut m = Node::text(msg);
        m.style.descriptor.text_color = Some(text_color);
        m.style.text_size = Some(font_size);
        card = card.child(m);
    }

    // Cancel action — bordered control (contract §8 `.page-loading__cancel`).
    if spec.can_cancel {
        let mut cancel = Node::text("Cancel");
        {
            let s = &mut cancel.style;
            s.descriptor.text_color = Some(text_secondary);
            s.text_size = Some(font_size);
            s.descriptor.border.width = border_width;
            s.descriptor.border.color = border;
            let c = &mut s.descriptor.corner_radii;
            c.top_left = control_radius;
            c.top_right = control_radius;
            c.bottom_right = control_radius;
            c.bottom_left = control_radius;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = rem_to_px(0.875);
            pad.right = rem_to_px(0.875);
            pad.top = rem_to_px(0.375);
            pad.bottom = rem_to_px(0.375);
            s.descriptor.cursor = CursorHint::Pointer;
        }
        if let Some(handler) = &on_cancel {
            let handler = Arc::clone(handler);
            cancel.interaction.on_activate = Some(Arc::new(move || handler()));
        }
        card = card.child(cancel);
    }

    // Branch on presentation: inline in-flow centered; overlay scrim.
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        if is_inline {
            s.self_stretch = true;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.top = rem_to_px(3.0);
            pad.bottom = rem_to_px(3.0);
            pad.left = rem_to_px(1.0);
            pad.right = rem_to_px(1.0);
        } else {
            s.descriptor.background = Some(backdrop);
            s.descriptor.layout.width = LayoutSizing::Grow;
        }
    }
    let mut root = root.child(card);
    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            root.a11y.label = Some(label.to_string());
        }
    }
    root.a11y.role = Some(NodeRole::Status);
    root
}
