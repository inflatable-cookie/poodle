//! TabStrip — real GPUI component backed by TabStripSpec.
//!
//! Closable file/document tabs distinct from content-switching Tabs.
//! Contract: focus ring, disabled cursor, size/density tokens, keyboard nav.
//!
//! # Known GPUI deltas
//! - `role="tablist"`, `role="tab"`, `aria-selected`, `aria-disabled`: GPUI native
//!   rendering does not carry HTML ARIA attributes. These are documented-only deltas.

use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ControlDensity, ControlSize, IconSize, IconSpec, Orientation, TabStripItem, TabStripSpec};

use super::icon::Icon;

use crate::presentation::{
    control_space_x_rem, rem_to_px, resolve_semantic_size,
    size_font_rem, size_padding_x_offset_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px};

/// A real GPUI tab strip component backed by `TabStripSpec`.
pub struct TabStrip {
    spec: TabStripSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    on_change: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_close: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    /// Host-owned reorder commit. Args: `(value, direction)` where `direction`
    /// is `-1` (toward start) or `+1` (toward end). Fired on `Alt+Arrow` when
    /// `is_reorderable`. Drag reordering is host/preview-loop owned.
    on_reorder: Option<std::rc::Rc<dyn Fn(&str, i32, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for TabStrip {
    type Target = TabStripSpec;
    fn deref(&self) -> &TabStripSpec {
        &self.spec
    }
}

impl TabStrip {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: TabStripSpec::default(),
            theme: theme.clone(),
            id_prefix: String::new(),
            on_change: None,
            on_close: None,
            on_reorder: None,
        }
    }

    pub fn from_spec(spec: TabStripSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "poodle-tabstrip".to_string(),
            on_change: None,
            on_close: None,
            on_reorder: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn items(mut self, v: Vec<TabStripItem>) -> Self {
        self.spec.items = v;
        self
    }
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.spec.value = Some(v.into());
        self
    }
    pub fn default_value(mut self, v: impl Into<String>) -> Self {
        self.spec.default_value = Some(v.into());
        self
    }
    pub fn orientation(mut self, v: Orientation) -> Self {
        self.spec.orientation = v;
        self
    }
    pub fn reorderable(mut self, v: bool) -> Self {
        self.spec.is_reorderable = v;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }

    pub fn with_id(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(std::rc::Rc::new(handler));
        self
    }

    pub fn on_close(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(std::rc::Rc::new(handler));
        self
    }

    pub fn on_reorder(
        mut self,
        handler: impl Fn(&str, i32, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_reorder = Some(std::rc::Rc::new(handler));
        self
    }
}

impl IntoElement for TabStrip {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;

        // ── Size/density token resolution ─────────────────────────────────────
        // TabStrip uses Control size role (same as other interactive controls).
        use poodle_specs::SemanticControlSizeRole;
        let effective_size = resolve_semantic_size(self.spec.size, SemanticControlSizeRole::Control);

        let label_size = px(rem_to_px(size_font_rem(effective_size)));
        let inline_padding = px(rem_to_px(
            control_space_x_rem(self.spec.density)
                + size_padding_x_offset_rem(effective_size),
        ));
        let control_y = resolve_px(theme, "space.control.y");
        // Icon-to-label gap within each tab; matches Svelte Tabs space.inline.sm
        let inline_gap = resolve_px(theme, "space.inline.sm");

        let accent = resolve_color(theme, "color.accent.base");
        let border = resolve_color(theme, "color.border.default");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let focus_ring = resolve_color(theme, self.spec.focus_ring_color_token());
        let disabled_opacity = resolve_opacity(theme, self.spec.disabled_opacity_token());
        let hover_bg = resolve_color(theme, "color.background.elevated");
        let strip_gap = theme.resolve_space(self.spec.item_gap_token());

        // Close-button geometry, resolved from spec (contract §2: 1.25rem square,
        // radius = radius-control − 0.125rem). No bare float literals.
        let close_btn_size = px(rem_to_px(self.spec.close_button_size_rem()));
        let close_btn_radius = px((f32::from(resolve_px(
            theme,
            self.spec.close_button_radius_token(),
        ))
            - rem_to_px(self.spec.close_button_radius_inset_rem()))
        .max(0.0));
        let close_btn_gap = resolve_px(theme, self.spec.close_button_gap_token());
        // Named multiplier for the vertical active-tab fill tint (was magic 0.08).
        let vertical_active_opacity = self.spec.vertical_active_fill_opacity();

        let current_value = self.spec.current_value().map(|s| s.to_string());
        let is_vertical = self.spec.orientation == Orientation::Vertical;
        let is_reorderable = self.spec.is_reorderable;

        // ARIA delta: role="tablist" not expressible on GPUI native elements.
        let mut strip = div().gap(px(strip_gap));

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
        let tab_disabled: Vec<bool> = self.spec.items.iter().map(|i| i.is_disabled).collect();

