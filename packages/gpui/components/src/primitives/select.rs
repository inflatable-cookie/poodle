//! Select — real GPUI component backed by SelectSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ChoiceOption, ControlSize, IconSize, IconSpec, SelectMode, SelectSpec, SelectVariant,
    TextInputSpec,
};

use super::icon::Icon;
use super::text_input::TextInput;
use crate::presentation::{
    rem_to_px, resolve_semantic_size, size_font_rem, size_height_offset_rem,
    size_padding_x_offset_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI select/dropdown component backed by `SelectSpec`.
pub struct Select {
    spec: SelectSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_toggle: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_search_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Select {
    type Target = SelectSpec;
    fn deref(&self) -> &SelectSpec {
        &self.spec
    }
}

impl Select {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: SelectSpec::default(),
            theme: theme.clone(),
            id_suffix: None,
            on_toggle: None,
            on_change: None,
            on_search_change: None,
        }
    }

    pub fn from_spec(spec: SelectSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
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
    pub fn default_value(mut self, v: impl Into<String>) -> Self {
        self.spec.default_value = Some(v.into());
        self
    }
    pub fn placeholder(mut self, v: impl Into<String>) -> Self {
        self.spec.placeholder = Some(v.into());
        self
    }
    pub fn options(mut self, v: Vec<ChoiceOption>) -> Self {
        self.spec.options = v;
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.spec.is_disabled = v;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn description_id(mut self, v: impl Into<String>) -> Self {
        self.spec.description_id = Some(v.into());
        self
    }
    pub fn open(mut self, v: bool) -> Self {
        self.spec.open = Some(v);
        self
    }
    pub fn default_open(mut self, v: bool) -> Self {
        self.spec.default_open = v;
        self
    }
    pub fn size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn size_role(mut self, v: poodle_specs::SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn density(mut self, v: poodle_specs::ControlDensity) -> Self {
        self.spec.density = v;
        self
    }
    pub fn mode(mut self, v: SelectMode) -> Self {
        self.spec.mode = v;
        self
    }
    pub fn searchable(mut self, v: bool) -> Self {
        self.spec.searchable = v;
        self
    }
    pub fn freeform(mut self, v: bool) -> Self {
        self.spec.freeform = v;
        self
    }
    pub fn empty_message(mut self, v: impl Into<String>) -> Self {
        self.spec.empty_message = v.into();
        self
    }

    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
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

    pub fn search_query(mut self, q: impl Into<String>) -> Self {
        self.spec.search_query = Some(q.into());
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

impl IntoElement for Select {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Resolve effective size from size + size_role ────────
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        let base_height = resolve_px(theme, "size.control.height");
        let control_height = base_height + px(rem_to_px(size_height_offset_rem(effective_size)));
        let base_padding = resolve_px(theme, "space.control.x");
        let inline_padding =
            base_padding + px(rem_to_px(size_padding_x_offset_rem(effective_size)));
        let inline_gap = resolve_px(theme, "space.inline.sm");
        let control_radius = resolve_radius(theme, "radius.control");
        let stack_gap = resolve_px(theme, "space.stack.sm");
        let menu_max_h = resolve_px(theme, "size.menu.maxHeight");
        let select_min_w = resolve_px(theme, "size.select.minWidth");

        let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");
        let border_default = resolve_color(theme, "color.border.default");
        let surface_raw = resolve_color(theme, "color.background.surface");
        let elevated_raw = resolve_color(theme, "color.background.elevated");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let accent = resolve_color(theme, "color.accent.base");
        let body_size = px(rem_to_px(size_font_rem(effective_size)));

        // Svelte treatment-interactive-subtle values for trigger
        let surface_bg = Hsla {
            a: surface_raw.a * 0.82,
            ..surface_raw
        };
        let base_border = Hsla {
            a: border_default.a * 0.72,
            ..border_default
        };
        let hover_bg = Hsla {
            a: surface_raw.a * 0.88,
            ..surface_raw
        };

        // Validation-state border colour. Mirrors TextInput's three
        // tones — when set, the trigger advertises the state and
        // the closed-state border switches to the matching status
        // colour.
        use poodle_specs::ValidationState;
        let validation_border = match spec.validation_state {
            ValidationState::Invalid => Some(resolve_color(theme, "color.status.danger")),
            ValidationState::Valid => Some(resolve_color(theme, "color.status.success")),
            ValidationState::Pending => Some(accent),
            ValidationState::None => None,
        };
        let border = validation_border.unwrap_or(base_border);
        let hover_border = validation_border.unwrap_or(Hsla {
            a: border_default.a * 0.92,
            ..border_default
        });
        // Svelte: dropdown listbox bg = plain elevated, border = plain border-default (no alpha reduction)
        let elevated_bg = elevated_raw;
        let overlay_border = border_default;

        let trigger_text = spec.trigger_text().unwrap_or("");
        let is_placeholder = spec.current_value().is_none();
        let is_open = spec.current_open();
        let is_disabled = spec.is_disabled;

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("poodle-select-{}", suffix)
        } else {
            "poodle-select".to_string()
        };

        let focus_ring = resolve_color(theme, "color.accent.focusRing");
        let icon_muted = resolve_color(theme, "color.icon.muted");
        let is_ghost = spec.variant == SelectVariant::Ghost;

        // Trigger button — ghost variant strips all field chrome
        let mut trigger = div().id(SharedString::from(id_str)).focusable();

        if is_ghost {
            // Ghost: no border, background, padding, min-height, or shadow
            trigger = trigger
                .flex()
                .items_center()
                .gap(inline_gap)
                .text_size(body_size)
                .focus(move |s| s.shadow(crate::theme_ext::focus_ring_shadow(focus_ring)));
        } else {
            trigger = trigger
                .h(control_height)
                .px(inline_padding)
                .rounded(control_radius);

            if theme.brand_raised {
                trigger = trigger.bg(crate::theme_ext::brand_raised_subtle_fill(surface_bg));
            } else {
                trigger = trigger.bg(surface_bg);
            }

            trigger = trigger
                .border_1()
                .border_color(if is_open { focus_ring } else { border })
                .flex()
                .items_center()
                .justify_between()
                .gap(inline_gap)
                .text_size(body_size)
                .focus(move |s| {
                    s.border_color(focus_ring)
                        .shadow(crate::theme_ext::focus_ring_shadow(focus_ring))
                });
        }

        if is_disabled {
            trigger = trigger
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        } else if !is_ghost {
            trigger = trigger.cursor_pointer().hover(move |s| {
                s.bg(hover_bg)
                    .border_color(hover_border)
                    .shadow(vec![gpui::BoxShadow {
                        color: hsla(0.0, 0.0, 1.0, 0.10),
                        offset: point(px(0.0), px(1.0)),
                        blur_radius: px(0.0),
                        spread_radius: px(0.0),
                    }])
            });
        } else {
            trigger = trigger.cursor_pointer();
        }

        let text_col = if is_placeholder {
            text_secondary
        } else {
            text_primary
        };

        // Contract: indicator uses icon-muted color, not text-secondary with opacity
        trigger = trigger.child(
            div()
                .text_color(text_col)
                .flex_1()
                .child(trigger_text.to_string()),
        );

        // Ghost variant hides the chevron indicator
        if !is_ghost {
            trigger = trigger.child(
                Icon::from_spec(
                    IconSpec::new(if is_open {
                        "chevron-up"
                    } else {
                        "chevron-down"
                    })
                    .with_size(IconSize::Sm),
                    theme,
                )
                .with_color(icon_muted),
            );
        }

        // Wrap callbacks in Rc for sharing across trigger + option closures
        let on_toggle_rc: Option<std::rc::Rc<dyn Fn(&bool, &mut Window, &mut App)>> =
            self.on_toggle.map(|h| std::rc::Rc::from(h));
        let on_change_rc: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App)>> =
            self.on_change.map(|h| std::rc::Rc::from(h));
        let on_search_rc: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App)>> =
            self.on_search_change.map(|h| std::rc::Rc::from(h));

        // Collect selectable option values for keyboard navigation
        let selectable_values: Vec<String> = spec
            .options
            .iter()
            .filter(|o| !o.is_disabled)
            .map(|o| o.value.clone())
            .collect();
        let current_value = spec.current_value().map(|s| s.to_string());

        if !is_disabled {
            let click_toggle = on_toggle_rc.clone();
            let key_toggle = on_toggle_rc.clone();
            let key_change = on_change_rc.clone();
            let _key_close = on_toggle_rc.clone();
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
                            let current_idx = current_sel
                                .as_deref()
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

        // Root is `relative` so the open listbox can sit in an absolutely positioned layer
        // below the trigger (popover-style) instead of pushing following layout down.
        let mut wrapper = div().relative().min_w(select_min_w).child(trigger);

        // Dropdown list (when open)
        // Note: GPUI always renders a custom dropdown (no native mode).
        if is_open {
            // Svelte: color-mix(accent 14%, transparent) — matches visually on opaque bg
            // Svelte: option hover = color-mix(accent 14%, transparent)
            let option_hover = Hsla { a: accent.a * 0.14, ..accent };
            let empty_message = spec.empty_message.clone();

            let mut list = div().id("poodle-select-list").rounded(control_radius);

            // menu_min_width: parse CSS length string to pixels (supports rem and px)
            if let Some(ref min_width_str) = spec.menu_min_width {
                let min_px = parse_css_length_to_px(min_width_str);
                list = list.min_w(px(min_px));
            }

            // Brand-raised treatment: gradient fill for elevated surface
            if theme.brand_raised {
                list = list.bg(crate::theme_ext::brand_raised_surface_fill(elevated_bg));
            } else {
                list = list.bg(elevated_bg);
            }

            list = list
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
                // Svelte: padding: 0.25rem (4px all sides); items use grid gap: 0.0625rem
                .p(px(rem_to_px(0.25)))
                .gap(px(rem_to_px(0.0625)))
                .max_h(menu_max_h)
                .overflow_y_scroll();

            // Searchable: real TextInput wired to on_search_change callback.
            // The host holds the query in its own state and passes it back
            // through spec.search_query so the options list is filtered here.
            let search_query_lc: Option<String> = spec
                .search_query
                .as_deref()
                .filter(|q| !q.is_empty())
                .map(|q| q.to_lowercase());

            if spec.shows_search_input() {
                let search_rc = on_search_rc.clone();
                let current_query = spec.search_query.clone().unwrap_or_default();
                let mut search_input = TextInput::from_spec(
                    TextInputSpec::new()
                        .with_value(current_query)
                        .with_placeholder("Search..."),
                    theme,
                )
                .with_id("select-search");
                if let Some(handler) = search_rc {
                    search_input = search_input.on_change(move |q, window, cx| {
                        handler(q, window, cx);
                    });
                }
                list = list.child(div().mx(inline_padding).mb(stack_gap).child(search_input));
            }

            // Group-header styling tokens
            let caption_size = resolve_px(theme, "typography.caption.size");
            let separator_color = Hsla {
                a: border_default.a * 0.30,
                ..border_default
            };

            let mut has_visible_options = false;
            // `last_group`: None = no option rendered yet; Some(g) = last visible group.
            let mut last_group: Option<Option<String>> = None;

            for option in spec.options.iter() {
                // Filter by search query when searchable and query is non-empty
                if let Some(ref q) = search_query_lc {
                    if !option.label.to_lowercase().contains(q.as_str()) {
                        continue;
                    }
                }
                has_visible_options = true;

                // ── Group header ──────────────────────────────────────────────
                // Emit a separator + label when the group changes.  Options with
                // group = None are considered their own "unnamed" section.
                let this_group = option.group.clone();
                let group_changed = match &last_group {
                    None => true,
                    Some(prev) => prev != &this_group,
                };
                if group_changed {
                    // Thin separator between successive groups (not before the first)
                    if last_group.is_some() {
                        list = list.child(
                            div()
                                .mx(inline_padding)
                                .my(resolve_px(theme, "space.inline.xs"))
                                .border_b_1()
                                .border_color(separator_color),
                        );
                    }
                    // Named group label
                    // Svelte: padding: 0.375rem 0.5rem 0.1875rem (hardcoded, not size/density)
                    if let Some(ref group_name) = this_group {
                        list = list.child(
                            div()
                                .px(px(rem_to_px(0.5)))
                                .pt(px(rem_to_px(0.375)))
                                .pb(px(rem_to_px(0.1875)))
                                .text_size(caption_size)
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(text_secondary)
                                .child(group_name.clone()),
                        );
                    }
                    last_group = Some(this_group);
                }

                let is_selected = spec.current_value() == Some(option.value.as_str());
                let is_opt_disabled = option.is_disabled;

                let item_id = SharedString::from(format!("poodle-select-opt-{}", option.value));

                // Svelte: option padding: 0.375rem 0.5rem (hardcoded, not size/density responsive)
                let mut item = div()
                    .id(item_id)
                    .px(px(rem_to_px(0.5)))
                    .py(px(rem_to_px(0.375)))
                    .text_size(body_size)
                    .text_color(text_primary);

                // Svelte: selected = font-weight 500 only (no bg, no text-color change)
                if is_selected {
                    item = item.font_weight(FontWeight::MEDIUM);
                }

                // Named-group options get extra left indent (Svelte: padding-left: 1rem)
                let in_named_group = last_group.as_ref().map_or(false, |g| g.is_some());
                if in_named_group {
                    item = item.pl(px(rem_to_px(1.0)));
                }

                if is_opt_disabled {
                    item = item
                        .opacity(disabled_opacity)
                        .cursor(CursorStyle::OperationNotAllowed);
                } else {
                    item = item.cursor_pointer().hover(move |s| s.bg(option_hover));

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

            // Empty state message (shown when no options are visible)
            if !has_visible_options {
                list = list.child(
                    div()
                        .px(px(rem_to_px(0.5)))
                        .py(px(rem_to_px(0.375)))
                        .text_size(body_size)
                        .text_color(text_secondary)
                        .child(empty_message),
                );
            }

            // Position below the trigger; ghost rows are content-sized (no fixed control height).
            let list_top = if is_ghost {
                body_size + inline_gap + stack_gap
            } else {
                control_height + stack_gap
            };
            let list_overlay = div()
                .absolute()
                .left(px(0.0))
                .w_full()
                .top(list_top)
                .child(list);

            wrapper = wrapper.child(list_overlay);
        }

        wrapper.into_any_element()
    }
}

/// Parse a CSS length string (e.g. "12rem", "200px") to pixels.
/// Returns 0.0 on parse failure. Uses 16px as the rem base.
fn parse_css_length_to_px(value: &str) -> f32 {
    let trimmed = value.trim();
    if let Some(num_str) = trimmed.strip_suffix("rem") {
        num_str.trim().parse::<f32>().unwrap_or(0.0) * 16.0
    } else if let Some(num_str) = trimmed.strip_suffix("px") {
        num_str.trim().parse::<f32>().unwrap_or(0.0)
    } else {
        trimmed.parse::<f32>().unwrap_or(0.0)
    }
}
