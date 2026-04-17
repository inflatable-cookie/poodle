//! NavigationMenu — real GPUI component backed by NavigationMenuSpec.
//!
//! Contract: pill-style bordered triggers (not underline tabs),
//! trigger font 0.75rem/600, min-height control-height - 0.125rem,
//! viewport with border/radius/bg/shadow.

use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ControlDensity, ControlSize, NavigationMenuEntry, NavigationMenuSpec, SemanticControlSizeRole,
};

use crate::presentation::{
    panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_height_offset_rem,
};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI navigation menu component backed by `NavigationMenuSpec`.
pub struct NavigationMenu {
    spec: NavigationMenuSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for NavigationMenu {
    type Target = NavigationMenuSpec;
    fn deref(&self) -> &NavigationMenuSpec {
        &self.spec
    }
}

impl NavigationMenu {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: NavigationMenuSpec::default(),
            theme: theme.clone(),
            id_prefix: String::new(),
            on_change: None,
        }
    }

    pub fn from_spec(spec: NavigationMenuSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "poodle-nav".to_string(),
            on_change: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn items(mut self, v: Vec<NavigationMenuEntry>) -> Self {
        self.spec.items = v;
        self
    }
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.spec.value = Some(v.into());
        self
    }
    pub fn default_value(mut self, v: impl Into<String>) -> Self {
        self.spec.default_value = Some(v.into());
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

    pub fn with_id(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl IntoElement for NavigationMenu {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let effective_size = resolve_semantic_size(self.spec.size, self.spec.size_role);
        // Svelte: nav trigger font is compact-scale (0.75rem default, not control size_font_rem)
        let trigger_font = px(rem_to_px(match effective_size {
            ControlSize::Xs => 0.6875,
            ControlSize::Sm => 0.75,
            ControlSize::Md => 0.75,
            ControlSize::Lg => 0.8125,
            ControlSize::Xl => 0.875,
        }));
        let _base_height = resolve_px(theme, "size.control.height");
        let trigger_height_offset = px(rem_to_px(size_height_offset_rem(effective_size)));
        // Svelte: compact=0.5rem, default=space-control-x (0.75rem), comfortable=0.75rem (same as default)
        let trigger_pad_x = px(rem_to_px(match self.spec.density {
            ControlDensity::Compact => 0.5,
            ControlDensity::Default | ControlDensity::Comfortable => 0.75,
        }));
        let density_panel_x = px(rem_to_px(panel_space_x_rem(self.spec.density)));
        let density_panel_y = px(rem_to_px(panel_space_y_rem(self.spec.density)));

        let control_height = resolve_px(theme, "size.control.height");
        let trigger_radius = resolve_radius(theme, self.spec.trigger_radius_token());
        let viewport_radius = resolve_radius(theme, self.spec.viewport_radius_token());
        let viewport_gap = theme.resolve_space(self.spec.viewport_gap_token());
        let accent = resolve_color(theme, "color.accent.base");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let surface = resolve_color(theme, "color.background.surface");
        let panel = resolve_color(theme, "color.background.panel");
        let border_subtle = resolve_color(theme, "color.border.subtle");
        let border_default = resolve_color(theme, "color.border.default");
        let disabled_opacity = resolve_opacity(theme, self.spec.disabled_opacity_token());
        let body_size = resolve_px(theme, "typography.body.size");
        let focus_ring = resolve_color(theme, "color.accent.focusRing");
        let gap_sm = resolve_px(theme, "space.inline.sm");

        // Svelte: trigger bg = color-mix(surface 88%, transparent)
        let trigger_bg = Hsla { a: surface.a * 0.88, ..surface };
        // Svelte: trigger border = color-mix(border-subtle 72%, transparent)
        let trigger_border = Hsla { a: border_subtle.a * 0.72, ..border_subtle };
        // Svelte: active bg = color-mix(accent 16%, transparent)
        let active_bg = Hsla { a: accent.a * 0.16, ..accent };
        // Svelte: active border = color-mix(accent 42%, border-default)
        let active_border = color_mix(accent, border_default, 0.42);
        // Svelte: hover bg = color-mix(accent 12%, transparent)
        let hover_bg = Hsla { a: accent.a * 0.12, ..accent };
        // Svelte: viewport border = color-mix(border-subtle 74%, transparent)
        let viewport_border = Hsla { a: border_subtle.a * 0.74, ..border_subtle };
        // Svelte: viewport bg = color-mix(panel 96%, transparent)
        let viewport_bg = Hsla { a: panel.a * 0.96, ..panel };

        // Contract: trigger min-height = control-height + offset - 0.125rem
        let trigger_height = control_height + trigger_height_offset - px(2.0);

        let current_value = self.spec.current_value().map(|s| s.to_string());

        // Wrap on_change in Rc for sharing across item clicks
        let on_change_rc: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App)>> =
            self.on_change.map(|h| std::rc::Rc::from(h));

        // Contract: list = inline-flex, flex-wrap, gap 0.25rem
        let mut nav_row = div().flex().flex_wrap().items_center().gap(gap_sm);

        for item in &self.spec.items {
            let is_active = current_value.as_deref() == Some(item.value.as_str());
            let is_disabled = item.is_disabled;
            let item_id = SharedString::from(format!("{}-{}", self.id_prefix, item.value));

            // Contract: pill-style trigger with border, padding per density
            let mut trigger = div()
                .id(item_id)
                .focusable()
                .flex()
                .items_center()
                .min_h(trigger_height)
                .px(trigger_pad_x)
                .border_1()
                .rounded(trigger_radius)
                // Contract: font per effective size
                .text_size(trigger_font)
                .font_weight(FontWeight::SEMIBOLD);

            if is_active {
                trigger = trigger
                    .bg(active_bg)
                    .border_color(active_border)
                    .text_color(text_primary);
            } else {
                trigger = trigger
                    .bg(trigger_bg)
                    .border_color(trigger_border)
                    .text_color(text_primary);
            }

            trigger = trigger.focus(move |s| {
                s.border_color(focus_ring)
                    .shadow(crate::theme_ext::focus_ring_shadow(focus_ring))
            });

            if is_disabled {
                trigger = trigger
                    .opacity(disabled_opacity)
                    .cursor(CursorStyle::OperationNotAllowed);
            } else {
                trigger = trigger.cursor_pointer().hover(|s| s.bg(hover_bg));

                if let Some(ref handler) = on_change_rc {
                    let handler = handler.clone();
                    let val = item.value.clone();
                    trigger = trigger.on_click(move |_event, window, cx| {
                        handler(&val, window, cx);
                    });
                }
            }

            trigger = trigger.child(item.label.clone());
            nav_row = nav_row.child(trigger);
        }

        // Contract: root is grid with gap 0.5rem
        let mut wrapper = div().flex().flex_col().gap(px(viewport_gap));
        wrapper = wrapper.child(nav_row);

        // Viewport: show description for active item with border/radius/bg/shadow
        if let Some(ref current) = current_value {
            if let Some(item) = self.spec.items.iter().find(|i| &i.value == current) {
                if let Some(ref desc) = item.description {
                    wrapper = wrapper.child(
                        div()
                            .px(density_panel_x)
                            .py(density_panel_y)
                            .border_1()
                            .border_color(viewport_border)
                            .rounded(viewport_radius)
                            .bg(viewport_bg)
                            // Contract: elevation-popover shadow
                            .shadow(vec![
                                gpui::BoxShadow {
                                    color: hsla(0.0, 0.0, 0.0, 0.10),
                                    offset: point(px(0.0), px(4.0)),
                                    blur_radius: px(16.0),
                                    spread_radius: px(0.0),
                                },
                                gpui::BoxShadow {
                                    color: hsla(0.0, 0.0, 0.0, 0.06),
                                    offset: point(px(0.0), px(1.0)),
                                    blur_radius: px(4.0),
                                    spread_radius: px(0.0),
                                },
                            ])
                            .text_size(body_size)
                            .text_color(text_secondary)
                            .child(desc.clone()),
                    );
                }
            }
        }

        wrapper.into_any_element()
    }
}
