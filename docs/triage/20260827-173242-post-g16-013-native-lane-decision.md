# Post-g16.013 Native Lane Decision

Status: open — operator approval required for a clean Accordion Rust migration
Captured: 2026-08-27
Source: orchestrator evidence checkpoint after PR #87

## Checkpoint

The active-cohort ledger is at 42 mounted / 132 missing. The next useful lane
should close an observable behavior gap in a reusable control, not add mounted
proof to an inert presentation component.

## Recommended Lane — Accordion

Accordion is the strongest bounded candidate because the web authority, shared
machine, Rust contract, renderer, and GPUI wrapper currently disagree at one
selection/disclosure seam.

- Svelte and React use `toggleGroupTransition` and report the resulting open
  selection: `string | string[] | null`.
- `AccordionSpec` carries both `allow_multiple` and `selection_mode`. They
  describe the same public choice; the renderer ignores both and the GPUI
  specimen relies on the legacy field.
- Rust activation reports only the clicked item as `&str`. The GPUI specimen
  reconstructs single/multiple membership itself, contradicting the contract
  and duplicating the shared transition.
- `AccordionSelectionValue::Single(String)` cannot represent the contracted
  collapsed result. The outer `Option` is already needed to distinguish an
  omitted controlled value from a supplied value/default seed.
- `Accordion::with_id(...)` in the GPUI compatibility wrapper is a no-op.
  Triggers have no instance-scoped runtime identity or trigger/panel
  association.
- The root is always a `Group`, including single mode. Triggers are generic
  containers without Button role, expanded state, controls relation, focus
  ring, or disabled focus suppression. Open panels have Region role but no
  labelled-by relation.
- The only mounted Accordion regression proves inset-shadow painting. It does
  not exercise selection, disclosure, focus, disabled behavior, identity, or
  host rebuild.
- The GPUI specimen copy advertises Arrow/Home/End header navigation that the
  detailed contract does not require. The curated specimen should describe
  only the contracted Enter/Space/Tab behavior.

## Proposed Clean Migration

1. Remove `AccordionSpec.allow_multiple` and `with_allow_multiple`; keep
   `selection_mode` as the sole mode field.
2. Reshape the Rust selection value so single mode can carry an explicit empty
   result and multiple mode carries the ordered open set. Do not add aliases,
   legacy constructors, or fallbacks.
3. Add a required, lifetime-stable `AccordionHandlers` instance scope and a
   typed resulting-selection callback.
4. Reuse `poodle_headless::toggle_group_transition` for native selection. The
   renderer reports its result; hosts rebuild the spec and never reconstruct
   membership from an activated item.
5. Project mode-correct root semantics, Button triggers, expanded state,
   controls/labelled-by relations, structured focus rings, disabled focus
   suppression, and stable per-instance trigger/panel ids.
6. Migrate Poodle-owned GPUI and deferred-Jetstream callers directly. Jetstream
   remains compile-only and program-deferred.
7. Add one named mounted GPUI regression through real pointer/keyboard dispatch
   and host rebuild. Prove single selection, collapsible empty, multiple
   add/remove, disabled inertia, semantics, and two-instance identity.
8. Move only Accordion's mounted cell: 42 → 43 mounted and 132 → 131 missing.
   Accessibility and visual evidence do not move.

## Why Not The Other Gates Yet

- NumberInput still needs the separate raw-draft/committed-number decision in
  `20260826-213343-number-input-native-value-model.md`.
- EditableLabel has more coupled lifecycle questions: activation mode, draft
  ownership, commit/cancel payloads, select-on-focus, and focus restoration.
- Rating has a larger whole-step/fractional semantic split and legacy Rust
  value fields; it is not the smaller next card.
- Static components can gain construction or visual evidence later, but they
  do not repair a reusable interaction seam now.

## Approval Needed

Approve or reject the clean Rust migration above. Approval authorizes removal
of the legacy Accordion mode field and activated-item callback without a
compatibility layer. It does not authorize web API changes, Jetstream
admission, animation work, visual comparison, or broad accessibility claims.

After approval, promote this decision into `g16.014`, compile one worker
handoff, push the planning base, and return the repository-relative path to the
operator.
