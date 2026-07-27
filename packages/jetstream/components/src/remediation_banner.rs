//! RemediationBanner — Jetstream announcing recovery banner backed by RemediationBannerSpec.
//!
//! Contract: `docs/contracts/components/remediation-banner.md`
//! Reference: Svelte component NOT YET BUILT — the contract is the sole
//! authority (see `docs/parity/remediation-banner.md`). GPUI mirrors the same
//! contract.
//!
//! Anatomy (contract §2):
//! ```text
//! [Root .remediation-banner]
//!   ├── [Icon]      tone-based leading indicator
//!   ├── [Content]   Title (<strong>) + Message (<p>) + Actions row
//!   │     └── [Actions]  Primary + optional Secondary RemediationAction buttons
//!   └── [Dismiss]   optional close button
//! ```
//!
//! ALL dimensions resolve from tokens or contract-exact rem via `rem_to_px`.
//! ZERO hardcoded colors. The announce role (`spec.accessibility_role()`) has no
//! JsEl accessibility channel; hosts read the spec to apply `role`/`aria-live`.

use jetstream_ui::Color;
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::ButtonSpec;
use poodle_specs::RemediationAction;
use poodle_specs::RemediationBannerSpec;

use crate::button::js_button;
use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub fn js_remediation_banner(spec: &RemediationBannerSpec, theme: &JetstreamThemeProvider) -> JsEl {
    // ── Colors (all token-resolved) ──
    let tone_color: Color = resolve_color(theme, spec.border_token()).into();
    let panel: Color = resolve_color(theme, spec.background_token()).into();
    let icon_color = resolve_color(theme, spec.icon_color_token());
    let text_primary = resolve_color(theme, spec.title_color_token());
    let text_secondary = resolve_color(theme, spec.message_color_token());

    // Surface fill = color-mix(tone, panel) at the spec's tone ratio; border = tone.
    let fill = tone_color.mix_srgb(panel, spec.fill_tone_ratio());
    let border = resolve_color(theme, spec.border_token());

    // ── Dimensions ──
    // Radius / border-width resolve from tokens; the rest are contract-exact
    // rem values fed to `rem_to_px` (no semantic token exists for them).
    let radius = resolve_radius(theme, spec.radius_token());
    let border_width = resolve_px(theme, spec.border_width_token());
    // Typography: title at body size, message at label size (contract §2 maps
    // Content typography; resolved from tokens, not literals).
    let title_size = resolve_px(theme, "typography.body.size");
    let message_size = resolve_px(theme, "typography.label.size");
    let icon_size = rem_to_px(1.25); // note: contract icon size, no token
    let pad_x = resolve_px(theme, "space.panel.x");
    let pad_y = resolve_px(theme, "space.panel.y");
    let gap = resolve_px(theme, "space.inline.md"); // root row gap (note: approx)
    let content_gap = resolve_px(theme, "space.inline.xs"); // title↔message↔actions
    let action_gap = resolve_px(theme, "space.inline.sm"); // between action buttons
    let dismiss_size = rem_to_px(1.0); // note: contract dismiss size, no token

    let mut el = ui_element::div()
        .bg(fill)
        .border(border_width)
        .border_color(border)
        .rounded(radius)
        .pl(pad_x)
        .pr(pad_x)
        .pt(pad_y)
        .pb(pad_y)
        .flex_row()
        .items_start()
        .gap(gap);

    // Note: ARIA role from spec.accessibility_role() applied by host runtime
    // (JsEl has no accessibility metadata channel).

    // ── Icon (contract §2: tone-based leading indicator) ──
    el = el.child(
        ui_element::icon(spec.tone_icon_name())
            .w(icon_size)
            .h(icon_size)
            .text_color(icon_color),
    );

    // ── Content column: Title + Message + Actions ──
    let mut content = ui_element::div()
        .flex_col()
        .gap(content_gap)
        .grow()
        .min_w_0();

    content = content.child(
        ui_element::label(&spec.title)
            .text_color(text_primary)
            .text_size(title_size)
            .text_weight(600),
    );

    content = content.child(
        ui_element::label(&spec.message)
            .text_color(text_secondary)
            .text_size(message_size),
    );

    // ── Actions row: real buttons honoring variant + is_disabled ──
    if spec.action_count() > 0 {
        let mut actions_row = ui_element::div()
            .flex_row()
            .items_center()
            .gap(action_gap)
            .pt(content_gap);

        if let Some(ref primary) = spec.primary_action {
            actions_row = actions_row.child(action_button(primary, theme));
        }
        if let Some(ref secondary) = spec.secondary_action {
            actions_row = actions_row.child(action_button(secondary, theme));
        }

        content = content.child(actions_row);
    }

    el = el.child(content);

    // ── Dismiss (contract §5: aria-label="Dismiss"; no JsEl aria channel) ──
    if spec.is_dismissible {
        el = el.child(
            ui_element::icon("x")
                .id("remediation-banner-dismiss")
                .w(dismiss_size)
                .h(dismiss_size)
                .text_color(text_secondary)
                .cursor_pointer(),
        );
    }

    el.aria_role(jetstream_ui::accesskit::Role::Alert)
}

