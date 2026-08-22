# g15.043 — UiPresentationProvider Native Cascade

Status: **planned — operator approval required for the recommended breaking
migration before dispatch**
Found by: `g15.009`, retained by `g15.026`
Depends on: orchestrator-owned native presentation-context decision
Unblocks: `g15.013`
Governing refs: `../../architecture/001-poodle-system-shape.md`,
`../../contracts/001-working-rules.md`,
`../../contracts/components/ui-presentation-provider.md`,
`release-gap-register.md`

## Problem

Svelte and React scope density and semantic control size over a descendant
subtree. Native has the same public spec and resolver math, but the GPUI
provider is a preview-local passthrough: descendants have already resolved
their concrete values before a `Node` tree exists, so no value cascades.

Calling the passthrough complete would violate the active-cohort rule. Applying
post-hoc scale factors to painted nodes would be equally false: density and
semantic size affect component-specific layout decisions, not one universal
transform.

## Required Decision Before This Card Can Become Ready

The orchestrator must choose and document one native construction seam after a
bounded impact audit. Candidate families to compare are:

1. an explicit renderer context carrying theme plus inherited presentation
   through component construction;
2. a scoped resolver supplied while descendant specs are constructed;
3. another native-idiomatic context mechanism that still leaves shared
   composition in `poodle-render`.

The decision must quantify the migration surface and reject:

- backend-only style mutation after component recipes have resolved;
- a second GPUI component implementation;
- thread-local/global ambient state;
- a universal component or scene representation;
- a compatibility twin for the current no-op provider.

## Bounded Impact Audit — 2026-08-23

The missing seam is not confined to the provider wrapper:

- 125 component spec files expose semantic `ControlSize` or `ControlDensity`;
- 107 expose concrete `ControlSize` and 117 expose concrete
  `ControlDensity`;
- only seven spec files currently retain optional presentation inputs;
- 103 shared-render modules read `spec.size` or `spec.density`;
- 168 shared-render modules accept `ThemeProvider` directly;
- 113 shared-render modules construct at least one descendant spec.

The concrete fields have already erased whether `md` / `default` was authored
or merely defaulted. A context alone therefore cannot preserve explicit-child
precedence. Comparing a value with the default is also invalid: an explicit
`md` child under an `xl` provider must remain `md`.

### Candidate Verdicts

1. **Renderer context plus optional inputs — recommended.** Introduce one
   shared Rust `RenderContext` carrying the token provider and effective
   presentation defaults. Migrate semantic component `size` / `density`
   inputs to `Option`, resolve `None` from the context, and pass the same
   context through composite construction. A scoped child context implements
   the provider. This is explicit, nested, order-independent, and leaves the
   `Node` and both native backends presentation-agnostic.
2. **Spec-construction seeding — rejected.** Seeding today's concrete fields
   before builder calls avoids the renderer API migration, but correctness
   depends on call order. A spec configured before seeding silently loses an
   explicit `md` / `default` override. That is not a dependable public
   contract.
3. **Presentation carried by `ThemeProvider` — rejected.** It reduces signature
   churn but conflates theme identity with subtree presentation state and
   turns a token provider into ambient mutable render state.
4. **Post-build provider / backend context — rejected.** Descendant recipes
   have already resolved before either sees the tree.

### Recommended Break

The clean migration changes the shared Rust renderer API before v1.0:

- `poodle-render` component functions receive `&RenderContext`, not a bare
  `&dyn ThemeProvider`;
- `RenderContext` owns a borrowed theme provider plus copyable effective
  `UiPresentation` values and creates nested scoped contexts without global or
  thread-local state;
- native semantic `size` and `density` inputs preserve omission with `Option`;
- renderer helpers resolve `explicit.or(context default)` before semantic-role
  mapping;
- composites pass the same context into every descendant renderer and resolve
  their own optional inputs once;
- `UiPresentationProvider` constructs its child through a closure receiving
  the scoped context, then returns that child unchanged;
- GPUI remains a `Node` interpreter and receives no provider-specific recipe.

No old-signature wrapper, alias, default-value heuristic, or second provider
path may remain. Because this is repository-wide public Rust API churn, the
operator must approve it before this card is recompiled into writable worker
batches.

## Intended Outcome

Once the decision is fixed, implement a clean pre-v0.2 native cascade so an
outer provider changes real descendant size/density defaults, an inner provider
overrides only its subtree, and explicit component props still win. Shared Rust
composition owns the semantics; GPUI only interprets them.

## Acceptance Envelope

- [ ] One architecture note fixes construction ownership, nesting, explicit-
      override precedence, and migration cost before production edits begin.
- [ ] Provider scope is layout- and accessibility-neutral.
- [ ] Default, outer scope, nested override, and explicit child override are
      proved through real shared-render output and mounted GPUI evidence.
- [ ] No component-specific duplicate cascade logic appears in the backend.
- [ ] The contract, GPUI guide, specimen, release-gap register, and migration
      notes agree.

## Stop Conditions

- The proposed seam requires a repository-wide render API break whose cost and
  migration have not been reviewed by the operator.
- A candidate can only simulate cascade in the specimen instead of changing
  real descendant construction.
- The work starts inventing a cross-language provider or scene authority.
- The breaking `RenderContext` and optional-input migration has not been
  approved by the operator.

## Continuation

Keep this card non-dispatchable until the operator accepts or rejects the
recommended break. On acceptance, the orchestrator will write the architecture
note and compile exact substrate, roster-migration, provider/evidence, and
validation batches. On rejection, the native provider remains an explicit
release blocker; do not substitute a specimen-only cascade.
