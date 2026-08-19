//! RemediationBanner — announcing recovery banner.
//!
//! Contract: `docs/contracts/components/remediation-banner.md`
//! Ported from: `packages/jetstream/components/src/remediation_banner.rs`.
//!
//! Anatomy (contract §2): icon + content (title/message/actions) + dismiss.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, Node, NodeRole,
    StylePatch,
};
use poodle_specs::{
    ButtonSpec, ButtonVariant, RemediationAction, RemediationBannerSpec, SpinnerSize, SpinnerSpec,
    SpinnerTone, SpinnerVariant, StatusTone,
};

use crate::button::button;
use crate::color::{mix_srgb, solid_tone_surface, with_alpha};
use crate::presentation::rem_to_px;
use crate::spinner::spinner;

const DISMISS_ID: &str = "remediation-banner-dismiss";

#[derive(Default, Clone)]
pub struct RemediationBannerHandlers {
    pub on_action: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_dismiss: Option<Arc<dyn Fn() + Send + Sync>>,
    /// Stable native instance scope. Two banners with the same action ids
    /// would otherwise share one backend focus handle.
    pub instance_id: Option<String>,
}

/// The backend-state id of the dismiss control.
pub fn remediation_banner_dismiss_focus_id(instance_id: Option<&str>) -> String {
    match instance_id {
        Some(scope) => format!("remediation-banner:{scope}:{DISMISS_ID}"),
        None => DISMISS_ID.to_string(),
    }
}

/// The backend-state id of one recovery action.
pub fn remediation_banner_action_focus_id(instance_id: Option<&str>, action: &str) -> String {
    match instance_id {
        Some(scope) => format!("remediation-banner:{scope}:action:{action}"),
        None => format!("remediation-action-{action}"),
    }
}

fn scoped(instance_id: Option<&str>, part: &str) -> Option<String> {
    instance_id.map(|scope| format!("remediation-banner:{scope}:{part}"))
}

