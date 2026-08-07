use poodle_tokens::semantic;

use crate::composite_types::AspectRatio;
use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};

#[derive(Clone, Debug, PartialEq)]
pub struct VideoPlayerSpec {
    pub src: String,
    pub poster: Option<String>,
    pub aspect_ratio: AspectRatio,
    pub is_playing: bool,
    pub current_time: f64,
    pub duration: f64,
    pub volume: f64,
    pub is_fullscreen: bool,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
    /// Accessible name (contract §7). `None` falls back to the visible label.
    pub aria_label: Option<String>,
    /// Caption track URL.
    pub captions_src: Option<String>,
    /// Whether captions render by default.
    pub shows_captions: bool,
}

impl VideoPlayerSpec {
    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn new(src: impl Into<String>) -> Self {
        Self {
            src: src.into(),
            poster: None,
            aspect_ratio: AspectRatio::Video,
            is_playing: false,
            current_time: 0.0,
            duration: 0.0,
            volume: 1.0,
            is_fullscreen: false,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
            aria_label: None,
            captions_src: None,
            shows_captions: false,
        }
    }

    pub fn with_poster(mut self, poster: impl Into<String>) -> Self {
        self.poster = Some(poster.into());
        self
    }

    pub fn with_aspect_ratio(mut self, aspect_ratio: AspectRatio) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn with_playing(mut self, is_playing: bool) -> Self {
        self.is_playing = is_playing;
        self
    }

    pub fn with_current_time(mut self, current_time: f64) -> Self {
        self.current_time = current_time;
        self
    }

    pub fn with_duration(mut self, duration: f64) -> Self {
        self.duration = duration;
        self
    }

    pub fn with_volume(mut self, volume: f64) -> Self {
        self.volume = volume;
        self
    }

    pub fn with_fullscreen(mut self, is_fullscreen: bool) -> Self {
        self.is_fullscreen = is_fullscreen;
        self
    }

    pub fn with_captions_src(mut self, captions_src: impl Into<String>) -> Self {
        self.captions_src = Some(captions_src.into());
        self
    }

    pub fn with_show_captions(mut self, shows_captions: bool) -> Self {
        self.shows_captions = shows_captions;
        self
    }

    /// Contract §2: the captions `<track>` renders only when both
    /// `showCaptions` and `captionsSrc` are present. It carries no chrome of
    /// its own on any target.
    pub fn renders_captions_track(&self) -> bool {
        self.shows_captions && self.captions_src.is_some()
    }

    pub fn progress(&self) -> f64 {
        if self.duration <= 0.0 {
            0.0
        } else {
            self.current_time / self.duration
        }
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }

    pub fn overlay_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_OVERLAY
    }

    // ── Contract §7/§8 size + density geometry (rem) ──────────────────
    //
    // Single source of truth for the per-size control geometry so both Rust
    // targets resolve identical dimensions. `effective` is the size already
    // resolved through `resolve_semantic_size`. Values are contract-exact rem;
    // impls feed them through `rem_to_px`.

    /// Control button square size, in rem. Contract §8 size table:
    /// xs 1.25, sm 1.5, md 1.75, lg 2.125, xl 2.25.
    pub fn button_size_rem(effective: ControlSize) -> f32 {
        match effective {
            ControlSize::Xs => 1.25,
            ControlSize::Sm => 1.5,
            ControlSize::Md => 1.75,
            ControlSize::Lg => 2.125,
            ControlSize::Xl => 2.25,
        }
    }

    /// Control button glyph (SVG) size, in rem. Contract `.video-player__btn
    /// svg` md 0.875 + the xs/lg/xl size overrides (sm inherits md).
    pub fn icon_size_rem(effective: ControlSize) -> f32 {
        match effective {
            ControlSize::Xs => 0.75,
            ControlSize::Sm => 0.875,
            ControlSize::Md => 0.875,
            ControlSize::Lg => 1.0,
            ControlSize::Xl => 1.125,
        }
    }

    /// Volume slider width, in rem. Contract §8 size table:
    /// xs 2.5, sm 3, md 3.5, lg 4, xl 4.5.
    pub fn volume_width_rem(effective: ControlSize) -> f32 {
        match effective {
            ControlSize::Xs => 2.5,
            ControlSize::Sm => 3.0,
            ControlSize::Md => 3.5,
            ControlSize::Lg => 4.0,
            ControlSize::Xl => 4.5,
        }
    }

    /// Time-display font size, in rem. Contract `.video-player__time` md 0.6875
    /// + the per-size overrides.
    pub fn time_font_rem(effective: ControlSize) -> f32 {
        match effective {
            ControlSize::Xs => 0.5625,
            ControlSize::Sm => 0.625,
            ControlSize::Md => 0.6875,
            ControlSize::Lg => 0.75,
            ControlSize::Xl => 0.8125,
        }
    }

    /// Big center play button square size, in rem. Contract `.video-player__
    /// big-play` md 4 + the per-size overrides.
    pub fn big_play_size_rem(effective: ControlSize) -> f32 {
        match effective {
            ControlSize::Xs => 3.0,
            ControlSize::Sm => 3.5,
            ControlSize::Md => 4.0,
            ControlSize::Lg => 4.5,
            ControlSize::Xl => 5.0,
        }
    }

    /// Progress / volume track height, in rem (contract `0.25rem`).
    pub fn track_height_rem() -> f32 {
        0.25
    }

    /// Volume thumb diameter, in rem (contract `0.625rem`).
    pub fn volume_thumb_rem() -> f32 {
        0.625
    }

    /// Bar-left/bar-right inter-control gap, in rem, per density.
    /// Contract `.video-player__bar-left` md 0.375 + density overrides.
    pub fn bar_gap_rem(density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.25,
            ControlDensity::Default => 0.375,
            ControlDensity::Comfortable => 0.5,
        }
    }

    /// Pill corner radius, in rem, for progress/volume tracks and the big-play
    /// ring (contract `999rem`). Named so impls avoid a bare `999.0` literal.
    pub fn pill_radius_rem() -> f32 {
        999.0
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }
}
