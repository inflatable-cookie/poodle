# g15.043 — UiPresentationProvider Native Cascade

Status: **planned — architecture decision required before dispatch**
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

## Continuation

Keep this card non-dispatchable until the orchestrator records the native
presentation-context decision and replaces this planning envelope with exact
writable scope, acceptance, validation, and migration instructions.
