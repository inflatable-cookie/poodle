//! EditableLabel — Jetstream click-to-edit label backed by EditableLabelSpec.
//!
//! Contract: `docs/contracts/components/editable-label.md`
//! Reference: `packages/svelte/components/src/EditableLabel.svelte`
//!
//! A read-mostly label that transitions into a single-line input on activation.
//! This component RENDERS the correct mode for the current spec state:
//!   - display mode (`is_editing == false`): the label text (or empty-text /
//!     placeholder), an optional leading-pencil edit affordance, hover hint
//!     border + background, disabled opacity. Flush variant strips the
//!     padding / border / radius and inlines the text.
//!   - editing mode (`is_editing == true`): a composed text-input element
//!     seeded with the current value + placeholder, accent-focusRing border,
//!     surface background (flush → bottom border only). `max_length` is carried
//!     on the spec for the host editor.
//!
//! The activation gesture (click / double-click per `activation_mode`), commit
//! (Enter / blur), cancel (Escape), `select_on_focus`, and the
//! commit/cancel/edit-start callbacks are PREVIEW-EVENT-LOOP bound: the host
//! drives `is_editing` / `value` and re-renders. This component does NOT
//! synthesize key-by-key editing or fake interactivity — it renders the mode
//! the spec asks for.
//!
//! ARIA: N/A for Jetstream (no accessibility API).

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ControlDensity, ControlSize, EditableLabelSpec, EditableLabelVariant};

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// Per-size font-size in rem. Contract §8: xs 0.75, sm/md base label-size
/// (0.8125), lg 0.9375, xl 1.0. Mirrors the GPUI `size_font_override_rem`.
fn font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm | ControlSize::Md => 0.8125,
        ControlSize::Lg => 0.9375,
        ControlSize::Xl => 1.0,
    }
}

/// Per-size padding-y offset in rem, relative to `space.control.y`.
/// Contract §8 size table (y component of the `calc(...)` declarations).
fn pad_y_offset_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => -0.125,
        ControlSize::Sm => -0.0625,
        ControlSize::Md => 0.0,
        ControlSize::Lg => 0.0625,
        ControlSize::Xl => 0.125,
    }
}

/// Per-size padding-x offset in rem, relative to `space.control.x`.
/// Contract §8 size table: xs/sm -0.125/-0.0625, md 0, lg +0.125, xl +0.1875.
fn pad_x_offset_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => -0.125,
        ControlSize::Sm => -0.0625,
        ControlSize::Md => 0.0,
        ControlSize::Lg => 0.125,
        ControlSize::Xl => 0.1875,
    }
}

/// Density adjusts padding-inline only (Svelte: compact -0.125rem,
/// comfortable +0.125rem). Must NOT change padding-block / height.
fn density_pad_x_offset_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => -0.125,
        ControlDensity::Default => 0.0,
        ControlDensity::Comfortable => 0.125,
    }
}

