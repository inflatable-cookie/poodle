//! Construction-time render context — the native presentation cascade.
//!
//! Architecture: `docs/architecture/010-native-presentation-construction-context.md`.
//! Contract: `docs/contracts/components/ui-presentation-provider.md`.
//!
//! Every public component renderer receives one explicit borrowed
//! `RenderContext` instead of a bare `&dyn ThemeProvider`. The context carries
//! the token-only theme plus the effective UI-presentation defaults (semantic
//! control-size scale and density) that a `UiPresentationProvider` scope
//! establishes. Component specs keep omission in the type system
//! (`Option<ControlSize>` / `Option<ControlDensity>`); a renderer resolves an
//! omitted input from this context, then applies the component's semantic size
//! role. An explicit value always wins — including an explicit `md` or
//! `default` inside a non-default scope.
//!
//! The context is a plain borrowed value. There is no global, thread-local,
//! task-local, backend, or `poodle-node` presentation state anywhere in the
//! system; nesting works by ordinary reborrowing inside a child-building
//! closure, so exiting the closure restores the parent context by construction.

use poodle_adapter::ThemeProvider;
use poodle_specs::{ControlDensity, ControlSize, SemanticControlSizeRole, UiPresentationProviderSpec};

use crate::presentation::resolve_semantic_size;

/// One explicit construction context: a borrowed token-only theme plus the
/// effective size-scale and density defaults for the current scope.
///
/// Cheap to reborrow and derive: `scoped` copies the two `Copy` defaults and
/// reborrows the same theme, so a provider never mutates its parent.
pub struct RenderContext<'a> {
    theme: &'a dyn ThemeProvider,
    size_scale: ControlSize,
    density: ControlDensity,
}

impl<'a> RenderContext<'a> {
    /// The root context: size scale `md`, density `default`.
    pub fn new(theme: &'a dyn ThemeProvider) -> Self {
        Self {
            theme,
            size_scale: ControlSize::Md,
            density: ControlDensity::Default,
        }
    }

    /// A nested scope replacing both presentation defaults. The parent is
    /// untouched; dropping the derived context restores it by borrowing.
    pub fn scoped(&self, size_scale: ControlSize, density: ControlDensity) -> RenderContext<'_> {
        RenderContext {
            theme: self.theme,
            size_scale,
            density,
        }
    }

    /// The token-only theme. Internal theme-only helpers may take this
    /// explicitly; public component renderers take the context itself.
    pub fn theme(&self) -> &'a dyn ThemeProvider {
        self.theme
    }

    /// The effective size-scale default for this scope.
    pub fn size_scale(&self) -> ControlSize {
        self.size_scale
    }

    /// The effective density default for this scope.
    pub fn density(&self) -> ControlDensity {
        self.density
    }

    /// Base size before semantic-role mapping: the explicit value when
    /// present, otherwise this scope's size scale.
    pub fn base_size(&self, explicit: Option<ControlSize>) -> ControlSize {
        explicit.unwrap_or(self.size_scale)
    }

    /// A component's semantic size: explicit-or-inherited base size first,
    /// then the component's size role. An explicit `md` under an `xl` scope
    /// stays `md`-based.
    pub fn resolve_size(
        &self,
        explicit: Option<ControlSize>,
        role: SemanticControlSizeRole,
    ) -> ControlSize {
        resolve_semantic_size(self.base_size(explicit), role)
    }

    /// A component's density: the explicit value when present, otherwise this
    /// scope's density. An explicit `default` under a `comfortable` scope
    /// stays `default`.
    pub fn resolve_density(&self, explicit: Option<ControlDensity>) -> ControlDensity {
        explicit.unwrap_or(self.density)
    }
}

