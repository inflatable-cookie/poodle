//! Collapsible — Jetstream collapsible container backed by CollapsibleSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::CollapsibleSpec;

use crate::presentation::{panel_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

pub fn js_collapsible(spec: &CollapsibleSpec, theme: &JetstreamThemeProvider, content: Option<JsEl>) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let gap = resolve_px(theme, spec.content_gap_token());
    let is_open = spec.current_open();
    let text_color = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let font_size = rem_to_px(size_font_rem(effective_size));
    // Icon tracks body font size
    let icon_size = rem_to_px(size_font_rem(effective_size));

    // Surface styling
    let surface_fill = resolve_color(theme, "color.background.surface");
    let border_color = resolve_color(theme, "color.border.subtle");
    let radius = resolve_radius(theme, "radius.surface");

    // Density-based padding
    let pad_x = rem_to_px(panel_space_x_rem(spec.density));
    let pad_y = rem_to_px(0.625); // fixed 10px compact baseline

    let mut outer = ui_element::div()
        .flex_col()
        .bg(surface_fill)
        .border(1.0).border_color(border_color)
        .rounded(radius);

    // Header row
    let chevron_icon = if is_open { "chevron-down" } else { "chevron-right" };
    let header_gap = rem_to_px(0.25);
    let mut header = ui_element::div()
        .flex_row()
        .items_center()
        .gap(header_gap)
        .cursor_pointer()
        .pl(pad_x).pr(pad_x).pt(pad_y).pb(pad_y);

    header = header.child(
        ui_element::icon(chevron_icon)
            .w(icon_size).h(icon_size)
            .text_color(text_color)
    );

    if let Some(ref title) = spec.title {
        header = header.child(
            ui_element::label(title)
                .text_color(text_color)
                .text_size(font_size)
                .text_weight(600)
        );
    }

    // Description: shown inline below title when open
    if is_open {
        if let Some(ref description) = spec.description {
            let desc_font = rem_to_px(size_font_rem(effective_size) * 0.9375);
            header = header.child(
                ui_element::label(description)
                    .text_color(text_secondary)
                    .text_size(desc_font)
            );
        }
    }

    outer = outer.child(header);

    // Content (only when open), with horizontal padding to align with trigger text
    if is_open {
        if let Some(content_el) = content {
            let content_wrapper = ui_element::div()
                .flex_col()
                .gap(gap)
                .pl(pad_x).pr(pad_x)
                .child(content_el);
            outer = outer.child(content_wrapper);
        }
    }

    // Disabled state: reduce opacity of the whole element
    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "state.opacity.disabled");
        outer = outer.opacity(opacity);
    }

    outer
}
