//! Dialog — real GPUI component backed by DialogSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{DialogKind, DialogSpec};

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI dialog component backed by `DialogSpec`.
///
/// Renders the dialog surface (elevated card with title/description).
/// The parent is responsible for conditionally rendering based on `spec.current_open()`.
pub struct Dialog {
    spec: DialogSpec,
    theme: GpuiThemeProvider,
    /// Actions slot — typically buttons rendered by the parent.
    actions: Option<AnyElement>,
    /// Content slot — body content between description and actions.
    content: Option<AnyElement>,
    /// Called when the dialog should close (Escape key, backdrop click).
    on_close: Option<std::rc::Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Dialog {
    type Target = DialogSpec;
    fn deref(&self) -> &DialogSpec { &self.spec }
}

impl Dialog {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: DialogSpec::new(), theme: theme.clone(), actions: None, content: None, on_close: None }
    }

    pub fn from_spec(spec: DialogSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            actions: None,
            content: None,
            on_close: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn open(mut self, v: bool) -> Self { self.spec.open = Some(v); self }
    pub fn default_open(mut self, v: bool) -> Self { self.spec.default_open = v; self }
    pub fn title(mut self, v: impl Into<String>) -> Self { self.spec.title = Some(v.into()); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.spec.description = Some(v.into()); self }
    pub fn kind(mut self, v: DialogKind) -> Self { self.spec.kind = v; self }
    pub fn dismiss_on_escape(mut self, v: bool) -> Self { self.spec.dismiss_on_escape = v; self }
    pub fn dismiss_on_backdrop(mut self, v: bool) -> Self { self.spec.dismiss_on_backdrop = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }


    /// Called when the dialog should close (Escape, backdrop click).
    pub fn on_close(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(std::rc::Rc::new(handler));
        self
    }

    /// Add body content between the description and actions.
    pub fn with_content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    /// Add an actions row (e.g., Cancel + Confirm buttons).
    pub fn with_actions(mut self, actions: impl IntoElement) -> Self {
        self.actions = Some(actions.into_any_element());
        self
    }
}

impl IntoElement for Dialog {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let actions_gap = resolve_px(theme, "semantic.space.inline.sm");
        let panel_x = resolve_px(theme, "semantic.space.panel.x");
        let panel_y = resolve_px(theme, "semantic.space.panel.y");

        let elevated_bg = resolve_color(theme, "semantic.color.background.elevated");
        let border_default = resolve_color(theme, "semantic.color.border.default");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let radius = resolve_radius(theme, "semantic.radius.surface");
        let body_size = resolve_px(theme, "semantic.typography.body.size");
        let heading_size = resolve_px(theme, "semantic.typography.heading.size");

        // Matches Svelte treatment-surface-elevated values:
        //   fill: color-mix(elevated 94%, transparent)
        //   border: color-mix(border-default 22%, transparent)
        let bg = Hsla { a: elevated_bg.a * 0.94, ..elevated_bg };
        let border = Hsla { a: border_default.a * 0.22, ..border_default };

        let stack_lg = resolve_px(theme, "semantic.space.stack.lg");

        let mut dialog = div()
            .id("poodle-dialog")
            .focusable()
            .px(panel_x)
            .py(panel_y)
            .rounded(radius)
            .bg(bg)
            .border_1()
            .border_color(border)
            // Svelte: elevation-dialog shadow
            .shadow(vec![
                gpui::BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.12),
                    offset: point(px(0.0), px(8.0)),
                    blur_radius: px(24.0),
                    spread_radius: px(0.0),
                },
                gpui::BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.08),
                    offset: point(px(0.0), px(2.0)),
                    blur_radius: px(8.0),
                    spread_radius: px(0.0),
                },
            ])
            .flex()
            .flex_col()
            // Svelte: width min(34rem, 100%) = 544px
            .w(px(544.0))
            .max_w_full()
            // Svelte: gap 0.375rem (6px)
            .gap(px(6.0))
            .occlude();

        // Svelte: title font 1rem (16px), weight 600
        if let Some(ref title) = spec.title {
            dialog = dialog.child(
                div()
                    .text_size(heading_size)
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_primary)
                    .child(title.clone()),
            );
        }

        // Description: 0.875rem (14px)
        if let Some(ref description) = spec.description {
            dialog = dialog.child(
                div()
                    .text_size(body_size)
                    .text_color(text_secondary)
                    .child(description.clone()),
            );
        }

        // Content slot — body content between description and actions
        if let Some(content) = self.content {
            dialog = dialog.child(content);
        }

        // Actions slot — Svelte: margin-top stack-lg, flex-wrap, justify-end
        if let Some(actions) = self.actions {
            dialog = dialog.child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(actions_gap)
                    .justify_end()
                    .mt(stack_lg)
                    .child(actions),
            );
        }

        // Escape key on dialog surface
        if spec.dismiss_on_escape {
            if let Some(ref handler) = self.on_close {
                let esc_handler = handler.clone();
                dialog = dialog.on_key_down(move |event: &KeyDownEvent, window, cx| {
                    if event.keystroke.key == "escape" {
                        esc_handler(window, cx);
                    }
                });
            }
        }

        // Backdrop overlay — full-viewport scrim with centered dialog
        let mut backdrop = div()
            .id("poodle-dialog-backdrop")
            .absolute()
            .inset_0()
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            .flex()
            .items_center()
            .justify_center()
            .occlude()
            .child(dialog);

        // Backdrop click to dismiss
        if spec.dismiss_on_backdrop {
            if let Some(ref handler) = self.on_close {
                let click_handler = handler.clone();
                backdrop = backdrop.on_click(move |_event, window, cx| {
                    click_handler(window, cx);
                });
            }
        }

        backdrop.into_any_element()
    }
}
