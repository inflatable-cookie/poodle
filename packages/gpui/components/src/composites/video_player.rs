//! VideoPlayer — video playback controls backed by VideoPlayerSpec.
//! GPUI cannot play video — this renders the UI chrome only.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_composites::VideoPlayerSpec;
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

        let play_icon = if self.spec.is_playing { "⏸" } else { "▶" };
        let time = format!("{:.0}s / {:.0}s", self.spec.current_time, self.spec.duration);

        div()
            .bg(fill).rounded(radius)
            .w_full().min_h(px(180.0))
            .flex().flex_col().justify_end()
            .child(
                div().bg(overlay).px(px(12.0)).py(px(8.0))
                    .flex().flex_row().items_center().gap(px(8.0))
                    .child(div().text_color(text_color).cursor_pointer().child(play_icon))
                    .child(div().text_xs().text_color(text_color).child(time))
            )
            .into_any_element()
    }
}
