//! DetailItem — Jetstream label/value row backed by DetailItemSpec.
//!
//! Contract: `docs/contracts/components/detail-item.md`
//! Reference: `packages/svelte/components/src/DetailItem.svelte`
//! Mirrors the GPUI build-out (`packages/gpui/components/src/primitives/detail_item.rs`).
//!
//! Inline (default) or stacked layout, simple/surface presentation,
//! density-driven spacing, an optional trailing action slot, an optional info
//! description (rendered inline — JsEl has no popover channel), and an em-dash
//! empty placeholder. ALL geometry/colour resolves from tokens / the
//! density-aware spec helpers (no raw font literals).

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{DetailItemLayout, DetailItemPresentation, DetailItemSpan, DetailItemSpec};

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// Render a label/value detail row. The optional trailing `action` slot
/// (a button, link, etc.) is rendered after the value when supplied.
pub fn js_detail_item(spec: &DetailItemSpec, theme: &JetstreamThemeProvider) -> JsEl {
    js_detail_item_with_slots(spec, theme, None, None)
}

/// Variant exposing the value-content and trailing-action slots (contract §2
/// `valueContent()` / `action()`). `value_content` replaces the simple text
/// value when present.
pub fn js_detail_item_with_slots(
    spec: &DetailItemSpec,
    theme: &JetstreamThemeProvider,
    value_content: Option<JsEl>,
    action: Option<JsEl>,
) -> JsEl {
    let label_color = resolve_color(theme, spec.label_color_token());
    let value_color = resolve_color(theme, spec.value_color_token());
    let desc_color = resolve_color(theme, spec.description_color_token());
    let tertiary_color = resolve_color(theme, spec.stacked_label_color_token());
    let bg = resolve_color(theme, spec.background_token());
    let radius = resolve_radius(theme, spec.radius_token());

    // Typography from tokens (contract §8) — no raw font literals.
    let label_font = resolve_px(theme, spec.label_size_token());
    let value_font = resolve_px(theme, spec.value_size_token());
    let desc_font = rem_to_px(0.75); // contract: info content 0.75rem

    // Density-aware spacing resolved from the spec (contract §7/§8).
    let row_gap = rem_to_px(spec.row_gap_rem());
    let inline_gap = rem_to_px(spec.inline_gap_rem());
    let pad_x = rem_to_px(spec.surface_padding_x_rem());
    let pad_y = rem_to_px(spec.surface_padding_y_rem());

    let is_stacked = spec.layout == DetailItemLayout::Stacked;
    let is_surface = spec.presentation == DetailItemPresentation::Surface;
    let is_surface_stacked = is_surface && is_stacked;

    // Surface+stacked: label shifts to tertiary, 0.75rem (contract §8).
    let (eff_label_color, eff_label_font) = if is_surface_stacked {
        (tertiary_color, rem_to_px(0.75))
    } else {
        (label_color, label_font)
    };
    // Surface+stacked: value emphasis — 1rem / weight 600.
    let (eff_value_font, value_weight) = if is_surface_stacked {
        (rem_to_px(1.0), 600u16)
    } else {
        (value_font, 400u16)
    };

    // ── Label block (label + optional inline description) ──────
    let mut label_block = ui_element::div().flex_col().gap(row_gap).child(
        ui_element::label(&spec.label)
            .text_color(eff_label_color)
            .text_size(eff_label_font),
    );
    if let Some(ref desc) = spec.description {
        label_block = label_block.child(
            ui_element::label(desc)
                .text_color(desc_color)
                .text_size(desc_font),
        );
    }
    // Inline layout: fixed label column (contract: minmax(8rem, 11.25rem)).
    if !is_stacked {
        label_block = label_block.w(rem_to_px(11.25)).flex_shrink_0();
    }

    // ── Value (slot > value text > em-dash placeholder) ────────
    let value_block = if let Some(content) = value_content {
        ui_element::div().grow().child(content)
    } else if let Some(ref value) = spec.value {
        let mut v = ui_element::label(value)
            .text_color(value_color)
            .text_size(eff_value_font)
            .text_weight(value_weight)
            .grow();
        if spec.truncate_value {
            v = v.text_ellipsis().whitespace_nowrap();
        }
        v
    } else {
        // Empty: em-dash placeholder in muted colour.
        ui_element::label(&spec.empty_text)
            .text_color(desc_color)
            .text_size(eff_value_font)
            .grow()
    };

    // ── Root row ───────────────────────────────────────────────
    let mut el = if is_stacked {
        ui_element::div().flex_col().gap(row_gap)
    } else {
        ui_element::div().flex_row().items_center().gap(inline_gap)
    };

    if is_surface_stacked {
        el = el.items_start();
    }

    // Surface presentation adds background, padding, radius; Simple is plain.
    if is_surface {
        el = el
            .bg(bg)
            .rounded(radius)
            .pl(pad_x)
            .pr(pad_x)
            .pt(pad_y)
            .pb(pad_y);
    }

    // span="full" stretches across a parent grid; approximate by self-stretch.
    if matches!(spec.span, Some(DetailItemSpan::Full)) {
        el = el.self_stretch();
    }

    el = el.child(label_block).child(value_block);

    if let Some(action_el) = action {
        el = el.child(
            ui_element::div()
                .flex_row()
                .items_center()
                .flex_shrink_0()
                .child(action_el),
        );
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::ControlDensity;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn renders_label_and_value() {
        let spec = DetailItemSpec::new("Status").with_value("Active");
        let el = js_detail_item(&spec, &theme());
        let tree = probe(&el, 400.0, 200.0);
        assert!(tree.has_text("Status"), "label missing: {:?}", tree.texts());
        assert!(tree.has_text("Active"), "value missing: {:?}", tree.texts());
    }

    #[test]
    fn renders_inline_description() {
        let spec = DetailItemSpec::new("Created")
            .with_value("2026-01-15")
            .with_description("Date the record was created");
        let el = js_detail_item(&spec, &theme());
        let tree = probe(&el, 400.0, 200.0);
        assert!(
            tree.has_text("Date the record was created"),
            "description missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn empty_value_renders_em_dash_placeholder() {
        // No value set → em-dash placeholder from the spec default.
        let spec = DetailItemSpec::new("Owner");
        let el = js_detail_item(&spec, &theme());
        let tree = probe(&el, 400.0, 200.0);
        assert!(
            tree.has_text("—"),
            "em-dash placeholder missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn surface_presentation_has_background() {
        use jetstream_ui::Color;
        let bg: Color = resolve_color(&theme(), DetailItemSpec::new("x").background_token()).into();
        let expected = crate::render_probe::ProbeColor {
            r: bg.r,
            g: bg.g,
            b: bg.b,
            a: bg.a,
        };
        let spec = DetailItemSpec::new("Total")
            .with_value("42")
            .with_presentation(DetailItemPresentation::Surface);
        let el = js_detail_item(&spec, &theme());
        let tree = probe(&el, 400.0, 200.0);
        assert!(
            tree.has_background(expected, 0.02),
            "surface background missing"
        );
    }

    #[test]
    fn simple_presentation_drops_background() {
        let spec = DetailItemSpec::new("Total")
            .with_value("42")
            .with_presentation(DetailItemPresentation::Simple);
        let el = js_detail_item(&spec, &theme());
        // Root has no background fill in simple presentation.
        assert!(
            el.style.background.is_none(),
            "simple presentation should not paint a background"
        );
    }

    #[test]
    fn action_slot_renders_after_value() {
        let action = ui_element::button("Copy").id("copy-btn");
        let spec = DetailItemSpec::new("API Key").with_value("sk-...");
        let el = js_detail_item_with_slots(&spec, &theme(), None, Some(action));
        let tree = probe(&el, 400.0, 200.0);
        assert!(
            tree.find_token("copy-btn").is_some(),
            "action slot missing from tree"
        );
    }

    #[test]
    fn compact_density_tightens_inline_gap() {
        // Default inline gap = 0.75rem (12px); compact = 0.5rem (8px).
        let default = js_detail_item(&DetailItemSpec::new("A").with_value("1"), &theme());
        let compact = js_detail_item(
            &DetailItemSpec::new("A")
                .with_value("1")
                .with_density(ControlDensity::Compact),
            &theme(),
        );
        assert_eq!(
            default.layout.gap.width,
            taffy::LengthPercentage::length(rem_to_px(0.75)),
            "default inline gap = 0.75rem"
        );
        assert_eq!(
            compact.layout.gap.width,
            taffy::LengthPercentage::length(rem_to_px(0.5)),
            "compact inline gap = 0.5rem"
        );
    }
}
