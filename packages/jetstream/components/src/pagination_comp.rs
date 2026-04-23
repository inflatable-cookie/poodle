//! Pagination — Jetstream pagination backed by PaginationSpec.
//!
//! Contract: `docs/contracts/components/pagination.md`
//! Reference: `packages/svelte/components/src/Pagination.svelte`

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{PageItem, PaginationSpec, PaginationVariant};

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius, tint};

pub fn js_pagination(spec: &PaginationSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    let height = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let gap = resolve_px(theme, "space.inline.xs");
    let radius = resolve_radius(theme, spec.radius_token());

    let text_primary = resolve_color(theme, spec.button_text_token());
    let text_secondary = resolve_color(theme, spec.ellipsis_color_token());
    let border_color = resolve_color(theme, spec.button_border_token());
    let surface = resolve_color(theme, spec.button_fill_token());
    let accent = resolve_color(theme, spec.current_fill_token());

    // Current-page button: 18% accent bg, border at 42% accent / 58% default
    let current_bg = tint(accent, 0.18);
    let current_border: Color = {
        let a: Color = accent.into();
        let b: Color = border_color.into();
        a.mix(b, 0.58)
    };

    // Border at 78% opacity (matches Svelte `color-mix … 78%`)
    let button_border = tint(border_color, 0.78);

    let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());

    // ── outer wrapper ──────────────────────────────────────────────────────────
    // flex-col so the info row, controls, and full-variant extras stack naturally.
    let mut root = ui_element::div()
        .flex_col()
        .gap(gap);

    if spec.is_loading {
        root = root.opacity(disabled_opacity);
    }

    // ── info row ───────────────────────────────────────────────────────────────
    if spec.show_info {
        let info_text = build_info_text(spec);
        if let Some(text) = info_text {
            root = root.child(
                ui_element::label(&text)
                    .text_size(font_size)
                    .text_color(text_secondary),
            );
        }
    }

    // ── controls row ──────────────────────────────────────────────────────────
    let mut controls = ui_element::div()
        .flex_row()
        .items_center()
        .gap(gap);

    // Helper: build a page-item button (numbered or arrow)
    let make_button = |label: &str, is_current: bool, is_disabled: bool| {
        let bg: Color = if is_current {
            current_bg.into()
        } else {
            surface.into()
        };
        let bc: Color = if is_current {
            current_border
        } else {
            button_border.into()
        };

        let mut btn = ui_element::button(label)
            .w(height)
            .h(height)
            .rounded(radius)
            .border_1()
            .border_color(bc)
            .bg(bg)
            .text_color(text_primary)
            .text_size(font_size)
            .text_weight(600)
            .px(pad_x)
            .flex_row()
            .items_center()
            .justify_center()
            .focusable()
            .cursor_pointer();

        if is_disabled || spec.is_loading {
            btn = btn.opacity(disabled_opacity).disabled(true);
        }

        btn
    };

    match spec.variant {
        PaginationVariant::Simple => {
            // Simple: just Prev + range summary + Next
            let prev_disabled = spec.is_first_page();
            let next_disabled = spec.is_last_page();

            controls = controls.child(make_button("Prev", false, prev_disabled));

            // Summary: "startItem–endItem of total" or "page X of Y"
            let summary = build_simple_summary(spec);
            controls = controls.child(
                ui_element::label(&summary)
                    .text_size(font_size)
                    .text_color(text_secondary)
                    .px(pad_x),
            );

            controls = controls.child(make_button("Next", false, next_disabled));
        }

        PaginationVariant::Numbered | PaginationVariant::Full => {
            let prev_disabled = spec.is_first_page();
            let next_disabled = spec.is_last_page();

            // Prev arrow
            let prev_btn = {
                let mut btn = ui_element::button("")
                    .w(height)
                    .h(height)
                    .rounded(radius)
                    .border_1()
                    .border_color(button_border)
                    .bg(surface)
                    .flex_row()
                    .items_center()
                    .justify_center()
                    .focusable()
                    .cursor_pointer()
                    .child(
                        ui_element::icon("chevron-left")
                            .w(font_size)
                            .h(font_size)
                            .text_color(text_primary),
                    );
                if prev_disabled || spec.is_loading {
                    btn = btn.opacity(disabled_opacity).disabled(true);
                }
                btn
            };
            controls = controls.child(prev_btn);

            // Numbered page buttons
            let mut pages_row = ui_element::div()
                .flex_row()
                .items_center()
                .gap(gap);

            for item in spec.visible_pages() {
                match item {
                    PageItem::Page(n) => {
                        let is_current = n == spec.current_page;
                        pages_row = pages_row.child(make_button(&n.to_string(), is_current, false));
                    }
                    PageItem::Ellipsis => {
                        // Non-interactive separator, same square size
                        pages_row = pages_row.child(
                            ui_element::div()
                                .w(height)
                                .h(height)
                                .flex_row()
                                .items_center()
                                .justify_center()
                                .child(
                                    ui_element::label("…")
                                        .text_size(font_size)
                                        .text_color(text_secondary),
                                ),
                        );
                    }
                }
            }

            controls = controls.child(pages_row);

            // Next arrow
            let next_btn = {
                let mut btn = ui_element::button("")
                    .w(height)
                    .h(height)
                    .rounded(radius)
                    .border_1()
                    .border_color(button_border)
                    .bg(surface)
                    .flex_row()
                    .items_center()
                    .justify_center()
                    .focusable()
                    .cursor_pointer()
                    .child(
                        ui_element::icon("chevron-right")
                            .w(font_size)
                            .h(font_size)
                            .text_color(text_primary),
                    );
                if next_disabled || spec.is_loading {
                    btn = btn.opacity(disabled_opacity).disabled(true);
                }
                btn
            };
            controls = controls.child(next_btn);

            // Full variant: add "Go to page:" + page-size selector below controls
            if spec.variant == PaginationVariant::Full {
                root = root.child(controls);

                let full_row = build_full_row(spec, theme, height, font_size, pad_x, radius);
                root = root.child(full_row);
                return root;
            }
        }
    }

    root.child(controls)
}

