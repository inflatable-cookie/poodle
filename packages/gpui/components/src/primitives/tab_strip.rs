//! TabStrip — real GPUI component backed by TabStripSpec.
//!
//! Closable file/document tabs distinct from content-switching Tabs.
//! Contract: focus ring, disabled cursor, spec token usage.

use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{Orientation, TabStripItem, TabStripSpec};

use crate::theme_ext::{resolve_color, resolve_opacity};

/// A real GPUI tab strip component backed by `TabStripSpec`.
pub struct TabStrip {
    spec: TabStripSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_close: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for TabStrip {
    type Target = TabStripSpec;
    fn deref(&self) -> &TabStripSpec { &self.spec }
}

impl TabStrip {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: TabStripSpec::default(), theme: theme.clone(), id_prefix: String::new(), on_change: None, on_close: None }
    }

    pub fn from_spec(spec: TabStripSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "pug-tabstrip".to_string(),
            on_change: None,
            on_close: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn items(mut self, v: Vec<TabStripItem>) -> Self { self.spec.items = v; self }
    pub fn value(mut self, v: impl Into<String>) -> Self { self.spec.value = Some(v.into()); self }
    pub fn default_value(mut self, v: impl Into<String>) -> Self { self.spec.default_value = Some(v.into()); self }
    pub fn orientation(mut self, v: Orientation) -> Self { self.spec.orientation = v; self }
    pub fn reorderable(mut self, v: bool) -> Self { self.spec.is_reorderable = v; self }
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

    pub fn on_close(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_close = Some(Box::new(handler));
        self
    }
}

impl IntoElement for TabStrip {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;

        let accent = resolve_color(theme, "semantic.color.accent.base");
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let focus_ring = resolve_color(theme, self.spec.focus_ring_color_token());
        let disabled_opacity = resolve_opacity(theme, self.spec.disabled_opacity_token());
        let hover_bg = resolve_color(theme, "semantic.color.background.elevated");
        let gap = theme.resolve_space(self.spec.item_gap_token());

        let current_value = self.spec.current_value().map(|s| s.to_string());
        let is_vertical = self.spec.orientation == Orientation::Vertical;

        let mut strip = div().gap(px(gap));

        if is_vertical {
            strip = strip.flex().flex_col();
        } else {
            strip = strip
                .flex()
                .items_center()
                .border_b_1()
                .border_color(border);
        }

        for item in &self.spec.items {
            let is_active = current_value.as_deref() == Some(item.value.as_str());
            let is_disabled = item.is_disabled;
            let item_id = SharedString::from(format!("{}-{}", self.id_prefix, item.value));

            let mut tab = div()
                .id(item_id)
                .flex()
                .items_center()
                .gap(px(6.0))
                .px(px(10.0))
                .py(px(6.0))
                // Contract: label font
                .text_size(px(12.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(text_primary)
                // Focus ring
                .focus(move |s| s.border_color(focus_ring));

            if is_active {
                if is_vertical {
                    tab = tab
                        .text_color(accent)
                        .bg(accent.opacity(0.08));
                } else {
                    tab = tab
                        .text_color(accent)
                        .border_b_1()
                        .border_color(accent);
                }
            } else {
                tab = tab.text_color(text_secondary);
            }

            if is_disabled {
                tab = tab
                    .opacity(disabled_opacity)
                    .cursor(CursorStyle::OperationNotAllowed);
            } else {
                tab = tab
                    .cursor_pointer()
                    .hover(|s| s.bg(hover_bg));
            }

            tab = tab.child(item.label.clone());

            // Close button for closable tabs
            if item.is_closable {
                tab = tab.child(
                    div()
                        .text_xs()
                        .text_color(text_secondary)
                        .ml(px(4.0))
                        .cursor_pointer()
                        .child("×"),
                );
            }

            strip = strip.child(tab);
        }

        strip.into_any_element()
    }
}
