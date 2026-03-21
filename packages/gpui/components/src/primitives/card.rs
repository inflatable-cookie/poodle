use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{CardLayout, CardSpec, CardVariant};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

pub struct Card {
    spec: CardSpec,
    theme: GpuiThemeProvider,
    header: Option<AnyElement>,
    body: Option<AnyElement>,
    footer: Option<AnyElement>,
}

impl std::ops::Deref for Card {
    type Target = CardSpec;
    fn deref(&self) -> &CardSpec { &self.spec }
}

impl Card {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: CardSpec::new(),
            theme: theme.clone(),
            header: None,
            body: None,
            footer: None,
        }
    }

    pub fn from_spec(spec: CardSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            header: None,
            body: None,
            footer: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn variant(mut self, v: CardVariant) -> Self { self.spec.variant = v; self }
    pub fn layout(mut self, v: CardLayout) -> Self { self.spec.layout = v; self }
    pub fn interactive(mut self) -> Self { self.spec.is_interactive = true; self }
    pub fn selected(mut self) -> Self { self.spec.is_selected = true; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }

    // ── GPUI-specific builders ────────────────────────────────
    pub fn with_header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }

    pub fn with_body(mut self, body: impl IntoElement) -> Self {
        self.body = Some(body.into_any_element());
        self
    }

    pub fn with_footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }
}

impl IntoElement for Card {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let spec = &self.spec;
        let theme = &self.theme;

        // ── Resolve tokens at render time ─────────────────────
        let fill = resolve_color(theme, spec.fill_token());
        let border_color = spec.border_token().map(|t| resolve_color(theme, t));
        let radius = resolve_radius(theme, spec.radius_token());
        let selected_border_color = spec.selected_border_token().map(|t| resolve_color(theme, t));
        let hover_fill = spec.hover_fill_token().map(|t| resolve_color(theme, t));
        let _disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        let gap = resolve_px(theme, spec.gap_token());
        let padding_x = resolve_px(theme, spec.padding_x_token());
        let padding_y = resolve_px(theme, spec.padding_y_token());

        let is_horizontal = matches!(spec.layout, CardLayout::Horizontal);

        let mut el = div()
            .id("card")
            .bg(fill)
            .rounded(radius)
            .px(padding_x)
            .py(padding_y)
            .gap(gap)
            .overflow_hidden();

        // Layout direction
        if is_horizontal {
            el = el.flex().flex_row();
        } else {
            el = el.flex().flex_col();
        }

        // Border
        if let Some(border) = selected_border_color {
            el = el.border_2().border_color(border);
        } else if let Some(border) = border_color {
            el = el.border_1().border_color(border);
        }

        // Interactive hover
        if let Some(hover_fill) = hover_fill {
            el = el.cursor_pointer().hover(|s| s.bg(hover_fill));
        }

        // Header slot
        if let Some(header) = self.header {
            el = el.child(
                div()
                    .id("card-header")
                    .flex_shrink_0()
                    .child(header),
            );
        }

        // Body slot
        if let Some(body) = self.body {
            el = el.child(
                div()
                    .id("card-body")
                    .flex_grow()
                    .child(body),
            );
        }

        // Footer slot
        if let Some(footer) = self.footer {
            el = el.child(
                div()
                    .id("card-footer")
                    .flex_shrink_0()
                    .child(footer),
            );
        }

        el.into_any_element()
    }
}
