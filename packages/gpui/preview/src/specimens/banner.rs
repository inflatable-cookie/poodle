use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{BannerSpec, StatusTone};
use pug_gpui_components::PugBanner;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(16.0)).max_w(px(400.0))
        // --- Info ---
        .child(section_label("INFO", text_secondary))
        .child(
            PugBanner::new(
                BannerSpec::new()
                    .with_tone(StatusTone::Info)
                    .with_title("Information")
                    .with_message("This is an informational banner message."),
                theme,
            )
        )
        // --- Success ---
        .child(section_label("SUCCESS", text_secondary))
        .child(
            PugBanner::new(
                BannerSpec::new()
                    .with_tone(StatusTone::Success)
                    .with_title("Success")
                    .with_message("Operation completed successfully."),
                theme,
            )
        )
        // --- Warning ---
        .child(section_label("WARNING", text_secondary))
        .child(
            PugBanner::new(
                BannerSpec::new()
                    .with_tone(StatusTone::Warning)
                    .with_title("Warning")
                    .with_message("Please review before proceeding."),
                theme,
            )
        )
        // --- Danger ---
        .child(section_label("DANGER", text_secondary))
        .child(
            PugBanner::new(
                BannerSpec::new()
                    .with_tone(StatusTone::Danger)
                    .with_title("Error")
                    .with_message("Something went wrong. Please try again."),
                theme,
            )
        )
        // --- Dismissible ---
        .child(section_label("DISMISSIBLE", text_secondary))
        .child(
            PugBanner::new(
                BannerSpec::new()
                    .with_tone(StatusTone::Info)
                    .with_title("Dismissible")
                    .with_message("This banner can be dismissed by the user.")
                    .with_dismissible(true),
                theme,
            )
        )
        // --- Without Icon ---
        .child(section_label("WITHOUT ICON", text_secondary))
        .child(
            PugBanner::new(
                BannerSpec::new()
                    .with_tone(StatusTone::Neutral)
                    .with_title("No icon")
                    .with_message("This banner has no leading icon.")
                    .with_icon(false),
                theme,
            )
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
