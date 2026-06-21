use crate::presentation::rem_to_px;
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ControlDensity, NavCardSpec};

pub struct NavCard {
    spec: NavCardSpec,
    fill: Hsla,
    border_color: Hsla,
    hover_border_color: Hsla,
    hover_fill: Hsla,
    radius: Pixels,
    icon_radius: Pixels,
    badge_radius: Pixels,
    title_color: Hsla,
    description_color: Hsla,
    icon_bg: Hsla,
    _icon_color: Hsla,
    badge_bg: Hsla,
    badge_color: Hsla,
    arrow_color: Hsla,
    disabled_opacity: f32,
    focus_ring_color: Hsla,
    body_size: Pixels,
    label_size: Pixels,
    icon: Option<AnyElement>,
    on_click: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for NavCard {
    type Target = NavCardSpec;
    fn deref(&self) -> &NavCardSpec {
        &self.spec
    }
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
        let icon_radius = resolve_radius(theme, spec.icon_radius_token());
        let badge_radius = resolve_radius(theme, spec.badge_radius_token());
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
            icon_radius,
            badge_radius,
            title_color,
            description_color,
            icon_bg,
            _icon_color: icon_color,
            badge_bg,
            badge_color,
            arrow_color,
            disabled_opacity,
            focus_ring_color,
            body_size: resolve_px(theme, "typography.body.size"),
            label_size: resolve_px(theme, "typography.label.size"),
            icon: None,
            on_click: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn title(mut self, v: impl Into<String>) -> Self {
        self.spec.title = v.into();
        self
    }
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.spec.description = Some(v.into());
        self
    }
    pub fn href(mut self, v: impl Into<String>) -> Self {
        self.spec.href = Some(v.into());
        self
    }

    pub fn density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }

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
        let spec = &self.spec;
        let is_disabled = spec.is_disabled;
        let fill = self.fill;
        let border_color = self.border_color;
        let hover_border_color = self.hover_border_color;
        let hover_fill = self.hover_fill;
        let radius = self.radius;
        let icon_radius = self.icon_radius;
        let badge_radius = self.badge_radius;
        let title_color = self.title_color;
        let description_color = self.description_color;
        let icon_bg = self.icon_bg;
        let arrow_color = self.arrow_color;
        let disabled_opacity = self.disabled_opacity;
        let focus_ring_color = self.focus_ring_color;
        let badge_bg = self.badge_bg;
        let badge_color = self.badge_color;
        let _ = self.body_size;

        // ── Density-aware geometry (contract §8 Density Overrides table) ──
        let root_gap = px(rem_to_px(spec.root_gap_rem()));
        let pad_x = px(rem_to_px(spec.padding_x_rem()));
        let pad_y = px(rem_to_px(spec.padding_y_rem()));
        let content_gap = px(rem_to_px(spec.content_gap_rem()));
        let title_gap = px(rem_to_px(spec.title_gap_rem()));
        let icon_size = px(rem_to_px(spec.icon_size_rem()));
        let icon_font = px(rem_to_px(spec.icon_font_size_rem()));
        let badge_font = px(rem_to_px(spec.badge_font_size_rem()));
        let arrow_size = px(rem_to_px(1.0));
        let title_font = self.label_size; // contract §8 Title: typography.label.size

        // Group name so the arrow can reveal on ROOT hover (contract §8 Arrow root-hover).
        let group_name: SharedString = format!("nav-card-{}", spec.title).into();

        // Icon slot: density-sized square, accent-tinted background, control radius.
        let icon_slot = div()
            .w(icon_size)
            .h(icon_size)
            .rounded(icon_radius)
            .bg(icon_bg.opacity(0.12))
            .flex()
            .items_center()
            .justify_center()
            .flex_shrink_0()
            .text_size(icon_font)
            .children(self.icon);

        // Title row with optional badge — weight 600 (SEMIBOLD), title gap from density.
        let mut title_row = div().flex().flex_row().items_center().gap(title_gap).child(
            div()
                .text_size(title_font)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(title_color)
                .child(spec.title.clone()),
        );

        if let Some(badge) = &spec.badge {
            title_row = title_row.child(
                div()
                    .px(px(rem_to_px(0.375)))
                    .py(px(rem_to_px(0.0625)))
                    .rounded(badge_radius)
                    .bg(badge_bg.opacity(0.16))
                    .text_size(badge_font)
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(badge_color)
                    .child(badge.clone()),
            );
        }

        // Content column: title row + optional description, content gap from density.
        let mut content = div()
            .flex()
            .flex_col()
            .gap(content_gap)
            .flex_grow()
            .min_w_0()
            .child(title_row);

        if let Some(desc) = &spec.description {
            content = content.child(
                div()
                    .text_size(px(rem_to_px(spec.description_font_size_rem())))
                    .text_color(description_color)
                    .child(desc.clone()),
            );
        }

        // Arrow: 1rem square, hidden at rest, revealed on ROOT hover via group_hover.
        let arrow = div()
            .text_size(arrow_size)
            .text_color(arrow_color)
            .flex_shrink_0()
            .opacity(0.0)
            .group_hover(group_name.clone(), |s| s.opacity(1.0))
            .child("\u{2192}");

        // Root element
        let mut root = div()
            .id(ElementId::Name(format!("nav-card-{}", spec.title).into()))
            .group(group_name)
            .flex()
            .flex_row()
            .items_center()
            .gap(root_gap)
            .px(pad_x)
            .py(pad_y)
            .bg(fill)
            .border_1()
            .border_color(border_color.opacity(0.32))
            .rounded(radius)
            .child(icon_slot)
            .child(content)
            .child(arrow);

        root = root.focus(move |s| {
            s.border_color(focus_ring_color)
                .shadow(crate::theme_ext::focus_ring_shadow(focus_ring_color))
        });

        if is_disabled {
            root = root
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        } else {
            root = root.cursor_pointer().hover(move |style| {
                // Svelte: bg = color-mix(elevated 52%, surface)
                // Svelte: border = color-mix(accent 28%, border-subtle)
                style
                    .bg(color_mix(hover_fill, fill, 0.52))
                    .border_color(color_mix(hover_border_color, border_color, 0.28))
            });

            if let Some(handler) = self.on_click {
                root = root.on_click(move |_event, window, app| {
                    handler(window, app);
                });
            }
        }

        root.into_any_element()
    }
}
