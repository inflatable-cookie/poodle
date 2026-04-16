//! Dialog — real GPUI component backed by DialogSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ControlDensity, ControlSize, DialogKind, DialogSpec, DialogWidth, IconSize, IconSpec,
    SemanticControlSizeRole,
};

use super::icon::Icon;
use crate::presentation::{
    control_height_rem, panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size,
    size_font_rem,
};
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
    /// Optional header override. When set, replaces the default
    /// title/description rendering entirely.
    header: Option<AnyElement>,
    /// Optional footer override. When set, replaces the default actions
    /// row (including its layout) with a fully custom footer.
    footer: Option<AnyElement>,
    /// Called when the dialog open state should change.
    on_open_change: Option<std::rc::Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Dialog {
    type Target = DialogSpec;
    fn deref(&self) -> &DialogSpec {
        &self.spec
    }
}

impl Dialog {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: DialogSpec::new(),
            theme: theme.clone(),
            actions: None,
            content: None,
            header: None,
            footer: None,
            on_open_change: None,
        }
    }

    pub fn from_spec(spec: DialogSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            actions: None,
            content: None,
            header: None,
            footer: None,
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
    pub fn kind(mut self, v: DialogKind) -> Self {
        self.spec.kind = v;
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
    pub fn width(mut self, v: DialogWidth) -> Self {
        self.spec.width = v;
        self
    }
    pub fn bare(mut self, v: bool) -> Self {
        self.spec.bare = v;
        self
    }
    pub fn show_close_button(mut self, v: bool) -> Self {
        self.spec.show_close_button = v;
        self
    }
    pub fn close_label(mut self, v: impl Into<String>) -> Self {
        self.spec.close_label = v.into();
        self
    }

    /// Called when the dialog open state should change.
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

    /// Custom header override. Replaces the default title/description
    /// region entirely. The close button (when enabled) is still
    /// rendered as a sibling in the top-right corner.
    pub fn with_header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }

    /// Custom footer override. Replaces the default actions row
    /// (including margin-top, gap, and justification) with a fully
    /// custom footer element.
    pub fn with_footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }
}

impl IntoElement for Dialog {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let density_pad_x = px(rem_to_px(panel_space_x_rem(spec.density)));
        let density_pad_y = px(rem_to_px(panel_space_y_rem(spec.density)));

        let actions_gap = resolve_px(theme, "space.inline.sm");
        let panel_x = density_pad_x;
        let panel_y = density_pad_y;

        let elevated_bg = resolve_color(theme, "color.background.elevated");
        let border_default = resolve_color(theme, "color.border.default");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let radius = resolve_radius(theme, "radius.surface");
        let control_radius = resolve_radius(theme, "radius.control");
        let body_size = px(rem_to_px(size_font_rem(effective_size)));
        // Close button: chrome-sized (one step smaller than dialog size)
        let chrome_size = resolve_semantic_size(effective_size, SemanticControlSizeRole::Chrome);
        let close_btn_dim = px(rem_to_px(control_height_rem(chrome_size)));
        let heading_size = resolve_px(theme, "typography.heading.size");

        // Matches Svelte treatment-surface-elevated values:
        //   fill: color-mix(elevated 94%, transparent)
        //   border: color-mix(border-default 22%, transparent)
        let bg = Hsla {
            a: elevated_bg.a * 0.94,
            ..elevated_bg
        };
        let border = Hsla {
            a: border_default.a * 0.22,
            ..border_default
        };

        let stack_lg = resolve_px(theme, "space.stack.lg");

        // Resolve configured width preset. Full maps to "fill container"
        // (max_w_full only, no fixed width); everything else converts
        // the rem value directly to pixels.
        let width_px = px(rem_to_px(spec.surface_width_rem()));
        let is_full_width = spec.is_full_width();

        let mut dialog = div().id("poodle-dialog").focusable().rounded(radius);

        // Bare mode: drop the default padding — the consumer's content
        // extends to the full surface edges.
        if !spec.bare {
            dialog = dialog.px(panel_x).py(panel_y);
        }

