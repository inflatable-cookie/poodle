//! EditableList — list with add/remove/reorder capabilities.
//! No contract spec exists — implemented from Svelte reference.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{IconSize, IconSpec};
use super::super::primitives::Icon;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub struct EditableList {
    theme: GpuiThemeProvider,
    title: Option<String>,
    add_label: String,
    placeholder: String,
    max_items: Option<usize>,
    is_disabled: bool,
    is_reorderable: bool,
    items: Vec<String>,
    children: Vec<AnyElement>,
}

impl EditableList {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            theme: theme.clone(),
            title: None,
            add_label: "Add item".to_string(),
            placeholder: "New item".to_string(),
            max_items: None,
            is_disabled: false,
            is_reorderable: true,
            items: Vec::new(),
            children: Vec::new(),
        }
    }
    pub fn title(mut self, v: impl Into<String>) -> Self { self.title = Some(v.into()); self }
    pub fn add_label(mut self, v: impl Into<String>) -> Self { self.add_label = v.into(); self }
    pub fn placeholder(mut self, v: impl Into<String>) -> Self { self.placeholder = v.into(); self }
    pub fn max_items(mut self, v: usize) -> Self { self.max_items = Some(v); self }
    pub fn disabled(mut self, v: bool) -> Self { self.is_disabled = v; self }
    pub fn reorderable(mut self, v: bool) -> Self { self.is_reorderable = v; self }
    pub fn items(mut self, items: Vec<String>) -> Self { self.items = items; self }
    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element()); self
    }
}

impl IntoElement for EditableList {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;

        // ── Resolve tokens ────────────────────────────────────────
        let text_color = resolve_color(theme, "semantic.color.text.primary");
        let muted_color = resolve_color(theme, "semantic.color.text.muted");
        let secondary_color = resolve_color(theme, "semantic.color.text.secondary");
        let accent = resolve_color(theme, "semantic.color.accent.base");
        let border_color = resolve_color(theme, "semantic.color.border.default");
        let surface_bg = resolve_color(theme, "semantic.color.surface.default");
        let hover_bg = resolve_color(theme, "semantic.color.surface.raised");
        let danger_color = resolve_color(theme, "semantic.color.status.danger");
        let focus_ring = resolve_color(theme, "semantic.color.accent.focusRing");

        let gap = resolve_px(theme, "semantic.space.stack.sm");
        let item_gap = resolve_px(theme, "semantic.space.stack.xs");
        let control_px = resolve_px(theme, "semantic.space.control.x");
        let control_height = resolve_px(theme, "semantic.size.control.height");
        let radius = resolve_radius(theme, "semantic.radius.control");

        // ── Root container ────────────────────────────────────────
        let mut root = div().flex().flex_col().gap(gap).w_full();

        // ── Header row: title + item counter ──────────────────────
        let total = if !self.items.is_empty() { self.items.len() } else { self.children.len() };
        let has_header = self.title.is_some() || total > 0;
        if has_header {
            let mut header = div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .w_full();

            if let Some(ref title) = self.title {
                header = header.child(
                    div()
                        .text_size(px(14.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(text_color)
                        .child(title.clone()),
                );
            }

            let counter_text = if let Some(max) = self.max_items {
                format!("{}/{}", total, max)
            } else {
                format!("{} item{}", total, if total == 1 { "" } else { "s" })
            };
            let counter = div()
                .text_size(px(11.0))
                .text_color(secondary_color)
                .child(counter_text);

            header = header.child(counter);
            root = root.child(header);
        }

        // ── Helper: build a single item row ───────────────────────
        // Renders: [grip-handle?] [content] [remove-button]
        let build_item_row = |content: AnyElement, is_reorderable: bool| -> Div {
            let mut row = div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.0))
                .px(px(10.0))
                .py(px(8.0))
                .rounded(radius)
                .bg(surface_bg)
                .hover(|s| s.bg(hover_bg));

            // Drag handle (only when reorderable)
            if is_reorderable {
                let grip_icon = Icon::from_spec(
                    IconSpec::new("grip-vertical").with_size(IconSize::Sm),
                    theme,
                ).with_color(muted_color);

                let handle = div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor(CursorStyle::ClosedHand)
                    .flex_shrink_0()
                    .child(grip_icon);
                row = row.child(handle);
            }

            // Content (fills remaining space)
            row = row.child(
                div()
                    .flex_grow()
                    .min_w(px(0.0))
                    .overflow_hidden()
                    .child(content),
            );

            // Remove button
            let remove_icon = Icon::from_spec(
                IconSpec::new("x").with_size(IconSize::Sm),
                theme,
            ).with_color(secondary_color);

            let remove_btn = div()
                .flex()
                .items_center()
                .justify_center()
                .flex_shrink_0()
                .size(px(20.0))
                .rounded(px(4.0))
                .cursor_pointer()
                .hover(|s| s.bg(hover_bg))
                .child(remove_icon);

            row = row.child(remove_btn);
            row
        };

        // ── Render item rows ──────────────────────────────────────
        let mut items_container = div().flex().flex_col().gap(item_gap);

        // String items
        for item_text in &self.items {
            let label = div()
                .text_size(px(14.0))
                .text_color(text_color)
                .overflow_hidden()
                .text_ellipsis()
                .whitespace_nowrap()
                .child(item_text.clone());
            items_container = items_container
                .child(build_item_row(label.into_any_element(), self.is_reorderable));
        }

        // Custom child elements
        for child in self.children {
            items_container = items_container
                .child(build_item_row(child, self.is_reorderable));
        }

        root = root.child(items_container);

        // ── Add-item input row ────────────────────────────────────
        let can_add = !self.is_disabled
            && self.max_items.map_or(true, |max| total < max);

        if can_add {
            // Text input field
            let input_field = div()
                .flex_grow()
                .min_w(px(0.0))
                .h(control_height)
                .px(control_px)
                .flex()
                .items_center()
                .border_1()
                .border_color(border_color)
                .rounded(radius)
                .bg(surface_bg)
                .child(
                    div()
                        .text_size(px(14.0))
                        .text_color(secondary_color)
                        .child(self.placeholder.clone()),
                );

            // Add button
            let plus_icon = Icon::from_spec(
                IconSpec::new("plus").with_size(IconSize::Sm),
                theme,
            ).with_color(text_color);

            let add_btn = div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(6.0))
                .h(control_height)
                .px(px(12.0))
                .border_1()
                .border_color(border_color)
                .rounded(radius)
                .bg(surface_bg)
                .cursor_pointer()
                .flex_shrink_0()
                .hover(|s| s.bg(hover_bg))
                .child(plus_icon)
                .child(
                    div()
                        .text_size(px(13.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(text_color)
                        .child(self.add_label.clone()),
                );

            let add_row = div()
                .flex()
                .flex_row()
                .gap(px(6.0))
                .w_full()
                .child(input_field)
                .child(add_btn);

            root = root.child(add_row);
        }

        // ── Disabled state ────────────────────────────────────────
        if self.is_disabled {
            root = root.opacity(0.5);
        }

        root.into_any_element()
    }
}
