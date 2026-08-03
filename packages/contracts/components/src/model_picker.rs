//! ModelPickerSpec — generic model + capability-axis selection model, Rust
//! mirror of the `@poodle/svelte` ModelPicker type model and
//! `model-picker-model.ts` logic.
//!
//! Contract: `docs/contracts/components/model-picker.md`.
//!
//! Poodle understands models and axes. It never understands vendor vocabulary:
//! "reasoning", "fast mode" and "context window" are host-declared axes, not
//! component concepts.

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

/// Trigger treatment. `Bare` is the borderless inline opener used inside
/// composer toolbars; `Outlined` draws the standard control border and fill.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ModelPickerVariant {
    #[default]
    Bare,
    Outlined,
}

impl ModelPickerVariant {
    pub fn as_str(self) -> &'static str {
        match self {
            ModelPickerVariant::Bare => "bare",
            ModelPickerVariant::Outlined => "outlined",
        }
    }
}

/// Trigger emphasis. `Default` is full-strength text; `Subdued` dims the label
/// and summary so the picker recedes beside a more important control — its home
/// in `AgentChatInput`, where the editor should hold the eye. Hover, focus and
/// the open state restore full strength. Emphasis only ever changes colour and
/// opacity: shifting font weight would reflow the label under the pointer.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ModelPickerEmphasis {
    #[default]
    Default,
    Subdued,
}

impl ModelPickerEmphasis {
    pub fn as_str(self) -> &'static str {
        match self {
            ModelPickerEmphasis::Default => "default",
            ModelPickerEmphasis::Subdued => "subdued",
        }
    }

    pub fn is_subdued(self) -> bool {
        matches!(self, ModelPickerEmphasis::Subdued)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModelAxisKind {
    Select,
    Toggle,
}

/// Control shape for a `Select` axis. `Auto` uses a segmented control up to
/// `SEGMENTED_AXIS_MAX_OPTIONS` options and a vertical list beyond that — a 6-
/// or 7-level scale cannot read as segments in the rail's width.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ModelAxisControl {
    #[default]
    Auto,
    Segmented,
    List,
}

/// Resolved control shape (contract §4).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModelAxisControlKind {
    Segmented,
    List,
}

/// Above this many options an `Auto` select axis renders as a list.
pub const SEGMENTED_AXIS_MAX_OPTIONS: usize = 3;

impl ModelAxisKind {
    pub fn as_str(self) -> &'static str {
        match self {
            ModelAxisKind::Select => "select",
            ModelAxisKind::Toggle => "toggle",
        }
    }
}

/// A selected axis value. Deliberately `String | bool` only — keeps the
/// selection trivially serializable and identical across renderers.
#[derive(Clone, Debug, PartialEq)]
pub enum ModelAxisValue {
    Text(String),
    Flag(bool),
}

impl ModelAxisValue {
    pub fn as_text(&self) -> Option<&str> {
        match self {
            ModelAxisValue::Text(value) => Some(value),
            ModelAxisValue::Flag(_) => None,
        }
    }

    pub fn as_flag(&self) -> Option<bool> {
        match self {
            ModelAxisValue::Flag(value) => Some(*value),
            ModelAxisValue::Text(_) => None,
        }
    }
}

/// An image source for a model row — a provider logo, typically. `alt` defaults
/// to empty: the model label sits beside it, so the image is decorative unless
/// the host says otherwise.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ModelImage {
    pub src: String,
    pub alt: Option<String>,
}

impl ModelImage {
    pub fn new(src: impl Into<String>) -> Self {
        Self {
            src: src.into(),
            alt: None,
        }
    }

    pub fn with_alt(mut self, alt: impl Into<String>) -> Self {
        self.alt = Some(alt.into());
        self
    }

