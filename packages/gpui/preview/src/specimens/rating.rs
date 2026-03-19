use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::RatingSpec;
use pug_gpui_components::PugRating;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(24.0))
        // --- Default (5 stars) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("DEFAULT (5 STARS)", text_secondary))
                .child(PugRating::new(
                    RatingSpec::new().with_value(3.0),
                    theme,
                ))
        )
        // --- 10-star scale ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("10-STAR SCALE", text_secondary))
                .child(PugRating::new(
                    RatingSpec::new().with_value(7.0).with_max(10),
                    theme,
                ))
        )
        // --- Clearable ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("CLEARABLE", text_secondary))
                .child(PugRating::new(
                    RatingSpec::new().with_value(4.0),
                    theme,
                ))
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("DISABLED", text_secondary))
                .child(PugRating::new(
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
