//! Card — Jetstream card component backed by CardSpec.
//!
//! Contract: `docs/contracts/components/card.md`
//! Reference: `packages/svelte/primitives/src/Card.svelte`
//!
//! Uses Color::mix for hover/active states on interactive cards.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{CardLayout, CardSpec};

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub fn js_card(spec: &CardSpec, theme: &JetstreamThemeProvider, children: Vec<JsEl>) -> JsEl {
    let fill: Color = resolve_color(theme, spec.fill_token()).into();
    let radius = resolve_radius(theme, spec.radius_token());
    let gap = resolve_px(theme, spec.gap_token());
    let padding_x = resolve_px(theme, spec.padding_x_token());
    let padding_y = resolve_px(theme, spec.padding_y_token());

    // Hover state colors (contract: fill 92% + elevated 8%)
    let elevated: Color = resolve_color(theme, "color.background.elevated").into();
    let hover_fill = fill.mix(elevated, 0.92);

    let mut el = ui_element::div()
        .bg(fill)
        .rounded(radius)
        .pl(padding_x).pr(padding_x)
        .pt(padding_y).pb(padding_y)
        .gap(gap)
        .overflow_hidden();

    match spec.layout {
        CardLayout::Horizontal => { el = el.flex_row(); }
        _ => { el = el.flex_col(); }
    }

    if let Some(border_token) = spec.border_token() {
        let border_color = resolve_color(theme, border_token);
        el = el.border(1.0).border_color(border_color);
    }

    if let Some(selected_token) = spec.selected_border_token() {
        let selected_color = resolve_color(theme, selected_token);
        el = el.border(2.0).border_color(selected_color);
    }

    // Interactive cards get hover state
    if spec.is_interactive {
        el = el
            .hover(|s| s.bg(hover_fill))
            .cursor_pointer();
    }

    for child in children {
        el = el.child(child);
    }

    el
}
