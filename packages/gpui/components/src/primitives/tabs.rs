//! Tabs — real GPUI component backed by TabsSpec.
//!
//! Supports three variants matching the Svelte Tabs component:
//! - Underline (default): bottom border with accent indicator
//! - Card: bordered tabs
//! - Pill: rounded pill container with tinted active state

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{Orientation, TabActivationMode, TabDefinition, TabVariant, TabsSpec};

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI tabs component backed by `TabsSpec`.
pub struct Tabs {
    spec: TabsSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    on_change: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    /// Content elements keyed by tab value.
    content: Vec<(String, AnyElement)>,
}

impl std::ops::Deref for Tabs {
    type Target = TabsSpec;
    fn deref(&self) -> &TabsSpec { &self.spec }
}

impl Tabs {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: TabsSpec::default(), theme: theme.clone(), id_prefix: String::new(), on_change: None, content: Vec::new() }
    }

    pub fn from_spec(spec: TabsSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "poodle-tabs".to_string(),
            on_change: None,
            content: Vec::new(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn tabs(mut self, v: Vec<TabDefinition>) -> Self { self.spec.tabs = v; self }
    pub fn value(mut self, v: impl Into<String>) -> Self { self.spec.value = Some(v.into()); self }
    pub fn default_value(mut self, v: impl Into<String>) -> Self { self.spec.default_value = Some(v.into()); self }
    pub fn variant(mut self, v: TabVariant) -> Self { self.spec.variant = v; self }
    pub fn orientation(mut self, v: Orientation) -> Self { self.spec.orientation = v; self }
    pub fn activation_mode(mut self, v: TabActivationMode) -> Self { self.spec.activation_mode = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }


    pub fn with_id(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    pub fn on_change(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(std::rc::Rc::new(handler));
        self
    }

    /// Add content for a specific tab value.
    pub fn with_content(mut self, value: impl Into<String>, content: impl IntoElement) -> Self {
        self.content
            .push((value.into(), content.into_any_element()));
        self
    }

    fn render_underline(&self) -> Div {
        let theme = &self.theme;
        let inline_padding = resolve_px(theme, "semantic.space.inline.md");
        let control_y = resolve_px(theme, "semantic.space.control.y");
        let disabled_opacity = resolve_opacity(theme, self.spec.disabled_opacity_token());
        let accent = resolve_color(theme, self.spec.indicator_token());
        let border = resolve_color(theme, self.spec.list_border_token());
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let elevated = resolve_color(theme, "semantic.color.background.elevated");
        let focus_ring = resolve_color(theme, self.spec.focus_ring_color_token());

        let hover_bg = elevated;

        let current_value = self.spec.current_value().map(|s| s.to_string());

        let mut tab_row = div()
            .flex()
            .border_b_1()
            .border_color(border);

        let tab_values: Vec<String> = self.spec.tabs.iter().map(|t| t.value.clone()).collect();

        for (idx, tab_def) in self.spec.tabs.iter().enumerate() {
            let is_active = current_value.as_deref() == Some(&tab_def.value);
            let is_disabled = tab_def.is_disabled;
            let tab_id = SharedString::from(format!("{}-{}", self.id_prefix, tab_def.value));

            let mut tab = div()
                .id(tab_id)
                .focusable()
                .px(inline_padding)
                .py(control_y)
                .text_size(px(13.0)).font_weight(FontWeight::SEMIBOLD);

            if is_active {
                tab = tab
                    .text_color(accent)
                    .border_b_2()
                    .border_color(accent);
            } else {
                tab = tab.text_color(text_secondary);
            }

            tab = tab.focus(move |s| s.border_color(focus_ring));

            if is_disabled {
                tab = tab
                    .opacity(disabled_opacity)
                    .cursor(CursorStyle::OperationNotAllowed);
            } else {
                tab = tab
                    .cursor_pointer()
                    .hover(move |s| s.bg(hover_bg));

                if let Some(ref handler) = self.on_change {
                    let handler = handler.clone();
                    let val = tab_def.value.clone();
                    tab = tab.on_click(move |_event, window, cx| {
                        handler(&val, window, cx);
                    });
                }

                // Arrow key navigation
                if let Some(ref handler) = self.on_change {
                    let handler = handler.clone();
                    let nav_values = tab_values.clone();
                    let current_idx = idx;
                    tab = tab.on_key_down(move |event: &KeyDownEvent, window, cx| {
                        let next_idx = if event.keystroke.key == "right" || event.keystroke.key == "down" {
                            Some((current_idx + 1) % nav_values.len())
                        } else if event.keystroke.key == "left" || event.keystroke.key == "up" {
                            Some(if current_idx == 0 { nav_values.len() - 1 } else { current_idx - 1 })
                        } else {
                            None
                        };
                        if let Some(i) = next_idx {
                            handler(&nav_values[i], window, cx);
                        }
                    });
                }
            }

            tab = tab.child(tab_def.label.clone());
            tab_row = tab_row.child(tab);
        }

        tab_row
    }

    fn render_card(&self) -> Div {
        let theme = &self.theme;
        let inline_padding = resolve_px(theme, "semantic.space.inline.md");
        let control_y = resolve_px(theme, "semantic.space.control.y");
        let disabled_opacity = resolve_opacity(theme, self.spec.disabled_opacity_token());
        let accent = resolve_color(theme, self.spec.indicator_token());
        let border = resolve_color(theme, self.spec.list_border_token());
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let elevated = resolve_color(theme, "semantic.color.background.elevated");
        let radius = resolve_radius(theme, "semantic.radius.control");
        let focus_ring = resolve_color(theme, self.spec.focus_ring_color_token());

        let current_value = self.spec.current_value().map(|s| s.to_string());

        let mut tab_row = div()
            .flex()
            .items_end()
            .gap(px(2.0)); // small gap between card tabs

        let tab_values: Vec<String> = self.spec.tabs.iter().map(|t| t.value.clone()).collect();

        for (idx, tab_def) in self.spec.tabs.iter().enumerate() {
            let is_active = current_value.as_deref() == Some(&tab_def.value);
            let is_disabled = tab_def.is_disabled;
            let tab_id = SharedString::from(format!("{}-{}", self.id_prefix, tab_def.value));

            let mut tab = div()
                .id(tab_id)
                .px(inline_padding)
                .py(control_y)
                .text_size(px(13.0)).font_weight(FontWeight::SEMIBOLD)
                .border_1()
                .rounded_t(radius);

            if is_active {
                tab = tab
                    .text_color(accent)
                    .bg(surface_bg)
                    .border_color(border)
                    .border_b_0(); // blend with content below
            } else {
                tab = tab
                    .text_color(text_secondary)
                    .bg(gpui::transparent_black())
                    .border_color(gpui::transparent_black());
            }

            tab = tab.focus(move |s| s.border_color(focus_ring));

            if is_disabled {
                tab = tab
                    .opacity(disabled_opacity)
                    .cursor(CursorStyle::OperationNotAllowed);
            } else if !is_active {
                tab = tab
                    .cursor_pointer()
                    .hover(move |s| s.bg(elevated));
            }

            if !is_disabled {
                if let Some(ref handler) = self.on_change {
                    let handler = handler.clone();
                    let val = tab_def.value.clone();
                    tab = tab.on_click(move |_event, window, cx| {
                        handler(&val, window, cx);
                    });
                }

                // Arrow key navigation
                if let Some(ref handler) = self.on_change {
                    let handler = handler.clone();
                    let nav_values = tab_values.clone();
                    let current_idx = idx;
                    tab = tab.on_key_down(move |event: &KeyDownEvent, window, cx| {
                        let next_idx = if event.keystroke.key == "right" || event.keystroke.key == "down" {
                            Some((current_idx + 1) % nav_values.len())
                        } else if event.keystroke.key == "left" || event.keystroke.key == "up" {
                            Some(if current_idx == 0 { nav_values.len() - 1 } else { current_idx - 1 })
                        } else {
                            None
                        };
                        if let Some(i) = next_idx {
                            handler(&nav_values[i], window, cx);
                        }
                    });
                }
            }

            // Closable tab: label + close button in a flex row
            if tab_def.is_closable {
                let icon_muted = resolve_color(&self.theme, "semantic.color.icon.muted");
                tab = tab
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .child(tab_def.label.clone())
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(icon_muted)
                            .child("×"),
                    );
            } else {
                tab = tab.child(tab_def.label.clone());
            }
            tab_row = tab_row.child(tab);
        }

        tab_row
    }

    fn render_pill(&self) -> Div {
        let theme = &self.theme;
        let control_x = resolve_px(theme, "semantic.space.control.x");
        let control_height = resolve_px(theme, "semantic.size.control.height");
        let disabled_opacity = resolve_opacity(theme, self.spec.disabled_opacity_token());
        let accent = resolve_color(theme, self.spec.indicator_token());
        let border_subtle = resolve_color(theme, self.spec.list_border_token());
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let pill_radius = resolve_radius(theme, "semantic.radius.pill");
        let focus_ring = resolve_color(theme, self.spec.focus_ring_color_token());

        let current_value = self.spec.current_value().map(|s| s.to_string());

        // Container border: border-subtle with 68% opacity
        let container_border = border_subtle.opacity(border_subtle.a * self.spec.pill_border_opacity());

        // Svelte: padding 0.1875rem (3px), gap 0.125rem (2px), border 2px
        let mut tabs = div()
            .flex()
            .items_center()
            .gap(px(2.0))
            .rounded(pill_radius)
            .border_2()
            .border_color(container_border)
            .p(px(3.0));

        let tab_values: Vec<String> = self.spec.tabs.iter().map(|t| t.value.clone()).collect();

        // Svelte: min-height: calc(control-height - 0.5rem)
        let tab_height = control_height - px(8.0);

        for (idx, tab_def) in self.spec.tabs.iter().enumerate() {
            let is_active = current_value.as_deref() == Some(&tab_def.value);
            let is_disabled = tab_def.is_disabled;
            let tab_id = SharedString::from(format!("{}-{}", self.id_prefix, tab_def.value));

            let mut tab = div()
                .id(tab_id)
                .px(control_x)
                .h(tab_height)
                .flex()
                .items_center()
                .rounded(pill_radius)
                .text_size(px(13.0))
                .font_weight(FontWeight::SEMIBOLD);

            if is_active {
                let active_bg = accent.opacity(self.spec.pill_active_bg_opacity());
                tab = tab
                    .bg(active_bg)
                    .text_color(text_primary);
            } else {
                tab = tab.text_color(text_secondary);
            }

            tab = tab.focus(move |s| s.border_color(focus_ring));

            if is_disabled {
                tab = tab
                    .opacity(disabled_opacity)
                    .cursor(CursorStyle::OperationNotAllowed);
            } else {
                tab = tab.cursor_pointer();

                if let Some(ref handler) = self.on_change {
                    let handler = handler.clone();
                    let val = tab_def.value.clone();
                    tab = tab.on_click(move |_event, window, cx| {
                        handler(&val, window, cx);
                    });
                }

                // Arrow key navigation
                if let Some(ref handler) = self.on_change {
                    let handler = handler.clone();
                    let nav_values = tab_values.clone();
                    let current_idx = idx;
                    tab = tab.on_key_down(move |event: &KeyDownEvent, window, cx| {
                        let next_idx = if event.keystroke.key == "right" || event.keystroke.key == "down" {
                            Some((current_idx + 1) % nav_values.len())
                        } else if event.keystroke.key == "left" || event.keystroke.key == "up" {
                            Some(if current_idx == 0 { nav_values.len() - 1 } else { current_idx - 1 })
                        } else {
                            None
                        };
                        if let Some(i) = next_idx {
                            handler(&nav_values[i], window, cx);
                        }
                    });
                }
            }

            tab = tab.child(tab_def.label.clone());
            tabs = tabs.child(tab);
        }

        tabs
    }
}

impl IntoElement for Tabs {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let tab_row = match self.spec.variant {
            TabVariant::Pill => self.render_pill(),
            TabVariant::Card => self.render_card(),
            _ => self.render_underline(),
        };

        let current_value = self.spec.current_value().map(|s| s.to_string());
        let panel_padding = resolve_px(&self.theme, "semantic.space.panel.y");

        // Content pane
        let mut wrapper = div().flex().flex_col().child(tab_row);

        // Show content for active tab
        for (value, content) in self.content {
            if current_value.as_deref() == Some(&value) {
                wrapper = wrapper.child(div().p(panel_padding).child(content));
                break;
            }
        }

        wrapper.into_any_element()
    }
}
