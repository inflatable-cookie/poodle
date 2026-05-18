use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ButtonVariant, ControlDensity, ControlSize, IconButtonSpec, IconSize, IconSpec, OrderBySpec,
    SemanticControlSizeRole, SortDirection,
};

use super::{Button, Icon, IconButton};
use crate::presentation::{
    control_height_rem, rem_to_px, resolve_semantic_size, size_font_rem, size_padding_x_offset_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

pub struct OrderBy {
    spec: OrderBySpec,
    theme: GpuiThemeProvider,
    on_reset: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for OrderBy {
    type Target = OrderBySpec;
    fn deref(&self) -> &OrderBySpec {
        &self.spec
    }
}

impl OrderBy {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(OrderBySpec::new(), theme)
    }

    pub fn from_spec(spec: OrderBySpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_reset: None,
        }
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

    pub fn on_reset(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_reset = Some(Box::new(handler));
        self
    }
}

impl IntoElement for OrderBy {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let text_primary = resolve_color(theme, spec.field_text_token());
        let text_secondary = resolve_color(theme, spec.label_color_token());
        let border = resolve_color(theme, spec.field_border_token());
        let surface = resolve_color(theme, spec.field_fill_token());
        let elevated = resolve_color(theme, spec.field_hover_fill_token());
        let accent = resolve_color(theme, spec.active_border_token());
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        let radius = resolve_radius(theme, spec.radius_token());

        let trigger_height = px(rem_to_px(match effective_size {
            ControlSize::Xs => 1.625,
            ControlSize::Sm => 1.75,
            ControlSize::Md => 2.0,
            ControlSize::Lg => 2.25,
            ControlSize::Xl => 2.5,
        }));
        let trigger_gap = px(rem_to_px(match spec.density {
            ControlDensity::Compact => 0.375,
            ControlDensity::Default => 0.5,
            ControlDensity::Comfortable => 0.625,
        }));
        let trigger_pad_x = px(rem_to_px(match effective_size {
            ControlSize::Xs => 0.5,
            _ => 0.75 + size_padding_x_offset_rem(effective_size),
        }));
        let label_size = px(rem_to_px(if matches!(effective_size, ControlSize::Xs) {
            0.625
        } else {
            0.75
        }));
        let summary_size = px(rem_to_px(0.875));
        let panel_gap = px(rem_to_px(0.75));
        let list_gap = px(rem_to_px(0.5));
        let item_gap = px(rem_to_px(0.75));
        let item_pad_x = px(rem_to_px(0.75));
        let item_pad_y = px(rem_to_px(0.625));
        let item_bg = Hsla {
            a: surface.a * 0.88 + elevated.a * 0.12,
            ..surface
        };

        let mut trigger = div()
            .min_w(px(rem_to_px(12.0)))
            .max_w(px(rem_to_px(28.0)))
            .h(trigger_height)
            .px(trigger_pad_x)
            .rounded(radius)
            .border_1()
            .border_color(border)
            .bg(surface)
            .flex()
            .items_center()
            .gap(trigger_gap)
            .text_color(text_primary);

        trigger = trigger
            .child(
                div()
                    .text_size(label_size)
                    .text_color(text_secondary)
                    .child("SORT BY"),
            )
            .child(
                div()
                    .flex_grow()
                    .text_size(summary_size)
                    .child(spec.summary_text()),
            )
            .child(
                Icon::from_spec(IconSpec::new("chevron-down").with_size(IconSize::Sm), theme)
                    .with_color(text_secondary),
            );

        let mut root = div()
            .id("order-by")
            .flex()
            .flex_col()
            .gap(px(rem_to_px(0.375)));

        let mut trigger_row = div()
            .flex()
            .items_center()
            .gap(px(rem_to_px(0.375)))
            .child(trigger);

        if spec.has_value() {
            let mut reset = div()
                .w(px(rem_to_px(1.75)))
                .h(px(rem_to_px(1.75)))
                .rounded(radius)
                .flex()
                .items_center()
                .justify_center()
                .text_color(text_secondary)
                .child("×");

            if let Some(handler) = self.on_reset {
                reset = reset.on_click(move |event, window, cx| handler(event, window, cx));
            }
            trigger_row = trigger_row.child(reset);
        }

        root = root.child(trigger_row);

        if spec.is_open {
            let current_value = spec.current_value();
            let mut panel = div().flex().flex_col().gap(panel_gap);

            if current_value.is_empty() {
                panel = panel.child(
                    div()
                        .text_size(summary_size)
                        .text_color(text_secondary)
                        .child("No sort fields selected"),
                );
            } else {
                let mut list = div().flex().flex_col().gap(list_gap);
                for (index, item) in current_value.iter().enumerate() {
                    let direction_label = match item.direction {
                        SortDirection::Asc => "Ascending",
                        SortDirection::Desc => "Descending",
                    };

                    let actions = div()
                        .flex()
                        .items_center()
                        .gap(px(rem_to_px(0.25)))
                        .child(IconButton::from_spec(
                            IconButtonSpec::new()
                                .with_icon(match item.direction {
                                    SortDirection::Asc => "arrow-up",
                                    SortDirection::Desc => "arrow-down",
                                })
                                .with_aria_label("Toggle direction")
                                .with_size(ControlSize::Sm),
                            theme,
                        ))
                        .child(IconButton::from_spec(
                            IconButtonSpec::new()
                                .with_icon("chevron-up")
                                .with_aria_label("Move up")
                                .with_size(ControlSize::Sm)
                                .with_disabled(index == 0),
                            theme,
                        ))
                        .child(IconButton::from_spec(
                            IconButtonSpec::new()
                                .with_icon("chevron-down")
                                .with_aria_label("Move down")
                                .with_size(ControlSize::Sm)
                                .with_disabled(index + 1 == current_value.len()),
                            theme,
                        ))
                        .child(IconButton::from_spec(
                            IconButtonSpec::new()
                                .with_icon("x")
                                .with_aria_label("Remove field")
                                .with_size(ControlSize::Sm)
                                .with_tone(poodle_specs::ButtonTone::Danger),
                            theme,
                        ));

                    list = list.child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .gap(item_gap)
                            .px(item_pad_x)
                            .py(item_pad_y)
                            .rounded(resolve_radius(theme, "radius.surface"))
                            .border_1()
                            .border_color(border)
                            .bg(item_bg)
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(rem_to_px(0.125)))
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(rem_to_px(0.375)))
                                            .child(div().text_color(text_secondary).child("⠿"))
                                            .child(
                                                div()
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .child(spec.active_label(&item.key)),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .text_size(summary_size)
                                            .text_color(text_secondary)
                                            .child(direction_label),
                                    ),
                            )
                            .child(actions),
                    );
                }
                panel = panel.child(list);
            }

            if !spec.available_fields().is_empty()
                && spec
                    .max_fields
                    .map(|max| spec.active_count() < max)
                    .unwrap_or(true)
            {
                panel = panel.child(
                    div().flex().items_center().child(
                        div()
                            .min_w(px(rem_to_px(10.0)))
                            .h(px(rem_to_px(control_height_rem(ControlSize::Sm))))
                            .px(px(rem_to_px(0.75)))
                            .rounded(radius)
                            .border_1()
                            .border_color(border)
                            .bg(surface)
                            .flex()
                            .items_center()
                            .text_color(text_secondary)
                            .child("+ Add field"),
                    ),
                );
            }

            if spec.active_count() >= 2 {
                panel = panel.child(
                    div().child(Button::from_spec(
                        poodle_specs::ButtonSpec::new()
                            .with_variant(ButtonVariant::Ghost)
                            .with_size(ControlSize::Sm)
                            .with_label("Clear all"),
                        theme,
                    )),
                );
            }

            root = root.child(
                div()
                    .rounded(resolve_radius(theme, "radius.surface"))
                    .border_1()
                    .border_color(border)
                    .bg(elevated)
                    .p(px(rem_to_px(0.75)))
                    .child(panel),
            );
        }

        if spec.is_disabled {
            root = root.opacity(disabled_opacity);
        }

        let _ = focus_ring;
        root.into_any_element()
    }
}
