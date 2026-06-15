use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Button, TextInput, UiPresentationProvider};
use poodle_specs::{
    ButtonSpec, ControlDensity, ControlSize, TextInputSpec, UiPresentationProviderSpec,
};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(16.0))
        .child(
            UiPresentationProvider::from_spec(
                UiPresentationProviderSpec::new()
                    .with_density(ControlDensity::Compact)
                    .with_size_scale(ControlSize::Sm),
            )
            .with_child(
                div()
                    .flex()
                    .gap(px(10.0))
                    .child(Button::from_spec(
                        ButtonSpec::new().with_label("Compact"),
                        theme,
                    ))
                    .child(TextInput::from_spec(
                        TextInputSpec::new().with_default_value("Small scope"),
                        theme,
                    )),
            ),
        )
        .child(
            UiPresentationProvider::from_spec(
                UiPresentationProviderSpec::new()
                    .with_density(ControlDensity::Comfortable)
                    .with_size_scale(ControlSize::Lg),
            )
            .with_child(
                div()
                    .flex()
                    .gap(px(10.0))
                    .child(Button::from_spec(
                        ButtonSpec::new().with_label("Comfortable"),
                        theme,
                    ))
                    .child(TextInput::from_spec(
                        TextInputSpec::new().with_default_value("Large scope"),
                        theme,
                    )),
            ),
        )
}
