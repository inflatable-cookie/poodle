//! AlertDialog — composes the real `Dialog` primitive (surface + overlay +
//! backdrop + escape + close affordance) with two composed `Button`s for the
//! cancel/confirm actions, exactly as the Svelte reference does
//! (`AlertDialog.svelte`). The component owns only the alert-specific content
//! (title/description/optional item-detail row) and the tone→confirm-Button
//! mapping; everything visual (width, shadow, padding, radius, backdrop fill)
//! is delegated to the composed primitives so it stays in lock-step with the
//! Dialog/Button contracts and resolves entirely from tokens.
//!
//! Svelte parity notes:
//! - Surface: `Dialog role="alertdialog" width="sm"` (`AlertDialog.svelte:104-117`).
//! - Confirm tone: `tone === "danger" ? "danger" : "default"` — warning maps to
//!   the default (accent) Button tone (`AlertDialog.svelte:57`).
//! - Working state gates dismiss/close and disables both buttons while swapping
//!   the confirm label for `working_label` (`AlertDialog.svelte:113-137`).
//!
//! GPUI has no platform ARIA channel, so `role`/`aria-modal`/`aria-label` are
//! carried on the spec but not emitted (tracked repo-wide for GPUI).

use crate::primitives::{Button, Dialog};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    AlertDialogSpec, AlertDialogTone, ButtonTone, ButtonVariant, ControlDensity, ControlSize,
    DialogKind, DialogWidth, SemanticControlSizeRole,
};
use std::rc::Rc;

pub struct AlertDialog {
    spec: AlertDialogSpec,
    theme: GpuiThemeProvider,
    /// Optional item-detail row (`itemLabel`/`itemValue`) rendered between the
    /// description and any body content — matches Svelte lines 140-144.
    item_label: Option<String>,
    item_value: Option<String>,
    /// Optional body content rendered between the item-detail row and the
    /// actions (matches the Svelte default `children` slot).
    content: Option<AnyElement>,
    /// Working state: while true the confirm label is swapped for
    /// `working_label`, both buttons are disabled, and escape/backdrop/close
    /// dismissal is suppressed (Svelte `working` gating).
    working: bool,
    working_label: String,
    on_confirm: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    on_cancel: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
}

impl std::ops::Deref for AlertDialog {
    type Target = AlertDialogSpec;
    fn deref(&self) -> &AlertDialogSpec {
        &self.spec
    }
}

