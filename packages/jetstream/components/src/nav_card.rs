//! NavCard — Jetstream navigation card backed by NavCardSpec.
//!
//! Contract: `docs/contracts/components/nav-card.md`
//! Reference: `packages/svelte/components/src/NavCard.svelte`

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::NavCardSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

pub fn js_nav_card(spec: &NavCardSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill: Color = resolve_color(theme, spec.fill_token()).into();
    let border = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let text_primary = resolve_color(theme, spec.title_color_token());
    let text_secondary = resolve_color(theme, spec.description_color_token());

    // Contract §8: title uses typography.label.size; description 0.8125rem.
    let title_font = resolve_px(theme, spec.title_typography_token());
    let desc_font = rem_to_px(spec.description_font_size_rem());

    // Contract §8: padding 0.875rem 1rem; gap 0.25rem.
    let pad_x = rem_to_px(spec.padding_x_rem());
    let pad_y = rem_to_px(spec.padding_y_rem());
    let gap = rem_to_px(spec.content_gap_rem());

    // Hover fill: elevated color blended over surface.
    let elevated: Color = resolve_color(theme, spec.hover_fill_token()).into();
    let hover_fill = fill.mix(elevated, 0.92);

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .pl(pad_x).pr(pad_x).pt(pad_y).pb(pad_y)
        .flex_col().gap(gap)
        .cursor_pointer()
        .hover(|s| s.bg(hover_fill))
        .focusable();

    // Title row (title + optional badge)
    let mut title_row = ui_element::div()
        .flex_row()
        .items_center()
        .gap(rem_to_px(0.375));

    title_row = title_row.child(
        ui_element::label(&spec.title)
            .text_color(text_primary)
            .text_size(title_font)
            .text_weight(600),
    );

    if let Some(ref badge_text) = spec.badge {
        let badge_bg = resolve_color(theme, spec.badge_bg_token());
        let badge_color = resolve_color(theme, spec.badge_color_token());
        title_row = title_row.child(
            ui_element::label(badge_text)
                .text_color(badge_color)
                .text_size(rem_to_px(0.6875))
                .bg(badge_bg)
                .rounded(999.0)
                .px(rem_to_px(0.375))
                .py(rem_to_px(0.125)),
        );
    }

    el = el.child(title_row);

    if let Some(ref desc) = spec.description {
        el = el.child(
            ui_element::label(desc)
                .text_color(text_secondary)
                .text_size(desc_font),
        );
    }

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        el = el.opacity(opacity).disabled(true);
    }

    el
}
