use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{ControlDensity, ControlSize, OrderBySpec, SemanticControlSizeRole, SortDirection};
use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem, size_padding_x_offset_rem, control_space_x_rem, control_height_rem};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub struct OrderBy {
    spec: OrderBySpec,
    label_color: Hsla,
    field_fill: Hsla,
    field_border: Hsla,
    field_text: Hsla,
    field_hover_fill: Hsla,
    active_fill: Hsla,
    active_border: Hsla,
    active_text: Hsla,
    radius: Pixels,
    disabled_opacity: f32,
    focus_ring_color: Hsla,
    gap: Pixels,
    reset_color: Hsla,
    body_size: Pixels,
    on_sort: Option<Box<dyn Fn(&str, &SortDirection, &mut Window, &mut App) + 'static>>,
    on_reset: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for OrderBy {
    type Target = OrderBySpec;
    fn deref(&self) -> &OrderBySpec { &self.spec }
}

impl OrderBy {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(OrderBySpec::new(), theme)
    }

    pub fn from_spec(spec: OrderBySpec, theme: &GpuiThemeProvider) -> Self {
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        let label_color = resolve_color(theme, spec.label_color_token());
        let field_fill = resolve_color(theme, spec.field_fill_token());
        let field_border = resolve_color(theme, spec.field_border_token());
        let field_text = resolve_color(theme, spec.field_text_token());
        let field_hover_fill = resolve_color(theme, spec.field_hover_fill_token());
        let active_fill = resolve_color(theme, spec.active_fill_token());
        let active_border = resolve_color(theme, spec.active_border_token());
        let active_text = resolve_color(theme, spec.active_text_token());
        let radius = resolve_radius(theme, spec.radius_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        let focus_ring_color = resolve_color(theme, spec.focus_ring_color_token());
        let gap = px(rem_to_px(control_space_x_rem(spec.density)));
        let reset_color = resolve_color(theme, spec.reset_color_token());

        Self {
            spec,
            label_color,
            field_fill,
            field_border,
            field_text,
            field_hover_fill,
            active_fill,
            active_border,
            active_text,
            radius,
            disabled_opacity,
            focus_ring_color,
            gap,
            reset_color,
            body_size: px(rem_to_px(size_font_rem(effective_size))),
            on_sort: None,
            on_reset: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self { self.spec.size_role = v; self }
    pub fn with_density(mut self, v: ControlDensity) -> Self { self.spec.density = v; self }

    pub fn on_sort(
        mut self,
        handler: impl Fn(&str, &SortDirection, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_sort = Some(Box::new(handler));
        self
    }

    pub fn on_reset(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_reset = Some(Box::new(handler));
        self
    }
}

impl IntoElement for OrderBy {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let is_disabled = self.spec.is_disabled;
        let has_active_sort = self.spec.active_sort.is_some();
        let effective_size = resolve_semantic_size(self.spec.size, self.spec.size_role);
        let field_height = px(rem_to_px(control_height_rem(effective_size))) - px(2.0);
        let field_pad_x = px(rem_to_px(size_padding_x_offset_rem(effective_size))) + px(10.0);

        // "Sort by" label — use body_size - 1px chrome adjustment
        let label = div()
            .text_size(self.body_size)
            .font_weight(FontWeight::MEDIUM)
            .text_color(self.label_color)
            .flex_shrink_0()
            .child(self.spec.aria_label.to_uppercase());

        // Field buttons
        let on_sort = self.on_sort;
        let mut fields_row = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(self.gap);

        for field in &self.spec.fields {
            let is_active = self.spec.is_field_active(&field.value);
            let field_disabled = field.is_disabled || is_disabled;

            let direction_indicator = if is_active {
                let arrow = match self.spec.active_direction() {
                    Some(SortDirection::Asc) => "\u{25B2}",
                    Some(SortDirection::Desc) => "\u{25BC}",
                    None => "\u{25B2}",
                };
                Some(
                    div()
                        .text_size(px(9.0))
                        .ml(px(4.0))
                        .child(arrow.to_string()),
                )
            } else {
                None
            };

            let (btn_fill, btn_border, btn_text) = if is_active {
                (
                    self.active_fill.opacity(0.12),
                    self.active_border,
                    self.active_text,
                )
            } else {
                (self.field_fill, self.field_border, self.field_text)
            };

            let field_value = field.value.clone();
            let current_direction = if is_active {
                self.spec
                    .active_direction()
                    .cloned()
                    .unwrap_or(SortDirection::Asc)
            } else {
                SortDirection::Asc
            };

            let mut btn = div()
                .id(ElementId::Name(
                    format!("order-by-field-{}", field.value).into(),
                ))
                .flex()
                .flex_row()
                .items_center()
                .h(field_height)
                .px(field_pad_x)
                .bg(btn_fill)
                .border_1()
                .border_color(btn_border)
                .rounded(self.radius)
                .text_size(self.body_size)
                .font_weight(FontWeight::MEDIUM)
                .text_color(btn_text)
                .child(field.label.clone())
                .children(direction_indicator);

            {
                let focus_ring_color = self.focus_ring_color;
                btn = btn.focus(move |s| s.border_color(focus_ring_color).shadow(crate::theme_ext::focus_ring_shadow(focus_ring_color)));
            }

            if field_disabled {
                btn = btn
                    .opacity(self.disabled_opacity)
                    .cursor(CursorStyle::OperationNotAllowed);
            } else {
                let field_hover_fill = self.field_hover_fill;

                btn = btn.cursor_pointer();

                if !is_active {
                    btn = btn.hover(|style| style.bg(field_hover_fill));
                }

                if let Some(ref on_sort) = on_sort {
                    let on_sort_ptr = on_sort as *const Box<dyn Fn(&str, &SortDirection, &mut Window, &mut App)>;
                    let field_value = field_value.clone();
                    let next_direction = if is_active {
                        match current_direction {
                            SortDirection::Asc => SortDirection::Desc,
                            SortDirection::Desc => SortDirection::Asc,
                        }
                    } else {
                        SortDirection::Asc
                    };
                    btn = btn.on_click(move |_event, window, app| {
                        // SAFETY: The callback lives as long as the element
                        let on_sort = unsafe { &*on_sort_ptr };
                        on_sort(&field_value, &next_direction, window, app);
                    });
                }
            }

            fields_row = fields_row.child(btn);
        }

        // Reset button (only shown when sort is active)
        if has_active_sort {
            let reset_color = self.reset_color;
            let focus_ring_color = self.focus_ring_color;

            let mut reset_btn = div()
                .id("order-by-reset")
                .flex()
                .items_center()
                .justify_center()
                .w(field_height)
                .h(field_height)
                .rounded(self.radius)
                .text_size(self.body_size)
                .text_color(reset_color)
                .cursor_pointer()
                .hover(|style| style.bg(self.field_hover_fill))
                .focus(move |s| s.border_color(focus_ring_color))
                .child("\u{00D7}");

            if let Some(on_reset) = self.on_reset {
                reset_btn = reset_btn.on_click(move |_event, window, app| {
                    on_reset(window, app);
                });
            }

            fields_row = fields_row.child(reset_btn);
        }

        // Root
        let mut root = div()
            .id("order-by")
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .child(label)
            .child(fields_row);

        if is_disabled {
            root = root.opacity(self.disabled_opacity);
        }

        root.into_any_element()
    }
}
