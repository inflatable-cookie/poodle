//! PugSelect — real GPUI component backed by SelectSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::SelectSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI select/dropdown component backed by `SelectSpec`.
pub struct PugSelect {
    spec: SelectSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_toggle: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl PugSelect {
    pub fn new(spec: SelectSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_toggle: None,
            on_change: None,
        }
    }

    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    pub fn on_toggle(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Box::new(handler));
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

impl IntoElement for PugSelect {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let border = resolve_color(theme, "semantic.color.border.default");
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let accent = resolve_color(theme, "semantic.color.accent.base");
        let elevated_bg = resolve_color(theme, spec.overlay_fill_token());

        let trigger_text = spec.trigger_text().unwrap_or(
            spec.placeholder.as_deref().unwrap_or("Select..."),
        );
        let is_placeholder = spec.trigger_text().is_none();
        let is_open = spec.current_open();
        let is_disabled = spec.is_disabled;

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("pug-select-{}", suffix)
        } else {
            "pug-select".to_string()
        };

        // Trigger button
        let mut trigger = div()
            .id(SharedString::from(id_str))
            .h(px(36.0))
            .px(px(12.0))
            .rounded(px(6.0))
            .bg(surface_bg)
            .border_1()
            .border_color(if is_open { accent } else { border })
            .flex()
            .items_center()
            .justify_between()
            .gap(px(8.0))
            .text_sm();

        if is_disabled {
            trigger = trigger.opacity(0.48);
        } else {
            trigger = trigger
                .cursor_pointer()
                .hover(|s| s.bg(hsla(0.0, 0.0, 0.5, 0.04)));
        }

        let text_col = if is_placeholder {
            text_secondary
        } else {
            text_primary
        };

        trigger = trigger
            .child(
                div().text_color(text_col).child(trigger_text.to_string()),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(text_secondary)
                    .child(if is_open { "▴" } else { "▾" }),
            );

        if let Some(handler) = self.on_toggle {
            if !is_disabled {
                let next_open = !is_open;
                trigger = trigger.on_click(move |_event, window, cx| {
                    handler(&next_open, window, cx);
                });
            }
        }

        let mut wrapper = div().flex().flex_col().gap(px(4.0)).child(trigger);

        // Dropdown list (when open)
        if is_open {
            let mut list = div()
                .rounded(px(6.0))
                .bg(elevated_bg)
                .border_1()
                .border_color(border)
                .shadow_md()
                .py(px(4.0));

            // Collect option values for click handlers
            let option_values: Vec<String> = spec.options.iter().map(|o| o.value.clone()).collect();

            for (idx, option) in spec.options.iter().enumerate() {
                let is_selected =
                    spec.current_value() == Some(option.value.as_str());
                let is_opt_disabled = option.is_disabled;

                let item_id = SharedString::from(format!(
                    "pug-select-opt-{}",
                    option.value
                ));

                let mut item = div()
                    .id(item_id)
                    .px(px(10.0))
                    .py(px(6.0))
                    .text_sm()
                    .text_color(text_primary);

                if is_selected {
                    item = item
                        .bg(accent.opacity(0.1))
                        .text_color(accent);
                }

                if is_opt_disabled {
                    item = item.opacity(0.48);
                } else {
                    item = item
                        .cursor_pointer()
                        .hover(|s| s.bg(accent.opacity(0.08)));
                }

                item = item.child(option.label.clone());
                list = list.child(item);
            }

            wrapper = wrapper.child(list);
        }

        wrapper.into_any_element()
    }
}
