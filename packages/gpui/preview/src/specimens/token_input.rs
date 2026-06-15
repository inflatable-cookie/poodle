use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::TokenInput;
use poodle_specs::{ControlDensity, ControlSize, TokenInputSpec};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(16.0))
        .max_w(px(560.0))
        .child(TokenInput::from_spec(
            TokenInputSpec::new()
                .with_values(vec!["ifrs".into(), "tax".into(), "audit".into()])
                .with_placeholder("Type a tag..."),
            theme,
        ))
        .child(TokenInput::from_spec(
            TokenInputSpec::new()
                .with_values(vec![
                    "legacy-migration".into(),
                    "a-very-long-tag-that-wraps-in-the-field".into(),
                ])
                .with_placeholder("Type a label...")
                .with_size(ControlSize::Sm),
            theme,
        ))
        .child(TokenInput::from_spec(
            TokenInputSpec::new()
                .with_values(vec!["premium".into(), "acca".into()])
                .with_read_only(true)
                .with_density(ControlDensity::Compact),
            theme,
        ))
        .child(TokenInput::from_spec(
            TokenInputSpec::new()
                .with_values(vec!["finance".into(), "ethics".into()])
                .with_disabled(true),
            theme,
        ))
}
