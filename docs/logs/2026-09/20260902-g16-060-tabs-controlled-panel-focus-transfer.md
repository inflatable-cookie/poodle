# g16.060 — Tabs Controlled-panel Focus Transfer

Status: in review — awaiting orchestrator merge
Date: 2026-09-02
Card: `docs/roadmaps/g16/060-tabs-controlled-panel-focus-transfer.md`
Handoff: `docs/handoffs/20260902-160500-g16-060-tabs-controlled-focus.md`
Governing refs: `docs/contracts/001-working-rules.md`,
`docs/contracts/components/tabs.md`
Branch: `fix/g16-060-tabs-controlled-focus`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-060-tabs-controlled-focus`
PR head before exact-head repair: `f0909735a25f7b9248971cc4e3448e20ced5f114`
Live `origin/main` at worker close: `d82ba7202aaecff452ec2c59e79ea7be3be114af`
Repaired head: `e96826f91` (code), receipt commit on top

## Outcome

Web Tabs now owns one controlled-value focus policy:
`focusOnValueChange="preserve" | "selected-tab"`, default `"preserve"`.
When the host changes the controlled value and `document.activeElement` was
inside the outgoing selected panel, `"selected-tab"` focuses the newly
selected enabled tab after render. Preserve, outside focus, already-on-tab,
missing/disabled destinations, teardown, and uncontrolled mode stay inert.
Superseded A→B then B→C applies only C once.

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
  mutation occurs during render.
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
  `TabsControlledFocus.test.tsx` — 12 pass, 0 fail
- `bunx vitest run --project svelte-components` on
  `TabsControlledFocus.svelte.test.ts` — 12 pass, 0 fail
- Adversarial rows: destination disabled after scheduling, and policy changed
  to `"preserve"` after scheduling. Against the committed pre-repair React
  source (`f0909735a`) both rows fail — the stale destination receives
  `.focus()` once; the repaired source reruns green. Pre-repair Svelte already
  revalidated through live state reads, so its rows lock that behaviour.

Public declarations/exports (built by `react:package` / `svelte:package`):
`dist/Tabs.d.ts` and `dist/Tabs.svelte.d.ts` carry
`focusOnValueChange?: TabsFocusOnValueChange`; both `dist/types.d.ts` declare
`export type TabsFocusOnValueChange = "preserve" | "selected-tab"`; React root
re-exports through `export * from "./types"`, Svelte root through its public
type export list.

Required boards on the repaired worker tree:

- `effigy ci:web` — red only at `test:web-pack-install`, the inherited g16.059
  ordinary-PR certification-routing defect: "certification scope rejected
  paths outside writable allowlist" naming this lane's Tabs, core, and docs
  paths. Steps 1-11 were green on the repaired head `e96826f91`; steps 13-22
  were green on identical repaired content in the pre-commit board run. The
  certification file is unchanged from `origin/main`; the defect stays open
  and was not weakened here.
- `effigy docs:check` — pass
- `git diff --check origin/main...HEAD` — pass

Vitest `react-components` retains the required source alias for the focused
source-helper proof. The `react-preview` alias and the attempted certification
routing change are reverted.

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
