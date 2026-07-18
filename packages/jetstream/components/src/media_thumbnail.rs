//! MediaThumbnail — Jetstream media thumbnail backed by MediaThumbnailSpec.
//!
//! Renders the framed preview surface with a ratio-derived frame, fallback icon
//! placeholder, optional badge + play indicator, loading/error/empty posture
//! (with the shared grid spinner on loading), and a non-compact caption.
//!
//! Notes:
//! - Jetstream has no image decode, so the `ready` frame always shows the
//!   token-resolved placeholder/fallback icon (never a real bitmap).
//! - JsEl has no CSS `aspect-ratio`; the height is computed from the aspect
//!   ratio against a reference frame width (`FRAME_FILL_REF_REM` / `FRAME_XL_REM`)
//!   or the explicit pixel width.
//! - The contract frame background is a radial+panel gradient; JsEl paints the
//!   flat panel base layer here (gradient = preview-loop).
use jetstream_ui::Color;
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{MediaFrameWidth, MediaState, MediaThumbnailSpec};
use poodle_specs::{SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant};

use crate::presentation::rem_to_px;
use crate::spinner::js_spinner;
use crate::theme_ext::{resolve_color, resolve_radius};

const FRAME_FILL_REF_REM: f32 = 20.0;
const FRAME_XL_REM: f32 = 24.0;

