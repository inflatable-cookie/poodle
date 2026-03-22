//! Menubar — real GPUI component backed by MenubarSpec.
//!
//! Contract: list chrome (border/radius/bg/padding), trigger font 0.75rem/600,
//! trigger padding 0 0.75rem, min-height 2rem, hover accent 14%.

use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{MenuSpec, MenubarEntry, MenubarSpec};

use super::menu::Menu;
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_radius};

/// A real GPUI menubar component backed by `MenubarSpec`.
pub struct Menubar {
    spec: MenubarSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    on_select: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Menubar {
    type Target = MenubarSpec;
    fn deref(&self) -> &MenubarSpec { &self.spec }
}

impl Menubar {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: MenubarSpec::default(), theme: theme.clone(), id_prefix: String::new(), on_select: None }
    }

    pub fn from_spec(spec: MenubarSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "pug-menubar".to_string(),
            on_select: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn items(mut self, v: Vec<MenubarEntry>) -> Self { self.spec.items = v; self }
    pub fn value(mut self, v: impl Into<String>) -> Self { self.spec.value = Some(v.into()); self }
    pub fn default_value(mut self, v: impl Into<String>) -> Self { self.spec.default_value = Some(v.into()); self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }

    pub fn with_id(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    pub fn on_select(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Box::new(handler));
        self
    }
}

impl IntoElement for Menubar {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;

        let accent = resolve_color(theme, "semantic.color.accent.base");
        let panel = resolve_color(theme, "semantic.color.background.panel");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let border_subtle = resolve_color(theme, self.spec.list_border_token());
        let list_radius = resolve_radius(theme, self.spec.list_radius_token());
        let control_radius = resolve_radius(theme, "semantic.radius.control");
        let disabled_opacity = resolve_opacity(theme, self.spec.disabled_opacity_token());
        let focus_ring = resolve_color(theme, "semantic.color.accent.focusRing");
        let gap = theme.resolve_space(self.spec.trigger_gap_token());

        // Contract: list border 72% border-subtle, bg 96% panel
        let list_border = color_mix(border_subtle, panel, 0.72);
        let list_bg = color_mix(panel, gpui::transparent_black(), 0.96);
        // Contract: trigger hover 14% accent
        let trigger_hover = color_mix(accent, panel, 0.14);

        let current_value = self.spec.current_value().map(|s| s.to_string());

        // Contract: list has border/radius/bg/padding
        let mut trigger_row = div()
            .flex()
            .items_center()
            .gap(px(2.0)) // Contract: gap 0.125rem
            .p(px(3.0)) // Contract: padding 0.1875rem
            .border_1()
            .border_color(list_border)
            .rounded(list_radius)
            .bg(list_bg);

        for entry in &self.spec.items {
            let is_active = current_value.as_deref() == Some(entry.value.as_str());
            let is_disabled = entry.is_disabled;
            let item_id = SharedString::from(format!("{}-{}", self.id_prefix, entry.value));

            // Contract: trigger min-height 2rem, padding 0 0.75rem, font 0.75rem/600
            let mut trigger = div()
                .id(item_id)
                .flex()
                .items_center()
                .min_h(px(32.0)) // 2rem
                .px(px(12.0)) // 0.75rem
                .rounded(control_radius)
                .text_size(px(12.0)) // 0.75rem
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(text_primary);

            trigger = trigger.focus(move |s| s.border_color(focus_ring));

            if is_active {
                trigger = trigger.bg(trigger_hover);
            }

            if is_disabled {
                trigger = trigger
                    .opacity(disabled_opacity)
                    .cursor(CursorStyle::OperationNotAllowed);
            } else {
                trigger = trigger
                    .cursor_pointer()
                    .hover(|s| s.bg(trigger_hover));
            }

            trigger = trigger.child(entry.label.clone());
            trigger_row = trigger_row.child(trigger);
        }

        let mut wrapper = div().flex().flex_col();
        wrapper = wrapper.child(trigger_row);

        // Show dropdown menu for active entry
        if let Some(entry) = self.spec.current_menu() {
            if !entry.items.is_empty() {
                let menu_spec = MenuSpec::new(entry.items.clone());
                wrapper = wrapper.child(
                    Menu::from_spec(menu_spec, &self.theme)
                        .with_id(format!("{}-dropdown", self.id_prefix)),
                );
            }
        }

        wrapper.into_any_element()
    }
}
