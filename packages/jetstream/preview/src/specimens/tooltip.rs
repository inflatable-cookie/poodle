//! Tooltip specimen — contract §13 Default + cardinal Placements.
//!
//! The Jetstream tooltip component renders only the bubble surface; trigger
//! anchoring and open/delay/dismiss are runtime/preview-loop concerns. This
//! specimen shows the Default bubble and the four cardinal placement bubbles
//! (the contract's 2×2 Placements grid), each tagged with its placement so the
//! bubble surface can be visually compared across placements.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::tooltip::js_tooltip;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{OverlayPlacement, TooltipSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // Default (top placement) — contract §13 Default.
        .child(group("Default", secondary,
            js_tooltip(
                &TooltipSpec::new()
                    .with_content("Save your changes")
                    .with_placement(OverlayPlacement::Top),
                theme,
            )
        ))
        // Placements — contract §13 2×2 grid (top / bottom / left / right).
        .child(group("Placements", secondary,
            div().flex_col().gap(12.0)
                .child(
                    div().flex_row().gap(12.0)
                        .child(placement(theme, "Top tooltip", OverlayPlacement::Top))
                        .child(placement(theme, "Bottom tooltip", OverlayPlacement::Bottom))
                )
                .child(
                    div().flex_row().gap(12.0)
                        .child(placement(theme, "Left tooltip", OverlayPlacement::Left))
                        .child(placement(theme, "Right tooltip", OverlayPlacement::Right))
                )
        ))
}

fn placement(theme: &JetstreamThemeProvider, content: &str, placement: OverlayPlacement) -> JsEl {
    js_tooltip(
        &TooltipSpec::new().with_content(content).with_placement(placement),
        theme,
    )
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