    pub fn alt_text(&self) -> &str {
        self.alt.as_deref().unwrap_or("")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModelOption {
    pub value: String,
    pub label: String,
    pub description: Option<String>,
    pub badge: Option<String>,
    pub icon: Option<String>,
    /// Arbitrary image shown in place of `icon`. Takes precedence when both set.
    pub image: Option<ModelImage>,
    pub group: Option<String>,
    pub is_disabled: bool,
    /// Which capability axes this model exposes, by key, in display order.
    /// `None` inherits every declared axis — the single-provider case.
    pub axes: Option<Vec<ModelAxisRef>>,
}

impl ModelOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: None,
            badge: None,
            icon: None,
            image: None,
            group: None,
            is_disabled: false,
            axes: None,
        }
    }

    /// Declare the axes this model exposes, by key or binding, in display order.
    pub fn with_axes(mut self, axes: Vec<ModelAxisRef>) -> Self {
        self.axes = Some(axes);
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Show an arbitrary image (a provider logo) instead of a registry icon.
    pub fn with_image(mut self, image: ModelImage) -> Self {
        self.image = Some(image);
        self
    }

    pub fn with_group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModelAxisOption {
    pub value: String,
    pub label: String,
    pub description: Option<String>,
    pub is_disabled: bool,
}

impl ModelAxisOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: None,
            is_disabled: false,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }
}

/// A model's reference to a declared axis, overriding any part of it for that
/// model. Every field but `key` is optional and falls back to the shared
/// definition — a provider can swap the level set while inheriting the label,
/// kind and summary behaviour.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ModelAxisBinding {
    pub key: String,
    pub label: Option<String>,
    pub description: Option<String>,
    /// Empty means "inherit the shared option set".
    pub options: Vec<ModelAxisOption>,
    pub control: Option<ModelAxisControl>,
    pub default_value: Option<ModelAxisValue>,
    pub on_label: Option<String>,
    pub off_label: Option<String>,
    pub show_in_summary: Option<bool>,
    pub is_disabled: Option<bool>,
}

impl ModelAxisBinding {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            ..Self::default()
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_options(mut self, options: Vec<ModelAxisOption>) -> Self {
        self.options = options;
        self
    }

    pub fn with_control(mut self, control: ModelAxisControl) -> Self {
        self.control = Some(control);
        self
    }

    pub fn with_default_value(mut self, value: ModelAxisValue) -> Self {
        self.default_value = Some(value);
        self
    }

    pub fn with_labels(
        mut self,
        on_label: impl Into<String>,
        off_label: impl Into<String>,
    ) -> Self {
        self.on_label = Some(on_label.into());
        self.off_label = Some(off_label.into());
        self
    }

    pub fn with_show_in_summary(mut self, show: bool) -> Self {
        self.show_in_summary = Some(show);
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = Some(disabled);
        self
    }
}

/// How a model references an axis: by key, or by key plus overrides.
#[derive(Clone, Debug, PartialEq)]
pub enum ModelAxisRef {
    Key(String),
    Binding(ModelAxisBinding),
}

impl ModelAxisRef {
    pub fn key(&self) -> &str {
        match self {
            ModelAxisRef::Key(key) => key,
            ModelAxisRef::Binding(binding) => &binding.key,
        }
    }
}

impl From<&str> for ModelAxisRef {
    fn from(key: &str) -> Self {
        ModelAxisRef::Key(key.to_string())
    }
}

impl From<String> for ModelAxisRef {
    fn from(key: String) -> Self {
        ModelAxisRef::Key(key)
    }
}

impl From<ModelAxisBinding> for ModelAxisRef {
    fn from(binding: ModelAxisBinding) -> Self {
        ModelAxisRef::Binding(binding)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModelCapabilityAxis {
    pub key: String,
    pub label: String,
    pub kind: ModelAxisKind,
    pub description: Option<String>,
    /// `Select` axes only.
    pub options: Vec<ModelAxisOption>,
    /// Control shape for a `Select` axis.
    pub control: ModelAxisControl,
    pub default_value: Option<ModelAxisValue>,
    /// Summary labels for a `Toggle` axis.
    pub on_label: Option<String>,
    pub off_label: Option<String>,
    pub show_in_summary: bool,
    pub is_disabled: bool,
}

impl ModelCapabilityAxis {
    pub fn select(
        key: impl Into<String>,
        label: impl Into<String>,
        options: Vec<ModelAxisOption>,
    ) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            kind: ModelAxisKind::Select,
            description: None,
            options,
            control: ModelAxisControl::Auto,
            default_value: None,
            on_label: None,
            off_label: None,
            show_in_summary: true,
            is_disabled: false,
        }
    }

