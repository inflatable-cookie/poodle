//! TokenInput — GPUI tokenizing multi-value text entry backed by TokenInputSpec.
//!
//! Contract: `docs/contracts/components/token-input.md`
//! Reference: Svelte `TokenInput.svelte` (parity authority).
//!
//! Anatomy: field-chrome Root → Token Row (`.token-input__tokens`) holding the
//! committed-token Pills (each with its `x` Remove Button) followed by the live
//! draft Input Control (a real `TextInput`). Geometry resolves from size/density
//! tokens; commit/dedupe/separator parsing and caret editing are runtime
//! behaviors owned by the consumer's event loop (render-only here).

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ControlSize, PillAppearance, PillSize, PillSpec, PillTone, TextInputSpec, TokenInputSpec,
};
use std::rc::Rc;

use crate::presentation::{
    control_space_x_rem, rem_to_px, resolve_semantic_size, token_input_gap_rem,
    token_input_pad_x_offset_rem, token_input_pad_y_offset_rem,
};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};
use crate::primitives::{Pill, TextInput};

pub struct TokenInput {
    spec: TokenInputSpec,
    theme: GpuiThemeProvider,
    on_remove: Option<Rc<dyn Fn(usize, &ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl TokenInput {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(TokenInputSpec::new(), theme)
    }

    pub fn from_spec(spec: TokenInputSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_remove: None,
        }
    }

    /// Wire the per-token remove button. Receives the token index.
    pub fn on_remove(
        mut self,
        handler: impl Fn(usize, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_remove = Some(Rc::new(handler));
        self
    }
}

fn pill_size(size: ControlSize) -> PillSize {
    match size {
        ControlSize::Xs => PillSize::Xs,
        ControlSize::Sm => PillSize::Sm,
        ControlSize::Md => PillSize::Md,
        ControlSize::Lg => PillSize::Lg,
        ControlSize::Xl => PillSize::Xl,
    }
}

impl IntoElement for TokenInput {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        let surface = resolve_color(theme, "color.background.surface");
        let border = resolve_color(theme, "color.border.subtle");
        // Field family matches TextInput's interactive-subtle treatment.
        let fill = color_mix(surface, gpui::transparent_black(), 0.96);
        let radius = resolve_radius(theme, "radius.control");

        // Density-driven wrap gap; size+density-driven padding (contract §8).
        // padding-block = space.control.y token + per-size offset;
        // padding-inline = space.control.x (density) + per-size offset.
        let gap = px(rem_to_px(token_input_gap_rem(spec.density)));
        let control_y_px = f32::from(resolve_px(theme, "space.control.y"));
        let pad_y =
            px((control_y_px + rem_to_px(token_input_pad_y_offset_rem(effective_size))).max(0.0));
        let pad_x = px((rem_to_px(control_space_x_rem(spec.density))
            + rem_to_px(token_input_pad_x_offset_rem(effective_size)))
        .max(0.0));

        let opacity = if spec.disabled {
            resolve_opacity(theme, "state.opacity.disabled")
        } else {
            1.0
        };
        let can_edit = spec.can_edit();

        // Token Row (.token-input__tokens) — wraps committed pills + draft input.
        let mut row = div()
            .flex()
            .flex_wrap()
            .items_center()
            .flex_grow()
            .min_w_0()
            .gap(gap)
            .px(pad_x)
            .py(pad_y);

        for (index, token) in spec.values.iter().enumerate() {
            let mut pill = Pill::from_spec(
                PillSpec::new()
                    .with_label(token.clone())
                    .with_tone(PillTone::Neutral)
                    .with_appearance(PillAppearance::Subtle)
                    .with_size(pill_size(effective_size))
                    // Remove button is omitted in disabled / read-only mode
                    // (contract removal semantics).
                    .with_removable(can_edit),
                theme,
            );
            if can_edit {
                if let Some(ref handler) = self.on_remove {
                    let handler = handler.clone();
                    pill = pill
                        .on_remove(move |event, window, cx| handler(index, event, window, cx));
                }
            }
            row = row.child(pill);
        }

        // Live draft control — a real TextInput (not a faked placeholder div).
        // Forwards id, placeholder, disabled/read-only, maxLength, aria-label,
        // and size/density. Separator/dedupe/commit parsing is consumer-owned.
        let mut draft = TextInputSpec::new()
            .with_disabled(spec.disabled)
            .with_read_only(spec.read_only);
        // Match the field's resolved size/density to the draft input.
        draft.size = effective_size;
        draft.size_role = spec.size_role;
        draft.density = spec.density;
        if !spec.id.is_empty() {
            draft = draft.with_id(spec.id.clone());
        }
        if spec.values.is_empty() {
            if let Some(placeholder) = &spec.placeholder {
                draft = draft.with_placeholder(placeholder.clone());
            }
        }
        if let Some(max) = spec.max_length {
            draft = draft.with_max_length(max);
        }
        if let Some(aria) = &spec.aria_label {
            draft = draft.with_aria_label(aria.clone());
        }
        if let Some(described) = &spec.described_by {
            draft = draft.with_description_id(described.clone());
        }
        if let Some(name) = &spec.name {
            draft = draft.with_name(name.clone());
        }

        row = row.child(
            div()
                .flex_grow()
                .flex_basis(px(rem_to_px(8.0)))
                .min_w(px(rem_to_px(6.0)))
                .child(TextInput::from_spec(draft, theme)),
        );

        div()
            .w_full()
            .min_w_0()
            .flex()
            .rounded(radius)
            .border_1()
            .border_color(border)
            .bg(fill)
            .opacity(opacity)
            .child(row)
            .into_any_element()
    }
}
