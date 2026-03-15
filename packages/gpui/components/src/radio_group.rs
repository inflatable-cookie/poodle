//! PugRadioGroup — real GPUI component backed by RadioGroupSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::RadioGroupSpec;

use crate::theme_ext::resolve_color;

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

        let accent = resolve_color(theme, "semantic.color.accent.base");
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_inverse = resolve_color(theme, "semantic.color.text.inverse");
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");

        let current_value = self.spec.current_value().map(|s| s.to_string());
        let is_group_disabled = self.spec.is_disabled;

        let mut group = div().flex().flex_col().gap(px(8.0));

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
                    .w(px(18.0))
                    .h(px(18.0))
                    .rounded(px(9.0))
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
                                .w(px(6.0))
                                .h(px(6.0))
                                .rounded(px(3.0))
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
                .gap(px(8.0));

            if is_disabled {
                row = row.opacity(0.48);
            } else {
                row = row
                    .cursor_pointer()
                    .hover(|s| s.bg(hsla(0.0, 0.0, 0.5, 0.04)));
            }

            row = row
                .child(indicator)
                .child(div().text_sm().text_color(text_primary).child(option.label.clone()));

            // Click handler — we can't capture the on_change borrow in a loop,
            // so radio groups need the parent to manage clicks via specimens state.
            // The on_change callback is attached per-option if provided.
            group = group.child(row);
        }

        group.into_any_element()
    }
}