pub fn remediation_banner(
    spec: &RemediationBannerSpec,
    theme: &dyn ThemeProvider,
    handlers: RemediationBannerHandlers,
) -> Node {
    let instance_id = handlers.instance_id.clone();
    // ── Colors (all token-resolved) ──
    let tone_color = theme.resolve_color(spec.border_token());
    let panel = theme.resolve_color(spec.background_token());
    let icon_color = theme.resolve_color(spec.icon_color_token());
    let text_primary = theme.resolve_color(spec.title_color_token());
    let text_secondary = theme.resolve_color(spec.message_color_token());
    let solid = spec.is_solid_fill();
    let tint_border_mix = if spec.tone == StatusTone::Pending {
        0.24
    } else {
        0.34
    };
    let solid_surface = solid
        .then(|| solid_tone_surface(theme, tone_color, spec.is_neutral_tone(), tint_border_mix));

    // Surface fill = color-mix(tone, panel) at the spec's tone ratio; border = tone.
    let fill = solid_surface
        .map(|surface| surface.background)
        .unwrap_or_else(|| mix_srgb(tone_color, panel, spec.fill_tone_ratio()));
    let border = solid_surface
        .map(|surface| surface.border)
        .unwrap_or_else(|| theme.resolve_color(spec.border_token()));
    let foreground = solid_surface
        .map(|surface| surface.foreground)
        .unwrap_or(text_primary);
    let icon_color = if solid { foreground } else { icon_color };

    // ── Dimensions ──
    // Radius / border-width resolve from tokens; the rest are contract-exact
    // rem values (no semantic token exists for them).
    let radius = theme.resolve_radius(spec.radius_token());
    let border_width = theme.resolve_border_width(spec.border_width_token());
    // Typography: title at body size, message at label size (contract §2).
    let title_size = theme.resolve_space("typography.body.size");
    let message_size = theme.resolve_space("typography.label.size");
    let icon_size = rem_to_px(1.25); // note: contract icon size, no token
    let pad_x = theme.resolve_space("space.panel.x");
    let pad_y = theme.resolve_space("space.panel.y");
    let gap = theme.resolve_space("space.inline.md"); // root row gap (note: approx)
    let content_gap = theme.resolve_space("space.inline.xs"); // title↔message↔actions
    let action_gap = theme.resolve_space("space.inline.sm"); // between action buttons
    let dismiss_size = rem_to_px(1.0); // note: contract dismiss size, no token

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.background = Some(fill);
        s.descriptor.text_color = Some(foreground);
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
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
        s.descriptor.layout.spacing.gap = gap;
    }
    el.a11y.role = Some(NodeRole::Alert);

    // ── Icon (contract §2: tone-based leading indicator) ──
    let glyph = if spec.tone == StatusTone::Pending {
        let mut pending_spinner = spinner(
            &SpinnerSpec::new()
                .with_variant(SpinnerVariant::Ring)
                .with_size(SpinnerSize::Sm)
                .with_tone(if solid {
                    SpinnerTone::Current
                } else {
                    SpinnerTone::Accent
                }),
            theme,
        );
        if let Some(surface) = solid_surface {
            pending_spinner.style.descriptor.text_color = Some(surface.foreground);
        }
        pending_spinner
    } else {
        let mut glyph = Node::icon(spec.tone_icon_name(), icon_size);
        glyph.style.descriptor.text_color = Some(icon_color);
        glyph
    };
    let mut el = el.child(glyph);

    // ── Content column: Title + Message + Actions ──
    let mut content = Node::container();
    {
        let s = &mut content.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = content_gap;
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.min_width = Some(0.0);
    }

    let mut title = Node::text(&spec.title);
    title.style.descriptor.text_color = Some(foreground);
    title.style.text_size = Some(title_size);
    title.style.text_weight = Some(600);
    let mut content = content.child(title);

    let mut message = Node::text(&spec.message);
    message.style.descriptor.text_color = Some(if solid { foreground } else { text_secondary });
    message.style.text_size = Some(message_size);
    content = content.child(message);

    // ── Actions row: real buttons honoring variant + is_disabled ──
    if spec.action_count() > 0 {
        let mut actions_row = Node::container();
        {
            let s = &mut actions_row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = action_gap;
            s.descriptor.layout.spacing.padding.top = content_gap;
        }

        if let Some(ref primary) = spec.primary_action {
            actions_row = actions_row.child(action_button(
                primary,
                theme,
                handlers.on_action.as_ref(),
                instance_id.as_deref(),
                solid,
                foreground,
            ));
        }
        if let Some(ref secondary) = spec.secondary_action {
            actions_row = actions_row.child(action_button(
                secondary,
                theme,
                handlers.on_action.as_ref(),
                handlers.instance_id.as_deref(),
                solid,
                foreground,
            ));
        }

        content = content.child(actions_row);
    }

    el = el.child(content);

    // ── Dismiss (contract §5: aria-label="Dismiss") ──
    if spec.is_dismissible {
        let mut dismiss = Node::icon("x", dismiss_size);
        dismiss.id = Some(DISMISS_ID.to_string());
        dismiss.runtime_id = scoped(instance_id.as_deref(), DISMISS_ID);
        dismiss.style.descriptor.text_color = Some(if solid { foreground } else { text_secondary });
        dismiss.a11y.role = Some(NodeRole::Button);
        dismiss.a11y.label = Some(spec.dismiss_label.clone());
        dismiss.interaction.focusable = true;
        dismiss.style.focus = Some(StylePatch {
            background: None,
            border_color: Some(theme.resolve_color("color.accent.focusRing")),
            text_color: None,
            opacity: None,
        });
        if let Some(on_dismiss) = handlers.on_dismiss {
            dismiss.style.descriptor.cursor = CursorHint::Pointer;
            dismiss.interaction.on_activate = Some(on_dismiss);
        }
        el = el.child(dismiss);
    }

    el
}

