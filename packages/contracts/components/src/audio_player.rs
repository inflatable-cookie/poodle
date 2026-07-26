use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

#[derive(Clone, Debug, PartialEq)]
pub struct AudioPlayerSpec {
    pub src: String,
    pub is_playing: bool,
    pub current_time: f64,
    pub duration: f64,
    pub volume: f64,
    pub is_muted: bool,
    pub show_speed_control: bool,
    /// Playback rate (e.g. 1.0 = 1x); drives the SpeedSelect display.
    pub rate: f64,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
    /// Accessible name (contract §7). `None` falls back to the visible label.
    pub aria_label: Option<String>,
}

impl AudioPlayerSpec {
    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn new(src: impl Into<String>) -> Self {
        Self {
            src: src.into(),
            is_playing: false,
            current_time: 0.0,
            duration: 0.0,
            volume: 1.0,
            is_muted: false,
            show_speed_control: false,
            rate: 1.0,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
            aria_label: None,
        }
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

    pub fn with_muted(mut self, is_muted: bool) -> Self {
        self.is_muted = is_muted;
        self
    }

    pub fn with_show_speed_control(mut self, show_speed_control: bool) -> Self {
        self.show_speed_control = show_speed_control;
        self
    }

    pub fn with_rate(mut self, rate: f64) -> Self {
        self.rate = rate;
        self
    }

    /// Format the playback rate for the SpeedSelect label, e.g. `"1x"`/`"1.5x"`.
    pub fn rate_label(&self) -> String {
        if (self.rate.fract()).abs() < f64::EPSILON {
            format!("{}x", self.rate as i64)
        } else {
            format!("{}x", self.rate)
        }
    }

    pub fn progress(&self) -> f64 {
        if self.duration <= 0.0 {
            0.0
        } else {
            self.current_time / self.duration
        }
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn control_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
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
