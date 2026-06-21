//! ToastStack — real GPUI component backed by ToastStackSpec.

use std::rc::Rc;

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ButtonSpec, ButtonVariant, ControlDensity, ControlSize, IconSpec, SemanticControlSizeRole};
use poodle_specs::{Toast, ToastPosition, ToastStackSpec};

use crate::primitives::{Button, Icon};
use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{
    color_mix, elevation_overlay_shadow, resolve_color, resolve_px, resolve_radius,
};

/// A real GPUI toast stack component backed by `ToastStackSpec`.
///
/// Renders a stack of toast notifications at a chosen screen corner.
pub struct ToastStack {
    spec: ToastStackSpec,
    theme: GpuiThemeProvider,
    on_dismiss: Option<Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_action: Option<Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for ToastStack {
    type Target = ToastStackSpec;
    fn deref(&self) -> &ToastStackSpec {
        &self.spec
    }
}

impl ToastStack {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: ToastStackSpec::new(),
            theme: theme.clone(),
            on_dismiss: None,
            on_action: None,
        }
    }

    pub fn from_spec(spec: ToastStackSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_dismiss: None,
            on_action: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn toasts(mut self, v: Vec<Toast>) -> Self {
        self.spec.toasts = v;
        self
    }
    pub fn position(mut self, v: ToastPosition) -> Self {
        self.spec.position = v;
        self
    }
    pub fn with_size(mut self, v: ControlSize) -> Self {
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

    pub fn on_dismiss(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_dismiss = Some(Rc::new(handler));
        self
    }

    pub fn on_action(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_action = Some(Rc::new(handler));
        self
    }
}

/// Per-size title font-size in rem (contract §8 size table; md base 0.8125rem
/// `typography-label-size`, derived from the per-size `strong` overrides).
fn title_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.71875,
        ControlSize::Sm => 0.8125,
        ControlSize::Md => 0.8125,
        ControlSize::Lg => 0.9375,
        ControlSize::Xl => 1.0,
    }
}

/// Per-size message font-size in rem (contract §8 size table; md base 0.8125rem).
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
        ControlSize::Sm => 0.375,
        ControlSize::Md => 0.375,
        ControlSize::Lg => 0.5,
        ControlSize::Xl => 0.5,
    }
}

/// Density padding multiplier (contract §8: compact ×0.75, comfortable ×1.25).
fn density_pad_scale(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.75,
        ControlDensity::Default => 1.0,
        ControlDensity::Comfortable => 1.25,
    }
}

impl IntoElement for ToastStack {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        let gap = resolve_px(theme, spec.gap_token());
        let title_size = px(rem_to_px(title_font_rem(effective_size)));
        let message_size = px(rem_to_px(message_font_rem(effective_size)));
        // Contract §8 toast padding = space-panel-x scaled by density.
        let base_pad = resolve_px(theme, spec.padding_token());
        let pad = base_pad * density_pad_scale(spec.density);

        let elevated = resolve_color(theme, spec.fill_token());
        let border_default = resolve_color(theme, spec.border_token());
        let radius_base = resolve_radius(theme, spec.radius_token());
        // Contract §8: border-radius = calc(radius-surface - 0.125rem).
        let radius = px((f32::from(radius_base) - rem_to_px(0.125)).max(0.0));
        let title_color = resolve_color(theme, spec.title_color_token());
        let message_color = resolve_color(theme, spec.message_color_token());
        let dismiss_color = resolve_color(theme, spec.dismiss_color_token());
        let white = gpui::white();

        let dismiss_dim = px(rem_to_px(dismiss_size_rem(effective_size)));
        let dismiss_inset = px(rem_to_px(dismiss_inset_rem(effective_size)));

        // Container positioned at the chosen corner.
        let mut container = div()
            .absolute()
            .flex()
            .flex_col()
            .gap(gap)
            .w(px(rem_to_px(22.5))); // 360px container width

        match spec.position {
            ToastPosition::TopRight => container = container.top(pad).right(pad),
            ToastPosition::TopLeft => container = container.top(pad).left(pad),
            ToastPosition::BottomRight => container = container.bottom(pad).right(pad),
            ToastPosition::BottomLeft => container = container.bottom(pad).left(pad),
        }

