//! JsPasswordRequirements — password policy checklist backed by PasswordRequirementsSpec.
//!
//! Contract: `docs/contracts/components/password-requirements.md`
//! Reference: `packages/svelte/components/src/PasswordRequirements.svelte`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values.

use jetstream_ui::ui_element::{self, JsEl};
use jetstream_ui::Color;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::PasswordRequirementsSpec;

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

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

    // ── Sizing (contract §7 ladder, resolved via effective size) ──
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let title_size = rem_to_px(PasswordRequirementsSpec::title_size_rem(effective_size));
    let body_size = rem_to_px(PasswordRequirementsSpec::body_size_rem(effective_size));
    let icon_size = body_size; // indicator tracks body type size
    let gap = rem_to_px(PasswordRequirementsSpec::hint_gap_rem(effective_size)); // stack rhythm
    let item_gap = rem_to_px(0.375); // indicator↔label gap (JsEl approximation; no Svelte token)
    let padding = rem_to_px(PasswordRequirementsSpec::padding_rem(effective_size));
    let border_width = resolve_px(theme, spec.border_width_token());
    let radius = resolve_radius(theme, spec.radius_token());

    // ── Root container ──
    let mut root = ui_element::div()
        .flex_col()
        .gap(gap)
        .p(padding)
        .rounded(radius)
        .bg(fill)
        .border(border_width)
        .border_color(border_color);

    // ── Loading state (no title — matches Svelte) ──
    if spec.is_loading {
        root = root.child(
            ui_element::label(&spec.loading_label)
                .text_size(body_size)
                .text_color(text_color),
        );
        return root;
    }

    // ── Requirements checklist (title lives only in this branch) ──
    if let Some(ref policy) = spec.requirements {
        // Title
        root = root.child(
            ui_element::label(&format!("{}:", spec.title))
                .text_size(title_size)
                .text_color(title_color)
                .text_weight(600),
        );

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
                "Mix of uppercase and lowercase letters",
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
                    .text_size(body_size)
                    .text_color(text_color),
            );
        }

        // Hint — only inside the requirements branch (matches Svelte)
        if let Some(ref hint) = spec.hint {
            root = root.child(
                ui_element::label(hint.as_str())
                    .text_size(body_size)
                    .text_color(text_color),
            );
        }
    } else if let Some(ref error) = spec.error {
        // ── Error state (no requirements, no title — matches Svelte) ──
        root = root.child(
            ui_element::label(error.as_str())
                .text_size(body_size)
                .text_color(error_color),
        );
    }

    root.aria_role(jetstream_ui::accesskit::Role::Alert)
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
    use crate::render_probe::{probe, ProbeColor};
    use poodle_specs::{ControlSize, PasswordRequirementsPolicy};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn probe_color(theme: &JetstreamThemeProvider, token: &str) -> ProbeColor {
        let c = resolve_color(theme, token);
        ProbeColor {
            r: c.x,
            g: c.y,
            b: c.z,
            a: c.w,
        }
    }

    fn policy_all() -> PasswordRequirementsPolicy {
        PasswordRequirementsPolicy::new(8)
            .with_require_mixed_case(true)
            .with_require_digit(true)
            .with_require_special(true)
    }

    #[test]
    fn loading_state_shows_loading_label_no_title() {
        // Svelte: loading branch renders the loading paragraph only, no title.
        let spec = PasswordRequirementsSpec::new().with_loading(true);
        let el = js_password_requirements(&spec, &theme());
        assert_eq!(el.children.len(), 1, "loading shows only the loading label");
        let tree = probe(&el, 320.0, 200.0);
        assert!(tree.has_text("Loading requirements..."));
        assert!(
            !tree.has_text("Password requirements:"),
            "loading branch must not render the title"
        );
    }

    #[test]
    fn error_state_shows_error_no_title() {
        let spec = PasswordRequirementsSpec::new().with_error("Failed to load");
        let el = js_password_requirements(&spec, &theme());
        assert_eq!(el.children.len(), 1, "error shows only the error message");
        let tree = probe(&el, 320.0, 200.0);
        assert!(tree.has_text("Failed to load"));
        assert!(!tree.has_text("Password requirements:"));
    }

    #[test]
    fn empty_spec_renders_nothing_inside_panel() {
        // No requirements, no error, no loading → Svelte renders an empty panel.
        let spec = PasswordRequirementsSpec::new();
        let el = js_password_requirements(&spec, &theme());
        assert_eq!(el.children.len(), 0, "empty spec → no title, no hint");
    }

    #[test]
    fn requirements_branch_shows_title_rules_and_hint() {
        let spec = PasswordRequirementsSpec::new()
            .with_requirements(policy_all())
            .with_password("Test1!");
        let el = js_password_requirements(&spec, &theme());
        // Title + 4 requirement rows + hint = 6 children.
        assert_eq!(el.children.len(), 6);
        let tree = probe(&el, 360.0, 320.0);
        assert!(tree.has_text("Password requirements:"));
        assert!(tree.has_text("At least 8 characters"));
        // Label wording aligned to Svelte/contract.
        assert!(tree.has_text("Mix of uppercase and lowercase letters"));
        assert!(tree.has_text("At least one number"));
        assert!(tree.has_text("At least one special character"));
    }

    #[test]
    fn met_and_unmet_rows_use_distinct_indicators() {
        // Password satisfies length + mixed-case but not digit/special, so the
        // checklist carries both met (`check`) and unmet (`x`) indicators.
        let spec = PasswordRequirementsSpec::new()
            .with_requirements(policy_all())
            .with_password("Password");
        assert!(spec.length_met() && spec.mixed_case_met());
        assert!(!spec.digit_met() && !spec.special_met());

        let el = js_password_requirements(&spec, &theme());
        let tree = probe(&el, 360.0, 320.0);

        // Contract §6 non-color indicator: `check` for met, `x` for not-met.
        let checks = tree
            .nodes
            .iter()
            .filter(|n| n.text.as_deref() == Some("check"))
            .count();
        let crosses = tree
            .nodes
            .iter()
            .filter(|n| n.text.as_deref() == Some("x"))
            .count();
        assert_eq!(checks, 2, "length + mixed-case met → two check icons");
        assert_eq!(crosses, 2, "digit + special unmet → two cross icons");

        // Panel background resolves from the surface fill token (no hardcoded color).
        let fill = probe_color(&theme(), "color.background.surface");
        assert!(
            tree.has_background(fill, 0.01),
            "panel fill resolved from token"
        );
    }

    #[test]
    fn size_ladder_scales_padding_and_typography() {
        // xs and xl must produce different resolved title font sizes.
        let base = PasswordRequirementsSpec::new()
            .with_requirements(policy_all())
            .with_password("Test1!");
        let xs = base.clone().with_size(ControlSize::Xs);
        let xl = base.clone().with_size(ControlSize::Xl);

        let xs_title = rem_to_px(PasswordRequirementsSpec::title_size_rem(ControlSize::Xs));
        let xl_title = rem_to_px(PasswordRequirementsSpec::title_size_rem(ControlSize::Xl));
        assert!(xl_title > xs_title);

        let xs_tree = probe(&js_password_requirements(&xs, &theme()), 360.0, 320.0);
        let xl_tree = probe(&js_password_requirements(&xl, &theme()), 360.0, 320.0);
        let xs_title_node = xs_tree
            .nodes
            .iter()
            .find(|n| n.text.as_deref() == Some("Password requirements:"))
            .expect("xs title present");
        let xl_title_node = xl_tree
            .nodes
            .iter()
            .find(|n| n.text.as_deref() == Some("Password requirements:"))
            .expect("xl title present");
        assert_eq!(xs_title_node.text_size, Some(xs_title));
        assert_eq!(xl_title_node.text_size, Some(xl_title));
    }

    #[test]
    fn all_requirements_met_logic() {
        let spec = PasswordRequirementsSpec::new()
            .with_requirements(policy_all())
            .with_password("StrongPass1!");
        assert!(spec.all_met());
    }
}
