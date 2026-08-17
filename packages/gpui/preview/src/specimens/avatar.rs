use crate::app_state::AppState;
use crate::node_compat::{Avatar, Eyebrow};
use crate::specimens::specimen_layout::specimen_layout;
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{AvatarShape, AvatarSize, AvatarSpec, AvatarTone, ControlSize, EyebrowSpec};

fn avatar_size(size: ControlSize) -> AvatarSize {
    match size {
        ControlSize::Xs => AvatarSize::Xs,
        ControlSize::Sm => AvatarSize::Sm,
        ControlSize::Md => AvatarSize::Md,
        ControlSize::Lg => AvatarSize::Lg,
        ControlSize::Xl => AvatarSize::Xl,
    }
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
            "Initials",
            theme,
            Avatar::from_spec(AvatarSpec::new().with_initials("TA"), theme),
        ))
        .child(group(
            "Tone and shape",
            theme,
            div()
                .flex()
                .gap(px(12.0))
                .items_center()
                .child(Avatar::from_spec(
                    AvatarSpec::new()
                        .with_initials("AC")
                        .with_tone(AvatarTone::Neutral),
                    theme,
                ))
                .child(Avatar::from_spec(
                    AvatarSpec::new()
                        .with_initials("AC")
                        .with_tone(AvatarTone::Accent),
                    theme,
                ))
                .child(Avatar::from_spec(
                    AvatarSpec::new()
                        .with_initials("AC")
                        .with_tone(AvatarTone::Accent)
                        .with_shape(AvatarShape::Rounded),
                    theme,
                )),
        ))
        .child(group(
            "Image",
            theme,
            Avatar::from_spec(
                AvatarSpec::new()
                    .with_src("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 80 80'%3E%3Crect width='80' height='80' fill='%232563eb'/%3E%3Ccircle cx='40' cy='30' r='16' fill='%23fff'/%3E%3Cpath d='M14 74c5-18 17-28 26-28s21 10 26 28' fill='%23fff'/%3E%3C/svg%3E")
                    .with_alt("Example user")
                    .with_size(AvatarSize::Lg),
                theme,
            ),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "avatar",
        examples,
        |size, theme: &GpuiThemeProvider| {
            Avatar::from_spec(
                AvatarSpec::new()
                    .with_initials("TA")
                    .with_size(avatar_size(size)),
                theme,
            )
        },
        |_density, theme: &GpuiThemeProvider| {
            // Avatar has no density axis on the native spec.
            Avatar::from_spec(AvatarSpec::new().with_initials("TA"), theme)
        },
    )
}
