use gpui::*;
use poodle_primitives::{EyebrowSpec, SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant};
use poodle_gpui_components::{Eyebrow, Spinner};

use crate::PreviewRoot;
use crate::app_state::AppState;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    div().flex().flex_col().gap(px(24.0))
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Ring"), theme))
                .child(
                    div().flex().items_center().gap(px(16.0))
                        .child(Spinner::from_spec(SpinnerSpec::new().with_size(SpinnerSize::Sm), theme))
                        .child(Spinner::from_spec(SpinnerSpec::new().with_size(SpinnerSize::Md), theme))
                        .child(Spinner::from_spec(SpinnerSpec::new().with_size(SpinnerSize::Lg), theme))
                )
        )
        .child(
            div().flex().flex_col().gap(px(10.0))
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
}
