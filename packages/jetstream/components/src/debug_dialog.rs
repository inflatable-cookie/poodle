//! JsDebugDialog — debug-data disclosure backed by DebugDialogSpec.
//!
//! Contract: `docs/contracts/components/debug-dialog.md`
//! Reference: GPUI `composites/debug_dialog.rs`.
//!
//! Renders a trigger button + a JSON code block when a value is present (the
//! open/close dialog behavior lives in the preview event loop). Nothing renders
//! without a value. ZERO hardcoded values.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ButtonSpec, CodeSpec, DebugDialogSpec};

use crate::button::js_button;
use crate::code::js_code;
use crate::presentation::rem_to_px;

/// Build a debug-dialog element from its spec.
pub fn js_debug_dialog(spec: &DebugDialogSpec, theme: &JetstreamThemeProvider) -> JsEl {
    if !spec.has_value() {
        return ui_element::div();
    }

    let mut button = ButtonSpec::new()
        .with_label(spec.trigger_label.as_str())
        .with_variant(spec.trigger_variant);
    if let Some(size) = spec.trigger_size {
        button = button.with_size(size);
    }

    ui_element::div()
        .flex_col()
        .gap(rem_to_px(0.75))
        .child(js_button(&button, theme))
        .child(js_code(
            &CodeSpec::new()
                .with_content(spec.value.clone().unwrap_or_default())
                .with_language("json"),
            theme,
        ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn renders_trigger_when_value_present() {
        let el = js_debug_dialog(&DebugDialogSpec::new().with_value("{\"a\":1}"), &theme());
        let tree = probe(&el, 320.0, 220.0);
        assert!(
            tree.texts().iter().any(|t| t.contains("View debug data")),
            "trigger label missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn empty_without_value() {
        let el = js_debug_dialog(&DebugDialogSpec::new(), &theme());
        let tree = probe(&el, 320.0, 220.0);
        assert_eq!(tree.len(), 1, "no-value debug dialog should render only an empty root");
    }
}
