//! CollapseToggle — real GPUI component backed by CollapseToggleSpec.
//!
//! A standalone collapse/expand toggle button with a directional chevron icon.
//! Implements the collapse-toggle contract exactly, resolving all tokens
//! through CollapseToggleSpec + GpuiThemeProvider.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{CollapseDirection, CollapseToggleSpec, IconSize, IconSpec};

use super::icon::Icon;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI collapse toggle component backed by `CollapseToggleSpec`.
pub struct CollapseToggle {
    spec: CollapseToggleSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_toggle: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for CollapseToggle {
    type Target = CollapseToggleSpec;
    fn deref(&self) -> &CollapseToggleSpec { &self.spec }
}

impl CollapseToggle {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: CollapseToggleSpec::new(), theme: theme.clone(), id_suffix: None, on_toggle: None }
    }

    pub fn from_spec(spec: CollapseToggleSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_toggle: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn collapsed(mut self, v: bool) -> Self { self.spec.is_collapsed = v; self }
    pub fn direction(mut self, v: CollapseDirection) -> Self { self.spec.direction = v; self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }


    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    pub fn on_toggle(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Box::new(handler));
        self
    }
}

impl IntoElement for CollapseToggle {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Resolve tokens ───────────────────────────────────────
        let text_color = resolve_color(theme, spec.text_color_token());
        let hover_text = resolve_color(theme, spec.text_color_hover_token());
        let hover_fill = resolve_color(theme, spec.hover_fill_token());
        let radius = resolve_radius(theme, spec.radius_token());
        let icon_size = resolve_px(theme, spec.icon_size_token());

        let is_disabled = spec.is_disabled;

        // ── Determine icon ───────────────────────────────────────
        let icon_name = spec.effective_icon_name();

        // ── Build element ID ─────────────────────────────────────
        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("pug-collapse-toggle-{}", suffix)
        } else {
            format!("pug-collapse-toggle-{}", icon_name)
        };

        // ── Compact square button sized to icon + small padding ──
        // Contract: "compact button sized to icon", Svelte uses padding: 0.125rem (2px)
        let padding = px(2.0);
        let button_size = icon_size + padding * 2.0;

        // ── Build root element ───────────────────────────────────
        let mut el = div()
            .id(SharedString::from(id_str))
            .w(button_size)
            .h(button_size)
            .rounded(radius)
            .bg(gpui::transparent_black())
            .text_color(text_color)
            .flex()
            .items_center()
            .justify_center()
            .line_height(px(1.0));

        // Focus ring
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());
        el = el.focus(move |s| s.border_color(focus_ring));

        // ── Interactive states ───────────────────────────────────
        if is_disabled {
            let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
            el = el
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        } else {
            el = el
                .cursor_pointer()
                .hover(move |s| {
                    s.bg(hover_fill).text_color(hover_text)
                })
                .active(move |s| s.bg(hover_fill));
        }

        // ── Direction Icon (chevron) ─────────────────────────────
        el = el.child(
            Icon::from_spec(
                IconSpec::new(icon_name).with_size(IconSize::Sm),
                theme,
            )
            .with_color(text_color),
        );

        // ── Click handler ────────────────────────────────────────
        if let Some(handler) = self.on_toggle {
            if !is_disabled {
                el = el.on_click(move |event, window, cx| handler(event, window, cx));
            }
        }

        el.into_any_element()
    }
}