/// Build a real `button` for a `RemediationAction`, honoring its variant and
/// disabled flag (contract §2: RemediationAction button; §3 `is_disabled`).
/// `button` already applies the disabled-opacity token.
fn action_button(
    action: &RemediationAction,
    theme: &dyn ThemeProvider,
    on_action: Option<&Arc<dyn Fn(&str) + Send + Sync>>,
    instance_id: Option<&str>,
    solid: bool,
    foreground: ColorValue,
) -> Node {
    let on_click = on_action.map(|handler| {
        let handler = Arc::clone(handler);
        let id = action.id.clone();
        Arc::new(move || handler(&id)) as Arc<dyn Fn() + Send + Sync>
    });
    let mut b = button(
        &ButtonSpec::new()
            .with_variant(action.variant)
            .with_label(action.label.clone())
            .with_disabled(action.is_disabled),
        theme,
        on_click,
    );
    b.id = Some(format!("remediation-action-{}", action.id));
    b.runtime_id = scoped(instance_id, &format!("action:{}", action.id));
    if solid && action.variant != ButtonVariant::Primary {
        b.style.descriptor.background = Some(with_alpha(foreground, 0.16));
        b.style.descriptor.border.color = with_alpha(foreground, 0.56);
        b.style.descriptor.text_color = Some(foreground);
        b.style.descriptor.shadow = None;
        b.style.hover = Some(StylePatch {
            background: Some(with_alpha(foreground, 0.24)),
            border_color: Some(foreground),
            text_color: Some(foreground),
            opacity: None,
        });
        b.style.active = Some(StylePatch {
            background: Some(with_alpha(foreground, 0.30)),
            border_color: Some(foreground),
            text_color: Some(foreground),
            opacity: None,
        });
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_node::NodeKind;
    use poodle_specs::ButtonVariant;
    use poodle_specs::ToneFill;
    use std::sync::Mutex;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn action_and_dismiss_controls_reach_host_handlers() {
        let seen = Arc::new(Mutex::new(Vec::new()));
        let action_seen = Arc::clone(&seen);
        let dismiss_seen = Arc::clone(&seen);
        let spec = RemediationBannerSpec::new("Save failed", "Try again.")
            .with_primary_action(
                RemediationAction::new("retry", "Retry").with_variant(ButtonVariant::Primary),
            )
            .with_dismissible(true);
        let node = remediation_banner(
            &spec,
            &theme(),
            RemediationBannerHandlers {
                on_action: Some(Arc::new(move |id| {
                    action_seen.lock().unwrap().push(id.to_string())
                })),
                on_dismiss: Some(Arc::new(move || {
                    dismiss_seen.lock().unwrap().push("dismiss".to_string())
                })),
                instance_id: None,
            },
        );

        let retry = node
            .find(&|node| node.id.as_deref() == Some("remediation-action-retry"))
            .expect("retry action");
        let dismiss = node
            .find(&|node| node.id.as_deref() == Some("remediation-banner-dismiss"))
            .expect("dismiss action");
        (retry.interaction.on_activate.as_ref().unwrap())();
        (dismiss.interaction.on_activate.as_ref().unwrap())();

        assert_eq!(seen.lock().unwrap().as_slice(), ["retry", "dismiss"]);
        assert_eq!(dismiss.a11y.label.as_deref(), Some("Dismiss"));
        assert!(retry.runtime_id.is_none());
        assert!(dismiss.runtime_id.is_none());
    }

    #[test]
    fn an_instance_scope_isolates_backend_state_ids() {
        let spec = RemediationBannerSpec::new("Save failed", "Try again.")
            .with_primary_action(
                RemediationAction::new("retry", "Retry").with_variant(ButtonVariant::Primary),
            )
            .with_dismissible(true);
        let scoped_handlers = |scope: &str| RemediationBannerHandlers {
            instance_id: Some(scope.to_string()),
            ..RemediationBannerHandlers::default()
        };
        let first = remediation_banner(&spec, &theme(), scoped_handlers("first"));
        let second = remediation_banner(&spec, &theme(), scoped_handlers("second"));
        let action = remediation_banner_action_focus_id(Some("first"), "retry");
        let dismiss = remediation_banner_dismiss_focus_id(Some("first"));
        assert!(first
            .find(&|n| n.runtime_id.as_deref() == Some(action.as_str()))
            .is_some());
        assert!(first
            .find(&|n| n.runtime_id.as_deref() == Some(dismiss.as_str()))
            .is_some());
        assert!(first
            .find(&|n| n.runtime_id.as_deref()
                == Some(remediation_banner_action_focus_id(Some("second"), "retry").as_str()))
            .is_none());
        assert!(first
            .find(&|n| n.id.as_deref() == Some("remediation-action-retry"))
            .is_some());
        assert!(second
            .find(&|n| n.runtime_id.as_deref()
                == Some(remediation_banner_dismiss_focus_id(Some("second")).as_str()))
            .is_some());
    }

    #[test]
    fn solid_pending_banner_uses_primary_spinner_and_local_secondary_recipe() {
        let theme = theme();
        let spec = RemediationBannerSpec::new("Reconnecting", "Please wait.")
            .with_tone(StatusTone::Pending)
            .with_fill(ToneFill::Solid)
            .with_secondary_action(RemediationAction::new("details", "View details"));
        let expected = solid_tone_surface(
            &theme,
            theme.resolve_color(spec.border_token()),
            false,
            0.24,
        );
        let node = remediation_banner(&spec, &theme, RemediationBannerHandlers::default());

        assert_eq!(node.style.descriptor.background, Some(expected.background));
        assert_eq!(node.style.descriptor.border.color, expected.border);
        assert_eq!(node.style.descriptor.text_color, Some(expected.foreground));
        let spinner = node
            .find(&|n| matches!(&n.kind, NodeKind::Icon { name, .. } if name == "spinner"))
            .expect("pending spinner");
        assert_eq!(
            spinner.style.descriptor.text_color,
            Some(expected.foreground)
        );

        let action = node
            .find(&|n| n.id.as_deref() == Some("remediation-action-details"))
            .expect("secondary action");
        assert_eq!(
            action.style.descriptor.background,
            Some(with_alpha(expected.foreground, 0.16))
        );
        assert_eq!(
            action.style.descriptor.border.color,
            with_alpha(expected.foreground, 0.56)
        );
        assert_eq!(
            action.style.descriptor.text_color,
            Some(expected.foreground)
        );
        assert!(action.style.hover.is_some());
    }
}
