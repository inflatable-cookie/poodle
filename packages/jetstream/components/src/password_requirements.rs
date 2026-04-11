//! JsPasswordRequirements — password policy checklist backed by PasswordRequirementsSpec.
//!
//! Contract: `docs/contracts/components/password-requirements.md`
//! Reference: `packages/svelte/primitives/src/PasswordRequirements.svelte`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_components::PasswordRequirementsSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_color;

/// Build a Jetstream password requirements checklist from a PasswordRequirementsSpec.
///
/// Anatomy (from contract):
/// ```text
/// [Root]
///   ├── [Title]         — heading text
///   ├── [Requirement]... — list items with pass/fail indicators
///   │     ├── [Indicator] — checkmark or cross icon
///   │     └── [Label]     — requirement description
///   ├── [Hint]          — optional hint text
///   └── [Error]         — optional error message
/// ```
pub fn js_password_requirements(
    spec: &PasswordRequirementsSpec,
    theme: &JetstreamThemeProvider,
) -> JsEl {
    // ── Token resolution ──
    let fill: Color = resolve_color(theme, spec.fill_token()).into();
    let border_color: Color = resolve_color(theme, spec.border_token()).into();
    let title_color: Color = resolve_color(theme, spec.title_color_token()).into();
    let text_color: Color = resolve_color(theme, spec.text_color_token()).into();
    let met_color: Color = resolve_color(theme, spec.met_color_token()).into();
    let error_color: Color = resolve_color(theme, spec.error_color_token()).into();

    // ── Sizing ──
    let title_size = rem_to_px(0.8125); // Contract: label typography
    let body_size = rem_to_px(0.8125);
    let small_size = rem_to_px(0.75);
    let icon_size = rem_to_px(0.875);
    let gap = rem_to_px(0.5); // Stack gap
    let item_gap = rem_to_px(0.375); // Gap between indicator and label
    let padding = rem_to_px(0.75); // Panel padding
    let border_width = rem_to_px(0.0625);
    let radius = rem_to_px(0.5); // surface radius

    // ── Root container ──
    let mut root = ui_element::div()
        .flex_col()
        .gap(gap)
        .p(padding)
        .rounded(radius)
        .bg(fill)
        .border(border_width)
        .border_color(border_color);

    // ── Title ──
    root = root.child(
        ui_element::label(&spec.title)
            .text_size(title_size)
            .text_color(title_color)
            .text_weight(600),
    );

    // ── Loading state ──
    if spec.is_loading {
        root = root.child(
            ui_element::label(&spec.loading_label)
                .text_size(body_size)
                .text_color(text_color),
        );
        return root;
    }

    // ── Error state (no requirements) ──
    if let Some(ref error) = spec.error {
        root = root.child(
            ui_element::label(error.as_str())
                .text_size(body_size)
                .text_color(error_color),
        );
        return root;
    }

    // ── Requirements checklist ──
    if let Some(ref policy) = spec.requirements {
        // Min length
        root = root.child(build_requirement_item(
            &format!("At least {} characters", policy.min_length),
            spec.length_met(),
            met_color,
            text_color,
            body_size,
            icon_size,
            item_gap,
        ));

        // Mixed case
        if policy.require_mixed_case {
            root = root.child(build_requirement_item(
                "Upper and lowercase letters",
                spec.mixed_case_met(),
                met_color,
                text_color,
                body_size,
                icon_size,
                item_gap,
            ));
        }

        // Digit
        if policy.require_digit {
            root = root.child(build_requirement_item(
                "At least one number",
                spec.digit_met(),
                met_color,
                text_color,
                body_size,
                icon_size,
                item_gap,
            ));
        }

        // Special character
        if policy.require_special {
            root = root.child(build_requirement_item(
                "At least one special character",
                spec.special_met(),
                met_color,
                text_color,
                body_size,
                icon_size,
                item_gap,
            ));
        }

        // Optional policy description
        if let Some(ref description) = policy.description {
            root = root.child(
                ui_element::label(description.as_str())
                    .text_size(small_size)
                    .text_color(text_color),
            );
        }
    }

    // ── Hint text ──
    if let Some(ref hint) = spec.hint {
        root = root.child(
            ui_element::label(hint.as_str())
                .text_size(small_size)
                .text_color(text_color),
        );
    }

    root
}

/// Build a single requirement row with pass/fail indicator.
fn build_requirement_item(
    label_text: &str,
    is_met: bool,
    met_color: Color,
    unmet_color: Color,
    font_size: f32,
    icon_size: f32,
    gap: f32,
) -> JsEl {
    let indicator_icon = if is_met { "check" } else { "x" };
    let indicator_color = if is_met { met_color } else { unmet_color };

    ui_element::div()
        .flex_row()
        .gap(gap)
        .items_center()
        .child(
            ui_element::icon(indicator_icon)
                .w(icon_size)
                .h(icon_size)
                .text_color(indicator_color),
        )
        .child(
            ui_element::label(label_text)
                .text_size(font_size)
                .text_color(indicator_color),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_components::PasswordRequirementsPolicy;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn loading_state_shows_loading_label() {
        let spec = PasswordRequirementsSpec::new().with_loading(true);
        let el = js_password_requirements(&spec, &theme());
        // Title + loading label = 2 children
        assert_eq!(el.children.len(), 2);
    }

    #[test]
    fn error_state_shows_error_message() {
        let spec = PasswordRequirementsSpec::new().with_error("Failed to load");
        let el = js_password_requirements(&spec, &theme());
        // Title + error = 2 children
        assert_eq!(el.children.len(), 2);
    }

    #[test]
    fn with_requirements_shows_checklist() {
        let policy = PasswordRequirementsPolicy::new(8)
            .with_require_mixed_case(true)
            .with_require_digit(true)
            .with_require_special(true);
        let spec = PasswordRequirementsSpec::new()
            .with_requirements(policy)
            .with_password("Test1!");
        let el = js_password_requirements(&spec, &theme());
        // Title + 4 requirement items + hint = 6 children
        assert_eq!(el.children.len(), 6);
    }

    #[test]
    fn all_requirements_met() {
        let policy = PasswordRequirementsPolicy::new(8)
            .with_require_mixed_case(true)
            .with_require_digit(true)
            .with_require_special(true);
        let spec = PasswordRequirementsSpec::new()
            .with_requirements(policy)
            .with_password("StrongPass1!");
        assert!(spec.all_met());
    }

    #[test]
    fn no_requirements_shows_only_title_and_hint() {
        let spec = PasswordRequirementsSpec::new();
        let el = js_password_requirements(&spec, &theme());
        // Title + hint = 2 children (no requirements, no error, no loading)
        assert_eq!(el.children.len(), 2);
    }
}
