use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{EyebrowSpec, RatingSpec};
use poodle_gpui_components::{Eyebrow, Rating};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_primary = theme.resolve_color("semantic.color.text.primary");

    let interactive_rating = state.specimens.selections.get("rating-interactive")
        .copied()
        .unwrap_or(3);

    div().flex().flex_col().gap(px(24.0))
        // --- Default (5 stars) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default (5 stars)"), theme))
                .child(Rating::from_spec(
                    RatingSpec::new().with_value(interactive_rating as f64),
                    theme,
                ).on_change(cx.listener(|this, val: &usize, _w, cx| {
                    this.state.specimens.selections.insert("rating-interactive".to_string(), *val);
                    cx.notify();
                })))
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(color_to_hsla(text_primary))
                        .child(format!("Rating: {} / 5", interactive_rating))
                )
        )
        // --- 10-star scale ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("10-star scale"), theme))
                .child(Rating::from_spec(
                    RatingSpec::new().with_value(7.0).with_max(10),
                    theme,
                ))
        )
        // --- Clearable ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Clearable"), theme))
                .child(Rating::from_spec(
                    RatingSpec::new().with_value(2.0),
                    theme,
                ))
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child(Rating::from_spec(
                    RatingSpec::new().with_value(2.0).with_disabled(true),
                    theme,
                ))
        )
}
