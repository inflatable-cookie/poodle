//! Tabs — real GPUI component backed by TabsSpec.
//!
//! Supports three variants matching the Svelte Tabs component:
//! - Underline (default): bottom border with accent indicator
//! - Card: bordered tabs
//! - Pill: rounded pill container with tinted active state

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{Orientation, TabActivationMode, TabDefinition, TabVariant, TabsSpec};

use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI tabs component backed by `TabsSpec`.
pub struct Tabs {
    spec: TabsSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
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
            id_prefix: "pug-tabs".to_string(),
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
        self.on_change = Some(Box::new(handler));
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
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        let accent = resolve_color(theme, "semantic.color.accent.base");
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let elevated = resolve_color(theme, "semantic.color.background.elevated");

        // Contract: hover = color-mix with elevated
        let hover_bg = color_mix(elevated, elevated, 0.5);

        let current_value = self.spec.current_value().map(|s| s.to_string());

        let mut tab_row = div()
            .flex()
            .border_b_1()
            .border_color(border);

        for tab_def in &self.spec.tabs {
            let is_active = current_value.as_deref() == Some(&tab_def.value);
            let is_disabled = tab_def.is_disabled;
            let tab_id = SharedString::from(format!("{}-{}", self.id_prefix, tab_def.value));

            let mut tab = div()
                .id(tab_id)
                .px(inline_padding)
                .py(control_y)
                .text_sm();

            if is_active {
                tab = tab
                    .text_color(accent)
                    .border_b_2()
                    .border_color(accent);
            } else {
                tab = tab.text_color(text_secondary);
            }

            if is_disabled {
                tab = tab.opacity(disabled_opacity);
            } else {
                tab = tab
                    .cursor_pointer()
                    .hover(move |s| s.bg(hover_bg));
            }

            tab = tab.child(tab_def.label.clone());
            tab_row = tab_row.child(tab);
        }

        tab_row
    }

    fn render_pill(&self) -> Div {
        let theme = &self.theme;
        let list_gap = resolve_px(theme, self.spec.list_gap_token());
        let control_y = resolve_px(theme, "semantic.space.control.y");
        let control_x = resolve_px(theme, "semantic.space.control.x");
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        let accent = resolve_color(theme, self.spec.indicator_token());
        let border_subtle = resolve_color(theme, self.spec.list_border_token());
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let pill_radius = resolve_radius(theme, "semantic.radius.pill");

        let current_value = self.spec.current_value().map(|s| s.to_string());

        // Container border: border-subtle with 68% opacity mix
        let container_border = border_subtle.opacity(border_subtle.a * self.spec.pill_border_opacity());

        // Outer pill container
        let mut tabs = div()
            .flex()
            .items_center()
            .gap(list_gap)
            .rounded(pill_radius)
            .border_2()
            .border_color(container_border)
            .p(list_gap);

        for tab_def in &self.spec.tabs {
            let is_active = current_value.as_deref() == Some(&tab_def.value);
            let is_disabled = tab_def.is_disabled;
            let tab_id = SharedString::from(format!("{}-{}", self.id_prefix, tab_def.value));

            let mut tab = div()
                .id(tab_id)
                .px(control_x)
                .py(control_y)
                .rounded(pill_radius)
                .text_xs()
                .font_weight(FontWeight::SEMIBOLD);

            if is_active {
                let active_bg = accent.opacity(self.spec.pill_active_bg_opacity());
                tab = tab
                    .bg(active_bg)
                    .text_color(text_primary);
            } else {
                tab = tab.text_color(text_secondary);
            }

            if is_disabled {
                tab = tab.opacity(disabled_opacity);
            } else {
                tab = tab.cursor_pointer();
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
