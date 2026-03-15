use pug_tokens::semantic;

use crate::types::AspectRatio;

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
}

impl VideoPlayerSpec {
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
}
