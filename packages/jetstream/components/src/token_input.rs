//! JsTokenInput — multi-value token/chip input backed by TokenInputSpec.
//!
//! Contract: `docs/contracts/components/token-input.md`
//! Reference: Svelte `TokenInput.svelte` (parity authority); GPUI `primitives/token_input.rs`.
//!
//! Render-only: token *entry* (typing, separator split, Enter/Tab commit,
//! Backspace-removes-last, blur commit, dedupe) and remove-click handling live
//! in the preview event loop. This builder renders all committed-token chips
//! (each with its `×` remove affordance, omitted in disabled/read-only mode),
//! the live text-input draft control, and the field chrome — all token-resolved.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{PillAppearance, PillSpec, PillTone, TextInputSpec, TokenInputSpec};

use crate::pill::js_pill;
use crate::presentation::{
    control_space_x_rem, rem_to_px, resolve_semantic_size, token_input_font_rem,
    token_input_gap_rem, token_input_pad_x_offset_rem, token_input_pad_y_offset_rem,
};
use crate::text_input::js_text_input;
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// Build a token-input element from its spec.
/// TokenInput — committed tokens plus a draft field.
///
/// Mirrors the GPUI target's `on_remove`. Entry is typing, which this runtime
/// does not raise, so removal is the only wired route.
pub struct TokenInput {
    spec: TokenInputSpec,
    theme: JetstreamThemeProvider,
    on_remove: Option<crate::element::Handler>,
}

impl TokenInput {
    pub fn from_spec(spec: TokenInputSpec, theme: &JetstreamThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), on_remove: None }
    }

    /// Fires with the token's text when its `×` is pressed.
    ///
    /// The value, not the index: a host that removed by index would delete the
    /// wrong token whenever two removals arrive between renders.
    pub fn on_remove(mut self, handler: impl Fn(&str) + Send + Sync + 'static) -> Self {
        self.on_remove = Some(std::sync::Arc::new(handler));
        self
    }
}

impl crate::element::IntoJsEl for TokenInput {
    fn into_js_el(self) -> JsEl {
        build(&self.spec, &self.theme, self.on_remove)
    }
}

pub fn js_token_input(spec: &TokenInputSpec, theme: &JetstreamThemeProvider) -> JsEl {
    build(spec, theme, None)
}

fn build(
    spec: &TokenInputSpec,
    theme: &JetstreamThemeProvider,
    on_remove: Option<crate::element::Handler>,
) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    let surface = resolve_color(theme, "color.background.surface");
    let border = resolve_color(theme, "color.border.subtle");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    // Field family matches TextInput's interactive-subtle treatment: a
    // surface-mix fill behind a subtle border (Svelte `--token-input-fill`).
    let fill = color_mix(surface, glam::Vec4::ZERO, 0.96);
    let radius = resolve_radius(theme, "radius.control");

    // Size-driven font + density-driven wrap gap (contract §8 / Svelte overrides).
    let token_font = token_input_font_rem(effective_size)
        .map(rem_to_px)
        .unwrap_or_else(|| resolve_px(theme, "typography.body.size"));
    let gap = rem_to_px(token_input_gap_rem(spec.density));

    // Padding-block from control.y + per-size offset; padding-inline from
    // control.x (density) + per-size offset — never a flat literal.
    let pad_y = (resolve_px(theme, "space.control.y")
        + rem_to_px(token_input_pad_y_offset_rem(effective_size)))
    .max(0.0);
    let pad_x = (rem_to_px(control_space_x_rem(spec.density))
        + rem_to_px(token_input_pad_x_offset_rem(effective_size)))
    .max(0.0);

    let can_edit = spec.can_edit();

    // Wrapping token row: committed pills (+ remove ×) then the draft control.
    let mut row = ui_element::div()
        .flex_row()
        .flex_wrap()
        .items_center()
        .grow()
        .min_w_0()
        .gap(gap)
        .px(pad_x)
        .py(pad_y);

    for (index, token) in spec.values.iter().enumerate() {
        let pill = js_pill(
            &PillSpec::new()
                .with_label(token.clone())
                .with_tone(PillTone::Neutral)
                .with_appearance(PillAppearance::Subtle)
                .with_size(pill_size(effective_size)),
            theme,
        );
        // Token = pill label + remove `×` (contract §2 anatomy: Token Label +
        // Remove Button). The remove affordance is omitted while disabled or
        // read-only (removal semantics). Entry/removal wiring lives in preview.
        let mut chip = ui_element::div()
            .flex_row()
            .items_center()
            .min_w_0()
            .gap(rem_to_px(0.125))
            .child(pill);
        if can_edit {
            let mut remove = ui_element::button("×")
                .id(format!("poodle-token-remove-{index}"))
                .text_color(text_secondary)
                .text_size(token_font)
                .focusable();

            if let Some(handler) = &on_remove {
                let handler = std::sync::Arc::clone(handler);
                let value = token.clone();
                remove = remove.cursor_pointer().on_click(move |_event| handler(&value));
            }

            chip = chip.child(remove);
        }
        row = row.child(chip);
    }

    // Live draft control — a real text input (not a faked div). Inherits size,
    // density, placeholder, maxLength, disabled/read-only from the spec. Search
    // chrome is intentionally absent (token-input has no leading icon/clear).
    let mut draft = TextInputSpec::new()
        .with_size(effective_size)
        .with_size_role(spec.size_role)
        .with_density(spec.density)
        .with_disabled(spec.disabled)
        .with_read_only(spec.read_only);
    if !spec.id.is_empty() {
        draft = draft.with_id(spec.id.clone());
    }
    // Placeholder only shows when there are no committed tokens (Svelte parity).
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
    // The draft sits inline in the wrap row and grows to fill the trailing space.
    row = row.child(
        ui_element::div()
            .grow()
            .flex_basis(rem_to_px(8.0))
            .min_w(rem_to_px(6.0))
            .child(js_text_input(&draft, theme)),
    );

    let mut el = ui_element::div()
        .flex_row()
        .w_full()
        .min_w_0()
        .rounded(radius)
        .border_1()
        .border_color(border)
        .bg(fill)
        .child(row);

    if spec.disabled {
        el = el.opacity(resolve_opacity(theme, "state.opacity.disabled"));
    }
    crate::aria::with_aria_label(el, spec.aria_label.as_deref())
}

