//! VideoPlayer — video playback controls backed by VideoPlayerSpec.
//! GPUI cannot play video — this renders the UI chrome only.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{IconSize, IconSpec};
use pug_composites::VideoPlayerSpec;
use crate::primitives::Icon;
use crate::theme_ext::{resolve_color, resolve_radius};

pub struct VideoPlayer {
    spec: VideoPlayerSpec,
    theme: GpuiThemeProvider,
    has_captions: bool,
}

impl std::ops::Deref for VideoPlayer {
    type Target = VideoPlayerSpec;
    fn deref(&self) -> &VideoPlayerSpec { &self.spec }
}

impl VideoPlayer {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: VideoPlayerSpec::new(""), theme: theme.clone(), has_captions: false }
    }
    pub fn from_spec(spec: VideoPlayerSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), has_captions: false }
    }
    pub fn with_captions(mut self, has_captions: bool) -> Self {
        self.has_captions = has_captions;
        self
    }
}

impl IntoElement for VideoPlayer {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let fill = resolve_color(&self.theme, self.spec.fill_token());
        let overlay = resolve_color(&self.theme, self.spec.overlay_fill_token());
        let radius = resolve_radius(&self.theme, "semantic.radius.surface");
        let border_color = resolve_color(&self.theme, "semantic.color.border.default");
        let text_color = resolve_color(&self.theme, "semantic.color.text.inverse");

        // ── Big centered play/pause overlay button ──────────────────────
        let big_play_icon_name = if self.spec.is_playing { "pause" } else { "play" };
        let big_play_icon = Icon::from_spec(
            IconSpec::new(big_play_icon_name).with_size(IconSize::Lg),
            &self.theme,
        ).with_color(text_color);

        let big_play_button = div()
            .flex()
            .items_center()
            .justify_center()
            .size(px(56.0))
            .rounded(px(28.0))
            .bg(overlay)
            .cursor_pointer()
            .child(big_play_icon);

        // ── Video viewport area ─────────────────────────────────────────
        let viewport = div()
            .w_full()
            .flex_grow()
            .min_h(px(160.0))
            .flex()
            .items_center()
            .justify_center()
            .child(big_play_button);

        // ── Bottom control bar icons ────────────────────────────────────

        // Play / Pause small icon
        let play_icon_name = if self.spec.is_playing { "pause" } else { "play" };
        let play_icon = Icon::from_spec(
            IconSpec::new(play_icon_name).with_size(IconSize::Sm),
            &self.theme,
        ).with_color(text_color);

        // Progress / seek bar (same pattern as audio_player track_bar)
        let progress_pct = (self.spec.progress() * 100.0).clamp(0.0, 100.0);
        let track_bar = div()
            .h(px(4.0))
            .flex_grow()
            .rounded(px(2.0))
            .bg(text_color.opacity(0.3))
            .child(
                div()
                    .h_full()
                    .rounded(px(2.0))
                    .bg(text_color)
                    .w(relative(progress_pct as f32 / 100.0)),
            );

        // Time label
        let time = format!("{:.0}s / {:.0}s", self.spec.current_time, self.spec.duration);

        // Mute / unmute icon (volume == 0 treated as muted)
        let mute_icon_name = if self.spec.volume <= 0.0 { "volume-x" } else { "volume-2" };
        let mute_icon = Icon::from_spec(
            IconSpec::new(mute_icon_name).with_size(IconSize::Sm),
            &self.theme,
        ).with_color(text_color);

        // Fullscreen icon
        let fullscreen_icon_name = if self.spec.is_fullscreen { "minimize-2" } else { "maximize-2" };
        let fullscreen_icon = Icon::from_spec(
            IconSpec::new(fullscreen_icon_name).with_size(IconSize::Sm),
            &self.theme,
        ).with_color(text_color);

        // Captions icon (only shown when captions are available)
        let captions_icon = if self.has_captions {
            Some(
                div().cursor_pointer().child(
                    Icon::from_spec(
                        IconSpec::new("subtitles").with_size(IconSize::Sm),
                        &self.theme,
                    ).with_color(text_color),
                ),
            )
        } else {
            None
        };

        // ── Control bar ─────────────────────────────────────────────────

        // Helper: control button wrapper 28x28
        let ctrl_btn = |child: AnyElement| -> Div {
            div()
                .cursor_pointer()
                .w(px(28.0)).h(px(28.0))
                .rounded(px(4.0))
                .flex().items_center().justify_center()
                .hover(|s| s.bg(text_color.opacity(0.15)))
                .child(child)
        };

        // Left group: play + seek + time
        let left_group = div()
            .flex().flex_row().items_center().gap(px(8.0)).flex_grow()
            .child(ctrl_btn(play_icon.into_any_element()))
            .child(track_bar)
            .child(div().text_size(px(12.0)).text_color(text_color).child(time));

        // Right group: mute + fullscreen + captions
        let mut right_group = div()
            .flex().flex_row().items_center().gap(px(4.0))
            .child(ctrl_btn(mute_icon.into_any_element()))
            .child(ctrl_btn(fullscreen_icon.into_any_element()));

        if let Some(cap_el) = captions_icon {
            right_group = right_group.child(cap_el);
        }

        let control_bar = div()
            .bg(overlay)
            .px(px(12.0))
            .py(px(8.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .child(left_group)
            .child(right_group);

        // ── Outer container ─────────────────────────────────────────────
        div()
            .bg(fill)
            .rounded(radius)
            .border_1()
            .border_color(border_color)
            .w_full()
            .min_h(px(220.0))
            .overflow_hidden()
            .flex()
            .flex_col()
            .justify_end()
            .child(viewport)
            .child(control_bar)
            .into_any_element()
    }
}
