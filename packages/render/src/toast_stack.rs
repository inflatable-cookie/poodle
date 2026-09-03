//! ToastStack — toast notification stack.
//!
//! Contract: `docs/contracts/components/toast-stack.md`
//! Ported from: `packages/jetstream/components/src/toast_stack.rs`.
//!
//! Each toast: leading tone accent bar, title + optional message, optional
//! Button action, Icon-backed dismiss button, tone-tinted gradient fill, and
//! elevation-overlay shadow. Authored rows paint at the settled endpoint.

use std::sync::Arc;

use poodle_node::{
    CrossAxisAlignment, CursorHint, FocusRing, LayoutDirection, LayoutSizing, Node, NodePosition,
    NodeRole, StylePatch,
};
use poodle_specs::{
    ButtonSpec, ButtonVariant, ControlDensity, ControlSize, IconSpec, ToastPosition, ToastStackSpec,
};

use crate::button::button;
use crate::color::{mix_srgb, with_alpha, TRANSPARENT, WHITE};
use crate::context::RenderContext;
use crate::icon::icon;
use crate::presentation::rem_to_px;

/// Host callbacks: dismiss and action, each carrying the toast's id.
#[derive(Default)]
pub struct ToastStackHandlers {
    pub on_dismiss: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_action: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Stable native instance scope. Toast ids are queue-local, so duplicate
    /// hosts may legitimately render the same id without sharing backend
    /// focus, hit-test, or element state.
    pub instance_id: Option<String>,
}

fn scoped(instance_id: Option<&str>, part: &str) -> Option<String> {
    instance_id.map(|scope| format!("toast-host:{scope}:{part}"))
}

fn position_role(position: ToastPosition) -> &'static str {
    match position {
        ToastPosition::TopRight => "top-right",
        ToastPosition::TopLeft => "top-left",
        ToastPosition::BottomRight => "bottom-right",
        ToastPosition::BottomLeft => "bottom-left",
    }
}

/// Per-size title font-size in rem (contract §8 size table).
fn title_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.71875,
        ControlSize::Sm | ControlSize::Md => 0.8125,
        ControlSize::Lg => 0.9375,
        ControlSize::Xl => 1.0,
    }
}

/// Per-size message font-size in rem (contract §8 size table).
fn message_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.6875,
        ControlSize::Sm => 0.75,
        ControlSize::Md => 0.8125,
        ControlSize::Lg => 0.875,
        ControlSize::Xl => 0.9375,
    }
}

/// Per-size dismiss square dimension in rem (contract §8 size table).
fn dismiss_size_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.0,
        ControlSize::Sm => 1.125,
        ControlSize::Md => 1.25,
        ControlSize::Lg => 1.5,
        ControlSize::Xl => 1.75,
    }
}

/// Per-size dismiss top/right inset in rem (contract §8 size table).
fn dismiss_inset_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.25,
        ControlSize::Sm | ControlSize::Md => 0.375,
        ControlSize::Lg | ControlSize::Xl => 0.5,
    }
}

/// Density toast-padding multiplier (contract §8 density table).
fn density_pad_scale(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.75,
        ControlDensity::Default => 1.0,
        ControlDensity::Comfortable => 1.25,
    }
}

fn all_corners(node: &mut Node, r: f32) {
    let c = &mut node.style.descriptor.corner_radii;
    c.top_left = r;
    c.top_right = r;
    c.bottom_right = r;
    c.bottom_left = r;
}

