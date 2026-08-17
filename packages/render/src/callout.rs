//! Callout — inline notice: tone-tinted surface, icon badge, dismiss control.
//!
//! Contract: `docs/contracts/components/callout.md`
//! Ported from: `packages/jetstream/components/src/callout.rs`. Pending hosts
//! the shared ring spinner in the badge.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment, Node,
    NodeRole, StylePatch,
};
use poodle_specs::{
    CallOutSpec, SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant, StatusTone,
};

use crate::color::{mix_srgb, with_alpha};
use crate::presentation::{
    panel_space_x_rem, rem_to_px, resolve_semantic_size, resolve_supporting_visual_size,
    size_font_rem,
};
use crate::spinner::spinner;

/// Semantic id of the dismiss control. Backends key per-instance state on
/// `runtime_id`; this stays readable.
pub const CALLOUT_DISMISS_ID: &str = "poodle-callout-dismiss";

/// Host callbacks plus the native instance scope.
#[derive(Default)]
pub struct CalloutHandlers {
    pub on_dismiss: Option<Arc<dyn Fn() + Send + Sync>>,
    /// Stable native instance scope. Two dismissible callouts would otherwise
    /// share one backend focus handle.
    pub instance_id: Option<String>,
}

/// The backend-state id of the dismiss control.
pub fn callout_dismiss_focus_id(instance_id: Option<&str>) -> String {
    match instance_id {
        Some(scope) => format!("callout:{scope}:{CALLOUT_DISMISS_ID}"),
        None => CALLOUT_DISMISS_ID.to_string(),
    }
}

fn scoped(instance_id: Option<&str>, semantic: &str) -> Option<String> {
    instance_id.map(|scope| format!("callout:{scope}:{semantic}"))
}

/// Contract §6 icon map. Pending renders a spinner, not an icon.
fn tone_icon(tone: StatusTone) -> &'static str {
    match tone {
        StatusTone::Neutral | StatusTone::Info => "info",
        StatusTone::Success => "check",
        StatusTone::Warning => "triangle-alert",
        StatusTone::Danger => "circle-x",
        StatusTone::Pending => "info",
    }
}

pub fn callout(
    spec: &CallOutSpec,
    theme: &dyn ThemeProvider,
    on_dismiss: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    callout_with_handlers(
        spec,
        theme,
        CalloutHandlers {
            on_dismiss,
            instance_id: None,
        },
    )
}

