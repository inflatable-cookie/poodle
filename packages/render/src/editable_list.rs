//! EditableList — rows with remove, an add row, and edit actions.
//!
//! Contract: `docs/contracts/components/editable-list.md`
//! Ported from: `packages/jetstream/components/src/editable_list.rs`.
//!
//! Renders the contract anatomy (§2): an optional workflow header (cancel /
//! submit `button` primitives), optional error / info banners, the item list
//! (each row = drag **handle** when reorderable, **content** label, ghost
//! **remove** `icon_button` when editable/removable), an add row (composed
//! `text_input` + primary `button`), and a counter.
//!
//! `on_remove` carries the row's index (the contract's own `onRemove` is
//! index-based). No `on_add`: the add button renders disabled, because the
//! draft field is typed by the host and this component cannot know it has
//! content. No `on_reorder` / `on_change`: drag payloads and keystrokes are
//! host-owned.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, LayoutDirection, LayoutSizing, MainAxisAlignment, Node,
    NodeRole,
};
use poodle_specs::{
    ButtonSpec, ButtonVariant, EditableListSpec, IconButtonSpec, SemanticControlSizeRole,
    TextInputSpec,
};

use crate::button::button;
use crate::color::{mix_srgb, TRANSPARENT};
use crate::icon_button::icon_button;
use crate::presentation::{
    editable_list_font_rem, editable_list_handle_size_rem, editable_list_item_gap_rem,
    editable_list_item_x_rem, editable_list_item_y_rem, editable_list_list_gap_rem, rem_to_px,
    resolve_semantic_size,
};
use crate::text_input::text_input;

/// Host callbacks. All optional; a missing handler leaves that control inert.
#[derive(Default)]
pub struct EditableListHandlers {
    /// Fires with the removed row's index.
    pub on_remove: Option<Arc<dyn Fn(usize) + Send + Sync>>,
    pub on_submit: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_cancel: Option<Arc<dyn Fn() + Send + Sync>>,
}

