//! VideoPlayer — video playback controls backed by VideoPlayerSpec.
//! GPUI cannot play video — this renders the UI chrome only.
//!
//! Per contract §8 the player is black-background chrome with fixed
//! `rgba(255,255,255,…)` colors regardless of theme; only the surface radius
//! and control radius come from tokens. All geometry resolves from the
//! `VideoPlayerSpec` size/density rem ladders via `rem_to_px`.
//!
//! # Known GPUI deltas
//! - No real `<video>` playback, fullscreen, or auto-hide controls (contract §10).
//! - No ARIA (GPUI has no accessibility API). Playback/seek/volume interaction
//!   lives in the preview event loop.

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::primitives::Icon;
use crate::theme_ext::resolve_radius;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::VideoPlayerSpec;
use poodle_specs::{ControlDensity, ControlSize, SemanticControlSizeRole};

/// Format seconds as `m:ss` (contract `.video-player__time`).
fn format_time(seconds: f64) -> String {
    let total = seconds.max(0.0) as u64;
    format!("{}:{:02}", total / 60, total % 60)
}

pub struct VideoPlayer {
    spec: VideoPlayerSpec,
    theme: GpuiThemeProvider,
    has_captions: bool,
}

impl std::ops::Deref for VideoPlayer {
    type Target = VideoPlayerSpec;
    fn deref(&self) -> &VideoPlayerSpec {
        &self.spec
    }
}

