//! Accordion — real GPUI component backed by AccordionSpec.
//!
//! Visual structure matches the Svelte implementation:
//! - Each item is a bordered card with padding, tinted background, and inset shadow
//! - Items are separated by gap (grid layout)
//! - Trigger shows title + optional description on left, chevron on right
//! - Chevron-down icon rotates when item is open

use std::rc::Rc;
use gpui::*;
use flint_gpui::GpuiThemeProvider;
use flint_primitives::{AccordionItemSpec, AccordionSelectionValue, AccordionSpec, IconSize, IconSpec};

use super::icon::Icon;
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI accordion component backed by `AccordionSpec`.
pub struct Accordion {
    spec: AccordionSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    on_toggle: Option<Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    content: Vec<(String, AnyElement)>,
}

impl std::ops::Deref for Accordion {
    type Target = AccordionSpec;
    fn deref(&self) -> &AccordionSpec { &self.spec }
}

impl Accordion {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: AccordionSpec::default(), theme: theme.clone(), id_prefix: String::new(), on_toggle: None, content: Vec::new() }
    }

    pub fn from_spec(spec: AccordionSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "flint-accordion".to_string(),
            on_toggle: None,
            content: Vec::new(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn items(mut self, v: Vec<AccordionItemSpec>) -> Self { self.spec.items = v; self }
    pub fn value(mut self, v: AccordionSelectionValue) -> Self { self.spec.value = Some(v); self }
    pub fn default_value(mut self, v: AccordionSelectionValue) -> Self { self.spec.default_value = Some(v); self }
    pub fn allow_multiple(mut self, v: bool) -> Self { self.spec.allow_multiple = v; self }
    pub fn collapsible(mut self, v: bool) -> Self { self.spec.is_collapsible = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }

    /// Add content for a specific item value, shown when that item is expanded.
    pub fn with_content(mut self, value: impl Into<String>, content: impl IntoElement) -> Self {
        self.content.push((value.into(), content.into_any_element()));
        self
    }

    pub fn with_id(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    pub fn on_toggle(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Rc::new(handler));
        self
    }
}

impl IntoElement for Accordion {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;

        let disabled_opacity = resolve_opacity(theme, self.spec.disabled_opacity_token());
        let border_subtle = resolve_color(theme, "semantic.color.border.subtle");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let text_inverse = resolve_color(theme, "semantic.color.text.inverse");
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let focus_ring = resolve_color(theme, self.spec.focus_ring_color_token());
        let surface_radius = resolve_radius(theme, "semantic.radius.surface");
        let panel_pad_x = resolve_px(theme, "semantic.space.panel.x");
        let panel_pad_y = resolve_px(theme, "semantic.space.panel.y");
        let stack_md = resolve_px(theme, "semantic.space.stack.md");

        // Item background: color-mix(surface 93%, text-primary)
        let item_bg = color_mix(surface_bg, text_primary, 0.93);
        // Item border: border-subtle at 36% opacity
        let item_border = Hsla { a: border_subtle.a * 0.36, ..border_subtle };
        // Inset shadow highlight: text-inverse at 8% opacity
        let _inset_highlight = Hsla { a: text_inverse.a * 0.08, ..text_inverse };

        let expanded = self.spec.expanded_values();

        // Build content map from the content vec
        let mut content_map: std::collections::HashMap<String, AnyElement> = std::collections::HashMap::new();
        for (key, el) in self.content {
            content_map.insert(key, el);
        }

        // Outer container: grid with gap between items
        let mut col = div().flex().flex_col().gap(stack_md);

        for item in &self.spec.items {
            let is_open = expanded.contains(&item.value.as_str());
            let is_disabled = item.is_disabled;
            let item_id = SharedString::from(format!("{}-{}", self.id_prefix, item.value));

            // ── Trigger button (title + description on left, chevron on right) ──
            let mut trigger = div()
                .id(item_id)
                .focusable()
                .w_full()
                .flex()
                .items_center()
                .justify_between()
                .gap(px(8.0))
                .cursor_pointer();

            trigger = trigger.focus(move |s| s.border_color(focus_ring));

            if is_disabled {
                trigger = trigger
                    .opacity(disabled_opacity)
                    .cursor(CursorStyle::OperationNotAllowed);
            }

            // Left side: title + optional description stacked vertically
            let mut summary = div().flex().flex_col().gap(px(4.0)).min_w(px(0.0)).flex_1();

            // Title: bold, 16px, 1.2 line-height
            summary = summary.child(
                div()
                    .text_size(px(16.0))
                    .font_weight(FontWeight::BOLD)
                    .line_height(relative(1.2))
                    .text_color(text_primary)
                    .child(item.label.clone()),
            );

            // Description shown in trigger (not in expanded panel) per Svelte
            if let Some(ref desc) = item.description {
                summary = summary.child(
                    div()
                        .text_size(px(13.0))
                        .line_height(relative(1.45))
                        .text_color(text_secondary)
                        .child(desc.clone()),
                );
            }

            trigger = trigger.child(summary);

            // Chevron indicator — always chevron-down, visually rotated when open
            // GPUI doesn't support CSS transform rotation on divs easily,
            // so we use chevron-up when open
            let chevron_name = if is_open { "chevron-up" } else { "chevron-down" };
            trigger = trigger.child(
                div()
                    .flex()
                    .items_center()
                    .flex_shrink_0()
                    .child(
                        Icon::from_spec(
                            IconSpec::new(chevron_name).with_size(IconSize::Sm),
                            theme,
                        )
                        .with_color(text_secondary),
                    ),
            );

            // Click + keyboard handler
            if !is_disabled {
                if let Some(ref handler) = self.on_toggle {
                    let click_handler = handler.clone();
                    let key_handler = handler.clone();
                    let value = item.value.clone();
                    let value2 = item.value.clone();
                    trigger = trigger
                        .on_click(move |_event, window, cx| {
                            click_handler(&value, window, cx);
                        })
                        .on_key_down(move |event: &KeyDownEvent, window, cx| {
                            if event.keystroke.key == "space" || event.keystroke.key == "enter" {
                                key_handler(&value2, window, cx);
                            }
                        });
                }
            }

            // ── Item card container (bordered, padded, tinted bg) ──
            let mut item_card = div()
                .flex()
                .flex_col()
                .gap(stack_md)
                .px(panel_pad_x)
                .py(panel_pad_y)
                .border_1()
                .border_color(item_border)
                .rounded(surface_radius)
                .bg(item_bg);

            item_card = item_card.child(trigger);

            // Panel content (when expanded)
            if is_open {
                if let Some(panel_content) = content_map.remove(&item.value) {
                    item_card = item_card.child(
                        div().min_w(px(0.0)).child(panel_content)
                    );
                }
            }

            col = col.child(item_card);
        }

        col.into_any_element()
    }
}
