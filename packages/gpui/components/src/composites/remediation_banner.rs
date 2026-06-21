//! RemediationBanner — real GPUI component backed by RemediationBannerSpec.
//!
//! Contract: `docs/contracts/components/remediation-banner.md`
//! Reference: Svelte component NOT YET BUILT — the contract is the sole
//! authority (see `docs/parity/remediation-banner.md`). Jetstream mirrors the
//! same contract.
//!
//! Renders an announcing recovery banner: tone-tinted surface + border, a
//! tone-based leading icon, a title (`<strong>`) + message (`<p>`), up to two
//! `RemediationAction` buttons, and an optional dismiss control.
//!
//! GPUI emits no `aria-live` (contract §8 Known Delta); the announce role from
//! `spec.accessibility_role()` is a host concern.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ButtonSpec, IconSize, IconSpec, RemediationBannerSpec};

use crate::primitives::{Button, Icon};
use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius};

/// A real GPUI remediation banner backed by `RemediationBannerSpec`.
pub struct RemediationBanner {
    spec: RemediationBannerSpec,
    theme: GpuiThemeProvider,
    on_action: Option<Box<dyn Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static>>,
    on_dismiss: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for RemediationBanner {
    type Target = RemediationBannerSpec;
    fn deref(&self) -> &RemediationBannerSpec {
        &self.spec
    }
}

impl RemediationBanner {
    pub fn new(
        title: impl Into<String>,
        message: impl Into<String>,
        theme: &GpuiThemeProvider,
    ) -> Self {
        Self::from_spec(RemediationBannerSpec::new(title, message), theme)
    }

    pub fn from_spec(spec: RemediationBannerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_action: None,
            on_dismiss: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn tone(mut self, v: poodle_specs::StatusTone) -> Self {
        self.spec.tone = v;
        self
    }
    pub fn title(mut self, v: impl Into<String>) -> Self {
        self.spec.title = v.into();
        self
    }
    pub fn message(mut self, v: impl Into<String>) -> Self {
        self.spec.message = v.into();
        self
    }
    pub fn with_announce_mode(mut self, v: poodle_specs::AnnouncementMode) -> Self {
        self.spec.announce_mode = v;
        self
    }
    pub fn primary_action(mut self, v: poodle_specs::RemediationAction) -> Self {
        self.spec.primary_action = Some(v);
        self
    }
    pub fn secondary_action(mut self, v: poodle_specs::RemediationAction) -> Self {
        self.spec.secondary_action = Some(v);
        self
    }
    pub fn dismissible(mut self, v: bool) -> Self {
        self.spec.is_dismissible = v;
        self
    }

    pub fn on_action(
        mut self,
        handler: impl Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_action = Some(Box::new(handler));
        self
    }

    pub fn on_dismiss(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_dismiss = Some(Box::new(handler));
        self
    }
}

impl IntoElement for RemediationBanner {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Colors (all token-resolved) ──
        let tone_color = resolve_color(theme, spec.border_token());
        let panel = resolve_color(theme, spec.background_token());
        let icon_color = resolve_color(theme, spec.icon_color_token());
        let text_primary = resolve_color(theme, spec.title_color_token());
        let text_secondary = resolve_color(theme, spec.message_color_token());

        // Surface fill = color-mix(tone, panel) at the spec ratio; border = tone.
        let fill = color_mix(tone_color, panel, spec.fill_tone_ratio());
        let border = tone_color;

        // ── Dimensions ──
        let radius = resolve_radius(theme, spec.radius_token());
        let border_width = resolve_px(theme, spec.border_width_token());
        let title_size = resolve_px(theme, "typography.body.size");
        let message_size = resolve_px(theme, "typography.label.size");
        let pad_x = resolve_px(theme, "space.panel.x");
        let pad_y = resolve_px(theme, "space.panel.y");
        let gap = resolve_px(theme, "space.inline.md");
        let content_gap = resolve_px(theme, "space.inline.xs");
        let action_gap = resolve_px(theme, "space.inline.sm");

        let mut el = div()
            .w_full()
            .flex()
            .gap(gap)
            .items_start()
            .px(pad_x)
            .py(pad_y)
            .rounded(radius)
            .bg(fill)
            .border(border_width)
            .border_color(border);

        // ── Icon (contract §2: tone-based leading indicator) ──
        el = el.child(
            div().flex_shrink_0().child(
                Icon::from_spec(
                    IconSpec::new(spec.tone_icon_name()).with_size(IconSize::Md),
                    theme,
                )
                .with_color(icon_color),
            ),
        );

        // ── Content column: Title + Message + Actions ──
        let mut content = div()
            .flex()
            .flex_col()
            .gap(content_gap)
            .flex_1()
            .min_w(px(0.0));

        content = content.child(
            div()
                .text_size(title_size)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(text_primary)
                .child(spec.title.clone()),
        );

        content = content.child(
            div()
                .text_size(message_size)
                .text_color(text_secondary)
                .child(spec.message.clone()),
        );

        // ── Actions row: real Buttons honoring variant + is_disabled ──
        if spec.action_count() > 0 {
            let mut actions_row = div()
                .flex()
                .flex_wrap()
                .items_center()
                .gap(action_gap)
                .pt(content_gap);

            let on_action = self.on_action.map(std::rc::Rc::new);

            for action in [spec.primary_action.as_ref(), spec.secondary_action.as_ref()]
                .into_iter()
                .flatten()
            {
                let action_id = action.id.clone();
                let mut btn = Button::from_spec(
                    ButtonSpec::new()
                        .with_variant(action.variant)
                        .with_label(action.label.clone())
                        .with_disabled(action.is_disabled),
                    theme,
                )
                .with_id(format!("remediation-action-{}", action.id));

                if let Some(ref handler) = on_action {
                    let handler = handler.clone();
                    btn = btn.on_click(move |event, window, cx| {
                        handler(&action_id, event, window, cx);
                    });
                }

                actions_row = actions_row.child(btn);
            }

            content = content.child(actions_row);
        }

        el = el.child(content);

        // ── Dismiss (contract §5: aria-label="Dismiss"; GPUI has no ARIA) ──
        if spec.is_dismissible {
            let dismiss_id = SharedString::from("remediation-banner-dismiss");
            // Ghost-style hover overlay. TOKEN GAP: no semantic state-overlay
            // alpha token exists (~0.08); matches the Callout dismiss affordance.
            let hover_alpha = 0.08_f32;
            let mut dismiss_btn = div()
                .id(dismiss_id)
                .flex_shrink_0()
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .rounded(resolve_radius(theme, "radius.control"))
                .hover(move |s| {
                    s.bg(Hsla {
                        a: text_secondary.a * hover_alpha,
                        ..text_secondary
                    })
                })
                .child(
                    Icon::from_spec(IconSpec::new("x").with_size(IconSize::Sm), theme)
                        .with_color(text_secondary),
                );

            if let Some(handler) = self.on_dismiss {
                dismiss_btn =
                    dismiss_btn.on_click(move |event, window, cx| handler(event, window, cx));
            }

            el = el.child(dismiss_btn);
        }

        el.into_any_element()
    }
}
