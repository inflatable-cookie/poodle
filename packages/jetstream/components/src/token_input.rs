//! JsTokenInput — multi-value token/chip input backed by TokenInputSpec.
//!
//! Contract: `docs/contracts/components/token-input.md`
//! Reference: GPUI `primitives/token_input.rs`.
//!
//! Render-only: token entry/removal (typing, separators, backspace) lives in the
//! preview event loop. Renders the committed values as Pill chips + a trailing
//! placeholder inside a bordered, control-height field. ZERO hardcoded values.

use glam::Vec4;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{PillAppearance, PillSpec, PillTone, TokenInputSpec};

use crate::pill::js_pill;
use crate::presentation::{control_height_rem, control_space_x_rem, rem_to_px};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_radius};

/// Build a token-input element from its spec.
pub fn js_token_input(spec: &TokenInputSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let surface = resolve_color(theme, "color.background.surface");
    let border = resolve_color(theme, "color.border.subtle");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let fill = color_mix(surface, Vec4::ZERO, 0.96);

    let min_h = rem_to_px(control_height_rem(spec.size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let radius = resolve_radius(theme, "radius.control");

    // Wrapping row of committed tokens + trailing placeholder.
    let mut row = ui_element::div()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(rem_to_px(0.375));
    for token in &spec.values {
        row = row.child(js_pill(
            &PillSpec::new()
                .with_label(token.clone())
                .with_tone(PillTone::Neutral)
                .with_appearance(PillAppearance::Subtle),
            theme,
        ));
    }
    if let Some(placeholder) = &spec.placeholder {
        row = row.child(ui_element::label(placeholder.as_str()).text_color(text_secondary));
    }

    let mut el = ui_element::div()
        .min_h(min_h)
        .px(pad_x)
        .py(rem_to_px(0.3125))
        .rounded(radius)
        .border_1()
        .border_color(border)
        .bg(fill)
        .child(row);

    if spec.disabled {
        el = el.opacity(resolve_opacity(theme, "state.opacity.disabled"));
    }
    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn renders_each_token_chip() {
        let el = js_token_input(
            &TokenInputSpec::new().with_values(vec!["rust".into(), "svelte".into()]),
            &theme(),
        );
        let tree = probe(&el, 320.0, 60.0);
        assert!(tree.has_text("rust") && tree.has_text("svelte"), "chips missing: {:?}", tree.texts());
    }

    #[test]
    fn renders_placeholder() {
        let el = js_token_input(
            &TokenInputSpec::new().with_placeholder("Add tag…"),
            &theme(),
        );
        let tree = probe(&el, 320.0, 60.0);
        assert!(tree.has_text("Add tag…"), "placeholder missing: {:?}", tree.texts());
    }

    #[test]
    fn disabled_dims() {
        let el = js_token_input(&TokenInputSpec::new().with_disabled(true), &theme());
        assert!(el.style.opacity < 1.0, "disabled token-input should be dimmed");
    }
}
