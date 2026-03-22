//! ListCard — real GPUI component backed by ListCardSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{LeadingFill, LeadingShape, ListCardSpec};

use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_radius};

/// A real GPUI list-card component backed by `ListCardSpec`.
pub struct ListCard {
    spec: ListCardSpec,
    theme: GpuiThemeProvider,
    leading: Option<AnyElement>,
    footer: Option<AnyElement>,
    trailing: Option<AnyElement>,
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
            on_click: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn title(mut self, v: impl Into<String>) -> Self { self.spec.title = v.into(); self }
    pub fn subtitle(mut self, v: impl Into<String>) -> Self { self.spec.subtitle = Some(v.into()); self }
    pub fn interactive(mut self, v: bool) -> Self { self.spec.is_interactive = v; self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }


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
        let panel = resolve_color(theme, "semantic.color.background.panel");
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
                    .text_sm()
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
                        .text_xs()
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
                .text_xs()
                .text_color(meta_color)
                .flex_shrink_0()
                .child(meta.clone())
        });

        // ── Root container ──────────────────────────────────────────
        let mut root = div()
            .id(SharedString::from(format!("pug-list-card-{}", spec.title)))
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
            .border_color(border)
            .child(leading_el)
            .child(body);

        if let Some(meta) = meta_el {
            root = root.child(meta);
        }

        if let Some(trailing) = self.trailing {
            root = root.child(trailing);
        }

        root = root.focus(move |s| s.border_color(focus_ring_color));

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
