use gpui::*;
use poodle_composites::ConfirmActionSpec;
use poodle_gpui_components::{ConfirmAction, Button, Eyebrow};
use poodle_primitives::{ButtonSpec, ButtonVariant, StatusTone, EyebrowSpec};
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        // -- Default --
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child(
                    ConfirmAction::from_spec(
                        ConfirmActionSpec::new(
                            "Delete item?",
                            "This action cannot be undone. The item will be permanently removed.",
                            "Delete",
                            "Cancel",
                        ).with_tone(StatusTone::Danger),
                        theme,
                    ).with_trigger(
                        Button::from_spec(
                            ButtonSpec::new().with_variant(ButtonVariant::Danger).with_label("Delete item"),
                            theme,
                        ).with_id("misc-confirm-trigger")
                    )
                )
        )
}