pub fn editable_list(
    spec: &EditableListSpec,
    theme: &dyn ThemeProvider,
    handlers: EditableListHandlers,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let density = spec.density;
    let is_unavailable = spec.is_disabled || spec.is_submitting;
    let show_remove = spec.is_editable || spec.is_removable;

    // ── Token-exact size / density geometry (contract §8) ──
    let handle_size = rem_to_px(editable_list_handle_size_rem(effective_size));
    let item_pad_x = rem_to_px(editable_list_item_x_rem(effective_size));
    let item_pad_y = rem_to_px(editable_list_item_y_rem(effective_size));
    let item_font = rem_to_px(editable_list_font_rem(effective_size));
    let list_gap = rem_to_px(editable_list_list_gap_rem(density));
    let item_gap = rem_to_px(editable_list_item_gap_rem(density));

    // ── Container gaps (contract §7/§8 fixed rem) ──
    let root_gap = rem_to_px(0.75);
    let row_gap = rem_to_px(0.5);
    let panel_pad = rem_to_px(0.75);
    let panel_font = rem_to_px(0.875);
    let item_border = rem_to_px(0.0625);

    // ── Token-resolved colors ──
    let text_primary = theme.resolve_color("color.text.primary");
    let handle_color = theme.resolve_color(spec.remove_color_token());
    let counter_color = theme.resolve_color(spec.counter_color_token());
    let label_px = rem_to_px(0.8125); // typography.label.size = 0.8125rem
    let control_radius = theme.resolve_radius("radius.control");
    let surface_radius = theme.resolve_radius("radius.surface");

    // ── Root container (session) ──
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = root_gap;
    }

    // ── Workflow header (cancel / submit button primitives) ──
    //
    // Svelte gates on `onSubmit || onCancel`; here we surface chrome when the
    // list advertises pending work (dirty or submitting).
    if spec.is_dirty || spec.is_submitting {
        let cancel_btn = button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Secondary)
                .with_size(effective_size)
                .with_density(density)
                .with_label(spec.cancel_label.clone())
                .with_disabled(is_unavailable),
            theme,
            handlers.on_cancel.clone(),
        );

        let submit_label = if spec.is_submitting {
            String::from("Saving\u{2026}")
        } else {
            spec.submit_label.clone()
        };
        let submit_btn = button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Primary)
                .with_size(effective_size)
                .with_density(density)
                .with_label(submit_label)
                // Svelte: disabled unless dirty (or while submitting).
                .with_disabled(is_unavailable || !spec.is_dirty),
            theme,
            handlers.on_submit.clone(),
        );

        let mut header = Node::container();
        {
            let s = &mut header.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::End;
            s.descriptor.layout.spacing.gap = row_gap;
        }
        root = root.child(header.child(cancel_btn).child(submit_btn));
    }

    // ── Error / info banners (contract §8) ──
    //
    // Both panels carry a tinted border + tinted background per contract,
    // resolved via color-mix over the danger/accent tokens (no flat fills).
    let surface = theme.resolve_color("color.background.surface");
    let panel_border_w = rem_to_px(0.0625);
    let panel = |border_color: ColorValue, bg: ColorValue, text: &str, text_color: ColorValue| {
        let mut p = Node::container();
        {
            let s = &mut p.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = panel_pad;
            pad.right = panel_pad;
            pad.top = panel_pad;
            pad.bottom = panel_pad;
            s.descriptor.corner_radii.top_left = surface_radius;
            s.descriptor.corner_radii.top_right = surface_radius;
            s.descriptor.corner_radii.bottom_right = surface_radius;
            s.descriptor.corner_radii.bottom_left = surface_radius;
            s.descriptor.border.width = panel_border_w;
            s.descriptor.border.color = border_color;
            s.descriptor.background = Some(bg);
        }
        let mut msg = Node::text(text);
        msg.style.text_size = Some(panel_font);
        msg.style.descriptor.text_color = Some(text_color);
        p.child(msg)
    };
    if let Some(ref error) = spec.error_message {
        let danger = theme.resolve_color(spec.error_color_token());
        // border color-mix(danger 40%, transparent); bg color-mix(danger 8%, surface).
        root = root.child(panel(
            mix_srgb(danger, TRANSPARENT, 0.40),
            mix_srgb(danger, surface, 0.08),
            error,
            danger,
        ));
    } else if let Some(ref info) = spec.info_message {
        let info_color = theme.resolve_color(spec.info_color_token());
        let accent = theme.resolve_color("color.accent.base");
        // border color-mix(accent 22%, transparent); bg color-mix(accent 6%, surface).
        root = root.child(panel(
            mix_srgb(accent, TRANSPARENT, 0.22),
            mix_srgb(accent, surface, 0.06),
            info,
            info_color,
        ));
    }

    // ── Item rows ──
    // Row count comes from the items when the host supplied them; `item_count`
    // remains the fallback for a host that only tells us how many rows to draw.
    let row_count = if spec.items.is_empty() {
        spec.item_count
    } else {
        spec.items.len()
    };

    if row_count > 0 {
        // Contract: the rows are a `listbox` of `option`s, not anonymous
        // structure that happens to be vertical.
        let mut item_list = Node::container();
        item_list.a11y.role = Some(NodeRole::ListBox);
        {
            let s = &mut item_list.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = list_gap;
        }

        for i in 0..row_count {
            let mut row = Node::container();
            // Each row is an `option` of the listbox above it.
            row.a11y.role = Some(NodeRole::ListBoxOption);
            {
                let s = &mut row.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.spacing.gap = item_gap;
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = item_pad_x;
                pad.right = item_pad_x;
                pad.top = item_pad_y;
                pad.bottom = item_pad_y;
                s.descriptor.corner_radii.top_left = control_radius;
                s.descriptor.corner_radii.top_right = control_radius;
                s.descriptor.corner_radii.bottom_right = control_radius;
                s.descriptor.corner_radii.bottom_left = control_radius;
                // contract item: border 0.0625rem solid transparent, transparent bg.
                s.descriptor.border.width = item_border;
                s.descriptor.border.color = TRANSPARENT;
                s.descriptor.background = Some(TRANSPARENT);
            }

            // Drag handle: 6-dot grip (`grip-vertical`), sized to the contract
            // handle-size square. Decorative; shown only when reorderable.
            // `embedded_handle` means the host draws its own grip inside the
            // row, so the component must not add a second one.
            if spec.is_reorderable && !spec.has_embedded_handle {
                let mut handle = Node::container();
                {
                    let s = &mut handle.style;
                    s.descriptor.layout.direction = LayoutDirection::Row;
                    s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                    s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
                    s.flex_shrink_zero = true;
                    s.descriptor.layout.width = LayoutSizing::Fixed(handle_size);
                    s.descriptor.layout.height = LayoutSizing::Fixed(handle_size);
                }
                let mut grip = Node::icon("grip-vertical", handle_size);
                grip.style.descriptor.text_color = Some(handle_color);
                row = row.child(handle.child(grip));
            }

            // Content area: flex-grow, ellipsis overflow.
            let mut content = Node::container();
            {
                let s = &mut content.style;
                // Explicit Row (see switch.rs).
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.width = LayoutSizing::Grow;
                s.min_width = Some(0.0);
            }
            let mut label = Node::text(
                spec.items
                    .get(i)
                    .and_then(|item| item.label.as_deref())
                    .unwrap_or(&format!("Item {}", i + 1)),
            );
            label.style.descriptor.text_color = Some(text_primary);
            label.style.text_size = Some(item_font);
            row = row.child(content.child(label));

            // Remove control: real ghost icon_button (icon `x`, chrome size
            // role). Shown only when editable || removable.
            if show_remove {
                let on_click = handlers.on_remove.as_ref().map(|handler| {
                    let handler = Arc::clone(handler);
                    Arc::new(move || handler(i)) as Arc<dyn Fn() + Send + Sync>
                });
                let remove_btn = icon_button(
                    &IconButtonSpec::new()
                        .with_icon("x")
                        .with_variant(ButtonVariant::Ghost)
                        .with_size(effective_size)
                        .with_size_role(SemanticControlSizeRole::Chrome)
                        .with_density(density)
                        .with_disabled(is_unavailable)
                        .with_aria_label("Remove item"),
                    theme,
                    on_click,
                );
                let mut slot = Node::container();
                {
                    let s = &mut slot.style;
                    // Explicit Row (see switch.rs).
                    s.descriptor.layout.direction = LayoutDirection::Row;
                    s.flex_shrink_zero = true;
                }
                row = row.child(slot.child(remove_btn));
            }

            item_list = item_list.child(row);
        }
        root = root.child(item_list);
    }

    // ── Add row (real text_input + primary button primitives) ──
    // Svelte: canAdd = editable && !disabled && under max.
    let can_add =
        spec.is_editable && !is_unavailable && spec.max_items.map_or(true, |max| row_count < max);

    if can_add {
        let input = text_input(
            &TextInputSpec::new()
                .with_placeholder(spec.placeholder.clone())
                .with_size(effective_size)
                .with_density(density)
                .with_disabled(is_unavailable),
            theme,
            None,
        );

        let add_btn = button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Primary)
                .with_size(effective_size)
                .with_density(density)
                .with_label(spec.add_label.clone())
                // Svelte: add button disabled when input empty/whitespace. No
                // live input value here → render disabled (empty).
                .with_disabled(true),
            theme,
            None,
        );

        let mut input_slot = Node::container();
        {
            let s = &mut input_slot.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.width = LayoutSizing::Grow;
            s.min_width = Some(0.0);
        }
        let mut btn_slot = Node::container();
        {
            let s = &mut btn_slot.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.flex_shrink_zero = true;
        }
        let mut add_row = Node::container();
        {
            let s = &mut add_row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = row_gap;
        }
        root = root.child(
            add_row
                .child(input_slot.child(input))
                .child(btn_slot.child(add_btn)),
        );
    }

    // ── Counter (contract: shown only when maxItems is set) ──
    if spec.shows_counter() {
        let max = spec.max_items.unwrap_or(0);
        let mut counter_row = Node::container();
        {
            let s = &mut counter_row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.main = MainAxisAlignment::End;
        }
        let mut counter = Node::text(&format!("{}/{}", spec.item_count, max));
        counter.style.descriptor.text_color = Some(counter_color);
        counter.style.text_size = Some(label_px);
        root = root.child(counter_row.child(counter));
    }

    // ── Disabled state: list opacity via token (contract §8) ──
    if is_unavailable {
        root.style.descriptor.opacity = theme.resolve_opacity("state.opacity.disabled");
    }

    if !spec.aria_label.is_empty() {
        root.a11y.label = Some(spec.aria_label.clone());
    }
    root
}
