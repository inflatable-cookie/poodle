//! ToastHost — real GPUI component backed by ToastHostSpec.
//!
//! Renders a fixed-position toast container at specified placement, using the
//! ToastStack component internally to display individual toast notifications.
//! Manages auto-dismiss timers and sticky tone semantics.

use std::rc::Rc;

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{Toast, ToastHostPlacement, ToastHostSpec, ToastPosition, ToastStackSpec, ToastTone};
use poodle_specs::{ControlDensity, ControlSize, SemanticControlSizeRole};

use super::ToastStack;

/// A real GPUI toast host component backed by `ToastHostSpec`.
///
/// Wraps a `ToastStack` and positions it at a chosen screen corner.
/// In GPUI, auto-dismiss timing is managed externally by the application;
/// this component provides the visual layout and placement.
pub struct ToastHost {
    spec: ToastHostSpec,
    theme: GpuiThemeProvider,
    toasts: Vec<Toast>,
    on_dismiss: Option<Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_action: Option<Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for ToastHost {
    type Target = ToastHostSpec;
    fn deref(&self) -> &ToastHostSpec { &self.spec }
}

impl ToastHost {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: ToastHostSpec::new(),
            theme: theme.clone(),
            toasts: Vec::new(),
            on_dismiss: None,
            on_action: None,
        }
    }

    pub fn from_spec(spec: ToastHostSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            toasts: Vec::new(),
            on_dismiss: None,
            on_action: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn placement(mut self, v: ToastHostPlacement) -> Self { self.spec.placement = v; self }
    pub fn auto_dismiss_ms(mut self, v: u32) -> Self { self.spec.auto_dismiss_ms = v; self }
    pub fn sticky_tones(mut self, v: Vec<ToastTone>) -> Self { self.spec.sticky_tones = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = v.into(); self }
    pub fn with_size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self { self.spec.size_role = v; self }
    pub fn with_density(mut self, v: ControlDensity) -> Self { self.spec.density = v; self }

    /// Set the list of toasts to display.
    pub fn toasts(mut self, v: Vec<Toast>) -> Self { self.toasts = v; self }

    /// Add a single toast to the display list.
    pub fn add_toast(mut self, toast: Toast) -> Self { self.toasts.push(toast); self }

    pub fn on_dismiss(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_dismiss = Some(Rc::new(handler));
        self
    }

    pub fn on_action(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_action = Some(Rc::new(handler));
        self
    }
}

impl IntoElement for ToastHost {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        // No toasts — render nothing
        if self.toasts.is_empty() {
            return div().into_any_element();
        }

        let padding = px(16.0); // 1rem
        let width = px(448.0);  // min(28rem, calc(100vw - 2rem))

        // ── Position the host container at the chosen corner ──────
        let mut container = div()
            .absolute()
            .w(width);

        match self.spec.placement {
            ToastHostPlacement::BottomEnd => {
                container = container.right(padding).bottom(padding);
            }
            ToastHostPlacement::BottomStart => {
                container = container.left(padding).bottom(padding);
            }
            ToastHostPlacement::TopEnd => {
                container = container.right(padding).top(padding);
            }
            ToastHostPlacement::TopStart => {
                container = container.left(padding).top(padding);
            }
        }

        // ── Map placement to ToastPosition for ToastStack ─────────
        let toast_position = match self.spec.placement {
            ToastHostPlacement::BottomEnd => ToastPosition::BottomRight,
            ToastHostPlacement::BottomStart => ToastPosition::BottomLeft,
            ToastHostPlacement::TopEnd => ToastPosition::TopRight,
            ToastHostPlacement::TopStart => ToastPosition::TopLeft,
        };

        // ── Build ToastStack spec ─────────────────────────────────
        let stack_spec = ToastStackSpec::new()
            .with_toasts(self.toasts)
            .with_position(toast_position)
            .with_size(self.spec.size)
            .with_size_role(self.spec.size_role)
            .with_density(self.spec.density);

        let mut stack = ToastStack::from_spec(stack_spec, &self.theme);

        if let Some(handler) = self.on_dismiss {
            let handler = Rc::clone(&handler);
            stack = stack.on_dismiss(move |id, window, cx| handler(id, window, cx));
        }

        if let Some(handler) = self.on_action {
            let handler = Rc::clone(&handler);
            stack = stack.on_action(move |id, window, cx| handler(id, window, cx));
        }

        container = container.child(stack);

        container.into_any_element()
    }
}
