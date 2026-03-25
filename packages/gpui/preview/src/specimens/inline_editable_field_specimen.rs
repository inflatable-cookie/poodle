use gpui::*;
use poodle_composites::InlineEditableFieldSpec;
use poodle_gpui_components::{InlineEditableField, Eyebrow};
use poodle_primitives::EyebrowSpec;
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        // -- Default --
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(
                            InlineEditableField::from_spec(
                                InlineEditableFieldSpec::new("Click to edit this value"),
                                theme,
                            )
                        )
                        .child(
                            InlineEditableField::from_spec(
                                InlineEditableFieldSpec::new("Editing mode")
                                    .with_editing(true),
                                theme,
                            )
                        )
                        .child(
                            InlineEditableField::from_spec(
                                InlineEditableFieldSpec::new("")
                                    .with_placeholder("Empty \u{2014} click to add"),
                                theme,
                            )
                        )
                )
        )
}
