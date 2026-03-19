//! PugTextInput — real GPUI component backed by TextInputSpec.
//!
//! Note: gpui doesn't provide a native text input widget, so this renders
//! a styled container displaying the current value. Real text editing would
//! require gpui's internal input handling.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{IconSize, IconSpec, TextInputSpec};

use crate::icon::PugIcon;
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI text input component backed by `TextInputSpec`.
pub struct PugTextInput {
    spec: TextInputSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_focus: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl PugTextInput {
    pub fn new(spec: TextInputSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_focus: None,
        }
    }

    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    pub fn on_focus(
        mut self,
        handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_focus = Some(Box::new(handler));
        self
    }
}

impl IntoElement for PugTextInput {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let control_height = resolve_px(theme, "semantic.size.control.height");
        let inline_padding = resolve_px(theme, "semantic.space.control.x");
        let inline_gap = resolve_px(theme, "semantic.space.inline.sm");
        let control_radius = resolve_radius(theme, "semantic.radius.control");

        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        let border = resolve_color(theme, spec.border_token());
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let elevated = resolve_color(theme, "semantic.color.background.elevated");

        let value = spec.current_value();
        let is_empty = value.is_empty();
        let display_text = if is_empty {
            spec.placeholder.clone().unwrap_or_default()
        } else {
            value.to_string()
        };
        let text_col = if is_empty { text_secondary } else { text_primary };

        // Contract: hover = color-mix with elevated
        let hover_bg = color_mix(surface_bg, elevated, 0.84);

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("pug-input-{}", suffix)
        } else {
            "pug-input".to_string()
        };

        let mut el = div()
            .id(SharedString::from(id_str))
            .h(control_height)
            .px(inline_padding)
            .rounded(control_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(border)
            .flex()
            .items_center()
            .gap(inline_gap)
            .text_sm();

        if spec.is_disabled {
            el = el.opacity(disabled_opacity);
        } else {
            el = el
                .cursor_pointer()
                .hover(move |s| s.bg(hover_bg));
        }

        // Leading icon — render via PugIcon
        if let Some(ref icon) = spec.leading_icon {
            el = el.child(
                PugIcon::new(
                    IconSpec::new(icon).with_size(IconSize::Sm),
                    theme,
                )
                .with_color(text_secondary),
            );
        }

        // Value / placeholder
        el = el.child(
            div()
                .flex_1()
                .text_color(text_col)
                .overflow_x_hidden()
                .text_ellipsis()
                .whitespace_nowrap()
                .child(display_text),
        );

        // Trailing icon — render via PugIcon
        if let Some(ref icon) = spec.trailing_icon {
            el = el.child(
                PugIcon::new(
                    IconSpec::new(icon).with_size(IconSize::Sm),
                    theme,
                )
                .with_color(text_secondary),
            );
        }

        // Focus click handler
        if let Some(handler) = self.on_focus {
            if !spec.is_disabled {
                el = el.on_click(move |_event, window, cx| {
                    handler(window, cx);
                });
            }
        }

        el.into_any_element()
    }
}
