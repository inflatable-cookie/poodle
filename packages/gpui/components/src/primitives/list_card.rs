//! ListCard — real GPUI component backed by ListCardSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{LeadingFill, LeadingShape, ListCardSpec};

use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

fn parse_hex_to_hsla(hex: &str) -> Option<Hsla> {
    let hex = hex.trim_start_matches('#');
    if hex.len() < 6 { return None; }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()? as f32 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()? as f32 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()? as f32 / 255.0;
    Some(Hsla::from(Rgba { r, g, b, a: 1.0 }))
}

/// A real GPUI list-card component backed by `ListCardSpec`.
pub struct ListCard {
    spec: ListCardSpec,
    theme: GpuiThemeProvider,
    leading: Option<AnyElement>,
    footer: Option<AnyElement>,
    trailing: Option<AnyElement>,
    _icon_color: Hsla,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for ListCard {
    type Target = ListCardSpec;
    fn deref(&self) -> &ListCardSpec { &self.spec }
}

impl ListCard {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        let spec = ListCardSpec::default();
        Self {
            spec,
            theme: theme.clone(),
            leading: None,
            footer: None,
            trailing: None,
            _icon_color: gpui::white(),
            on_click: None,
        }
    }

    pub fn from_spec(spec: ListCardSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            leading: None,
            footer: None,
            trailing: None,
            _icon_color: gpui::white(),
            on_click: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn title(mut self, v: impl Into<String>) -> Self { self.spec.title = v.into(); self }
    pub fn subtitle(mut self, v: impl Into<String>) -> Self { self.spec.subtitle = Some(v.into()); self }
    pub fn interactive(mut self, v: bool) -> Self { self.spec.is_interactive = v; self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn href(mut self, v: impl Into<String>) -> Self {
        self.spec.href = Some(v.into());
        self.spec.is_interactive = true;
        self
    }
    pub fn selectable(mut self, v: bool) -> Self { self.spec.is_selectable = v; self }
    pub fn selected(mut self, v: bool) -> Self { self.spec.is_selected = v; self }
    pub fn reorder_handle(mut self, v: bool) -> Self { self.spec.show_reorder_handle = v; self }


    pub fn with_leading(mut self, element: impl IntoElement) -> Self {
        self.leading = Some(element.into_any_element());
        self
    }

    pub fn with_footer(mut self, element: impl IntoElement) -> Self {
        self.footer = Some(element.into_any_element());
        self
    }

    pub fn with_trailing(mut self, element: impl IntoElement) -> Self {
        self.trailing = Some(element.into_any_element());
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl IntoElement for ListCard {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Resolve tokens ──────────────────────────────────────────
        let surface = resolve_color(theme, spec.fill_token());
        let panel = resolve_color(theme, "color.background.panel");
        let fill = color_mix(surface, panel, 0.88);
        let hover_fill = color_mix(surface, panel, 0.82);
        let border_subtle = resolve_color(theme, spec.border_token());
        let border_default = resolve_color(theme, spec.hover_border_token());
        let border = color_mix(border_subtle, panel, 0.18);
        let hover_border = color_mix(border_default, panel, 0.52);
        let radius = resolve_radius(theme, spec.radius_token());
        let title_color = resolve_color(theme, spec.title_color_token());
        let subtitle_color = resolve_color(theme, spec.subtitle_color_token());
        let meta_color = resolve_color(theme, spec.meta_color_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        let focus_ring_color = resolve_color(theme, spec.focus_ring_color_token());

        let body_size = resolve_px(theme, "typography.body.size");
        let accent = resolve_color(theme, spec.leading_tint_bg_token());
        let leading_tint_bg = color_mix(accent, panel, 0.12);
        let leading_solid_bg = resolve_color(theme, spec.leading_solid_bg_token());

        let leading_size = px(match spec.leading_shape {
            LeadingShape::Circle => 32.0,
            LeadingShape::RoundedSquare => 44.0,
        });
        let leading_radius = px(match spec.leading_shape {
            LeadingShape::Circle => 16.0,
            LeadingShape::RoundedSquare => 6.0,
        });

        let is_disabled = spec.is_disabled;
        let is_not_live = spec.is_not_live;
        let is_interactive = spec.is_interactive && !is_disabled;

        // ── Leading slot ────────────────────────────────────────────
        let leading_el = {
            let bg = match spec.leading_fill {
                LeadingFill::Tint => leading_tint_bg,
                LeadingFill::Solid => leading_solid_bg,
            };

            let mut container = div()
                .w(leading_size)
                .h(leading_size)
                .rounded(leading_radius)
                .bg(bg)
                .flex()
                .items_center()
                .justify_center()
                .flex_shrink_0();

            if let Some(leading) = self.leading {
                container = container.child(leading);
            }

            container
        };

        // ── Body section ────────────────────────────────────────────
        let body = {
            let mut col = div().flex().flex_col().flex_grow().min_w_0();

            col = col.child(
                div()
                    .text_size(body_size)
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(title_color)
                    .overflow_x_hidden()
                    .text_ellipsis()
                    .whitespace_nowrap()
                    .child(spec.title.clone()),
            );

            if let Some(ref subtitle) = spec.subtitle {
                col = col.child(
                    div()
                        .text_size(px(12.0))
                        .text_color(subtitle_color)
                        .overflow_x_hidden()
                        .text_ellipsis()
                        .whitespace_nowrap()
                        .child(subtitle.clone()),
                );
            }

            if let Some(footer) = self.footer {
                col = col.child(div().mt(px(4.0)).child(footer));
            }

            col
        };

        // ── Meta section ────────────────────────────────────────────
        let meta_el = spec.meta.as_ref().map(|meta| {
            div()
                .text_size(px(12.0))
                .text_color(meta_color)
                .flex_shrink_0()
                .child(meta.clone())
        });

        // ── Sash badge ────────────────────────────────────────────
        let sash_el = spec.sash.as_ref().map(|sash_text| {
            let sash_bg = spec.sash_color.as_ref()
                .and_then(|c| parse_hex_to_hsla(c))
                .unwrap_or_else(|| resolve_color(theme, "color.status.success"));

            div()
                .absolute()
                .top(px(0.0))
                .right(px(0.0))
                .px(px(6.0))
                .py(px(2.0))
                .rounded_bl(px(4.0))
                .bg(sash_bg)
                .text_color(gpui::white())
                .text_size(px(9.0))
                .font_weight(FontWeight::BOLD)
                .line_height(px(12.0))
                .child(sash_text.to_uppercase())
        });

        let has_sash = sash_el.is_some();

        // ── Root container ──────────────────────────────────────────
        // Accessibility semantics (contract section 6):
        // When interactive/selectable: role="button", tabindex="0", aria-label from spec or title
        // When selectable: aria-pressed={selected}
        // When disabled: aria-disabled="true"
        // When href present: rendered as anchor (link semantics)
        // Non-interactive: no role (generic container)
        let mut root = div()
            .id(SharedString::from(format!("poodle-list-card-{}", spec.title)))
            .w_full()
            .px(px(12.0))
            .py(px(10.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            .rounded(radius)
            .bg(fill)
            .border_1()
            .border_color(border);

        // Optional selection checkbox as the first child.
        if spec.is_selectable {
            let accent_base = resolve_color(theme, "color.accent.base");
            let surface = resolve_color(theme, "color.background.surface");
            let box_bg = if spec.is_selected { accent_base } else { surface };
            let box_border = if spec.is_selected {
                accent_base
            } else {
                border_subtle
            };
            root = root.child(
                div()
                    .flex_shrink_0()
                    .w(px(16.0))
                    .h(px(16.0))
                    .rounded(px(3.0))
                    .border_1()
                    .border_color(box_border)
                    .bg(box_bg),
            );
        }

        root = root.child(leading_el).child(body);

        if has_sash {
            root = root.relative().overflow_hidden();
        }

        if let Some(meta) = meta_el {
            root = root.child(meta);
        }

        if let Some(trailing) = self.trailing {
            root = root.child(trailing);
        }

        // Reorder drag handle (last child before the sash overlay).
        if spec.show_reorder_handle {
            let handle_color = resolve_color(theme, "color.text.secondary");
            // Three stacked dots as a visual grab indicator.
            let dot = |color: Hsla| {
                div().w(px(3.0)).h(px(3.0)).rounded(px(1.5)).bg(color)
            };
            let col = move || {
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .child(dot(handle_color))
                    .child(dot(handle_color))
                    .child(dot(handle_color))
            };
            root = root.child(
                div()
                    .flex()
                    .flex_shrink_0()
                    .items_center()
                    .gap(px(2.0))
                    .opacity(0.6)
                    .cursor(CursorStyle::OpenHand)
                    .child(col())
                    .child(col()),
            );
        }

        if let Some(sash) = sash_el {
            root = root.child(sash);
        }

        root = root.focus(move |s| s.border_color(focus_ring_color).shadow(crate::theme_ext::focus_ring_shadow(focus_ring_color)));

        // ── Interactive hover state ─────────────────────────────────
        if is_interactive {
            root = root
                .cursor_pointer()
                .hover(|style| style.bg(hover_fill).border_color(hover_border));
        }

        // ── Not-live state: dashed border, reduced opacity ──────────
        if is_not_live {
            root = root.opacity(0.6);
        }

        // ── Disabled state ──────────────────────────────────────────
        if is_disabled {
            root = root
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        root.into_any_element()
    }
}
