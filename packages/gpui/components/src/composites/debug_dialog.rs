use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ButtonSpec, CodeSpec, DebugDialogSpec};

use crate::{Button, Code};

pub struct DebugDialog {
    spec: DebugDialogSpec,
    theme: GpuiThemeProvider,
}

impl DebugDialog {
    pub fn from_spec(spec: DebugDialogSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for DebugDialog {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        if !self.spec.has_value() {
            return div().into_any_element();
        }

        div()
            .flex()
            .flex_col()
            .gap(px(12.0))
            .child(Button::from_spec(
                ButtonSpec::new()
                    .with_label(self.spec.trigger_label.clone())
                    .with_variant(self.spec.trigger_variant),
                &self.theme,
            ))
            .child(Code::from_spec(
                CodeSpec::new()
                    .with_content(self.spec.value.unwrap_or_default())
                    .with_language("json"),
                &self.theme,
            ))
            .into_any_element()
    }
}
