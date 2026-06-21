//! DebugDialog — developer debug overlay backed by DebugDialogSpec.
//!
//! Contract: `docs/contracts/components/debug-dialog.md`.
//! Composes `Button` (trigger) + `Dialog` (surface) wrapping a `Code` block of
//! the value serialized as JSON. Renders nothing when `value` is null. The
//! trigger toggles the dialog `open` state — a preview-loop interaction, so
//! here both the trigger and the dialog surface are built for verification.
//! ZERO hardcoded px/hsla; all spacing/colour resolves from tokens.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ButtonSpec, CodeSpec, DebugDialogSpec, DialogWidth};

use crate::primitives::{Button, Code, Dialog};
use crate::theme_ext::resolve_px;

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
        // Hidden when value is null/undefined (contract: trigger only renders
        // when value is non-null).
        if !self.spec.has_value() {
            return div().into_any_element();
        }

        let theme = &self.theme;
        let spec = &self.spec;

        // Trigger button: variant + size from spec.
        let mut trigger_spec = ButtonSpec::new()
            .with_label(spec.trigger_label.clone())
            .with_variant(spec.trigger_variant);
        if let Some(size) = spec.trigger_size {
            trigger_spec = trigger_spec.with_size(size);
        }
        let trigger = Button::from_spec(trigger_spec, theme);

        // Code block: JSON value, max-height clamp from the spec's CSS string.
        let mut code_spec = CodeSpec::new()
            .with_content(spec.value.clone().unwrap_or_default())
            .with_language("json");
        if let Some(mh) = spec.max_height_px() {
            code_spec = code_spec.with_max_height(mh);
        }
        let code = Code::from_spec(code_spec, theme);

        // Dialog surface: title + close button (lg width, per Svelte). Open
        // state is parent-owned (preview loop) — built here for verification.
        let dialog = Dialog::from_spec(
            poodle_specs::DialogSpec::new()
                .with_title(spec.title.clone())
                .with_width(DialogWidth::Lg)
                .with_show_close_button(spec.show_close_button)
                .with_close_label(spec.close_label.clone()),
            theme,
        )
        .default_open(true)
        .with_content(code);

        // Stack the trigger above the dialog surface. Gap resolves from the
        // stack spacing token (replaces the prior hardcoded px(12.0)).
        div()
            .flex()
            .flex_col()
            .gap(resolve_px(theme, "space.stack.md"))
            .child(trigger)
            .child(dialog)
            .into_any_element()
    }
}
