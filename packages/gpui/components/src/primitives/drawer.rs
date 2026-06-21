//! Drawer — real GPUI component backed by DrawerSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ControlDensity, ControlSize, DrawerEdge, DrawerSpec, SemanticControlSizeRole};

use crate::presentation::{
    drawer_title_font_rem, panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size,
    size_font_rem,
};
use crate::theme_ext::{color_mix, resolve_color, resolve_px};

/// A real GPUI drawer component backed by `DrawerSpec`.
///
/// Renders a side panel. The parent controls the `open` state.
pub struct Drawer {
    spec: DrawerSpec,
    theme: GpuiThemeProvider,
    /// The content to show inside the drawer panel.
    content: Option<AnyElement>,
    /// The footer actions row (`.drawer__actions`) — flex-end, wraps.
    actions: Option<AnyElement>,
    /// The main area content (shown next to the drawer).
    main_content: Option<AnyElement>,
    /// Called when the drawer open state should change.
    on_open_change: Option<std::rc::Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Drawer {
    type Target = DrawerSpec;
    fn deref(&self) -> &DrawerSpec {
        &self.spec
    }
}

impl Drawer {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: DrawerSpec::new(),
            theme: theme.clone(),
            content: None,
            actions: None,
            main_content: None,
            on_open_change: None,
        }
    }

    pub fn from_spec(spec: DrawerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            content: None,
            actions: None,
            main_content: None,
            on_open_change: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn open(mut self, v: bool) -> Self {
        self.spec.open = Some(v);
        self
    }
    pub fn default_open(mut self, v: bool) -> Self {
        self.spec.default_open = v;
        self
    }
    pub fn title(mut self, v: impl Into<String>) -> Self {
        self.spec.title = Some(v.into());
        self
    }
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.spec.description = Some(v.into());
        self
    }
    pub fn edge(mut self, v: DrawerEdge) -> Self {
        self.spec.edge = v;
        self
    }
    pub fn modal(mut self, v: bool) -> Self {
        self.spec.is_modal = v;
        self
    }
    pub fn dismiss_on_escape(mut self, v: bool) -> Self {
        self.spec.dismiss_on_escape = v;
        self
    }
    pub fn dismiss_on_backdrop(mut self, v: bool) -> Self {
        self.spec.dismiss_on_backdrop = v;
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

    pub fn with_content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    /// Footer action buttons rendered in the `.drawer__actions` row
    /// (contract anatomy: Actions part — flex-end, wrap).
    pub fn with_actions(mut self, actions: impl IntoElement) -> Self {
        self.actions = Some(actions.into_any_element());
        self
    }

    pub fn with_main_content(mut self, main: impl IntoElement) -> Self {
        self.main_content = Some(main.into_any_element());
        self
    }

    /// Called when the drawer open state should change.
    pub fn on_open_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(std::rc::Rc::new(handler));
        self
    }

    /// Compatibility shim for close-only listeners.
    pub fn on_close(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_open_change = Some(std::rc::Rc::new(move |_open, window, cx| {
            handler(window, cx);
        }));
        self
    }
}

impl IntoElement for Drawer {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let density_pad_x = px(rem_to_px(panel_space_x_rem(spec.density)));
        // Svelte: drawer uses panel-y for vertical, panel-x for horizontal (not both from panel-x)
        let density_pad_y = px(rem_to_px(panel_space_y_rem(spec.density)));
        let _body_font = px(rem_to_px(size_font_rem(effective_size)));
        // Contract §8 size table: header title font-size per size (md == 1rem).
        let title_size = px(rem_to_px(drawer_title_font_rem(effective_size)));

        let stack_gap = resolve_px(theme, "space.stack.sm");
        // Contract: header margin-bottom + actions margin-top = space.stack.md.
        let stack_md = resolve_px(theme, "space.stack.md");
        let actions_gap = resolve_px(theme, "space.inline.sm");

        let elevated_bg = resolve_color(theme, "color.background.elevated");
        let panel_bg = resolve_color(theme, "color.background.panel");
        let border_default = resolve_color(theme, "color.border.default");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let body_size = resolve_px(theme, "typography.body.size");

        // Svelte: treatment-surface-elevated-fill = color-mix(elevated 98%, panel)
        //         treatment-surface-elevated-border = color-mix(border-default 78%, transparent)
        let surface_bg = color_mix(elevated_bg, panel_bg, 0.98);
        let border = Hsla {
            a: border_default.a * 0.78,
            ..border_default
        };

        let is_left = spec.edge == DrawerEdge::Left || spec.edge == DrawerEdge::Top;
        let is_vertical_edge = spec.edge == DrawerEdge::Top || spec.edge == DrawerEdge::Bottom;

