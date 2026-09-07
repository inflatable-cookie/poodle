# g17.002 — Web Focus Rings Follow Input Modality

Status: ready
Type: presentation rule + shared web helper — no component API change
Opened: 2026-09-07
Depends on: none
Governing refs: `../../contracts/001-working-rules.md` ("Focus Visibility"),
`../../contracts/components/token-input.md`, `agent-chat-input.md`,
`select.md`, `filter-builder.md`, `duration-input.md`, `number-input.md`,
`order-by.md`, `../../architecture/006-headless-core-and-machine-model.md`
Operator decision: 2026-09-07 — a focus ring is for keyboard interaction;
mouse interaction must not paint one. The TokenInput composer ring was the
reported case.
Dispatch manifest: `../dispatch.md`

## Goal

Stop composite web controls painting a focus ring, or any "focus treatment",
when a pointer moves focus into them. Keyboard users keep every ring. The
browser's own `:focus-visible` already does this for simple controls (173
rules). Two classes leak:

- Seven container rules use `:focus-within`, which has no keyboard-only
  form: `token-input.css:31`, `agent-chat-input.css:47`, `select.css:59`
  and `:367` (ghost), `filter-builder.css:44`, `duration-input.css:34`,
  `number-input.css:47`, `order-by.css:55`.
- Six bare `:focus` rules: `log-list.css:89`, `duration-input.css:56`,
  `color-picker.css:70` and `:265`, `agent-chat-input.css:141`.

`:has(:focus-visible)` is not enough: browsers match `:focus-visible` on text
inputs for pointer focus too, so a composer would still light up on click.
The fix is an input-modality tracker, the standard `what-input` shape.

## Fixed Boundary

- Add `packages/core/src/dom/input-modality.ts`: one idempotent per-document
  installer that listens (capture phase, passive) for `keydown` (excluding
  modifier-only keys) → `keyboard`, and `pointerdown` / `mousedown` /
  `touchstart` → `pointer`, and writes
  `data-poodle-input-modality="keyboard" | "pointer"` on
  `document.documentElement`. Initial value: `keyboard` (safe default for
  focus arriving before any input, e.g. autofocus after page load). Export
  `installInputModality(doc?)` and `getInputModality(doc?)` from core's
  index. No global singleton beyond the document attribute; SSR-safe (no-op
  without `document`).
- Every Svelte and React component whose CSS gates on modality calls the
  installer on mount (the seven composites above). Consumers need no setup.
  Do not add a provider or context.
- Rewrite the seven container rules as
  `:root[data-poodle-input-modality="keyboard"] .poodle-token-input:focus-within`
  (same shape for each). The whole focus treatment gates, not only the
  ring: border, fill, and shadow move together, per the operator's report.
  Pointer focus keeps whatever the resting/hover state already shows.
- Rewrite the six bare `:focus` rules as `:focus-visible`.
- Contracts: in each affected component contract, change the focus state
  rows and the "Root — focus-within" style tables to say "keyboard focus
  within" and name the root attribute. Svelte is the authority; the
  contract follows the implementation this card lands.
- Add the "Focus Visibility" rule to `docs/contracts/001-working-rules.md`
  (Chatterbox wrote the rule text; the card only cites it).
- No component prop, no token change, no GPUI change. GPUI keyboard-origin
  gating is a separate g17 card.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Pointer focus paints no treatment | click into TokenInput's live input | computed `box-shadow` and `border-color` equal the resting values |
| Keyboard focus paints it | Tab into the same input | computed values equal the focus recipe |
| Tracker flips both ways | pointerdown then keydown then pointerdown | attribute reads `pointer`, `keyboard`, `pointer` |
| Idempotent | install twice on one document | one listener set (spy count) |
| SSR-safe | call without `document` | no throw |
| Bare `:focus` gone | `grep -P ':focus(?!-visible\|-within)' packages/core/src/styles` | zero matches |
| Both shells install | mount React and Svelte TokenInput in jsdom | attribute present on the root |
| Contract agrees | `effigy docs:contract-drift`, `docs:check` | green |

Prove the first two rows with the headless Playwright web probe path
(`test/drag-drop/probe.ts` is the existing shape) on Chromium and WebKit,
foreground, one engine per run; record computed values in the execution
log. Unit-test the tracker in the vitest `packages/svelte/preview/test/**`
project.

## Validation

`effigy test:components`, `effigy docs:check`, `effigy ci:web`,
`git diff --check origin/main...HEAD`. Never run release, windowed, or
native selectors.

## Owned Paths

`packages/core/src/dom/input-modality.ts`, `packages/core/src/index.ts`
(export lines), the seven CSS files and `log-list.css`, `color-picker.css`,
the seven Svelte and seven React components (mount call only), the seven
component contracts (focus rows and tables), the tracker test, the probe
transcript, execution log under `docs/logs/2026-09/`, `PAPERCUTS.md`
(append). Reserved for the coordinator at merge: `docs/roadmaps/g17/README.md`,
`docs/roadmaps/generation-index.md`.

## Stop Conditions

Stop and report when: a component paints its focus ring on an element other
than the ones listed (report it, do not widen); jsdom cannot express a
required proof (use the probe path instead of weakening the oracle);
`ci:web` fails for an unrelated reason. Escalation owner: Chatterbox.

## Continuation

GPUI paints the ring whenever the element or its subtree is focused
(`packages/gpui/node-backend/src/interaction.rs:376`) with no
keyboard-origin concept; that gate is the next Nucleus-facing card once the
web rule lands.
