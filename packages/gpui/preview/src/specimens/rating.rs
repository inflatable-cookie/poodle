use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::RatingSpec;
use pug_gpui_components::Rating;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let text_primary = theme.resolve_color("semantic.color.text.primary");

    let interactive_rating = state.specimens.selections.get("rating-interactive")
        .copied()
        .unwrap_or(3);

    div().flex().flex_col().gap(px(24.0))
        // --- Interactive ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("INTERACTIVE (CLICK TO RATE)", text_secondary))
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
                .child(section_label("10-STAR SCALE", text_secondary))
                .child(Rating::from_spec(
                    RatingSpec::new().with_value(7.0).with_max(10),
                    theme,
                ))
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("DISABLED", text_secondary))
                .child(Rating::from_spec(
                    RatingSpec::new().with_value(2.0).with_disabled(true),
                    theme,
                ))
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(crate::style_bridge::color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
