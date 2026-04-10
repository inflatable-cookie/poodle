use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{ControlSize, PillAppearance, PillFont, PillSize, PillSpec, PillTone, EyebrowSpec};
use poodle_gpui_components::{Pill, Eyebrow};
use crate::app_state::AppState;
use crate::specimens::specimen_layout::specimen_layout;
use crate::PreviewRoot;

fn control_size_to_pill(size: ControlSize) -> PillSize {
    match size {
        ControlSize::Xs => PillSize::Xs,
        ControlSize::Sm => PillSize::Sm,
        ControlSize::Md => PillSize::Md,
        ControlSize::Lg => PillSize::Lg,
        ControlSize::Xl => PillSize::Xl,
    }
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let examples = div().flex().flex_col().gap(px(24.0))
        // --- Tones ---
        .child(
            div().flex().flex_col().gap(px(8.0))
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
        // --- Code font ---
        .child(
            div().flex().flex_col().gap(px(8.0))
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
            div().flex().flex_col().gap(px(8.0))
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
            div().flex().flex_col().gap(px(8.0))
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
        // --- Custom accent ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Custom accent"), theme))
                .child(
                    div().flex().flex_wrap().gap(px(8.0)).items_center()
                        .child(Pill::from_spec(PillSpec::new().with_label("Teal").with_accent_color("#14b8a6"), theme))
                        .child(Pill::from_spec(PillSpec::new().with_label("Purple").with_accent_color("#a855f7"), theme))
                        .child(Pill::from_spec(PillSpec::new().with_label("Pink").with_accent_color("#ec4899"), theme))
                )
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "pill",
        examples,
        |size, theme: &GpuiThemeProvider| {
            Pill::from_spec(
                PillSpec::new().with_label("Pill").with_size(control_size_to_pill(size)),
                theme,
            )
            .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            Pill::from_spec(
                PillSpec::new().with_label("Pill").with_density(density),
                theme,
            )
            .into_any_element()
        },
    )
}
