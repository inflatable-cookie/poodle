use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::NavCardSpec;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub struct NavCard {
    spec: NavCardSpec,
    fill: Hsla,
    border_color: Hsla,
    hover_border_color: Hsla,
    hover_fill: Hsla,
    radius: Pixels,
    title_color: Hsla,
    description_color: Hsla,
    icon_bg: Hsla,
    icon_color: Hsla,
    badge_bg: Hsla,
    badge_color: Hsla,
    arrow_color: Hsla,
    disabled_opacity: f32,
    focus_ring_color: Hsla,
    icon: Option<AnyElement>,
    on_click: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for NavCard {
    type Target = NavCardSpec;
    fn deref(&self) -> &NavCardSpec { &self.spec }
}

impl NavCard {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(NavCardSpec::new(), theme)
    }

    pub fn from_spec(spec: NavCardSpec, theme: &GpuiThemeProvider) -> Self {
        let fill = resolve_color(theme, spec.fill_token());
        let border_color = resolve_color(theme, spec.border_token());
        let hover_border_color = resolve_color(theme, spec.hover_border_token());
        let hover_fill = resolve_color(theme, spec.hover_fill_token());
        let radius = resolve_radius(theme, spec.radius_token());
        let title_color = resolve_color(theme, spec.title_color_token());
        let description_color = resolve_color(theme, spec.description_color_token());
        let icon_bg = resolve_color(theme, spec.icon_bg_token());
        let icon_color = resolve_color(theme, spec.icon_color_token());
        let badge_bg = resolve_color(theme, spec.badge_bg_token());
        let badge_color = resolve_color(theme, spec.badge_color_token());
        let arrow_color = resolve_color(theme, spec.arrow_color_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        let focus_ring_color = resolve_color(theme, spec.focus_ring_color_token());

        Self {
            spec,
            fill,
            border_color,
            hover_border_color,
            hover_fill,
            radius,
            title_color,
            description_color,
            icon_bg,
            icon_color,
            badge_bg,
            badge_color,
            arrow_color,
            disabled_opacity,
            focus_ring_color,
            icon: None,
            on_click: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn title(mut self, v: impl Into<String>) -> Self { self.spec.title = v.into(); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.spec.description = Some(v.into()); self }
    pub fn href(mut self, v: impl Into<String>) -> Self { self.spec.href = Some(v.into()); self }


    pub fn with_icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl IntoElement for NavCard {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let is_disabled = self.spec.is_disabled;
        let fill = self.fill;
        let border_color = self.border_color;
        let hover_border_color = self.hover_border_color;
        let hover_fill = self.hover_fill;
        let radius = self.radius;
        let title_color = self.title_color;
        let description_color = self.description_color;
        let icon_bg = self.icon_bg;
        let arrow_color = self.arrow_color;
        let disabled_opacity = self.disabled_opacity;
        let focus_ring_color = self.focus_ring_color;
        let badge_bg = self.badge_bg;
        let badge_color = self.badge_color;

        // Icon slot: 2rem square, rounded, accent-tinted background
        let icon_slot = div()
            .w(px(32.0))
            .h(px(32.0))
            .rounded(px(8.0))
            .bg(icon_bg.opacity(0.12))
            .flex()
            .items_center()
            .justify_center()
            .flex_shrink_0()
            .children(self.icon);

        // Title row with optional badge
        let mut title_row = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .child(
                div()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(title_color)
                    .child(self.spec.title.clone()),
            );

        if let Some(badge) = &self.spec.badge {
            title_row = title_row.child(
                div()
                    .px(px(8.0))
                    .py(px(2.0))
                    .rounded(px(9999.0))
                    .bg(badge_bg.opacity(0.16))
                    .text_size(px(11.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(badge_color)
                    .child(badge.clone()),
            );
        }

        // Content column: title row + optional description
        let mut content = div().flex().flex_col().gap(px(4.0)).flex_grow().child(title_row);

        if let Some(desc) = &self.spec.description {
            content = content.child(
                div()
                    .text_size(px(13.0))
                    .text_color(description_color)
                    .child(desc.clone()),
            );
        }

        // Arrow indicator
        let arrow = div()
            .text_size(px(14.0))
            .text_color(arrow_color)
            .flex_shrink_0()
            .opacity(0.0)
            .child("\u{2192}");

        // Root element
        let mut root = div()
            .id(ElementId::Name(
                format!("nav-card-{}", self.spec.title).into(),
            ))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            .px(px(16.0))
            .py(px(14.0))
            .bg(fill)
            .border_1()
            .border_color(border_color.opacity(0.32))
            .rounded(radius)
            .child(icon_slot)
            .child(content)
            .child(arrow);

        root = root.focus(move |s| s.border_color(focus_ring_color));

        if is_disabled {
            root = root
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        } else {
            root = root
                .cursor_pointer()
                .hover(|style| {
                    style
                        .bg(hover_fill.opacity(0.52))
                        .border_color(hover_border_color.opacity(0.28))
                });

            // Show arrow on hover via group
            // Note: GPUI hover on child visibility requires the arrow to be
            // handled via the hover style on the root. We set arrow opacity
            // to 0 by default and rely on the root hover state.

            if let Some(handler) = self.on_click {
                root = root.on_click(move |_event, window, app| {
                    handler(window, app);
                });
            }
        }

        root.into_any_element()
    }
}
