//! CollapseToggle — the chevron that collapses a region.
//!
//! Contract: `docs/contracts/components/collapse-toggle.md`
//! Ported from: `packages/jetstream/components/src/collapse_toggle.rs`.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, MainAxisAlignment, Node, StylePatch,
};
use poodle_specs::CollapseToggleSpec;

use crate::presentation::rem_to_px;

pub fn collapse_toggle(
    spec: &CollapseToggleSpec,
    theme: &dyn ThemeProvider,
    on_toggle: Option<Arc<dyn Fn(bool) + Send + Sync>>,
) -> Node {
    let icon_size = theme.resolve_space(spec.icon_size_token());
    let radius = theme.resolve_radius(spec.radius_token());
    let text_color = theme.resolve_color(spec.text_color_token());
    let hover_fill = theme.resolve_color(spec.hover_fill_token());
    let hover_text = theme.resolve_color(spec.text_color_hover_token());

    let pad_y = rem_to_px(spec.padding_rem());
    let pad_x = rem_to_px(spec.padding_inline_rem());

    let mut el = Node::button("");
    // A bare chevron: the name says what pressing does, aria_expanded the state.
    el.a11y.label = Some("Toggle section".to_string());
    {
        let s = &mut el.style;
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.layout.spacing.padding.left = pad_x;
        s.descriptor.layout.spacing.padding.right = pad_x;
        s.descriptor.layout.spacing.padding.top = pad_y;
        s.descriptor.layout.spacing.padding.bottom = pad_y;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.line_height = Some(1.0);
    }
    el.interaction.focusable = true;

    let mut chevron = Node::icon(spec.effective_icon_name(), icon_size);
    chevron.style.descriptor.text_color = Some(text_color);
    el = el.child(chevron);

    if spec.is_disabled {
        el.style.descriptor.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
        el.style.descriptor.cursor = CursorHint::Default;
    } else {
        el.style.descriptor.cursor = CursorHint::Pointer;
        el.style.hover = Some(StylePatch {
            background: Some(hover_fill),
            border_color: None,
            text_color: Some(hover_text),
        });
        if let Some(handler) = on_toggle {
            let next = !spec.is_collapsed;
            el.interaction.on_activate = Some(Arc::new(move || handler(next)));
        }
    }

    if let Some(label) = spec.aria_label.as_deref() {
        el.a11y.label = Some(label.to_string());
    }
    el
}