        // Contract: drawer radius = 0, shadow. Edge-anchored sizing:
        //   left/right → width min(28rem, 100vw), height 100vh
        //   top/bottom → width 100vw, height min(24rem, 100vh)
        // GPUI has no viewport-relative units in the component, so the rem
        // ceiling resolves to a fixed dimension (the `100vw`/`100vh` cap is
        // applied by the host layout box the drawer fills).
        let mut drawer_panel = div()
            .id("poodle-drawer-panel")
            .focusable()
            .rounded(px(0.0)); // Contract: drawer radius = 0
        if is_vertical_edge {
            drawer_panel = drawer_panel.w_full().h(px(rem_to_px(24.0))); // min(24rem, 100vh)
        } else {
            drawer_panel = drawer_panel.min_w(px(rem_to_px(28.0))).h_full(); // min(28rem, 100vw)
        }

        // Brand-raised treatment: gradient fill for elevated surface
        if theme.brand_raised {
            drawer_panel = drawer_panel.bg(crate::theme_ext::brand_raised_surface_fill(surface_bg));
        } else {
            drawer_panel = drawer_panel.bg(surface_bg);
        }

        drawer_panel = drawer_panel
            .px(density_pad_x)
            .py(density_pad_y)
            .flex()
            .flex_col()
            .gap(stack_gap)
            // Contract: elevation-dialog shadow
            .shadow(crate::theme_ext::elevation_dialog_shadow());

        // Contract: all-around border (0.0625rem solid border-default 78%)
        drawer_panel = drawer_panel.border_1().border_color(border);

        // Contract: Header part — grid of title + description with
        // margin-bottom: space-stack-md. Title font is size-table driven.
        if spec.title.is_some() || spec.description.is_some() {
            let mut header = div().flex().flex_col().gap(stack_gap).mb(stack_md);
            if let Some(ref title) = spec.title {
                header = header.child(
                    div()
                        .text_size(title_size)
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(text_primary)
                        .child(title.clone()),
                );
            }
            if let Some(ref description) = spec.description {
                header = header.child(
                    div()
                        .text_size(body_size)
                        .text_color(text_secondary)
                        .child(description.clone()),
                );
            }
            drawer_panel = drawer_panel.child(header);
        }

        // Escape key to close
        if spec.dismiss_on_escape {
            if let Some(ref handler) = self.on_open_change {
                let esc_handler = handler.clone();
                drawer_panel = drawer_panel.on_key_down(move |event: &KeyDownEvent, window, cx| {
                    if event.keystroke.key == "escape" {
                        esc_handler(false, window, cx);
                    }
                });
            }
        }

        // Body content (flex-grows so the actions row pins to the bottom)
        if let Some(content) = self.content {
            drawer_panel = drawer_panel.child(div().flex_1().child(content));
        }

        // Contract: Actions part — footer row, flex-end, wraps,
        // margin-top: space-stack-md.
        if let Some(actions) = self.actions {
            drawer_panel = drawer_panel.child(
                div()
                    .flex()
                    .flex_wrap()
                    .items_center()
                    .justify_end()
                    .gap(actions_gap)
                    .mt(stack_md)
                    .child(actions),
            );
        }

        // Main area
        let main = if let Some(main_content) = self.main_content {
            div()
                .flex_1()
                .flex()
                .items_center()
                .justify_center()
                .child(main_content)
        } else {
            div().flex_1().flex().items_center().justify_center().child(
                div()
                    .text_size(body_size)
                    .text_color(text_secondary)
                    .child("Main area"),
            )
        };

        // Contract: backdrop fill = color.background.overlay (token-resolved).
        let backdrop_fill = resolve_color(theme, spec.backdrop_fill_token());

        if spec.is_modal {
            // Modal mode: main as base layer, backdrop overlay on top
            let mut backdrop = div()
                .id("poodle-drawer-backdrop")
                .absolute()
                .inset_0()
                .bg(backdrop_fill)
                .flex()
                .occlude();
            // Top/bottom edges stack the panel vertically; left/right horizontally.
            if is_vertical_edge {
                backdrop = backdrop.flex_col();
            }

            // Backdrop click to dismiss
            if spec.dismiss_on_backdrop {
                if let Some(ref handler) = self.on_open_change {
                    let click_handler = handler.clone();
                    backdrop = backdrop.on_click(move |_event, window, cx| {
                        click_handler(false, window, cx);
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
            // Inline mode: side-by-side flex row (or stacked for top/bottom).
            let mut row = div().flex().size_full();
            if is_vertical_edge {
                row = row.flex_col();
            }

            if is_left {
                row = row.child(drawer_panel).child(main);
            } else {
                row = row.child(main).child(drawer_panel);
            }

            row.into_any_element()
        }
    }
}
