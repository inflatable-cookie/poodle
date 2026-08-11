# 11 — g13.009 Dialog Initial-Focus Hook (batch log)

Branch: `thread/g13-009-dialog-initial-focus` (dedicated worktree, pushed with
`git push -u origin thread/g13-009-dialog-initial-focus`)
Date: 2026-08-11
Card: `docs/roadmaps/g13/batch-cards/009-dialog-initial-focus.md`
Status: **STOPPED** — implementation complete and all runtime tests green, but
`effigy docs:lint` and `effigy docs:check` fail because the contract ↔
poodle-specs drift gate requires `DialogSpec`/`FormDialogSpec` to carry
`initial_focus` (see §5). Per the card's stop conditions, the Rust Spec is
reported, not implemented.

## 1. Baseline (step 1)

| Command | Exit | Notes |
|---|---|---|
| `bun install` | 0 | 234 packages installed |
| `effigy test:components` | 0 | 38 files / 810 tests |
| `effigy test:parity` | 0 | 2 files / 158 tests |
| `effigy docs:lint` | 0 | 170 contracts validated |
| `git diff --check` | 0 | clean tree at branch point `b1a4a5e7` |

## 2. Prop shape — decided (card step 3)

Chosen: `initialFocus?: "auto" | "none" | string`, default `"auto"` — the
card's recommended shape. Used identically in Svelte and React, and in both
contracts.

Rationale:

- One union string covers all three card-floated shapes (union string,
  predicate, selector) without introducing a new type or a framework-specific
  callback shape. A predicate would need a Snippet on the Svelte side and a
  render-prop on the React side — two idioms for one semantic, exactly the
  parity cost the card warns about.
- `"auto"` keeps the common case (first focusable in the body) a default, not
  a consumer decision; it is expressed through the existing shared
  `getFocusableElements` (poodle-core), which accepts any root element, so
  scoping to `.poodle-dialog__body` needs no core change (stop-condition #4
  checked: not hit).
- `"none"` is the escape hatch for hosts that manage focus themselves; the
  focus trap still applies (surface keydown handler unchanged).
- A string selector covers the app pattern the card exists to kill — the
  `bind:this` + `$effect` boilerplate — with one attribute.

`Dialog` resolves in this order, with the b1a4a5e7 already-focused guard
running **before** any resolution (Svelte: guard in the open-edge `$effect`
before `focusInitialElement()`; React: guard in the surface ref callback
before `resolveInitialFocus(node)`):

1. `"none"` → no focus.
2. non-`"auto"` string → `surface.querySelector(selector)`; matched → focus
   it; unmatched → fall through to auto (never throws).
3. `"auto"` (and the fallback) → first focusable in
   `surface.querySelector(".poodle-dialog__body")`; body has none, or bare
   mode (no body region) → the surface itself.

`FormDialog` exposes the same prop, default `"auto"`, passed through to
`Dialog` — no second focus mechanism. The default lands on the first field
because the FormDialog body region is always the form (FormLayout/`body`
snippet); a consumer-supplied `initialFocus` wins because it is forwarded
unchanged.

## 3. Implementation (card steps 3–4, 6)

- `packages/svelte/components/src/Dialog.svelte` — `initialFocus` prop;
  open-edge `$effect` tick callback now runs the b1a4a5e7 guard first
  (`!surfaceElement || surfaceElement.contains(document.activeElement)` →
  return), then `focusInitialElement()`. The `setTimeout(0)` close
  focus-restore from b1a4a5e7 is untouched.
