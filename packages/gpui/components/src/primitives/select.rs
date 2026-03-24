//! Select — real GPUI component backed by SelectSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{ChoiceOption, IconSize, IconSpec, SelectSpec};

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
        let border_default = resolve_color(theme, "semantic.color.border.default");
        let surface_raw = resolve_color(theme, "semantic.color.background.surface");
        let elevated_raw = resolve_color(theme, "semantic.color.background.elevated");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let accent = resolve_color(theme, "semantic.color.accent.base");

        // Svelte treatment-interactive-subtle values for trigger
        let surface_bg = Hsla { a: surface_raw.a * 0.82, ..surface_raw };
        let border = Hsla { a: border_default.a * 0.72, ..border_default };
        let hover_bg = Hsla { a: surface_raw.a * 0.88, ..surface_raw };
        let hover_border = Hsla { a: border_default.a * 0.92, ..border_default };
        // Elevated treatment for dropdown overlay
        let elevated_bg = Hsla { a: elevated_raw.a * 0.94, ..elevated_raw };
        let overlay_border = Hsla { a: border_default.a * 0.22, ..border_default };

        let trigger_text = spec.trigger_text().unwrap_or(
            spec.placeholder.as_deref().unwrap_or("Select..."),
        );
        let is_placeholder = spec.trigger_text().is_none();
        let is_open = spec.current_open();
        let is_disabled = spec.is_disabled;

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("poodle-select-{}", suffix)
        } else {
            "poodle-select".to_string()
        };

        let focus_ring = resolve_color(theme, "semantic.color.accent.focusRing");
        let icon_muted = resolve_color(theme, "semantic.color.icon.muted");

        // Trigger button
        let mut trigger = div()
            .id(SharedString::from(id_str))
            .focusable()
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
            .text_size(px(14.0))
            .focus(move |s| s.border_color(focus_ring));

        if is_disabled {
            trigger = trigger.opacity(disabled_opacity).cursor(CursorStyle::OperationNotAllowed);
        } else {
            trigger = trigger
                .cursor_pointer()
                .hover(move |s| s.bg(hover_bg).border_color(hover_border));
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

        // Wrap callbacks in Rc for sharing across trigger + option closures
        let on_toggle_rc: Option<std::rc::Rc<dyn Fn(&bool, &mut Window, &mut App)>> =
            self.on_toggle.map(|h| std::rc::Rc::from(h));
        let on_change_rc: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App)>> =
            self.on_change.map(|h| std::rc::Rc::from(h));

        // Collect selectable option values for keyboard navigation
        let selectable_values: Vec<String> = spec.options.iter()
            .filter(|o| !o.is_disabled)
            .map(|o| o.value.clone())
            .collect();
        let current_value = spec.current_value().map(|s| s.to_string());

        if !is_disabled {
            let click_toggle = on_toggle_rc.clone();
            let key_toggle = on_toggle_rc.clone();
            let key_change = on_change_rc.clone();
            let key_close = on_toggle_rc.clone();
            let nav_values = selectable_values.clone();
            let current_sel = current_value.clone();
            let next_open = !is_open;

            if let Some(ref handler) = click_toggle {
                let handler = handler.clone();
                trigger = trigger.on_click(move |_event, window, cx| {
                    handler(&next_open, window, cx);
                });
            }

            trigger = trigger.on_key_down(move |event: &KeyDownEvent, window, cx| {
                let key = event.keystroke.key.as_str();
                match key {
                    "space" | "enter" => {
                        if let Some(ref handler) = key_toggle {
                            handler(&next_open, window, cx);
                        }
                    }
                    "escape" => {
                        if is_open {
                            if let Some(ref handler) = key_toggle {
                                handler(&false, window, cx);
                            }
                        }
                    }
                    "down" | "up" => {
                        if !is_open {
                            // Open the dropdown
                            if let Some(ref handler) = key_toggle {
                                handler(&true, window, cx);
                            }
                        } else if !nav_values.is_empty() {
                            // Navigate between options
                            let current_idx = current_sel.as_deref()
                                .and_then(|cv| nav_values.iter().position(|v| v == cv));
                            let next_idx = match key {
                                "down" => match current_idx {
                                    Some(i) => (i + 1) % nav_values.len(),
                                    None => 0,
                                },
                                _ => match current_idx {
                                    Some(0) | None => nav_values.len() - 1,
                                    Some(i) => i - 1,
                                },
                            };
                            if let Some(ref handler) = key_change {
                                handler(&nav_values[next_idx], window, cx);
                            }
                        }
                    }
                    _ => {}
                }
            });
        }

        let mut wrapper = div().flex().flex_col().gap(stack_gap).min_w(px(128.0)).child(trigger);

        // Dropdown list (when open)
        if is_open {
            let option_hover = color_mix(accent, elevated_bg, 0.08);
            let option_selected = color_mix(accent, elevated_bg, 0.10);

            let mut list = div()
                .id("poodle-select-list")
                .rounded(control_radius)
                .bg(elevated_bg)
                .border_1()
                .border_color(overlay_border)
                // Contract: elevation-popover shadow
                .shadow(vec![
                    gpui::BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.10),
                        offset: point(px(0.0), px(4.0)),
                        blur_radius: px(16.0),
                        spread_radius: px(0.0),
                    },
                    gpui::BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.06),
                        offset: point(px(0.0), px(1.0)),
                        blur_radius: px(4.0),
                        spread_radius: px(0.0),
                    },
                ])
                .py(stack_gap)
                .max_h(px(240.0))
                .overflow_y_scroll();

            for option in spec.options.iter() {
                let is_selected =
                    spec.current_value() == Some(option.value.as_str());
                let is_opt_disabled = option.is_disabled;

                let item_id = SharedString::from(format!(
                    "poodle-select-opt-{}",
                    option.value
                ));

                let mut item = div()
                    .id(item_id)
                    .px(inline_padding)
                    .py(stack_gap)
                    .text_size(px(14.0))
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

                    // Click handler: select option and close dropdown
                    if let Some(ref change_handler) = on_change_rc {
                        let change_handler = change_handler.clone();
                        let close_handler = on_toggle_rc.clone();
                        let val = option.value.clone();
                        item = item.on_click(move |_event, window, cx| {
                            change_handler(&val, window, cx);
                            if let Some(ref close) = close_handler {
                                close(&false, window, cx);
                            }
                        });
                    }
                }

                item = item.child(option.label.clone());
                list = list.child(item);
            }

            wrapper = wrapper.child(list);
        }

        wrapper.into_any_element()
    }
}
