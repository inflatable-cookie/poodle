# g16.060 — Tabs Controlled-panel Focus Transfer

Status: in review — awaiting orchestrator merge
Date: 2026-09-02
Card: `docs/roadmaps/g16/060-tabs-controlled-panel-focus-transfer.md`
Handoff: `docs/handoffs/20260902-160500-g16-060-tabs-controlled-focus.md`
Governing refs: `docs/contracts/001-working-rules.md`,
`docs/contracts/components/tabs.md`
Branch: `fix/g16-060-tabs-controlled-focus`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-060-tabs-controlled-focus`
Dispatch HEAD: `cf3d12853f9fa7e09ccc1c3a754fdcfe5e79cbe7`
Live `origin/main` at worker close: `d82ba7202aaecff452ec2c59e79ea7be3be114af`

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
- Svelte: capture in `$effect.pre` against the still-mounted panel, one
  cancellable `setTimeout(0)` apply through `tabElements`.
- React: capture during render against `panelRef`, epoch-gated layout effect
  so later `setFocusIndex` / indicator commits do not cancel the timer, apply
  through `tabRefs`, unmount clears the timer.
- Public types/exports on both shells. Contract row, Focus section, Svelte
  notes, Known Deltas, catalogue prop, `WEB_ONLY_BY_SLUG.tabs`.
- Paired lifecycle tests in dedicated files. Not in `Tabs.test.ts` /
  `Tabs.test.tsx`, which stub `requestAnimationFrame` as sync.

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
- `bunx vitest run` `Tabs.test.ts` / `Tabs.test.tsx`, both controlled-focus
  files, and `catalogue-nav.test.tsx` — 5 files, 53 pass

Required boards on the dirty worker tree, then again after refresh onto
`d82ba7202aaecff452ec2c59e79ea7be3be114af`:

- `effigy ci:web` — pass (gate-tree-guard clean after committing
  `packages/react/preview/artifacts/component-docs.json`)
- `effigy docs:check` — pass
- `git diff --check origin/main...HEAD` — pass

Also repaired Vitest `react-components` / `react-preview` to inherit
`workspaceAliases` so `@inflatable-cookie/poodle-react` resolves to source
after g16.058 dist-only exports.

After rebase onto certified main, `test:web-pack-install` rejected the Tabs
paths as outside the g16.059 certification allowlist. The scope guard now
keeps forbidden-surface rejection, classifies ordinary/unchanged ranges
without failing pack-install, and only treats allowlist-pure ranges as
certification.

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
