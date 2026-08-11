# 11 — g13.010 TextInput Focus Parity (autofocus + focus()) — batch log

Branch: `thread/g13-010-text-input-focus-parity` (dedicated worktree)
Date: 2026-08-11
Card: `docs/roadmaps/g13/batch-cards/010-text-input-focus-parity.md`
Status: **STOPPED — `docs:spec-drift` red** (card's named stop condition; see
§6). All implementation, docs, usage docs, and tests for the card are done and
green under `test:components` / `test:parity`; the delivery is blocked on one
one-line exclusion in a script outside the card's writable paths. Nothing
committed (a red-gate state is not a delivery).

## 1. Reference read (steps 2–4 of the card)

- Svelte reference (`packages/svelte/components/src/TextInput.svelte`): prop
  `autofocus?: boolean` default `false` (line ~40, destructured ~57); forwarded
  as `autofocus={autofocus || undefined}` to BOTH the `<textarea>` branch
  (line 530) and the `<input>` branch (line 570) — falsy omits the attribute;
  `let control = $state<...>(null)` bound via `bind:this` on both branches;
  `function focus(): void { control?.focus(); }` exported via `export { focus }`.
- Handle pattern precedent (per card): Svelte `MenuSurface.svelte` exports
  `focusFirstItem` / `moveHighlight` / `moveToBoundary`; React
  `MenuSurface.tsx` exposes the equivalent via
  `forwardRef<MenuSurfaceHandle, MenuSurfaceProps>` +
  `useImperativeHandle(ref, () => ({ ... }))` with an exported handle
  interface. React `TextInput` was a plain function component — converted.
- Dependency commit `b1a4a5e7` confirmed as the Svelte-only source of the gap.

## 2. React port — `packages/react/components/src/TextInput.tsx`

- `TextInputProps` gains `autofocus?: boolean` (after `readOnly`, mirroring the
  Svelte Props interface order).
- New `export interface TextInputHandle { focus: () => void }` (type export
  only; the package `index.ts` was NOT touched — it is outside the card's
  writable paths, so the handle type is importable from `./TextInput`).
- `export function TextInput(...)` → `export const TextInput =
  forwardRef<TextInputHandle, TextInputProps>(function TextInput(...))` —
  named export unchanged, so all existing consumers (CommandPalette, DataTable,
  EditableList, EmbedInput, FilterBuilder, LogList, MediaPicker, RefSelect,
  RelationPicker, preview gallery) import unchanged.
- `const controlRef = useRef<HTMLInputElement | HTMLTextAreaElement | null>(null)`
  attached via `ref={controlRef}` to BOTH the `<textarea>` and `<input>`.
- `useImperativeHandle(ref, () => ({ focus: () => controlRef.current?.focus() }))`
  — focuses the underlying control, never the wrapper (Svelte: `control?.focus()`).
- `autoFocus={autofocus || undefined}` on BOTH branches — falsy → attribute
  absent, mirroring Svelte's `autofocus={autofocus || undefined}`.
- No other prop's behaviour altered. `git diff` on the file shows only the
  conversion + the two features.

### Mechanism note (why the React tests assert mount-focus, not the attribute)

React's canonical `autoFocus` prop does not set the DOM attribute on client
renders — `react-dom-client.development.js`: `case "autoFocus": break` in
`setProp` (line 20187) and `newProps.autoFocus && domElement.focus()` in
`commitMount` (line 22158). The attribute IS emitted in server markup
(`react-dom/server` renders `<input autofocus=""/>` for `autoFocus={true}`).
Svelte renders the attribute and the browser focuses. Both produce the same
browser-visible result (control focused when it appears); each suite asserts
its own mechanism (Svelte: attribute; React: mount focus), and the falsy side
asserts no attribute and no focus on both.

## 3. Contract docs — `docs/contracts/components/text-input.md`

- §3 Public Props table: `autofocus` row (type `boolean`, default `false`)
  after `readOnly`, marked web-only native attribute, "Excluded from
  `TextInputSpec` alongside the other native attributes".
- §3 new subsection "### Imperative Methods": documents `focus()` — moves
  focus to the underlying control; Svelte exports it from the component
  instance, React exposes it via the ref handle (`TextInputHandle`).
- Statement that neither reaches `TextInputSpec`: `autofocus` is a web-native
  attribute like `autocomplete`/`spellcheck`/`autocapitalize`/`autocorrect`/
  `enterKeyHint`; `focus()` is an imperative escape hatch into the DOM — Rust
  targets own focus through `isFocused` (Caret Ownership, §6).

## 4. Usage docs — `packages/svelte/preview/src/component-docs.ts` (`text-input` entry only)

- `autofocus` prop row added to the `text-input` props array (after
  `readOnly`).
