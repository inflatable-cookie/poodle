//! EditableList — Jetstream editable list backed by EditableListSpec.
//!
//! Contract: `docs/contracts/components/editable-list.md`
//! Reference: `packages/svelte/components/src/EditableList.svelte`
//!
//! Renders the contract anatomy (§2): an optional workflow header (cancel /
//! submit `Button` primitives), optional error / info banners, the item list
//! (each row = drag **handle** when reorderable, **content** label, ghost
//! **remove** `IconButton` when editable/removable), an add row (composed
//! `TextInput` + primary `Button`), and a counter.
//!
//! All geometry resolves from token-exact size/density scales
//! (`crate::presentation::editable_list_*`) and theme tokens — no hardcoded
//! pixel or color literals beyond contract-exact rem fed through `rem_to_px`.
//!
//! ## Accepted limits (probe-verified only)
//! - **No interactivity**: drag-and-drop reorder, keyboard grab/move, add /
//!   remove / submit callbacks, and the dirty/dragging/drop-target/grabbed
//!   visual states are preview-event-loop bound and absent here.
//! - **No ARIA / listbox semantics**: the `JsEl` builder has no accessibility
//!   sink, so `<ul role="listbox">` / `<li role="option">`, the live region,
//!   and `aria-label`s are accepted omissions. Plain `div`s stand in.
//! - **No window-nav / long-list warning**: feature scope, not yet built.

use jetstream_ui::Color;
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    ButtonSpec, ButtonVariant, IconButtonSpec, SemanticControlSizeRole, TextInputSpec,
};
use poodle_specs::EditableListSpec;

use crate::button::js_button;
use crate::icon_button::js_icon_button;
use crate::presentation::{
    editable_list_font_rem, editable_list_handle_size_rem, editable_list_item_gap_rem,
    editable_list_item_x_rem, editable_list_item_y_rem, editable_list_list_gap_rem, rem_to_px,
    resolve_semantic_size,
};
use crate::text_input::js_text_input;
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_radius};

