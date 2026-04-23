//! Dialog — Jetstream dialog container backed by DialogSpec.
//!
//! Contract: `docs/contracts/components/dialog.md`
//! Uses overlay() for modal rendering with backdrop.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::DialogSpec;

use crate::presentation::{
    panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_dialog(
    spec: &DialogSpec,
    theme: &JetstreamThemeProvider,
    children: Vec<JsEl>,
    actions: Option<JsEl>,
) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    // Title is 1rem at Md; body follows the size scale.
    let title_font = rem_to_px(1.0_f32.max(size_font_rem(effective_size) + 0.1875));
    let body_font = rem_to_px(size_font_rem(effective_size));
    let space_x = rem_to_px(panel_space_x_rem(spec.density));
    let space_y = rem_to_px(panel_space_y_rem(spec.density));

    let fill = resolve_color(theme, spec.surface_fill_token());
    let backdrop_fill: Color = resolve_color(theme, spec.backdrop_fill_token()).into();
    let border = resolve_color(theme, "color.border.default");
    let border_subtle = resolve_color(theme, "color.border.subtle");
    let radius = resolve_radius(theme, "radius.surface");
    let title_color = resolve_color(theme, "color.text.primary");
    let desc_color = resolve_color(theme, "color.text.secondary");
    let muted_color = resolve_color(theme, "color.text.secondary");

    // ── Width ────────────────────────────────────────────────────────────────
    // `surface_width_rem()` returns INFINITY for Full; all others are finite rem values.
    let width_rem = spec.surface_width_rem();

    // ── Panel ────────────────────────────────────────────────────────────────
    let mut panel = ui_element::div()
        .bg(fill)
        .border(1.0)
        .border_color(border)
        .rounded(radius)
        .flex_col()
        .gap(rem_to_px(1.0))
        .shadow_lg();

    if width_rem.is_finite() {
        panel = panel.w(rem_to_px(width_rem));
    } else {
        // Full — fill the overlay (constrained by the centering flex parent)
        panel = panel.grow();
    }

    // ── Bare mode ────────────────────────────────────────────────────────────
    if spec.bare {
        // No internal padding; close button floats absolutely when needed.
        // Children fill the panel directly.
        for child in children {
            panel = panel.child(child);
        }

        return ui_element::div()
            .bg(backdrop_fill)
            .overlay()
            .items_center()
            .justify_center()
            .child(panel);
    }

    // ── Non-bare: apply padding and build chrome ─────────────────────────────
    panel = panel
        .pl(space_x)
        .pr(space_x)
        .pt(space_y)
        .pb(space_y);

    // Header row: title/description on the left, optional close button on the right.
    let has_header = spec.title.is_some() || spec.description.is_some() || spec.show_close_button;
    if has_header {
        let mut header_col = ui_element::div()
            .flex_col()
            .gap(rem_to_px(0.375))
            .grow();

        if let Some(ref title) = spec.title {
            header_col = header_col.child(
                ui_element::label(title)
                    .text_color(title_color)
                    .text_size(title_font)
                    .text_weight(600),
            );
        }

        if let Some(ref description) = spec.description {
            header_col = header_col.child(
                ui_element::label(description)
                    .text_color(desc_color)
                    .text_size(body_font),
            );
        }

        let mut header_row = ui_element::div()
            .flex_row()
            .items_center()
            .justify_between()
            .gap(rem_to_px(0.75))
            .child(header_col);

        if spec.show_close_button {
            let icon_size = rem_to_px(size_font_rem(effective_size));
            header_row = header_row.child(
                ui_element::button("")
                    .w(icon_size * 1.5)
                    .h(icon_size * 1.5)
                    .flex_row()
                    .items_center()
                    .justify_center()
                    .rounded(rem_to_px(0.25))
                    .cursor_pointer()
                    .focusable()
                    .child(
                        ui_element::icon("x")
                            .w(icon_size)
                            .h(icon_size)
                            .text_color(muted_color),
                    ),
            );
        }

        panel = panel.child(header_row);
    }

    // ── Body children ────────────────────────────────────────────────────────
    for child in children {
        panel = panel.child(child);
    }

    // ── Actions slot ─────────────────────────────────────────────────────────
    if let Some(actions_el) = actions {
        // 1px divider before actions
        panel = panel.child(
            ui_element::div()
                .h(1.0)
                .bg(border_subtle),
        );
        panel = panel.child(actions_el);
    }

    // ── Overlay wrapper ───────────────────────────────────────────────────────
    ui_element::div()
        .bg(backdrop_fill)
        .overlay()
        .items_center()
        .justify_center()
        .child(panel)
}
