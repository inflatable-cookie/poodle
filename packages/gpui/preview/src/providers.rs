//! Context-provider boundaries for the GPUI preview.
//!
//! Contracts: `docs/contracts/components/icon-provider.md`,
//! `docs/contracts/components/ui-presentation-provider.md`.
//!
//! `IconProvider`'s contract anatomy is explicitly "Root (no DOM element)".
//! GPUI has no CSS custom properties and resolves icons through one shared
//! registry, so there is nothing for a `poodle-render` recipe to emit: a Node
//! recipe here would have to invent chrome the contract does not describe.
//! It stays a preview-local passthrough, which is exactly what the old
//! `packages/gpui/components` tier did, and it outlives its deletion.
//!
//! `UiPresentationProvider` is NOT here: since g15.043 (architecture 010) the
//! presentation cascade is construction-time and shared — see
//! `poodle_render::context::ui_presentation_provider`, which builds a child
//! inside a scoped `RenderContext` and returns it unchanged. A preview-local
//! facade could only wrap an already-built element, which is exactly the
//! manual-equivalent passthrough the architecture forbids.

use gpui::{div, AnyElement, IntoElement};

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
