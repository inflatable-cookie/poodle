//! AudioPlayer — audio playback controls backed by AudioPlayerSpec.
//! GPUI cannot play audio — this renders the UI chrome only.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_composites::AudioPlayerSpec;
use crate::theme_ext::{resolve_color, resolve_radius};

pub struct AudioPlayer {
    spec: AudioPlayerSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for AudioPlayer {
    type Target = AudioPlayerSpec;
    fn deref(&self) -> &AudioPlayerSpec { &self.spec }
}

impl AudioPlayer {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: AudioPlayerSpec::new(""), theme: theme.clone() }
    }
    pub fn from_spec(spec: AudioPlayerSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone() }
    }
}

impl IntoElement for AudioPlayer {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let fill = resolve_color(&self.theme, self.spec.fill_token());
        let control_color = resolve_color(&self.theme, self.spec.control_color_token());
        let radius = resolve_radius(&self.theme, "semantic.radius.surface");
        let muted = resolve_color(&self.theme, "semantic.color.text.secondary");

        let play_icon = if self.spec.is_playing { "⏸" } else { "▶" };
        let time = format!("{:.0}s / {:.0}s", self.spec.current_time, self.spec.duration);

        div()
            .bg(fill).rounded(radius)
            .px(px(12.0)).py(px(8.0))
            .flex().flex_row().items_center().gap(px(8.0))
            .child(div().text_color(control_color).cursor_pointer().child(play_icon))
            .child(div().text_xs().text_color(muted).child(time))
            .into_any_element()
    }
}
