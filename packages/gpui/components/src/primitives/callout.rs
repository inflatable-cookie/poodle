//! Callout — real GPUI component backed by CallOutSpec (contract: callout).

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    CallOutSpec, CalloutAnnounceMode, ControlDensity, ControlSize, IconSize, IconSpec,
    SemanticControlSizeRole, SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant, StatusTone,
};

use super::icon::Icon;
use super::spinner::Spinner;
use crate::presentation::{
    callout_dismiss_size_rem, callout_gap_rem, callout_icon_size_rem, panel_space_x_rem,
    rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius};

/// A real GPUI call-out component backed by `CallOutSpec`.
pub struct Callout {
    spec: CallOutSpec,
    theme: GpuiThemeProvider,
    is_dismissible: bool,
    on_dismiss: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Callout {
    type Target = CallOutSpec;
    fn deref(&self) -> &CallOutSpec {
        &self.spec
    }
}

impl Callout {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: CallOutSpec::new(),
            theme: theme.clone(),
            is_dismissible: false,
            on_dismiss: None,
        }
    }

    pub fn from_spec(spec: CallOutSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            is_dismissible: spec.dismissible,
            spec,
            theme: theme.clone(),
            on_dismiss: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn tone(mut self, v: StatusTone) -> Self {
        self.spec.tone = v;
        self
    }
    pub fn title(mut self, v: impl Into<String>) -> Self {
        self.spec.title = Some(v.into());
        self
    }
    pub fn content(mut self, v: impl Into<String>) -> Self {
        self.spec.content = Some(v.into());
        self
    }

    /// Alias for `content` — some contracts call it `message`.
    pub fn message(mut self, v: impl Into<String>) -> Self {
        self.spec.content = Some(v.into());
        self
    }

    pub fn dismissible(mut self, v: bool) -> Self {
        self.is_dismissible = v;
        self.spec.dismissible = v;
        self
    }
    pub fn with_announce_mode(mut self, v: CalloutAnnounceMode) -> Self {
        self.spec.announce_mode = v;
        self
    }
    pub fn with_aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn with_dismiss_label(mut self, v: impl Into<String>) -> Self {
        self.spec.dismiss_label = v.into();
        self
    }
    pub fn size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn with_density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
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

impl IntoElement for Callout {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let body_font = px(rem_to_px(size_font_rem(effective_size)));
        // Svelte: X = compact=0.5, default=panel_space_x=1.0, comfortable=1.25rem
        // compact is 0.5rem (not panel_space_x_rem which gives 0.75rem)
        let panel_x = px(rem_to_px(match spec.density {
            poodle_specs::ControlDensity::Compact => 0.5,
            poodle_specs::ControlDensity::Default => panel_space_x_rem(spec.density),
            poodle_specs::ControlDensity::Comfortable => panel_space_x_rem(spec.density),
        }));
        // Svelte: Y is hardcoded 0.625rem (all densities)
        let panel_y = px(rem_to_px(0.625));

        // Tone base color (info → dedicated status-info, pending → accent,
        // neutral → text-secondary). Resolved through the spec so info no longer
        // tints with accent.
        let tone_color = resolve_color(theme, spec.tone_color_token());
        let surface_bg = resolve_color(theme, spec.icon_badge_token());
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let radius = resolve_radius(theme, "radius.surface");
        let control_radius = resolve_radius(theme, "radius.control");
        let border_width = resolve_px(theme, spec.border_width_token());
        let outer_gap = px(rem_to_px(callout_gap_rem(effective_size)));
        let icon_size = px(rem_to_px(callout_icon_size_rem(effective_size)));
        let dismiss_size = px(rem_to_px(callout_dismiss_size_rem(effective_size)));
        let body_size = body_font;
        let label_size = resolve_px(theme, "typography.label.size");

        // Tone icon name (contract §6 icon map). Pending renders a ring spinner
        // instead of an icon (handled below).
        let icon_name = match spec.tone {
            StatusTone::Neutral | StatusTone::Info => "info",
            StatusTone::Success => "check",
            StatusTone::Warning => "triangle-alert",
            StatusTone::Danger => "circle-x",
            StatusTone::Pending => "info", // unused: pending uses the spinner branch
        };

        let panel_bg = resolve_color(theme, spec.fill_mix_target_token());
        let border_default = resolve_color(theme, spec.border_mix_target_token());
        let border_subtle = resolve_color(theme, spec.neutral_border_token());

        // Per-tone fill/border (contract §8):
        //   neutral: panel 94% / border-subtle 88% (mixed with transparent)
        //   pending: accent 8% / 26% over panel / border-default
        //   info/success/warning/danger: status 10% / 34% over panel / border-default
        let is_neutral = spec.is_neutral_tone();
        let is_pending = spec.is_pending_tone();
        let bg = if is_neutral {
            Hsla {
                a: panel_bg.a * 0.94,
                ..panel_bg
            }
        } else if is_pending {
            color_mix(tone_color, panel_bg, 0.08)
        } else {
            color_mix(tone_color, panel_bg, 0.10)
        };
        let border = if is_neutral {
            Hsla {
                a: border_subtle.a * 0.88,
                ..border_subtle
            }
        } else if is_pending {
            color_mix(tone_color, border_default, 0.26)
        } else {
            color_mix(tone_color, border_default, 0.34)
        };

        let mut el = div()
            .w_full()
            .px(panel_x)
            .py(panel_y)
            .rounded(radius)
            .bg(bg)
            .border(border_width)
            .border_color(border)
            .flex()
            .gap(outer_gap);

        // Icon column — circular bg container, size per effective_size, surface at 78% opacity.
        // Pending tone hosts the shared ring spinner (accent) instead of a tone icon.
        let icon_bg = Hsla {
            a: surface_bg.a * 0.78,
            ..surface_bg
        };
        let mut icon_badge = div()
            .flex_shrink_0()
            .w(icon_size)
            .h(icon_size)
            .rounded(px(999.0))
            .bg(icon_bg)
            .flex()
            .items_center()
            .justify_center();
        if is_pending {
            icon_badge = icon_badge.child(
                Spinner::from_spec(
                    SpinnerSpec::new()
                        .with_variant(SpinnerVariant::Ring)
                        .with_size(SpinnerSize::Sm)
                        .with_tone(SpinnerTone::Accent),
                    theme,
                ),
            );
        } else {
            icon_badge = icon_badge.child(
                Icon::from_spec(IconSpec::new(icon_name).with_size(IconSize::Sm), theme)
                    .with_color(tone_color),
            );
        }
        el = el.child(icon_badge);

        // Content column — gap: space.inline.sm (matches Svelte .callout__content { gap })
        let mut content_col = div()
            .flex()
            .flex_col()
            .gap(resolve_px(theme, "space.inline.sm"))
            .flex_1()
            .min_w(px(0.0));

        // Title: label family/size/weight
        if let Some(ref title) = spec.title {
            content_col = content_col.child(
                div()
                    .text_size(body_size)
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_primary)
                    .child(title.clone()),
            );
        }

        // Content: Svelte uses 0.8125rem (13px)
        if let Some(ref content) = spec.content {
            content_col = content_col.child(
                div()
                    .text_size(label_size)
                    .text_color(text_secondary)
                    .child(content.clone()),
            );
        }

        el = el.child(content_col);

        // Dismiss button — size per effective_size, rounded control radius.
        // Reads from the spec field (set by `from_spec` or the `dismissible`
        // builder), so both construction paths agree.
        if self.is_dismissible || spec.dismissible {
            let dismiss_id = SharedString::from("poodle-callout-dismiss");
            // Svelte: border-radius = control - 0.0625rem (border-width token);
            // margin-right = -0.5 * panel-x.
            let radius_offset = resolve_px(theme, spec.border_width_token());
            // Hover overlay alpha for the ghost dismiss button. TOKEN GAP: no
            // semantic state-overlay alpha token exists (~0.08); the contract
            // token table omits the dismiss hover affordance entirely. Kept as a
            // small fixed alpha until a `state.overlay.*` token is added.
            let hover_alpha = 0.08_f32;
            let mut dismiss_btn = div()
                .id(dismiss_id)
                .flex_shrink_0()
                .w(dismiss_size)
                .h(dismiss_size)
                .rounded(control_radius - radius_offset)
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .mr(panel_x * -0.5)
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
