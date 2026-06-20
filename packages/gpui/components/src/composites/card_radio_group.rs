//! CardRadioGroup — single-select card group backed by CardRadioGroupSpec.
//!
//! Each option composes the `Card` primitive (interactive, and selected when
//! chosen) so the selected fill/border/focus ring all come from Card's own
//! token-resolved treatment — never a hand-rolled accent tint. The Card body
//! carries a header row (radio indicator + title) and an optional description.

use crate::presentation::{control_space_x_rem, rem_to_px, resolve_semantic_size};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};
use crate::Card;
use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::CardRadioGroupSpec;
use poodle_specs::{CardSpec, ControlDensity, ControlSize, SemanticControlSizeRole};

pub struct CardRadioGroup {
    spec: CardRadioGroupSpec,
    theme: GpuiThemeProvider,
    on_change: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for CardRadioGroup {
    type Target = CardRadioGroupSpec;
    fn deref(&self) -> &CardRadioGroupSpec {
        &self.spec
    }
}

impl CardRadioGroup {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: CardRadioGroupSpec::new(Vec::new()),
            theme: theme.clone(),
            on_change: None,
        }
    }
    pub fn from_spec(spec: CardRadioGroupSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_change: None,
        }
    }

    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(std::rc::Rc::new(handler));
        self
    }
    pub fn with_size(mut self, v: ControlSize) -> Self {
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
}

impl IntoElement for CardRadioGroup {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        // Contract §7/§8 size scale — resolved through the spec, not literals.
        let indicator_size = px(rem_to_px(CardRadioGroupSpec::indicator_size_rem(
            effective_size,
        )));
        let dot_size = px(rem_to_px(CardRadioGroupSpec::dot_size_rem(effective_size)));
        let indicator_border = px(rem_to_px(spec.indicator_border_rem()));
        let title_font = px(rem_to_px(CardRadioGroupSpec::title_font_rem(effective_size)));
        let description_font = px(rem_to_px(CardRadioGroupSpec::description_font_rem(
            effective_size,
        )));

        // Density-driven grid gap (contract §8: compact 0.5 · default 0.75 · comfortable 0.875).
        let grid_gap = px(rem_to_px(control_space_x_rem(spec.density)));
        // Header row gap is density-fixed 0.5rem per contract (§8); inline.sm = 0.5rem.
        let header_gap = resolve_px(theme, "space.inline.sm");
        // Body column rhythm between header and description (Card stack rhythm).
        let body_gap = resolve_px(theme, "space.stack.sm");

        let indicator_border_color = resolve_color(theme, spec.border_token());
        let accent = resolve_color(theme, "color.accent.base");
        let dot_color = resolve_color(theme, "color.text.inverse");
        let pill_radius = resolve_radius(theme, "radius.pill");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");

        let selected = spec.value.as_deref().or(spec.default_value.as_deref());

        let mut el = div().flex().flex_row().flex_wrap().gap(grid_gap);

        for option in spec.options.iter() {
            let is_selected = selected == Some(option.value.as_str());
            let is_option_disabled = spec.is_disabled || option.is_disabled;

            // Radio indicator: border-only unchecked; accent fill + inner dot checked.
            let indicator = div()
                .w(indicator_size)
                .h(indicator_size)
                .rounded(pill_radius)
                .border(indicator_border)
                .border_color(if is_selected {
                    accent
                } else {
                    indicator_border_color
                })
                .when(is_selected, |s| s.bg(accent))
                .flex()
                .items_center()
                .justify_center()
                .flex_shrink_0()
                .when(is_selected, |s| {
                    s.child(div().w(dot_size).h(dot_size).rounded(pill_radius).bg(dot_color))
                });

            // Header row: indicator + title. Title font-size comes from the
            // contract per-size scale; color/weight from tokens.
            let header = div()
                .flex()
                .items_center()
                .gap(header_gap)
                .child(indicator)
                .child(
                    div()
                        .text_size(title_font)
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(text_primary)
                        .child(option.label.clone()),
                );

            // Card body: header row + optional description.
            let body = div()
                .flex()
                .flex_col()
                .gap(body_gap)
                .child(header)
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
            let aria = option
                .aria_label
                .clone()
                .unwrap_or_else(|| option.label.clone());
            card_spec = card_spec.with_aria_label(aria);

            let card_id = format!("poodle-card-radio-{}", option.value);
            let card = Card::from_spec(card_spec, theme)
                .with_id(card_id.clone())
                .with_body(body);

            // Wrap each Card in the option container, carrying interaction +
            // per-item disabled styling (contract §8 option selectors).
            // The wrapper id makes the option focusable/clickable (role="radio").
            let option_id = SharedString::from(format!("poodle-card-radio-option-{}", option.value));
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
                if let Some(ref handler) = self.on_change {
                    let handler = handler.clone();
                    let val = option.value.clone();
                    option_el = option_el.on_click(move |_event, window, cx| {
                        handler(&val, window, cx);
                    });
                }
            }

            el = el.child(option_el);
        }

        if spec.is_disabled {
            let opacity = resolve_opacity(theme, "state.opacity.disabled");
            el = el.opacity(opacity);
        }
        el.into_any_element()
    }
}