- `packages/react/components/src/Dialog.tsx` — `initialFocus` prop; surface
  ref callback consumes `pendingFocus` with the already-focused guard first
  (`!node.contains(document.activeElement)`), then `resolveInitialFocus(node)`.
  b1a4a5e7 was Svelte-only (its commit message says so: "these are
  Svelte-only"), so the guard did not exist on the React side; it is added
  here as part of the initialFocus semantics, which the card requires to be
  at parity ("Never steal focus when something inside the surface is already
  focused — the guard … must be preserved").
- `packages/svelte/components/src/FormDialog.svelte` and
  `packages/react/components/src/FormDialog.tsx` — `initialFocus` prop,
  default `"auto"`, forwarded to `Dialog`.
- `docs/contracts/components/dialog.md` — `initialFocus` row in §3 Public
  Props; §4 Behavior Machine open-effect bullet now points at §6; §6 Focus
  And Announcement describes the resolution order and the already-focused
  guard; §9 Svelte Notes bullet; §11 Tier-1 parity checklist row.
- `docs/contracts/components/form-dialog.md` — `initialFocus` row in §3 with
  the differing default (first field) and bare-mode note; §6 focus section;
  §9 Svelte Notes bullet.
- `packages/svelte/preview/src/component-docs.ts` — `initialFocus` in the
  Dialog and FormDialog usage-doc prop tables.

## 4. Latent `??` bug found and fixed during implementation

The base open-edge line was
`focusable[0]?.focus() ?? surfaceElement?.focus()`. `focus()` returns
`undefined`, so the `??` fallback always evaluated: the first focusable was
focused and then the surface was focused over it, so the surface always won.
The card's acceptance criteria require `"auto"` to land on the first body
focusable, so the rewritten resolution uses element-level coalescing —
`const target = focusable[0] ?? surface; target.focus()` — matching the
React spelling `(focusable[0] ?? node).focus()`. This was in the open-edge
code the card has me rewrite; the b1a4a5e7 guard and deferred close restore
are untouched.

## 5. Stop condition — `DialogSpec`/`FormDialogSpec` would need the prop

**Finding.** The card's stop condition "The Rust `DialogSpec` or a native
adapter appears to need the prop" is hit. Evidence:

- `packages/svelte/preview/scripts/lint-docs.ts:3272-3279` —
  `contractSpecDrift()` is part of `effigy docs:lint` and fails on any
  documented Public Prop absent from the poodle-specs `Spec`:
  "contract/spec drift: dialog.md documents prop(s) absent from its
  poodle-specs Spec: initialFocus" (same for form-dialog.md).
- `packages/svelte/preview/scripts/contract-spec-drift.ts:118-126` — the
  `OPEN_GAPS` baseline comment: "a prop shipped to the web without reaching
  the shared spec surface … is the thing this gate exists to stop." Adding
  `initialFocus` there is the classified-debt path, not a fix, and the file
  is outside the card's writable paths.
- `packages/contracts/components/src/dialog.rs:6-26` — `DialogSpec` has no
  `initial_focus` field (and no `with_initial_focus` builder).
- `packages/contracts/components/src/form_dialog.rs:8-30` — `FormDialogSpec`
  has no `initial_focus` field.
- `WEB_ONLY_PROPS` (`contract-spec-drift.ts:51-100`) is for web-platform
  plumbing (className/style/ARIA ids/HTML attributes). `initialFocus` is
  component semantics — the dialog contract §10 GPUI notes already assign
  focus handling to the native renderers ("GPUI implementation must
  explicitly own modal stacking, focus trapping, background blocking,
  announcement, and restoration behavior") — so classifying it web-only
  would be a false entry.

**Why stopped, not patched.** Implementing `initial_focus` on the Specs is
Rust work the card explicitly forbids ("report it, do not implement it") and
is outside every writable path. Removing the prop from the contracts would
violate the card's own rule that the prop must land in the contract in the
same commit, and would leave the implementation undocumented.

**Smallest unresolved question.** Should `DialogSpec`/`FormDialogSpec` gain
`initial_focus` (with native renderers honoring it), or is initial-focus a
web-only decision — in which case `contract-spec-drift.ts` needs a sanctioned
exception (e.g. `WEB_ONLY_PROPS` or a documented web-only carve-out)? That is
a spec-surface decision owned by the repository, not this card.

## 6. Validation (card step 8) — run on the delivered state

| Command | Exit | Notes |
|---|---|---|
| `effigy test:components` | 0 | 40 files / 826 tests (was 38/810; +16 new focus tests) |
| `effigy test:parity` | 0 | 2 files / 158 tests |
| `effigy docs:lint` | **1** | contract/spec drift on `initialFocus` (dialog.md, form-dialog.md) — §5 |
| `effigy docs:contract-drift` | 0 | 128 checked, 35 skipped — Svelte implements every documented prop |
| `effigy docs:check` | **1** | fails inside docs:lint for the same drift; no `packages/tokens/artifacts/rust/*` rewrite occurred |
| `git checkout -- packages/tokens/artifacts/rust/` | 0 | no-op safeguard; nothing dirty |
| `git diff --check` | 0 | — |
| `git status --porcelain` | — | only the writable paths below + new test files |

New focused tests (both runtimes, mirroring each other):
`DialogInitialFocus.svelte.test.ts` / `DialogInitialFocus.test.tsx` + two
Svelte harness components. Each covers the step-7 minimum: `"auto"` skips
the close button and lands on the first body focusable (also the no-prop
default); `"none"` focuses nothing; a selector resolves within the surface;
an unmatched selector falls back to `"auto"`; an already-focused element
inside the surface is not stolen (Svelte: focus before the tick resolution
runs; React: a child inline ref focuses on attach, before the parent surface
ref); FormDialog focuses its first field by default and a consumer override
wins.

Changed paths:

```
 docs/contracts/components/dialog.md                        | initialFocus documented
 docs/contracts/components/form-dialog.md                   | initialFocus documented
 packages/svelte/components/src/Dialog.svelte               | initialFocus + guard-first resolution
 packages/svelte/components/src/FormDialog.svelte           | initialFocus pass-through
 packages/react/components/src/Dialog.tsx                   | initialFocus + guard-first resolution
 packages/react/components/src/FormDialog.tsx               | initialFocus pass-through
 packages/svelte/preview/src/component-docs.ts              | usage docs for both
 packages/svelte/components/test/DialogInitialFocus.svelte.test.ts
 packages/svelte/components/test/DialogInitialFocusHarness.svelte
 packages/svelte/components/test/FormDialogInitialFocusHarness.svelte
 packages/react/components/test/DialogInitialFocus.test.tsx
 docs/logs/2026-08/11-g13-009-dialog-initial-focus.md       | this log
 PAPERCUTS.md                                               | friction entry
```

## 7. Acceptance criteria — status

- [x] `Dialog` accepts `initialFocus` with one consistent shape in Svelte and
  React, defaulting to `"auto"` — implemented and tested.
- [x] `"auto"` prefers the first focusable in `.poodle-dialog__body`, skips
  the header close button, and falls back to the surface — implemented and
  tested.
- [x] `FormDialog` focuses its first field by default; a consumer-supplied
  value wins — implemented and tested.
- [x] The b1a4a5e7 already-focused guard and deferred close restore are
  preserved and still work — guard verified by tests in both runtimes;
  `setTimeout(0)` close restore untouched. Note: the guard was Svelte-only
  in b1a4a5e7; React gained it as part of initialFocus parity (§3).
- [x] `dialog.md` and `form-dialog.md` document the prop, its resolution
  order, and FormDialog's differing default.
- [x] Usage docs updated for both components.
- [x] Tests cover every case in card step 7 — both runtimes.
- [x] Svelte and React remain at parity (`test:parity` 0); no other component
  changed; no Rust, adapter, or baseline touched.
- [ ] All commands in step 8 exit 0 — **not met**: `docs:lint` and
  `docs:check` exit 1 on contract/spec drift (§5).
- [x] Batch log records commands, exit states, and the chosen prop shape
  with its rationale — this log.

No `test:visual`/`test:a11y` baseline invoked; nothing refreshed. No
roadmap/card/status files or `docs/roadmaps/dispatch.md` touched. No
`git add -A` used; only writable paths staged explicitly.
