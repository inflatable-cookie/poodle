use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::Text;
use poodle_specs::{TextLeading, TextSize, TextSpec, TextTone, TextWeight};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(10.0))
        .child(Text::from_spec(TextSpec::new("Default body text."), theme))
        .child(Text::from_spec(
            TextSpec::new("Secondary supporting text.").with_tone(TextTone::Secondary),
            theme,
        ))
        .child(Text::from_spec(
            TextSpec::new("Success confirmation text.").with_tone(TextTone::Success),
            theme,
        ))
        .child(Text::from_spec(
            TextSpec::new("Danger validation text.").with_tone(TextTone::Danger),
            theme,
        ))
        .child(Text::from_spec(
            TextSpec::new("Extra-small label text.").with_size(TextSize::Xs),
            theme,
        ))
        .child(Text::from_spec(
            TextSpec::new("Relaxed semibold body copy.")
                .with_leading(TextLeading::Relaxed)
                .with_weight(TextWeight::Semibold),
            theme,
        ))
}
