//! PugSelect — real GPUI component backed by SelectSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{IconSize, IconSpec, SelectSpec};

use crate::icon::PugIcon;
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

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

        let control_height = resolve_px(theme, "semantic.size.control.height");
        let inline_padding = resolve_px(theme, "semantic.space.control.x");
        let inline_gap = resolve_px(theme, "semantic.space.inline.sm");
        let control_radius = resolve_radius(theme, "semantic.radius.control");
        let stack_gap = resolve_px(theme, "semantic.space.stack.sm");

        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        let border = resolve_color(theme, "semantic.color.border.default");
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let elevated_bg = resolve_color(theme, spec.overlay_fill_token());
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let accent = resolve_color(theme, "semantic.color.accent.base");

        // Contract: hover = color-mix(fill 84%, elevated)
        let hover_bg = color_mix(surface_bg, elevated_bg, 0.84);

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
            .h(control_height)
            .px(inline_padding)
            .rounded(control_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(if is_open { accent } else { border })
            .flex()
            .items_center()
            .justify_between()
            .gap(inline_gap)
            .text_sm();

        if is_disabled {
            trigger = trigger.opacity(disabled_opacity);
        } else {
            trigger = trigger
                .cursor_pointer()
                .hover(move |s| s.bg(hover_bg));
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
                    .opacity(0.5)
                    .child(
                        PugIcon::new(
                            IconSpec::new(if is_open { "chevron-up" } else { "chevron-down" })
                                .with_size(IconSize::Sm),
                            theme,
                        )
                        .with_color(text_secondary),
                    ),
            );

        if let Some(handler) = self.on_toggle {
            if !is_disabled {
                let next_open = !is_open;
                trigger = trigger.on_click(move |_event, window, cx| {
                    handler(&next_open, window, cx);
                });
            }
        }

        let mut wrapper = div().flex().flex_col().gap(stack_gap).child(trigger);

        // Dropdown list (when open)
        if is_open {
            let option_hover = color_mix(accent, elevated_bg, 0.08);
            let option_selected = color_mix(accent, elevated_bg, 0.10);

            let mut list = div()
                .rounded(control_radius)
                .bg(elevated_bg)
                .border_1()
                .border_color(border)
                .shadow_md()
                .py(stack_gap);

            for option in spec.options.iter() {
                let is_selected =
                    spec.current_value() == Some(option.value.as_str());
                let is_opt_disabled = option.is_disabled;

                let item_id = SharedString::from(format!(
                    "pug-select-opt-{}",
                    option.value
                ));

                let mut item = div()
                    .id(item_id)
                    .px(inline_padding)
                    .py(stack_gap)
                    .text_sm()
                    .text_color(text_primary);

                if is_selected {
                    item = item
                        .bg(option_selected)
                        .text_color(accent);
                }

                if is_opt_disabled {
                    item = item.opacity(disabled_opacity);
                } else {
                    item = item
                        .cursor_pointer()
                        .hover(move |s| s.bg(option_hover));
                }

                item = item.child(option.label.clone());
                list = list.child(item);
            }

            wrapper = wrapper.child(list);
        }

        wrapper.into_any_element()
    }
}
