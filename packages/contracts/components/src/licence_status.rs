use poodle_headless::licence::{LicenceAttention, LicenceTrustBasis, LicenceUsability};

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};

/// LicenceStatus — a window onto the held licence: usability, trust basis,
/// and both coverage windows. Reports state; never enforces entitlement.
///
/// Contract: `docs/contracts/components/licence-status.md`
///
/// `usable` and `attention` are authority reads echoed as data state only —
/// no branch below hides a row, disables a control, or turns a licence read
/// into a feature permission. Display decisions (title, tone, row terms,
/// timestamps) resolve once through
/// `poodle_headless::licence::licence_status_view`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LicenceStatusSpec {
    /// Authority projection; every state renders distinctly.
    pub usability: LicenceUsability,
    /// Shown quietly; contains no credential.
    pub trust_basis: LicenceTrustBasis,
    /// Authority timestamp in integer Unix seconds.
    pub use_until: Option<i64>,
    /// Authority timestamp in integer Unix seconds.
    pub update_until: Option<i64>,
    /// Reported through copy/data state only; never gates child controls.
    pub usable: bool,
    /// Authority emphasis; not re-derived.
    pub attention: LicenceAttention,
    pub title: String,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for LicenceStatusSpec {
    fn default() -> Self {
        Self {
            usability: LicenceUsability::Active,
            trust_basis: LicenceTrustBasis::OfflineSignature,
            use_until: None,
            update_until: None,
            usable: true,
            attention: LicenceAttention::None,
            title: "Licence".to_string(),
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl LicenceStatusSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_usability(mut self, usability: LicenceUsability) -> Self {
        self.usability = usability;
        self
    }

    pub fn with_trust_basis(mut self, trust_basis: LicenceTrustBasis) -> Self {
        self.trust_basis = trust_basis;
        self
    }

    pub fn with_use_until(mut self, use_until: Option<i64>) -> Self {
        self.use_until = use_until;
        self
    }

    pub fn with_update_until(mut self, update_until: Option<i64>) -> Self {
        self.update_until = update_until;
        self
    }

    pub fn with_usable(mut self, usable: bool) -> Self {
        self.usable = usable;
        self
    }

    pub fn with_attention(mut self, attention: LicenceAttention) -> Self {
        self.attention = attention;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
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
}
