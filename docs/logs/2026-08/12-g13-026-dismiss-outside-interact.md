---
title: g13 batch 026 — dismissOnOutsideInteract across the overlay family
status: complete
milestone: side-quest (component API, outside the g13 IR lane)
owner: Poodle core
updated: 2026-08-12
tags: [log, g13, dismissal, overlay, dismissable-layer, side-quest]
---

## What this batch did

Executed batch card
`docs/roadmaps/g13/batch-cards/026-dismiss-on-outside-interact-prop.md` on branch
`thread/g13-026-dismiss-outside-interact`: gave all fourteen overlays that
register a dismissable layer a `dismissOnOutsideInteract?: boolean` prop,
plumbed to the layer registration, with tests proving `false` suppresses
outside dismissal and contracts documenting the prop and its machine guard.
Outside dismissal is now a consumer decision everywhere it is a decision —
before this card only `Popover` exposed it.

Rulings applied as written: default is `true` for the twelve non-modal
components (their layers hardcoded `true`); **Dialog and Drawer default to
`false`** because they already register `false` today; escape handling,
`resolveDismiss`, `dismiss.ts`, and all `HistoryCenter.*` files untouched.

## The one place the evidence table was wrong

The card's Evidence table lists `Dialog` and `Drawer` under "Hardcodes `true`
in the layer". Their current registrations register `dismissOnOutsideInteract:
false` — and have since `7b319635` (2026-07-10, "move dialog family onto
shared modal machine"), which predates the card's measurement commit
(`761f81d8`). The Fixed By Ruling anticipated exactly this ("if a component
already registers `false`, preserve that as its default and say so in the
contract"), so no re-decision was needed: defaults stayed `false`, and both
contracts say so.

## The Dialog/Drawer layer registration change

Plumbing the prop for Dialog/Drawer required one change beyond passing it: the
layer's `contains` moved from `() => true` to the surface. With `contains:
() => true` the outside axis of the dismissable layer can never fire —
`resolveDismiss` only dismisses layers whose `contains(target)` is false — so
a truthy prop would have been the exact "type-checks and does nothing" defect
the card exists to prevent. With the default `false` the `contains` callback is
never consulted (short-circuit in `resolveDismiss`), so today's behaviour is
bit-for-bit preserved; with `true` a document-level mousedown outside the
surface dismisses through the layer's existing `ESCAPE` path, still guarded by
`dismissOnEscape`. The backdrop button remains the modal's own dismissal path,
guarded by `dismissOnBackdrop`. Both runtimes changed identically
(`packages/svelte/components/src/Dialog.svelte`, `Drawer.svelte`,
`packages/react/components/src/Dialog.tsx`, `Drawer.tsx`).

## Rust specs: where dismissal is modelled

Only `DialogSpec` and `DrawerSpec` model dismissal at all
(`dismiss_on_escape`, `dismiss_on_backdrop`). Both gained
`dismiss_on_outside_interact: bool` (default `false`) plus the matching
`with_dismiss_on_outside_interact` builder — native parity for the two modals.

The other twelve specs (`select`, `menu`, `context-menu`, `menubar`,
`navigation-menu`, `split-button`, `theme-select`, `ref-select`,
`model-picker`, `order-by`, `list-card`, `filter-builder`) do not model
dismissal, and per the card's instruction the field was **not invented** there:
a flag whose default matches every native platform's standard
outside-dismissal for these overlays would be invented data. Noted here, and
the gate is handled via `OPEN_GAPS` in
`packages/svelte/preview/scripts/contract-spec-drift.ts` (see Findings).

## Deliverables (only the scoped writes)

- Fourteen Svelte components in `packages/svelte/components/src/` —
  `dismissOnOutsideInteract?: boolean` prop, default `true` (Dialog/Drawer
  `false`), plumbed into `registerDismissLayer`; Dialog/Drawer `contains`
  changed to the surface as described above.
- Fourteen React counterparts in `packages/react/components/src/` — identical
  props, defaults, and layer plumbing, with the prop in the effect dependency
  arrays.
- Tests in both runtimes (`packages/svelte/components/test/`,
  `packages/react/components/test/`) — per component, two tests: outside
  mousedown dismisses by default, and stays open with `dismissOnOutsideInteract
  = false`. New files for the twelve without existing suites; dismissal blocks
  appended to the existing `RefSelect`, `ModelPicker`, and `ListCard` suites in
  both runtimes.
- Fourteen contracts in `docs/contracts/components/` — the prop table row and
  the machine guard statement, matching `popover.md`'s treatment adapted to
  each contract's machine format (prose machines, not tables).
- `packages/contracts/components/src/dialog.rs`, `drawer.rs` — the new field
  and builder.
- `packages/svelte/preview/scripts/contract-spec-drift.ts` — `OPEN_GAPS`
  entries for the twelve non-dismissal-modeling specs (see Findings for why
  this sits outside the writable paths).
- `docs/logs/2026-08/12-g13-026-dismiss-outside-interact.md` — this log.
- `PAPERCUTS.md` — the spec-gate friction (see Findings).

## Findings

**A documented prop that a spec deliberately does not carry fails `effigy
docs:lint`** (stopped-condition adjacent; recorded not worked around beyond the
minimum). The contract ↔ poodle-specs drift gate
(`packages/svelte/preview/scripts/contract-spec-drift.ts`, run inside
`docs:lint`) requires every documented Public Prop to exist on the matching
`*Spec` struct. The card's instruction — "where a spec does not model
dismissal, note that in the log rather than inventing one" — conflicts with
that gate for the twelve specs that model no dismissal: documenting
`dismissOnOutsideInteract` made `docs:lint` fail with twelve
`contract/spec drift` findings. Precedent: g13.009's `initialFocus` hit the
same gate and was resolved by giving `DialogSpec`/`FormDialogSpec` the field
(recorded in `PAPERCUTS.md`, 2026-08-11, "a decision is required before the
next prop of this kind lands"). For this card the resolution is the tool's own
sanctioned debt mechanism: `OPEN_GAPS` entries for the twelve, each with the
reason (specs deliberately model dismissal only where the default differs;
a field that always reads `true` on native would be invented data; native
renderers keep platform outside-dismissal until a native host needs to refuse
it). This file is outside the card's writable paths — same class of sanctioned
edit as b010's `WEB_ONLY_PROPS` entry, which the b014 log explicitly noted
"sits outside its writable paths". Burning the gap down (spec fields or a
sanctioned carve-out, per the g13.009 papercut) is follow-up, recorded in
`PAPERCUTS.md`.

No other stop conditions were reached: no component's dismissal is inexpressible
as a single boolean; no focus-return path or layer-stuck state was introduced
(the layer stays registered on a refused close — `modalTransition` guards the
`ESCAPE` transition, the component never unregisters early); no spec models
dismissal in a way the boolean cannot express.

## The Svelte / React shapes used

Svelte (identical shape to `Popover.svelte`):

```svelte
interface Props {
  /* ... */
  dismissOnOutsideInteract?: boolean;
  /* ... */
}

