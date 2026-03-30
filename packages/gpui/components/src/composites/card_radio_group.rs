//! CardRadioGroup — selectable card group backed by CardRadioGroupSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_composites::CardRadioGroupSpec;
use poodle_primitives::{ControlDensity, ControlSize, SemanticControlSizeRole};
use crate::presentation::{resolve_semantic_size, size_font_rem, panel_space_x_rem, panel_space_y_rem, control_space_x_rem, rem_to_px};
use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius, resolve_opacity};

pub struct CardRadioGroup {
    spec: CardRadioGroupSpec,
    theme: GpuiThemeProvider,
    on_change: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for CardRadioGroup {
    type Target = CardRadioGroupSpec;
    fn deref(&self) -> &CardRadioGroupSpec { &self.spec }
}

impl CardRadioGroup {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: CardRadioGroupSpec::new(Vec::new()), theme: theme.clone(), on_change: None }
    }
    pub fn from_spec(spec: CardRadioGroupSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), on_change: None }
    }

    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(std::rc::Rc::new(handler));
        self
    }
    pub fn with_size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self { self.spec.size_role = v; self }
    pub fn with_density(mut self, v: ControlDensity) -> Self { self.spec.density = v; self }
}

impl IntoElement for CardRadioGroup {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let font_size = rem_to_px(size_font_rem(effective_size));
        let pad_x = rem_to_px(panel_space_x_rem(spec.density));
        let pad_y = rem_to_px(panel_space_y_rem(spec.density));
        let gap = rem_to_px(control_space_x_rem(spec.density));

        let unselected_fill = resolve_color(theme, spec.unselected_fill_token());
        let border = resolve_color(theme, spec.border_token());
        let radius = resolve_radius(theme, "semantic.radius.surface");
        let text_color = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let accent = resolve_color(theme, "semantic.color.accent.base");
        let focus_ring = resolve_color(theme, "semantic.color.accent.focusRing");
        let body_size = px(font_size);
        let selected = spec.value.as_deref().or(spec.default_value.as_deref());

        // Selected card: accent-tinted background (color-mix accent 12%)
        let selected_fill = color_mix(accent, unselected_fill, 0.12);

        // Grid-like layout: flex-wrap with gap
        let mut el = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .gap(px(gap))
;

        for (idx, option) in spec.options.iter().enumerate() {
            let is_selected = selected == Some(option.value.as_str());
            let is_option_disabled = spec.is_disabled || option.is_disabled;
            let fill = if is_selected { selected_fill } else { unselected_fill };
            let border_c = if is_selected { accent } else { border };
            let bw = if is_selected { 2.0 } else { 1.0 };

            // Radio indicator: 18px outer circle with border, 8px inner dot when selected
            let indicator = div()
                .w(px(18.0))
                .h(px(18.0))
                .rounded(px(999.0))
                .bg(unselected_fill)
                .border_1()
                .border_color(if is_selected { accent } else { border })
                .flex()
                .items_center()
                .justify_center()
                .flex_shrink_0()
                .child(
                    div()
                        .w(px(8.0))
                        .h(px(8.0))
                        .rounded(px(999.0))
                        .bg(if is_selected { accent } else { gpui::transparent_black() }),
                );

            // Content: label + optional description
            let mut content = div()
                .flex()
                .flex_col()
                .gap(px(2.0))
                .child(
                    div()
                        .text_size(body_size)
                        .text_color(text_color)
                        .child(option.label.clone()),
                );

            // Show description below label if present
            if let Some(ref description) = option.description {
                content = content.child(
                    div()
                        .text_size(px(12.0))
                        .text_color(text_secondary)
                        .child(description.clone()),
                );
            }

            let card_id = SharedString::from(format!("poodle-card-radio-{}", idx));
            let mut card = div()
                .id(card_id)
                .focusable()
                // Grid-like sizing: flex-1 with min-width for wrapping
                .flex_1()
                .min_w(px(200.0))
                .bg(fill)
                .rounded(radius)
                .border(px(bw))
                .border_color(border_c)
                .px(px(pad_x))
                .py(px(pad_y))
                .flex()
                .items_center()
                .gap(px(gap))
                // Focus ring
                .focus(move |s| s.border_color(focus_ring))
                .child(indicator)
                .child(content);

            if is_option_disabled {
                let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
                card = card
                    .opacity(opacity)
                    .cursor(CursorStyle::OperationNotAllowed);
            } else {
                card = card.cursor_pointer();

                if let Some(ref handler) = self.on_change {
                    let handler = handler.clone();
                    let val = option.value.clone();
                    card = card.on_click(move |_event, window, cx| {
                        handler(&val, window, cx);
                    });
                }
            }

            el = el.child(card);
        }
        if spec.is_disabled {
            let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
            el = el.opacity(opacity);
        }
        el.into_any_element()
    }
}
