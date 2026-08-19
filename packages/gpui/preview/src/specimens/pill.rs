use crate::app_state::AppState;
use crate::node_compat::{Eyebrow, Pill};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ControlSize, EyebrowSpec, InlineTypographyMode, PillAppearance, PillFont, PillSize, PillSpec,
    PillTone, SemanticControlSizeRole,
};

fn pill_size(size: ControlSize) -> PillSize {
    match size {
        ControlSize::Xs => PillSize::Xs,
        ControlSize::Sm => PillSize::Sm,
        ControlSize::Md => PillSize::Md,
        ControlSize::Lg => PillSize::Lg,
        ControlSize::Xl => PillSize::Xl,
    }
}

fn pill(theme: &GpuiThemeProvider, spec: PillSpec) -> Pill {
    Pill::from_spec(spec.with_size_role(SemanticControlSizeRole::Control), theme)
}

fn group(label: &str, theme: &GpuiThemeProvider, child: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(child)
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            "Tones",
            theme,
            div()
                .flex()
                .gap(px(8.0))
                .flex_wrap()
                .child(pill(theme, PillSpec::new().with_label("Neutral")))
                .child(pill(
                    theme,
                    PillSpec::new().with_label("Info").with_tone(PillTone::Info),
                ))
                .child(pill(
                    theme,
                    PillSpec::new()
                        .with_label("Success")
                        .with_tone(PillTone::Success),
                ))
                .child(pill(
                    theme,
                    PillSpec::new()
                        .with_label("Warning")
                        .with_tone(PillTone::Warning),
                ))
                .child(pill(
                    theme,
                    PillSpec::new()
                        .with_label("Danger")
                        .with_tone(PillTone::Danger),
                )),
        ))
        .child(group(
            "Code font",
            theme,
            div()
                .flex()
                .gap(px(8.0))
                .flex_wrap()
                .child(pill(
                    theme,
                    PillSpec::new()
                        .with_label("v2.4.1")
                        .with_font(PillFont::Mono),
                ))
                .child(pill(
                    theme,
                    PillSpec::new()
                        .with_label("stable")
                        .with_font(PillFont::Mono)
                        .with_tone(PillTone::Success),
                ))
                .child(pill(
                    theme,
                    PillSpec::new()
                        .with_label("beta")
                        .with_font(PillFont::Mono)
                        .with_tone(PillTone::Warning),
                )),
        ))
        .child(group(
            "Muted",
            theme,
            div()
                .flex()
                .gap(px(8.0))
                .flex_wrap()
                .child(pill(
                    theme,
                    PillSpec::new().with_label("Muted neutral").with_muted(true),
                ))
                .child(pill(
                    theme,
                    PillSpec::new()
                        .with_label("Muted success")
                        .with_tone(PillTone::Success)
                        .with_muted(true),
                ))
                .child(pill(
                    theme,
                    PillSpec::new()
                        .with_label("Muted danger")
                        .with_tone(PillTone::Danger)
                        .with_muted(true),
                )),
        ))
        .child(group(
            "Badge",
            theme,
            div()
                .flex()
                .gap(px(8.0))
                .flex_wrap()
                .child(pill(
                    theme,
                    PillSpec::new()
                        .with_label("3")
                        .with_appearance(PillAppearance::Badge),
                ))
                .child(pill(
                    theme,
                    PillSpec::new()
                        .with_label("12")
                        .with_appearance(PillAppearance::Badge),
                ))
                .child(pill(
                    theme,
                    PillSpec::new()
                        .with_label("99+")
                        .with_appearance(PillAppearance::Badge),
                ))
                .child(pill(
                    theme,
                    PillSpec::new()
                        .with_label("New")
                        .with_appearance(PillAppearance::Badge),
                ))
                .child(pill(
                    theme,
                    PillSpec::new()
                        .with_label("Draft")
                        .with_appearance(PillAppearance::Badge)
                        .with_tone(PillTone::Neutral),
                )),
        ))
        .child(group(
            "Inherited typography",
            theme,
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .text_size(px(20.0))
                .child(div().child("Release"))
                .child(pill(
                    theme,
                    PillSpec::new()
                        .with_label("Active")
                        .with_appearance(PillAppearance::Badge)
                        .with_tone(PillTone::Success)
                        .with_typography(InlineTypographyMode::Inherit),
                )),
        ))
        .child(group(
            "Custom accent",
            theme,
            div()
                .flex()
                .gap(px(8.0))
                .flex_wrap()
                .child(pill(
                    theme,
                    PillSpec::new()
                        .with_label("Info-ish")
                        .with_accent_color("#3b82f6"),
                ))
                .child(pill(
                    theme,
                    PillSpec::new()
                        .with_label("Positive-ish")
                        .with_accent_color("#22c55e"),
                ))
                .child(pill(
                    theme,
                    PillSpec::new()
                        .with_label("Caution-ish")
                        .with_accent_color("#f59e0b"),
                ))
                .child(pill(
                    theme,
                    PillSpec::new()
                        .with_label("Danger-ish")
                        .with_accent_color("#ef4444"),
                )),
        ))
        .child(group(
            "Appearances",
            theme,
            div()
                .flex()
                .gap(px(8.0))
                .flex_wrap()
                .child(pill(
                    theme,
                    PillSpec::new()
                        .with_label("Tint")
                        .with_tone(PillTone::Success)
                        .with_appearance(PillAppearance::Tint),
                ))
                .child(pill(
                    theme,
                    PillSpec::new()
                        .with_label("Solid")
                        .with_tone(PillTone::Success)
                        .with_appearance(PillAppearance::Solid),
                ))
                .child(pill(
                    theme,
                    PillSpec::new()
                        .with_label("Subtle")
                        .with_tone(PillTone::Success)
                        .with_appearance(PillAppearance::Subtle),
                ))
                .child(pill(
                    theme,
                    PillSpec::new()
                        .with_label("Badge")
                        .with_tone(PillTone::Success)
                        .with_appearance(PillAppearance::Badge),
                )),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "pill",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                Pill::from_spec(
                    PillSpec::new()
                        .with_label("Neutral")
                        .with_size(pill_size(size))
                        .with_size_role(SemanticControlSizeRole::Control),
                    theme,
                )
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                Pill::from_spec(
                    PillSpec::new()
                        .with_label("Neutral")
                        .with_density(density)
                        .with_size_role(SemanticControlSizeRole::Control),
                    theme,
                )
                .into_any_element()
            }),
    )
}
