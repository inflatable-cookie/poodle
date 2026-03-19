use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{CardLayout, CardSpec};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

pub struct PugCard {
    spec: CardSpec,
    fill: Hsla,
    border_color: Option<Hsla>,
    radius: Pixels,
    selected_border_color: Option<Hsla>,
    hover_fill: Option<Hsla>,
    disabled_opacity: f32,
    gap: Pixels,
    padding_x: Pixels,
    padding_y: Pixels,
    header: Option<AnyElement>,
    body: Option<AnyElement>,
    footer: Option<AnyElement>,
}

impl PugCard {
    pub fn new(spec: CardSpec, theme: &GpuiThemeProvider) -> Self {
        let fill = resolve_color(theme, spec.fill_token());
        let border_color = spec.border_token().map(|t| resolve_color(theme, t));
        let radius = resolve_radius(theme, spec.radius_token());
        let selected_border_color = spec.selected_border_token().map(|t| resolve_color(theme, t));
        let hover_fill = spec.hover_fill_token().map(|t| resolve_color(theme, t));
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        let gap = resolve_px(theme, spec.gap_token());
        let padding_x = resolve_px(theme, spec.padding_x_token());
        let padding_y = resolve_px(theme, spec.padding_y_token());

        Self {
            spec,
            fill,
            border_color,
            radius,
            selected_border_color,
            hover_fill,
            disabled_opacity,
            gap,
            padding_x,
            padding_y,
            header: None,
            body: None,
            footer: None,
        }
    }

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

impl IntoElement for PugCard {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let is_horizontal = matches!(self.spec.layout, CardLayout::Horizontal);

        let mut el = div()
            .id("card")
            .bg(self.fill)
            .rounded(self.radius)
            .px(self.padding_x)
            .py(self.padding_y)
            .gap(self.gap)
            .overflow_hidden();

        // Layout direction
        if is_horizontal {
            el = el.flex().flex_row();
        } else {
            el = el.flex().flex_col();
        }

        // Border
        if let Some(border) = self.selected_border_color {
            el = el.border_2().border_color(border);
        } else if let Some(border) = self.border_color {
            el = el.border_1().border_color(border);
        }

        // Interactive hover
        if let Some(hover_fill) = self.hover_fill {
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
