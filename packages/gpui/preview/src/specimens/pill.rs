use gpui::*;
use flint_gpui::GpuiThemeProvider;
use flint_primitives::{PillAppearance, PillFont, PillSize, PillSpec, PillTone, EyebrowSpec};
use flint_gpui_components::{Pill, Eyebrow};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        // --- Tones ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Tones"), theme))
                .child(
                    div().flex().flex_wrap().gap(px(8.0)).items_center()
                        .child(Pill::from_spec(PillSpec::new().with_label("Neutral").with_tone(PillTone::Neutral), theme))
                        .child(Pill::from_spec(PillSpec::new().with_label("Info").with_tone(PillTone::Info), theme))
                        .child(Pill::from_spec(PillSpec::new().with_label("Success").with_tone(PillTone::Success), theme))
                        .child(Pill::from_spec(PillSpec::new().with_label("Warning").with_tone(PillTone::Warning), theme))
                        .child(Pill::from_spec(PillSpec::new().with_label("Danger").with_tone(PillTone::Danger), theme))
                )
        )
        // --- Sizes ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Sizes"), theme))
                .child(
                    div().flex().flex_wrap().gap(px(8.0)).items_center()
                        .child(Pill::from_spec(PillSpec::new().with_label("Small").with_size(PillSize::Sm), theme))
                        .child(Pill::from_spec(PillSpec::new().with_label("Medium"), theme))
                )
        )
        // --- Code font ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Code font"), theme))
                .child(
                    div().flex().flex_wrap().gap(px(8.0)).items_center()
                        .child(Pill::from_spec(PillSpec::new().with_label("v2.4.1").with_font(PillFont::Mono), theme))
                        .child(Pill::from_spec(PillSpec::new().with_label("stable").with_font(PillFont::Mono).with_tone(PillTone::Success), theme))
                        .child(Pill::from_spec(PillSpec::new().with_label("beta").with_font(PillFont::Mono).with_tone(PillTone::Warning), theme))
                )
        )
        // --- Muted ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Muted"), theme))
                .child(
                    div().flex().flex_wrap().gap(px(8.0)).items_center()
                        .child(Pill::from_spec(PillSpec::new().with_label("Muted neutral").with_muted(true), theme))
                        .child(Pill::from_spec(PillSpec::new().with_label("Muted success").with_tone(PillTone::Success).with_muted(true), theme))
                        .child(Pill::from_spec(PillSpec::new().with_label("Muted danger").with_tone(PillTone::Danger).with_muted(true), theme))
                )
        )
        // --- Badge ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Badge"), theme))
                .child(
                    div().flex().flex_wrap().gap(px(8.0)).items_center()
                        .child(Pill::from_spec(PillSpec::new().with_label("3").with_appearance(PillAppearance::Badge), theme))
                        .child(Pill::from_spec(PillSpec::new().with_label("12").with_appearance(PillAppearance::Badge), theme))
                        .child(Pill::from_spec(PillSpec::new().with_label("99+").with_appearance(PillAppearance::Badge), theme))
                        .child(Pill::from_spec(PillSpec::new().with_label("New").with_appearance(PillAppearance::Badge), theme))
                        .child(Pill::from_spec(PillSpec::new().with_label("Draft").with_appearance(PillAppearance::Badge).with_tone(PillTone::Neutral), theme))
                )
        )
}
