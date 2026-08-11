# 010 TextInput Focus Parity (autofocus + focus())

Status: ready
Milestone: side-quest (runtime parity, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-010-text-input-focus-parity`
Depends on: `b1a4a5e7` (Svelte `autofocus` + `focus()`)
Governing refs: `docs/contracts/001-working-rules.md` §Runtime Parity Authority,
`docs/contracts/components/text-input.md`

## Need

`b1a4a5e7` added `autofocus` and an exported `focus()` to the Svelte
`TextInput`. Neither exists in React, and `text-input.md` documents neither. By
the Runtime Parity Authority rule, a capability in Svelte and absent elsewhere
is a gap to port, and an undocumented capability is drift on every side.

The maintainer has ruled both features **mandatory** for `TextInput`.

## Ask

1. Port `autofocus` and `focus()` to the React `TextInput`.
2. Document both in `text-input.md`.

## Established Patterns (follow these — do not invent)

**`MenuSurface` is the precedent for a cross-runtime imperative handle.**
`packages/svelte/components/src/MenuSurface.svelte` exports a method and
`packages/react/components/src/MenuSurface.tsx` exposes the equivalent through
`useImperativeHandle`. Read both before writing anything, and match that shape.

React `TextInput` is currently a plain function component
(`packages/react/components/src/TextInput.tsx:94`, `export function TextInput({`)
with no `forwardRef`. Converting it is expected; keep the change minimal and do
not alter any other prop's behaviour.

**Classification, already decided** (`001-working-rules.md` §Runtime Parity
Authority):

- `autofocus` is a **web-native attribute**, like `autocomplete` and
  `spellcheck`. It belongs in both web runtimes and the contract's §3 props
  table, and stays **excluded from `TextInputSpec`**. Do not add it to Rust.
- `focus()` is an **imperative escape hatch**. Document it as a method, not a
  prop. Both web runtimes expose it. Not a Rust concern.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Do not touch Rust, native adapters, `poodle-specs`, or `poodle-render`. If
  `TextInputSpec` appears to need either feature, that is a stop condition —
  the classification above says it does not.
- Do not change the Svelte `TextInput`'s existing behaviour. It is the
  reference; React conforms to it. Fixing a genuine Svelte bug you find is a
  stop condition to report, not to act on.
- Do not change any component other than React `TextInput`.
- Do not refresh any visual or native baseline.
- Do not edit roadmap/milestone/card status files or `docs/roadmaps/dispatch.md`.
- Another worker is concurrently editing
  `packages/svelte/preview/src/component-docs.ts` for Dialog/FormDialog. Touch
  only the `text-input` entry in that file and leave every other entry byte
  identical, so the two branches merge cleanly.
- `effigy docs:check` rewrites `packages/tokens/artifacts/rust/*`; restore with
  `git checkout -- packages/tokens/artifacts/rust/` and never commit it.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Run `bun install` before any web generator or build command.
- Commit and push with
  `git push -u origin thread/g13-010-text-input-focus-parity`. Do not merge.

## Writable Paths

- `packages/react/components/src/TextInput.tsx`
- `docs/contracts/components/text-input.md`
- `packages/svelte/preview/src/component-docs.ts` (`text-input` entry only)
- Tests under `test/` or the components' existing test files
- `docs/logs/2026-08/<DD>-g13-010-text-input-focus-parity.md`
- `PAPERCUTS.md` (new, non-duplicate friction only)

Regenerated preview artifacts are permitted only as generator output.

## Steps

1. **Baseline.** `bun install`, `effigy test:components`, `effigy test:parity`,
   `effigy docs:lint`, `git diff --check`. Record exit states.
2. **Read the reference.** The Svelte implementation in
   `packages/svelte/components/src/TextInput.svelte`: the `autofocus` prop, its
   forwarding as `autofocus={autofocus || undefined}` to both the `input` and
   the `textarea` branches, the bound `control` reference, and the exported
   `focus()`. Then read both `MenuSurface` implementations for the handle
   pattern.
3. **Port `autofocus`** to React `TextInput`, covering **both** the input and
   textarea branches, matching Svelte's semantics exactly (falsy → attribute
   absent, not `autofocus={false}`).
4. **Port `focus()`** via `useImperativeHandle`, following `MenuSurface.tsx`.
   It must focus the same element Svelte's does — the underlying
   input/textarea, not a wrapper.
5. **Document both** in `text-input.md`: `autofocus` in the §3 props table with
   type and default, marked web-only and excluded from the portable spec
   alongside the other native attributes; `focus()` as an imperative method in
   the appropriate section, noting both web runtimes expose it. State that
   neither reaches `TextInputSpec`.
6. **Usage docs.** Add both to the `text-input` entry in `component-docs.ts`.
7. **Tests.** Cover: `autofocus` renders the attribute when true and omits it
   when false, in both input and textarea modes; `focus()` moves focus to the
   control; React and Svelte agree.
8. **Validate.**
   ```sh
   effigy test:components
   effigy test:parity
   effigy docs:lint
   effigy docs:contract-drift
   effigy docs:spec-drift
   effigy docs:check
   git checkout -- packages/tokens/artifacts/rust/
   git diff --check
   git status --porcelain
   ```
   Record every command and exit state. `docs:spec-drift` must stay green —
   documenting a web-only prop must not make the contract look wider than
   `TextInputSpec`. If it goes red, that is a stop condition: report how the
   contract marks web-only props for the other native attributes.

## Acceptance Criteria

- [ ] React `TextInput` accepts `autofocus` with Svelte's exact semantics, in
  both input and textarea modes.
- [ ] React `TextInput` exposes `focus()` through `useImperativeHandle`,
  following the `MenuSurface` precedent, focusing the underlying control.
- [ ] `text-input.md` documents `autofocus` as a web-only prop and `focus()` as
  an imperative method, both marked as excluded from `TextInputSpec`.
- [ ] Usage docs updated for `text-input` only; no other entry in
  `component-docs.ts` modified.
- [ ] Tests cover every case in step 7.
- [ ] No Rust, adapter, spec, Svelte component, or baseline changed.
- [ ] All commands in step 8 exit 0.
- [ ] Batch log records commands, exit states, and the handle pattern used.

## Stop Conditions

- `docs:spec-drift` goes red after documenting the web-only prop.
- Converting React `TextInput` to expose a handle forces a behaviour change in
  another prop.
- The Svelte reference has a bug that React cannot faithfully match.
- `TextInputSpec` appears to need either feature.
- A visual baseline would need refreshing.

Stop with exact paths, line numbers, commands, and the smallest unresolved
question.
