# 044 Two Preview-Shell Defects `b035` Measured

Status: ready
Milestone: side-quest (preview shell, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-044-preview-shell-defects`
Depends on: none
Governing refs: `PAPERCUTS.md` (2026-08-12, both entries),
`docs/logs/2026-08/12-g13-035-shell-scene-rust-authoring-and-web.md`

## Goal

`b035` found two preview-shell defects while doing something else, measured
both live, and correctly left them alone as out of scope. Fix them.

Both are already diagnosed down to the line. This card is small on purpose —
do not widen it.

## Fixed By Ruling (do not re-decide)

### R1 — `hasMounted` is not reactive, so SHELL-08 never persists.

`packages/svelte/preview/src/App.svelte` gates its URL-persistence `$effect` on
`hasMounted`, a plain `let` in a runes-mode component. A plain `let` is not
reactive, so the effect never re-runs and `theme` / `density` / `controlSize`
are never written back to the query string.

Measured by `b035`: clicking density or size updates the top-bar pills and
`data-theme`, but `location.href` keeps the pre-change query string, with no
console error. **React's equivalent `useEffect` does persist** — it reaches
`?theme=forest&density=default&controlSize=lg` after the same clicks — so the
two shells disagree on `SHELL-08`.

React is the correct behaviour here. Make the Svelte side match: either
`hasMounted` becomes `$state`, or the guard goes and the write runs on mount.
Pick one and say which in the log.

### R2 — Both catalogue grids ignore the filtered list they are handed.

`packages/svelte/preview/src/pages/CatalogueLanding.svelte` and its React
mirror render `componentsByTag()` — every component — and use the `components`
prop only for the count line.

Measured by `b035`: with a search query active, the sidebar filters correctly
and the header reads "2 components" while the grid still renders ~164 cards, in
**both** previews. The search axis works and the data arrives; the grid throws
it away.

Derive the groups from the `components` prop — filter `componentsByTag()`'s
items by the passed set — in both files. The count line and the grid must agree.

### R3 — Preview only. Nothing shared.

Neither defect is in a shipped component. Do not touch
`packages/core/src/styles`, `packages/{svelte,react}/components`, `poodle-ir`,
`poodle-codegen` or `poodle-render`. Another worker holds `packages/codegen`
and `packages/render`.

`packages/svelte/preview/src/generated/**` and
`packages/{svelte,react}/components/src/generated/**` are emitted artifacts —
do not hand-edit them, and do not let a fix depend on changing one.

## Required Tests

- After a theme/density/size change, the Svelte preview's `location.href`
  carries the new values — measured in a browser, matching React's behaviour.
- With a query active, the catalogue grid's card count equals the header count,
  in both previews. Assert the number, not merely that filtering "works".
- No console errors in either preview.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Measure in a browser. Both defects were found that way and neither is visible
  from the source alone.
- Run `effigy check:svelte`, `docs:lint`, `ci:web`.
- Verify every governing-ref path resolves before relying on it.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-044-preview-shell-defects`. Do not merge.
- `PAPERCUTS.md` is shared with another worker: append or mark resolved in
  place, never reflow a neighbouring entry.

## Writable Paths

- `packages/svelte/preview/src/App.svelte`
- `packages/svelte/preview/src/pages/CatalogueLanding.svelte`
- `packages/react/preview/src/gallery/CatalogueLanding.tsx`
- `packages/{svelte,react}/preview/src/**` (only if a fix genuinely requires a
  sibling file — say which and why)
- `docs/logs/2026-08/<DD>-g13-044-preview-shell-defects.md`
- `PAPERCUTS.md` (mark both entries resolved)

## Steps

1. Baseline: `effigy ci:web`, `git diff --check`. Green.
2. Reproduce both in a browser. Record the before-state.
3. Fix R1; verify `location.href` updates.
4. Fix R2 in both runtimes; verify the counts agree.
5. Mark both `PAPERCUTS.md` entries resolved.
6. Validate:
   ```sh
   effigy check:svelte
   effigy test:components
   effigy docs:lint
   effigy ci:web
   git diff --check
   ```

## Acceptance Criteria

- [ ] Svelte's URL persistence matches React's, shown in a browser.
- [ ] Grid count equals header count with a query active, both previews.
- [ ] Both papercut entries marked resolved.
- [ ] All step-6 commands exit 0; no baseline refreshed.

## Stop Conditions

- Making `hasMounted` reactive causes the effect to loop.
- The catalogue grid cannot read the filtered set without a prop change that
  reaches a shipped component.

Stop with exact paths, commands, and the smallest unresolved question.