pub fn js_editable_list(spec: &EditableListSpec, theme: &JetstreamThemeProvider) -> JsEl {
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
    let text_primary = resolve_color(theme, "color.text.primary");
    let handle_color = resolve_color(theme, spec.remove_color_token());
    let counter_color = resolve_color(theme, spec.counter_color_token());
    let label_px = rem_to_px(0.8125); // typography.label.size = 0.8125rem
    let control_radius = resolve_radius(theme, "radius.control");
    let surface_radius = resolve_radius(theme, "radius.surface");

    // ── Root container (session) ──
    let mut root = ui_element::div().flex_col().gap(root_gap);

    // ── Workflow header (cancel / submit Button primitives) ──
    //
    // Svelte gates on `onSubmit || onCancel`; no callbacks here, so we surface
    // chrome when the list advertises pending work (dirty or submitting).
    if spec.is_dirty || spec.is_submitting {
        let cancel_btn = js_button(
            &ButtonSpec::new()
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
        let submit_btn = js_button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Primary)
                .with_size(effective_size)
                .with_density(density)
                .with_label(submit_label)
                // Svelte: disabled unless dirty (or while submitting).
                .with_disabled(is_unavailable || !spec.is_dirty),
            theme,
        );

        let header = ui_element::div()
            .flex_row()
            .items_center()
            .justify_end()
            .gap(row_gap)
            .child(cancel_btn)
            .child(submit_btn);
        root = root.child(header);
    }

    // ── Error / info banners (contract §8) ──
    //
    // Both panels carry a tinted border + tinted background per contract,
    // resolved via color-mix over the danger/accent tokens (no flat fills).
    let surface = resolve_color(theme, "color.background.surface");
    let transparent = glam::Vec4::ZERO;
    let panel_border_w = rem_to_px(0.0625);
    if let Some(ref error) = spec.error_message {
        let danger = resolve_color(theme, spec.error_color_token());
        // border color-mix(danger 40%, transparent); bg color-mix(danger 8%, surface).
        let panel_border = color_mix(danger, transparent, 0.40);
        let panel_bg = color_mix(danger, surface, 0.08);
        root = root.child(
            ui_element::div()
                .p(panel_pad)
                .rounded(surface_radius)
                .border(panel_border_w)
                .border_color(panel_border)
                .bg(panel_bg)
                .child(
                    ui_element::label(error)
                        .text_size(panel_font)
                        .text_color(danger),
                ),
        );
    } else if let Some(ref info) = spec.info_message {
        let info_color = resolve_color(theme, spec.info_color_token());
        let accent = resolve_color(theme, "color.accent.base");
        // border color-mix(accent 22%, transparent); bg color-mix(accent 6%, surface).
        let panel_border = color_mix(accent, transparent, 0.22);
        let panel_bg = color_mix(accent, surface, 0.06);
        root = root.child(
            ui_element::div()
                .p(panel_pad)
                .rounded(surface_radius)
                .border(panel_border_w)
                .border_color(panel_border)
                .bg(panel_bg)
                .child(
                    ui_element::label(info)
                        .text_size(panel_font)
                        .text_color(info_color),
                ),
        );
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
        let mut item_list = ui_element::div().flex_col().gap(list_gap);

        for i in 0..row_count {
            let mut row = ui_element::div()
                .flex_row()
                .items_center()
                .gap(item_gap)
                .pl(item_pad_x)
                .pr(item_pad_x)
                .pt(item_pad_y)
                .pb(item_pad_y)
                .rounded(control_radius)
                // contract item: border 0.0625rem solid transparent, transparent bg.
                .border(item_border)
                .border_color(Color::TRANSPARENT)
                .bg(Color::TRANSPARENT);

            // Drag handle: 6-dot grip (`grip-vertical`), sized to the contract
            // handle-size square. Decorative; shown only when reorderable.
            // `embedded_handle` means the host draws its own grip inside the row,
            // so the component must not add a second one.
            if spec.is_reorderable && !spec.has_embedded_handle {
                let handle = ui_element::div()
                    .flex_row()
                    .items_center()
                    .justify_center()
                    .flex_shrink_0()
                    .w(handle_size)
                    .h(handle_size)
                    .child(
                        ui_element::icon("grip-vertical")
                            .w(handle_size)
                            .h(handle_size)
                            .text_color(handle_color),
                    );
                row = row.child(handle);
            }

            // Content area: flex-grow, ellipsis overflow.
            row = row.child(
                ui_element::div().grow().min_w_0().child(
                    // The spec used to carry only a count, so this drew
                    // "Item 1", "Item 2" — placeholder text standing in for
                    // content it had no way to reach.
                    ui_element::label(
                        spec.items
                            .get(i)
                            .and_then(|item| item.label.as_deref())
                            .unwrap_or(&format!("Item {}", i + 1)),
                    )
                        .text_color(text_primary)
                        .text_size(item_font),
                ),
            );

            // Remove control: real ghost IconButton (icon `x`, chrome size role).
            // Shown only when editable || removable.
            if show_remove {
                let remove_btn = js_icon_button(
                    &IconButtonSpec::new()
                        .with_icon("x")
                        .with_variant(ButtonVariant::Ghost)
                        .with_size(effective_size)
                        .with_size_role(SemanticControlSizeRole::Chrome)
                        .with_density(density)
                        .with_disabled(is_unavailable)
                        .with_aria_label("Remove item"),
                    theme,
                );
                row = row.child(ui_element::div().flex_shrink_0().child(remove_btn));
            }

            item_list = item_list.child(row);
        }
        root = root.child(item_list);
    }

    // ── Add row (real TextInput + primary Button primitives) ──
    // Svelte: canAdd = editable && !disabled && under max.
    let can_add =
        spec.is_editable && !is_unavailable && spec.max_items.map_or(true, |max| row_count < max);

    if can_add {
        let input = js_text_input(
            &TextInputSpec::new()
                .with_placeholder(spec.placeholder.clone())
                .with_size(effective_size)
                .with_density(density)
                .with_disabled(is_unavailable),
            theme,
        );

        let add_btn = js_button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Primary)
                .with_size(effective_size)
                .with_density(density)
                .with_label(spec.add_label.clone())
                // Svelte: add button disabled when input empty/whitespace. No
                // live input value here → render disabled (empty).
                .with_disabled(true),
            theme,
        );

        let add_row = ui_element::div()
            .flex_row()
            .items_center()
            .gap(row_gap)
            .child(ui_element::div().grow().min_w_0().child(input))
            .child(ui_element::div().flex_shrink_0().child(add_btn));
        root = root.child(add_row);
    }

    // ── Counter (contract: shown only when maxItems is set) ──
    if spec.shows_counter() {
        let max = spec.max_items.unwrap_or(0);
        root = root.child(
            ui_element::div().flex_row().justify_end().child(
                ui_element::label(&format!("{}/{}", spec.item_count, max))
                    .text_color(counter_color)
                    .text_size(label_px),
            ),
        );
    }

    // ── Disabled state: list opacity via token (contract §8) ──
    if is_unavailable {
        let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");
        root = root.opacity(disabled_opacity);
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::ControlSize;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn renders_item_rows() {
        let th = theme();
        let el = js_editable_list(
            &EditableListSpec::new().with_item_count(3).with_editable(true),
            &th,
        );
        let tree = probe(&el, 400.0, 400.0);
        assert!(tree.has_text("Item 1"), "row 1 renders");
        assert!(tree.has_text("Item 2"), "row 2 renders");
        assert!(tree.has_text("Item 3"), "row 3 renders");
    }

    #[test]
    fn reorderable_renders_drag_handle() {
        let th = theme();
        let with_handle = js_editable_list(
            &EditableListSpec::new().with_item_count(2).with_reorderable(true),
            &th,
        );
        let no_handle = js_editable_list(
            &EditableListSpec::new()
                .with_item_count(2)
                .with_reorderable(false),
            &th,
        );
        let with_tree = probe(&with_handle, 400.0, 400.0);
        let no_tree = probe(&no_handle, 400.0, 400.0);
        assert!(
            with_tree.has_text("grip-vertical"),
            "reorderable list renders the grip handle"
        );
        assert!(
            !no_tree.has_text("grip-vertical"),
            "non-reorderable list omits the handle"
        );
    }

    #[test]
    fn remove_shown_only_when_editable_or_removable() {
        let th = theme();
        // editable → remove present (ghost IconButton renders an "x" glyph).
        let editable = js_editable_list(
            &EditableListSpec::new()
                .with_item_count(1)
                .with_editable(true)
                .with_reorderable(false),
            &th,
        );
        let editable_tree = probe(&editable, 400.0, 400.0);
        assert!(editable_tree.has_text("x"), "editable row has a remove button");

        // removable-only → remove present, no add row.
        let removable = js_editable_list(
            &EditableListSpec::new()
                .with_item_count(1)
                .with_removable(true)
                .with_reorderable(false),
            &th,
        );
        let removable_tree = probe(&removable, 400.0, 400.0);
        assert!(removable_tree.has_text("x"), "removable row has a remove button");

        // neither editable nor removable → no remove button.
        let plain = js_editable_list(
            &EditableListSpec::new()
                .with_item_count(1)
                .with_reorderable(false),
            &th,
        );
        let plain_tree = probe(&plain, 400.0, 400.0);
        assert!(!plain_tree.has_text("x"), "plain row has no remove button");
    }

    #[test]
    fn editable_renders_add_affordance() {
        let th = theme();
        let el = js_editable_list(
            &EditableListSpec::new()
                .with_item_count(1)
                .with_editable(true)
                .with_add_label("Add tag")
                .with_placeholder("New tag")
                .with_reorderable(false),
            &th,
        );
        let tree = probe(&el, 400.0, 400.0);
        assert!(tree.has_text("Add tag"), "add button label renders");
        assert!(tree.has_text("New tag"), "input placeholder renders");
    }

    #[test]
    fn empty_state_shows_only_add_row() {
        let th = theme();
        let el = js_editable_list(
            &EditableListSpec::new()
                .with_item_count(0)
                .with_editable(true)
                .with_add_label("Add item")
                .with_reorderable(false),
            &th,
        );
        let tree = probe(&el, 400.0, 400.0);
        assert!(!tree.has_text("Item 1"), "no item rows when empty");
        assert!(tree.has_text("Add item"), "add affordance still present when empty");
    }

    #[test]
    fn counter_shown_when_max_items_set() {
        let th = theme();
        let el = js_editable_list(
            &EditableListSpec::new()
                .with_item_count(2)
                .with_editable(true)
                .with_max_items(5)
                .with_reorderable(false),
            &th,
        );
        let tree = probe(&el, 400.0, 400.0);
        assert!(tree.has_text("2/5"), "counter shows N/M: {:?}", tree.texts());
    }

    #[test]
    fn at_max_hides_add_row() {
        let th = theme();
        let el = js_editable_list(
            &EditableListSpec::new()
                .with_item_count(5)
                .with_editable(true)
                .with_max_items(5)
                .with_add_label("Add item")
                .with_reorderable(false),
            &th,
        );
        let tree = probe(&el, 400.0, 400.0);
        // At capacity: add button hidden, counter still shows 5/5.
        assert!(!tree.has_text("Add item"), "add row hidden at max");
        assert!(tree.has_text("5/5"), "counter shows full");
    }

    #[test]
    fn workflow_chrome_when_dirty() {
        let th = theme();
        let el = js_editable_list(
            &EditableListSpec::new()
                .with_item_count(3)
                .with_dirty(true)
                .with_submit_label("Save Order")
                .with_cancel_label("Cancel"),
            &th,
        );
        let tree = probe(&el, 400.0, 400.0);
        assert!(tree.has_text("Save Order"), "submit button renders when dirty");
        assert!(tree.has_text("Cancel"), "cancel button renders when dirty");
    }

    #[test]
    fn submitting_shows_saving_label() {
        let th = theme();
        let el = js_editable_list(
            &EditableListSpec::new()
                .with_item_count(3)
                .with_submitting(true),
            &th,
        );
        let tree = probe(&el, 400.0, 400.0);
        assert!(
            tree.texts().iter().any(|t| t.contains("Saving")),
            "submitting shows Saving label: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn error_banner_renders_with_tinted_panel() {
        let th = theme();
        let el = js_editable_list(
            &EditableListSpec::new()
                .with_item_count(1)
                .with_error_message("Something failed")
                .with_reorderable(false),
            &th,
        );
        let tree = probe(&el, 400.0, 400.0);
        assert!(tree.has_text("Something failed"), "error message renders");
        // Contract §8: error panel carries a tinted (color-mix danger 8%/surface)
        // background — not a bare text row. Assert some panel has a visible fill.
        assert!(
            tree.nodes
                .iter()
                .any(|n| n.kind == "Panel" && n.bg.map_or(false, |c| c.a > 0.0)),
            "error panel should have a tinted background fill"
        );
    }

    #[test]
    fn info_banner_renders_with_tinted_panel() {
        let th = theme();
        let el = js_editable_list(
            &EditableListSpec::new()
                .with_item_count(1)
                .with_info_message("Drag to reorder")
                .with_reorderable(false),
            &th,
        );
        let tree = probe(&el, 400.0, 400.0);
        assert!(tree.has_text("Drag to reorder"), "info message renders");
        // Contract §8: info panel carries a tinted (color-mix accent 6%/surface)
        // background fill.
        assert!(
            tree.nodes
                .iter()
                .any(|n| n.kind == "Panel" && n.bg.map_or(false, |c| c.a > 0.0)),
            "info panel should have a tinted background fill"
        );
    }

    #[test]
    fn disabled_reduces_opacity_via_token() {
        let th = theme();
        let el = js_editable_list(
            &EditableListSpec::new()
                .with_item_count(2)
                .with_disabled(true),
            &th,
        );
        let expected = resolve_opacity(&th, "state.opacity.disabled");
        assert!(
            (el.style.opacity - expected).abs() < 0.001,
            "disabled opacity resolves from token, got {}",
            el.style.opacity
        );
    }

    #[test]
    fn size_changes_row_geometry() {
        let th = theme();
        let sm = js_editable_list(
            &EditableListSpec::new()
                .with_item_count(1)
                .with_size(ControlSize::Sm)
                .with_reorderable(false),
            &th,
        );
        let xl = js_editable_list(
            &EditableListSpec::new()
                .with_item_count(1)
                .with_size(ControlSize::Xl)
                .with_reorderable(false),
            &th,
        );
        let sm_tree = probe(&sm, 400.0, 400.0);
        let xl_tree = probe(&xl, 400.0, 400.0);
        // The item row (depth 2: root → list → row) is taller at xl.
        let sm_row_h = sm_tree.nodes.iter().find(|n| n.depth == 2).map(|n| n.h).unwrap_or(0.0);
        let xl_row_h = xl_tree.nodes.iter().find(|n| n.depth == 2).map(|n| n.h).unwrap_or(0.0);
        assert!(xl_row_h > sm_row_h, "xl row taller than sm ({xl_row_h} vs {sm_row_h})");
    }
}
