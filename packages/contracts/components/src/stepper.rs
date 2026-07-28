use poodle_tokens::semantic;

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};

/// What a step is doing, independent of where it sits.
///
/// **Never derive this from position.** `index < current` is the obvious
/// implementation and it is wrong for any wizard whose steps do work: a step
/// that ran for two minutes and was rejected by a gate has to read as `Failed`,
/// where position-derived state renders it as "not yet reached" — actively
/// misleading rather than merely imprecise. See `stepper.md` §1.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum StepStatus {
    #[default]
    Pending,
    Running,
    Complete,
    Failed,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct StepperStep {
    pub value: String,
    pub label: String,
    pub status: StepStatus,
    pub is_disabled: bool,
    pub description: Option<String>,
}

impl StepperStep {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            ..Self::default()
        }
    }

    pub fn with_status(mut self, status: StepStatus) -> Self {
        self.status = status;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StepperSpec {
    pub steps: Vec<StepperStep>,
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
    /// Accessible name for the re-run control, suffixed with the step label.
    pub rerun_label: String,
    /// Whether completed steps offer a re-run control.
    ///
    /// Separate from the trigger by design: re-running spends whatever the step
    /// costs, so it cannot be something a user does by clicking to look at a
    /// finished step. See `stepper.md` §2.
    pub show_rerun: bool,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for StepperSpec {
    fn default() -> Self {
        Self {
            steps: Vec::new(),
            value: None,
            default_value: None,
            is_disabled: false,
            aria_label: None,
            rerun_label: "Re-run step".to_string(),
            show_rerun: false,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl StepperSpec {
    pub fn new(steps: Vec<StepperStep>) -> Self {
        Self {
            steps,
            ..Self::default()
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_default_value(mut self, default_value: impl Into<String>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_rerun_label(mut self, rerun_label: impl Into<String>) -> Self {
        self.rerun_label = rerun_label.into();
        self
    }

    pub fn with_show_rerun(mut self, show_rerun: bool) -> Self {
        self.show_rerun = show_rerun;
        self
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

    /// The step that is current, falling back to the first.
    ///
    /// A stepper with no current step renders every row as "not here", which is
    /// never what a wizard means.
    pub fn current_value(&self) -> Option<&str> {
        self.value
            .as_deref()
            .or(self.default_value.as_deref())
            .or_else(|| self.steps.first().map(|step| step.value.as_str()))
    }

    pub fn resolved_size(&self) -> ControlSize {
        crate::types::resolve_semantic_control_size(self.size, self.size_role)
    }

    // ── Token targets ──

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn surface_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    pub fn label_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn active_label_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn accent_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn danger_token(&self) -> &'static str {
        semantic::COLOR_STATUS_DANGER
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    /// Marker colour for a step, by status and whether it is current.
    ///
    /// Current and status compose rather than exclude: a step that is both the
    /// one you are on and the one that broke reports danger, because that is
    /// the more urgent of the two facts.
    pub fn marker_token(&self, status: StepStatus, is_current: bool) -> &'static str {
        match status {
            StepStatus::Failed => self.danger_token(),
            StepStatus::Complete | StepStatus::Running => self.accent_token(),
            StepStatus::Pending if is_current => self.accent_token(),
            StepStatus::Pending => self.label_token(),
        }
    }

    // ── Size ladder (rem), from `stepper.md` §8 ──

    pub fn row_height_rem(&self) -> f32 {
        match self.resolved_size() {
            ControlSize::Xs => 2.5,
            ControlSize::Sm => 2.875,
            ControlSize::Md => 3.25,
            ControlSize::Lg => 3.625,
            ControlSize::Xl => 4.0,
        }
    }

    pub fn marker_size_rem(&self) -> f32 {
        match self.resolved_size() {
            ControlSize::Xs => 1.125,
            ControlSize::Sm => 1.25,
            ControlSize::Md => 1.35,
            ControlSize::Lg => 1.5,
            ControlSize::Xl => 1.75,
        }
    }

    pub fn font_size_rem(&self) -> f32 {
        match self.resolved_size() {
            ControlSize::Xs => 0.625,
            ControlSize::Sm => 0.6875,
            ControlSize::Md => 0.75,
            ControlSize::Lg => 0.8125,
            ControlSize::Xl => 0.875,
        }
    }

    pub fn marker_font_size_rem(&self) -> f32 {
        match self.resolved_size() {
            ControlSize::Xs => 0.5625,
            ControlSize::Sm => 0.5625,
            ControlSize::Md => 0.625,
            ControlSize::Lg => 0.6875,
            ControlSize::Xl => 0.75,
        }
    }

    pub fn padding_block_rem(&self) -> f32 {
        match self.resolved_size() {
            ControlSize::Xs => 0.5,
            ControlSize::Sm => 0.625,
            ControlSize::Md => 0.7,
            ControlSize::Lg => 0.8,
            ControlSize::Xl => 0.9,
        }
    }

    // ── Density: horizontal spacing only, never height ──

    pub fn padding_inline_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.625,
            ControlDensity::Default => 0.8,
            ControlDensity::Comfortable => 1.0,
        }
    }

    pub fn gap_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.4375,
            ControlDensity::Default => 0.55,
            ControlDensity::Comfortable => 0.6875,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The whole reason `status` exists as a field.
    #[test]
    fn a_failed_step_reports_danger_wherever_it_sits() {
        let spec = StepperSpec::new(vec![
            StepperStep::new("a", "Read").with_status(StepStatus::Complete),
            StepperStep::new("b", "Gate").with_status(StepStatus::Failed),
            StepperStep::new("c", "Apply"),
        ])
        .with_value("c");

        // `b` sits behind the current step, so anything deriving state from
        // position would call it complete.
        assert_eq!(
            spec.marker_token(StepStatus::Failed, false),
            spec.danger_token()
        );
        assert_eq!(
            spec.marker_token(StepStatus::Failed, true),
            spec.danger_token(),
            "current does not override failed — the breakage is the more urgent fact"
        );
    }

    #[test]
    fn current_falls_back_to_the_first_step() {
        let spec = StepperSpec::new(vec![
            StepperStep::new("a", "One"),
            StepperStep::new("b", "Two"),
        ]);
        assert_eq!(spec.current_value(), Some("a"));
        assert_eq!(spec.with_value("b").current_value(), Some("b"));
    }

    /// Density must not change height — the size/density contract.
    #[test]
    fn density_moves_only_horizontal_spacing() {
        let base = StepperSpec::new(vec![StepperStep::new("a", "One")]);
        let dense = base.clone().with_density(ControlDensity::Compact);
        assert_eq!(dense.row_height_rem(), base.row_height_rem());
        assert_eq!(dense.padding_block_rem(), base.padding_block_rem());
        assert!(dense.padding_inline_rem() < base.padding_inline_rem());
    }
}
