# 009 Dialog Initial-Focus Hook

Status: merged (`e4af527e`)
Milestone: side-quest (consumer-driven component work, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-009-dialog-initial-focus`
Depends on: `b1a4a5e7` (Dialog focus guard + restore deferral, TextInput
`autofocus`/`focus()`)
Governing refs: `docs/contracts/components/dialog.md`,
`docs/contracts/components/form-dialog.md`,
`docs/contracts/001-working-rules.md`

## Need

Consumers cannot control which element receives focus when a `Dialog` or
`FormDialog` opens. `Dialog` focuses `focusable[0]` inside the surface, which is
the header close button — never the right target for a form dialog. Apps work
around it by exporting `focus()` from `TextInput` and wiring `bind:this` plus an
`$effect` per dialog. That boilerplate belongs in Poodle.

## Ask

Add an initial-focus mechanism to `Dialog` and default it sensibly in
`FormDialog`.

- `Dialog` gains an `initialFocus` prop. Default `"auto"` prefers the first
  focusable element in the **content region** (`.poodle-dialog__body`), skipping
  header chrome (`.poodle-dialog__close`), and falls back to the surface itself
  when the body has no focusable element.
- `FormDialog` defaults to focusing its first field on open.
- Never steal focus when something inside the surface is already focused — the
  guard added in `b1a4a5e7` already does this and must be preserved.

## Already Landed (your base — do not redo)

Commit `b1a4a5e7` is the parent of your branch and already contains:

- `Dialog.svelte` — open-focus guard: skips `focusable[0]` when
  `surfaceElement` already contains `document.activeElement`.
- `Dialog.svelte` — close focus-restore deferred one macrotask (`setTimeout 0`)
  so a pending Enter keyup cannot re-activate the trigger and reopen the dialog.
- `TextInput.svelte` — `autofocus` prop forwarded to the native
  input/textarea, exported `focus()`, bound `control` reference.

Build on these. Do not revert, restructure, or "clean up" any of them.

## Prop Shape — decide, then apply consistently

The need statement floats three possible shapes (union string, predicate,
selector). Pick **one** and use it in Svelte, React, and both contracts.

Recommended, unless the code argues otherwise:

```
initialFocus?: "auto" | "none" | string
```

`"auto"` (default) — first focusable in `.poodle-dialog__body`, else the
surface. `"none"` — focus nothing; the surface still traps focus. A string is
treated as a CSS selector resolved **within the surface**; if it matches
nothing, fall back to `"auto"` behaviour rather than throwing.

If you choose a different shape, record the reason in the batch log. Do not
implement two shapes.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- **Svelte and React must stay at parity.** They are parity-tested against each
  other (`effigy test:parity`). Implement the same prop, same semantics, same
  defaults in both.
- Contracts are authority and must be updated in the same commit — a new public
  prop that is not in the contract's §3 props table is incomplete work.
- Do not change any component other than `Dialog` and `FormDialog` in the two
  web runtimes, and their two contracts.
- Do not touch Rust, native adapters, specs, or the `poodle-render` tier. If
  the Rust `DialogSpec` appears to need the prop, that is a stop condition —
  report it, do not implement it.
- Do not refresh any visual or native baseline.
- Do not edit roadmap/milestone/card status files or `docs/roadmaps/dispatch.md`.
- `effigy docs:check` rewrites `packages/tokens/artifacts/rust/*`; restore with
  `git checkout -- packages/tokens/artifacts/rust/` and never commit it.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Run `bun install` before any web generator or build command.
- Commit and push with
  `git push -u origin thread/g13-009-dialog-initial-focus`. Do not merge.

## Writable Paths

- `packages/svelte/components/src/Dialog.svelte`
- `packages/svelte/components/src/FormDialog.svelte`
- `packages/react/components/src/Dialog.tsx`
- `packages/react/components/src/FormDialog.tsx`
- `docs/contracts/components/dialog.md`
- `docs/contracts/components/form-dialog.md`
- `packages/svelte/preview/src/component-docs.ts` (usage docs for the new prop)
- Tests under `packages/core/test/`, `test/`, or the components' existing test
  files, as needed
- `docs/logs/2026-08/<DD>-g13-009-dialog-initial-focus.md`
- `PAPERCUTS.md` (new, non-duplicate friction only)

Regenerated preview artifacts are permitted only as generator output.

## Steps

1. **Baseline.** `bun install`, `effigy test:components`, `effigy test:parity`,
   `effigy docs:lint`, `git diff --check`. All should exit 0 on the branch
   point. Record exit states.
2. **Read first.** `Dialog.svelte` (focus lifecycle around lines 105–145),
   `Dialog.tsx`, `FormDialog.svelte` (it wraps `Dialog` — line 114), and
   `FormDialog.tsx`. Note how `getFocusableElements`
   (`@inflatable-cookie/poodle-core`, re-exported via
   `packages/svelte/components/src/internal.ts:95`) is used and whether it can
   be scoped to a subtree.
3. **Implement `initialFocus` on `Dialog`** in Svelte and React. Preserve the
   `b1a4a5e7` guard: the already-focused check runs *before* any
   `initialFocus` resolution. Preserve the deferred close restore untouched.
4. **Default `FormDialog` to its first field.** Prefer expressing this by
   passing an appropriate `initialFocus` down to `Dialog` rather than adding a
   second focus mechanism. A consumer-supplied `initialFocus` on `FormDialog`
   must win over the default.
5. **Update both contracts.** Add `initialFocus` to each §3 props table with
   type, default, and behaviour; describe the `"auto"` resolution order and the
   already-focused guard in the accessibility/behaviour section. Document
   `FormDialog`'s differing default explicitly.
6. **Usage docs.** Add the prop to the Dialog and FormDialog entries in
   `component-docs.ts`.
7. **Tests.** Cover, at minimum: `"auto"` skips the close button and lands on
   the first body focusable; `"none"` focuses nothing; a selector resolves
   within the surface; an unmatched selector falls back to `"auto"`; an
   already-focused element inside the surface is not stolen; `FormDialog`
   focuses its first field by default and a consumer override wins.
8. **Validate.**
   ```sh
   effigy test:components
   effigy test:parity
   effigy docs:lint
   effigy docs:contract-drift
   effigy docs:check
   git checkout -- packages/tokens/artifacts/rust/
   git diff --check
   git status --porcelain
   ```
   Record every command and exit state.

## Acceptance Criteria

- [x] `Dialog` accepts `initialFocus` with one consistent shape in Svelte and
  React, defaulting to `"auto"`.
- [x] `"auto"` prefers the first focusable in `.poodle-dialog__body`, skips the
  header close button, and falls back to the surface.
- [x] `FormDialog` focuses its first field by default; a consumer-supplied
  value wins.
- [x] The `b1a4a5e7` already-focused guard and deferred close restore are
  preserved and still work.
- [x] `dialog.md` and `form-dialog.md` document the prop, its resolution order,
  and FormDialog's differing default.
- [x] Usage docs updated for both components.
- [x] Tests cover every case in step 7.
- [x] Svelte and React remain at parity; no other component changed; no Rust,
  adapter, or baseline touched.
- [x] All commands in step 8 exit 0.
- [x] Batch log records commands, exit states, and the chosen prop shape with
  its rationale.

## Stop Conditions

- Implementing the default requires changing a component other than `Dialog`
  and `FormDialog`.
- The Rust `DialogSpec` or a native adapter appears to need the prop.
- Svelte and React cannot express the same semantics without diverging.
- `getFocusableElements` cannot be scoped to the body region without changing
  `poodle-core` (report it — a core change is a separate decision).
- A visual baseline would need refreshing.

Stop with exact paths, line numbers, commands, and the smallest unresolved
question. Do not patch around it.
