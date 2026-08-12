use poodle_tokens::semantic;

use crate::composite_types::{AspectRatio, MediaKind, MediaState};

/// Compact mode hides the caption and the state message (contract §4
/// `presentation`). Kept distinct from `show_caption` so the GPUI compose
/// path (which already gates the caption explicitly) keeps working.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum MediaPresentation {
    #[default]
    Default,
    Compact,
}

/// Object-fit mode applied to image content (contract §4 `fit`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum MediaFit {
    #[default]
    Cover,
    Contain,
}

/// Explicit frame width (contract §4 `frameWidth`). `Fill` stretches to the
/// parent, `Xl` applies the preset wide size (`min(100%, 24rem)`), `Px` sets an
/// explicit pixel width.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum MediaFrameWidth {
    #[default]
    Fill,
    Xl,
    Px(f32),
}

impl Eq for MediaFrameWidth {}

#[derive(Clone, Debug, PartialEq)]
pub struct MediaThumbnailSpec {
    pub kind: MediaKind,
    pub state: MediaState,
    pub aspect_ratio: AspectRatio,
    pub title: Option<String>,
    pub meta: Option<String>,
    pub badge_label: Option<String>,
    pub state_title: Option<String>,
    pub state_message: Option<String>,
    pub show_caption: bool,
    pub presentation: MediaPresentation,
    pub fit: MediaFit,
    pub frame_width: MediaFrameWidth,
    pub frame_min_height: Option<f32>,
    pub frame_max_height: Option<f32>,
}

impl Eq for MediaThumbnailSpec {}

impl MediaThumbnailSpec {
    pub fn new(kind: MediaKind) -> Self {
        Self {
            kind,
            state: MediaState::Ready,
            aspect_ratio: AspectRatio::Landscape,
            title: None,
            meta: None,
            badge_label: None,
            state_title: None,
            state_message: None,
            show_caption: true,
            presentation: MediaPresentation::Default,
            fit: MediaFit::Cover,
            frame_width: MediaFrameWidth::Fill,
            frame_min_height: None,
            frame_max_height: None,
        }
    }

    pub fn with_state(mut self, state: MediaState) -> Self {
        self.state = state;
        self
    }

    pub fn with_aspect_ratio(mut self, aspect_ratio: AspectRatio) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_meta(mut self, meta: impl Into<String>) -> Self {
        self.meta = Some(meta.into());
        self
    }

    pub fn with_badge_label(mut self, badge_label: impl Into<String>) -> Self {
        self.badge_label = Some(badge_label.into());
        self
    }

    pub fn with_state_title(mut self, state_title: impl Into<String>) -> Self {
        self.state_title = Some(state_title.into());
        self
    }

    pub fn with_state_message(mut self, state_message: impl Into<String>) -> Self {
        self.state_message = Some(state_message.into());
        self
    }

    pub fn with_show_caption(mut self, show_caption: bool) -> Self {
        self.show_caption = show_caption;
        self
    }

    pub fn with_presentation(mut self, presentation: MediaPresentation) -> Self {
        self.presentation = presentation;
        self
    }

    pub fn with_fit(mut self, fit: MediaFit) -> Self {
        self.fit = fit;
        self
    }

    pub fn with_frame_width(mut self, frame_width: MediaFrameWidth) -> Self {
        self.frame_width = frame_width;
        self
    }

    pub fn with_frame_min_height(mut self, frame_min_height: f32) -> Self {
        self.frame_min_height = Some(frame_min_height);
        self
    }

    pub fn with_frame_max_height(mut self, frame_max_height: f32) -> Self {
        self.frame_max_height = Some(frame_max_height);
        self
    }

    pub fn is_compact(&self) -> bool {
        self.presentation == MediaPresentation::Compact
    }

    pub fn shows_fallback_copy(&self) -> bool {
        self.state != MediaState::Ready
    }

    pub fn caption_visible(&self) -> bool {
        // Compact presentation hides the caption entirely (contract §3 Caption,
        // §9 Root Compact).
        self.show_caption && !self.is_compact() && self.title.is_some()
    }

    /// True when the body state message should render. Hidden in compact mode
    /// (contract §3 State Display, §4 `stateMessage`).
    pub fn state_message_visible(&self) -> bool {
        self.state != MediaState::Ready && !self.is_compact() && self.state_message.is_some()
    }

    /// True for kinds that show the play/music overlay indicator
    /// (contract §3 Play Indicator — audio/video only).
    pub fn shows_play_indicator(&self) -> bool {
        self.state == MediaState::Ready && matches!(self.kind, MediaKind::Audio | MediaKind::Video)
    }