        // Brand-raised treatment: gradient fill for elevated surface
        if theme.brand_raised {
            dialog = dialog.bg(crate::theme_ext::brand_raised_surface_fill(bg));
        } else {
            dialog = dialog.bg(bg);
        }

        dialog = dialog
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
            .max_w_full()
            // Svelte: gap 0.375rem (6px)
            .gap(px(6.0))
            .overflow_hidden()
            .occlude();

        if !is_full_width {
            dialog = dialog.w(width_px);
        } else {
            dialog = dialog.w_full();
        }

        // Bare mode: consumer owns the full surface — skip header,
        // description, and the default actions/footer chrome.
        if spec.bare {
            if let Some(content) = self.content {
                dialog = dialog.child(content);
            }
        } else {
            // Capture whether a custom header was supplied BEFORE moving it
            // into the header row — the description fallback needs to know.
            let had_custom_header = self.header.is_some();
            let has_header_content = had_custom_header || spec.title.is_some();

            if has_header_content || spec.show_close_button {
                let mut header_row = div()
                    .flex()
                    .items_start()
                    .justify_between()
                    .gap(actions_gap);

                if let Some(header_el) = self.header {
                    header_row = header_row.child(div().flex_1().child(header_el));
                } else if let Some(ref title) = spec.title {
                    header_row = header_row.child(
                        div()
                            .flex_1()
                            .text_size(heading_size)
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(text_primary)
                            .child(title.clone()),
                    );
                } else {
                    // No header content but a close button — still reserve
                    // the flex-1 child so the close button anchors right.
                    header_row = header_row.child(div().flex_1());
                }

                if spec.show_close_button {
                    let close_id = SharedString::from("poodle-dialog-close");
                    let close_hover_bg = Hsla { a: 0.08, ..text_secondary };
                    let mut close_button = div()
                        .id(close_id)
                        .flex()
                        .items_center()
                        .justify_center()
                        .w(close_btn_dim)
                        .h(close_btn_dim)
                        .rounded(control_radius)
                        .cursor_pointer()
                        .flex_shrink_0()
                        .hover(move |s| s.bg(close_hover_bg))
                        .child(
                            Icon::from_spec(
                                IconSpec::new("x").with_size(IconSize::Sm),
                                theme,
                            )
                            .with_color(text_secondary),
                        );
                    if let Some(ref handler) = self.on_open_change {
                        let handler = handler.clone();
                        close_button = close_button.on_click(move |_event, window, cx| {
                            handler(false, window, cx);
                        });
                    }
                    header_row = header_row.child(close_button);
                }

                dialog = dialog.child(header_row);
            }

            // Description: only emitted when no custom header is in use
            // (a custom header is expected to carry its own secondary copy).
            if !had_custom_header {
                if let Some(ref description) = spec.description {
                    dialog = dialog.child(
                        div()
                            .text_size(body_size)
                            .text_color(text_secondary)
                            .child(description.clone()),
                    );
                }
            }

            // Content slot — body content between description and actions
            if let Some(content) = self.content {
                dialog = dialog.child(content);
            }

            // Footer: custom footer replaces the default actions row.
            if let Some(footer) = self.footer {
                dialog = dialog.child(div().mt(stack_lg).child(footer));
            } else if let Some(actions) = self.actions {
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
        }

        // Escape key on dialog surface
        if spec.dismiss_on_escape {
            if let Some(ref handler) = self.on_open_change {
                let esc_handler = handler.clone();
                dialog = dialog.on_key_down(move |event: &KeyDownEvent, window, cx| {
                    if event.keystroke.key == "escape" {
                        esc_handler(false, window, cx);
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
            if let Some(ref handler) = self.on_open_change {
                let click_handler = handler.clone();
                backdrop = backdrop.on_click(move |_event, window, cx| {
                    click_handler(false, window, cx);
                });
            }
        }

        backdrop.into_any_element()
    }
}
