//! PugButton — real GPUI component backed by ButtonSpec.
//!
//! Resolves all tokens through the spec's token methods and GpuiThemeProvider,
//! producing a fully styled, interactive gpui element with hover/active/disabled states.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{ButtonSpec, ButtonVariant};

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI button component backed by `ButtonSpec`.
///
/// Usage:
/// ```ignore
/// PugButton::new(
///     ButtonSpec::new().with_variant(ButtonVariant::Primary).with_label("Save"),
///     &theme,
/// )
/// .on_click(|_event, _window, _cx| { /* handler */ })
/// ```
pub struct PugButton {
    spec: ButtonSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl PugButton {
    pub fn new(spec: ButtonSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_click: None,
        }
    }

    /// Set a unique ID suffix to disambiguate multiple buttons.
    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    /// Set the click handler.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl IntoElement for PugButton {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // Resolve all tokens through the spec + theme
        let fill = resolve_color(theme, spec.resolved_fill_token());
        let text_color = resolve_color(theme, spec.resolved_text_token());
        let border_color = resolve_color(theme, spec.resolved_border_token());
        let radius = resolve_radius(theme, spec.radius_token());
        let height = resolve_px(theme, spec.control_height_token());
        let pad_x = resolve_px(theme, spec.horizontal_padding_token());
        let min_width = resolve_px(theme, spec.control_min_width_token());

        // Build element ID
        let label_text = spec.label.clone().unwrap_or_default();
        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("pug-btn-{}", suffix)
        } else {
            format!("pug-btn-{}", label_text)
        };
        let id = SharedString::from(id_str);

        // Compute hover/active colors based on variant
        let hover_fill = match spec.variant {
            ButtonVariant::Ghost => fill.opacity(0.08),
            _ => fill.opacity(0.85),
        };
        let active_fill = match spec.variant {
            ButtonVariant::Ghost => fill.opacity(0.14),
            _ => fill.opacity(0.7),
        };
        let hover_border = border_color.opacity(0.78);

        let is_disabled = spec.is_disabled || spec.is_loading;
        let is_ghost = spec.variant == ButtonVariant::Ghost;

        // Build the element
        let mut el = div()
            .id(id)
            .h(height)
            .min_w(min_width)
            .px(pad_x)
            .rounded(radius)
            .bg(fill)
            .text_color(text_color)
            .flex()
            .items_center()
            .justify_center()
            .gap(px(6.0))
            .text_sm();

        // Border — ghost variant gets no visible border
        if is_ghost {
            el = el.border_1().border_color(gpui::transparent_black());
        } else {
            el = el.border_1().border_color(border_color);
        }

        // Interactive states
        if is_disabled {
            let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
            el = el.opacity(disabled_opacity);
        } else {
            el = el
                .cursor_pointer()
                .hover(move |s| s.bg(hover_fill).border_color(hover_border))
                .active(move |s| s.bg(active_fill));
        }

        // Loading spinner indicator
        if spec.is_loading {
            el = el.child(
                div()
                    .text_xs()
                    .child("⟳"),
            );
        }

        // Leading icon placeholder
        if let Some(ref icon) = spec.leading_icon {
            el = el.child(
                div().text_xs().child(icon.clone()),
            );
        }

        // Label
        if !label_text.is_empty() {
            el = el.child(label_text);
        }

        // Trailing icon placeholder
        if let Some(ref icon) = spec.trailing_icon {
            el = el.child(
                div().text_xs().child(icon.clone()),
            );
        }

        // Click handler
        if let Some(handler) = self.on_click {
            if !is_disabled {
                el = el.on_click(move |event, window, cx| handler(event, window, cx));
            }
        }

        el.into_any_element()
    }
}