impl VideoPlayer {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: VideoPlayerSpec::new(""),
            theme: theme.clone(),
            has_captions: false,
        }
    }
    pub fn from_spec(spec: VideoPlayerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            has_captions: false,
        }
    }
    pub fn with_captions(mut self, has_captions: bool) -> Self {
        self.has_captions = has_captions;
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

impl IntoElement for VideoPlayer {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        // ── Token-resolved chrome radii (the only theme-driven values) ───
        let radius = resolve_radius(&self.theme, "radius.surface");
        let radius_control = resolve_radius(&self.theme, "radius.control");

        // ── Fixed colors (contract §8: white-on-black regardless of theme) ─
        let black = gpui::black();
        let white_90 = gpui::white().opacity(0.9); // text / icons
        let white_80 = gpui::white().opacity(0.8); // time display
        let white_20 = gpui::white().opacity(0.2); // progress track
        let white_50 = gpui::white().opacity(0.5); // volume track
        let white_15 = gpui::white().opacity(0.15); // button hover
        // Controls overlay gradient bottom — emulate `rgba(0,0,0,0.7)`.
        let overlay = gpui::black().opacity(0.7);
        let accent = crate::theme_ext::resolve_color(&self.theme, "color.accent.base");

        // ── Size/density geometry (contract §7/§8 rem ladders) ───────────
        let btn_size = px(rem_to_px(VideoPlayerSpec::button_size_rem(effective_size)));
        let icon_px = rem_to_px(VideoPlayerSpec::icon_size_rem(effective_size));
        let volume_w = px(rem_to_px(VideoPlayerSpec::volume_width_rem(effective_size)));
        let time_font = px(rem_to_px(VideoPlayerSpec::time_font_rem(effective_size)));
        let big_play_size = px(rem_to_px(VideoPlayerSpec::big_play_size_rem(effective_size)));
        let track_h = px(rem_to_px(VideoPlayerSpec::track_height_rem()));
        let volume_thumb = px(rem_to_px(VideoPlayerSpec::volume_thumb_rem()));
        let bar_gap = px(rem_to_px(VideoPlayerSpec::bar_gap_rem(spec.density)));
        // Controls padding `1.5rem 0.5rem 0.375rem` (contract `.video-player__controls`).
        let controls_pt = px(rem_to_px(1.5));
        let controls_px = px(rem_to_px(0.5));
        let controls_pb = px(rem_to_px(0.375));
        let progress_mb = px(rem_to_px(0.375)); // `.progress-bar` margin-bottom

        // ── Big center play button — transparent ring, only when paused at 0 ─
        // Contract §4: shown only when paused at currentTime=0. Transparent bg.
        let show_big_play = !spec.is_playing && spec.current_time <= 0.0;
        let big_play = if show_big_play {
            let play_glyph = Icon::from_spec(
                poodle_specs::IconSpec::new("play"),
                &self.theme,
            )
            .with_color(white_90)
            .with_px_size(f32::from(big_play_size) * 0.5);
            Some(
                div()
                    .absolute()
                    .flex()
                    .items_center()
                    .justify_center()
                    .size(big_play_size)
                    .rounded_full()
                    .border_2()
                    .border_color(white_90)
                    .cursor_pointer()
                    .child(play_glyph),
            )
        } else {
            None
        };

        // ── Video viewport ───────────────────────────────────────────────
        let mut viewport = div()
            .relative()
            .w_full()
            .flex_grow()
            .flex()
            .items_center()
            .justify_center();
        if let Some(bp) = big_play {
            viewport = viewport.child(bp);
        }

        // ── Transport icons (real Icons, sized from contract SVG ladder) ──
        let icon = |name: &'static str| {
            Icon::from_spec(poodle_specs::IconSpec::new(name), &self.theme)
                .with_color(white_90)
                .with_px_size(icon_px)
        };
        let play_icon = icon(if spec.is_playing { "pause" } else { "play" });
        let mute_icon = icon(if spec.volume <= 0.0 { "volume-x" } else { "volume-2" });
        let fullscreen_icon = icon(if spec.is_fullscreen {
            "minimize-2"
        } else {
            "maximize-2"
        });

        // ── Control button wrapper (square, hover bg) ────────────────────
        let ctrl_btn = move |child: AnyElement| -> Div {
            div()
                .cursor_pointer()
                .w(btn_size)
                .h(btn_size)
                .rounded(radius_control)
                .flex()
                .items_center()
                .justify_center()
                .hover(move |s| s.bg(white_15))
                .child(child)
        };

        // ── Progress / seek bar — proportional fill via relative(frac) ────
        let frac = (spec.progress() as f32).clamp(0.0, 1.0);
        let progress_bar = div()
            .relative()
            .h(track_h)
            .w_full()
            .rounded(px(VideoPlayerSpec::pill_radius_rem()))
            .overflow_hidden()
            .bg(white_20)
            .child(
                div()
                    .h_full()
                    .rounded(px(VideoPlayerSpec::pill_radius_rem()))
                    .bg(accent)
                    .w(relative(frac)),
            );

        // ── Time display (m:ss / m:ss, monospace) ────────────────────────
        let time_text = format!(
            "{} / {}",
            format_time(spec.current_time),
            format_time(spec.duration)
        );
        let time_display = div()
            .font_family("monospace")
            .text_size(time_font)
            .text_color(white_80)
            .child(time_text);

        // ── Volume slider chrome (track + thumb at current volume) ───────
        let vol_frac = (spec.volume as f32).clamp(0.0, 1.0);
        let volume_slider = div()
            .relative()
            .flex()
            .items_center()
            .w(volume_w)
            .h(px(rem_to_px(1.0)))
            .child(
                // Track
                div()
                    .w_full()
                    .h(track_h)
                    .rounded(px(rem_to_px(0.125)))
                    .bg(white_50),
            )
            .child(
                // Thumb positioned by volume fraction
                div()
                    .absolute()
                    .left(relative(vol_frac))
                    .size(volume_thumb)
                    .rounded_full()
                    .bg(gpui::white()),
            );

        // ── Bar groups ───────────────────────────────────────────────────
        // Left: play + mute + volume + time.
        let left_group = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(bar_gap)
            .child(ctrl_btn(play_icon.into_any_element()))
            .child(ctrl_btn(mute_icon.into_any_element()))
            .child(volume_slider)
            .child(time_display);

        // Right: captions (optional) + fullscreen.
        let mut right_group = div().flex().flex_row().items_center().gap(bar_gap);
        if self.has_captions {
            let cap_icon = icon("subtitles");
            right_group = right_group.child(ctrl_btn(cap_icon.into_any_element()));
        }
        right_group = right_group.child(ctrl_btn(fullscreen_icon.into_any_element()));

        let bar = div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .child(left_group)
            .child(right_group);

        // ── Controls overlay (gradient-ish dark fill, bottom-anchored) ───
        let controls = div()
            .w_full()
            .bg(overlay)
            .pt(controls_pt)
            .px(controls_px)
            .pb(controls_pb)
            .flex()
            .flex_col()
            .child(progress_bar.mb(progress_mb))
            .child(bar);

        // ── Outer container — black surface, token radius ────────────────
        div()
            .bg(black)
            .rounded(radius)
            .w_full()
            .min_h(px(rem_to_px(13.75))) // 220px @ 16px base, contract chrome min
            .overflow_hidden()
            .flex()
            .flex_col()
            .justify_end()
            .child(viewport)
            .child(controls)
            .into_any_element()
    }
}
