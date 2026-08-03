use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpinnerVariant {
    Ring,
    Grid,
    /// Three dots rising in sequence. The quietest of the three — it reads as
    /// "still working" beside text rather than as a control that is busy, which
    /// is what an agent's activity footer wants.
    Dots,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpinnerSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpinnerTone {
    Current,
    Accent,
    Muted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpinnerSpec {
    pub variant: SpinnerVariant,
    pub size: SpinnerSize,
    pub tone: SpinnerTone,
    pub aria_label: Option<String>,
    /// Presentation axes (contract §3): size is intrinsic, density is sibling
    /// spacing, size_role resolves size from the inherited presentation.
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for SpinnerSpec {
    fn default() -> Self {
        Self {
            variant: SpinnerVariant::Ring,
            size: SpinnerSize::Md,
            tone: SpinnerTone::Current,
            aria_label: None,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl SpinnerSpec {
    /// Size after the semantic role is applied — the same one-stop shift the
    /// control scale uses, so a spinner sitting in chrome matches the controls
    /// beside it.
    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }

    pub fn resolved_size(&self) -> SpinnerSize {
        let control = match self.size {
            SpinnerSize::Xs => ControlSize::Xs,
            SpinnerSize::Sm => ControlSize::Sm,
            SpinnerSize::Md => ControlSize::Md,
            SpinnerSize::Lg => ControlSize::Lg,
            SpinnerSize::Xl => ControlSize::Xl,
        };
        match crate::types::resolve_semantic_control_size(control, self.size_role) {
            ControlSize::Xs => SpinnerSize::Xs,
            ControlSize::Sm => SpinnerSize::Sm,
            ControlSize::Md => SpinnerSize::Md,
            ControlSize::Lg => SpinnerSize::Lg,
            ControlSize::Xl => SpinnerSize::Xl,
        }
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_variant(mut self, variant: SpinnerVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_size(mut self, size: SpinnerSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_tone(mut self, tone: SpinnerTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn size_px(&self) -> f32 {
        match self.resolved_size() {
            SpinnerSize::Xs => 10.0,
            SpinnerSize::Sm => 12.0,
            SpinnerSize::Md => 16.0,
            SpinnerSize::Lg => 24.0,
            SpinnerSize::Xl => 32.0,
        }
    }

    /// Ring diameter in rem (contract §7 ring sizes). `size_px()` returns the
    /// same values pre-multiplied by the 16px root; this is the rem origin so
    /// runtimes can resolve via their own `rem_to_px`.
    pub fn ring_size_rem(&self) -> f32 {
        match self.resolved_size() {
            SpinnerSize::Xs => 0.625,
            SpinnerSize::Sm => 0.75,
            SpinnerSize::Md => 1.0,
            SpinnerSize::Lg => 1.5,
            SpinnerSize::Xl => 1.875,
        }
    }

    /// Grid wrapper width in rem (contract §7 grid sizes).
    pub fn grid_width_rem(&self) -> f32 {
        match self.resolved_size() {
            SpinnerSize::Xs => 0.375,
            SpinnerSize::Sm => 0.4375,
            SpinnerSize::Md => 0.5625,
            SpinnerSize::Lg => 0.75,
            SpinnerSize::Xl => 0.9375,
        }
    }

    /// Grid wrapper height in rem (contract §7 grid sizes).
    pub fn grid_height_rem(&self) -> f32 {
        match self.resolved_size() {
            SpinnerSize::Xs => 0.5625,
            SpinnerSize::Sm => 0.6875,
            SpinnerSize::Md => 0.9375,
            SpinnerSize::Lg => 1.25,
            SpinnerSize::Xl => 1.5625,
        }
    }

    /// Per-size grid gap in rem (Svelte `--poodle-spinner-grid-gap`,
    /// `Spinner.svelte:91-119`). Contract §8 calls this a "size-dependent small
    /// fixed gap"; the rem values are the Svelte source of truth.
    pub fn grid_gap_rem(&self) -> f32 {
        match self.resolved_size() {
            SpinnerSize::Xs => 0.0625,
            SpinnerSize::Sm => 0.078125,
            SpinnerSize::Md => 0.09375,
            SpinnerSize::Lg => 0.125,
            SpinnerSize::Xl => 0.15625,
        }
    }

    /// Ring stroke / border width in rem. Contract §8 Ring Variant:
    /// `border: 0.125rem solid ...`.
    pub fn ring_border_width_rem(&self) -> f32 {
        0.125
    }

    /// Grid cell corner radius in rem. Contract §8 Grid Variant:
    /// `cell border-radius: 0.125rem`.
    pub fn cell_radius_rem(&self) -> f32 {
        0.125
    }

    /// Fraction of the tone color kept for the ring's track (non-arc) border.
    /// Contract §8: `color-mix(in srgb, currentColor 24%, transparent)`. CSS
    /// color-mix percent (not token-backed) — centralized so runtimes stop
    /// hardcoding the `0.24` literal.
    pub fn track_opacity(&self) -> f32 {
        0.24
    }

    /// Grid cell idle/baseline opacity floor. Contract §8: `cell idle opacity:
    /// 0.2` (the rest value and bottom of the `0.2 → 0.76` ramp band).
    pub fn opacity_floor(&self) -> f32 {
        0.2
    }

    /// Grid cell phase-peak opacity. Contract §8 `cell opacity range`: peaks at
    /// `0.76`.
    pub fn opacity_peak(&self) -> f32 {
        0.76
    }

    pub fn tone_color_token(&self) -> Option<&'static str> {
        match self.tone {
            SpinnerTone::Current => None,
            SpinnerTone::Accent => Some(semantic::COLOR_ACCENT_BASE),
            SpinnerTone::Muted => Some(semantic::COLOR_TEXT_SECONDARY),
        }
    }
}

#[cfg(test)]
mod size_role_tests {
    use super::*;

    /// The role shifts the spinner one stop, so a spinner in chrome matches the
    /// controls beside it instead of always rendering at its literal size.
    #[test]
    fn size_role_shifts_the_resolved_size() {
        let base = SpinnerSpec::new().with_size(SpinnerSize::Md);
        assert_eq!(base.resolved_size(), SpinnerSize::Md);

        let chrome = SpinnerSpec::new()
            .with_size(SpinnerSize::Md)
            .with_size_role(SemanticControlSizeRole::Chrome);
        assert_eq!(chrome.resolved_size(), SpinnerSize::Sm);

        let prominent = SpinnerSpec::new()
            .with_size(SpinnerSize::Md)
            .with_size_role(SemanticControlSizeRole::Prominent);
        assert_eq!(prominent.resolved_size(), SpinnerSize::Lg);
    }
}