pub fn js_editable_label(spec: &EditableLabelSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // ── Token-resolved geometry ──────────────────────────────────────────
    let text_color = resolve_color(theme, spec.text_color_token());
    let placeholder_color = resolve_color(theme, spec.placeholder_color_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let font_size = rem_to_px(font_rem(effective_size));

    let base_pad_y = resolve_px(theme, "space.control.y");
    let base_pad_x = resolve_px(theme, "space.control.x");
    let pad_y = base_pad_y + rem_to_px(pad_y_offset_rem(effective_size));
    let pad_x = base_pad_x
        + rem_to_px(pad_x_offset_rem(effective_size))
        + rem_to_px(density_pad_x_offset_rem(spec.density));

    // Focus / editing border width = `border.width.focus` token.
    let focus_width = resolve_px(theme, "border.width.focus");
    let display_gap = resolve_px(theme, "space.inline.sm");

    let is_flush = spec.variant == EditableLabelVariant::Flush;
    let is_empty = spec.value.is_empty();

    let mut el = if spec.is_editing {
        // ── Editing mode: composed text-input ───────────────────────────
        // Seeded with the current value + placeholder. The host editor owns
        // keystrokes and max_length enforcement; we render the seeded input.
        let edit_border = resolve_color(theme, spec.edit_border_token());
        let surface_bg = resolve_color(theme, spec.fill_token());
        let value = spec.value.clone();
        let placeholder = spec.placeholder.clone().unwrap_or_default();

        let mut input = ui_element::text_input(value, placeholder)
            .w_full()
            .text_color(text_color)
            .text_size(font_size);

        if is_flush {
            // Flush editing: bottom accent border only, transparent bg, no padding.
            input = input
                .border_b_1()
                .border_color(edit_border);
        } else {
            // Default editing: accent border + surface bg + padding + radius.
            input = input
                .px(pad_x)
                .py(pad_y)
                .rounded(radius)
                .border(focus_width)
                .border_color(edit_border)
                .bg(surface_bg);
        }
        input.focusable()
    } else {
        // ── Display mode: label text + optional edit affordance ─────────
        // empty_text takes precedence over placeholder in display mode.
        let display_text = if is_empty {
            spec.empty_text
                .clone()
                .or_else(|| spec.placeholder.clone())
                .unwrap_or_default()
        } else {
            spec.value.clone()
        };
        let text_col = if is_empty {
            placeholder_color
        } else {
            text_color
        };

        // Text span + optional pencil icon (contract: 0.75rem icon, color
        // text-secondary, shown on hover/focus — here always present when set).
        let mut row = ui_element::div()
            .flex_row()
            .items_center()
            .gap(display_gap)
            .w_full()
            .child(
                ui_element::label(display_text)
                    .text_color(text_col)
                    .text_size(font_size),
            );

        if spec.show_edit_icon && !spec.is_disabled {
            let icon_sz = rem_to_px(0.75);
            row = row.child(
                ui_element::icon("pencil")
                    .w(icon_sz)
                    .h(icon_sz)
                    .text_color(placeholder_color),
            );
        }

        if is_flush {
            // Flush display: no padding / border / radius, transparent bg.
            row.cursor_pointer()
        } else {
            // Default display: padding + radius + transparent border, hover hint.
            // Svelte hover = color-mix(border-default 72%, transparent) border
            //              + color-mix(surface 52%, transparent) bg.
            let border_default = resolve_color(theme, "color.border.default");
            let surface = resolve_color(theme, spec.fill_token());
            let transparent = glam::Vec4::ZERO;
            let hover_border = color_mix(border_default, transparent, 0.72);
            let hover_bg = color_mix(surface, transparent, 0.52);
            let transparent_c: Color = transparent.into();
            row.px(pad_x)
                .py(pad_y)
                .rounded(radius)
                .border(focus_width)
                .border_color(transparent_c)
                .cursor_pointer()
                .hover(move |s| s.border_color(hover_border).bg(hover_bg))
        }
    };

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        el = el.opacity(opacity).disabled(true);
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use jetstream_runtime::ui_element::WidgetKind;
    use poodle_specs::{EditableLabelSpec, EditableLabelVariant};

    fn test_theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    /// Recursively search the element tree for a Label whose text matches.
    fn has_label_text(el: &JsEl, needle: &str) -> bool {
        if let WidgetKind::Label(text) = &el.kind {
            if text == needle {
                return true;
            }
        }
        el.children.iter().any(|c| has_label_text(c, needle))
    }

    /// Find the first TextInput in the tree and return its seeded text.
    fn find_input_text(el: &JsEl) -> Option<String> {
        if let WidgetKind::TextInput { text, .. } = &el.kind {
            return Some(text.clone());
        }
        el.children.iter().find_map(find_input_text)
    }

    #[test]
    fn display_mode_renders_label_text() {
        let theme = test_theme();
        let spec = EditableLabelSpec::new().with_value("My project title");
        let el = js_editable_label(&spec, &theme);
        // Display mode renders a Label carrying the value — never a TextInput.
        assert!(
            has_label_text(&el, "My project title"),
            "display mode should render the label text"
        );
        assert!(
            find_input_text(&el).is_none(),
            "display mode must not render a TextInput"
        );
    }

    #[test]
    fn empty_display_uses_empty_text() {
        let theme = test_theme();
        let spec = EditableLabelSpec::new()
            .with_value("")
            .with_empty_text("Add a description…");
        let el = js_editable_label(&spec, &theme);
        assert!(
            has_label_text(&el, "Add a description…"),
            "empty value should fall back to empty_text"
        );
    }

    #[test]
    fn editing_mode_renders_input_with_value() {
        let theme = test_theme();
        let spec = EditableLabelSpec::new()
            .with_value("Short text")
            .with_editing(true)
            .with_placeholder("Enter text…");
        let el = js_editable_label(&spec, &theme);
        // Editing mode renders the composed input seeded with the value.
        assert_eq!(
            find_input_text(&el).as_deref(),
            Some("Short text"),
            "editing mode should seed the input with the current value"
        );
    }

    #[test]
    fn flush_editing_differs_from_default() {
        let theme = test_theme();
        let default = js_editable_label(
            &EditableLabelSpec::new()
                .with_value("Heading")
                .with_editing(true),
            &theme,
        );
        let flush = js_editable_label(
            &EditableLabelSpec::new()
                .with_value("Heading")
                .with_editing(true)
                .with_variant(EditableLabelVariant::Flush),
            &theme,
        );
        // Default editing has a full uniform border + surface background;
        // flush editing has bottom-border only and no background fill.
        assert!(
            default.style.border_width > 0.0,
            "default editing should have a uniform border"
        );
        assert!(
            default.style.background.is_some(),
            "default editing should have a surface background"
        );
        assert!(
            flush.style.background.is_none(),
            "flush editing should not fill a background"
        );
        // Flush uses a per-side (bottom) border, so the uniform width stays 0.
        assert!(
            flush.style.border_width <= 0.0 || flush.style.border_widths[2] > 0.0,
            "flush editing should use a bottom-only border"
        );
    }

    #[test]
    fn disabled_reduces_opacity() {
        let theme = test_theme();
        let spec = EditableLabelSpec::new()
            .with_value("Read-only value")
            .with_disabled(true);
        let el = js_editable_label(&spec, &theme);
        assert!(el.style.disabled, "disabled spec should mark node disabled");
        assert!(el.style.opacity < 1.0, "disabled should reduce opacity");
    }
}