pub fn callout_with_handlers(
    spec: &CallOutSpec,
    theme: &dyn ThemeProvider,
    handlers: CalloutHandlers,
) -> Node {
    let on_dismiss = handlers.on_dismiss;
    let instance_id = handlers.instance_id;
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let icon_glyph = rem_to_px(size_font_rem(resolve_supporting_visual_size(
        effective_size,
    )));

    let pad_x = rem_to_px(panel_space_x_rem(spec.density));
    let pad_y = rem_to_px(0.625);
    let gap = theme.resolve_space("space.inline.md");
    let content_gap = theme.resolve_space("space.inline.sm");

    let tone_color = theme.resolve_color(spec.tone_color_token());
    let radius = theme.resolve_radius("radius.surface");
    let border_width = theme.resolve_space(spec.border_width_token());
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let panel = theme.resolve_color(spec.fill_mix_target_token());
    let border_default = theme.resolve_color(spec.border_mix_target_token());
    let border_subtle = theme.resolve_color(spec.neutral_border_token());
    let surface = theme.resolve_color(spec.icon_badge_token());

    // Per-tone fill/border (contract §8).
    let is_neutral = spec.is_neutral_tone();
    let is_pending = spec.is_pending_tone();
    let fill = if is_neutral {
        with_alpha(panel, panel.3 * 0.94)
    } else if is_pending {
        mix_srgb(tone_color, panel, 0.08)
    } else {
        mix_srgb(tone_color, panel, 0.10)
    };
    let border = if is_neutral {
        with_alpha(border_subtle, border_subtle.3 * 0.88)
    } else if is_pending {
        mix_srgb(tone_color, border_default, 0.26)
    } else {
        mix_srgb(tone_color, border_default, 0.34)
    };

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = border_width;
        s.descriptor.border.color = border;
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
        s.descriptor.layout.spacing.gap = gap;
        s.fill_width = true;
    }

    // ── Body: icon badge + content ──
    let mut body = Node::container();
    {
        let s = &mut body.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
        s.descriptor.layout.spacing.gap = gap;
        s.descriptor.layout.width = LayoutSizing::Grow;
    }

    // Circular icon badge — 1.375rem, surface at 78%.
    let badge_size = rem_to_px(1.375);
    let mut badge = Node::container();
    {
        let s = &mut badge.style;
        s.descriptor.layout.width = LayoutSizing::Fixed(badge_size);
        s.descriptor.layout.height = LayoutSizing::Fixed(badge_size);
        s.descriptor.corner_radii.top_left = 999.0;
        s.descriptor.corner_radii.top_right = 999.0;
        s.descriptor.corner_radii.bottom_right = 999.0;
        s.descriptor.corner_radii.bottom_left = 999.0;
        s.descriptor.background = Some(with_alpha(surface, surface.3 * 0.78));
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.flex_shrink_zero = true;
    }
    if is_pending {
        badge = badge.child(spinner(
            &SpinnerSpec::new()
                .with_variant(SpinnerVariant::Ring)
                .with_size(SpinnerSize::Sm)
                .with_tone(SpinnerTone::Accent),
            theme,
        ));
    } else {
        let mut icon = Node::icon(tone_icon(spec.tone), icon_glyph);
        icon.style.descriptor.text_color = Some(tone_color);
        badge = badge.child(icon);
    }
    body = body.child(badge);

    // Content column — title + message.
    let mut content = Node::container();
    {
        let s = &mut content.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = content_gap;
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.min_width = Some(0.0);
    }
    if let Some(ref title) = spec.title {
        let mut t = Node::text(title);
        t.style.descriptor.text_color = Some(text_primary);
        t.style.text_size = Some(font_size);
        t.style.text_weight = Some(600);
        content = content.child(t);
    }
    if let Some(ref message) = spec.content {
        let mut m = Node::text(message);
        m.style.descriptor.text_color = Some(text_secondary);
        m.style.text_size = Some(font_size);
        content = content.child(m);
    }
    body = body.child(content);
    el = el.child(body);

    // ── Dismiss control (ghost "x") ──
    if spec.dismissible {
        let dismiss_size = rem_to_px(1.75);
        let control_radius = theme.resolve_radius("radius.control");
        let dismiss_radius = (control_radius - border_width).max(0.0);
        let mut dismiss = Node::container();
        dismiss.id = Some(CALLOUT_DISMISS_ID.to_string());
        dismiss.runtime_id = scoped(instance_id.as_deref(), CALLOUT_DISMISS_ID);
        dismiss.a11y.role = Some(NodeRole::Button);
        dismiss.a11y.label = Some(spec.dismiss_label.clone());
        dismiss.interaction.focusable = true;
        dismiss.style.focus = Some(StylePatch {
            background: None,
            border_color: Some(theme.resolve_color("color.accent.focusRing")),
            text_color: None,
            opacity: None,
        });
        {
            let s = &mut dismiss.style;
            s.descriptor.layout.width = LayoutSizing::Fixed(dismiss_size);
            s.descriptor.layout.height = LayoutSizing::Fixed(dismiss_size);
            s.descriptor.corner_radii.top_left = dismiss_radius;
            s.descriptor.corner_radii.top_right = dismiss_radius;
            s.descriptor.corner_radii.bottom_right = dismiss_radius;
            s.descriptor.corner_radii.bottom_left = dismiss_radius;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            s.descriptor.cursor = CursorHint::Pointer;
        }
        let mut x = Node::icon("x", icon_glyph);
        x.style.descriptor.text_color = Some(text_secondary);
        dismiss = dismiss.child(x);

        if let Some(handler) = &on_dismiss {
            let handler = Arc::clone(handler);
            dismiss.interaction.on_activate = Some(Arc::new(move || handler()));
        }
        el = el.child(dismiss);
    }

    if let Some(label) = spec.aria_label.as_deref() {
        el.a11y.label = Some(label.to_string());
    }
    el.a11y.role = Some(NodeRole::Alert);
    el
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn dismiss_control_is_a_named_focusable_button() {
        let node = callout(
            &CallOutSpec::new()
                .with_title("Dismissible callout")
                .with_content("This callout can be dismissed by the user.")
                .dismissible(true),
            &theme(),
            Some(Arc::new(|| {})),
        );
        let dismiss = node
            .find(&|child| child.id.as_deref() == Some("poodle-callout-dismiss"))
            .expect("dismiss control");
        assert_eq!(dismiss.a11y.role, Some(NodeRole::Button));
        assert_eq!(dismiss.a11y.label.as_deref(), Some("Dismiss message"));
        assert!(dismiss.interaction.focusable);
        assert!(dismiss.style.focus.is_some());
        assert!(dismiss.interaction.on_activate.is_some());
        assert!(dismiss.runtime_id.is_none());
    }

    #[test]
    fn an_instance_scope_isolates_backend_state_ids() {
        let spec = CallOutSpec::new()
            .with_title("Dismissible callout")
            .dismissible(true);
        let first = callout_with_handlers(
            &spec,
            &theme(),
            CalloutHandlers {
                instance_id: Some("first".to_string()),
                ..CalloutHandlers::default()
            },
        );
        let second = callout_with_handlers(
            &spec,
            &theme(),
            CalloutHandlers {
                instance_id: Some("second".to_string()),
                ..CalloutHandlers::default()
            },
        );
        let expected_first = callout_dismiss_focus_id(Some("first"));
        let expected_second = callout_dismiss_focus_id(Some("second"));
        assert!(first
            .find(&|n| n.runtime_id.as_deref() == Some(expected_first.as_str()))
            .is_some());
        assert!(second
            .find(&|n| n.runtime_id.as_deref() == Some(expected_second.as_str()))
            .is_some());
        assert!(first
            .find(&|n| n.runtime_id.as_deref() == Some(expected_second.as_str()))
            .is_none());
        assert!(first
            .find(&|n| n.id.as_deref() == Some(CALLOUT_DISMISS_ID))
            .is_some());
    }
}
