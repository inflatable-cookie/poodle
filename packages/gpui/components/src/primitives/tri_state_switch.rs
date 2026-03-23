//! TriStateSwitch — real GPUI component backed by TriStateSwitchSpec.
//!
//! Renders as a 3-segment radiogroup: Excluded | Default | Included.

use gpui::*;
use flint_gpui::GpuiThemeProvider;
use flint_primitives::{CheckState, TriStateSwitchSpec};

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px};

/// A real GPUI tri-state switch component backed by `TriStateSwitchSpec`.
///
/// Renders as a 3-segment radiogroup (Excluded / Default / Included) instead
/// of a sliding switch.
pub struct TriStateSwitch {
    spec: TriStateSwitchSpec,
    theme: GpuiThemeProvider,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for TriStateSwitch {
    type Target = TriStateSwitchSpec;
    fn deref(&self) -> &TriStateSwitchSpec { &self.spec }
}

impl TriStateSwitch {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: TriStateSwitchSpec::new(), theme: theme.clone(), on_click: None }
    }

    pub fn from_spec(spec: TriStateSwitchSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_click: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn state(mut self, v: CheckState) -> Self { self.spec.state = v; self }
    pub fn label(mut self, v: impl Into<String>) -> Self { self.spec.label = Some(v.into()); self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl IntoElement for TriStateSwitch {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let inline_gap = resolve_px(theme, "semantic.space.inline.sm");

        // Colors
        let border_color = resolve_color(theme, "semantic.color.border.default");
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let danger = resolve_color(theme, "semantic.color.status.danger");
        let success = resolve_color(theme, "semantic.color.status.success");
        let elevated_bg = resolve_color(theme, "semantic.color.background.elevated");
        let focus_ring = resolve_color(theme, "semantic.color.accent.focusRing");
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");

        // Active segment background colors (with subtle opacity for danger/success)
        let danger_bg = Hsla { a: 0.15, ..danger };
        let success_bg = Hsla { a: 0.15, ..success };

        // Determine which segment is active
        let is_excluded = matches!(spec.state, CheckState::Unchecked);
        let is_default = matches!(spec.state, CheckState::Mixed);
        let is_included = matches!(spec.state, CheckState::Checked);

        // ── Build individual segments ──────────────────────────

        let segment_base = |label: &'static str| {
            div()
                .flex_1()
                .flex()
                .items_center()
                .justify_center()
                .py(px(4.0))
                .px(px(8.0))
                .text_size(px(12.0))
                .font_weight(FontWeight::MEDIUM)
                .child(label)
        };

        // Excluded segment (left)
        let excluded_seg = if is_excluded {
            segment_base("\u{2715}") // ✕
                .bg(danger_bg)
                .text_color(danger)
        } else {
            segment_base("\u{2715}")
                .bg(gpui::transparent_black())
                .text_color(text_secondary)
        };

        // Default segment (center)
        let default_seg = if is_default {
            segment_base("\u{2014}") // —
                .bg(elevated_bg)
                .text_color(text_primary)
        } else {
            segment_base("\u{2014}")
                .bg(gpui::transparent_black())
                .text_color(text_secondary)
        };

        // Included segment (right)
        let included_seg = if is_included {
            segment_base("\u{2713}") // ✓
                .bg(success_bg)
                .text_color(success)
        } else {
            segment_base("\u{2713}")
                .bg(gpui::transparent_black())
                .text_color(text_secondary)
        };

        // ── Container (pill-shaped segmented control) ──────────

        let switch_id = SharedString::from("flint-tri-state-switch");

        let mut container = div()
            .id(switch_id)
            .focusable()
            .flex()
            .flex_row()
            .rounded(px(10.0))
            .border_1()
            .border_color(border_color)
            .bg(surface_bg)
            .overflow_hidden()
            .child(excluded_seg)
            .child(default_seg)
            .child(included_seg);

        // Focus ring
        container = container.focus(move |s| s.border_color(focus_ring));

        // Disabled / interactive handling
        if !spec.is_disabled {
            container = container.cursor_pointer();
            if let Some(handler) = self.on_click {
                let handler = std::rc::Rc::new(handler);
                let click_handler = handler.clone();
                container =
                    container.on_click(move |event, window, cx| click_handler(event, window, cx));
                let key_handler = handler.clone();
                container = container.on_key_down(move |event: &KeyDownEvent, window, cx| {
                    if event.keystroke.key == "space" || event.keystroke.key == "enter" {
                        key_handler(&ClickEvent::default(), window, cx);
                    }
                });
            }
        } else {
            container = container
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        // ── Outer wrapper with optional label ──────────────────

        let mut el = div()
            .flex()
            .items_center()
            .gap(inline_gap)
            .child(container);

        if let Some(ref label) = spec.label {
            el = el.child(
                div()
                    .text_size(px(14.0))
                    .text_color(text_primary)
                    .child(label.clone()),
            );
        }

        el.into_any_element()
    }
}
