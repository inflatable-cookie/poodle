//! TextInput — Jetstream text input backed by TextInputSpec.
//!
//! Contract: `docs/contracts/components/text-input.md`
//! Reference: `packages/svelte/components/src/TextInput.svelte`
//!
//! Supports: leading/trailing icons, prefix/suffix affixes, validation state
//! indicators, multiline mode, char count display, and search clear button.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{TextInputSpec, ValidationState};

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

pub fn js_text_input(spec: &TextInputSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let height = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let pad_y = resolve_px(theme, spec.vertical_padding_token());
    let icon_sz = rem_to_px(size_font_rem(effective_size));
    let inline_gap = resolve_px(theme, spec.inline_gap_token());

    let fill = resolve_color(theme, spec.fill_token());
    let border_color = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let text_color = resolve_color(theme, spec.text_color_token());
    let placeholder_color = resolve_color(theme, spec.placeholder_color_token());
    let icon_color = resolve_color(theme, spec.icon_color_token());
    let affix_color = resolve_color(theme, spec.affix_color_token());
    let affix_sep_color = resolve_color(theme, spec.affix_separator_color_token());

    // Hover border: contract color-mix(border 78%, text-primary)
    let border_c: Color = border_color.into();
    let text_primary: Color = resolve_color(theme, "color.text.primary").into();
    let hover_border = border_c.mix(text_primary, 0.78);

    let current_value = spec.current_value();
    let is_placeholder = current_value.is_empty() || spec.value.is_none();
    let show_text = if is_placeholder {
        spec.placeholder.as_deref().unwrap_or("")
    } else {
        current_value
    };
    let show_color = if is_placeholder { placeholder_color } else { text_color };

    // ── Input row ──────────────────────────────────────────────────────────
    let mut input_row = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border_color)
        .rounded(radius)
        .pl(pad_x).pr(pad_x)
        .flex_row()
        .gap(inline_gap)
        .focusable()
        .hover(|s| s.border_color(hover_border));

    if spec.is_multiline() {
        // Multiline: min-height based on rows, top-aligned, vertical padding
        input_row = input_row
            .min_h(rem_to_px(control_height_rem(effective_size) * spec.rows as f32))
            .items_start()
            .pt(pad_y).pb(pad_y);
    } else {
        input_row = input_row
            .h(height)
            .items_center();
    }

    // Prefix affix (left edge, with right divider)
    if let Some(prefix) = &spec.prefix {
        let divider = ui_element::div()
            .w(1.0)
            .bg(affix_sep_color);
        input_row = input_row
            .child(
                ui_element::label(prefix.as_str())
                    .text_color(affix_color)
                    .text_size(font_size),
            )
            .child(divider);
    }

    // Leading icon
    if let Some(icon_name) = &spec.leading_icon {
        input_row = input_row.child(
            ui_element::icon(icon_name.as_str())
                .w(icon_sz).h(icon_sz)
                .text_color(icon_color),
        );
    }

    // Text content (grows)
    input_row = input_row.child(
        ui_element::label(show_text)
            .text_color(show_color)
            .text_size(font_size)
            .grow(),
    );

    // Trailing icon or validation icon (trailing_icon takes precedence)
    if let Some(icon_name) = &spec.trailing_icon {
        input_row = input_row.child(
            ui_element::icon(icon_name.as_str())
                .w(icon_sz).h(icon_sz)
                .text_color(icon_color),
        );
    } else {
        // Validation state icon (only when no explicit trailing_icon)
        match spec.validation_state {
            ValidationState::Valid => {
                let color = resolve_color(theme, "color.status.success");
                input_row = input_row.child(
                    ui_element::icon("check-circle")
                        .w(icon_sz).h(icon_sz)
                        .text_color(color),
                );
            }
            ValidationState::Invalid => {
                let color = resolve_color(theme, "color.status.danger");
                input_row = input_row.child(
                    ui_element::icon("alert-circle")
                        .w(icon_sz).h(icon_sz)
                        .text_color(color),
                );
            }
            ValidationState::Pending => {
                let color = resolve_color(theme, "color.text.secondary");
                input_row = input_row.child(
                    ui_element::icon("loader")
                        .w(icon_sz).h(icon_sz)
                        .text_color(color),
                );
            }
            ValidationState::None => {}
        }
    }

    // Clear button (search type, non-empty value)
    if spec.input_type == "search" && spec.show_clear_button && !current_value.is_empty() {
        input_row = input_row.child(
            ui_element::button("")
                .cursor_pointer()
                .child(
                    ui_element::icon("x")
                        .w(icon_sz).h(icon_sz)
                        .text_color(icon_color),
                ),
        );
    }

    // Suffix affix (right edge, with left divider)
    if let Some(suffix) = &spec.suffix {
        let divider = ui_element::div()
            .w(1.0)
            .bg(affix_sep_color);
        input_row = input_row
            .child(divider)
            .child(
                ui_element::label(suffix.as_str())
                    .text_color(affix_color)
                    .text_size(font_size),
            );
    }

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        input_row = input_row.opacity(opacity).disabled(true);
    }

    // ── Char count (optional, wraps input row in a flex_col) ──────────────
    if spec.show_char_count {
        if let Some(max_len) = spec.max_length {
            let current_len = current_value.len();
            let over = current_len > max_len;
            let count_color = if over {
                resolve_color(theme, spec.char_count_over_color_token())
            } else {
                resolve_color(theme, spec.char_count_color_token())
            };
            let count_text = format!("{}/{}", current_len, max_len);

            let char_count_row = ui_element::div()
                .flex_row()
                .justify_end()
                .child(
                    ui_element::label(&count_text)
                        .text_color(count_color)
                        .text_size(font_size),
                );

            return ui_element::div()
                .flex_col()
                .gap(rem_to_px(0.25))
                .child(input_row)
                .child(char_count_row);
        }
    }

    input_row
}
