//! `BadgeSpec` — no Svelte/web contract. Actively used in Jetstream workstation
//! rendering (`render_feedback.rs`). If a web component is added, create
//! `docs/contracts/components/badge.md` to match.

use poodle_tokens::semantic;

use crate::types::BadgeVariant;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BadgeSpec {
    pub variant: BadgeVariant,
    pub aria_label: Option<String>,
    pub content: Option<String>,
}

impl Default for BadgeSpec {
    fn default() -> Self {
        Self {
            variant: BadgeVariant::Accent,
            aria_label: None,
            content: None,
        }
    }
}

impl BadgeSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn fill_token(&self) -> &'static str {
        match self.variant {
            BadgeVariant::Accent => semantic::COLOR_ACCENT_BASE,
            BadgeVariant::Muted => semantic::COLOR_BACKGROUND_SURFACE,
        }
    }
}
