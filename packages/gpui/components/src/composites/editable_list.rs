//! EditableList — real GPUI component backed by EditableListSpec.
//!
//! Renders a list of items with remove buttons, an add row with text input
//! and add button, and an optional counter showing "N/max" or "N items".
//! Supports disabled state, reorderable mode, and size/density responsiveness.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_composites::EditableListSpec;
use poodle_primitives::{ControlDensity, ControlSize, IconSize, IconSpec, SemanticControlSizeRole};

use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem, control_height_rem};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};
use super::super::primitives::Icon;

/// A real GPUI editable list component backed by `EditableListSpec`.
pub struct EditableList {
    spec: EditableListSpec,
    theme: GpuiThemeProvider,
    items: Vec<String>,
    children: Vec<AnyElement>,
}

impl std::ops::Deref for EditableList {
    type Target = EditableListSpec;
    fn deref(&self) -> &EditableListSpec { &self.spec }
}

impl EditableList {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: EditableListSpec::new(),
            theme: theme.clone(),
            items: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn from_spec(spec: EditableListSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            items: Vec::new(),
            children: Vec::new(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn add_label(mut self, v: impl Into<String>) -> Self { self.spec.add_label = v.into(); self }
    pub fn placeholder(mut self, v: impl Into<String>) -> Self { self.spec.placeholder = v.into(); self }
    pub fn max_items(mut self, v: usize) -> Self { self.spec.max_items = Some(v); self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn reorderable(mut self, v: bool) -> Self { self.spec.is_reorderable = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = v.into(); self }
    pub fn with_size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self { self.spec.size_role = v; self }
    pub fn with_density(mut self, v: ControlDensity) -> Self { self.spec.density = v; self }

    // ── Content builders ──────────────────────────────────────
    pub fn items(mut self, items: Vec<String>) -> Self {
        self.spec.item_count = items.len();
        self.items = items;
        self
    }

    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.spec.item_count += 1;
        self.children.push(child.into_any_element());
        self
    }
}

impl IntoElement for EditableList {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        // ── Size-dependent layout (from Svelte CSS) ───────────────
        let (remove_size, item_pad_x, add_pad_x) = match effective_size {
            ControlSize::Xs => (rem_to_px(1.0), rem_to_px(0.5), rem_to_px(0.625)),
            ControlSize::Sm => (rem_to_px(1.125), rem_to_px(0.625), rem_to_px(0.75)),
            ControlSize::Md => (rem_to_px(1.25), rem_to_px(0.625), rem_to_px(0.75)),
            ControlSize::Lg => (rem_to_px(1.375), rem_to_px(0.75), rem_to_px(0.875)),
            ControlSize::Xl => (rem_to_px(1.5), rem_to_px(0.875), rem_to_px(1.0)),
        };

        // ── Density-dependent spacing (from Svelte CSS) ───────────
        let (root_gap, static_gap, item_pad_y, add_gap) = match spec.density {
            ControlDensity::Compact => (
                rem_to_px(0.375),
                rem_to_px(0.0625),
                rem_to_px(0.375),
                rem_to_px(0.25),
            ),
            ControlDensity::Default => (
                rem_to_px(0.5),
                rem_to_px(0.125),
                rem_to_px(0.5),
                rem_to_px(0.375),
            ),
            ControlDensity::Comfortable => (
                rem_to_px(0.625),
                rem_to_px(0.1875),
                rem_to_px(0.625),
                rem_to_px(0.5),
            ),
        };

        // ── Token resolution ──────────────────────────────────────
        let text_color = resolve_color(theme, "color.text.primary");
        let secondary_color = resolve_color(theme, "color.text.secondary");
        let muted_color = resolve_color(theme, "color.text.muted");
        let input_border = resolve_color(theme, spec.input_border_token());
        let input_fill = resolve_color(theme, spec.input_fill_token());
        let _focus_ring = resolve_color(theme, spec.input_focus_ring_token());
        let remove_color = resolve_color(theme, spec.remove_color_token());
        let remove_hover_color = resolve_color(theme, spec.remove_hover_color_token());
        let counter_color = resolve_color(theme, spec.counter_color_token());
        let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");
        let hover_bg = resolve_color(theme, "color.surface.raised");
        let control_height = px(rem_to_px(control_height_rem(effective_size)));
        let control_radius = resolve_radius(theme, "radius.control");
        let gap_sm = resolve_px(theme, "space.inline.sm");
        let gap_md = resolve_px(theme, "space.inline.md");
        let body_size = px(rem_to_px(size_font_rem(effective_size)));
        let label_size = px(rem_to_px(size_font_rem(effective_size) * 0.92));

        let total = if !self.items.is_empty() { self.items.len() } else { self.children.len() };

        // ── Root container ────────────────────────────────────────
        let mut root = div().flex().flex_col().gap(px(root_gap)).w_full();

        // ── Item rows ─────────────────────────────────────────────
        let mut items_container = div().flex().flex_col().gap(px(static_gap));

        let build_item_row = |content: AnyElement, is_reorderable: bool| -> Div {
            let mut row = div()
                .flex()
                .flex_row()
                .items_center()
                .gap(gap_md)
                .px(px(item_pad_x))
                .py(px(item_pad_y))
                .rounded(control_radius)
                .bg(input_fill)
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
            ).with_color(remove_color);

            let rh = remove_hover_color;
            let remove_btn = div()
                .flex()
                .items_center()
                .justify_center()
                .flex_shrink_0()
                .size(px(remove_size))
                .rounded(control_radius)
                .cursor_pointer()
                .text_color(remove_color)
                .hover(move |s| s.text_color(rh).bg(hover_bg))
                .child(remove_icon);

            row = row.child(remove_btn);
            row
        };

        // String items
        for item_text in &self.items {
            let label = div()
                .text_size(body_size)
                .text_color(text_color)
                .overflow_hidden()
                .text_ellipsis()
                .whitespace_nowrap()
                .child(item_text.clone());
            items_container = items_container
                .child(build_item_row(label.into_any_element(), spec.is_reorderable));
        }

        // Custom child elements
        for child in self.children {
            items_container = items_container
                .child(build_item_row(child, spec.is_reorderable));
        }

        if total > 0 {
            root = root.child(items_container);
        }

        // ── Add-item input row ────────────────────────────────────
        let can_add = !spec.is_disabled
            && spec.max_items.map_or(true, |max| total < max);

        if can_add {
            // Text input field
            let input_field = div()
                .flex_grow()
                .min_w(px(0.0))
                .h(control_height)
                .px(px(item_pad_x))
                .flex()
                .items_center()
                .border_1()
                .border_color(input_border)
                .rounded(control_radius)
                .bg(input_fill)
                .child(
                    div()
                        .text_size(body_size)
                        .text_color(secondary_color)
                        .child(spec.placeholder.clone()),
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
                .gap(gap_sm)
                .h(control_height)
                .px(px(add_pad_x))
                .border_1()
                .border_color(input_border)
                .rounded(control_radius)
                .bg(input_fill)
                .cursor_pointer()
                .flex_shrink_0()
                .hover(|s| s.bg(hover_bg))
                .child(plus_icon)
                .child(
                    div()
                        .text_size(label_size)
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(text_color)
                        .child(spec.add_label.clone()),
                );

            let add_row = div()
                .flex()
                .flex_row()
                .gap(px(add_gap))
                .w_full()
                .child(input_field)
                .child(add_btn);

            root = root.child(add_row);
        }

        // ── Counter ───────────────────────────────────────────────
        if spec.shows_counter() {
            let counter_text = if let Some(max) = spec.max_items {
                format!("{}/{}", total, max)
            } else {
                format!("{} item{}", total, if total == 1 { "" } else { "s" })
            };

            root = root.child(
                div()
                    .flex()
                    .justify_end()
                    .child(
                        div()
                            .text_size(label_size)
                            .text_color(counter_color)
                            .child(counter_text),
                    ),
            );
        }

        // ── Disabled state ────────────────────────────────────────
        if spec.is_disabled {
            root = root.opacity(disabled_opacity);
        }

        root.into_any_element()
    }
}
