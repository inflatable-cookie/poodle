use poodle_headless::model_connection::ModelConnectionReadiness;

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};

/// ModelConnectionCard — a controlled disclosure card for one configured
/// model connection.
///
/// Contract: `docs/contracts/components/model-connection-card.md`
///
/// Safe summary data only. `access_summary` is the host's sanitised access
/// wording, never a credential or credential reference; `readiness` is a
/// supplied display posture, never something Poodle derives. Disclosure and
/// enabled state are independent and both host-owned: the web `defaultOpen`
/// seed has no Rust counterpart because the host holds `is_open` and rerenders
/// after `on_open_change`.
///
/// Provider marks, badges, the closed accessory, actions, and the details body
/// are host-composed content passed to the renderer, keyed by this card's
/// opaque `id`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelConnectionCardSpec {
    /// Opaque configured-connection id. Callback payloads and node ids only.
    pub id: String,
    /// Instance label, or the provider label when there is only one.
    pub title: String,
    pub provider_label: String,
    pub route_label: Option<String>,
    /// Observed safe version text.
    pub version: Option<String>,
    /// Ready-state status label; sanitised access summary.
    pub access_summary: Option<String>,
    pub readiness: ModelConnectionReadiness,
    pub readiness_label: String,
    /// Controlled disclosure.
    pub is_open: bool,
    /// Host preference. Independent of readiness and availability.
    pub is_enabled: bool,
    /// Disables only the enable Switch.
    pub is_enable_disabled: bool,
    /// Disables the card's controls; content stays readable.
    pub is_disabled: bool,
    pub aria_label: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for ModelConnectionCardSpec {
    fn default() -> Self {
        Self {
            id: String::new(),
            title: String::new(),
            provider_label: String::new(),
            route_label: None,
            version: None,
            access_summary: None,
            readiness: ModelConnectionReadiness::Unknown,
            readiness_label: "Status unknown".to_string(),
            is_open: false,
            is_enabled: true,
            is_enable_disabled: false,
            is_disabled: false,
            aria_label: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl ModelConnectionCardSpec {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        provider_label: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            provider_label: provider_label.into(),
            ..Self::default()
        }
    }

    pub fn with_route_label(mut self, route_label: impl Into<String>) -> Self {
        self.route_label = Some(route_label.into());
        self
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn with_access_summary(mut self, access_summary: impl Into<String>) -> Self {
        self.access_summary = Some(access_summary.into());
        self
    }

    pub fn with_readiness(
        mut self,
        readiness: ModelConnectionReadiness,
        readiness_label: impl Into<String>,
    ) -> Self {
        self.readiness = readiness;
        self.readiness_label = readiness_label.into();
        self
    }

    pub fn with_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn with_enabled(mut self, is_enabled: bool) -> Self {
        self.is_enabled = is_enabled;
        self
    }

    pub fn with_enable_disabled(mut self, is_enable_disabled: bool) -> Self {
        self.is_enable_disabled = is_enable_disabled;
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

    pub fn effective_aria_label(&self) -> &str {
        self.aria_label.as_deref().unwrap_or(&self.title)
    }

    /// The identity's quiet second line: route and version, in that order.
    /// Empty when the host supplied neither.
    pub fn meta_line(&self) -> String {
        [self.route_label.as_deref(), self.version.as_deref()]
            .into_iter()
            .flatten()
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join(" · ")
    }

    /// Instance-scoped details-region id, so repeated connection records never
    /// share one `controls` relationship.
    pub fn details_id(&self) -> String {
        format!("model-connection-card:{}:details", self.id)
    }

    /// Instance-scoped disclosure-control id, used as the focus destination
    /// when the details region closes.
    pub fn disclosure_id(&self) -> String {
        format!("model-connection-card:{}:disclosure", self.id)
    }

    /// The Switch's explicit accessible name.
    pub fn enable_label(&self) -> String {
        format!("Enable {}", self.title)
    }

    /// The disclosure button's accessible name, which states the action.
    pub fn disclosure_label(&self) -> String {
        if self.is_open {
            format!("Collapse {}", self.title)
        } else {
            format!("Expand {}", self.title)
        }
    }

    pub fn details_label(&self) -> String {
        format!("{} details", self.title)
    }
}