let {
  /* ... */
  dismissOnOutsideInteract = true, /* Dialog/Drawer: false */
  /* ... */
}: Props = $props();

/* in the open effect */
return registerDismissLayer({
  contains: /* unchanged per component */,
  dismissOnOutsideInteract,
  onDismiss: /* unchanged per component */,
});
```

React mirrors exactly, with the prop added to the effect dependency array
(e.g. `}, [isOpen, dismissOnOutsideInteract]);`). Dialog/Drawer in both
runtimes additionally changed `contains: () => true` to the surface check
(`surfaceElement?.contains(target)` / `surfaceRef.current?.contains(target)`).

## Validation

| Command | Exit state |
|---------|-----------|
| `effigy test:components` | 0 — 46 files / 910 tests (baseline 46 / 898; +28: fourteen components × two dismissal tests) |
| `effigy test:parity` | 0 — 164 tests (unchanged) |
| `effigy docs:lint` | 0 |
| `effigy docs:contract-drift` | 0 — twelve documented props newly checked and implemented in Svelte |
| `effigy docs:spec-drift` | 0 — checked 113; twelve `OPEN_GAPS` exemptions |
| `effigy check:svelte` | 0 — 449 files, 0 errors |
| `bunx tsc --noEmit -p packages/react/preview/tsconfig.json` | 5 pre-existing errors, all in `TextInput.tsx` / `AgentChatInput.tsx` / `AgentQuestionSpecimen.tsx` — none in the fourteen touched components |
| `cargo test --manifest-path packages/contracts/components/Cargo.toml` | 0 |
| `git diff --check` | 0 |
| `git status --porcelain` | only the writable paths plus the sanctioned `contract-spec-drift.ts` edit and this log |

## Not done

Per batch card and worker rules: no `dismiss.ts` change, no `HistoryCenter.*`
touch, no escape-prop, no focus-trapping or `resolveDismiss` change, no change
to any current default (Dialog/Drawer `false` preserved, twelve `true`
preserved), no `contract-prop-drift` reverse-direction enforcement (recorded,
not fixed), no merge, no `git add -A`. The `OPEN_GAPS` burn-down (spec fields
or a sanctioned carve-out decision) is left as recorded follow-up.