pub fn toast_stack(
    spec: &ToastStackSpec,
    ctx: &RenderContext<'_>,
    handlers: ToastStackHandlers,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let title_px = rem_to_px(title_font_rem(effective_size));
    let message_px = rem_to_px(message_font_rem(effective_size));
    let dismiss_px = rem_to_px(dismiss_size_rem(effective_size));
    let dismiss_inset = rem_to_px(dismiss_inset_rem(effective_size));
    let instance_id = handlers.instance_id.as_deref();

    // Contract §8 toast padding = space-panel-x scaled by density.
    let base_pad = ctx.theme().resolve_space(spec.padding_token());
    let pad = base_pad * density_pad_scale(density);
    // Contract §8: comfortable widens the stack gap to space-stack-lg;
    // compact and default retain space-stack-sm.
    let stack_gap = match density {
        ControlDensity::Comfortable => ctx.theme().resolve_space("space.stack.lg"),
        ControlDensity::Compact | ControlDensity::Default => {
            ctx.theme().resolve_space(spec.gap_token())
        }
    };
    let item_gap = ctx.theme().resolve_space(spec.gap_token());

    let elevated = ctx.theme().resolve_color(spec.fill_token());
    let border_default = ctx.theme().resolve_color(spec.border_token());
    let radius_base = ctx.theme().resolve_radius(spec.radius_token());
    // Contract §8: border-radius = calc(radius-surface - 0.125rem).
    let radius = (radius_base - rem_to_px(0.125)).max(0.0);
    let title_color = ctx.theme().resolve_color(spec.title_color_token());
    let message_color = ctx.theme().resolve_color(spec.message_color_token());
    let dismiss_color = ctx.theme().resolve_color(spec.dismiss_color_token());
    let dismiss_hover_color = ctx.theme().resolve_color(spec.title_color_token());
    let dismiss_hover_fill =
        with_alpha(ctx.theme().resolve_color("color.background.surface"), 0.60);

    let mut el = Node::container();
    el.runtime_id = scoped(instance_id, "stack");
    el.roles.insert(
        "size".to_owned(),
        format!("{effective_size:?}").to_ascii_lowercase(),
    );
    el.roles.insert(
        "density".to_owned(),
        format!("{density:?}").to_ascii_lowercase(),
    );
    el.roles.insert(
        "position".to_owned(),
        position_role(spec.position).to_owned(),
    );
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = stack_gap;
        s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(22.5));
    }
    // Corner-mounted overlay: keep the stack out of flow and anchor it to
    // the nearest relative host, matching the old GPUI wrapper.
    el.position = match spec.position {
        ToastPosition::TopRight => NodePosition::Absolute {
            top: Some(pad),
            right: Some(pad),
            left: None,
            bottom: None,
        },
        ToastPosition::TopLeft => NodePosition::Absolute {
            top: Some(pad),
            left: Some(pad),
            right: None,
            bottom: None,
        },
        ToastPosition::BottomRight => NodePosition::Absolute {
            bottom: Some(pad),
            right: Some(pad),
            top: None,
            left: None,
        },
        ToastPosition::BottomLeft => NodePosition::Absolute {
            bottom: Some(pad),
            left: Some(pad),
            top: None,
            right: None,
        },
    };

    for toast in &spec.toasts {
        let tone_color = ctx.theme().resolve_color(spec.tone_color(&toast.tone));

        // Contract §8 tone treatments:
        //   accent bar = color-mix(tone 94%, white)
        //   border     = color-mix(tone 34%, border-default)
        //   background = color-mix(tone 12%, elevated) tint
        let accent_bar_color = mix_srgb(tone_color, WHITE, 0.94);
        let toast_border = mix_srgb(tone_color, border_default, 0.34);
        let bg_tinted = mix_srgb(tone_color, elevated, 0.12);

        // Leading tone accent bar — contract §8: 0.1875rem (3px), full height.
        let mut accent_bar = Node::container();
        accent_bar.position = NodePosition::Absolute {
            top: Some(0.0),
            right: None,
            bottom: Some(0.0),
            left: Some(0.0),
        };
        {
            let s = &mut accent_bar.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(0.1875));
            s.descriptor.background = Some(accent_bar_color);
        }

        // Title + optional message column.
        let mut content = Node::container();
        {
            let s = &mut content.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = rem_to_px(0.25);
            s.descriptor.layout.width = LayoutSizing::Grow;
        }
        let mut title = Node::text(toast.title.as_str());
        title.style.descriptor.text_color = Some(title_color);
        title.style.text_size = Some(title_px);
        title.style.text_weight = Some(600);
        let mut content = content.child(title);

        if let Some(message) = &toast.message {
            let mut msg = Node::text(message.as_str());
            msg.style.descriptor.text_color = Some(message_color);
            msg.style.text_size = Some(message_px);
            content = content.child(msg);
        }
        // Optional action affordance — the contract-owned Button primitive.
        if let Some(action) = &toast.action_label {
            let on_click = handlers.on_action.as_ref().map(|handler| {
                let handler = Arc::clone(handler);
                let id = toast.id.clone();
                Arc::new(move || handler(&id)) as Arc<dyn Fn() + Send + Sync>
            });
            let mut action_button = button(
                &ButtonSpec::new()
                    .with_label(action.as_str())
                    .with_variant(ButtonVariant::Secondary)
                    .with_size(effective_size)
                    .with_density(density),
                ctx,
                on_click,
            );
            action_button.id = Some(format!("poodle-toast-action-{}", toast.id));
            action_button.runtime_id = scoped(instance_id, &format!("toast:{}:action", toast.id));
            action_button
                .roles
                .insert("dependency".to_owned(), "button".to_owned());

            let mut actions = Node::container();
            actions.runtime_id = scoped(instance_id, &format!("toast:{}:actions", toast.id));
            actions
                .roles
                .insert("part".to_owned(), "actions".to_owned());
            actions.style.descriptor.layout.spacing.margin.top = rem_to_px(0.25);
            content = content.child(actions.child(action_button));
        }

        // Dismiss affordance — a native button containing the real Icon
        // primitive. It stays a focus stop without a handler, matching a web
        // button whose event has no listener while keeping activation inert.
        let dismiss_aria = format!("Dismiss {}", toast.title);
        let mut dismiss_icon = icon(&IconSpec::new("x"), ctx);
        dismiss_icon.runtime_id = scoped(instance_id, &format!("toast:{}:dismiss-icon", toast.id));
        dismiss_icon.style.descriptor.text_color = Some(dismiss_color);
        dismiss_icon
            .roles
            .insert("dependency".to_owned(), "icon".to_owned());

        let mut dismiss = Node::button("");
        dismiss.id = Some(format!("poodle-toast-dismiss-{}", toast.id));
        dismiss.runtime_id = scoped(instance_id, &format!("toast:{}:dismiss", toast.id));
        dismiss.position = NodePosition::Absolute {
            top: Some(dismiss_inset),
            right: Some(dismiss_inset),
            left: None,
            bottom: None,
        };
        dismiss.a11y.role = Some(NodeRole::Button);
        dismiss.a11y.label = Some(dismiss_aria);
        dismiss.a11y.tab_index = Some(0);
        dismiss.interaction.focusable = true;
        dismiss.style.focus_ring = Some(FocusRing {
            color: ctx.theme().resolve_color("color.accent.focusRing"),
            width: ctx.theme().resolve_border_width("border.width.focus"),
            offset: rem_to_px(0.125),
        });
        {
            let s = &mut dismiss.style;
            s.descriptor.layout.width = LayoutSizing::Fixed(dismiss_px);
            s.descriptor.layout.height = LayoutSizing::Fixed(dismiss_px);
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = poodle_node::MainAxisAlignment::Center;
            s.descriptor.background = Some(TRANSPARENT);
            s.descriptor.border.width = 0.0;
            s.descriptor.cursor = CursorHint::Pointer;
            s.hover = Some(StylePatch {
                background: Some(dismiss_hover_fill),
                border_color: None,
                text_color: Some(dismiss_hover_color),
                opacity: None,
            });
        }
        all_corners(&mut dismiss, ctx.theme().resolve_radius("radius.sm"));
        if let Some(handler) = &handlers.on_dismiss {
            let handler = Arc::clone(handler);
            let id = toast.id.clone();
            dismiss.interaction.on_activate = Some(Arc::new(move || handler(&id)));
        }
        let dismiss = dismiss.child(dismiss_icon);

        // Toast box: tinted fill + fade gradient, tone border,
        // elevation-overlay shadow, clipped. Danger projects Alert; other
        // rows stay list items. That metadata is not a GPUI AT-parity claim.
        let mut toast_el = Node::container();
        toast_el.a11y.role = Some(if toast.tone == poodle_specs::ToastTone::Danger {
            NodeRole::Alert
        } else {
            NodeRole::ListItem
        });
        toast_el.position = NodePosition::Relative;
        toast_el.id = Some(format!("poodle-toast-{}", toast.id));
        toast_el.runtime_id = scoped(instance_id, &format!("toast:{}", toast.id));
        toast_el.roles.insert(
            "tone".to_owned(),
            format!("{:?}", toast.tone).to_ascii_lowercase(),
        );
        {
            let s = &mut toast_el.style;
            s.descriptor.background = Some(bg_tinted);
            // Contract §8: linear gradient (90deg) of tone tint fading into
            // elevated.
            s.gradient = Some((
                90.0,
                vec![
                    (mix_srgb(tone_color, elevated, 0.12), 0.0),
                    (elevated, 0.18),
                ],
            ));
            s.descriptor.border.width = 1.0;
            s.descriptor.border.color = toast_border;
            s.descriptor.layout.overflow_x = poodle_node::LayoutOverflow::Hidden;
            s.descriptor.layout.overflow_y = poodle_node::LayoutOverflow::Hidden;
            // Token-accurate elevation.overlay (single layer, spread 0).
            s.descriptor.shadow = Some(poodle_tokens::typed::semantic::ELEVATION_OVERLAY);
            let padc = &mut s.descriptor.layout.spacing.padding;
            padc.left = pad;
            padc.right = pad + rem_to_px(1.5);
            padc.top = pad;
            padc.bottom = pad;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
            s.descriptor.layout.spacing.gap = item_gap;
        }
        all_corners(&mut toast_el, radius);

        el = el.child(toast_el.child(accent_bar).child(content).child(dismiss));
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            el.a11y.label = Some(label.to_string());
        }
    }
    // Contract: the stack is a list of toasts.
    el.a11y.role = Some(NodeRole::List);
    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_node::NodeKind;
    use poodle_specs::{Toast, ToastTone};
    use std::sync::Mutex;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn preloaded_items_do_not_enter() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = ToastStackSpec::new().with_toasts(vec![Toast::new("save", "Saved")]);
        let node = toast_stack(&spec, &ctx, ToastStackHandlers::default());
        let toast = node
            .find(&|n| n.id.as_deref() == Some("poodle-toast-save"))
            .expect("toast exists");
        assert!(
            toast.style.animation.is_none(),
            "authored items paint the endpoint; construction does not attach enter"
        );
    }

    #[test]
    fn danger_uses_alert_and_other_tones_stay_list_items() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = ToastStackSpec::new().with_toasts(vec![
            Toast::new("ok", "Saved").with_tone(poodle_specs::ToastTone::Success),
            Toast::new("fail", "Publishing failed").with_tone(poodle_specs::ToastTone::Danger),
        ]);
        let node = toast_stack(&spec, &ctx, ToastStackHandlers::default());
        let success = node
            .find(&|n| n.id.as_deref() == Some("poodle-toast-ok"))
            .expect("success toast");
        let danger = node
            .find(&|n| n.id.as_deref() == Some("poodle-toast-fail"))
            .expect("danger toast");
        assert_eq!(success.a11y.role, Some(NodeRole::ListItem));
        assert_eq!(danger.a11y.role, Some(NodeRole::Alert));
    }

    #[test]
    fn contract_components_callbacks_and_scope_stay_distinct() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let seen = Arc::new(Mutex::new(Vec::new()));
        let action_seen = Arc::clone(&seen);
        let dismiss_seen = Arc::clone(&seen);
        let node = toast_stack(
            &ToastStackSpec::new()
                .with_toasts(vec![Toast::new("job", "Publishing")
                    .with_tone(ToastTone::Warning)
                    .with_action_label("Retry")])
                .with_size(ControlSize::Lg)
                .with_density(ControlDensity::Comfortable),
            &ctx,
            ToastStackHandlers {
                on_action: Some(Arc::new(move |id| {
                    action_seen
                        .lock()
                        .expect("seen lock")
                        .push(format!("action:{id}"));
                })),
                on_dismiss: Some(Arc::new(move |id| {
                    dismiss_seen
                        .lock()
                        .expect("seen lock")
                        .push(format!("dismiss:{id}"));
                })),
                instance_id: Some("subject".to_owned()),
            },
        );

        assert_eq!(node.runtime_id.as_deref(), Some("toast-host:subject:stack"));
        assert_eq!(node.roles.get("size").map(String::as_str), Some("lg"));
        assert_eq!(
            node.roles.get("density").map(String::as_str),
            Some("comfortable")
        );

        let toast = node
            .find(&|node| node.id.as_deref() == Some("poodle-toast-job"))
            .expect("toast row");
        assert_eq!(
            toast.runtime_id.as_deref(),
            Some("toast-host:subject:toast:job")
        );
        assert_eq!(toast.roles.get("tone").map(String::as_str), Some("warning"));

        let action = node
            .find(&|node| node.id.as_deref() == Some("poodle-toast-action-job"))
            .expect("action button");
        assert!(matches!(action.kind, NodeKind::Button { .. }));
        assert_eq!(
            action.roles.get("dependency").map(String::as_str),
            Some("button")
        );
        assert_eq!(
            action.runtime_id.as_deref(),
            Some("toast-host:subject:toast:job:action")
        );

        let dismiss = node
            .find(&|node| node.id.as_deref() == Some("poodle-toast-dismiss-job"))
            .expect("dismiss button");
        assert!(matches!(dismiss.kind, NodeKind::Button { .. }));
        assert!(dismiss.interaction.focusable);
        assert_eq!(dismiss.a11y.tab_index, Some(0));
        assert_eq!(dismiss.a11y.label.as_deref(), Some("Dismiss Publishing"));
        assert_eq!(
            dismiss.runtime_id.as_deref(),
            Some("toast-host:subject:toast:job:dismiss")
        );
        let icon = dismiss
            .find(&|node| matches!(&node.kind, NodeKind::Icon { name, .. } if name == "x"))
            .expect("dismiss icon");
        assert_eq!(
            icon.roles.get("dependency").map(String::as_str),
            Some("icon")
        );

        (action
            .interaction
            .on_activate
            .as_ref()
            .expect("action handler"))();
        (dismiss
            .interaction
            .on_activate
            .as_ref()
            .expect("dismiss handler"))();
        assert_eq!(
            seen.lock().expect("seen lock").as_slice(),
            ["action:job", "dismiss:job"]
        );
    }

    #[test]
    fn unavailable_action_is_reachable_but_inert() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = toast_stack(
            &ToastStackSpec::new().with_toasts(vec![
                Toast::new("job", "Publishing").with_action_label("Unavailable")
            ]),
            &ctx,
            ToastStackHandlers::default(),
        );
        let action = node
            .find(&|node| node.id.as_deref() == Some("poodle-toast-action-job"))
            .expect("action button");
        assert!(action.interaction.focusable);
        assert_eq!(action.a11y.tab_index, Some(0));
        assert!(action.interaction.on_activate.is_none());
    }
}
