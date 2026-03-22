//! Select — real GPUI component backed by SelectSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{ChoiceOption, IconSize, IconSpec, SelectSpec};

use super::icon::Icon;
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI select/dropdown component backed by `SelectSpec`.
pub struct Select {
    spec: SelectSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_toggle: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Select {
    type Target = SelectSpec;
    fn deref(&self) -> &SelectSpec { &self.spec }
}

impl Select {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: SelectSpec::default(), theme: theme.clone(), id_suffix: None, on_toggle: None, on_change: None }
    }

    pub fn from_spec(spec: SelectSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_toggle: None,
            on_change: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: impl Into<String>) -> Self { self.spec.value = Some(v.into()); self }
    pub fn default_value(mut self, v: impl Into<String>) -> Self { self.spec.default_value = Some(v.into()); self }
    pub fn placeholder(mut self, v: impl Into<String>) -> Self { self.spec.placeholder = Some(v.into()); self }
    pub fn options(mut self, v: Vec<ChoiceOption>) -> Self { self.spec.options = v; self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }
    pub fn description_id(mut self, v: impl Into<String>) -> Self { self.spec.description_id = Some(v.into()); self }
    pub fn open(mut self, v: bool) -> Self { self.spec.open = Some(v); self }
    pub fn default_open(mut self, v: bool) -> Self { self.spec.default_open = v; self }


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

impl IntoElement for Select {
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

        let focus_ring = resolve_color(theme, "semantic.color.accent.focusRing");
        let icon_muted = resolve_color(theme, "semantic.color.icon.muted");

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
            .text_sm()
            .focus(move |s| s.border_color(focus_ring));

        if is_disabled {
            trigger = trigger.opacity(disabled_opacity).cursor(CursorStyle::OperationNotAllowed);
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

        // Contract: indicator uses icon-muted color, not text-secondary with opacity
        trigger = trigger
            .child(
                div().text_color(text_col).flex_1().child(trigger_text.to_string()),
            )
            .child(
                Icon::from_spec(
                    IconSpec::new(if is_open { "chevron-up" } else { "chevron-down" })
                        .with_size(IconSize::Sm),
                    theme,
                )
                .with_color(icon_muted),
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
                    item = item
                        .opacity(disabled_opacity)
                        .cursor(CursorStyle::OperationNotAllowed);
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
