//! AudioPlayer — audio playback controls backed by AudioPlayerSpec.
//! GPUI cannot play audio — this renders the UI chrome only.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{ControlDensity, ControlSize, IconSize, IconSpec, SemanticControlSizeRole};
use poodle_composites::AudioPlayerSpec;
use crate::presentation::{resolve_semantic_size, size_font_rem, panel_space_x_rem, panel_space_y_rem, control_space_x_rem, rem_to_px};
use crate::primitives::Icon;
use crate::theme_ext::{color_mix, resolve_color, resolve_radius};

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
    pub fn with_size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self { self.spec.size_role = v; self }
    pub fn with_density(mut self, v: ControlDensity) -> Self { self.spec.density = v; self }
}

impl IntoElement for AudioPlayer {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let effective_size = resolve_semantic_size(self.spec.size, self.spec.size_role);
        let font_size = rem_to_px(size_font_rem(effective_size));
        let pad_x = rem_to_px(panel_space_x_rem(self.spec.density));
        let pad_y = rem_to_px(panel_space_y_rem(self.spec.density));
        let gap = rem_to_px(control_space_x_rem(self.spec.density));

        let fill = resolve_color(&self.theme, self.spec.fill_token());
        let control_color = resolve_color(&self.theme, self.spec.control_color_token());
        let radius = resolve_radius(&self.theme, "radius.surface");
        let muted_color = resolve_color(&self.theme, "color.text.secondary");
        let accent = resolve_color(&self.theme, "color.accent.base");
        let focus_ring = resolve_color(&self.theme, "color.accent.focusRing");
        let hover_bg = color_mix(accent, fill, 0.12);

        // Play / Pause icon
        let play_icon_name = if self.spec.is_playing { "pause" } else { "play" };
        let play_icon = Icon::from_spec(
            IconSpec::new(play_icon_name).with_size(IconSize::Sm),
            &self.theme,
        ).with_color(control_color);

        // Time label
        let time = format!("{:.0}s / {:.0}s", self.spec.current_time, self.spec.duration);

        // Progress / seek bar
        let progress_pct = (self.spec.progress() * 100.0).clamp(0.0, 100.0);
        let track_bar = div()
            .h(px(4.0))
            .flex_grow()
            .rounded(px(2.0))
            .bg(resolve_color(&self.theme, "color.border.default"))
            .child(
                div()
                    .h_full()
                    .rounded(px(2.0))
                    .bg(control_color)
                    .w(relative(progress_pct as f32 / 100.0)),
            );

        // Mute / unmute icon
        let mute_icon_name = if self.spec.is_muted { "volume-x" } else { "volume-2" };
        let mute_icon = Icon::from_spec(
            IconSpec::new(mute_icon_name).with_size(IconSize::Sm),
            &self.theme,
        ).with_color(control_color);

        div()
            .bg(fill).rounded(radius)
            .px(px(pad_x)).py(px(pad_y))
            .flex().flex_row().items_center().gap(px(gap))
            .child(
                div()
                    .id(SharedString::from("poodle-audio-play"))
                    .focusable()
                    .cursor_pointer()
                    .w(px(32.0)).h(px(32.0))
                    .rounded(px(4.0))
                    .flex().items_center().justify_center()
                    .hover(move |s| s.bg(hover_bg))
                    .focus(move |s| s.border_color(focus_ring))
                    .child(play_icon)
            )
            .child(div().text_size(px(font_size * 0.85)).text_color(muted_color).child(time))
            .child(track_bar)
            .child(
                div()
                    .id(SharedString::from("poodle-audio-mute"))
                    .focusable()
                    .cursor_pointer()
                    .w(px(32.0)).h(px(32.0))
                    .rounded(px(4.0))
                    .flex().items_center().justify_center()
                    .hover(move |s| s.bg(hover_bg))
                    .focus(move |s| s.border_color(focus_ring))
                    .child(mute_icon)
            )
            .into_any_element()
    }
}
