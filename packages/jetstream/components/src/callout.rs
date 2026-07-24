//! Callout — Jetstream callout/alert component backed by CallOutSpec.
//!
//! Contract: `docs/contracts/components/callout.md`
//! Reference: `packages/svelte/components/src/Callout.svelte`
//!
//! Resolves tone fill/border via `Color::mix` and renders the leading icon
//! badge, optional dismiss control, and all six tones (incl. neutral + pending).

use jetstream_ui::Color;
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{CallOutSpec, SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant, StatusTone};

use crate::presentation::{
    panel_space_x_rem, rem_to_px, resolve_semantic_size, resolve_supporting_visual_size,
    size_font_rem,
};
use crate::spinner::js_spinner;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// Map a status tone to its icon name (contract §6 icon map).
/// Pending is handled separately by a ring spinner, not an icon.
fn tone_icon(tone: StatusTone) -> &'static str {
    match tone {
        StatusTone::Neutral | StatusTone::Info => "info",
        StatusTone::Success => "check",
        StatusTone::Warning => "triangle-alert",
        StatusTone::Danger => "circle-x",
        StatusTone::Pending => "info", // unused: pending renders a spinner
    }
}

pub fn js_callout(spec: &CallOutSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    // Supporting (icon) visual size — used for the glyph inside the badge.
    let icon_glyph = rem_to_px(size_font_rem(resolve_supporting_visual_size(effective_size)));

    // Padding: contract §8 root = 0.625rem block / space.panel.x inline.
    let pad_x = rem_to_px(panel_space_x_rem(spec.density));
    let pad_y = rem_to_px(0.625);
    // Gaps: outer = space.inline.md, content = space.inline.sm (token-resolved).
    let gap = resolve_px(theme, "space.inline.md");
    let content_gap = resolve_px(theme, "space.inline.sm");

    // Tone base color (info → dedicated status-info; pending → accent;
    // neutral → text-secondary) via the spec, so info no longer tints accent.
    let tone_color: Color = resolve_color(theme, spec.tone_color_token()).into();
    let radius = resolve_radius(theme, "radius.surface");
    let border_width = resolve_px(theme, spec.border_width_token());
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let panel: Color = resolve_color(theme, spec.fill_mix_target_token()).into();
    let border_default: Color = resolve_color(theme, spec.border_mix_target_token()).into();
    let border_subtle = resolve_color(theme, spec.neutral_border_token());
    let surface: Color = resolve_color(theme, spec.icon_badge_token()).into();

    // Per-tone fill/border (contract §8):
    //   neutral: panel 94% / border-subtle 88% (mixed toward transparent)
    //   pending: accent 8% / 26% over panel / border-default
    //   info/success/warning/danger: status 10% / 34% over panel / border-default
    let is_neutral = spec.is_neutral_tone();
    let is_pending = spec.is_pending_tone();
    let fill = if is_neutral {
        Color::new(panel.r, panel.g, panel.b, panel.a * 0.94)
    } else if is_pending {
        tone_color.mix_srgb(panel, 0.08)
    } else {
        tone_color.mix_srgb(panel, 0.10)
    };
    let border = if is_neutral {
        Color::new(border_subtle.x, border_subtle.y, border_subtle.z, border_subtle.w * 0.88)
    } else if is_pending {
        tone_color.mix_srgb(border_default, 0.26)
    } else {
        tone_color.mix_srgb(border_default, 0.34)
    };

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
        .items_center()
        .gap(gap);

    // ── Body: icon badge + content ───────────────────────────
    let mut body = ui_element::div()
        .flex_row()
        .items_start()
        .gap(gap)
        .grow();

    // Circular icon badge — 1.375rem, surface at 78%, 999px radius (contract §8).
    let badge_size = rem_to_px(1.375);
    let badge_bg = Color::new(surface.r, surface.g, surface.b, surface.a * 0.78);
    let mut badge = ui_element::div()
        .w(badge_size)
        .h(badge_size)
        .rounded(999.0)
        .bg(badge_bg)
        .flex_row()
        .items_center()
        .justify_center();
    if is_pending {
        // Pending hosts the shared ring spinner (accent) instead of a tone icon.
        badge = badge.child(js_spinner(
            &SpinnerSpec::new()
                .with_variant(SpinnerVariant::Ring)
                .with_size(SpinnerSize::Sm)
                .with_tone(SpinnerTone::Accent),
            theme,
        ));
    } else {
        badge = badge.child(
            ui_element::icon(tone_icon(spec.tone))
                .w(icon_glyph)
                .h(icon_glyph)
                .text_color(tone_color),
        );
    }
    body = body.child(badge);

    // Content column — title + message, gap = space.inline.sm.
    let mut content = ui_element::div().flex_col().gap(content_gap).grow();
    if let Some(ref title) = spec.title {
        content = content.child(
            ui_element::label(title)
                .text_color(text_primary)
                .text_size(font_size)
                // Title uses the label typography weight (semibold).
                .text_weight(600),
        );
    }
    if let Some(ref message) = spec.content {
        content = content.child(
            ui_element::label(message)
                .text_color(text_secondary)
                .text_size(font_size),
        );
    }
    body = body.child(content);
    el = el.child(body);

    // ── Dismiss control (ghost "x") ──────────────────────────
    if spec.dismissible {
        let dismiss_size = rem_to_px(1.75);
        let control_radius = resolve_radius(theme, "radius.control");
        // Contract: border-radius = control - 0.0625rem (border-width token).
        let dismiss_radius = (control_radius - border_width).max(0.0);
        el = el.child(
            ui_element::div()
                .id("poodle-callout-dismiss")
                .w(dismiss_size)
                .h(dismiss_size)
                .rounded(dismiss_radius)
                .flex_row()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .child(
                    ui_element::icon("x")
                        .w(icon_glyph)
                        .h(icon_glyph)
                        .text_color(text_secondary),
                ),
        );
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::ControlDensity;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn renders_icon_badge_and_title() {
        let spec = CallOutSpec::new()
            .with_tone(StatusTone::Info)
            .with_title("Information")
            .with_content("Body text.");
        let tree = probe(&js_callout(&spec, &theme()), 400.0, 200.0);
        assert!(!tree.is_empty(), "callout produced no nodes");
        assert!(tree.has_text("Information"), "title missing: {:?}", tree.texts());
        // Icon badge present: an Icon node carrying the tone icon name.
        assert!(
            tree.count_kind("Icon") >= 1,
            "no icon badge rendered: {:?}",
            tree.texts()
        );
        assert!(tree.has_text("info"), "info tone icon missing: {:?}", tree.texts());
    }

    #[test]
    fn dismissible_renders_x_control() {
        let plain = CallOutSpec::new().with_tone(StatusTone::Info).with_title("T");
        let dismissible = plain.clone().dismissible(true);
        let plain_tree = probe(&js_callout(&plain, &theme()), 400.0, 200.0);
        let dismiss_tree = probe(&js_callout(&dismissible, &theme()), 400.0, 200.0);

        // The dismiss control carries the interaction id and an "x" icon.
        assert!(
            dismiss_tree.find_token("poodle-callout-dismiss").is_some(),
            "dismiss control missing when dismissible"
        );
        assert!(dismiss_tree.has_text("x"), "x icon missing: {:?}", dismiss_tree.texts());
        // Non-dismissible callout has neither.
        assert!(
            plain_tree.find_token("poodle-callout-dismiss").is_none(),
            "dismiss control present when not dismissible"
        );
    }

    #[test]
    fn neutral_and_pending_tones_render() {
        let neutral = CallOutSpec::new().with_tone(StatusTone::Neutral).with_title("N");
        let pending = CallOutSpec::new().with_tone(StatusTone::Pending).with_title("P");
        let neutral_tree = probe(&js_callout(&neutral, &theme()), 400.0, 200.0);
        let pending_tree = probe(&js_callout(&pending, &theme()), 400.0, 200.0);

        // Both render a non-empty tree with their title and an icon badge container.
        assert!(neutral_tree.has_text("N"), "neutral title missing");
        assert!(pending_tree.has_text("P"), "pending title missing");
        // Neutral uses the info icon glyph; pending renders a spinner (no tone icon),
        // so its tree still has more than the bare root.
        assert!(neutral_tree.has_text("info"), "neutral info icon missing");
        assert!(pending_tree.len() > 3, "pending spinner badge not rendered");
    }

    #[test]
    fn neutral_fill_differs_from_toned_fill() {
        // Neutral fill is panel-derived; danger fill is a status mix — distinct bg.
        let neutral = CallOutSpec::new().with_tone(StatusTone::Neutral);
        let danger = CallOutSpec::new().with_tone(StatusTone::Danger);
        let n = probe(&js_callout(&neutral, &theme()), 400.0, 200.0);
        let d = probe(&js_callout(&danger, &theme()), 400.0, 200.0);
        let n_bg = n.nodes[0].bg.expect("neutral root has no bg");
        let d_bg = d.nodes[0].bg.expect("danger root has no bg");
        assert!(!n_bg.approx(d_bg, 0.01), "neutral and danger fills should differ");
    }

    #[test]
    fn density_changes_inline_padding() {
        // Density drives inline padding (panel.x): compact < default < comfortable.
        let compact = CallOutSpec::new().with_density(ControlDensity::Compact);
        let default = CallOutSpec::new().with_density(ControlDensity::Default);
        let comfortable = CallOutSpec::new().with_density(ControlDensity::Comfortable);
        let cx = probe(&js_callout(&compact, &theme()), 400.0, 200.0).nodes[0].x;
        let dx = probe(&js_callout(&default, &theme()), 400.0, 200.0).nodes[0].x;
        // Inner content x-offset reflects inline padding; compare body child x.
        let c_body = probe(&js_callout(&compact, &theme()), 400.0, 200.0).nodes[1].x;
        let d_body = probe(&js_callout(&default, &theme()), 400.0, 200.0).nodes[1].x;
        let cmf_body = probe(&js_callout(&comfortable, &theme()), 400.0, 200.0).nodes[1].x;
        let _ = (cx, dx);
        assert!(c_body < d_body, "compact padding not tighter than default");
        assert!(d_body < cmf_body, "comfortable padding not looser than default");
    }
}
