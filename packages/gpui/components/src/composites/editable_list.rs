//! EditableList — real GPUI composite backed by EditableListSpec.
//!
//! Renders the contract anatomy (§2): an optional workflow header (cancel /
//! submit `Button` primitives), optional error / info banners, the item list,
//! and an add row. Each item row composes the real row primitives —
//! a drag **handle** (`grip-vertical` 6-dot grip Icon), the item **content**
//! (label text), and a ghost **remove** `IconButton` — and the add row composes
//! the real `TextInput` and primary `Button` primitives.
//!
//! All geometry resolves from token-exact size/density scales
//! (`crate::presentation::editable_list_*`) and theme tokens — no hardcoded
//! pixel or color literals.
//!
//! ## Accepted limits (build-verified only)
//! - **No interactivity**: drag-and-drop reorder, keyboard grab/move, add /
//!   remove / submit callbacks, and the dirty/dragging/drop-target/grabbed
//!   visual states are preview-event-loop bound and intentionally absent here.
//!   Controls render at their current static state.
//! - **No ARIA / listbox semantics**: GPUI has no accessibility API, so the
//!   `<ul role="listbox">` / `<li role="option">` structure, the live region,
//!   and `aria-label`s are accepted omissions. Plain `div`s stand in.
//! - **Remove `--danger-on-hover`**: the Svelte wrapper recolors the ghost
//!   IconButton to danger on hover; GPUI's IconButton owns its own hover, so
//!   the wrapper-level override is an accepted limit. The real ghost
//!   IconButton (icon `x`, chrome size role) is composed.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ButtonSpec, ButtonVariant, ControlDensity, ControlSize, IconButtonSpec, IconSize, IconSpec,
    SemanticControlSizeRole, TextInputSpec,
};

use super::super::primitives::{Button, Icon, IconButton, TextInput};
use crate::presentation::{
    editable_list_font_rem, editable_list_handle_size_rem, editable_list_item_gap_rem,
    editable_list_item_x_rem, editable_list_item_y_rem, editable_list_list_gap_rem, rem_to_px,
    resolve_semantic_size,
};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_radius, resolve_px};

/// A real GPUI editable list component backed by `EditableListSpec`.
pub struct EditableList {
    spec: EditableListSpec,
    theme: GpuiThemeProvider,
    items: Vec<String>,
    children: Vec<AnyElement>,
}

use poodle_specs::{EditableListItem, EditableListSpec};

impl std::ops::Deref for EditableList {
    type Target = EditableListSpec;
    fn deref(&self) -> &EditableListSpec {
        &self.spec
    }
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
    pub fn add_label(mut self, v: impl Into<String>) -> Self {
        self.spec.add_label = v.into();
        self
    }
    pub fn placeholder(mut self, v: impl Into<String>) -> Self {
        self.spec.placeholder = v.into();
        self
    }
    pub fn max_items(mut self, v: usize) -> Self {
        self.spec.max_items = Some(v);
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.spec.is_disabled = v;
        self
    }
    pub fn editable(mut self, v: bool) -> Self {
        self.spec.is_editable = v;
        self
    }
    pub fn removable(mut self, v: bool) -> Self {
        self.spec.is_removable = v;
        self
    }
    pub fn reorderable(mut self, v: bool) -> Self {
        self.spec.is_reorderable = v;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = v.into();
        self
    }
    pub fn dirty(mut self, v: bool) -> Self {
        self.spec.is_dirty = v;
        self
    }
    pub fn submitting(mut self, v: bool) -> Self {
        self.spec.is_submitting = v;
        self
    }
    pub fn error_message(mut self, v: impl Into<String>) -> Self {
        self.spec.error_message = Some(v.into());
        self
    }
    pub fn info_message(mut self, v: impl Into<String>) -> Self {
        self.spec.info_message = Some(v.into());
        self
    }
    pub fn submit_label(mut self, v: impl Into<String>) -> Self {
        self.spec.submit_label = v.into();
        self
    }
    pub fn cancel_label(mut self, v: impl Into<String>) -> Self {
        self.spec.cancel_label = v.into();
        self
    }
    pub fn with_size(mut self, v: ControlSize) -> Self {
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

    // ── Content builders ──────────────────────────────────────
    pub fn items(mut self, items: Vec<String>) -> Self {
        // Mirror the labels onto the spec, not just the builder: a caller that
        // round-trips through the spec used to get the count and lose the rows.
        self.spec.items = items
            .iter()
            .enumerate()
            .map(|(i, label)| EditableListItem::new(i.to_string()).with_label(label.clone()))
            .collect();
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
        let density = spec.density;
        let is_unavailable = spec.is_disabled || spec.is_submitting;
        let show_remove = spec.is_editable || spec.is_removable;

        // ── Token-exact size / density geometry (contract §8) ─────
        let handle_size = px(rem_to_px(editable_list_handle_size_rem(effective_size)));
        let item_pad_x = px(rem_to_px(editable_list_item_x_rem(effective_size)));
        let item_pad_y = px(rem_to_px(editable_list_item_y_rem(effective_size)));
        let item_font = px(rem_to_px(editable_list_font_rem(effective_size)));
        let list_gap = px(rem_to_px(editable_list_list_gap_rem(density)));
        let item_gap = px(rem_to_px(editable_list_item_gap_rem(density)));

        // ── Container gaps (contract §7/§8 fixed rem) ─────────────
        // These are fixed in the contract (not density-keyed): session root
        // gap `0.75rem`; header / add-row gap `0.5rem`; error/info padding
        // `0.75rem`. Per CLAUDE.md, rem_to_px of contract-exact rem is the
        // correct form when the value is a literal contract dimension.
        let root_gap = px(rem_to_px(0.75));
        let row_gap = px(rem_to_px(0.5));
        let panel_pad = px(rem_to_px(0.75));

        // ── Token-resolved colors ─────────────────────────────────
        let text_primary = resolve_color(theme, "color.text.primary");
        let handle_color = resolve_color(theme, spec.remove_color_token());
        let counter_color = resolve_color(theme, spec.counter_color_token());
        let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");
        let control_radius = resolve_radius(theme, "radius.control");
        let surface_radius = resolve_radius(theme, "radius.surface");
        let label_size = resolve_px(theme, "typography.label.size");
        let panel_font = px(rem_to_px(0.875)); // contract: error/info font-size 0.875rem

        // The builder's own labels win; the spec's are the fallback for a
        // caller that built the spec directly.
        let spec_labels: Vec<String> = self
            .spec
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                item.label
                    .clone()
                    .unwrap_or_else(|| format!("Item {}", i + 1))
            })
            .collect();
        let item_labels: &Vec<String> = if self.items.is_empty() {
            &spec_labels
        } else {
            &self.items
        };

