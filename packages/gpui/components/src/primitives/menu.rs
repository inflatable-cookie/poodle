//! Menu — real GPUI component backed by MenuSpec.
//!
//! Contract: overlay min-width 14rem, radius-surface, 98% elevated/panel bg,
//! item padding 0.375rem 0.5rem, min-height 2rem, radius control-0.125rem,
//! hover accent 16%, disabled cursor not-allowed.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{IconSize, IconSpec, MenuEntry, MenuItemKind, MenuSpec, OverlayPlacement};

use super::icon::Icon;

use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_radius};

/// A real GPUI menu component backed by `MenuSpec`.
pub struct Menu {
    spec: MenuSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    selected_value: Option<String>,
}

impl std::ops::Deref for Menu {
    type Target = MenuSpec;
    fn deref(&self) -> &MenuSpec { &self.spec }
}

impl Menu {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: MenuSpec::default(), theme: theme.clone(), id_prefix: String::new(), selected_value: None }
    }

    pub fn from_spec(spec: MenuSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "pug-menu".to_string(),
            selected_value: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn items(mut self, v: Vec<MenuEntry>) -> Self { self.spec.items = v; self }
    pub fn open(mut self, v: bool) -> Self { self.spec.open = Some(v); self }
    pub fn default_open(mut self, v: bool) -> Self { self.spec.default_open = v; self }
    pub fn placement(mut self, v: OverlayPlacement) -> Self { self.spec.placement = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }

    pub fn with_id(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    /// Highlight a specific item as selected/active.
    pub fn with_selected(mut self, value: impl Into<String>) -> Self {
        self.selected_value = Some(value.into());
        self
    }
}

impl IntoElement for Menu {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;

        let overlay_radius = resolve_radius(theme, self.spec.overlay_radius_token());
        let control_radius = resolve_radius(theme, "semantic.radius.control");
        // Contract: item radius = control - 0.125rem
        let item_radius = control_radius - px(2.0);

        let elevated = resolve_color(theme, self.spec.surface_fill_token());
        let panel = resolve_color(theme, "semantic.color.background.panel");
        // Contract: 98% elevated, 2% panel
        let surface_bg = color_mix(elevated, panel, 0.98);
        let border_raw = resolve_color(theme, self.spec.overlay_border_token());
        // Contract: 72% border-default
        let border = color_mix(border_raw, panel, 0.72);
        let text_primary = resolve_color(theme, self.spec.item_text_token());
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let accent = resolve_color(theme, self.spec.item_highlight_token());
        // Contract: 16% accent for hover
        let item_hover = color_mix(accent, panel, 0.16);
        let separator_raw = resolve_color(theme, self.spec.separator_color_token());
        // Contract: 72% border-subtle
        let separator_color = color_mix(separator_raw, panel, 0.72);
        let disabled_opacity = resolve_opacity(theme, self.spec.disabled_opacity_token());
        let focus_ring = resolve_color(theme, "semantic.color.accent.focusRing");

        // Contract: min-width 14rem, padding 0.25rem
        let mut menu = div()
            .min_w(px(224.0)) // 14rem
            .rounded(overlay_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(border)
            .shadow_md()
            .p(px(4.0)); // 0.25rem

        for item in &self.spec.items {
            // Separator
            if item.kind == MenuItemKind::Separator {
                // Contract: height 0.0625rem, margin 0.25rem 0
                menu = menu.child(
                    div()
                        .w_full()
                        .h(px(1.0))
                        .my(px(4.0)) // 0.25rem
                        .bg(separator_color),
                );
                continue;
            }

            let is_active = self
                .selected_value
                .as_deref()
                .map(|s| s == item.value)
                .unwrap_or(false);
            let is_disabled = item.is_disabled;
            let is_checked = item.is_checked;
            let item_id = SharedString::from(format!("{}-{}", self.id_prefix, item.value));

            // Contract: item min-height 2rem, padding 0.375rem 0.5rem
            let mut row = div()
                .id(item_id)
                .w_full()
                .min_h(px(32.0)) // 2rem
                .px(px(8.0))  // 0.5rem
                .py(px(6.0)) // 0.375rem
                .rounded(item_radius)
                // Contract: font 0.875rem
                .text_size(px(14.0))
                .flex()
                .items_center()
                .justify_between();

            row = row.focus(move |s| s.border_color(focus_ring));

            if is_active {
                row = row
                    .bg(item_hover)
                    .text_color(accent);
            } else if is_disabled {
                row = row
                    .text_color(text_secondary)
                    .opacity(disabled_opacity)
                    .cursor(CursorStyle::OperationNotAllowed);
            } else {
                row = row
                    .text_color(text_primary)
                    .cursor_pointer()
                    .hover(|s| s.bg(item_hover));
            }

            // Label + checkbox indicator
            let mut label_row = div().flex().items_center().gap(px(6.0));
            if is_checked {
                label_row = label_row.child(
                    Icon::from_spec(
                        IconSpec::new("check").with_size(IconSize::Sm),
                        theme,
                    )
                    .with_color(accent),
                );
            }
            label_row = label_row.child(item.label.clone());
            row = row.child(label_row);

            // Shortcut hint — Contract: code font 0.6875rem
            if let Some(ref shortcut) = item.shortcut_label {
                row = row.child(
                    div()
                        .text_size(px(11.0)) // 0.6875rem
                        .text_color(text_secondary)
                        .child(shortcut.clone()),
                );
            }

            menu = menu.child(row);
        }

        menu.into_any_element()
    }
}
