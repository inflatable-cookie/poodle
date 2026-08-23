//! UiPresentationProvider — GPUI specimen.
//!
//! Contract: `docs/contracts/components/ui-presentation-provider.md`
//! Architecture: `docs/architecture/010-native-presentation-construction-context.md`
//!
//! The provider is a construction-time boundary, not a painted node
//! (`poodle_render::context::ui_presentation_provider`): it derives a nested
//! `RenderContext`, builds its child inside that scope, and returns the child
//! unchanged. This specimen demonstrates the real cascade — every scoped
//! control below OMITS size/density and inherits them from its provider
//! scope. Nothing here copies provider values into child specs by hand.

use crate::app_state::AppState;
use crate::node_compat::Eyebrow;
use crate::specimens::specimen_axes::density_key;
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_render::context::{ui_presentation_provider, RenderContext};
use poodle_specs::{
    ButtonSpec, ControlDensity, ControlSize, EyebrowSpec, TextInputSpec, UiPresentationProviderSpec,
};

/// A button + text input row whose controls omit size/density and resolve
/// them from whatever context they are built with.
fn plain_row(ctx: &RenderContext<'_>, label: &str) -> Div {
    div()
        .flex()
        .gap(px(10.0))
        .child(poodle_gpui_node_backend::to_gpui(&poodle_render::button(
            &ButtonSpec::new().with_label(label.to_string()),
            ctx,
            None,
        )))
        .child(poodle_gpui_node_backend::to_gpui(
            &poodle_render::text_input(
                &TextInputSpec::new().with_default_value(label.to_string()),
                ctx,
                None,
            ),
        ))
}

/// A button + text input row built INSIDE a provider scope: both controls
/// omit size/density and inherit the provider's values.
fn scoped_row(spec: &UiPresentationProviderSpec, ctx: &RenderContext<'_>, label: &str) -> Div {
    ui_presentation_provider(spec, ctx, |scoped| plain_row(scoped, label))
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

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let ctx = RenderContext::new(theme);

    // Root defaults: no provider; omitted inputs resolve to md/default.
    let root_row = plain_row(&ctx, "Root md/default");

    // Nested override: an lg/comfortable scope whose closure holds one
    // inherited row and one nested sm/compact provider with its own
    // inherited row.
    let nested = ui_presentation_provider(
        &UiPresentationProviderSpec::new()
            .with_size_scale(ControlSize::Lg)
            .with_density(ControlDensity::Comfortable),
        &ctx,
        |outer| {
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(plain_row(outer, "Outer lg/comfortable"))
                .child(scoped_row(
                    &UiPresentationProviderSpec::new()
                        .with_size_scale(ControlSize::Sm)
                        .with_density(ControlDensity::Compact),
                    outer,
                    "Nested sm/compact",
                ))
        },
    );

    // Explicit reset: inside an xl/comfortable scope, an explicit md/default
    // button stays md/default beside an inherited xl/comfortable sibling.
    let explicit_reset = ui_presentation_provider(
        &UiPresentationProviderSpec::new()
            .with_size_scale(ControlSize::Xl)
            .with_density(ControlDensity::Comfortable),
        &ctx,
        |scoped| {
            div()
                .flex()
                .gap(px(10.0))
                .child(poodle_gpui_node_backend::to_gpui(&poodle_render::button(
                    &ButtonSpec::new()
                        .with_label("Explicit md/default")
                        .with_size(ControlSize::Md)
                        .with_density(ControlDensity::Default),
                    scoped,
                    None,
                )))
                .child(poodle_gpui_node_backend::to_gpui(&poodle_render::button(
                    &ButtonSpec::new().with_label("Inherited xl/comfortable"),
                    scoped,
                    None,
                )))
        },
    );

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group("Root defaults", theme, root_row))
        .child(group(
            "Inherited scope (compact / sm)",
            theme,
            scoped_row(
                &UiPresentationProviderSpec::new()
                    .with_size_scale(ControlSize::Sm)
                    .with_density(ControlDensity::Compact),
                &ctx,
                "Scoped compact/sm",
            ),
        ))
        .child(group(
            "Inherited scope (comfortable / lg)",
            theme,
            scoped_row(
                &UiPresentationProviderSpec::new()
                    .with_size_scale(ControlSize::Lg)
                    .with_density(ControlDensity::Comfortable),
                &ctx,
                "Scoped comfortable/lg",
            ),
        ))
        .child(group("Nested override", theme, nested))
        .child(group("Explicit reset inside a scope", theme, explicit_reset))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "ui-presentation-provider",
        examples,
        SpecimenAxes::examples_only().with_densities(|density, theme: &GpuiThemeProvider| {
            scoped_row(
                &UiPresentationProviderSpec::new()
                    .with_size_scale(ControlSize::Md)
                    .with_density(density),
                &RenderContext::new(theme),
                density_key(density),
            )
            .into_any_element()
        }),
    )
}
