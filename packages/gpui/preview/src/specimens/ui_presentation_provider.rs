//! UiPresentationProvider — GPUI specimen.
//!
//! Contract: `docs/contracts/components/ui-presentation-provider.md`
//!
//! The provider renders no visual chrome of its own (contract §4/§12). Its
//! effect is demonstrated by wrapping real primitives at different `density`
//! and `sizeScale` values and observing the descendant sizing/spacing cascade.
//! This specimen covers all three contract §12 integration rows: compact/sm,
//! comfortable/lg, and a NESTED override (outer default/md, inner compact/sm).

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Button, Eyebrow, TextInput, UiPresentationProvider};
use poodle_specs::{
    ButtonSpec, ControlDensity, ControlSize, EyebrowSpec, TextInputSpec,
    UiPresentationProviderSpec,
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
                ButtonSpec::new().with_label(label.to_string()),
                theme,
            ))
            .child(TextInput::from_spec(
                TextInputSpec::new().with_default_value(label.to_string()),
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
        .child(Eyebrow::from_spec(EyebrowSpec::new().with_content(label), theme))
        .child(child)
}

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // ── Compact / sm region ──
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
        // ── Comfortable / lg region ──
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
        // ── Nested override (contract §12 row 3) ──
        // Outer provider sets default/md; an inner provider shadows it with
        // compact/sm for its subtree only. Outer controls stay default/medium;
        // inner controls render compact/small.
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
                                ButtonSpec::new().with_label("Outer default/md"),
                                theme,
                            ))
                            .child(TextInput::from_spec(
                                TextInputSpec::new().with_default_value("Outer scope"),
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
