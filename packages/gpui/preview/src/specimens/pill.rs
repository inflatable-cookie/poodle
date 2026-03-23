use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{PillAppearance, PillFont, PillSize, PillSpec, PillTone};
use pug_gpui_components::Pill;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(24.0))
        // --- Tones ---
        .child(section_label("TONES", text_secondary))
        .child(
            div().flex().flex_wrap().gap(px(8.0)).items_center()
                .child(Pill::from_spec(PillSpec::new().with_label("Neutral").with_tone(PillTone::Neutral), theme))
                .child(Pill::from_spec(PillSpec::new().with_label("Info").with_tone(PillTone::Info), theme))
                .child(Pill::from_spec(PillSpec::new().with_label("Success").with_tone(PillTone::Success), theme))
                .child(Pill::from_spec(PillSpec::new().with_label("Warning").with_tone(PillTone::Warning), theme))
                .child(Pill::from_spec(PillSpec::new().with_label("Danger").with_tone(PillTone::Danger), theme))
        )
        // --- Sizes ---
        .child(section_label("SIZES", text_secondary))
        .child(
            div().flex().flex_wrap().gap(px(8.0)).items_center()
                .child(Pill::from_spec(PillSpec::new().with_label("Small").with_size(PillSize::Sm), theme))
                .child(Pill::from_spec(PillSpec::new().with_label("Medium").with_size(PillSize::Xs), theme))
        )
        // --- Code font ---
        .child(section_label("CODE FONT", text_secondary))
        .child(
            div().flex().flex_wrap().gap(px(8.0)).items_center()
                .child(Pill::from_spec(PillSpec::new().with_label("v2.4.1").with_font(PillFont::Mono), theme))
                .child(Pill::from_spec(PillSpec::new().with_label("stable").with_font(PillFont::Mono), theme))
                .child(Pill::from_spec(PillSpec::new().with_label("beta").with_font(PillFont::Mono), theme))
        )
        // --- Muted ---
        .child(section_label("MUTED", text_secondary))
        .child(
            div().flex().flex_wrap().gap(px(8.0)).items_center()
                .child(Pill::from_spec(PillSpec::new().with_label("Muted neutral").with_muted(true), theme))
                .child(Pill::from_spec(PillSpec::new().with_label("Muted success").with_tone(PillTone::Success).with_muted(true), theme))
                .child(Pill::from_spec(PillSpec::new().with_label("Muted danger").with_tone(PillTone::Danger).with_muted(true), theme))
        )
        // --- Badge ---
        .child(section_label("BADGE", text_secondary))
        .child(
            div().flex().flex_wrap().gap(px(8.0)).items_center()
                .child(Pill::from_spec(PillSpec::new().with_label("3").with_appearance(PillAppearance::Badge), theme))
                .child(Pill::from_spec(PillSpec::new().with_label("NEW").with_appearance(PillAppearance::Badge).with_tone(PillTone::Info), theme))
                .child(Pill::from_spec(PillSpec::new().with_label("PRO").with_appearance(PillAppearance::Badge).with_tone(PillTone::Success), theme))
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