    /// Fallback icon name keyed by kind (contract §9 Fallback Icons By Kind).
    pub fn fallback_icon(&self) -> &'static str {
        match self.kind {
            MediaKind::Image => "image",
            MediaKind::Audio => "music",
            MediaKind::Video => "play",
            MediaKind::Document => "file-text",
            MediaKind::Embed => "external-link",
        }
    }

    /// Play-indicator icon name (contract §3): music for audio, play for video.
    pub fn play_indicator_icon(&self) -> &'static str {
        match self.kind {
            MediaKind::Audio => "music",
            _ => "play",
        }
    }

    /// Aspect ratio as a `(width, height)` pair (contract §9 Aspect Ratios).
    /// Every variant maps to a concrete ratio for placeholder framing. `Auto` has no
    /// intrinsic media in the Rust targets, so it falls back to the landscape default;
    /// use `is_auto()` if an impl wants content-driven height instead of a fixed frame.
    pub fn aspect_ratio_pair(&self) -> (f32, f32) {
        match self.aspect_ratio {
            AspectRatio::Auto => (16.0, 10.0),
            AspectRatio::Square => (1.0, 1.0),
            AspectRatio::Landscape => (16.0, 10.0),
            AspectRatio::Portrait => (3.0, 4.0),
            AspectRatio::Video => (16.0, 9.0),
        }
    }

    /// True when the frame should follow the media's intrinsic ratio rather than a
    /// fixed aspect (contract `data-aspect-ratio="auto"`).
    pub fn is_auto(&self) -> bool {
        matches!(self.aspect_ratio, AspectRatio::Auto)
    }

    /// Frame height derived from a resolved frame width and the aspect ratio
    /// (contract §9 — CSS `aspect-ratio` applied to a token-driven width).
    pub fn frame_height_for_width(&self, width_px: f32) -> f32 {
        let (w, h) = self.aspect_ratio_pair();
        width_px * (h / w)
    }

    pub fn resolved_state_title(&self) -> &str {
        if let Some(ref title) = self.state_title {
            return title;
        }

        match self.state {
            MediaState::Loading => "Loading preview",
            MediaState::Error => "Preview unavailable",
            MediaState::Empty | MediaState::Ready => "No preview",
        }
    }

    pub fn resolved_state_message(&self) -> Option<&str> {
        self.state_message.as_deref()
    }

    pub fn frame_fill_token(&self) -> &'static str {
        // Contract §9 Frame: the frame fill is a panel-tinted gradient. The flat
        // fallback fill (when a renderer can't paint the gradient) is the panel
        // background; `frame_panel_token` is the preferred resolve target.
        semantic::COLOR_BACKGROUND_SURFACE
    }

    /// Frame background panel tint (contract §9 Frame background base layer).
    pub fn frame_panel_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }

    /// Frame border (contract §9 Frame `border`).
    pub fn frame_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    /// Frame radial-gradient accent tint (contract §9 Frame background).
    pub fn frame_accent_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    /// Frame radius token (contract §9 Frame: `calc(radius.surface − 0.125rem)`).
    /// Callers subtract `0.125rem` from the resolved surface radius.
    pub fn frame_radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    /// Inset (rem) subtracted from the resolved surface radius for the frame
    /// (contract §9 Frame `border-radius`).
    pub fn frame_radius_inset_rem(&self) -> f32 {
        0.125
    }

    /// Badge background (contract §9 Badge: surface-mix; flat fallback = surface).
    pub fn badge_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    /// Badge text color (contract §9 Badge `color`).
    pub fn badge_text_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    /// Badge radius (contract §9 Badge `border-radius`).
    pub fn badge_radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    /// Play-indicator background (contract §9 Play Indicator: elevated-mix;
    /// flat fallback = elevated).
    pub fn play_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    /// Play-indicator icon/text color (contract §9 Play Indicator `color`).
    pub fn play_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    /// Play-indicator radius (contract §9 Play Indicator `border-radius`).
    pub fn play_radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    /// Placeholder / fallback icon color (contract §9 Placeholder `icon color`).
    pub fn placeholder_icon_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_aspect_ratio_resolves_and_flags() {
        // Contract `AspectRatio = "auto" | …` — Auto must resolve a placeholder pair
        // (landscape fallback, no real media) and report is_auto() for content-driven impls.
        let spec = MediaThumbnailSpec::new(MediaKind::Image).with_aspect_ratio(AspectRatio::Auto);
        assert!(spec.is_auto());
        assert_eq!(spec.aspect_ratio_pair(), (16.0, 10.0));
        assert!(spec.frame_height_for_width(160.0) > 0.0);

        let fixed =
            MediaThumbnailSpec::new(MediaKind::Image).with_aspect_ratio(AspectRatio::Square);
        assert!(!fixed.is_auto());
        assert_eq!(fixed.aspect_ratio_pair(), (1.0, 1.0));
    }
}