    pub fn toggle(key: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            kind: ModelAxisKind::Toggle,
            description: None,
            options: Vec::new(),
            control: ModelAxisControl::Auto,
            default_value: None,
            on_label: None,
            off_label: None,
            show_in_summary: true,
            is_disabled: false,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_control(mut self, control: ModelAxisControl) -> Self {
        self.control = control;
        self
    }

    /// Merge a binding over this definition. Only the fields the binding sets
    /// are overridden.
    pub fn with_binding(&self, binding: &ModelAxisBinding) -> Self {
        let mut merged = self.clone();
        if let Some(label) = &binding.label {
            merged.label = label.clone();
        }
        if binding.description.is_some() {
            merged.description = binding.description.clone();
        }
        if !binding.options.is_empty() {
            merged.options = binding.options.clone();
        }
        if let Some(control) = binding.control {
            merged.control = control;
        }
        if binding.default_value.is_some() {
            merged.default_value = binding.default_value.clone();
        }
        if binding.on_label.is_some() {
            merged.on_label = binding.on_label.clone();
        }
        if binding.off_label.is_some() {
            merged.off_label = binding.off_label.clone();
        }
        if let Some(show) = binding.show_in_summary {
            merged.show_in_summary = show;
        }
        if let Some(disabled) = binding.is_disabled {
            merged.is_disabled = disabled;
        }
        merged
    }

    /// Which control this axis renders as: the explicit hint, else the
    /// option-count rule (contract §4).
    pub fn control_kind(&self) -> ModelAxisControlKind {
        if self.kind == ModelAxisKind::Toggle {
            return ModelAxisControlKind::Segmented;
        }
        match self.control {
            ModelAxisControl::Segmented => ModelAxisControlKind::Segmented,
            ModelAxisControl::List => ModelAxisControlKind::List,
            ModelAxisControl::Auto => {
                if self.options.len() > SEGMENTED_AXIS_MAX_OPTIONS {
                    ModelAxisControlKind::List
                } else {
                    ModelAxisControlKind::Segmented
                }
            }
        }
    }

    pub fn with_default_value(mut self, value: ModelAxisValue) -> Self {
        self.default_value = Some(value);
        self
    }

    pub fn with_labels(
        mut self,
        on_label: impl Into<String>,
        off_label: impl Into<String>,
    ) -> Self {
        self.on_label = Some(on_label.into());
        self.off_label = Some(off_label.into());
        self
    }

    pub fn with_show_in_summary(mut self, show: bool) -> Self {
        self.show_in_summary = show;
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    /// The value a fresh selection should take: declared default, else the first
    /// enabled option (`Select`) or `false` (`Toggle`).
    pub fn resolved_default(&self) -> ModelAxisValue {
        if let Some(default) = &self.default_value {
            if self.accepts(default) {
                return default.clone();
            }
        }
        match self.kind {
            ModelAxisKind::Select => ModelAxisValue::Text(
                self.options
                    .iter()
                    .find(|option| !option.is_disabled)
                    .map(|option| option.value.clone())
                    .unwrap_or_default(),
            ),
            ModelAxisKind::Toggle => ModelAxisValue::Flag(false),
        }
    }

    /// Whether a value is representable on this axis.
    pub fn accepts(&self, value: &ModelAxisValue) -> bool {
        match (self.kind, value) {
            (ModelAxisKind::Select, ModelAxisValue::Text(text)) => {
                self.options.iter().any(|option| &option.value == text)
            }
            (ModelAxisKind::Toggle, ModelAxisValue::Flag(_)) => true,
            _ => false,
        }
    }

    /// This axis' contribution to the trigger summary, if any.
    pub fn summary_fragment(&self, value: &ModelAxisValue) -> Option<String> {
        if !self.show_in_summary {
            return None;
        }
        match (self.kind, value) {
            (ModelAxisKind::Select, ModelAxisValue::Text(text)) => self
                .options
                .iter()
                .find(|option| &option.value == text)
                .map(|option| option.label.clone()),
            (ModelAxisKind::Toggle, ModelAxisValue::Flag(flag)) => {
                if *flag {
                    self.on_label.clone()
                } else {
                    self.off_label.clone()
                }
            }
            _ => None,
        }
    }
}

/// The committed selection: a model plus its applicable axis values.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ModelSelection {
    pub model: String,
    /// Ordered pairs rather than a map — keeps the emission order stable and
    /// matches the axis declaration order used by the summary.
    pub axes: Vec<(String, ModelAxisValue)>,
}

impl ModelSelection {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            axes: Vec::new(),
        }
    }

    pub fn with_axis(mut self, key: impl Into<String>, value: ModelAxisValue) -> Self {
        let key = key.into();
        match self.axes.iter_mut().find(|(existing, _)| existing == &key) {
            Some(slot) => slot.1 = value,
            None => self.axes.push((key, value)),
        }
        self
    }

    pub fn axis(&self, key: &str) -> Option<&ModelAxisValue> {
        self.axes
            .iter()
            .find(|(existing, _)| existing == key)
            .map(|(_, value)| value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModelPickerSpec {
    pub models: Vec<ModelOption>,
    pub axes: Vec<ModelCapabilityAxis>,
    pub value: ModelSelection,
    pub placeholder: String,
    pub aria_label: String,
    pub is_disabled: bool,
    pub show_axis_summary: bool,
    pub show_model_descriptions: bool,
    pub variant: ModelPickerVariant,
    pub emphasis: ModelPickerEmphasis,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
    pub is_open: bool,
}

impl Default for ModelPickerSpec {
    fn default() -> Self {
        Self::new()
    }
}

impl ModelPickerSpec {
    pub fn new() -> Self {
        Self {
            models: Vec::new(),
            axes: Vec::new(),
            value: ModelSelection::default(),
            placeholder: "Select model".to_string(),
            aria_label: "Model".to_string(),
            is_disabled: false,
            show_axis_summary: true,
            show_model_descriptions: true,
            variant: ModelPickerVariant::Bare,
            emphasis: ModelPickerEmphasis::Default,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
            is_open: false,
        }
    }

    pub fn with_models(mut self, models: Vec<ModelOption>) -> Self {
        self.models = models;
        self
    }

    pub fn with_axes(mut self, axes: Vec<ModelCapabilityAxis>) -> Self {
        self.axes = axes;
        self
    }

    pub fn with_value(mut self, value: ModelSelection) -> Self {
        self.value = value;
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = aria_label.into();
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn with_show_axis_summary(mut self, show: bool) -> Self {
        self.show_axis_summary = show;
        self
    }

    pub fn with_show_model_descriptions(mut self, show: bool) -> Self {
        self.show_model_descriptions = show;
        self
    }

    pub fn with_variant(mut self, variant: ModelPickerVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_emphasis(mut self, emphasis: ModelPickerEmphasis) -> Self {
        self.emphasis = emphasis;
        self
    }

    /// Trigger label colour: secondary while subdued and at rest, primary
    /// otherwise. Native targets have no hover state at spec-resolution time, so
    /// they render the resting treatment (contract §12).
    pub fn trigger_label_color_token(&self) -> &'static str {
        if self.emphasis.is_subdued() {
            semantic::COLOR_TEXT_SECONDARY
        } else {
            semantic::COLOR_TEXT_PRIMARY
        }
    }

    /// Opacity applied to the trigger's icon, summary and chevron while subdued.
    pub fn trigger_subdued_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_MUTED
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

    pub fn with_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn model(&self, value: &str) -> Option<&ModelOption> {
        self.models.iter().find(|model| model.value == value)
    }

    pub fn selected_model(&self) -> Option<&ModelOption> {
        self.model(&self.value.model)
    }

    pub fn has_selection(&self) -> bool {
        !self.value.model.is_empty()
    }

    /// Trigger label: the selected option's label, the raw value when the host
    /// holds a model outside the current list, else the placeholder.
    pub fn trigger_label(&self) -> String {
        if !self.has_selection() {
            return self.placeholder.clone();
        }
        match self.selected_model() {
            Some(model) => model.label.clone(),
            None => self.value.model.clone(),
        }
    }

    /// The axes a given model exposes, merged with its bindings, in the order
    /// the model declares them. A model with no `axes` declaration inherits
    /// every axis in declaration order — the single-provider case. References to
    /// undeclared keys are dropped.
    pub fn axes_for_model(&self, model: Option<&ModelOption>) -> Vec<ModelCapabilityAxis> {
        let Some(refs) = model.and_then(|model| model.axes.as_ref()) else {
            return self.axes.clone();
        };
        refs.iter()
            .filter_map(|axis_ref| {
                let base = self.axes.iter().find(|axis| axis.key == axis_ref.key())?;
                Some(match axis_ref {
                    ModelAxisRef::Key(_) => base.clone(),
                    ModelAxisRef::Binding(binding) => base.with_binding(binding),
                })
            })
            .collect()
    }

    /// Axes that apply to the currently selected model.
    pub fn applicable_axes(&self) -> Vec<ModelCapabilityAxis> {
        if self.value.model.is_empty() {
            return Vec::new();
        }
        self.axes_for_model(self.selected_model())
    }

    /// The effective value for an axis: the held value when representable, else
    /// the axis' resolved default.
    pub fn axis_value(&self, axis: &ModelCapabilityAxis) -> ModelAxisValue {
        match self.value.axis(&axis.key) {
            Some(value) if axis.accepts(value) => value.clone(),
            _ => axis.resolved_default(),
        }
    }

    /// Contract §4: drop scoped-out axes, fill applicable ones with defaults.
    pub fn resolved_selection(&self) -> ModelSelection {
        let mut resolved = ModelSelection::new(self.value.model.clone());
        for axis in self.applicable_axes() {
            resolved
                .axes
                .push((axis.key.clone(), self.axis_value(&axis)));
        }
        resolved
    }

    /// Trigger summary: applicable axis fragments joined with " · ".
    pub fn summary_text(&self) -> String {
        if !self.show_axis_summary || !self.has_selection() {
            return String::new();
        }
        self.applicable_axes()
            .into_iter()
            .filter_map(|axis| {
                let value = self.axis_value(&axis);
                axis.summary_fragment(&value)
            })
            .collect::<Vec<_>>()
            .join(" · ")
    }

    /// Full accessible name for the trigger (contract §6).
    pub fn trigger_aria_label(&self) -> String {
        let summary = self.summary_text();
        if summary.is_empty() {
            format!("{}: {}", self.aria_label, self.trigger_label())
        } else {
            format!("{}: {}, {}", self.aria_label, self.trigger_label(), summary)
        }
    }

    /// Group heading to emit before this option, if it opens a new group.
    pub fn group_heading_for(&self, index: usize) -> Option<&str> {
        let group = self.models.get(index)?.group.as_deref()?;
        let previous = index
            .checked_sub(1)
            .and_then(|prev| self.models.get(prev))
            .and_then(|model| model.group.as_deref());
        if previous == Some(group) {
            None
        } else {
            Some(group)
        }
    }

    // ── Token accessors (shared by GPUI + Jetstream) ──────────────────────

    pub fn label_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn muted_color_token(&self) -> &'static str {
        "color.text.placeholder"
    }

    pub fn secondary_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn trigger_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn trigger_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn surface_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn surface_radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    pub fn item_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn selected_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn effort_axis() -> ModelCapabilityAxis {
        ModelCapabilityAxis::select(
            "effort",
            "Effort",
            vec![
                ModelAxisOption::new("low", "Low"),
                ModelAxisOption::new("high", "High"),
            ],
        )
        .with_default_value(ModelAxisValue::Text("low".into()))
    }

    fn speed_axis() -> ModelCapabilityAxis {
        ModelCapabilityAxis::toggle("speed", "Speed").with_labels("Fast", "Normal")
    }

    fn sample() -> ModelPickerSpec {
        ModelPickerSpec::new()
            .with_models(vec![
                // `big` exposes both axes; `small` only the effort scale.
                ModelOption::new("big", "Big Model")
                    .with_group("Frontier")
                    .with_axes(vec!["effort".into(), "speed".into()]),
                ModelOption::new("small", "Small Model")
                    .with_group("Frontier")
                    .with_axes(vec!["effort".into()]),
                ModelOption::new("legacy", "Legacy")
                    .with_group("Archive")
                    .with_disabled(true),
            ])
            .with_axes(vec![effort_axis(), speed_axis()])
            .with_value(ModelSelection::new("big"))
    }

    #[test]
    fn per_model_axis_refs_drop_axes_the_model_does_not_expose() {
        let mut spec = sample();
        assert_eq!(spec.applicable_axes().len(), 2);

        // `speed` is scoped to `big` only.
        spec.value = ModelSelection::new("small");
        let keys: Vec<_> = spec
            .applicable_axes()
            .iter()
            .map(|a| a.key.clone())
            .collect();
        assert_eq!(keys, vec!["effort".to_string()]);

        // A held value for a scoped-out axis never survives resolution.
        spec.value = ModelSelection::new("small").with_axis("speed", ModelAxisValue::Flag(true));
        let resolved = spec.resolved_selection();
        assert!(resolved.axis("speed").is_none());
        assert_eq!(
            resolved.axis("effort"),
            Some(&ModelAxisValue::Text("low".into()))
        );
    }

    #[test]
    fn bindings_override_only_what_they_set() {
        // Two providers, one shared `effort` key, different level sets — the
        // host still reads `selection.axes["effort"]` either way.
        let spec = ModelPickerSpec::new()
            .with_models(vec![
                ModelOption::new("anthropic", "Anthropic").with_axes(vec![ModelAxisBinding::new(
                    "effort",
                )
                .with_options(vec![
                    ModelAxisOption::new("low", "Low"),
                    ModelAxisOption::new("high", "High"),
                    ModelAxisOption::new("max", "Max"),
                ])
                .with_default_value(ModelAxisValue::Text("high".into()))
                .into()]),
                ModelOption::new("openai", "OpenAI").with_axes(vec![ModelAxisBinding::new(
                    "effort",
                )
                .with_options(vec![
                    ModelAxisOption::new("minimal", "Minimal"),
                    ModelAxisOption::new("medium", "Medium"),
                ])
                .into()]),
            ])
            .with_axes(vec![ModelCapabilityAxis::select(
                "effort",
                "Effort",
                vec![ModelAxisOption::new("low", "Low")],
            )])
            .with_value(ModelSelection::new("anthropic"));

        let anthropic = spec.applicable_axes();
        assert_eq!(anthropic.len(), 1);
        // The binding swapped the option set but inherited the shared label.
        assert_eq!(anthropic[0].label, "Effort");
        assert_eq!(anthropic[0].options.len(), 3);
        assert_eq!(spec.summary_text(), "High");

        // Switching provider re-resolves the axis and drops the level that only
        // exists on the other one.
        let openai = spec.clone().with_value(
            ModelSelection::new("openai").with_axis("effort", ModelAxisValue::Text("max".into())),
        );
        assert_eq!(openai.applicable_axes()[0].options.len(), 2);
        assert_eq!(
            openai.resolved_selection().axis("effort"),
            Some(&ModelAxisValue::Text("minimal".into()))
        );
        assert_eq!(openai.summary_text(), "Minimal");
    }

    #[test]
    fn unknown_axis_references_are_dropped() {
        let spec = ModelPickerSpec::new()
            .with_models(vec![
                ModelOption::new("m", "M").with_axes(vec!["effort".into(), "ghost".into()])
            ])
            .with_axes(vec![effort_axis()])
            .with_value(ModelSelection::new("m"));
        let keys: Vec<_> = spec
            .applicable_axes()
            .iter()
            .map(|a| a.key.clone())
            .collect();
        assert_eq!(keys, vec!["effort".to_string()]);
    }

    #[test]
    fn a_model_without_axis_refs_inherits_every_axis() {
        let spec = ModelPickerSpec::new()
            .with_models(vec![ModelOption::new("m", "M")])
            .with_axes(vec![effort_axis(), speed_axis()])
            .with_value(ModelSelection::new("m"));
        assert_eq!(spec.applicable_axes().len(), 2);
    }

    #[test]
    fn control_kind_switches_to_a_list_past_the_option_threshold() {
        let three = ModelCapabilityAxis::select(
            "effort",
            "Effort",
            (0..3)
                .map(|n| ModelAxisOption::new(n.to_string(), n.to_string()))
                .collect(),
        );
        assert_eq!(three.control_kind(), ModelAxisControlKind::Segmented);

        let seven = ModelCapabilityAxis::select(
            "effort",
            "Effort",
            (0..7)
                .map(|n| ModelAxisOption::new(n.to_string(), n.to_string()))
                .collect(),
        );
        assert_eq!(seven.control_kind(), ModelAxisControlKind::List);

        // The explicit hint wins in both directions.
        assert_eq!(
            seven
                .clone()
                .with_control(ModelAxisControl::Segmented)
                .control_kind(),
            ModelAxisControlKind::Segmented
        );
        assert_eq!(
            three.with_control(ModelAxisControl::List).control_kind(),
            ModelAxisControlKind::List
        );
        // Toggles are never lists.
        assert_eq!(
            ModelCapabilityAxis::toggle("fast", "Fast").control_kind(),
            ModelAxisControlKind::Segmented
        );
    }

    #[test]
    fn defaults_fall_back_to_first_enabled_option() {
        let axis = ModelCapabilityAxis::select(
            "effort",
            "Effort",
            vec![
                ModelAxisOption::new("low", "Low").with_disabled(true),
                ModelAxisOption::new("high", "High"),
            ],
        );
        assert_eq!(axis.resolved_default(), ModelAxisValue::Text("high".into()));
        assert_eq!(
            ModelCapabilityAxis::toggle("speed", "Speed").resolved_default(),
            ModelAxisValue::Flag(false)
        );
    }

    #[test]
    fn unrepresentable_values_fall_back_to_the_default() {
        let spec = sample().with_value(
            ModelSelection::new("big").with_axis("effort", ModelAxisValue::Text("nope".into())),
        );
        let effort = spec.applicable_axes()[0].clone();
        assert_eq!(spec.axis_value(&effort), ModelAxisValue::Text("low".into()));
    }

    #[test]
    fn summary_joins_applicable_axes_in_declaration_order() {
        let spec = sample().with_value(
            ModelSelection::new("big")
                .with_axis("effort", ModelAxisValue::Text("high".into()))
                .with_axis("speed", ModelAxisValue::Flag(true)),
        );
        assert_eq!(spec.summary_text(), "High · Fast");
        assert_eq!(spec.trigger_aria_label(), "Model: Big Model, High · Fast");

        // Suppressed summary → trigger carries the model alone.
        let quiet = spec.clone().with_show_axis_summary(false);
        assert_eq!(quiet.summary_text(), "");
        assert_eq!(quiet.trigger_aria_label(), "Model: Big Model");
    }

    #[test]
    fn toggle_without_labels_contributes_nothing() {
        let spec = ModelPickerSpec::new()
            .with_models(vec![ModelOption::new("m", "M")])
            .with_axes(vec![ModelCapabilityAxis::toggle("speed", "Speed")])
            .with_value(ModelSelection::new("m").with_axis("speed", ModelAxisValue::Flag(true)));
        assert_eq!(spec.summary_text(), "");
    }

    #[test]
    fn subdued_emphasis_dims_the_resting_trigger_label() {
        let spec = sample();
        assert_eq!(
            spec.trigger_label_color_token(),
            semantic::COLOR_TEXT_PRIMARY
        );
        assert_eq!(
            spec.with_emphasis(ModelPickerEmphasis::Subdued)
                .trigger_label_color_token(),
            semantic::COLOR_TEXT_SECONDARY
        );
    }

    #[test]
    fn placeholder_and_unknown_model_labels() {
        let empty = sample().with_value(ModelSelection::default());
        assert_eq!(empty.trigger_label(), "Select model");
        assert!(!empty.has_selection());

        let unknown = sample().with_value(ModelSelection::new("ghost"));
        assert_eq!(unknown.trigger_label(), "ghost");
    }

    #[test]
    fn an_image_takes_precedence_over_an_icon() {
        let model = ModelOption::new("m", "M")
            .with_icon("sparkles")
            .with_image(ModelImage::new("/logos/m.svg"));
        // Both set: the renderer draws the image and ignores the icon name.
        assert!(model.image.is_some());
        assert_eq!(model.image.as_ref().unwrap().alt_text(), "");
        assert_eq!(
            ModelImage::new("/logos/m.svg")
                .with_alt("M logo")
                .alt_text(),
            "M logo"
        );
    }

    #[test]
    fn group_headings_emit_once_per_run() {
        let spec = sample();
        assert_eq!(spec.group_heading_for(0), Some("Frontier"));
        assert_eq!(spec.group_heading_for(1), None);
        assert_eq!(spec.group_heading_for(2), Some("Archive"));
    }
}