// ── helpers ────────────────────────────────────────────────────────────────────

fn build_info_text(spec: &PaginationSpec) -> Option<String> {
    if let Some(ref text) = spec.info_text {
        return Some(text.clone());
    }
    if let Some(total) = spec.total_items {
        if total == 0 {
            return None;
        }
        let page_size = spec.page_size.unwrap_or(20).max(1);
        let start = ((spec.current_page.saturating_sub(1)) * page_size) + 1;
        let end = (spec.current_page * page_size).min(total);
        return Some(format!("Showing {} to {} of {}", start, end, total));
    }
    None
}

fn build_simple_summary(spec: &PaginationSpec) -> String {
    if let Some(total) = spec.total_items {
        let page_size = spec.page_size.unwrap_or(20).max(1);
        let start = ((spec.current_page.saturating_sub(1)) * page_size) + 1;
        let end = (spec.current_page * page_size).min(total);
        format!("{}–{} of {}", start, end, total)
    } else {
        format!("Page {} of {}", spec.current_page, spec.total_pages)
    }
}

/// Full-variant extra row: "Go to page:" label + bordered box + optional page-size selector.
fn build_full_row(
    spec: &PaginationSpec,
    theme: &JetstreamThemeProvider,
    height: f32,
    font_size: f32,
    pad_x: f32,
    radius: f32,
) -> JsEl {
    let gap = resolve_px(theme, "space.inline.xs");
    let text_secondary = resolve_color(theme, spec.ellipsis_color_token());
    let border_color = tint(resolve_color(theme, spec.button_border_token()), 0.78);
    let surface = resolve_color(theme, spec.button_fill_token());
    let text_primary = resolve_color(theme, spec.button_text_token());

    let mut row = ui_element::div()
        .flex_row()
        .items_center()
        .gap(gap);

    // "Go to page:" label
    row = row.child(
        ui_element::label("Go to page:")
            .text_size(font_size)
            .text_color(text_secondary),
    );

    // Static bordered box representing the page input
    row = row.child(
        ui_element::div()
            .w(height * 2.0)
            .h(height)
            .rounded(radius)
            .border_1()
            .border_color(border_color)
            .bg(surface)
            .flex_row()
            .items_center()
            .justify_center()
            .child(
                ui_element::label(&spec.current_page.to_string())
                    .text_size(font_size)
                    .text_color(text_primary),
            ),
    );

    // Optional page-size selector (static visual)
    if spec.show_limit_selector && !spec.limit_options.is_empty() {
        row = row.child(
            ui_element::label("per page")
                .text_size(font_size)
                .text_color(text_secondary)
                .px(pad_x),
        );

        let page_size_label = spec
            .page_size
            .map(|s| s.to_string())
            .unwrap_or_else(|| spec.limit_options[0].to_string());

        row = row.child(
            ui_element::div()
                .h(height)
                .rounded(radius)
                .border_1()
                .border_color(border_color)
                .bg(surface)
                .flex_row()
                .items_center()
                .px(pad_x)
                .gap(gap)
                .child(
                    ui_element::label(&page_size_label)
                        .text_size(font_size)
                        .text_color(text_primary),
                )
                .child(
                    ui_element::icon("chevron-down")
                        .w(font_size)
                        .h(font_size)
                        .text_color(text_secondary),
                ),
        );
    }

    row
}
