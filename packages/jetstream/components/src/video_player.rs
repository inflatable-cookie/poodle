//! VideoPlayer — Jetstream video player backed by VideoPlayerSpec.
//!
//! Contract: `docs/contracts/components/video-player.md`
//! Reference: `packages/svelte/components/src/VideoPlayer.svelte`
//!
//! Renders the player chrome only (Jetstream has no media decode). Per contract
//! §8 the player is white-on-black regardless of theme — only the surface radius
//! comes from a token; all geometry resolves from the `VideoPlayerSpec` size /
//! density rem ladders (single source of truth shared with the GPUI impl).
//!
//! # Known deltas / notes
//! - No real playback / fullscreen / auto-hide. Interaction (play/seek/mute/
//!   fullscreen, Space/Enter) lives in the preview event loop; buttons are
//!   `.focusable()`.
//! - No ARIA channel (Jetstream has no a11y tree); aria-labels are carried as the
//!   transport buttons' fallback text alongside the icon glyph.

use jetstream_ui::ui_element::{self, FontFamily, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::VideoPlayerSpec;

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::resolve_radius;

/// Format seconds as m:ss (contract `.video-player__time`).
fn format_time(seconds: f64) -> String {
    let total = seconds.max(0.0) as u64;
    format!("{}:{:02}", total / 60, total % 60)
}

pub fn js_video_player(spec: &VideoPlayerSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let radius = resolve_radius(theme, "radius.surface");

    // ── Size/density geometry (contract §7/§8 rem ladders via spec helpers) ─
    let btn_size = rem_to_px(VideoPlayerSpec::button_size_rem(effective_size));
    let icon_size = rem_to_px(VideoPlayerSpec::icon_size_rem(effective_size));
    let volume_width = rem_to_px(VideoPlayerSpec::volume_width_rem(effective_size));
    let time_font = rem_to_px(VideoPlayerSpec::time_font_rem(effective_size));
    let big_play_size = rem_to_px(VideoPlayerSpec::big_play_size_rem(effective_size));
    let track_height = rem_to_px(VideoPlayerSpec::track_height_rem());
    let volume_thumb = rem_to_px(VideoPlayerSpec::volume_thumb_rem());
    let bar_gap = rem_to_px(VideoPlayerSpec::bar_gap_rem(spec.density));
    let pill = VideoPlayerSpec::pill_radius_rem();

    // ── Fixed colors (contract §8: white-on-black regardless of theme) ──────
    let black = glam::Vec4::new(0.0, 0.0, 0.0, 1.0);
    let white = glam::Vec4::new(1.0, 1.0, 1.0, 1.0);
    let white_90 = glam::Vec4::new(1.0, 1.0, 1.0, 0.9);
    let white_80 = glam::Vec4::new(1.0, 1.0, 1.0, 0.8);
    let white_50 = glam::Vec4::new(1.0, 1.0, 1.0, 0.5);
    let white_20 = glam::Vec4::new(1.0, 1.0, 1.0, 0.2);
    let overlay = glam::Vec4::new(0.0, 0.0, 0.0, 0.7);
    // Note: the contract progress fill is `color.accent.base`; the runtime
    // ProgressBar widget owns its own fill color, so the seek bar relies on the
    // widget's default accent fill rather than an explicit token here.

    // ── Transport icon button helper (real Icon glyph + square wrapper) ─────
    let transport = |name: &'static str| -> JsEl {
        ui_element::button("")
            .w(btn_size)
            .h(btn_size)
            .flex_row()
            .items_center()
            .justify_center()
            .focusable()
            .cursor_pointer()
            .child(
                ui_element::icon(name)
                    .w(icon_size)
                    .h(icon_size)
                    .text_color(white_90),
            )
    };

    // ── Root container — black surface, token radius ───────────────────────
    let mut el = ui_element::div()
        .bg(black)
        .rounded(radius)
        .overflow_hidden()
        .flex_col()
        .self_stretch();

    // ── Video area (placeholder — no native video) ─────────────────────────
    let mut video_area = ui_element::div()
        .bg(black)
        .grow()
        .items_center()
        .justify_center()
        .flex_col();

    // Big play button — transparent ring, only when paused at currentTime=0.
    if !spec.is_playing && spec.current_time <= 0.0 {
        video_area = video_area.child(
            ui_element::button("")
                .w(big_play_size)
                .h(big_play_size)
                .rounded(pill)
                .flex_row()
                .items_center()
                .justify_center()
                .focusable()
                .cursor_pointer()
                .child(
                    ui_element::icon("play")
                        .w(big_play_size * 0.5)
                        .h(big_play_size * 0.5)
                        .text_color(white_90),
                ),
        );
    }

    el = el.child(video_area);

    // ── Controls overlay ───────────────────────────────────────────────────
    let mut controls = ui_element::div().bg(overlay).flex_col();

    // Progress / seek bar — proportional fill via the runtime ProgressBar widget.
    let progress = spec.progress();
    let progress_bar = ui_element::progress(progress as f32)
        .min_h(track_height)
        .self_stretch()
        .mb(rem_to_px(0.375))
        .rounded(pill)
        .bg(white_20);
    controls = controls.child(progress_bar);

    // Control bar.
    let mut bar = ui_element::div().flex_row().items_center().gap(bar_gap);

    // Play/pause (icon swaps).
    bar = bar.child(transport(if spec.is_playing { "pause" } else { "play" }));

    // Mute (volume == 0 → muted glyph).
    bar = bar.child(transport(if spec.volume <= 0.0 {
        "volume-x"
    } else {
        "volume-2"
    }));

    // Volume slider — track with a thumb positioned by volume fraction.
    // Lay out as [filled-track | thumb | rest-track] across a fixed width so the
    // thumb sits proportionally without absolute positioning (chrome only).
    let vol_frac = (spec.volume as f32).clamp(0.0, 1.0);
    let usable = (volume_width - volume_thumb).max(0.0);
    let filled_w = usable * vol_frac;
    let rest_w = usable - filled_w;
    let volume_slider = ui_element::div()
        .flex_row()
        .items_center()
        .w(volume_width)
        .child(
            ui_element::div()
                .w(filled_w)
                .min_h(track_height)
                .rounded(rem_to_px(0.125))
                .bg(white_50),
        )
        .child(
            ui_element::div()
                .w(volume_thumb)
                .h(volume_thumb)
                .rounded(pill)
                .bg(white),
        )
        .child(
            ui_element::div()
                .w(rest_w)
                .min_h(track_height)
                .rounded(rem_to_px(0.125))
                .bg(white_50),
        );
    bar = bar.child(volume_slider);

    // Time display (m:ss / m:ss).
    let time_text = format!(
        "{} / {}",
        format_time(spec.current_time),
        format_time(spec.duration)
    );
    // Time display — monospace (contract §8 `.video-player__time`).
    bar = bar.child(
        ui_element::label(&time_text)
            .text_color(white_80)
            .text_size(time_font)
            .font_family(FontFamily::Mono),
    );

    // Spacer pushes fullscreen to the right edge.
    bar = bar.child(ui_element::div().grow());

    // Fullscreen (icon swaps).
    bar = bar.child(transport(if spec.is_fullscreen {
        "minimize-2"
    } else {
        "maximize-2"
    }));

    controls = controls.child(bar);
    el = el.child(controls);

    crate::aria::with_aria_label(el, spec.aria_label.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{ControlSize, VideoPlayerSpec};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    /// Transport controls render real Icon glyphs (not text labels).
    #[test]
    fn renders_transport_icons() {
        let th = theme();
        let spec = VideoPlayerSpec::new("clip.mp4").with_playing(true);
        let tree = probe(&js_video_player(&spec, &th), 480.0, 270.0);
        // Playing → pause glyph; volume-2 (unmuted); maximize-2 (windowed).
        assert!(tree.has_text("pause"), "missing pause icon; icons: {:?}", tree.texts());
        assert!(tree.has_text("volume-2"), "missing volume icon; icons: {:?}", tree.texts());
        assert!(tree.has_text("maximize-2"), "missing fullscreen icon; icons: {:?}", tree.texts());
        assert!(tree.count_kind("Icon") >= 3, "expected >=3 transport icons");
    }

    /// Icon glyphs swap with state (paused/ muted / fullscreen).
    #[test]
    fn transport_icons_swap_on_state() {
        let th = theme();
        let spec = VideoPlayerSpec::new("clip.mp4")
            .with_playing(false)
            .with_current_time(5.0)
            .with_volume(0.0)
            .with_fullscreen(true);
        let tree = probe(&js_video_player(&spec, &th), 480.0, 270.0);
        assert!(tree.has_text("play"), "paused → play glyph; icons: {:?}", tree.texts());
        assert!(tree.has_text("volume-x"), "muted → volume-x; icons: {:?}", tree.texts());
        assert!(tree.has_text("minimize-2"), "fullscreen → minimize-2; icons: {:?}", tree.texts());
    }

    /// Seek bar uses the ProgressBar widget (proportional fill), not a fixed sliver.
    #[test]
    fn seek_bar_is_progress_widget() {
        let th = theme();
        let spec = VideoPlayerSpec::new("clip.mp4")
            .with_current_time(30.0)
            .with_duration(120.0);
        let tree = probe(&js_video_player(&spec, &th), 480.0, 270.0);
        assert_eq!(
            tree.count_kind("ProgressBar"),
            1,
            "expected exactly one ProgressBar seek track"
        );
    }

    /// Time display renders m:ss / m:ss (not raw seconds).
    #[test]
    fn time_label_is_mss_format() {
        let th = theme();
        let spec = VideoPlayerSpec::new("clip.mp4")
            .with_current_time(13.0)
            .with_duration(125.0);
        let tree = probe(&js_video_player(&spec, &th), 480.0, 270.0);
        // 13s → 0:13, 125s → 2:05.
        assert!(
            tree.has_text("0:13 / 2:05"),
            "time not in m:ss format; texts: {:?}",
            tree.texts()
        );
    }

    /// Big play button (with `play` glyph) only shows when paused at start.
    #[test]
    fn big_play_only_at_start() {
        let th = theme();
        // Paused at 0 → big play present (two `play` glyphs: big + transport).
        let paused = VideoPlayerSpec::new("clip.mp4");
        let tree = probe(&js_video_player(&paused, &th), 480.0, 270.0);
        assert!(tree.count_kind("Icon") >= 4, "big-play + transport icons expected");

        // Playing → no big play (transport shows pause).
        let playing = VideoPlayerSpec::new("clip.mp4").with_playing(true);
        let tree2 = probe(&js_video_player(&playing, &th), 480.0, 270.0);
        assert!(!tree2.has_text("play"), "big play should be hidden while playing");
    }

    /// Size scaling flows through (xl bigger button than xs).
    #[test]
    fn size_scales_geometry() {
        assert!(
            VideoPlayerSpec::button_size_rem(ControlSize::Xl)
                > VideoPlayerSpec::button_size_rem(ControlSize::Xs)
        );
    }
}
