use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::DebugDialog;
use poodle_specs::DebugDialogSpec;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(16.0))
        .child(DebugDialog::from_spec(
            DebugDialogSpec::new()
                .with_title("Asset payload")
                .with_trigger_label("Inspect payload")
                .with_value("{\n  \"id\": \"asset_42\",\n  \"status\": \"ready\"\n}"),
            theme,
        ))
        .child(DebugDialog::from_spec(DebugDialogSpec::new(), theme))
}
