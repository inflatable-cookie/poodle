use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::EditableLabelSpec;
use pug_gpui_components::PugEditableLabel;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let display_spec = EditableLabelSpec::new()
        .with_value("Project title");

    let editing_spec = EditableLabelSpec::new()
        .with_value("Editing mode")
        .with_editing(true);

    let placeholder_spec = EditableLabelSpec::new()
        .with_placeholder("Click to edit...");

    let disabled_spec = EditableLabelSpec::new()
        .with_value("Disabled label")
        .with_disabled(true);

    div().flex().flex_col().gap(px(8.0))
        .child(PugEditableLabel::new(display_spec, theme))
        .child(PugEditableLabel::new(editing_spec, theme))
        .child(PugEditableLabel::new(placeholder_spec, theme))
        .child(PugEditableLabel::new(disabled_spec, theme))
}
