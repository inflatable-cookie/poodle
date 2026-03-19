//! PugRadioGroup — real GPUI component backed by RadioGroupSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::RadioGroupSpec;

use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px};

/// A real GPUI radio group component backed by `RadioGroupSpec`.
pub struct PugRadioGroup {
    spec: RadioGroupSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl PugRadioGroup {
    pub fn new(spec: RadioGroupSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "pug-radio".to_string(),
            on_change: None,
        }
    }

    pub fn with_id(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    pub fn on_change(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl IntoElement for PugRadioGroup {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;

        // Contract: gap = space-inline-sm
        let inline_gap = resolve_px(theme, "semantic.space.inline.sm");
        // Contract: indicator = 1.125rem (18px), same as checkbox
        let indicator_f = theme.resolve_space("semantic.size.icon.md");
        let indicator_size = px(indicator_f);
        let indicator_half = px(indicator_f / 2.0);
        // Contract: inner dot = ~33% of indicator
        let dot_f = indicator_f / 3.0;
        let dot_size = px(dot_f);
        let dot_half = px(dot_f / 2.0);

        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        let accent = resolve_color(theme, "semantic.color.accent.base");
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_inverse = resolve_color(theme, "semantic.color.text.inverse");
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let elevated = resolve_color(theme, "semantic.color.background.elevated");

        // Contract: hover = color-mix with elevated
        let hover_bg = color_mix(surface_bg, elevated, 0.84);

        let current_value = self.spec.current_value().map(|s| s.to_string());
        let is_group_disabled = self.spec.is_disabled;

        let mut group = div().flex().flex_col().gap(inline_gap);

        if let Some(ref label) = self.spec.aria_label {
            group = group.child(
                div().text_sm().text_color(text_primary).child(label.to_string()),
            );
        }

        for option in &self.spec.options {
            let is_selected = current_value.as_deref() == Some(&option.value);
            let is_disabled = is_group_disabled || option.is_disabled;
            let option_id =
                SharedString::from(format!("{}-{}", self.id_prefix, option.value));

            // Radio circle indicator
            let indicator = {
                let mut circle = div()
                    .w(indicator_size)
                    .h(indicator_size)
                    .rounded(indicator_half)
                    .flex()
                    .items_center()
                    .justify_center()
                    .flex_shrink_0();

                if is_selected {
                    circle = circle
                        .bg(accent)
                        .border_1()
                        .border_color(accent)
                        .child(
                            div()
                                .w(dot_size)
                                .h(dot_size)
                                .rounded(dot_half)
                                .bg(text_inverse),
                        );
                } else {
                    circle = circle
                        .bg(surface_bg)
                        .border_1()
                        .border_color(border);
                }

                circle
            };

            let mut row = div()
                .id(option_id)
                .flex()
                .items_center()
                .gap(inline_gap);

            if is_disabled {
                row = row.opacity(disabled_opacity);
            } else {
                row = row
                    .cursor_pointer()
                    .hover(move |s| s.bg(hover_bg));
            }

            row = row
                .child(indicator)
                .child(div().text_sm().text_color(text_primary).child(option.label.clone()));

            group = group.child(row);
        }

        group.into_any_element()
    }
}
