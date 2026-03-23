//! Drawer — real GPUI component backed by DrawerSpec.

use gpui::*;
use flint_gpui::GpuiThemeProvider;
use flint_primitives::{DrawerEdge, DrawerSpec};

use crate::theme_ext::{color_mix, resolve_color, resolve_px};

/// A real GPUI drawer component backed by `DrawerSpec`.
///
/// Renders a side panel. The parent controls the `open` state.
pub struct Drawer {
    spec: DrawerSpec,
    theme: GpuiThemeProvider,
    /// The content to show inside the drawer panel.
    content: Option<AnyElement>,
    /// The main area content (shown next to the drawer).
    main_content: Option<AnyElement>,
    /// Called when the drawer should close (Escape key, backdrop click).
    on_close: Option<std::rc::Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Drawer {
    type Target = DrawerSpec;
    fn deref(&self) -> &DrawerSpec { &self.spec }
}

impl Drawer {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: DrawerSpec::new(), theme: theme.clone(), content: None, main_content: None, on_close: None }
    }

    pub fn from_spec(spec: DrawerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            content: None,
            main_content: None,
            on_close: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn open(mut self, v: bool) -> Self { self.spec.open = Some(v); self }
    pub fn default_open(mut self, v: bool) -> Self { self.spec.default_open = v; self }
    pub fn title(mut self, v: impl Into<String>) -> Self { self.spec.title = Some(v.into()); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.spec.description = Some(v.into()); self }
    pub fn edge(mut self, v: DrawerEdge) -> Self { self.spec.edge = v; self }
    pub fn modal(mut self, v: bool) -> Self { self.spec.is_modal = v; self }
    pub fn dismiss_on_escape(mut self, v: bool) -> Self { self.spec.dismiss_on_escape = v; self }
    pub fn dismiss_on_backdrop(mut self, v: bool) -> Self { self.spec.dismiss_on_backdrop = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }


    pub fn with_content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    pub fn with_main_content(mut self, main: impl IntoElement) -> Self {
        self.main_content = Some(main.into_any_element());
        self
    }

    /// Called when the drawer should close (Escape, backdrop click).
    pub fn on_close(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(std::rc::Rc::new(handler));
        self
    }
}

impl IntoElement for Drawer {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let stack_gap = resolve_px(theme, "semantic.space.stack.sm");
        let panel_padding = resolve_px(theme, "semantic.space.panel.x");

        let surface_raw = resolve_color(theme, spec.surface_fill_token());
        let panel = resolve_color(theme, "semantic.color.background.panel");
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");

        // Contract: bg = color-mix 98% surface
        let surface_bg = color_mix(surface_raw, panel, 0.98);

        let is_left = spec.edge == DrawerEdge::Left || spec.edge == DrawerEdge::Top;

        // Contract: drawer radius = 0, min-width min(28rem, 100vw) ≈ 448px, shadow
        let mut drawer_panel = div()
            .id("flint-drawer-panel")
            .focusable()
            .min_w(px(448.0))
            .h_full()
            .rounded(px(0.0)) // Contract: drawer radius = 0
            .bg(surface_bg)
            .p(panel_padding)
            .flex()
            .flex_col()
            .gap(stack_gap)
            // Contract: elevation-dialog shadow
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
            ]);

        // Contract: border on side facing main area only
        if is_left {
            drawer_panel = drawer_panel.border_r_1().border_color(border);
        } else {
            drawer_panel = drawer_panel.border_l_1().border_color(border);
        }

        // Contract: title font 1rem (16px), weight 600
        if let Some(ref title) = spec.title {
            drawer_panel = drawer_panel.child(
                div()
                    .text_size(px(16.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_primary)
                    .child(title.clone()),
            );
        }

        // Contract: description font 0.875rem (14px)
        if let Some(ref description) = spec.description {
            drawer_panel = drawer_panel.child(
                div()
                    .text_size(px(14.0))
                    .text_color(text_secondary)
                    .child(description.clone()),
            );
        }

        // Escape key to close
        if spec.dismiss_on_escape {
            if let Some(ref handler) = self.on_close {
                let esc_handler = handler.clone();
                drawer_panel = drawer_panel.on_key_down(move |event: &KeyDownEvent, window, cx| {
                    if event.keystroke.key == "escape" {
                        esc_handler(window, cx);
                    }
                });
            }
        }

        // Content
        if let Some(content) = self.content {
            drawer_panel = drawer_panel.child(content);
        }

        // Main area
        let main = if let Some(main_content) = self.main_content {
            div().flex_1().flex().items_center().justify_center().child(main_content)
        } else {
            div()
                .flex_1()
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(text_secondary)
                        .child("Main area"),
                )
        };

        if spec.is_modal {
            // Modal mode: main as base layer, backdrop overlay on top
            let mut backdrop = div()
                .id("flint-drawer-backdrop")
                .absolute()
                .inset_0()
                .bg(hsla(0.0, 0.0, 0.0, 0.5))
                .flex()
                .occlude();

            // Backdrop click to dismiss
            if spec.dismiss_on_backdrop {
                if let Some(ref handler) = self.on_close {
                    let click_handler = handler.clone();
                    backdrop = backdrop.on_click(move |_event, window, cx| {
                        click_handler(window, cx);
                    });
                }
            }

            if is_left {
                backdrop = backdrop.child(drawer_panel).child(div().flex_1());
            } else {
                backdrop = backdrop.child(div().flex_1()).child(drawer_panel);
            }

            div()
                .relative()
                .size_full()
                .child(main)
                .child(backdrop)
                .into_any_element()
        } else {
            // Inline mode: side-by-side flex row
            let mut row = div().flex().h_full();

            if is_left {
                row = row.child(drawer_panel).child(main);
            } else {
                row = row.child(main).child(drawer_panel);
            }

            row.into_any_element()
        }
    }
}
