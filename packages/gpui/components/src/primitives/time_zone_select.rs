//! TimeZoneSelect — real GPUI component backed by TimeZoneSelectSpec.
//!
//! Thin timezone wrapper over the `Select` overlay surface: the trigger shows
//! the selected zone (or placeholder), and the open dropdown renders a search
//! input plus the filtered, selectable option list sourced from the shared
//! `default_time_zone_options()` (or host-provided options). Mirrors the Svelte
//! wrapper, which delegates to `Select` in always-searchable mode.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ControlDensity, ControlSize, IconSize, IconSpec, SemanticControlSizeRole, TextInputSpec,
    TimeZoneSelectSpec,
};

use super::icon::Icon;
use super::text_input::TextInput;
use crate::presentation::{
    rem_to_px, resolve_semantic_size, size_font_rem, size_height_offset_rem,
    size_padding_x_offset_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI timezone select dropdown component backed by `TimeZoneSelectSpec`.
pub struct TimeZoneSelect {
    spec: TimeZoneSelectSpec,
    theme: GpuiThemeProvider,
    on_toggle: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_search_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for TimeZoneSelect {
    type Target = TimeZoneSelectSpec;
    fn deref(&self) -> &TimeZoneSelectSpec {
        &self.spec
    }
}

impl TimeZoneSelect {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: TimeZoneSelectSpec::new(),
            theme: theme.clone(),
            on_toggle: None,
            on_change: None,
            on_search_change: None,
        }
    }

    pub fn from_spec(spec: TimeZoneSelectSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_toggle: None,
            on_change: None,
            on_search_change: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.spec.value = Some(v.into());
        self
    }
    pub fn placeholder(mut self, v: impl Into<String>) -> Self {
        self.spec.placeholder = Some(v.into());
        self
    }
    pub fn open(mut self, v: bool) -> Self {
        self.spec.is_open = v;
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.spec.is_disabled = v;
        self
    }
    pub fn search_query(mut self, q: impl Into<String>) -> Self {
        self.spec.search_query = Some(q.into());
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

    pub fn on_toggle(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_toggle = Some(Box::new(handler));
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }

    pub fn on_search_change(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_search_change = Some(Box::new(handler));
        self
    }
}

impl IntoElement for TimeZoneSelect {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        let base_height = resolve_px(theme, "size.control.height");
        let control_height = base_height + px(rem_to_px(size_height_offset_rem(effective_size)));
        let base_pad = resolve_px(theme, "space.inline.md");
        let inline_padding = base_pad + px(rem_to_px(size_padding_x_offset_rem(effective_size)));
        let inline_gap = resolve_px(theme, "space.inline.sm");
        let control_radius = resolve_radius(theme, "radius.control");
        let stack_gap = resolve_px(theme, "space.stack.sm");
        let menu_max_h = resolve_px(theme, "size.menu.maxHeight");
        let select_min_w = resolve_px(theme, "size.select.minWidth");
        let row_pad_x = resolve_px(theme, "space.inline.sm");
        let row_pad_y = resolve_px(theme, "space.control.y");

        let border = resolve_color(theme, spec.border_token());
        let surface_bg = resolve_color(theme, "color.background.surface");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let elevated_bg = resolve_color(theme, spec.overlay_fill_token());
        let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");
        let icon_muted = resolve_color(theme, "color.icon.muted");
        let accent = resolve_color(theme, "color.accent.base");
        let body_size = px(rem_to_px(size_font_rem(effective_size)));

        // Trigger label: selected zone or placeholder (Svelte default
        // "Search time zones...", never an ad-hoc string).
        let trigger_text = spec
            .trigger_text()
            .unwrap_or_else(|| spec.effective_placeholder().to_string());
        let is_placeholder = spec.current_value().is_none();
        let text_col = if is_placeholder {
            text_secondary
        } else {
            text_primary
        };

        let focus_ring = resolve_color(theme, "color.accent.focusRing");

        let mut trigger = div()
            .id(SharedString::from("poodle-tz-select"))
            .focusable()
            .h(control_height)
            .px(inline_padding)
            .rounded(control_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(if spec.is_open { focus_ring } else { border })
            .flex()
            .items_center()
            .justify_between()
            .gap(inline_gap)
            .text_size(body_size)
            .child(
                div()
                    .text_color(text_col)
                    .flex_1()
                    .child(trigger_text),
            )
            .child(
                Icon::from_spec(
                    IconSpec::new(if spec.is_open {
                        "chevron-up"
                    } else {
                        "chevron-down"
                    })
                    .with_size(IconSize::Sm),
                    theme,
                )
                .with_color(icon_muted),
            );

        trigger = trigger.focus(move |s| {
            s.border_color(focus_ring)
                .shadow(crate::theme_ext::focus_ring_shadow(focus_ring))
        });

        if spec.is_disabled {
            trigger = trigger
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        } else {
            trigger = trigger.cursor_pointer();
        }

        let is_open = spec.is_open;
        let is_disabled = spec.is_disabled;

        // Share callbacks across trigger + option closures.
        let on_toggle_rc: Option<std::rc::Rc<dyn Fn(&bool, &mut Window, &mut App)>> =
            self.on_toggle.map(std::rc::Rc::from);
        let on_change_rc: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App)>> =
            self.on_change.map(std::rc::Rc::from);
        let on_search_rc: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App)>> =
            self.on_search_change.map(std::rc::Rc::from);

        if !is_disabled {
            let next_open = !is_open;
            let click_toggle = on_toggle_rc.clone();
            let key_toggle = on_toggle_rc.clone();
            if let Some(handler) = click_toggle {
                trigger = trigger.on_click(move |_event, window, cx| {
                    handler(&next_open, window, cx);
                });
            }
            trigger = trigger.on_key_down(move |event: &KeyDownEvent, window, cx| {
                let key = event.keystroke.key.as_str();
                if let Some(ref handler) = key_toggle {
                    match key {
                        "space" | "enter" | "down" | "up" => {
                            if !is_open {
                                handler(&true, window, cx);
                            }
                        }
                        "escape" => {
                            if is_open {
                                handler(&false, window, cx);
                            }
                        }
                        _ => {}
                    }
                }
            });
        }

        // Root is `relative` so the open listbox sits in an absolutely
        // positioned layer below the trigger (popover-style).
        let mut wrapper = div().relative().min_w(select_min_w).child(trigger);

        if spec.is_open {
            // Resolve the effective option set and filter by the live query.
            let options = spec.select_options();
            let query_lc: Option<String> = spec
                .search_query
                .as_deref()
                .filter(|q| !q.is_empty())
                .map(|q| q.to_lowercase());
            let option_hover = Hsla {
                a: accent.a * 0.14,
                ..accent
            };
            let current_value = spec.current_value().map(|v| v.to_string());

            let mut dropdown = div()
                .id("poodle-tz-select-list")
                .rounded(control_radius)
                .bg(elevated_bg)
                .border_1()
                .border_color(border)
                .shadow(crate::theme_ext::elevation_overlay_shadow())
                .p(px(rem_to_px(0.25)))
                .gap(px(rem_to_px(0.0625)))
                .max_h(menu_max_h)
                .overflow_y_scroll()
                .text_size(body_size)
                .text_color(text_primary);

            // Search input row (searchable is always on for TimeZoneSelect).
            {
                let current_query = spec.search_query.clone().unwrap_or_default();
                let mut search_input = TextInput::from_spec(
                    TextInputSpec::new()
                        .with_value(current_query)
                        .with_placeholder(spec.effective_placeholder()),
                    theme,
                )
                .with_id("tz-select-search");
                if let Some(ref handler) = on_search_rc {
                    let handler = handler.clone();
                    search_input = search_input.on_change(move |q, window, cx| {
                        handler(q, window, cx);
                    });
                }
                dropdown =
                    dropdown.child(div().mx(row_pad_x).mb(stack_gap).child(search_input));
            }

            let mut has_visible = false;
            for option in options.iter() {
                if let Some(ref q) = query_lc {
                    if !option.label.to_lowercase().contains(q.as_str()) {
                        continue;
                    }
                }
                has_visible = true;
                let is_selected = current_value.as_deref() == Some(option.value.as_str());

                let mut row = div()
                    .id(SharedString::from(format!(
                        "poodle-tz-opt-{}",
                        option.value
                    )))
                    .px(row_pad_x)
                    .py(row_pad_y)
                    .flex()
                    .items_center()
                    .justify_between()
                    .cursor_pointer()
                    .hover(move |s| s.bg(option_hover));

                if is_selected {
                    row = row.font_weight(FontWeight::MEDIUM);
                }

                row = row.child(div().flex_1().child(option.label.clone()));

                // Selected indicator: check icon.
                if is_selected {
                    row = row.child(
                        Icon::from_spec(
                            IconSpec::new("check").with_size(IconSize::Sm),
                            theme,
                        )
                        .with_color(icon_muted),
                    );
                }

                // Click selects the option and closes the dropdown.
                if let Some(ref change_handler) = on_change_rc {
                    let change_handler = change_handler.clone();
                    let close_handler = on_toggle_rc.clone();
                    let val = option.value.clone();
                    row = row.on_click(move |_event, window, cx| {
                        change_handler(&val, window, cx);
                        if let Some(ref close) = close_handler {
                            close(&false, window, cx);
                        }
                    });
                }

                dropdown = dropdown.child(row);
            }

            // Empty state (Svelte: "No matching time zones").
            if !has_visible {
                dropdown = dropdown.child(
                    div()
                        .px(row_pad_x)
                        .py(row_pad_y)
                        .text_color(text_secondary)
                        .child(poodle_specs::TIME_ZONE_EMPTY_MESSAGE),
                );
            }

            let list_overlay = div()
                .absolute()
                .left(px(0.0))
                .w_full()
                .top(control_height + stack_gap)
                .child(dropdown);

            wrapper = wrapper.child(list_overlay);
        }

        wrapper.into_any_element()
    }
}
