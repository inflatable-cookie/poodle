//! CardToggleGroup — card toggle group backed by CardToggleGroupSpec.
//!
//! Mirrors `card_radio_group.rs`: each option composes the `Card` primitive
//! (interactive, and selected when its value is selected) so the selected
//! fill/border/focus ring all come from Card's own token-resolved treatment —
//! never a hand-rolled accent tint. The Card body carries the title and an
//! optional description. This is a multi-select toggle group (`values`), which
//! is the correct model for a toggle group; render is selection-state-agnostic.

use crate::presentation::{control_space_x_rem, rem_to_px, resolve_semantic_size};
use crate::theme_ext::{resolve_color, resolve_opacity};
use crate::Card;
use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{CardSpec, CardToggleGroupSpec};

pub struct CardToggleGroup {
    spec: CardToggleGroupSpec,
    theme: GpuiThemeProvider,
    on_toggle: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl CardToggleGroup {
    pub fn from_spec(spec: CardToggleGroupSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_toggle: None,
        }
    }

    pub fn on_toggle(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_toggle = Some(std::rc::Rc::new(handler));
        self
    }
}

impl IntoElement for CardToggleGroup {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        // Contract §7 size scale — resolved through the spec helpers, not literals.
        let title_font = px(rem_to_px(CardToggleGroupSpec::title_font_rem(effective_size)));
        let description_font = px(rem_to_px(CardToggleGroupSpec::description_font_rem(
            effective_size,
        )));

        // Density-driven grid gap (contract §7 density table).
        let grid_gap = px(rem_to_px(control_space_x_rem(spec.density)));
        // Card body rhythm between title and description.
        let body_gap = rem_to_px(0.25);

        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");

        let mut root = div().flex().flex_row().flex_wrap().gap(grid_gap);

        for option in spec.options.iter() {
            let is_selected = spec.is_selected(&option.value);
            let is_option_disabled = spec.disabled || option.disabled;

            // Card body: title + optional description, both token-resolved.
            let body = div()
                .flex()
                .flex_col()
                .gap(px(body_gap))
                .child(
                    div()
                        .text_size(title_font)
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(text_primary)
                        .child(option.title.clone()),
                )
                .when_some(option.description.as_ref(), |el, description| {
                    el.child(
                        div()
                            .text_size(description_font)
                            .text_color(text_secondary)
                            .child(description.clone()),
                    )
                });

            // Compose the Card primitive — selected state owns the fill/border.
            let mut card_spec = CardSpec::new().interactive();
            if is_selected {
                card_spec = card_spec.selected();
            }
            card_spec = card_spec.with_aria_label(option.title.clone());

            let card_id = format!("poodle-card-toggle-{}", option.value);
            let card = Card::from_spec(card_spec, theme)
                .with_id(card_id)
                .with_body(body);

            // Wrap each Card in the option container, carrying interaction +
            // per-item disabled styling.
            let option_id = SharedString::from(format!("poodle-card-toggle-option-{}", option.value));
            let mut option_el = div()
                .id(option_id)
                .focusable()
                .flex_1()
                .min_w(px(0.0))
                .child(card);

            if is_option_disabled {
                let opacity = resolve_opacity(theme, "state.opacity.disabled");
                option_el = option_el
                    .opacity(opacity)
                    .cursor(CursorStyle::OperationNotAllowed);
            } else {
                option_el = option_el.cursor_pointer();
                if let Some(ref handler) = self.on_toggle {
                    let handler = handler.clone();
                    let val = option.value.clone();
                    option_el = option_el.on_click(move |_event, window, cx| {
                        handler(&val, window, cx);
                    });
                }
            }

            root = root.child(option_el);
        }

        if spec.disabled {
            let opacity = resolve_opacity(theme, "state.opacity.disabled");
            root = root.opacity(opacity);
        }

        root.into_any_element()
    }
}