        let total = if !item_labels.is_empty() {
            item_labels.len()
        } else {
            self.children.len()
        };

        // ── Root container (session) ──────────────────────────────
        let mut root = div().flex().flex_col().gap(root_gap).w_full();

        // ── Workflow header (cancel / submit Button primitives) ───
        //
        // Svelte gates this on `onSubmit || onCancel`; GPUI has no callbacks,
        // so we surface the chrome when the list advertises pending work
        // (dirty or submitting). Buttons are the real primitives, which own
        // their own geometry, typography, fill, and foreground tokens.
        if spec.is_dirty || spec.is_submitting {
            let cancel_btn = Button::from_spec(
                ButtonSpec::new()
                    .with_variant(ButtonVariant::Secondary)
                    .with_size(effective_size)
                    .with_density(density)
                    .with_label(spec.cancel_label.clone())
                    .with_disabled(is_unavailable),
                theme,
            );

            let submit_label = if spec.is_submitting {
                String::from("Saving\u{2026}")
            } else {
                spec.submit_label.clone()
            };
            let submit_btn = Button::from_spec(
                ButtonSpec::new()
                    .with_variant(ButtonVariant::Primary)
                    .with_size(effective_size)
                    .with_density(density)
                    .with_label(submit_label)
                    // Svelte: disabled unless dirty (or while submitting).
                    .with_disabled(is_unavailable || !spec.is_dirty),
                theme,
            );

            // Header bottom border: 0.0625rem solid
            // color-mix(border-default 76%, transparent) (contract §8 Header).
            let border_default = resolve_color(theme, "color.border.default");
            let header_border = color_mix(border_default, gpui::transparent_black(), 0.76);
            let header = div()
                .flex()
                .flex_row()
                .items_center()
                .justify_end()
                .gap(row_gap)
                .w_full()
                .pb(px(rem_to_px(0.5)))
                .border_b(px(rem_to_px(0.0625)))
                .border_color(header_border)
                .child(cancel_btn)
                .child(submit_btn);
            root = root.child(header);
        }

        // ── Error / info banners (contract §8) ────────────────────
        //
        // Both panels carry a tinted border + tinted background per contract,
        // resolved via color-mix over the danger/accent tokens (no flat fills).
        let surface = resolve_color(theme, "color.background.surface");
        let panel_border_w = px(rem_to_px(0.0625));
        if let Some(ref error) = spec.error_message {
            let danger = resolve_color(theme, spec.error_color_token());
            // border color-mix(danger 40%, transparent); bg color-mix(danger 8%, surface).
            let panel_border = color_mix(danger, gpui::transparent_black(), 0.40);
            let panel_bg = color_mix(danger, surface, 0.08);
            root = root.child(
                div()
                    .p(panel_pad)
                    .rounded(surface_radius)
                    .border(panel_border_w)
                    .border_color(panel_border)
                    .bg(panel_bg)
                    .text_size(panel_font)
                    .text_color(danger)
                    .child(error.clone()),
            );
        } else if let Some(ref info) = spec.info_message {
            let info_color = resolve_color(theme, spec.info_color_token());
            let accent = resolve_color(theme, "color.accent.base");
            // border color-mix(accent 22%, transparent); bg color-mix(accent 6%, surface).
            let panel_border = color_mix(accent, gpui::transparent_black(), 0.22);
            let panel_bg = color_mix(accent, surface, 0.06);
            root = root.child(
                div()
                    .p(panel_pad)
                    .rounded(surface_radius)
                    .border(panel_border_w)
                    .border_color(panel_border)
                    .bg(panel_bg)
                    .text_size(panel_font)
                    .text_color(info_color)
                    .child(info.clone()),
            );
        }