        for (idx, item) in self.spec.items.iter().enumerate() {
            let is_active = current_value.as_deref() == Some(item.value.as_str());
            let is_disabled = item.is_disabled;
            let item_id = SharedString::from(format!("{}-{}", self.id_prefix, item.value));

            // ARIA delta: role="tab", aria-selected, aria-disabled not expressible.
            let mut tab = div()
                .id(item_id)
                .focusable()
                .flex()
                .items_center()
                .gap(inline_gap)
                .px(inline_padding)
                .py(control_y)
                .text_size(label_size)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(text_primary)
                .focus(move |s| {
                    s.border_color(focus_ring)
                        .shadow(crate::theme_ext::focus_ring_shadow(focus_ring))
                });

            if is_active {
                if is_vertical {
                    tab = tab.text_color(accent).bg(accent.opacity(vertical_active_opacity));
                } else {
                    tab = tab.text_color(accent).border_b_1().border_color(accent);
                }
            } else {
                tab = tab.text_color(text_secondary);
            }

            if is_disabled {
                tab = tab
                    .opacity(disabled_opacity)
                    .cursor(CursorStyle::OperationNotAllowed);
            } else {
                tab = tab.cursor_pointer().hover(|s| s.bg(hover_bg));

                if let Some(ref handler) = self.on_change {
                    let handler = handler.clone();
                    let val = item.value.clone();
                    tab = tab.on_click(move |_event, window, cx| {
                        handler(&val, window, cx);
                    });

                    // ── Keyboard navigation ───────────────────────────────────
                    // Arrow keys navigate between tabs, skipping disabled ones.
                    // Home/End jump to first/last enabled tab.
                    // Delete closes a closable tab (fires on_close).
                    let nav_handler = self.on_change.as_ref().unwrap().clone();
                    let close_handler = self.on_close.clone();
                    let reorder_handler = self.on_reorder.clone();
                    let tvs = tab_values.clone();
                    let td = tab_disabled.clone();
                    let current_idx = idx;
                    let item_closable = item.is_closable;
                    let item_val = item.value.clone();
                    let item_reorderable = is_reorderable;

                    tab = tab.on_key_down(move |event: &KeyDownEvent, window, cx| {
                        let n = tvs.len();
                        // Alt+Arrow reorders (host-owned commit) when reorderable —
                        // checked before plain arrow navigation so the modifier wins.
                        if item_reorderable
                            && event.keystroke.modifiers.alt
                            && matches!(
                                event.keystroke.key.as_str(),
                                "left" | "right" | "up" | "down"
                            )
                        {
                            if let Some(ref handler) = reorder_handler {
                                let dir = match event.keystroke.key.as_str() {
                                    "left" | "up" => -1,
                                    _ => 1,
                                };
                                handler(&item_val, dir, window, cx);
                            }
                            return;
                        }
                        match event.keystroke.key.as_str() {
                            "enter" | "space" => {
                                // Enter/Space confirms/activates the focused tab.
                                nav_handler(&item_val, window, cx);
                            }
                            "right" | "down" => {
                                // Advance to next enabled tab, wrapping
                                let mut i = (current_idx + 1) % n;
                                for _ in 0..n {
                                    if !td[i] {
                                        nav_handler(&tvs[i], window, cx);
                                        return;
                                    }
                                    i = (i + 1) % n;
                                }
                            }
                            "left" | "up" => {
                                // Move to previous enabled tab, wrapping
                                let mut i = if current_idx == 0 { n - 1 } else { current_idx - 1 };
                                for _ in 0..n {
                                    if !td[i] {
                                        nav_handler(&tvs[i], window, cx);
                                        return;
                                    }
                                    i = if i == 0 { n - 1 } else { i - 1 };
                                }
                            }
                            "home" => {
                                // First enabled tab
                                for (i, disabled) in td.iter().enumerate() {
                                    if !disabled {
                                        nav_handler(&tvs[i], window, cx);
                                        return;
                                    }
                                }
                            }
                            "end" => {
                                // Last enabled tab
                                for (i, disabled) in td.iter().enumerate().rev() {
                                    if !disabled {
                                        nav_handler(&tvs[i], window, cx);
                                        return;
                                    }
                                }
                            }
                            "delete" | "backspace" => {
                                if item_closable {
                                    if let Some(ref handler) = close_handler {
                                        handler(&item_val, window, cx);
                                    }
                                }
                            }
                            _ => {}
                        }
                    });
                }
            }

            tab = tab.child(item.label.clone());

            // Close button for closable tabs
            if item.is_closable {
                let close_icon = Icon::from_spec(IconSpec::new("x").with_size(IconSize::Sm), theme)
                    .with_color(text_secondary);

                let mut close_btn = div()
                    .id(SharedString::from(format!(
                        "{}-close-{}",
                        self.id_prefix, item.value
                    )))
                    .ml(close_btn_gap)
                    .cursor_pointer()
                    .w(close_btn_size)
                    .h(close_btn_size)
                    .rounded(close_btn_radius)
                    .flex()
                    .items_center()
                    .justify_center()
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
