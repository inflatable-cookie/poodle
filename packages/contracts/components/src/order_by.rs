use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum SortDirection {
    #[default]
    Asc,
    Desc,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum OrderByTriggerVariant {
    #[default]
    Summary,
    Icon,
}

#[derive(Clone, Debug)]
pub struct SortField {
    pub value: String,
    pub key: Option<String>,
    pub label: String,
    pub is_disabled: bool,
    pub default_direction: SortDirection,
}

impl SortField {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            key: None,
            label: label.into(),
            is_disabled: false,
            default_direction: SortDirection::Asc,
        }
    }

    pub fn with_key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn with_default_direction(mut self, default_direction: SortDirection) -> Self {
        self.default_direction = default_direction;
        self
    }

    pub fn resolved_key(&self) -> &str {
        self.key.as_deref().unwrap_or(&self.value)
    }
}

#[derive(Clone, Debug)]
pub struct ActiveSort {
    pub field: String,
    pub direction: SortDirection,
}

impl ActiveSort {
    pub fn new(field: impl Into<String>, direction: SortDirection) -> Self {
        Self {
            field: field.into(),
            direction,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrderByField {
    pub key: String,
    pub direction: SortDirection,
}

impl OrderByField {
    pub fn new(key: impl Into<String>, direction: SortDirection) -> Self {
        Self {
            key: key.into(),
            direction,
        }
    }
}

#[derive(Clone)]
pub struct OrderBySpec {
    pub fields: Vec<SortField>,
    pub value: Vec<OrderByField>,
    pub active_sort: Option<ActiveSort>,
    pub aria_label: String,
    pub is_disabled: bool,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
    pub max_fields: Option<usize>,
    pub compact: bool,
    pub trigger_variant: OrderByTriggerVariant,
    pub show_clear_button: bool,
    pub is_open: bool,
    /// Refuses outside-interact dismissal when false. Matches Svelte
    /// `dismissOnOutsideInteract` (default `true`).
    pub dismiss_on_outside_interact: bool,
}

impl Default for OrderBySpec {
    fn default() -> Self {
        Self::new()
    }
}

impl OrderBySpec {
    pub fn new() -> Self {
        Self {
            fields: Vec::new(),
            value: Vec::new(),
            active_sort: None,
            aria_label: "Sort by".to_string(),
            is_disabled: false,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
            max_fields: None,
            compact: false,
            trigger_variant: OrderByTriggerVariant::Summary,
            show_clear_button: true,
            is_open: false,
            dismiss_on_outside_interact: true,
        }
    }

    pub fn with_fields(mut self, fields: Vec<SortField>) -> Self {
        self.fields = fields;
        self
    }

    pub fn with_value(mut self, value: Vec<OrderByField>) -> Self {
        self.value = value;
        self
    }

    pub fn with_active_sort(mut self, active_sort: ActiveSort) -> Self {
        self.active_sort = Some(active_sort);
        self
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = label.into();
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn with_max_fields(mut self, max_fields: usize) -> Self {
        self.max_fields = Some(max_fields);
        self
    }

    pub fn with_compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    pub fn with_trigger_variant(mut self, trigger_variant: OrderByTriggerVariant) -> Self {
        self.trigger_variant = trigger_variant;
        self
    }

    pub fn with_show_clear_button(mut self, show_clear_button: bool) -> Self {
        self.show_clear_button = show_clear_button;
        self
    }

    pub fn with_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn with_dismiss_on_outside_interact(mut self, dismiss_on_outside_interact: bool) -> Self {
        self.dismiss_on_outside_interact = dismiss_on_outside_interact;
        self
    }

    pub fn normalized_fields(&self) -> Vec<SortField> {
        self.fields
            .iter()
            .filter(|field| !field.resolved_key().trim().is_empty())
            .cloned()
            .collect()
    }

    pub fn current_value(&self) -> Vec<OrderByField> {
        if !self.value.is_empty() {
            return self.value.clone();
        }

        self.active_sort
            .as_ref()
            .map(|active| {
                vec![OrderByField::new(
                    active.field.clone(),
                    active.direction.clone(),
                )]
            })
            .unwrap_or_default()
    }

    pub fn has_value(&self) -> bool {
        !self.current_value().is_empty()
    }

    pub fn active_count(&self) -> usize {
        self.current_value().len()
    }

    pub fn available_fields(&self) -> Vec<SortField> {
        let active_keys = self
            .current_value()
            .into_iter()
            .map(|item| item.key)
            .collect::<Vec<_>>();

        self.normalized_fields()
            .into_iter()
            .filter(|field| !active_keys.iter().any(|key| key == field.resolved_key()))
            .collect()
    }

    pub fn active_label(&self, key: &str) -> String {
        self.normalized_fields()
            .into_iter()
            .find(|field| field.resolved_key() == key)
            .map(|field| field.label)
            .unwrap_or_else(|| key.to_string())
    }

    pub fn active_direction(&self, key: &str) -> Option<SortDirection> {
        self.current_value()
            .iter()
            .find(|item| item.key == key)
            .map(|item| item.direction.clone())
    }

    pub fn summary_text(&self) -> String {
        let value = self.current_value();
        if value.is_empty() {
            return "Sort by...".to_string();
        }

        let render_item = |item: &OrderByField| {
            format!(
                "{} {}",
                self.active_label(&item.key),
                match item.direction {
                    SortDirection::Asc => "↑",
                    SortDirection::Desc => "↓",
                }
            )
        };

        if self.compact && value.len() > 2 {
            let head = value.iter().take(2).map(render_item).collect::<Vec<_>>();
            return format!("{} +{}", head.join(", "), value.len() - 2);
        }

        value.iter().map(render_item).collect::<Vec<_>>().join(", ")
    }

    pub fn label_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    /// Muted color for the summary placeholder text (empty state) and the
    /// drag handle glyph. Maps to Svelte `--poodle-color-text-muted`; the
    /// Rust token set exposes the equivalent placeholder color.
    pub fn muted_color_token(&self) -> &'static str {
        "color.text.placeholder"
    }

    /// Border color for an active sort item row (`border-subtle`).
    pub fn item_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    /// Surface radius for the dropdown surface.
    pub fn surface_radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    pub fn field_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn field_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn field_text_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn field_hover_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn active_fill_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn active_border_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn active_text_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }

    pub fn reset_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
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