        // ── Item rows ─────────────────────────────────────────────
        let mut items_container = div().flex().flex_col().gap(list_gap);

        // Each row: handle (when reorderable) · content · remove (when
        // editable || removable). All from real primitives / token geometry.
        let build_item_row = |content: AnyElement| -> Div {
            let mut row = div()
                .flex()
                .flex_row()
                .items_center()
                .gap(item_gap)
                .px(item_pad_x)
                .py(item_pad_y)
                .rounded(control_radius)
                .bg(gpui::transparent_black())
                // contract item border: 0.0625rem solid transparent.
                .border(px(rem_to_px(0.0625)))
                .border_color(gpui::transparent_black());

            // Drag handle: 6-dot grip (`grip-vertical`), sized to the
            // contract handle-size square. Decorative; cursor: grab.
            if spec.is_reorderable {
                let grip = Icon::from_spec(
                    IconSpec::new("grip-vertical").with_size(IconSize::Sm),
                    theme,
                )
                .with_color(handle_color);

                let handle = div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .flex_shrink_0()
                    .size(handle_size)
                    .cursor(CursorStyle::OpenHand)
                    .child(grip);
                row = row.child(handle);
            }

            // Content area: flex-grow, min-width 0, ellipsis overflow.
            row = row.child(
                div()
                    .flex_grow()
                    .min_w(px(0.0))
                    .overflow_hidden()
                    .child(content),
            );

            // Remove control: real ghost IconButton (icon `x`, chrome size
            // role). Shown only when editable || removable.
            if show_remove {
                let remove_btn = IconButton::from_spec(
                    IconButtonSpec::new()
                        .with_icon("x")
                        .with_variant(ButtonVariant::Ghost)
                        .with_size(effective_size)
                        .with_size_role(SemanticControlSizeRole::Chrome)
                        .with_density(density)
                        .with_disabled(is_unavailable)
                        .with_aria_label("Remove item"),
                    theme,
                );
                row = row.child(div().flex_shrink_0().child(remove_btn));
            }
            row
        };

        for item_text in item_labels {
            let label = div()
                .text_size(item_font)
                .text_color(text_primary)
                .overflow_hidden()
                .text_ellipsis()
                .whitespace_nowrap()
                .child(item_text.clone());
            items_container = items_container.child(build_item_row(label.into_any_element()));
        }
        for child in self.children {
            items_container = items_container.child(build_item_row(child));
        }

        if total > 0 {
            root = root.child(items_container);
        }

        // ── Add row (real TextInput + primary Button primitives) ──
        let can_add =
            spec.is_editable && !is_unavailable && spec.max_items.map_or(true, |max| total < max);

        if can_add {
            let input = TextInput::from_spec(
                TextInputSpec::new()
                    .with_placeholder(spec.placeholder.clone())
                    .with_size(effective_size)
                    .with_density(density)
                    .with_disabled(is_unavailable),
                theme,
            );

            let add_btn = Button::from_spec(
                ButtonSpec::new()
                    .with_variant(ButtonVariant::Primary)
                    .with_size(effective_size)
                    .with_density(density)
                    .with_label(spec.add_label.clone())
                    // Svelte: add button disabled when input empty/whitespace
                    // or !canAdd. No live input here → disabled (empty value).
                    .with_disabled(true),
                theme,
            );

            let add_row = div()
                .flex()
                .flex_row()
                .items_center()
                .gap(row_gap)
                .w_full()
                .child(div().flex_grow().min_w(px(0.0)).child(input))
                .child(div().flex_shrink_0().child(add_btn));
            root = root.child(add_row);
        }

        // ── Counter (contract: shown only when maxItems is set) ───
        if spec.shows_counter() {
            if let Some(max) = spec.max_items {
                let counter_text = format!("{}/{}", total, max);
                root = root.child(
                    div()
                        .flex()
                        .justify_end()
                        .w_full()
                        .child(
                            div()
                                .text_size(label_size)
                                .text_color(counter_color)
                                .child(counter_text),
                        ),
                );
            }
        }

        // ── Disabled state: list opacity via token ────────────────
        if is_unavailable {
            root = root.opacity(disabled_opacity);
        }

        root.into_any_element()
    }
}
