# g15.041 — Popover Interactive Trigger Semantics

Status: **complete** — PR #59 accepted at `3b4f1571`; merge `e19aea4b`
Found by: `032-review-composition-navigation-overlays.md`
Unblocked: `g15.032` closeout and `g15.033`; `g15.046` remains behind the
completed human-centred audit
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/popover.md`,
`../../contracts/components/button.md`,
`../../specs/022-packaging-versioning-and-release-channel-rules.md`,
`specimen-catalogue-audit.md`

## Problem

Popover's web adapters expose two trigger shapes and neither satisfies the
contract when the caller supplies a real Button or IconButton.

- The default wrapper adds `role="button"`, `tabindex`, keyboard handling, and
  disclosure ARIA around the caller's interactive control. That creates nested
  interactive semantics and restores focus to the wrapper rather than the
  operable control.
- `triggerIsInteractive` correctly makes the wrapper roleless and untabbable,
  but also removes `aria-expanded` and `aria-controls`. The caller cannot apply
  the relationship because Popover does not expose its generated surface id.

PR #58 therefore left Popover at C/C/A `contract/runtime-blocker`. This is a
public interface defect, not a specimen-only problem.

## Goal

Make interactive Popover composition semantic in server output and hydrated
DOM: one real trigger owns keyboard behavior, disabled state, disclosure state,
and the surface relationship, while the Popover wrapper remains a layout/event
host. Keep the ordinary non-interactive wrapper path unchanged.

## Fixed API Decision

The operator approved a clean pre-v0.2 break. Do not retain the old
`triggerIsInteractive` + inert node/snippet shape through an overload, alias,
deprecated twin, runtime fallback, or DOM mutation.

Core owns one framework-neutral state payload:

```ts
export type PopoverTriggerState = Readonly<{
  expanded: boolean;
  controls: string | null;
  disabled: boolean;
}>;
```

`popoverParts(...)` returns that payload beside its existing part attributes.
Its exact semantics are:

- `expanded` is `false` while closed and `true` while open;
- `controls` is `null` while closed and the rendered surface id while open;
- `disabled` mirrors the effective Popover disabled state.

Both framework packages re-export the core-authored type from their public
roots. Their trigger props become discriminated shapes:

- Svelte interactive mode requires
  `triggerIsInteractive: true` and
  `trigger: Snippet<[state: PopoverTriggerState]>`;
- React interactive mode requires
  `triggerIsInteractive: true` and
  `trigger: (state: PopoverTriggerState) => ReactNode`;
- default mode keeps `triggerIsInteractive?: false` and the existing zero-arg
  Svelte snippet / React node.

In interactive mode the caller must apply the state to the actual interactive
control. The wrapper keeps its layout/data hooks and observes bubbled click,
but owns no role, tab stop, ARIA relationship, disabled semantics, or keyboard
handler. Focus restoration still targets the real interactive descendant.

Button gains `controls: string | null` beside `ariaExpanded`, projected as
`aria-controls`. The portable `ButtonSpec` gains `controls: Option<String>` and
`with_controls`; shared rendering projects it to `NodeA11y.controls`. This is
the same existing semantic seam already used by IconButton.

## Scope

### 1. Contract and shared state

- Update Popover's props/snippet, anatomy, part-output, accessibility, focus,
  and migration wording before implementation.
- Add `PopoverTriggerState` and the state payload to the existing core Popover
  parts result. Keep it pure and allocation-insensitive beyond the one parts
  object the adapters already request.
- Update Button's public props, portable-spec table, and accessibility contract
  for `controls`.
- Use framework-native SSR-safe instance identity where needed so the trigger's
  `controls` value and the rendered surface id agree in server output. Do not
  turn this into a repository-wide id-system migration.

### 2. Paired web implementation

- Implement the discriminated trigger surface in Svelte and React.
- In interactive mode, render the trigger with the core-authored state payload.
  Do not clone children, walk the DOM to attach ARIA, mutate attributes after
  mount, or make correct semantics depend on an effect.
- Keep default wrapper behavior, controlled/uncontrolled state, anchored
  positioning, dismissal, initial focus, and repeatability unchanged.
- Keep the existing real-descendant focus restoration behavior for interactive
  mode. DOM lookup is permitted for focus restoration, not ARIA projection.
- Add `controls` to Button in both web packages and render the native
  `aria-controls` attribute when non-null.

### 3. Active-cohort Button parity

- Add `controls` to `ButtonSpec`, its defaults/builders, shared render
  composition, and focused Rust evidence.
- Prove the renderer-neutral Button node carries `a11y.controls`. Reuse the
  general node field and current GPUI structural path; do not add a
  Button-specific backend channel or claim that GPUI 0.2.2 exposes platform
  accessibility attributes it does not support.

### 4. Migrate every current caller

Migrate every in-repository `triggerIsInteractive` caller, including retained
tests and fixtures. The current production set is exact at planning time:

- Svelte: `HistoryCenter`, `MessageCenter`, `UpdateCenter`;
- React: `HistoryCenter`, `MessageCenter`, `UpdateCenter`.

Each actual Button, IconButton, or native button receives `expanded`,
`controls`, and `disabled` from the trigger payload. Preserve badges, progress
rings, and other non-interactive decoration inside the trigger host.

Run a fresh repository search before closeout so no zero-arg interactive
trigger remains. Also run a read-only targeted search under `~/Dev/projects`
for downstream `triggerIsInteractive` use, record affected repositories and
their migration in the batch log, and do not edit external repositories.

### 5. Evidence, specimen, and audit closeout

- Restore the paired Popover specimens with real Poodle Button triggers using
  the state payload. Keep Examples curated; do not add a prop matrix.
- Add focused paired evidence for default and interactive modes, SSR output,
  client behavior, disabled state, dismissal, and focus restoration.
- Add focused Button evidence in both web runtimes and the shared Rust path.
- Update generated package/API material only through its canonical generator.
- Add an Unreleased changelog migration note and one August g15.041 batch log
  satisfying spec 022's package, public-entry-point, change-class, migration,
  and downstream re-check fields.
- After implementation evidence and operator live review pass, return Popover
  to A/A/A `keep`, remove the sole blocker, and mechanically recount the audit:
  Svelte A 88→89 / C 45→44; React A 101→102 / C 48→47; worst A 65→66 /
  C 53→52; `keep` 55→56; `contract/runtime-blocker` 1→0.

## Public Release Surface

- `@inflatable-cookie/poodle-core`: additive public
  `PopoverTriggerState`/parts-result state shape.
- `@inflatable-cookie/poodle-svelte`: breaking interactive-trigger snippet
  signature; additive root type export and Button `controls` prop.
- `@inflatable-cookie/poodle-react`: breaking interactive-trigger render
  signature; additive root type export and Button `controls` prop.
- `poodle-specs`: breaking source change for direct `ButtonSpec` struct
  literals, additive builder for ordinary construction.
- `poodle-render`: behavioral accessibility projection; no new component API.
- `poodle-gpui`: structural consumption/evidence only; no platform AT claim.

Downstream migration: change every interactive trigger from a static node or
zero-argument snippet to a state-aware render, and pass its three fields to the
real control. Direct Rust `ButtonSpec` literals must initialize `controls`;
builder callers remain source-compatible unless they opt into the relationship.

## Acceptance

- [ ] Popover's default trigger mode retains its wrapper button semantics,
      Enter/Space behavior, disabled suppression, disclosure ARIA, and focus
      restoration.
- [ ] Interactive mode requires the new state-aware trigger shape at compile
      time. No compatibility route accepts the old static shape.
- [ ] The interactive wrapper is roleless and untabbable. Exactly one actual
      trigger is operable and owns `aria-expanded="false"` while closed,
      `aria-expanded="true"` plus matching `aria-controls` while open, and the
      effective disabled state.
- [ ] Paired server-render evidence contains correct closed/open trigger
      attributes and matching open surface identity without post-mount repair.
- [ ] Client evidence proves click and keyboard opening, Escape/outside close,
      focus restoration to the real control, controlled/uncontrolled use, and
      repeated open/close operation.
- [ ] HistoryCenter, MessageCenter, and UpdateCenter preserve their visible
      trigger composition and expose the relationship on the actual control in
      both web runtimes.
- [ ] Button `controls` reaches web DOM and renderer-neutral native a11y output;
      omission leaves the attribute/field absent.
- [ ] Packed root consumers can import `PopoverTriggerState`, compose the new
      trigger, and use Button `controls` from both web packages.
- [ ] The paired Popover specimen routes are concise, work live, and receive
      operator sign-off before merge.
- [ ] The audit row and totals return to the exact values above only after all
      semantic and live-review evidence passes.
- [ ] The changelog and batch log name every changed public-intent package,
      classify the break, give the migration, and record the downstream search.

## Stop Conditions

- Svelte cannot express and check the discriminated snippet contract without a
  broader public composition abstraction.
- Correct server output requires a repository-wide identity or hydration
  redesign rather than a Popover-local framework-native id fix.
- Button `controls` requires a new node/backend accessibility architecture
  instead of the existing `NodeA11y.controls` seam.
- A current in-repository or downstream caller needs behavior that cannot be
  expressed by the fixed three-field payload.
- Focus restoration cannot target the actual control without adding a new
  public ref/element contract.
- Validation exposes a family-wide overlay defect outside Popover's current
  anchored/dismiss/focus machinery.

## Writable Scope

- Popover contract, core state/parts, paired web adapters, public type exports,
  focused tests, and paired specimens
- Button contract, paired web adapters, `ButtonSpec`, shared render projection,
  and focused active-cohort evidence
- the six named production composites plus existing Popover/Button test and
  package-install fixtures required by the migration
- generated docs/artifacts only through their canonical generator
- `CHANGELOG.md`, `specimen-catalogue-audit.md`, the g15.032 execution log, one
  August g15.041 batch log, and `PAPERCUTS.md`

Out of scope: another overlay family, a generic `asChild`/slot-cloning system,
new styling or tokens, exhaustive specimens, GPUI platform accessibility,
Jetstream runtime work, visual conformance, `g15.033`, release mutation, and
external-repository edits.

## Validation

- focused paired Popover, Button, HistoryCenter, MessageCenter, and
  UpdateCenter component tests, including SSR evidence
- focused `poodle-core`, `poodle-specs`, and `poodle-render` tests
- `effigy test:core`
- `effigy test:components`
- `effigy check:svelte`
- `effigy react:build`
- `effigy ci:rust`
- `effigy check:gpui`
- `effigy regressions:native`
- `effigy test:web-pack-install`
- `effigy catalogue:check`
- `effigy docs:check`
- `effigy qa`
- `git diff --check origin/main...HEAD`

Headless only. Never run `*-windowed`, `test:native-visual`, Jetstream, or
release selectors.

## Continuation

PR #59 landed the clean state-aware trigger migration and closed the routed
Popover blocker. The operator explicitly authorised final fixes and merge
without a renewed live-route pass; no fresh visual evidence is claimed.
`g15.032` closes with this repair and `g15.033` is ready.
