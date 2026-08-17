//! UiPresentationProvider — GPUI specimen.
//!
//! Contract: `docs/contracts/components/ui-presentation-provider.md`
//!
//! The provider renders no visual chrome of its own (contract §4/§12). Native
//! ambient propagation is a declared capability absence, so this specimen
//! shows the explicit child-spec values a host must currently supply. It does
//! not claim the passthrough wrapper caused a cascade.

use crate::node_compat::{Button, Eyebrow, TextInput};
use crate::providers::UiPresentationProvider;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ButtonSpec, ControlDensity, ControlSize, EyebrowSpec, TextInputSpec, UiPresentationProviderSpec,
};

/// A row of real controls scoped by a presentation provider.
fn scoped_controls(
    density: ControlDensity,
    size: ControlSize,
    label: &str,
    theme: &GpuiThemeProvider,
) -> UiPresentationProvider {
    UiPresentationProvider::from_spec(
        UiPresentationProviderSpec::new()
            .with_density(density)
            .with_size_scale(size),
    )
    .with_child(
        div()
            .flex()
            .gap(px(10.0))
            .child(Button::from_spec(
                ButtonSpec::new()
                    .with_label(label.to_string())
                    .with_size(size)
                    .with_density(density),
                theme,
            ))
            .child(TextInput::from_spec(
                TextInputSpec::new()
                    .with_default_value(label.to_string())
                    .with_size(size)
                    .with_density(density),
                theme,
            )),
    )
}

/// An `Eyebrow`-labeled group wrapper.
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

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // Explicit host equivalent for compact / sm.
        .child(group(
            "Compact / sm region",
            theme,
            scoped_controls(
                ControlDensity::Compact,
                ControlSize::Sm,
                "Small scope",
                theme,
            ),
        ))
        // Explicit host equivalent for comfortable / lg.
        .child(group(
            "Comfortable / lg region",
            theme,
            scoped_controls(
                ControlDensity::Comfortable,
                ControlSize::Lg,
                "Large scope",
                theme,
            ),
        ))
        // Explicit host equivalent for a nested override. The provider
        // wrappers remain layout-neutral; the child specs carry the values.
        .child(group(
            "Nested override",
            theme,
            UiPresentationProvider::from_spec(
                UiPresentationProviderSpec::new()
                    .with_density(ControlDensity::Default)
                    .with_size_scale(ControlSize::Md),
            )
            .with_child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .child(
                        div()
                            .flex()
                            .gap(px(10.0))
                            .child(Button::from_spec(
                                ButtonSpec::new()
                                    .with_label("Outer default/md")
                                    .with_size(ControlSize::Md)
                                    .with_density(ControlDensity::Default),
                                theme,
                            ))
                            .child(TextInput::from_spec(
                                TextInputSpec::new()
                                    .with_default_value("Outer scope")
                                    .with_size(ControlSize::Md)
                                    .with_density(ControlDensity::Default),
                                theme,
                            )),
                    )
                    .child(scoped_controls(
                        ControlDensity::Compact,
                        ControlSize::Sm,
                        "Inner compact/sm",
                        theme,
                    )),
            ),
        ))
}