/// Build a real `js_button` for a `RemediationAction`, honoring its variant and
/// disabled flag (contract §2: RemediationAction button; §3 `is_disabled`).
/// `js_button` already applies the disabled-opacity token, so the disabled tint
/// is handled there.
fn action_button(action: &RemediationAction, theme: &JetstreamThemeProvider) -> JsEl {
    js_button(
        &ButtonSpec::new()
            .with_variant(action.variant)
            .with_label(action.label.clone())
            .with_disabled(action.is_disabled),
        theme,
    )
    .id(format!("remediation-action-{}", action.id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::ButtonVariant;
    use poodle_specs::StatusTone;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn renders_icon_title_message() {
        let th = theme();
        let el = js_remediation_banner(
            &RemediationBannerSpec::new("Review needed", "Resolve the blocking error."),
            &th,
        );
        let tree = probe(&el, 480.0, 200.0);
        assert!(!tree.is_empty(), "probe produced no nodes");
        // Tone icon (default warning → alert-triangle).
        assert!(
            tree.has_text("alert-triangle"),
            "tone icon missing: {:?}",
            tree.texts()
        );
        assert!(tree.has_text("Review needed"), "title missing: {:?}", tree.texts());
        assert!(
            tree.has_text("Resolve the blocking error."),
            "message missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn primary_and_secondary_actions_render_as_buttons() {
        let th = theme();
        let el = js_remediation_banner(
            &RemediationBannerSpec::new("Title", "Message")
                .with_primary_action(
                    RemediationAction::new("resolve", "Resolve")
                        .with_variant(ButtonVariant::Primary),
                )
                .with_secondary_action(RemediationAction::new("inspect", "Inspect")),
            &th,
        );
        let tree = probe(&el, 480.0, 200.0);
        // Real Button widgets (not bare labels) carry the action labels.
        assert!(tree.count_kind("Button") >= 2, "expected 2 action buttons");
        assert!(tree.has_text("Resolve"), "primary label missing: {:?}", tree.texts());
        assert!(tree.has_text("Inspect"), "secondary label missing: {:?}", tree.texts());
        assert!(
            tree.find_token("remediation-action-resolve").is_some(),
            "primary action id missing"
        );
    }

    #[test]
    fn disabled_action_reduces_opacity() {
        let th = theme();
        let action = RemediationAction::new("retry", "Retry").with_disabled(true);
        let el = action_button(&action, &th);
        assert!(el.style.opacity < 1.0, "disabled action should dim, got {}", el.style.opacity);
    }

    #[test]
    fn dismiss_control_present_when_dismissible() {
        let th = theme();
        let el = js_remediation_banner(
            &RemediationBannerSpec::new("T", "M").with_dismissible(true),
            &th,
        );
        let tree = probe(&el, 480.0, 200.0);
        assert!(
            tree.find_token("remediation-banner-dismiss").is_some(),
            "dismiss control missing"
        );
        assert!(tree.has_text("x"), "dismiss glyph missing: {:?}", tree.texts());
    }

    #[test]
    fn tone_drives_border_and_icon() {
        let th = theme();
        let danger_color: Color = resolve_color(&th, "color.status.danger").into();
        let el = js_remediation_banner(
            &RemediationBannerSpec::new("Critical", "Pipeline failed").with_tone(StatusTone::Danger),
            &th,
        );
        // Danger tone → x-circle icon + status-danger border.
        let tree = probe(&el, 480.0, 200.0);
        assert!(tree.has_text("x-circle"), "danger icon missing: {:?}", tree.texts());
        let border = el.style.border_color.expect("border color set");
        assert!(
            (border.r - danger_color.r).abs() < 0.02 && (border.g - danger_color.g).abs() < 0.02,
            "danger border should be status-danger, got {border:?}"
        );
    }

    #[test]
    fn no_actions_omits_action_row() {
        let th = theme();
        let el = js_remediation_banner(
            &RemediationBannerSpec::new("Confirmed", "All good").with_tone(StatusTone::Success),
            &th,
        );
        let tree = probe(&el, 480.0, 200.0);
        assert_eq!(tree.count_kind("Button"), 0, "no actions → no buttons");
        // Success tone icon present.
        assert!(tree.has_text("check-circle"), "success icon missing: {:?}", tree.texts());
    }
}