/// The `UiPresentationProvider` construction boundary.
///
/// Creates a nested context from the provider's two values, invokes the
/// immediate child builder with it, and returns the resulting child unchanged:
/// no wrapper node, layout, paint, accessibility entry, focus target, or
/// interaction state can exist here because none is constructed.
pub fn ui_presentation_provider<R>(
    spec: &UiPresentationProviderSpec,
    ctx: &RenderContext<'_>,
    build_child: impl FnOnce(&RenderContext<'_>) -> R,
) -> R {
    build_child(&ctx.scoped(spec.size_scale, spec.density))
}

#[cfg(test)]
mod tests {
    //! Resolver laws for architecture 010: root defaults, outer scope, nested
    //! scope, sibling restoration, and explicit `md` / `default` resets. These
    //! laws gate the roster migration — they exist before any renderer moves
    //! to the context API.

    use super::*;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    /// Every law runs against a caller-owned theme so lifetimes stay local.
    fn with_root<R>(f: impl FnOnce(&RenderContext<'_>) -> R) -> R {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        f(&ctx)
    }

    #[test]
    fn root_defaults_are_md_and_default() {
        with_root(|ctx| {
            assert_eq!(ctx.size_scale(), ControlSize::Md);
            assert_eq!(ctx.density(), ControlDensity::Default);
            assert_eq!(ctx.base_size(None), ControlSize::Md);
            assert_eq!(ctx.resolve_density(None), ControlDensity::Default);
        });
    }

    #[test]
    fn outer_scope_replaces_both_defaults() {
        with_root(|ctx| {
            let outer = ctx.scoped(ControlSize::Xl, ControlDensity::Comfortable);
            assert_eq!(outer.base_size(None), ControlSize::Xl);
            assert_eq!(outer.resolve_density(None), ControlDensity::Comfortable);
        });
    }

    #[test]
    fn nested_scope_replaces_the_outer_defaults() {
        with_root(|ctx| {
            let outer = ctx.scoped(ControlSize::Xl, ControlDensity::Comfortable);
            let inner = outer.scoped(ControlSize::Sm, ControlDensity::Compact);
            assert_eq!(inner.base_size(None), ControlSize::Sm);
            assert_eq!(inner.resolve_density(None), ControlDensity::Compact);
            // The outer scope is untouched by the nested derivation.
            assert_eq!(outer.base_size(None), ControlSize::Xl);
            assert_eq!(outer.resolve_density(None), ControlDensity::Comfortable);
        });
    }

    #[test]
    fn sibling_scopes_restore_the_parent_by_borrowing() {
        with_root(|ctx| {
            {
                let first = ctx.scoped(ControlSize::Xl, ControlDensity::Comfortable);
                assert_eq!(first.base_size(None), ControlSize::Xl);
            }
            {
                let second = ctx.scoped(ControlSize::Xs, ControlDensity::Compact);
                assert_eq!(second.base_size(None), ControlSize::Xs);
            }
            // After both siblings, the parent still resolves root defaults.
            assert_eq!(ctx.base_size(None), ControlSize::Md);
            assert_eq!(ctx.resolve_density(None), ControlDensity::Default);
        });
    }

    #[test]
    fn explicit_md_wins_inside_a_non_default_scope() {
        with_root(|ctx| {
            let outer = ctx.scoped(ControlSize::Xl, ControlDensity::Comfortable);
            assert_eq!(outer.base_size(Some(ControlSize::Md)), ControlSize::Md);
            assert_eq!(
                outer.resolve_density(Some(ControlDensity::Default)),
                ControlDensity::Default
            );
        });
    }

    #[test]
    fn explicit_default_density_wins_inside_a_comfortable_scope() {
        with_root(|ctx| {
            let outer = ctx.scoped(ControlSize::Xl, ControlDensity::Comfortable);
            assert_eq!(
                outer.resolve_size(Some(ControlSize::Md), SemanticControlSizeRole::Control),
                ControlSize::Md
            );
            assert_eq!(
                outer.resolve_density(Some(ControlDensity::Default)),
                ControlDensity::Default
            );
        });
    }

    #[test]
    fn role_mapping_happens_after_base_size_selection() {
        with_root(|ctx| {
            let outer = ctx.scoped(ControlSize::Xl, ControlDensity::Comfortable);
            // Inherited base: xl mapped through Chrome shifts one stop down.
            assert_eq!(
                outer.resolve_size(None, SemanticControlSizeRole::Chrome),
                ControlSize::Lg
            );
            // Explicit base: md mapped through Prominent shifts one stop up,
            // independent of the xl scope.
            assert_eq!(
                outer.resolve_size(Some(ControlSize::Md), SemanticControlSizeRole::Prominent),
                ControlSize::Lg
            );
            // Control role is identity on the chosen base.
            assert_eq!(
                outer.resolve_size(None, SemanticControlSizeRole::Control),
                ControlSize::Xl
            );
        });
    }

    #[test]
    fn provider_builds_child_inside_its_scope_and_returns_it_unchanged() {
        with_root(|ctx| {
            let spec = UiPresentationProviderSpec::new()
                .with_size_scale(ControlSize::Xl)
                .with_density(ControlDensity::Comfortable);
            // The builder observes the scoped context; its product comes back
            // exactly as built.
            let child = ui_presentation_provider(&spec, ctx, |scoped| {
                assert_eq!(scoped.base_size(None), ControlSize::Xl);
                assert_eq!(scoped.resolve_density(None), ControlDensity::Comfortable);
                // Nested provider inside the closure: replaces both defaults.
                let inner_spec = UiPresentationProviderSpec::new()
                    .with_size_scale(ControlSize::Sm)
                    .with_density(ControlDensity::Compact);
                ui_presentation_provider(&inner_spec, scoped, |inner| {
                    (inner.base_size(None), inner.resolve_density(None))
                })
            });
            assert_eq!(child, (ControlSize::Sm, ControlDensity::Compact));
            // Exiting the closures restored the root context.
            assert_eq!(ctx.base_size(None), ControlSize::Md);
            assert_eq!(ctx.resolve_density(None), ControlDensity::Default);
        });
    }

    #[test]
    fn context_exposes_the_borrowed_token_only_theme() {
        with_root(|ctx| {
            // The theme accessor is the token-only provider the root was built
            // from — presentation state never rides on it.
            let accent = ctx.theme().resolve_color("color.accent.base");
            let expected = theme().resolve_color("color.accent.base");
            assert_eq!(accent, expected);
        });
    }
}
