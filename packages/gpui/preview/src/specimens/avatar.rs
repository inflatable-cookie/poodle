use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::Avatar;
use poodle_specs::{AvatarShape, AvatarSize, AvatarSpec, AvatarTone};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(18.0))
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(12.0))
                .child(Avatar::from_spec(
                    AvatarSpec::new()
                        .with_initials("TA")
                        .with_size(AvatarSize::Xs),
                    theme,
                ))
                .child(Avatar::from_spec(
                    AvatarSpec::new()
                        .with_initials("TA")
                        .with_size(AvatarSize::Sm),
                    theme,
                ))
                .child(Avatar::from_spec(
                    AvatarSpec::new()
                        .with_initials("TA")
                        .with_size(AvatarSize::Md),
                    theme,
                ))
                .child(Avatar::from_spec(
                    AvatarSpec::new()
                        .with_initials("TA")
                        .with_size(AvatarSize::Lg),
                    theme,
                )),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(12.0))
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
                        .with_shape(AvatarShape::Rounded)
                        .with_tone(AvatarTone::Accent),
                    theme,
                )),
        )
}
