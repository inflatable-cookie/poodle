//! CardRadioGroup — Jetstream single-select card group backed by CardRadioGroupSpec.
//!
//! Contract: `docs/contracts/components/card-radio-group.md`
//! Reference: GPUI `composites/card_radio_group.rs` (the Card-composition template).
//!
//! Each option composes the `Card` primitive (interactive, and selected when
//! chosen) so the selected fill/border/focus ring all come from Card's own
//! token-resolved treatment — never a hand-rolled accent tint. The Card body
//! carries a header row (radio indicator + title) and an optional description.
//!
//! Render-only: selection + arrow-key navigation live in the preview event loop
//! (`crate::render_probe` covers the rendered structure). ZERO hardcoded
//! hsla/px — every dimension comes from the spec size/density helpers or a token.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{CardRadioGroupSpec, CardSpec};

use crate::card::js_card;
use crate::presentation::{control_space_x_rem, rem_to_px, resolve_semantic_size};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub fn js_card_radio_group(spec: &CardRadioGroupSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // Contract §7/§8 size scale — resolved through the spec helpers, not literals.
    let indicator_size = rem_to_px(CardRadioGroupSpec::indicator_size_rem(effective_size));
    let dot_size = rem_to_px(CardRadioGroupSpec::dot_size_rem(effective_size));
    let indicator_border = rem_to_px(spec.indicator_border_rem());
    let title_font = rem_to_px(CardRadioGroupSpec::title_font_rem(effective_size));
    let description_font = rem_to_px(CardRadioGroupSpec::description_font_rem(effective_size));

    // Density-driven grid gap (contract §8: compact 0.5 · default 0.75 · comfortable 0.875).
    let grid_gap = rem_to_px(control_space_x_rem(spec.density));
    // Header row gap is density-fixed 0.5rem per contract §8.
    let header_gap = rem_to_px(0.5);
    // Body column rhythm between header and description.
    let body_gap = rem_to_px(0.25);

    let indicator_border_color = resolve_color(theme, spec.border_token());
    let accent = resolve_color(theme, "color.accent.base");
    let dot_color = resolve_color(theme, "color.text.inverse");
    let pill_radius = resolve_radius(theme, "radius.pill");
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");

    let current_value = spec.current_value();

    // Root: grid container (role="radiogroup" lives in the preview/ARIA layer).
    let mut root = ui_element::div().flex_row().flex_wrap().gap(grid_gap);

    for option in &spec.options {
        let is_selected = current_value == Some(option.value.as_str());
        let is_item_disabled = spec.is_disabled || option.is_disabled;

        // Radio indicator: border-only unchecked; accent fill + inner dot checked.
        let mut indicator = ui_element::div()
            .w(indicator_size)
            .h(indicator_size)
            .rounded(pill_radius)
            .items_center()
            .justify_center();

        if is_selected {
            indicator = indicator
                .bg(accent)
                .border(indicator_border)
                .border_color(accent)
                .child(
                    ui_element::div()
                        .w(dot_size)
                        .h(dot_size)
                        .rounded(pill_radius)
                        .bg(dot_color),
                );
        } else {
            indicator = indicator
                .border(indicator_border)
                .border_color(indicator_border_color);
        }

        // Header row: indicator + title. Title font-size comes from the contract
        // per-size scale (continuous rem), so it's a direct label rather than a
        // discrete TextSize — mirrors the GPUI template. Color/weight from tokens.
        let header = ui_element::div()
            .flex_row()
            .items_center()
            .gap(header_gap)
            .child(indicator)
            .child(
                ui_element::label(&option.label)
                    .text_color(text_primary)
                    .text_size(title_font)
                    .text_weight(600),
            );

        // Card body: header row + optional description.
        let mut body = ui_element::div().flex_col().gap(body_gap).child(header);
        if let Some(description) = &option.description {
            body = body.child(
                ui_element::label(description)
                    .text_color(text_secondary)
                    .text_size(description_font),
            );
        }

        // Compose the Card primitive — selected state owns the fill/border.
        let mut card_spec = CardSpec::new().interactive();
        if is_selected {
            card_spec = card_spec.selected();
        }
        let aria = option
            .aria_label
            .clone()
            .unwrap_or_else(|| option.label.clone());
        card_spec = card_spec.with_aria_label(aria);

        // `js_card` reports `button` for an interactive card, which is right in
        // general and wrong here: these are mutually exclusive choices, so each
        // is a `radio` carrying its own checked state.
        let mut card = js_card(&card_spec, theme, vec![body])
            .aria_role(jetstream_ui::accesskit::Role::RadioButton)
            .aria_checked(crate::aria::toggled(Some(is_selected)))
            .grow();

        if is_item_disabled {
            card = card.opacity(resolve_opacity(theme, "state.opacity.disabled"));
        }

        root = root.child(card);
    }

    if spec.is_disabled {
        root = root.opacity(resolve_opacity(theme, "state.opacity.disabled"));
    }

    // Contract: the group of cards is a `radiogroup`.
    root.aria_role(jetstream_ui::accesskit::Role::RadioGroup)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use poodle_specs::{ChoiceOption, ControlSize};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn options() -> Vec<ChoiceOption> {
        vec![
            ChoiceOption::new("free", "Free").with_description("Starter plan"),
            ChoiceOption::new("pro", "Pro").with_description("For teams"),
            ChoiceOption::new("ent", "Enterprise"),
        ]
    }

    fn accent_color(theme: &JetstreamThemeProvider) -> ProbeColor {
        let c = resolve_color(theme, "color.accent.base");
        ProbeColor {
            r: c.x,
            g: c.y,
            b: c.z,
            a: c.w,
        }
    }

    #[test]
    fn renders_each_option_title_and_description() {
        let th = theme();
        let el = js_card_radio_group(&CardRadioGroupSpec::new(options()), &th);
        let tree = probe(&el, 480.0, 240.0);
        assert!(
            tree.has_text("Free") && tree.has_text("Pro") && tree.has_text("Enterprise"),
            "option titles missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Starter plan") && tree.has_text("For teams"),
            "descriptions missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn selected_option_differs_from_unselected() {
        let th = theme();
        let accent = accent_color(&th);

        // Unselected group: no accent-filled indicator anywhere.
        let unselected = js_card_radio_group(&CardRadioGroupSpec::new(options()), &th);
        let unselected_tree = probe(&unselected, 480.0, 240.0);
        assert!(
            !unselected_tree.has_background(accent, 0.01),
            "unselected group should have no accent-filled indicator"
        );

        // Selecting "pro" fills that option's indicator with the accent color.
        let selected =
            js_card_radio_group(&CardRadioGroupSpec::new(options()).with_value("pro"), &th);
        let selected_tree = probe(&selected, 480.0, 240.0);
        assert!(
            selected_tree.has_background(accent, 0.01),
            "selected option must render an accent-filled radio indicator"
        );
    }

    #[test]
    fn custom_indicator_size_from_spec() {
        let th = theme();
        // Xl indicator is 1.375rem; Xs is 0.875rem — the probe sees distinct
        // indicator node widths driven entirely by the spec size helper.
        let xl = js_card_radio_group(
            &CardRadioGroupSpec::new(options())
                .with_value("pro")
                .with_size(ControlSize::Xl),
            &th,
        );
        let xs = js_card_radio_group(
            &CardRadioGroupSpec::new(options())
                .with_value("pro")
                .with_size(ControlSize::Xs),
            &th,
        );

        let xl_w = rem_to_px(CardRadioGroupSpec::indicator_size_rem(ControlSize::Xl));
        let xs_w = rem_to_px(CardRadioGroupSpec::indicator_size_rem(ControlSize::Xs));
        assert!(xl_w > xs_w, "xl indicator should be larger than xs");

        let xl_tree = probe(&xl, 480.0, 240.0);
        let xs_tree = probe(&xs, 480.0, 240.0);
        let has_w = |t: &crate::render_probe::ProbeTree, w: f32| {
            t.nodes
                .iter()
                .any(|n| (n.w - w).abs() < 0.5 && (n.h - w).abs() < 0.5)
        };
        assert!(has_w(&xl_tree, xl_w), "xl indicator width {xl_w} not found");
        assert!(has_w(&xs_tree, xs_w), "xs indicator width {xs_w} not found");
    }
}
