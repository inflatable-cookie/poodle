//! DetailSection — Jetstream detail section backed by DetailSectionSpec.
//!
//! Contract: `docs/contracts/components/detail-section.md`
//! Reference: `packages/svelte/components/src/DetailSection.svelte`
//! Mirrors the GPUI build-out (`packages/gpui/components/src/composites/detail_section.rs`).
//!
//! Titled grouping of detail rows: optional separator rule, header
//! (title + description + actions), body slot, density-driven spacing, and a
//! multi-column body (flex-wrap approximation of the Svelte grid). ALL
//! geometry/colour resolves from tokens / the density-aware spec helpers.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::DetailSectionSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px};

/// Render a titled detail section.
///
/// - `content`: body children (detail rows, form fields, etc.)
/// - `actions`: optional trailing action slot in the header row (e.g. an edit button)
pub fn js_detail_section(
    spec: &DetailSectionSpec,
    theme: &JetstreamThemeProvider,
    content: Vec<JsEl>,
    actions: Option<JsEl>,
) -> JsEl {
    let text_primary = resolve_color(theme, spec.title_color_token());
    let text_secondary = resolve_color(theme, spec.description_color_token());
    let border = resolve_color(theme, spec.separator_color_token());

    // Density-aware spacing resolved from the spec (contract §8).
    let root_gap = rem_to_px(spec.root_gap_rem());
    let header_gap = rem_to_px(spec.header_gap_rem());
    let title_gap = rem_to_px(spec.title_gap_rem());
    let body_gap = rem_to_px(spec.body_gap_rem());
    let separated_gap = rem_to_px(spec.separated_gap_rem());
    // Contract §8: separator rule height 0.0625rem.
    let separator_h = rem_to_px(0.0625);

    // Title font: 1.125rem heading (contract §8); description: body size token.
    let title_font = rem_to_px(1.125);
    let body_font = resolve_px(theme, "typography.body.size");

    let mut el = ui_element::div().flex_col();

    // Top separator rule — rendered when is_separated (density-driven gap).
    if spec.is_separated {
        el = el.child(
            ui_element::div()
                .h(separator_h)
                .self_stretch()
                .bg(border)
                .mb(separated_gap),
        );
    }

    // Header row: title block on start, optional actions on end.
    let has_header = spec.title.is_some() || spec.description.is_some() || actions.is_some();
    if has_header {
        let mut header = ui_element::div()
            .flex_row()
            .items_start()
            .justify_between()
            .gap(header_gap)
            .mb(root_gap);

        // Title + description stacked vertically (title-gap between).
        let mut title_block = ui_element::div().flex_col().gap(title_gap).flex_grow();

        if let Some(ref title) = spec.title {
            title_block = title_block.child(
                ui_element::label(title)
                    .text_color(text_primary)
                    .text_size(title_font)
                    .text_weight(700),
            );
        }

        if let Some(ref desc) = spec.description {
            title_block = title_block.child(
                ui_element::label(desc)
                    .text_color(text_secondary)
                    .text_size(body_font),
            );
        }

        header = header.child(title_block);

        if let Some(actions_el) = actions {
            header = header.child(
                ui_element::div()
                    .flex_row()
                    .items_center()
                    .flex_shrink_0()
                    .child(actions_el),
            );
        }

        el = el.child(header);
    }

    // Body: content rows.
    // columns > 1 → flex-wrap multi-column approximation of the Svelte grid.
    if !content.is_empty() {
        let mut body = if spec.columns > 1 {
            ui_element::div().flex_row().flex_wrap().gap(body_gap).self_stretch()
        } else {
            ui_element::div().flex_col().gap(body_gap).self_stretch()
        };
        for child in content {
            body = body.child(child);
        }
        el = el.child(body);
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

    fn rows(theme: &JetstreamThemeProvider) -> Vec<JsEl> {
        use crate::detail_item::js_detail_item;
        use poodle_specs::DetailItemSpec;
        vec![
            js_detail_item(&DetailItemSpec::new("Name").with_value("Acme"), theme),
            js_detail_item(&DetailItemSpec::new("Owner").with_value("Jo"), theme),
        ]
    }

    #[test]
    fn renders_header_title_description_and_body_rows() {
        let th = theme();
        let spec = DetailSectionSpec::new()
            .with_title("Project details")
            .with_description("Core metadata for this project.");
        let el = js_detail_section(&spec, &th, rows(&th), None);
        let tree = probe(&el, 600.0, 400.0);
        assert!(tree.has_text("Project details"), "title missing: {:?}", tree.texts());
        assert!(
            tree.has_text("Core metadata for this project."),
            "description missing: {:?}",
            tree.texts()
        );
        // Body rows made it through.
        assert!(tree.has_text("Name"), "body row label missing");
        assert!(tree.has_text("Acme"), "body row value missing");
    }

    #[test]
    fn separated_draws_divider_rule() {
        use jetstream_ui::Color;
        let th = theme();
        let border: Color =
            resolve_color(&th, DetailSectionSpec::new().separator_color_token()).into();
        let expected = crate::render_probe::ProbeColor {
            r: border.r,
            g: border.g,
            b: border.b,
            a: border.a,
        };
        let spec = DetailSectionSpec::new().with_title("Billing").with_separated(true);
        let el = js_detail_section(&spec, &th, rows(&th), None);
        let tree = probe(&el, 600.0, 400.0);
        assert!(
            tree.has_background(expected, 0.02),
            "separator rule (border-subtle) missing"
        );
    }

    #[test]
    fn unseparated_omits_divider_rule() {
        use jetstream_ui::Color;
        let th = theme();
        let border: Color =
            resolve_color(&th, DetailSectionSpec::new().separator_color_token()).into();
        let expected = crate::render_probe::ProbeColor {
            r: border.r,
            g: border.g,
            b: border.b,
            a: border.a,
        };
        let spec = DetailSectionSpec::new().with_title("Billing").with_separated(false);
        let el = js_detail_section(&spec, &th, rows(&th), None);
        let tree = probe(&el, 600.0, 400.0);
        assert!(
            !tree.has_background(expected, 0.02),
            "unseparated section should not draw the divider rule"
        );
    }

    #[test]
    fn actions_slot_renders_in_header() {
        let th = theme();
        let action = ui_element::button("Edit").id("edit-btn");
        let spec = DetailSectionSpec::new().with_title("Billing");
        let el = js_detail_section(&spec, &th, rows(&th), Some(action));
        let tree = probe(&el, 600.0, 400.0);
        assert!(
            tree.find_token("edit-btn").is_some(),
            "actions slot missing from header"
        );
    }

    #[test]
    fn multi_column_body_wraps() {
        let th = theme();
        let spec = DetailSectionSpec::new().with_title("Server metrics").with_columns(3);
        let el = js_detail_section(&spec, &th, rows(&th), None);
        // The body container (last child) uses a row direction + wrap for columns > 1.
        let body = el.children.last().expect("body present");
        assert_eq!(
            body.layout.flex_direction,
            taffy::FlexDirection::Row,
            "multi-column body should lay out as a wrapping row"
        );
        assert_eq!(
            body.layout.flex_wrap,
            taffy::FlexWrap::Wrap,
            "multi-column body should wrap"
        );
    }

    #[test]
    fn compact_density_tightens_body_gap() {
        let th = theme();
        let default = js_detail_section(
            &DetailSectionSpec::new().with_title("S"),
            &th,
            rows(&th),
            None,
        );
        let compact = js_detail_section(
            &DetailSectionSpec::new()
                .with_title("S")
                .with_density(ControlDensity::Compact),
            &th,
            rows(&th),
            None,
        );
        let body_gap = |el: &JsEl| el.children.last().unwrap().layout.gap.height;
        assert_eq!(
            body_gap(&default),
            taffy::LengthPercentage::length(rem_to_px(0.75)),
            "default body gap = 0.75rem"
        );
        assert_eq!(
            body_gap(&compact),
            taffy::LengthPercentage::length(rem_to_px(0.625)),
            "compact body gap = 0.625rem"
        );
    }
}
