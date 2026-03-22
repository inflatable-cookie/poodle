//! ToggleGroup — grouped toggle selection control.
//!
//! Contract: `docs/contracts/foundation/toggle-group.md`

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{ToggleGroupOption, ToggleGroupSelectionMode, ToggleGroupSpec};

use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

pub struct ToggleGroup {
    spec: ToggleGroupSpec,
    theme: GpuiThemeProvider,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for ToggleGroup {
    type Target = ToggleGroupSpec;
    fn deref(&self) -> &ToggleGroupSpec { &self.spec }
}

impl ToggleGroup {
    pub fn new(options: Vec<ToggleGroupOption>, theme: &GpuiThemeProvider) -> Self {
        Self { spec: ToggleGroupSpec::new(options), theme: theme.clone(), on_change: None }
    }

    pub fn from_spec(spec: ToggleGroupSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), on_change: None }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: Vec<String>) -> Self { self.spec.value = Some(v); self }
    pub fn default_value(mut self, v: Vec<String>) -> Self { self.spec.default_value = Some(v); self }
    pub fn selection_mode(mut self, v: ToggleGroupSelectionMode) -> Self { self.spec.selection_mode = v; self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }

    // ── GPUI-specific ─────────────────────────────────────────
    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl IntoElement for ToggleGroup {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let gap = resolve_px(theme, spec.item_gap_token());
        let radius = resolve_radius(theme, "semantic.radius.control");
        let accent = resolve_color(theme, "semantic.color.accent.base");
        let surface = resolve_color(theme, "semantic.color.background.surface");
        let elevated = resolve_color(theme, "semantic.color.background.elevated");
        let border_default = resolve_color(theme, "semantic.color.border.default");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());

        // Contract: selected = accent 22% tinted bg, accent 42% border
        let selected_fill = color_mix(accent, surface, 0.22);
        let selected_border = color_mix(accent, border_default, 0.42);

        let mut el = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .gap(gap);

        for option in &spec.options {
            let is_selected = spec.is_selected(&option.value);
            let item_disabled = option.is_disabled || spec.is_disabled;

            let (fill, border_color, text_color) = if is_selected {
                (selected_fill, selected_border, text_primary)
            } else {
                (surface, border_default, text_primary)
            };

            let hover_fill = color_mix(fill, elevated, 0.84);

            let mut item = div()
                .id(SharedString::from(format!("tg-{}", option.value)))
                .px(px(12.0))
                .py(px(4.0))
                .rounded(radius)
                .bg(fill)
                .border_1().border_color(border_color)
                .text_color(text_color)
                .text_size(px(12.0))
                .font_weight(FontWeight::SEMIBOLD)
                .flex().items_center().justify_center()
                .focus(move |s| s.border_color(focus_ring));

            if item_disabled {
                let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
                item = item.opacity(opacity).cursor(CursorStyle::OperationNotAllowed);
            } else {
                item = item.cursor_pointer().hover(move |s| s.bg(hover_fill));
            }

            item = item.child(option.label.clone());
            el = el.child(item);
        }

        if spec.is_disabled {
            let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
            el = el.opacity(opacity);
        }

        el.into_any_element()
    }
}
