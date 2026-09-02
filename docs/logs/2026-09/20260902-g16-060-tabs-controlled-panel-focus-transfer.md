# g16.060 — Tabs Controlled-panel Focus Transfer

Status: in review — awaiting orchestrator merge
Date: 2026-09-02
Card: `docs/roadmaps/g16/060-tabs-controlled-panel-focus-transfer.md`
Handoff: `docs/handoffs/20260902-160500-g16-060-tabs-controlled-focus.md`
Governing refs: `docs/contracts/001-working-rules.md`,
`docs/contracts/components/tabs.md`
Branch: `fix/g16-060-tabs-controlled-focus`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-060-tabs-controlled-focus`
Rebased onto `origin/main` at `38f4fa2a7` (g16.061 routing repair merged as PR
#166 resolved the ordinary-PR certification block; pre-rebase receipts below
describe the earlier open-defect state against `d82ba7202`). Rebase kept the
lane's accepted focus semantics; the lane carries no certification-lane
changes (routing files are byte-identical to `origin/main`).

## Outcome

Web Tabs now owns one controlled-value focus policy:
`focusOnValueChange="preserve" | "selected-tab"`, default `"preserve"`.
When the host changes the controlled value and `document.activeElement` was
inside the outgoing selected panel, `"selected-tab"` focuses the newly
selected enabled tab after render. Preserve, outside focus, already-on-tab,
missing/disabled destinations, teardown, and uncontrolled mode stay inert.
Superseded A→B then B→C applies only C once — whether the changes are batched
or committed separately before the first timer fires.

No panel initial-focus callback, IconButton autofocus, exported `focus()`
handle, consumer selector, body/panel fallback, Figmatic edit, or
Rust/GPUI/Jetstream change.

## What landed

- Core: `TabsFocusOnValueChange`, `nextTabsControlledFocusDestination`,
  `resolveTabsControlledFocusDestination`.
- Svelte: capture through owned `focusin`/`focusout` lifecycle state, one
  cancellable `setTimeout(0)` apply through `tabElements`, with live policy,
  value, destination, and teardown checks.
- React: capture through owned focus lifecycle state, record the committed
  controlled value in `useLayoutEffect`, and apply one cancellable
  `setTimeout(0)` through `tabRefs` with live policy, value, destination,
  generation, and teardown checks. No focus-policy DOM read or state/ref
  mutation occurs during render. The pending request is the latch: separate
  commits retarget it to the latest value (the second exact-head review found
  the pre-emptive inequality invalidation lost separate-commit A→B→C
  transfers; it is removed).
- Contract §6 Focus And Announcement and §9 Svelte Notes now describe owned
  `focusin`/`focusout` capture with commit-phase application; the unused React
  `panelRef` and Svelte `panelElement` bindings are removed.
- Public types/exports on both shells. Contract row, Focus section, Svelte
  notes, Known Deltas, catalogue prop, `WEB_ONLY_BY_SLUG.tabs`.
- Paired lifecycle tests in dedicated files, including stale disable and policy
  change after scheduling. Not in `Tabs.test.ts` / `Tabs.test.tsx`, which stub
  `requestAnimationFrame` as sync.

## Falsification

Green consumer-shaped proofs first. Plant skipped destination `.focus()` in
both apply paths, then restored.

| Row | Plant | Result |
| --- | --- | --- |
| Async Components → Tree (Svelte) | skip `tabElements[resolved]?.focus()` | `activeElement` was `body`, expected Tree tab |
| Async Components → Tree (React) | skip `tabRefs.current[resolved]?.focus()` | `activeElement` was `tree-return`, expected Tree tab |

Restored sources reran green. Core helper plant against `packages/core/src`
did not bite the component tests: Vitest resolves `@inflatable-cookie/poodle-core`
from `dist`, not source.

## Validation

Focused:

- `bun run --cwd packages/core test test/tabs.test.ts` — 23 pass, 0 fail
- `bunx vitest run --project react-components` on
  `TabsControlledFocus.test.tsx` — 13 pass, 0 fail
- `bunx vitest run --project svelte-components` on
  `TabsControlledFocus.svelte.test.ts` — 13 pass, 0 fail
- Adversarial rows: destination disabled after scheduling, and policy changed
  to `"preserve"` after scheduling. Against the committed pre-repair React
  source (`f0909735a`) both rows fail — the stale destination receives
  `.focus()` once; the repaired source reruns green. Pre-repair Svelte already
  revalidated through live state reads, so its rows lock that behaviour.
- Separate-commit supersession rows (second exact-head review): `flushSync`
  A→B then a microtask B→C before the first timer (Svelte: tick-separated
  assignments). Pre-fix React head failed — Tree received zero focus calls
  because the latch was invalidated before retarget. After removing the
  pre-emptive inequality invalidation: React 13 pass, Svelte 13 pass, B never
  focused, C focused exactly once. Svelte retargeted correctly pre-fix through
  its live `$effect.pre` state.
- Full `react-components` + `svelte-components` projects after the repair —
  346 files, 2697 tests, pass.

Public declarations/exports (built by `react:package` / `svelte:package`):
`dist/Tabs.d.ts` and `dist/Tabs.svelte.d.ts` carry
`focusOnValueChange?: TabsFocusOnValueChange`; both `dist/types.d.ts` declare
`export type TabsFocusOnValueChange = "preserve" | "selected-tab"`; React root
re-exports through `export * from "./types"`, Svelte root through its public
type export list.

Required boards after the rebase onto `38f4fa2a7`:

- `effigy ci:web` — pass, all 22 steps green on the rebased head. The routing
  red is gone: g16.061 (PR #166) resolved the ordinary-PR certification block
  on `origin/main`, and this lane carries no certification-lane changes
  (`test/package-install/*`, `PAPERCUTS.md` byte-identical to `origin/main`).
- `effigy docs:check` — pass
- `git diff --check origin/main...HEAD` — pass

Vitest `react-components` retains the required source alias for the focused
source-helper proof; the `react-preview` project matches `origin/main`.

## Figmatic

Do not edit Figmatic from this lane. After merge, Figmatic PR #69 should
source-link the exact accepted Poodle SHA.

- Prop: `focusOnValueChange?: "preserve" | "selected-tab"` (default `"preserve"`)
- Svelte: `import { Tabs, type TabsFocusOnValueChange } from "@inflatable-cookie/poodle-svelte"`
- React: `import { Tabs, type TabsProps, type TabsFocusOnValueChange } from "@inflatable-cookie/poodle-react"`
- Local check against this checkout: `effigy deps link bun <poodle-path>` then
  `bun install`. Guide: Effigy `docs/guides/077-local-dependency-linking.md`.
- Keep the mounted keyboard Open counterexample. Accepted Open should land
  focus on the Tree **tab** (Poodle), after which Figmatic can move to
  “Return to screen”.

## Unresolved

- Orchestrator owns review, merge, closeout, and the receipt back to Figmatic.
- Native/GPUI/Jetstream still have no equivalent; the contract records that
  web-only delta.
