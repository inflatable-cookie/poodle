use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{EyebrowSpec, SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant};
use poodle_gpui_components::{Eyebrow, Spinner};

use crate::PreviewRoot;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_primary = theme.resolve_color("semantic.color.text.primary");
    let text_inverse = theme.resolve_color("semantic.color.text.inverse");
    let surface_bg = theme.resolve_color("semantic.color.background.surface");
    let border_default = theme.resolve_color("semantic.color.border.default");
    let control_radius = theme.resolve_space("semantic.radius.control");

    div().flex().flex_col().gap(px(24.0))
        // --- Ring ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Ring"), theme))
                .child(
                    div().flex().items_center().gap(px(16.0))
                        .child(Spinner::from_spec(SpinnerSpec::new().with_size(SpinnerSize::Sm), theme))
                        .child(Spinner::from_spec(SpinnerSpec::new().with_size(SpinnerSize::Md), theme))
                        .child(Spinner::from_spec(SpinnerSpec::new().with_size(SpinnerSize::Lg), theme))
                )
        )
        // --- CLI grid ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("CLI grid"), theme))
                .child(
                    div().flex().items_center().gap(px(16.0))
                        .child(
                            Spinner::from_spec(
                                SpinnerSpec::new()
                                    .with_variant(SpinnerVariant::Grid)
                                    .with_size(SpinnerSize::Sm)
                                    .with_tone(SpinnerTone::Muted),
                                theme,
                            )
                        )
                        .child(
                            Spinner::from_spec(
                                SpinnerSpec::new()
                                    .with_variant(SpinnerVariant::Grid)
                                    .with_size(SpinnerSize::Md)
                                    .with_tone(SpinnerTone::Accent),
                                theme,
                            )
                        )
                        .child(
                            Spinner::from_spec(
                                SpinnerSpec::new()
                                    .with_variant(SpinnerVariant::Grid)
                                    .with_size(SpinnerSize::Lg),
                                theme,
                            )
                        )
                )
        )
        // --- Context tones ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Context tones"), theme))
                .child(
                    div().flex().items_center().gap(px(12.0))
                        // Inverse chip: dark text on light bg
                        .child(
                            div()
                                .min_w(px(40.0)).min_h(px(40.0))
                                .p(px(8.0))
                                .flex().items_center().justify_center()
                                .border_1()
                                .border_color(color_to_hsla(border_default))
                                .rounded(px(control_radius))
                                .bg(color_to_hsla(text_primary))
                                .text_color(color_to_hsla(text_inverse))
                                .child(
                                    Spinner::from_spec(
                                        SpinnerSpec::new().with_tone(SpinnerTone::Current),
                                        theme,
                                    )
                                )
                        )
                        // Accent chip
                        .child(
                            div()
                                .min_w(px(40.0)).min_h(px(40.0))
                                .p(px(8.0))
                                .flex().items_center().justify_center()
                                .border_1()
                                .border_color(color_to_hsla(border_default))
                                .rounded(px(control_radius))
                                .bg(color_to_hsla(surface_bg))
                                .child(
                                    Spinner::from_spec(
                                        SpinnerSpec::new().with_tone(SpinnerTone::Accent),
                                        theme,
                                    )
                                )
                        )
                        // Muted grid chip
                        .child(
                            div()
                                .min_w(px(40.0)).min_h(px(40.0))
                                .p(px(8.0))
                                .flex().items_center().justify_center()
                                .border_1()
                                .border_color(color_to_hsla(border_default))
                                .rounded(px(control_radius))
                                .bg(color_to_hsla(surface_bg))
                                .child(
                                    Spinner::from_spec(
                                        SpinnerSpec::new()
                                            .with_variant(SpinnerVariant::Grid)
                                            .with_tone(SpinnerTone::Muted),
                                        theme,
                                    )
                                )
                        )
                )
        )
}
