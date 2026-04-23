//! Menu — Jetstream vertical menu backed by MenuSpec.
//!
//! Contract: `docs/contracts/components/menu.md`
//! Reference: `packages/svelte/components/src/Menu.svelte`

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{MenuItemKind, MenuSpec};

use crate::presentation::{
    control_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius, tint};

pub fn js_menu(spec: &MenuSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let meta_font_size = rem_to_px(size_font_rem(effective_size) * 0.85);
    let item_px = rem_to_px(control_space_x_rem(spec.density));
    let item_py = rem_to_px(panel_space_y_rem(spec.density) - 0.375);
    let menu_py = rem_to_px(0.25);
    let item_gap = rem_to_px(0.5);
    let separator_my = rem_to_px(0.25);
    let section_label_font = rem_to_px(size_font_rem(effective_size) * 0.75);

    let fill = resolve_color(theme, spec.surface_fill_token());
    let border = resolve_color(theme, spec.overlay_border_token());
    let radius = resolve_radius(theme, spec.overlay_radius_token());
    let text_color = resolve_color(theme, spec.item_text_token());
    let muted_color = resolve_color(theme, "color.text.secondary");
    let separator_color = resolve_color(theme, spec.separator_color_token());
    let danger_color = resolve_color(theme, "color.status.danger");
    let accent_color = resolve_color(theme, spec.item_highlight_token());
    let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());

    // Subtle hover tint applied as base bg for focused/active items.
    let hover_tint = tint(accent_color, 0.12);

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0)
        .border_color(border)
        .rounded(radius)
        .flex_col()
        .pt(menu_py)
        .pb(menu_py)
        .min_w(rem_to_px(10.0))
        .shadow_md()
        .overlay();

    for entry in &spec.items {
        match entry.kind {
            // ── Separator / section header ────────────────────────────────────
            MenuItemKind::Separator => {
                // 1px divider — always rendered.
                let divider = ui_element::div()
                    .h(1.0)
                    .bg(separator_color)
                    .mt(separator_my)
                    .mb(separator_my);

                if entry.label.trim().is_empty() {
                    // Plain divider
                    el = el.child(divider);
                } else {
                    // Section header: divider + uppercase muted label above items
                    el = el.child(divider);
                    el = el.child(
                        ui_element::label(&entry.label)
                            .text_color(muted_color)
                            .text_size(section_label_font)
                            .text_weight(600)
                            .pl(item_px)
                            .pr(item_px)
                            .pt(separator_my)
                            .pb(separator_my),
                    );
                }
            }

            // ── Checkbox / Radio items ────────────────────────────────────────
            MenuItemKind::Checkbox | MenuItemKind::Radio => {
                let label_color = if entry.is_destructive { danger_color } else { text_color };

                // Leading check indicator (or blank spacer to align labels)
                let check_size = font_size;
                let leading = if entry.is_checked {
                    ui_element::icon("check")
                        .w(check_size)
                        .h(check_size)
                        .text_color(muted_color)
                } else {
                    // Blank spacer so labels stay aligned
                    ui_element::div()
                        .w(check_size)
                        .h(check_size)
                };

                let mut item = ui_element::div()
                    .flex_row()
                    .items_center()
                    .gap(item_gap)
                    .pl(item_px)
                    .pr(item_px)
                    .pt(item_py)
                    .pb(item_py)
                    .focusable()
                    .child(leading)
                    .child(
                        ui_element::label(&entry.label)
                            .text_color(label_color)
                            .text_size(font_size)
                            .grow(),
                    );

                if entry.is_disabled {
                    item = item.opacity(disabled_opacity);
                } else {
                    item = item.cursor_pointer().hover(|s| s.bg(hover_tint));
                }

                el = el.child(item);
            }

            // ── Action items ─────────────────────────────────────────────────
            MenuItemKind::Action => {
                let label_color = if entry.is_destructive { danger_color } else { text_color };

                let mut item = ui_element::div()
                    .flex_row()
                    .items_center()
                    .gap(item_gap)
                    .pl(item_px)
                    .pr(item_px)
                    .pt(item_py)
                    .pb(item_py)
                    .focusable()
                    .child(
                        ui_element::label(&entry.label)
                            .text_color(label_color)
                            .text_size(font_size)
                            .grow(),
                    );

                // Trailing: shortcut label or checkmark
                if entry.is_checked {
                    item = item.child(
                        ui_element::icon("check")
                            .w(font_size)
                            .h(font_size)
                            .text_color(muted_color),
                    );
                } else if let Some(ref shortcut) = entry.shortcut_label {
                    item = item.child(
                        ui_element::label(shortcut)
                            .text_color(muted_color)
                            .text_size(meta_font_size),
                    );
                }

                if entry.is_disabled {
                    item = item.opacity(disabled_opacity);
                } else {
                    item = item.cursor_pointer().hover(|s| s.bg(hover_tint));
                }

                el = el.child(item);
            }
        }
    }

    el
}
