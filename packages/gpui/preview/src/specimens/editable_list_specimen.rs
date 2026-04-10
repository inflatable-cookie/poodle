use gpui::*;
use poodle_gpui_components::{EditableList, Eyebrow};
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
                            EditableList::new(theme)
                                .aria_label("Tags")
                                .items(vec![
                                    "design-system".to_string(),
                                    "rust".to_string(),
                                    "gpui".to_string(),
                                ])
                                .max_items(10)
                                .reorderable(true)
                        )
                        .child(
                            EditableList::new(theme)
                                .aria_label("Disabled list")
                                .items(vec![
                                    "locked-item".to_string(),
                                ])
                                .disabled(true)
                        )
                )
        )
}