impl AlertDialog {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(AlertDialogSpec::default(), theme)
    }

    pub fn from_spec(spec: AlertDialogSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            item_label: None,
            item_value: None,
            content: None,
            working: false,
            working_label: "Working\u{2026}".to_string(),
            on_confirm: None,
            on_cancel: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn open(mut self, v: bool) -> Self {
        self.spec.open = Some(v);
        self
    }
    pub fn title(mut self, v: impl Into<String>) -> Self {
        self.spec.title = v.into();
        self
    }
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.spec.description = Some(v.into());
        self
    }
    pub fn tone(mut self, v: AlertDialogTone) -> Self {
        self.spec.tone = v;
        self
    }
    pub fn confirm_label(mut self, v: impl Into<String>) -> Self {
        self.spec.confirm_label = v.into();
        self
    }
    pub fn cancel_label(mut self, v: impl Into<String>) -> Self {
        self.spec.cancel_label = v.into();
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn size(mut self, v: ControlSize) -> Self {
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

    // ── Component-level builders ──────────────────────────────
    /// Item-detail row (`itemLabel`/`itemValue`) — Svelte lines 140-144.
    pub fn item_detail(mut self, label: impl Into<String>, value: impl Into<String>) -> Self {
        self.item_label = Some(label.into());
        self.item_value = Some(value.into());
        self
    }

    /// Body content rendered between the item-detail row and the actions
    /// (matches the Svelte default slot).
    pub fn with_content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    /// Drives the working state. While true: confirm shows `working_label`,
    /// both buttons are disabled, and escape/backdrop/close dismissal is
    /// suppressed (matches Svelte `working`).
    pub fn working(mut self, v: bool) -> Self {
        self.working = v;
        self
    }

    /// Label shown on the confirm button while `working` is true.
    pub fn working_label(mut self, v: impl Into<String>) -> Self {
        self.working_label = v.into();
        self
    }

    pub fn on_confirm(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_confirm = Some(Rc::new(handler));
        self
    }

    pub fn on_cancel(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_cancel = Some(Rc::new(handler));
        self
    }

    /// Svelte: `confirmTone = tone === "danger" ? "danger" : "default"`.
    fn confirm_button_tone(&self) -> ButtonTone {
        match self.spec.tone {
            AlertDialogTone::Danger => ButtonTone::Danger,
            AlertDialogTone::Warning => ButtonTone::Default,
        }
    }
}

impl IntoElement for AlertDialog {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let is_open = self.spec.open.unwrap_or(false);
        if !is_open {
            return div().into_any_element();
        }

        let theme = &self.theme;
        let size = self.spec.size;
        let size_role = self.spec.size_role;
        let density = self.spec.density;
        let working = self.working;
        let confirm_tone = self.confirm_button_tone();

        // ── Cancel button (ghost) — Svelte AlertDialog.svelte:119-127 ──
        let mut cancel_btn = Button::new(theme)
            .with_id("alert-dialog-cancel")
            .variant(ButtonVariant::Ghost)
            .size(size)
            .size_role(size_role)
            .density(density)
            .label(self.spec.cancel_label.clone())
            .disabled(working);
        if let Some(handler) = self.on_cancel.clone() {
            cancel_btn = cancel_btn.on_click(move |_event, window, cx| handler(window, cx));
        }

        // ── Confirm button (primary, tone-driven) — Svelte :128-137 ──
        let confirm_text = if working {
            self.working_label.clone()
        } else {
            self.spec.confirm_label.clone()
        };
        let mut confirm_btn = Button::new(theme)
            .with_id("alert-dialog-confirm")
            .variant(ButtonVariant::Primary)
            .tone(confirm_tone)
            .size(size)
            .size_role(size_role)
            .density(density)
            .label(confirm_text)
            .disabled(working);
        if let Some(handler) = self.on_confirm.clone() {
            confirm_btn = confirm_btn.on_click(move |_event, window, cx| handler(window, cx));
        }

        let actions = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .child(cancel_btn)
            .child(confirm_btn);

        // ── Body content: item-detail row + optional children slot ──
        let body: Option<AnyElement> = {
            let item_detail = match (self.item_label.as_ref(), self.item_value.as_ref()) {
                (Some(label), Some(value)) => Some((label.clone(), value.clone())),
                _ => None,
            };
            if item_detail.is_none() && self.content.is_none() {
                None
            } else {
                let text_secondary =
                    crate::theme_ext::resolve_color(theme, self.spec.description_color_token());
                let text_primary =
                    crate::theme_ext::resolve_color(theme, self.spec.title_color_token());
                let body_size = crate::theme_ext::resolve_px(theme, "typography.body.size");
                let stack_md = crate::theme_ext::resolve_px(theme, self.spec.content_gap_token());
                let mut col = div().flex().flex_col().gap(stack_md);
                if let Some((label, value)) = item_detail {
                    // Svelte: <strong>{label}:</strong> {value}, text-secondary body.
                    col = col.child(
                        div()
                            .text_size(body_size)
                            .text_color(text_secondary)
                            .child(
                                div()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(text_primary)
                                    .child(format!("{label}:")),
                            )
                            .child(value),
                    );
                }
                if let Some(content) = self.content {
                    col = col.child(content);
                }
                Some(col.into_any_element())
            }
        };

        // ── Compose the real Dialog primitive for the surface/overlay ──
        // Svelte: role="alertdialog", width="sm", dismiss/close gated by working.
        let mut dialog = Dialog::new(theme)
            .role(DialogKind::AlertDialog)
            .width(DialogWidth::Sm)
            .title(self.spec.title.clone())
            .size(size)
            .with_size_role(size_role)
            .with_density(density)
            .dismiss_on_escape(!working)
            .dismiss_on_backdrop(!working)
            .show_close_button(!working)
            .with_actions(actions);

        if let Some(ref desc) = self.spec.description {
            dialog = dialog.description(desc.clone());
        }
        if let Some(label) = self.spec.aria_label.clone() {
            dialog = dialog.aria_label(label);
        }
        if let Some(body) = body {
            dialog = dialog.with_content(body);
        }

        // Dialog dismissal (escape/backdrop/close) routes to cancel, matching
        // Svelte `handleDialogOpenChange` → onCancel when not working.
        if let Some(handler) = self.on_cancel.clone() {
            dialog = dialog.on_open_change(move |next_open, window, cx| {
                if !next_open {
                    handler(window, cx);
                }
            });
        }

        dialog.open(true).into_any_element()
    }
}
