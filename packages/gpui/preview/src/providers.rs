//! Context-provider boundaries for the GPUI preview.
//!
//! Contracts: `docs/contracts/components/icon-provider.md`,
//! `docs/contracts/components/ui-presentation-provider.md`.
//!
//! Both providers are pure context boundaries. `IconProvider`'s contract
//! anatomy is explicitly "Root (no DOM element)", and
//! `UiPresentationProvider`'s root exists only to carry CSS custom properties
//! that scope descendant sizing — neither paints anything of its own.
//!
//! GPUI has no CSS custom properties and resolves icons through one shared
//! registry, so there is nothing for a `poodle-render` recipe to emit: a Node
//! recipe here would have to invent chrome the contract does not describe.
//! These stay preview-local passthroughs, which is exactly what the old
//! `packages/gpui/components` tier did, and they outlive its deletion.

use gpui::{div, AnyElement, IntoElement};
use poodle_specs::UiPresentationProviderSpec;

/// Icon registry boundary. GPUI resolves icons through one shared registry, so
/// this renders its child unchanged.
pub(crate) struct IconProvider {
    child: Option<AnyElement>,
}

impl IconProvider {
    pub(crate) fn new() -> Self {
        Self { child: None }
    }

    pub(crate) fn with_child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }
}

impl IntoElement for IconProvider {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        self.child.unwrap_or_else(|| div().into_any_element())
    }
}

/// Presentation-context boundary. The spec is retained so the specimen keeps
/// declaring the scope it demonstrates, but GPUI descendants read size and
/// density from their own specs rather than an ambient cascade, so the
/// provider paints nothing.
pub(crate) struct UiPresentationProvider {
    _spec: UiPresentationProviderSpec,
    child: Option<AnyElement>,
}

impl UiPresentationProvider {
    pub(crate) fn from_spec(spec: UiPresentationProviderSpec) -> Self {
        Self {
            _spec: spec,
            child: None,
        }
    }

    pub(crate) fn with_child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }
}

impl IntoElement for UiPresentationProvider {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        self.child.unwrap_or_else(|| div().into_any_element())
    }
}