/// Token pills track the field size so they remain visually secondary.
fn pill_size(size: poodle_specs::ControlSize) -> poodle_specs::PillSize {
    use poodle_specs::{ControlSize, PillSize};
    match size {
        ControlSize::Xs => PillSize::Xs,
        ControlSize::Sm => PillSize::Sm,
        ControlSize::Md => PillSize::Md,
        ControlSize::Lg => PillSize::Lg,
        ControlSize::Xl => PillSize::Xl,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn renders_each_token_chip() {
        let el = js_token_input(
            &TokenInputSpec::new().with_values(vec!["rust".into(), "svelte".into()]),
            &theme(),
        );
        let tree = probe(&el, 360.0, 80.0);
        assert!(
            tree.has_text("rust") && tree.has_text("svelte"),
            "chips missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn renders_remove_x_per_token_when_editable() {
        let el = js_token_input(
            &TokenInputSpec::new().with_values(vec!["a".into(), "b".into()]),
            &theme(),
        );
        let tree = probe(&el, 360.0, 80.0);
        // One id-tagged remove button per committed token.
        assert!(tree.find_token("poodle-token-remove-0").is_some());
        assert!(tree.find_token("poodle-token-remove-1").is_some());
        assert!(tree.has_text("×"), "remove glyph missing: {:?}", tree.texts());
    }

    #[test]
    fn hides_remove_when_read_only() {
        let el = js_token_input(
            &TokenInputSpec::new()
                .with_values(vec!["a".into()])
                .with_read_only(true),
            &theme(),
        );
        let tree = probe(&el, 360.0, 80.0);
        assert!(
            tree.find_token("poodle-token-remove-0").is_none(),
            "read-only must hide remove affordance"
        );
        // chip text still present (values stay visible in read-only).
        assert!(tree.has_text("a"));
    }

    #[test]
    fn renders_input_control_and_placeholder() {
        let el = js_token_input(
            &TokenInputSpec::new().with_placeholder("Add tag…"),
            &theme(),
        );
        let tree = probe(&el, 360.0, 80.0);
        // The live draft renders via js_text_input (a real field, focusable);
        // its placeholder text is present as a rendered label.
        assert!(
            tree.has_text("Add tag…"),
            "placeholder missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn placeholder_hidden_when_populated() {
        let el = js_token_input(
            &TokenInputSpec::new()
                .with_values(vec!["x".into()])
                .with_placeholder("Add tag…"),
            &theme(),
        );
        let tree = probe(&el, 360.0, 80.0);
        assert!(
            !tree.has_text("Add tag…"),
            "placeholder must hide once tokens exist"
        );
    }

    #[test]
    fn disabled_dims() {
        let el = js_token_input(&TokenInputSpec::new().with_disabled(true), &theme());
        assert!(el.style.opacity < 1.0, "disabled token-input should be dimmed");
    }

    /// The token's text, not its index: a host removing by index would delete
    /// the wrong token whenever two removals arrive between renders.
    #[test]
    fn removing_a_token_reports_its_value() {
        use crate::element::IntoJsEl;
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let values = Arc::clone(&seen);

        let spec = TokenInputSpec::new().with_values(vec!["rust".into(), "svelte".into()]);
        let el = TokenInput::from_spec(spec, &theme())
            .on_remove(move |value| values.lock().unwrap().push(value.to_string()))
            .into_js_el();

        // Two identical × glyphs, so target the second by its rendered rect.
        let tree = crate::render_probe::probe(&el, 480.0, 120.0);
        let crosses: Vec<_> = tree
            .nodes
            .iter()
            .filter(|n| n.text.as_deref() == Some("×"))
            .collect();
        assert_eq!(crosses.len(), 2, "one remove control per token");
        let second = crosses[1];
        crate::element::click_probe::click_at(
            &el,
            480.0,
            120.0,
            second.x + second.w / 2.0,
            second.y + second.h / 2.0,
        );

        assert_eq!(seen.lock().unwrap().as_slice(), ["svelte"]);
    }

    #[test]
    fn a_read_only_token_input_draws_no_remove_control() {
        let el = js_token_input(
            &TokenInputSpec::new()
                .with_values(vec!["rust".into()])
                .with_read_only(true),
            &theme(),
        );
        let tree = crate::render_probe::probe(&el, 480.0, 120.0);

        assert!(
            !tree.texts().iter().any(|t| *t == "×"),
            "a read-only field offered a remove control: {:?}",
            tree.texts()
        );
    }

}
