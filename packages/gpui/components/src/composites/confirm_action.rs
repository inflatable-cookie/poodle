//! ConfirmAction — composes the `AlertDialog` primitive (which itself composes
//! `Dialog` + `Button`) with a trigger that is, by default, a composed
//! secondary `Button`. Mirrors the Svelte reference (`ConfirmAction.svelte`):
//! the component owns the open/trigger wiring and tone derivation, and
//! delegates every dialog/button visual to the composed primitives so it never
//! re-implements (and never drifts from) the AlertDialog/Button contracts.
//!
//! Svelte parity notes:
//! - Default trigger: `Button variant="secondary"` with derived tone
//!   (`ConfirmAction.svelte:86`); custom trigger replaces it (lines 69-84).
//! - Trigger tone: `tone === "danger" ? "danger" : "default"` (line 48).
//! - Dialog: delegates to `AlertDialog` with the same tone/labels/size/density
//!   and the `children` body slot (lines 91-105).

use crate::primitives::{AlertDialog, Button};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    AlertDialogSpec, AlertDialogTone, ButtonTone, ButtonVariant, ControlDensity, ControlSize,
    ConfirmActionSpec, SemanticControlSizeRole, StatusTone,
};

pub struct ConfirmAction {
    spec: ConfirmActionSpec,
    theme: GpuiThemeProvider,
    /// Optional trigger element that opens the confirmation dialog. When
    /// absent, a composed secondary Button (with derived tone) is rendered.
    trigger: Option<AnyElement>,
    /// Optional body content rendered between the description and the
    /// action row (matches the Svelte default slot).
    content: Option<AnyElement>,
    on_confirm: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_cancel: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for ConfirmAction {
    type Target = ConfirmActionSpec;
    fn deref(&self) -> &ConfirmActionSpec {
        &self.spec
    }
}

impl ConfirmAction {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: ConfirmActionSpec::new("Confirm", "Are you sure?", "Confirm", "Cancel"),
            theme: theme.clone(),
            trigger: None,
            content: None,
            on_confirm: None,
            on_cancel: None,
        }
    }
    pub fn from_spec(spec: ConfirmActionSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            trigger: None,
            content: None,
            on_confirm: None,
            on_cancel: None,
        }
    }

    /// Set a trigger element (e.g. a button) that opens the confirmation dialog.
    pub fn with_trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    /// Body content rendered between the description and the action row.
    pub fn with_content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    /// Set the open state via spec. Included as a fluent shortcut alongside
    /// `ConfirmActionSpec::with_open`.
    pub fn open(mut self, v: bool) -> Self {
        self.spec.is_open = v;
        self
    }

    /// Fired when the confirm button is clicked. Signature matches
    /// `Div::on_click` so specimen code can pass `cx.listener(...)` directly.
    pub fn on_confirm(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_confirm = Some(Box::new(handler));
        self
    }

    /// Fired when the cancel button is clicked. Signature matches
    /// `Div::on_click` so specimen code can pass `cx.listener(...)` directly.
    pub fn on_cancel(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_cancel = Some(Box::new(handler));
        self
    }
    pub fn with_size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn with_density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }

    /// Svelte: `triggerTone = tone === "danger" ? "danger" : "default"`.
    fn trigger_button_tone(&self) -> ButtonTone {
        if self.spec.is_destructive() {
            ButtonTone::Danger
        } else {
            ButtonTone::Default
        }
    }

    /// Map the ConfirmAction `StatusTone` onto the `AlertDialogTone` the
    /// composed AlertDialog accepts. Only danger vs. non-danger is meaningful
    /// (AlertDialog has just `Danger | Warning`); non-danger → `Warning`, which
    /// resolves to the default (accent) confirm Button — matching Svelte where
    /// non-danger tones map the confirm Button to `default`.
    fn alert_tone(&self) -> AlertDialogTone {
        match self.spec.tone {
            StatusTone::Danger => AlertDialogTone::Danger,
            _ => AlertDialogTone::Warning,
        }
    }
}

impl IntoElement for ConfirmAction {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;

        // When closed, render the trigger. Prefer a caller-provided trigger;
        // otherwise compose a secondary Button with derived tone — matching the
        // Svelte reference (`ConfirmAction.svelte:85-89`).
        if !self.spec.is_open {
            return if let Some(trigger) = self.trigger {
                div().child(trigger).into_any_element()
            } else {
                Button::new(theme)
                    .with_id("confirm-action-trigger")
                    .variant(ButtonVariant::Secondary)
                    .tone(self.trigger_button_tone())
                    .size(self.spec.size)
                    .size_role(self.spec.size_role)
                    .density(self.spec.density)
                    .label(self.spec.trigger_label.clone())
                    .into_any_element()
            };
        }

        // Open: delegate to the AlertDialog primitive (Dialog + Buttons).
        let alert_spec = AlertDialogSpec::new(self.spec.title.clone())
            .with_tone(self.alert_tone())
            .with_confirm_label(self.spec.confirm_label.clone())
            .with_cancel_label(self.spec.cancel_label.clone())
            .with_open(true)
            .with_size(self.spec.size)
            .with_size_role(self.spec.size_role)
            .with_density(self.spec.density);

        let mut alert = AlertDialog::from_spec(alert_spec, theme)
            .description(self.spec.description.clone());

        if let Some(content) = self.content {
            alert = alert.with_content(content);
        }

        // Adapt the AlertDialog `Fn(&mut Window, &mut App)` callbacks to the
        // `Fn(&ClickEvent, ...)` handlers this composite exposes.
        if let Some(handler) = self.on_confirm {
            alert = alert.on_confirm(move |window, cx| {
                handler(&ClickEvent::default(), window, cx);
            });
        }
        if let Some(handler) = self.on_cancel {
            alert = alert.on_cancel(move |window, cx| {
                handler(&ClickEvent::default(), window, cx);
            });
        }

        // Render the trigger alongside the open dialog when one is provided,
        // so the trigger remains visible behind the overlay (matches Svelte
        // where the trigger and AlertDialog are siblings).
        if let Some(trigger) = self.trigger {
            div()
                .child(trigger)
                .child(alert)
                .into_any_element()
        } else {
            alert.into_any_element()
        }
    }
}
