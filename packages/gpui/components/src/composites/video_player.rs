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
}

impl std::ops::Deref for VideoPlayer {
    type Target = VideoPlayerSpec;
    fn deref(&self) -> &VideoPlayerSpec { &self.spec }
}

impl VideoPlayer {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: VideoPlayerSpec::new(""), theme: theme.clone() }
    }
    pub fn from_spec(spec: VideoPlayerSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone() }
    }
}

impl IntoElement for VideoPlayer {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let fill = resolve_color(&self.theme, self.spec.fill_token());
        let overlay = resolve_color(&self.theme, self.spec.overlay_fill_token());
        let radius = resolve_radius(&self.theme, "semantic.radius.surface");
        let text_color = resolve_color(&self.theme, "semantic.color.text.inverse");

        // Play / Pause icon
        let play_icon_name = if self.spec.is_playing { "pause" } else { "play" };
        let play_icon = Icon::from_spec(
            IconSpec::new(play_icon_name).with_size(IconSize::Sm),
            &self.theme,
        ).with_color(text_color);

        // Time label
        let time = format!("{:.0}s / {:.0}s", self.spec.current_time, self.spec.duration);

        // Progress bar
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

        // Mute icon (volume indicator — VideoPlayerSpec has volume but no is_muted field,
        // so we treat volume == 0 as muted)
        let mute_icon_name = if self.spec.volume <= 0.0 { "volume-x" } else { "volume-2" };
        let mute_icon = Icon::from_spec(
            IconSpec::new(mute_icon_name).with_size(IconSize::Sm),
            &self.theme,
        ).with_color(text_color);

        div()
            .bg(fill).rounded(radius)
            .w_full().min_h(px(180.0))
            .flex().flex_col().justify_end()
            .child(
                div().bg(overlay).px(px(12.0)).py(px(8.0))
                    .flex().flex_row().items_center().gap(px(8.0))
                    .child(div().cursor_pointer().child(play_icon))
                    .child(div().text_xs().text_color(text_color).child(time))
                    .child(track_bar)
                    .child(div().cursor_pointer().child(mute_icon))
            )
            .into_any_element()
    }
}
