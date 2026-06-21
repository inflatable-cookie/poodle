//! Field — Jetstream form field wrapper backed by FieldSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{FieldSpec, ValidationState};

use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius, tint};

pub fn js_field(spec: &FieldSpec, theme: &JetstreamThemeProvider, control: Option<JsEl>) -> JsEl {
    let label_size = resolve_px(theme, spec.label_typography_token());
    let desc_color = resolve_color(theme, spec.description_color_token());
    let error_color = resolve_color(theme, spec.error_color_token());
    // Contract §8: label color = color-mix(text-primary 45%, text-secondary).
    let label_primary = resolve_color(theme, spec.label_color_primary_token());
    let label_secondary = resolve_color(theme, spec.label_color_secondary_token());
    let label_color = color_mix(
        label_primary,
        label_secondary,
        FieldSpec::LABEL_COLOR_PRIMARY_RATIO,
    );
    let row_gap = resolve_px(theme, spec.row_gap_token());
    let supporting_size = resolve_px(theme, spec.supporting_text_typography_token());

    // `span` / `grid_area` (FieldSpec) are CSS-grid placement props with no
    // Jetstream flex equivalent — accepted layout delta (contract §10/§12); the
    // owning layout positions the field.

    let mut el = ui_element::div().flex_col().gap(row_gap);

    // Header — space-between so the optional marker right-aligns (contract §8).
    let mut header = ui_element::div()
        .flex_row()
        .items_center()
        .justify_between()
        .gap(resolve_px(theme, spec.header_gap_token()));

    // Label row — label + required marker + info icon (contract §2).
    let mut label_row = ui_element::div()
        .flex_row()
        .items_center()
        .gap(resolve_px(theme, spec.label_row_gap_token()));

    label_row = label_row.child(
        ui_element::label(&spec.label)
            .text_color(label_color)
            .text_size(label_size)
            .text_weight(500),
    );

    if spec.is_required {
        label_row = label_row.child(
            ui_element::label("*")
                .text_color(error_color)
                .text_size(label_size),
        );
    }

    // Info icon — contract §2/§7/§8: pill wrapper next to the label when a
    // description/hint is set. The description renders here (not inline,
    // contract §4/§9). Popover open/close lives in the preview event loop
    // (accepted runtime limit) — the icon part + its content carry parity.
    if spec.info_text().is_some() {
        // em-relative to the label font: 1.25em wrapper, 0.75em glyph.
        let icon_box = label_size * FieldSpec::INFO_ICON_EM;
        let icon_glyph = label_size * FieldSpec::INFO_ICON_SVG_EM;
        let info_bg = tint(
            resolve_color(theme, spec.info_icon_bg_token()),
            FieldSpec::INFO_ICON_BG_ALPHA,
        );
        let info_color = resolve_color(theme, spec.info_icon_color_token());
        let info_radius = resolve_radius(theme, spec.info_icon_radius_token());
        label_row = label_row.child(
            ui_element::div()
                .flex_row()
                .items_center()
                .justify_center()
                .flex_shrink_0()
                .w(icon_box)
                .h(icon_box)
                .rounded(info_radius)
                .bg(info_bg)
                .cursor_pointer()
                .child(
                    ui_element::icon("info")
                        .w(icon_glyph)
                        .h(icon_glyph)
                        .text_color(info_color),
                ),
        );
    }

    header = header.child(label_row);

    // Optional marker — supporting (smaller) typography, sits at header end.
    if spec.shows_optional_label() {
        if let Some(ref opt_label) = spec.optional_label {
            header = header.child(
                ui_element::label(opt_label)
                    .text_color(desc_color)
                    .text_size(supporting_size)
                    .flex_shrink_0(),
            );
        }
    }

    el = el.child(header);

    // Control slot
    if let Some(control_el) = control {
        el = el.child(control_el);
    }

    // Description is NOT rendered inline (contract §4/§9) — it lives in the
    // info-icon affordance built above.

    // Error message
    if spec.validation_state == ValidationState::Invalid {
        if let Some(ref error) = spec.error {
            el = el.child(
                ui_element::label(error)
                    .text_color(error_color)
                    .text_size(supporting_size),
            );
        }
    }

    // Pending message
    if spec.validation_state == ValidationState::Pending {
        if let Some(ref pending) = spec.pending_message {
            el = el.child(
                ui_element::label(pending)
                    .text_color(desc_color)
                    .text_size(supporting_size),
            );
        }
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::FieldSpec;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn field_renders_label_required_marker_and_info_icon() {
        let th = theme();
        let spec = FieldSpec::new("display-name", "Display name")
            .with_required(true)
            .with_description("How your name appears to others.");
        let el = js_field(&spec, &th, None);
        let tree = probe(&el, 400.0, 200.0);

        // Label text present.
        assert!(tree.has_text("Display name"), "label missing: {:?}", tree.texts());
        // Required marker present.
        assert!(tree.has_text("*"), "required marker missing: {:?}", tree.texts());
        // Info-icon anatomy part present (icon named "info"), and description is
        // NOT rendered inline as visible body copy.
        assert!(
            tree.has_text("info"),
            "info-icon part missing: {:?}",
            tree.texts()
        );
        assert!(
            !tree.has_text("How your name appears to others."),
            "description must not render inline (contract §4/§9): {:?}",
            tree.texts()
        );
    }

    #[test]
    fn field_renders_error_message_at_supporting_size() {
        let th = theme();
        let spec = FieldSpec::new("username", "Username")
            .with_error("This username is already taken.")
            .with_validation_state(ValidationState::Invalid);
        let el = js_field(&spec, &th, None);
        let tree = probe(&el, 400.0, 200.0);

        assert!(
            tree.has_text("This username is already taken."),
            "error message missing: {:?}",
            tree.texts()
        );
        let supporting = resolve_px(&th, spec.supporting_text_typography_token());
        let err_node = tree
            .nodes
            .iter()
            .find(|n| n.text.as_deref() == Some("This username is already taken."))
            .expect("error node");
        if let Some(sz) = err_node.text_size {
            assert!(
                (sz - supporting).abs() < 0.01,
                "error message should use supporting size {supporting}, got {sz}"
            );
        }
    }

    #[test]
    fn field_optional_marker_uses_supporting_size_not_label_size() {
        let th = theme();
        let spec = FieldSpec::new("phone", "Phone number").with_optional_label("Optional");
        let el = js_field(&spec, &th, None);
        let tree = probe(&el, 400.0, 200.0);

        assert!(tree.has_text("Optional"), "optional marker missing: {:?}", tree.texts());
        let label_size = resolve_px(&th, spec.label_typography_token());
        let supporting = resolve_px(&th, spec.supporting_text_typography_token());
        // The two sizes differ at md so this assertion is meaningful.
        assert!(supporting < label_size, "supporting/label sizes should differ");
        let opt_node = tree
            .nodes
            .iter()
            .find(|n| n.text.as_deref() == Some("Optional"))
            .expect("optional node");
        if let Some(sz) = opt_node.text_size {
            assert!(
                (sz - supporting).abs() < 0.01,
                "optional marker should use supporting size {supporting}, got {sz}"
            );
        }
    }
}
