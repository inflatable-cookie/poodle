//! AudioPlayer — audio playback controls backed by AudioPlayerSpec.
//! GPUI cannot play audio — this renders the UI chrome only.

use crate::presentation::{
    control_space_x_rem, rem_to_px, resolve_semantic_size,
};
use crate::primitives::Icon;
use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::AudioPlayerSpec;
use poodle_specs::{ControlDensity, ControlSize, IconSize, IconSpec, SemanticControlSizeRole};

pub struct AudioPlayer {
    spec: AudioPlayerSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for AudioPlayer {
    type Target = AudioPlayerSpec;
    fn deref(&self) -> &AudioPlayerSpec {
        &self.spec
    }
}

impl AudioPlayer {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: AudioPlayerSpec::new(""),
            theme: theme.clone(),
        }
    }
    pub fn from_spec(spec: AudioPlayerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
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

/// Format seconds as `m:ss` (contract CurrentTime/TotalTime).
fn format_time(seconds: f64) -> String {
    let t = seconds.max(0.0) as u64;
    format!("{}:{:02}", t / 60, t % 60)
}

impl IntoElement for AudioPlayer {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let effective_size = resolve_semantic_size(self.spec.size, self.spec.size_role);
        let label_px = resolve_px(&self.theme, "typography.label.size");

        // Size-driven dimensions (contract §8), no longer fixed at 32px / Sm.
        let button_size = rem_to_px(match effective_size {
            ControlSize::Xs => 1.5,
            ControlSize::Sm => 1.75,
            ControlSize::Md => 2.0,
            ControlSize::Lg => 2.25,
            ControlSize::Xl => 2.5,
        });
        let icon_size = match effective_size {
            ControlSize::Xs | ControlSize::Sm | ControlSize::Md => IconSize::Sm,
            ControlSize::Lg => IconSize::Md,
            ControlSize::Xl => IconSize::Lg,
        };
        let volume_width = rem_to_px(match effective_size {
            ControlSize::Xs => 3.0,
            ControlSize::Sm => 4.0,
            ControlSize::Md => 4.0,
            ControlSize::Lg => 4.5,
            ControlSize::Xl => 5.0,
        });
        let time_min_w = rem_to_px(match effective_size {
            ControlSize::Xs => 2.0,
            ControlSize::Sm | ControlSize::Md => 2.5,
            ControlSize::Lg => 2.75,
            ControlSize::Xl => 3.0,
        });

        // Tighter density spacing (matches Svelte CSS vars).
        let pad_x = rem_to_px(control_space_x_rem(self.spec.density));
        let pad_y = match self.spec.density {
            ControlDensity::Compact => rem_to_px(0.375),
            ControlDensity::Default => rem_to_px(0.5),
            ControlDensity::Comfortable => rem_to_px(0.625),
        };
        let gap = pad_y;
        let track_h = rem_to_px(0.25);
        let track_r = rem_to_px(0.125);

        let fill = resolve_color(&self.theme, self.spec.fill_token());
        let control_color = resolve_color(&self.theme, self.spec.control_color_token());
        let radius = resolve_radius(&self.theme, "radius.surface");
        let muted_color = resolve_color(&self.theme, "color.text.secondary");
        let accent = resolve_color(&self.theme, "color.accent.base");
        let focus_ring = resolve_color(&self.theme, "color.accent.focusRing");
        let hover_bg = color_mix(accent, fill, 0.12);
        let seek_base = resolve_color(&self.theme, "color.text.primary");
        let vol_base = color_mix(accent, fill, 0.30);

        let icon = |name: &'static str| {
            Icon::from_spec(IconSpec::new(name).with_size(icon_size), &self.theme)
                .with_color(control_color)
        };
        let transport = |id: &'static str, child: Icon| {
            div()
                .id(SharedString::from(id))
                .focusable()
                .cursor_pointer()
                .w(px(button_size))
                .h(px(button_size))
                .rounded_full()
                .flex()
                .items_center()
                .justify_center()
                .hover(move |s| s.bg(hover_bg))
                .focus(move |s| s.border_color(focus_ring))
                .child(child)
        };

        let time_label = |secs: f64| {
            div()
                .text_size(label_px)
                .text_color(muted_color)
                .min_w(px(time_min_w))
                .child(format_time(secs))
        };

        // Seek bar — relative fill (GPUI handles proportional widths natively).
        let seek_bar = div()
            .h(px(track_h))
            .flex_grow()
            .rounded(px(track_r))
            .bg(seek_base)
            .child(
                div()
                    .h_full()
                    .rounded(px(track_r))
                    .bg(accent)
                    .w(relative(self.spec.progress().clamp(0.0, 1.0) as f32)),
            );

        // Volume bar.
        let vol_frac = if self.spec.is_muted { 0.0 } else { self.spec.volume };
        let volume_bar = div()
            .w(px(volume_width))
            .h(px(track_h))
            .rounded(px(track_r))
            .bg(vol_base)
            .child(
                div()
                    .h_full()
                    .rounded(px(track_r))
                    .bg(accent)
                    .w(relative(vol_frac.clamp(0.0, 1.0) as f32)),
            );

        let mut root = div()
            .bg(fill)
            .rounded(radius)
            .px(px(pad_x))
            .py(px(pad_y))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(gap))
            .child(transport(
                "poodle-audio-play",
                icon(if self.spec.is_playing { "pause" } else { "play" }),
            ))
            .child(time_label(self.spec.current_time))
            .child(seek_bar)
            .child(time_label(self.spec.duration))
            .child(transport(
                "poodle-audio-mute",
                icon(if self.spec.is_muted { "volume-x" } else { "volume-2" }),
            ))
            .child(volume_bar);

        // Speed select — shows the spec rate (the SpeedSelect's current value).
        if self.spec.show_speed_control {
            root = root.child(
                div()
                    .text_size(label_px)
                    .text_color(muted_color)
                    .child(self.spec.rate_label()),
            );
        }

        root.into_any_element()
    }
}
