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
use poodle_node::Node;
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

/// A host-content child builder for a composite that establishes an internal
/// presentation scope (architecture 010). The composite invokes the builder
/// immediately with its scoped context, so the host child resolves its omitted
/// size/density against the scope instead of arriving as an already-built
/// `Node` frozen under the caller's scope. This is a bounded construction
/// closure, not a stored scene or component abstraction.
pub type SlotBuilder<'a> = Box<dyn FnOnce(&RenderContext<'_>) -> Node + 'a>;

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

    // ── Provider cascade proofs through real components ─────────────────────
    //
    // Architecture 010 completion evidence: inherited output must be
    // indistinguishable from the explicit equivalent and distinguishable from
    // the root default — proving the cascade did the work, not the host.

    use poodle_specs::{ButtonSpec, FilterToolbarSpec, TextInputSpec};

    fn fixed_height(node: &Node) -> f32 {
        match node.style.descriptor.layout.height {
            poodle_node::LayoutSizing::Fixed(h) => h,
            ref other => panic!("expected fixed height, got {other:?}"),
        }
    }

    fn role<'a>(node: &'a Node, key: &str) -> Option<&'a str> {
        node.roles.get(key).map(String::as_str)
    }

    #[test]
    fn button_and_text_input_inherit_the_provider_scope() {
        with_root(|ctx| {
            let scope = UiPresentationProviderSpec::new()
                .with_size_scale(ControlSize::Xl)
                .with_density(ControlDensity::Comfortable);

            let (inherited_button, inherited_input) =
                ui_presentation_provider(&scope, ctx, |scoped| {
                    (
                        crate::button::button(
                            &ButtonSpec::new().with_label("Save"),
                            scoped,
                            None,
                        ),
                        crate::text_input::text_input(
                            &TextInputSpec::new().with_default_value("Save"),
                            scoped,
                            None,
                        ),
                    )
                });

            let explicit_button = crate::button::button(
                &ButtonSpec::new()
                    .with_label("Save")
                    .with_size(ControlSize::Xl)
                    .with_density(ControlDensity::Comfortable),
                ctx,
                None,
            );
            let explicit_input = crate::text_input::text_input(
                &TextInputSpec::new()
                    .with_default_value("Save")
                    .with_size(ControlSize::Xl)
                    .with_density(ControlDensity::Comfortable),
                ctx,
                None,
            );
            let root_button =
                crate::button::button(&ButtonSpec::new().with_label("Save"), ctx, None);

            // xl control height = 3.25rem = 52px (contract §8 ladder).
            assert_eq!(fixed_height(&inherited_button), 52.0);
            // Inherited output matches the explicit reference exactly...
            assert_eq!(fixed_height(&inherited_button), fixed_height(&explicit_button));
            assert_eq!(
                inherited_button.style.descriptor.layout.spacing,
                explicit_button.style.descriptor.layout.spacing
            );
            assert_eq!(role(&inherited_button, "size"), Some("xl"));
            assert_eq!(role(&inherited_button, "density"), Some("comfortable"));
            assert_eq!(role(&explicit_button, "size"), Some("xl"));
            // ...and differs from the root default, so the scope did the work.
            assert_eq!(fixed_height(&root_button), 36.0);
            assert_ne!(fixed_height(&inherited_button), fixed_height(&root_button));

            // TextInput: inherited equals explicit, and the resolved roles
            // reach the node exactly like the web's data-* projection.
            assert_eq!(
                inherited_input.style.descriptor.layout,
                explicit_input.style.descriptor.layout
            );
            assert_eq!(role(&inherited_input, "size"), Some("xl"));
            assert_eq!(role(&inherited_input, "density"), Some("comfortable"));
        });
    }

    #[test]
    fn explicit_md_and_default_reset_wins_inside_a_non_default_scope() {
        with_root(|ctx| {
            let scope = UiPresentationProviderSpec::new()
                .with_size_scale(ControlSize::Xl)
                .with_density(ControlDensity::Comfortable);
            let reset = ui_presentation_provider(&scope, ctx, |scoped| {
                crate::button::button(
                    &ButtonSpec::new()
                        .with_label("Save")
                        .with_size(ControlSize::Md)
                        .with_density(ControlDensity::Default),
                    scoped,
                    None,
                )
            });
            let root = crate::button::button(&ButtonSpec::new().with_label("Save"), ctx, None);
            // md control height = 2.25rem = 36px, exactly the root default.
            assert_eq!(fixed_height(&reset), 36.0);
            assert_eq!(reset.style.descriptor.layout, root.style.descriptor.layout);
            assert_eq!(role(&reset, "size"), Some("md"));
            assert_eq!(role(&reset, "density"), Some("default"));
        });
    }

    #[test]
    fn nested_provider_replaces_the_outer_scope_for_its_closure_only() {
        with_root(|ctx| {
            let outer = UiPresentationProviderSpec::new()
                .with_size_scale(ControlSize::Xl)
                .with_density(ControlDensity::Comfortable);
            let (inner_child, outer_sibling) = ui_presentation_provider(&outer, ctx, |outer_ctx| {
                let inner = UiPresentationProviderSpec::new()
                    .with_size_scale(ControlSize::Sm)
                    .with_density(ControlDensity::Compact);
                let inner_child = ui_presentation_provider(&inner, outer_ctx, |inner_ctx| {
                    crate::button::button(&ButtonSpec::new().with_label("In"), inner_ctx, None)
                });
                let outer_sibling = crate::button::button(
                    &ButtonSpec::new().with_label("Out"),
                    outer_ctx,
                    None,
                );
                (inner_child, outer_sibling)
            });
            // sm = 1.75rem = 28px inside the nested scope; xl = 52px outside it.
            assert_eq!(fixed_height(&inner_child), 28.0);
            assert_eq!(role(&inner_child, "size"), Some("sm"));
            assert_eq!(role(&inner_child, "density"), Some("compact"));
            assert_eq!(fixed_height(&outer_sibling), 52.0);
            assert_eq!(role(&outer_sibling, "size"), Some("xl"));
        });
    }

    #[test]
    fn a_scoped_host_slot_builds_inside_the_composites_scope() {
        with_root(|ctx| {
            // FilterToolbar's web pair wraps host content in a provider
            // publishing the toolbar's raw base size and resolved density.
            // Under an outer xl/comfortable provider, an omitted-size host
            // button in the toolbar's controls grid must inherit xl/comfortable.
            let outer = UiPresentationProviderSpec::new()
                .with_size_scale(ControlSize::Xl)
                .with_density(ControlDensity::Comfortable);
            let toolbar = ui_presentation_provider(&outer, ctx, |scoped| {
                crate::filter_toolbar::filter_toolbar(
                    &FilterToolbarSpec::new().with_collapsed(false),
                    scoped,
                    vec![Box::new(|slot_ctx| {
                        crate::button::button(
                            &ButtonSpec::new().with_label("Host filter"),
                            slot_ctx,
                            None,
                        )
                    })],
                    None,
                    None,
                    None,
                )
            });
            let host_button = toolbar
                .find(&|n| matches!(n.a11y.role, Some(poodle_node::NodeRole::Button)))
                .expect("host button inside the toolbar");
            assert_eq!(fixed_height(host_button), 52.0);
            assert_eq!(role(host_button, "size"), Some("xl"));
            assert_eq!(role(host_button, "density"), Some("comfortable"));
        });
    }

    #[test]
    fn the_provider_adds_no_wrapper_node_layout_or_accessibility_entry() {
        with_root(|ctx| {
            let scope = UiPresentationProviderSpec::new()
                .with_size_scale(ControlSize::Lg)
                .with_density(ControlDensity::Compact);
            // The provider returns exactly what its child builder produced:
            // the button node itself, not a container around it.
            let provided = ui_presentation_provider(&scope, ctx, |scoped| {
                crate::button::button(&ButtonSpec::new().with_label("Save"), scoped, None)
            });
            assert!(matches!(provided.kind, poodle_node::NodeKind::Button { .. }));
            assert_eq!(provided.a11y.role, Some(poodle_node::NodeRole::Button));
            assert_eq!(provided.a11y.tab_index, Some(0));
            assert!(provided.interaction.focusable);
            // The a11y projection carries no grouping a wrapper would add:
            // the provided node's accessible name is the button's own.
            assert_eq!(provided.a11y.label, None);
            // Geometry comes from the scope (lg = 2.75rem = 44px)...
            assert_eq!(fixed_height(&provided), 44.0);
            // ...and no provider shell sits between: the child's own subtree
            // is all there is (buttons without icons have no children).
            assert!(provided.children.is_empty());
        });
    }
}
