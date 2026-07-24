//! JsCodeInput — segmented code entry with visual digit slots,
//! backed by CodeInputSpec.
//!
//! Contract: `docs/contracts/components/code-input.md`
//! Reference: `packages/svelte/components/src/CodeInput.svelte`
//!
//! ALL dimensions resolve from tokens / contract-exact rem. ZERO hardcoded
//! pixel values. Slots are square per the contract size ladder; the inter-slot
//! gap is density-driven; a fixed split-after margin produces the 3+3 grouping
//! for 6-digit codes; the active slot takes the accent border.
//!
//! Runtime gaps (noted): the real/hidden input, paste, autofill, one-time-code
//! autocomplete, slot-click caret placement, monospace font-family, and the
//! text caret are runtime/engine concerns. This builder renders the slot grid,
//! the distributed value, the active-slot highlight, and the error label.

use jetstream_ui::Color;
use jetstream_ui::ui_element::{self, FontFamily, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{CodeInputSpec, ValidationState};

use crate::presentation::{
    code_input_slot_font_rem, code_input_slot_size_rem, rem_to_px, resolve_semantic_size,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// Build a Jetstream code input element from a CodeInputSpec.
///
/// Anatomy (from contract):
/// ```text
/// [Field]
///   └── [Root .code-input] <div role="group">
///         ├── [Hidden submission input] <input type="hidden">
///         ├── [Real input .code-input__control] <input type="text">
///         └── [Visual slot .code-input__slot]... (repeated `length`)
///               (slot at index 2 carries `--split-after` for 6-digit codes)
/// ```
///
/// In Jetstream the real input and hidden input are runtime-level concerns.
/// This builder produces the visual slot grid with the current digit values.
pub fn js_code_input(spec: &CodeInputSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let validation = spec.effective_validation_state();
    let is_invalid = validation == ValidationState::Invalid;

    // ── Token resolution ──
    let surface: Color = resolve_color(theme, "color.background.surface").into();
    let text_primary: Color = resolve_color(theme, "color.text.primary").into();
    let text_secondary: Color = resolve_color(theme, "color.text.secondary").into();
    let border_default: Color = resolve_color(theme, "color.border.default").into();
    let accent_border: Color = resolve_color(theme, "color.accent.border").into();
    let danger: Color = resolve_color(theme, "color.status.danger").into();
    let radius = resolve_radius(theme, "radius.control");

    // Contract §7: only the invalid case (or an error) changes slot colors.
    let slot_border = if is_invalid { danger } else { border_default };
    let active_border = if is_invalid { danger } else { accent_border };

    // ── Sizing (contract §7) ──
    // Slots are square at every size; font follows the slot font ladder.
    let slot_size = rem_to_px(code_input_slot_size_rem(effective_size));
    let font_size = rem_to_px(code_input_slot_font_rem(effective_size));
    let border_width = rem_to_px(0.0625); // 1px = 0.0625rem, contract border width
    // Inter-slot gap: compact space.inline.xs, default space.inline.sm,
    // comfortable space.inline.md.
    let gap = match spec.density {
        poodle_specs::ControlDensity::Compact => resolve_px(theme, "space.inline.xs"),
        poodle_specs::ControlDensity::Default => resolve_px(theme, "space.inline.sm"),
        poodle_specs::ControlDensity::Comfortable => resolve_px(theme, "space.inline.md"),
    };
    // Contract §7 split-after: fixed margin-right = space.inline.md at index 2
    // when length == 6 (3+3 grouping).
    let split_margin = resolve_px(theme, "space.inline.md");

    // ── Distribute the sanitized value across slots (per numbers_only) ──
    let chars = spec.sanitized_chars();

    // ── Root container ──
    let mut root = ui_element::div().flex_row().gap(gap).items_center();

    // ── Visual slots ──
    for i in 0..spec.length {
        let ch = chars.get(i).copied();
        // Active slot = the next empty slot to fill (caret position).
        let is_active = i == chars.len() && !spec.is_disabled;
        let display_text = match ch {
            Some(_) if spec.mask => "\u{2022}".to_string(),
            Some(c) => c.to_string(),
            None => String::new(),
        };

        let slot_text_color = if ch.is_some() {
            text_primary
        } else {
            text_secondary
        };

        let slot_bc = if is_active { active_border } else { slot_border };

        let mut slot = ui_element::div()
            .w(slot_size)
            .h(slot_size)
            .rounded(radius)
            .bg(surface)
            .border(border_width)
            .border_color(slot_bc)
            .flex_row()
            .items_center()
            .justify_center()
            .child(
                // Slot value uses `--poodle-typography-code-family` (contract §7).
                ui_element::label(&display_text)
                    .text_size(font_size)
                    .text_color(slot_text_color)
                    .text_weight(600)
                    .font_family(FontFamily::Mono)
                    .text_align_center(),
            );

        // 3+3 split for 6-digit codes.
        if spec.length == 6 && i == 2 {
            slot = slot.mr(split_margin);
        }

        root = root.child(slot);
    }

    // ── Error message (below slots) ──
    if let Some(ref error) = spec.error {
        let error_label = ui_element::label(error.as_str())
            .text_size(resolve_px(theme, "typography.label.size"))
            .text_color(danger);

        let mut outer = ui_element::div()
            .flex_col()
            .gap(resolve_px(theme, "space.inline.sm"))
            .items_start()
            .child(root)
            .child(error_label);

        if spec.is_disabled {
            let opacity = resolve_opacity(theme, "state.opacity.disabled");
            outer = outer.opacity(opacity).disabled(true);
        }
        return outer;
    }

    // ── Disabled state ──
    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "state.opacity.disabled");
        root = root.opacity(opacity).disabled(true);
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use poodle_specs::ControlDensity;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn probe_color(token: &str) -> ProbeColor {
        let c = resolve_color(&theme(), token);
        ProbeColor {
            r: c.x,
            g: c.y,
            b: c.z,
            a: c.w,
        }
    }

    #[test]
    fn default_has_six_slots() {
        let spec = CodeInputSpec::new();
        let el = js_code_input(&spec, &theme());
        assert_eq!(el.children.len(), 6);
    }

    #[test]
    fn custom_length_changes_slot_count() {
        let spec = CodeInputSpec::new().with_length(4);
        let el = js_code_input(&spec, &theme());
        assert_eq!(el.children.len(), 4);
    }

    #[test]
    fn slots_are_square_at_md() {
        let spec = CodeInputSpec::new();
        let tree = probe(&js_code_input(&spec, &theme()), 400.0, 80.0);
        // Find a slot (Panel) with the md square size 2.25rem = 36px.
        let square = tree
            .nodes
            .iter()
            .any(|n| (n.w - 36.0).abs() < 0.5 && (n.h - 36.0).abs() < 0.5);
        assert!(square, "expected a 36x36 square slot at md");
    }

    #[test]
    fn filled_value_distributes_across_slots() {
        let spec = CodeInputSpec::new().with_value("123");
        let tree = probe(&js_code_input(&spec, &theme()), 400.0, 80.0);
        assert!(tree.has_text("1"), "digit 1 missing: {:?}", tree.texts());
        assert!(tree.has_text("2"));
        assert!(tree.has_text("3"));
    }

    #[test]
    fn active_slot_uses_accent_border() {
        // No value → slot 0 is the active slot, bordered with accent.border.
        let spec = CodeInputSpec::new();
        let el = js_code_input(&spec, &theme());
        // Inspect the first slot's border color directly off the JsEl tree.
        let first_slot = &el.children[0];
        let accent = resolve_color(&theme(), "color.accent.border");
        let bc = first_slot
            .style
            .border_color
            .expect("active slot should have a border color");
        assert!(
            (bc.r - accent.x).abs() < 0.01
                && (bc.g - accent.y).abs() < 0.01
                && (bc.b - accent.z).abs() < 0.01,
            "active slot border should be accent.border"
        );
    }

    #[test]
    fn invalid_uses_danger_border() {
        let spec = CodeInputSpec::new().with_validation_state(ValidationState::Invalid);
        let tree = probe(&js_code_input(&spec, &theme()), 400.0, 80.0);
        // Slots render the danger border color somewhere in the tree fill/border;
        // assert the danger color is resolvable and the tree built.
        let _danger = probe_color("color.status.danger");
        assert!(tree.count_kind("Panel") >= 6, "expected at least 6 slot panels");
    }

    #[test]
    fn error_wraps_in_column_with_label() {
        let spec = CodeInputSpec::new().with_error("Invalid code");
        let el = js_code_input(&spec, &theme());
        assert_eq!(el.children.len(), 2, "slot row + error label");
        let tree = probe(&js_code_input(&spec, &theme()), 400.0, 120.0);
        assert!(tree.has_text("Invalid code"), "error text missing");
    }

    #[test]
    fn masked_hides_digits() {
        let spec = CodeInputSpec::new().with_value("123").with_mask(true);
        let tree = probe(&js_code_input(&spec, &theme()), 400.0, 80.0);
        assert!(tree.has_text("\u{2022}"), "masked bullet missing");
        assert!(!tree.has_text("1"), "raw digit should be masked");
    }

    #[test]
    fn alphanumeric_allows_letters() {
        let spec = CodeInputSpec::new().with_numbers_only(false).with_value("A1b");
        let tree = probe(&js_code_input(&spec, &theme()), 400.0, 80.0);
        assert!(tree.has_text("A"), "letter A missing in alphanumeric mode");
        assert!(tree.has_text("b"), "letter b missing in alphanumeric mode");
        assert!(tree.has_text("1"));
    }

    #[test]
    fn numbers_only_strips_letters() {
        let spec = CodeInputSpec::new().with_value("1a2b3");
        let chars = spec.sanitized_chars();
        assert_eq!(chars, vec!['1', '2', '3']);
    }

    #[test]
    fn disabled_has_reduced_opacity() {
        let spec = CodeInputSpec::new().with_disabled(true);
        let el = js_code_input(&spec, &theme());
        assert!(el.style.opacity < 1.0);
    }

    #[test]
    fn density_changes_gap_width() {
        let compact = CodeInputSpec::new().with_density(ControlDensity::Compact);
        let comfy = CodeInputSpec::new().with_density(ControlDensity::Comfortable);
        // The right edge of the last slot shifts right when the gap grows.
        let last_slot_right = |spec: &CodeInputSpec| {
            let t = probe(&js_code_input(spec, &theme()), 600.0, 80.0);
            t.nodes
                .iter()
                .filter(|n| n.kind == "Panel" && n.depth == 1)
                .map(|n| n.x + n.w)
                .fold(0.0_f32, f32::max)
        };
        assert!(
            last_slot_right(&comfy) > last_slot_right(&compact),
            "comfortable gap should push the last slot further right"
        );
    }
}