- `usage` snippet now shows `bind:this={emailInput}`, `autofocus`, and a
  `<button onclick={() => emailInput?.focus()}>` demonstrating `focus()`.
- Every other entry byte-identical (verified: `git diff` touches only the
  `text-input` block). The `ComponentDocs` type has no `methods` key and the
  `UsageDocs.svelte` renderer has no methods section, so the imperative method
  is documented in the rendered usage snippet rather than an unrendered schema
  extension.

## 5. Tests

- `packages/svelte/components/test/TextInput.test.ts` (new): autofocus
  attribute present when true / absent when false, input and textarea modes;
  exported `focus()` moves focus to the underlying control.
- `packages/react/components/test/TextInput.test.tsx` (new): mount focus when
  `autofocus` true (input + textarea modes); no attribute and no focus when
  false; `focus()` via ref (`TextInputHandle`) moves focus to the control.
- Pair mirrors the `Button.test.ts` / `Button.test.tsx` convention; each file
  documents the mechanism-equivalence contract.

## 6. Validation and the stop condition

| Command | Exit | Notes |
|---|---|---|
| `bun install` | 0 | 234 packages |
| `effigy test:components` | 0 → 0 | baseline 810 tests; after 818 (40 files, incl. the 10 new focus tests) |
| `effigy test:parity` | 0 → 0 | anatomy parity green after forwardRef conversion |
| `effigy docs:lint` | 0 → **1** | red on the card's stop condition (see below) |
| `effigy docs:contract-drift` | — → 0 | Svelte already implements `autofocus` |
| `effigy docs:spec-drift` | — → **1** | `1 documented prop(s) missing from poodle-specs across 1 component(s): [text-input] autofocus` |
| `effigy docs:check` | — → **1** | fails at its `docs:lint` stage; `packages/tokens/artifacts/rust/*` were rewritten as the card warned, restored with `git checkout -- packages/tokens/artifacts/rust/` (exit 0), nothing from there staged/committed |
| `git diff --check` | 0 | — |
| `git status --porcelain` | — | only the card's writable paths (below) |

**Stop condition (card §Stop Conditions, first bullet):** `docs:spec-drift`
goes red after documenting the web-only prop.

Mechanism, as the card asked to be reported ("how the contract marks web-only
props for the other native attributes"):

- The gate `packages/svelte/preview/scripts/contract-spec-drift.ts` parses
  every row of the contract's `### Public Props` table (`contractProps()`,
  lines 186–201) and flags each documented prop that is not a `TextInputSpec`
  field (`covered()`, lines 267–284), not an alias, and not in `WEB_ONLY_PROPS`
  (filter at lines 305–313).
- The other web-native attributes stay green because they are members of the
  hard-coded `WEB_ONLY_PROPS` set (lines 29–78): `autocapitalize`,
  `autocorrect`, `enterKeyHint`, `spellcheck`, `type`, `list`, `name`, `id`,
  `form*`, etc. The contract text itself carries no marker the gate reads.
- `autofocus` is not in that set, and `TextInputSpec`
  (`packages/contracts/components/src/text_input.rs`, struct lines 6–67) has
  no `autofocus` field — only `is_focused` (host-driven Rust focus, which
  does not match the `autofocus`/`is_autofocus`/`has_autofocus` variants).

**Smallest unresolved question:** may the one-line exclusion `"autofocus",`
be added to `WEB_ONLY_PROPS` in
`packages/svelte/preview/scripts/contract-spec-drift.ts` (alongside
`autocapitalize`/`autocorrect`/`enterKeyHint`/`spellcheck` at line ~38)? That
file is not in the card's writable paths, so this worker stopped per the
card's stop-condition instruction rather than editing it. With that one line,
the remaining steps (`docs:lint`, `docs:check`, log, commit, push) finish.

## 7. Changed paths (uncommitted — stop condition pending)

```
 docs/contracts/components/text-input.md          autofocus row + Imperative Methods section
 packages/react/components/src/TextInput.tsx      autofocus + forwardRef/useImperativeHandle focus()
 packages/svelte/preview/src/component-docs.ts    text-input entry only (autofocus prop + usage snippet)
 packages/svelte/components/test/TextInput.test.ts      new focus tests (svelte)
 packages/react/components/test/TextInput.test.tsx      new focus tests (react)
 docs/logs/2026-08/11-g13-010-text-input-focus-parity.md  this log
```

Nothing else changed: no Rust, adapters, `poodle-specs`, `poodle-render`,
Svelte components, baselines, or roadmap/status files. No visual baseline
touched. `PAPERCUTS.md`: no new non-duplicate friction found (the pre-existing
React TextInput `autocorrect` gap is already tracked there and is out of this
card's scope).
