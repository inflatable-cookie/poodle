//! TabStrip — real GPUI component backed by TabStripSpec.
//!
//! Closable file/document tabs distinct from content-switching Tabs.
//! Contract: focus ring, disabled cursor, spec token usage.

use gpui::*;
use flint_adapter::ThemeProvider;
use flint_gpui::GpuiThemeProvider;
use flint_primitives::{IconSize, IconSpec, Orientation, TabStripItem, TabStripSpec};

use super::icon::Icon;

use crate::theme_ext::{resolve_color, resolve_opacity};

/// A real GPUI tab strip component backed by `TabStripSpec`.
pub struct TabStrip {
    spec: TabStripSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    on_change: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_close: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
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
            id_prefix: "flint-tabstrip".to_string(),
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
        self.on_change = Some(std::rc::Rc::new(handler));
        self
    }

    pub fn on_close(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_close = Some(std::rc::Rc::new(handler));
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

        let tab_values: Vec<String> = self.spec.items.iter().map(|i| i.value.clone()).collect();

        for (idx, item) in self.spec.items.iter().enumerate() {
            let is_active = current_value.as_deref() == Some(item.value.as_str());
            let is_disabled = item.is_disabled;
            let item_id = SharedString::from(format!("{}-{}", self.id_prefix, item.value));

            let mut tab = div()
                .id(item_id)
                .focusable()
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

                if let Some(ref handler) = self.on_change {
                    let handler = handler.clone();
                    let val = item.value.clone();
                    tab = tab.on_click(move |_event, window, cx| {
                        handler(&val, window, cx);
                    });

                    // Arrow key navigation
                    let handler = self.on_change.as_ref().unwrap().clone();
                    let tvs = tab_values.clone();
                    let current_idx = idx;
                    tab = tab.on_key_down(move |event: &KeyDownEvent, window, cx| {
                        let next_idx = if event.keystroke.key == "right" || event.keystroke.key == "down" {
                            Some((current_idx + 1) % tvs.len())
                        } else if event.keystroke.key == "left" || event.keystroke.key == "up" {
                            Some(if current_idx == 0 { tvs.len() - 1 } else { current_idx - 1 })
                        } else {
                            None
                        };
                        if let Some(i) = next_idx {
                            handler(&tvs[i], window, cx);
                        }
                    });
                }
            }

            tab = tab.child(item.label.clone());

            // Close button for closable tabs
            if item.is_closable {
                let close_icon = Icon::from_spec(
                    IconSpec::new("x").with_size(IconSize::Sm),
                    theme,
                ).with_color(text_secondary);

                let mut close_btn = div()
                    .id(SharedString::from(format!("{}-close-{}", self.id_prefix, item.value)))
                    .ml(px(4.0))
                    .cursor_pointer()
                    .w(px(20.0)).h(px(20.0))
                    .rounded(px(4.0))
                    .flex().items_center().justify_center()
                    .hover(|s| s.bg(hover_bg))
                    .child(close_icon);

                if let Some(ref handler) = self.on_close {
                    let handler = handler.clone();
                    let val = item.value.clone();
                    close_btn = close_btn.on_click(move |_event, window, cx| {
                        handler(&val, window, cx);
                    });
                }

                tab = tab.child(close_btn);
            }

            strip = strip.child(tab);
        }

        strip.into_any_element()
    }
}
