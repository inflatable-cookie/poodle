//! DetailSection — Jetstream detail section backed by DetailSectionSpec.
use jetstream_runtime::ui_element::{self, JsEl};
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

    // Spacing from tokens — no hardcoded values
    let section_gap = resolve_px(theme, spec.section_gap_token());
    let body_gap = resolve_px(theme, spec.body_gap_token());
    let header_gap = resolve_px(theme, spec.header_gap_token());
    let title_body_gap = resolve_px(theme, spec.title_body_gap_token());

    // Title font: 1.125rem heading; description: body size via token
    let title_font = rem_to_px(1.125);
    let body_font = resolve_px(theme, "typography.body.size");

    let mut el = ui_element::div().flex_col();

    // Top 1px separator — rendered when is_separated is true
    if spec.is_separated {
        el = el.child(
            ui_element::div()
                .h(1.0)
                .self_stretch()
                .bg(border)
                .mb(section_gap),
        );
    }

    // Header row: title block on left, optional actions on right
    let has_header = spec.title.is_some() || spec.description.is_some() || actions.is_some();
    if has_header {
        let mut header = ui_element::div()
            .flex_row()
            .items_center()
            .justify_between()
            .gap(header_gap)
            .mb(title_body_gap);

        // Title + description stacked vertically
        let mut title_block = ui_element::div()
            .flex_col()
            .gap(rem_to_px(0.375))
            .flex_grow();

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

    // Body: content rows in a flex column with body_gap spacing
    if !content.is_empty() {
        let mut body = ui_element::div().flex_col().gap(body_gap).self_stretch();
        for child in content {
            body = body.child(child);
        }
        el = el.child(body);
    }

    el
}