pub fn js_media_thumbnail(spec: &MediaThumbnailSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let panel = resolve_color(theme, spec.frame_panel_token());
    let border = resolve_color(theme, spec.frame_border_token());
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let placeholder_color = resolve_color(theme, spec.placeholder_icon_token());
    let surface = resolve_color(theme, "color.background.surface");
    let elevated = resolve_color(theme, "color.background.elevated");

    // Frame radius: calc(radius.surface − 0.125rem).
    let frame_radius =
        (resolve_radius(theme, spec.frame_radius_token()) - rem_to_px(spec.frame_radius_inset_rem()))
            .max(0.0);
    let badge_radius = resolve_radius(theme, spec.badge_radius_token());
    let play_radius = resolve_radius(theme, spec.play_radius_token());

    let body_size = rem_to_px(0.875);
    let label_size = rem_to_px(0.8125);
    let caption_title_size = rem_to_px(0.875);
    let caption_meta_size = rem_to_px(0.8125);
    let badge_text_size = rem_to_px(0.6875);

    // Frame width / height (contract §9 Aspect Ratios).
    let (ref_width_px, explicit_width) = match spec.frame_width {
        MediaFrameWidth::Fill => (rem_to_px(FRAME_FILL_REF_REM), None),
        MediaFrameWidth::Xl => (rem_to_px(FRAME_XL_REM), None),
        MediaFrameWidth::Px(w) => (w, Some(w)),
    };
    let derived_h = spec.frame_height_for_width(ref_width_px);
    let frame_h = spec
        .frame_min_height
        .map(|m| m.max(derived_h))
        .unwrap_or(derived_h);
    let frame_h = spec
        .frame_max_height
        .map(|m| frame_h.min(m))
        .unwrap_or(frame_h);

    // ── Frame ──────────────────────────────────────────────────
    let mut frame = ui_element::div()
        .bg(panel)
        .border(1.0)
        .border_color(border)
        .rounded(frame_radius)
        .h(frame_h)
        .relative()
        .overflow_hidden()
        .flex_col()
        .items_center()
        .justify_center();
    frame = match explicit_width {
        Some(w) => frame.w(w),
        None => frame.w(ref_width_px).grow(),
    };

    if spec.shows_fallback_copy() {
        // ── State display (loading / error / empty) ────────────
        let state_bg = Color::new(surface.x, surface.y, surface.z, surface.w * 0.78);
        let mut state = ui_element::div()
            .bg(state_bg)
            .grow()
            .h_full()
            .flex_col()
            .gap(rem_to_px(0.5));
        state = if spec.is_compact() {
            state
                .items_center()
                .justify_center()
                .pl(rem_to_px(0.875))
                .pr(rem_to_px(0.875))
                .pt(rem_to_px(0.875))
                .pb(rem_to_px(0.875))
        } else {
            state
                .items_start()
                .justify_end()
                .pl(rem_to_px(0.875))
                .pr(rem_to_px(0.875))
                .pt(rem_to_px(0.875))
                .pb(rem_to_px(0.875))
        };

        if spec.state == MediaState::Loading {
            state = state.child(js_spinner(
                &SpinnerSpec::new()
                    .with_variant(SpinnerVariant::Grid)
                    .with_size(if spec.is_compact() {
                        SpinnerSize::Sm
                    } else {
                        SpinnerSize::Md
                    })
                    .with_tone(SpinnerTone::Accent),
                theme,
            ));
        }

        state = state.child(
            ui_element::label(spec.resolved_state_title())
                .text_color(text_primary)
                .text_size(body_size)
                .text_weight(600),
        );

        if spec.state_message_visible() {
            state = state.child(
                ui_element::label(spec.resolved_state_message().unwrap())
                    .text_color(text_secondary)
                    .text_size(label_size),
            );
        }

        frame = frame.child(state);
    } else {
        // ── Placeholder fallback icon (contract §9) ────────────
        frame = frame.child(
            ui_element::icon(spec.fallback_icon())
                .w(rem_to_px(1.75))
                .h(rem_to_px(1.75))
                .text_color(placeholder_color),
        );

        // ── Play indicator (audio/video, contract §3) ─────────
        if spec.shows_play_indicator() {
            let play_bg = Color::new(elevated.x, elevated.y, elevated.z, elevated.w * 0.78);
            let play_color = resolve_color(theme, spec.play_color_token());
            frame = frame.child(
                ui_element::div()
                    .absolute()
                    .left(rem_to_px(0.625))
                    .bottom(rem_to_px(0.625))
                    .w(rem_to_px(2.0))
                    .h(rem_to_px(2.0))
                    .rounded(play_radius)
                    .bg(play_bg)
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .child(
                        ui_element::icon(spec.play_indicator_icon())
                            .w(rem_to_px(0.9375))
                            .h(rem_to_px(0.9375))
                            .text_color(play_color),
                    ),
            );
        }
    }

    // ── Badge overlay (contract §9) ────────────────────────────
    if let Some(ref badge_label) = spec.badge_label {
        let badge_bg = Color::new(surface.x, surface.y, surface.z, surface.w * 0.74);
        let badge_color = resolve_color(theme, spec.badge_text_token());
        let inset = if spec.is_compact() { 0.5 } else { 0.625 };
        frame = frame.child(
            ui_element::div()
                .absolute()
                .top(rem_to_px(inset))
                .right(rem_to_px(inset))
                .h(rem_to_px(1.5))
                .pl(rem_to_px(0.625))
                .pr(rem_to_px(0.625))
                .rounded(badge_radius)
                .bg(badge_bg)
                .flex_col()
                .items_center()
                .justify_center()
                .child(
                    ui_element::label(&badge_label.to_uppercase())
                        .text_color(badge_color)
                        .text_size(badge_text_size)
                        .text_weight(600)
                        .letter_spacing_em(0.04), // contract badge: letter-spacing 0.04em
                ),
        );
    }

    // ── Root: frame + caption ──────────────────────────────────
    let mut root = ui_element::div().flex_col().overflow_hidden();
    root = if spec.is_compact() {
        root
    } else {
        root.gap(rem_to_px(0.5))
    };
    root = root.child(frame);

    if spec.caption_visible() {
        let mut caption = ui_element::div().flex_col().gap(rem_to_px(0.125));
        if let Some(ref title) = spec.title {
            caption = caption.child(
                ui_element::label(title)
                    .text_color(text_primary)
                    .text_size(caption_title_size)
                    .text_weight(500),
            );
        }
        if let Some(ref meta) = spec.meta {
            caption = caption.child(
                ui_element::label(meta)
                    .text_color(text_secondary)
                    .text_size(caption_meta_size),
            );
        }
        root = root.child(caption);
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{AspectRatio, MediaKind, MediaPresentation};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn renders_placeholder_icon_and_caption() {
        let th = theme();
        let spec = MediaThumbnailSpec::new(MediaKind::Image)
            .with_title("Photo 1")
            .with_meta("2.4 MB")
            .with_aspect_ratio(AspectRatio::Square);
        let tree = probe(&js_media_thumbnail(&spec, &th), 400.0, 400.0);

        assert!(!tree.is_empty());
        // Fallback icon present (contract §9 image → "image").
        assert!(
            tree.texts().contains(&"image"),
            "fallback icon missing: {:?}",
            tree.texts()
        );
        // Caption title + meta render below the frame.
        assert!(tree.has_text("Photo 1"), "title missing: {:?}", tree.texts());
        assert!(tree.has_text("2.4 MB"), "meta missing: {:?}", tree.texts());
        assert!(tree.count_kind("Icon") >= 1);
    }

    #[test]
    fn renders_badge_uppercased() {
        let th = theme();
        let spec = MediaThumbnailSpec::new(MediaKind::Image)
            .with_badge_label("New")
            .with_aspect_ratio(AspectRatio::Square);
        let tree = probe(&js_media_thumbnail(&spec, &th), 400.0, 400.0);
        assert!(
            tree.has_text("NEW"),
            "uppercased badge missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn video_renders_play_indicator() {
        let th = theme();
        let spec = MediaThumbnailSpec::new(MediaKind::Video).with_aspect_ratio(AspectRatio::Video);
        let tree = probe(&js_media_thumbnail(&spec, &th), 400.0, 400.0);
        // Fallback icon "play" + play-indicator overlay icon "play" ⇒ ≥ 2 icons.
        assert!(
            tree.count_kind("Icon") >= 2,
            "expected fallback + play indicator icons, got {}",
            tree.count_kind("Icon")
        );
    }

    #[test]
    fn loading_state_renders_spinner_and_title() {
        let th = theme();
        let spec = MediaThumbnailSpec::new(MediaKind::Image)
            .with_state(MediaState::Loading)
            .with_state_message("Preparing preview.")
            .with_aspect_ratio(AspectRatio::Square);
        let tree = probe(&js_media_thumbnail(&spec, &th), 400.0, 400.0);
        assert!(
            tree.has_text("Loading preview"),
            "state title missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Preparing preview."),
            "state message missing: {:?}",
            tree.texts()
        );
        // Grid spinner emits multiple Panel cells; tree must be non-trivial.
        assert!(tree.len() > 3, "loading tree too small: {}", tree.len());
    }

    #[test]
    fn error_state_uses_default_title() {
        let th = theme();
        let spec = MediaThumbnailSpec::new(MediaKind::Document)
            .with_state(MediaState::Error)
            .with_aspect_ratio(AspectRatio::Landscape);
        let tree = probe(&js_media_thumbnail(&spec, &th), 400.0, 400.0);
        assert!(
            tree.has_text("Preview unavailable"),
            "error title missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn compact_hides_caption_and_message() {
        let th = theme();
        let spec = MediaThumbnailSpec::new(MediaKind::Document)
            .with_title("Report.pdf")
            .with_presentation(MediaPresentation::Compact)
            .with_aspect_ratio(AspectRatio::Landscape);
        let tree = probe(&js_media_thumbnail(&spec, &th), 400.0, 400.0);
        // Caption title is hidden in compact presentation.
        assert!(
            !tree.has_text("Report.pdf"),
            "compact should hide caption title: {:?}",
            tree.texts()
        );
    }
}
