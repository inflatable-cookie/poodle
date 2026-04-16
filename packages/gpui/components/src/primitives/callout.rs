//! Callout — real GPUI component backed by CallOutSpec (contract: callout).

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    CallOutSpec, ControlDensity, ControlSize, IconSize, IconSpec, SemanticControlSizeRole,
    StatusTone,
};

use super::icon::Icon;
use crate::presentation::{
    callout_dismiss_size_rem, callout_gap_rem, callout_icon_size_rem, panel_space_x_rem,
    panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
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
            spec,
            theme: theme.clone(),
            is_dismissible: false,
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
        let density_pad_x = px(rem_to_px(panel_space_x_rem(spec.density)));
        let density_pad_y = px(rem_to_px(panel_space_y_rem(spec.density)));

        let panel_x = density_pad_x;
        let panel_y = density_pad_y;

        let tone_color = resolve_color(theme, spec.fill_token());
        let surface_bg = resolve_color(theme, "color.background.surface");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let radius = resolve_radius(theme, "radius.surface");
        let control_radius = resolve_radius(theme, "radius.control");
        let outer_gap = px(rem_to_px(callout_gap_rem(effective_size)));
        let icon_size = px(rem_to_px(callout_icon_size_rem(effective_size)));
        let dismiss_size = px(rem_to_px(callout_dismiss_size_rem(effective_size)));
        let body_size = body_font;
        let label_size = resolve_px(theme, "typography.label.size");

        // Tone icon name
        let icon_name = match spec.tone {
            StatusTone::Info => "info",
            StatusTone::Success => "check-circle",
            StatusTone::Warning => "triangle-alert", // Svelte: "triangle-alert" (lucide)
            StatusTone::Danger => "alert-circle",
            _ => "info",
        };

        let panel_bg = resolve_color(theme, "color.background.panel");
        let border_default = resolve_color(theme, "color.border.default");
        let border_subtle = resolve_color(theme, "color.border.subtle");

        // Matches Svelte Callout.svelte:
        //   Neutral fill: color-mix(panel 94%, transparent)
        //   Toned fill: color-mix(tone 10%, panel)
        //   Neutral border: color-mix(border-subtle 88%, transparent)
        //   Toned border: color-mix(tone 34%, border-default)
        let is_neutral = matches!(spec.tone, StatusTone::Neutral);
        let bg = if is_neutral {
            Hsla {
                a: panel_bg.a * 0.94,
                ..panel_bg
            }
        } else {
            color_mix(tone_color, panel_bg, 0.10)
        };
        let border = if is_neutral {
            Hsla {
                a: border_subtle.a * 0.88,
                ..border_subtle
            }
        } else {
            color_mix(tone_color, border_default, 0.34)
        };

        let mut el = div()
            .w_full()
            .px(panel_x)
            .py(panel_y)
            .rounded(radius)
            .bg(bg)
            .border_1()
            .border_color(border)
            .flex()
            .gap(outer_gap);

        // Icon column — circular bg container, size per effective_size, surface at 78% opacity
        let icon_bg = Hsla {
            a: surface_bg.a * 0.78,
            ..surface_bg
        };
        el = el.child(
            div()
                .flex_shrink_0()
                .w(icon_size)
                .h(icon_size)
                .rounded(px(999.0))
                .bg(icon_bg)
                .flex()
                .items_center()
                .justify_center()
                .child(
                    Icon::from_spec(IconSpec::new(icon_name).with_size(IconSize::Sm), theme)
                        .with_color(tone_color),
                ),
        );

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

        // Dismiss button — size per effective_size, rounded control radius
        if self.is_dismissible {
            let dismiss_id = SharedString::from("poodle-callout-dismiss");
            let mut dismiss_btn = div()
                .id(dismiss_id)
                .flex_shrink_0()
                .w(dismiss_size)
                .h(dismiss_size)
                // Svelte: border-radius = control - 0.0625rem; margin-right = -0.5 * panel-x
                .rounded(control_radius - px(rem_to_px(0.0625)))
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .mr(panel_x * -0.5)
                .hover(move |s| s.bg(Hsla { a: 0.08, ..text_secondary }))
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
