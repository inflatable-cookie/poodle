use poodle_tokens::semantic;

use crate::card::CardVariant;
use crate::composite_types::{AspectRatio, MediaKind, MediaState, RemediationAction};
use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MediaPreviewSpec {
    pub kind: MediaKind,
    pub state: MediaState,
    pub aspect_ratio: AspectRatio,
    pub title: String,
    pub description: Option<String>,
    pub eyebrow: Option<String>,
    pub caption: Option<String>,
    pub badge: Option<String>,
    pub thumbnail_meta: Option<String>,
    pub state_title: Option<String>,
    pub state_message: Option<String>,
    pub metadata: Vec<String>,
    pub variant: CardVariant,
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
    /// NOTE: not in the contract or Svelte. Retained as an additive field —
    /// removing it cascades into the GPUI preview (`demo_view.rs`) and the
    /// contracts smoke test, which cannot be build-verified in this pass. The
    /// component no longer renders a footer region; the parity follow-up is to
    /// drop this field once the preview can be rebuilt.
    pub footer_actions: Vec<RemediationAction>,
}

impl MediaPreviewSpec {
    pub fn new(kind: MediaKind, title: impl Into<String>) -> Self {
        Self {
            kind,
            state: MediaState::Ready,
            aspect_ratio: AspectRatio::Landscape,
            title: title.into(),
            description: None,
            eyebrow: None,
            caption: None,
            badge: None,
            thumbnail_meta: None,
            state_title: None,
            state_message: None,
            metadata: Vec::new(),
            variant: CardVariant::Default,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
            footer_actions: Vec::new(),
        }
    }

    pub fn with_state(mut self, state: MediaState) -> Self {
        self.state = state;
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_eyebrow(mut self, eyebrow: impl Into<String>) -> Self {
        self.eyebrow = Some(eyebrow.into());
        self
    }

    pub fn with_caption(mut self, caption: impl Into<String>) -> Self {
        self.caption = Some(caption.into());
        self
    }

    pub fn with_aspect_ratio(mut self, aspect_ratio: AspectRatio) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn with_badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    pub fn with_thumbnail_meta(mut self, thumbnail_meta: impl Into<String>) -> Self {
        self.thumbnail_meta = Some(thumbnail_meta.into());
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

    pub fn with_metadata(mut self, metadata: Vec<String>) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn with_variant(mut self, variant: CardVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = Some(density);
        self
    }

    pub fn with_footer_actions(mut self, footer_actions: Vec<RemediationAction>) -> Self {
        self.footer_actions = footer_actions;
        self
    }

    pub fn metadata_count(&self) -> usize {
        self.metadata.len()
    }

    pub fn has_footer_actions(&self) -> bool {
        !self.footer_actions.is_empty()
    }

    pub fn shows_fallback_copy(&self) -> bool {
        self.state != MediaState::Ready
    }

    // ── Card composition tokens (contract §10) ────────────────

    pub fn title_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn secondary_text_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn meta_fill_token(&self) -> &'static str {
        // Contract §9 Meta List Item: surface-mix; flat fallback = surface.
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn meta_radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    // ── Size variants (contract §9 Size Variants, values in rem) ──

    pub fn eyebrow_size_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.625,
            ControlSize::Sm => 0.65625,
            ControlSize::Md => 0.6875,
            ControlSize::Lg => 0.71875,
            ControlSize::Xl => 0.75,
        }
    }

    pub fn title_size_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.9375,
            ControlSize::Sm => 1.0,
            ControlSize::Md => 1.125,
            ControlSize::Lg => 1.1875,
            ControlSize::Xl => 1.25,
        }
    }

    pub fn body_size_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.75,
            ControlSize::Sm => 0.78125,
            ControlSize::Md => 0.8125,
            ControlSize::Lg => 0.875,
            ControlSize::Xl => 0.9375,
        }
    }

    /// Meta-chip padding `(y, x)` in rem (contract §9 Size Variants).
    pub fn meta_padding_rem(&self, size: ControlSize) -> (f32, f32) {
        match size {
            ControlSize::Xs => (0.25, 0.5),
            ControlSize::Sm => (0.3125, 0.5625),
            ControlSize::Md => (0.375, 0.625),
            ControlSize::Lg => (0.4375, 0.6875),
            ControlSize::Xl => (0.5, 0.75),
        }
    }

    // ── Density variants (contract §9 Density Variants, rem) ──

    pub fn header_gap_rem(&self, density: ControlDensity) -> f32 {
        match density {
            // default → space.stack.md (0.75rem)
            ControlDensity::Default => 0.75,
            ControlDensity::Compact => 0.625,
            ControlDensity::Comfortable => 1.0,
        }
    }

    pub fn section_gap_rem(&self, density: ControlDensity) -> f32 {
        match density {
            // default → space.stack.sm (0.5rem)
            ControlDensity::Default => 0.5,
            ControlDensity::Compact => 0.375,
            ControlDensity::Comfortable => 0.625,
        }
    }

    pub fn title_line_height(&self) -> f32 {
        1.2
    }
}