        for toast in &spec.toasts {
            let tone_color = resolve_color(theme, spec.tone_color(&toast.tone));

            // Contract §8 tone treatments:
            //   accent bar  = color-mix(tone 82%, white 6%)
            //   border      = color-mix(tone 34%, border-default)
            //   background  = linear-gradient(90deg, color-mix(tone 12%, elevated),
            //                 elevated 18%) — tone tint fading into elevated.
            let accent_bar_color = color_mix(tone_color, white, 0.94); // 94% tone + 6% white ≈ 82/6 mix intent
            let toast_border = color_mix(tone_color, border_default, 0.34);
            // 90deg gradient: tone-tinted leading edge → flat elevated by 18%.
            // The contract's two transparent-composited layers collapse to this on
            // GPUI (no multi-layer alpha background channel) — matches Jetstream.
            let toast_bg_lead = color_mix(tone_color, elevated, 0.12);
            let toast_bg = gpui::linear_gradient(
                90.0,
                gpui::linear_color_stop(toast_bg_lead, 0.0),
                gpui::linear_color_stop(elevated, 0.18),
            );

            // Build the content column: title + optional message + optional action.
            let mut content_col = div()
                .flex()
                .flex_col()
                .gap(px(rem_to_px(0.25)))
                .flex_grow()
                .child(
                    div()
                        .text_size(title_size)
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(title_color)
                        .child(toast.title.clone()),
                );

            if let Some(ref message) = toast.message {
                content_col = content_col.child(
                    div()
                        .text_size(message_size)
                        .text_color(message_color)
                        .child(message.clone()),
                );
            }

            // Optional action — real Button primitive (variant Secondary).
            if let Some(ref action_label) = toast.action_label {
                let mut action_btn = Button::from_spec(
                    ButtonSpec::new()
                        .with_label(action_label.clone())
                        .with_variant(ButtonVariant::Secondary)
                        .with_size(effective_size)
                        .with_density(spec.density),
                    theme,
                );

                if let Some(ref handler) = self.on_action {
                    let handler = Rc::clone(handler);
                    let toast_id = toast.id.clone();
                    action_btn = action_btn
                        .on_click(move |_event, window, app| handler(&toast_id, window, app));
                }

                content_col = content_col.child(div().mt(px(rem_to_px(0.25))).child(action_btn));
            }

            // Dismiss button — Icon primitive (name="x") with aria-label.
            let dismiss_element_id = SharedString::from(format!("toast-dismiss-{}", toast.id));
            let dismiss_aria = format!("Dismiss {}", toast.title);
            let mut dismiss_btn = div()
                .id(dismiss_element_id)
                .absolute()
                .top(dismiss_inset)
                .right(dismiss_inset)
                .w(dismiss_dim)
                .h(dismiss_dim)
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .child(
                    Icon::from_spec(IconSpec::new("x"), theme)
                        .aria_label(dismiss_aria)
                        .with_color(dismiss_color),
                );

            if let Some(ref handler) = self.on_dismiss {
                let handler = Rc::clone(handler);
                let toast_id = toast.id.clone();
                dismiss_btn = dismiss_btn
                    .on_click(move |_event, window, app| handler(&toast_id, window, app));
            }

            // Toast: relative root so the accent bar + dismiss can be absolute.
            // Contract §8: box-shadow = elevation-overlay.
            let toast_el = div()
                .relative()
                .flex()
                .flex_row()
                .items_start()
                .bg(toast_bg)
                .border_1()
                .border_color(toast_border)
                .rounded(radius)
                .overflow_hidden()
                .shadow(elevation_overlay_shadow())
                .child(
                    // Left accent bar — contract §8: 0.1875rem (3px).
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .bottom_0()
                        .w(px(rem_to_px(0.1875)))
                        .bg(accent_bar_color),
                )
                .child(
                    // Content area: padding, plus extra right inset for dismiss.
                    div()
                        .flex()
                        .flex_row()
                        .items_start()
                        .flex_grow()
                        .pl(pad)
                        .py(pad)
                        .pr(pad + px(rem_to_px(1.5)))
                        .child(content_col),
                )
                .child(dismiss_btn);

            container = container.child(toast_el);
        }

        container.into_any_element()
    }
}
