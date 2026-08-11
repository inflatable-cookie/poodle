---
title: g13 batch 017 — AppHeader centre region
status: complete
milestone: side-quest (component API, outside the g13 IR lane)
owner: Poodle core
updated: 2026-08-11
tags: [log, g13, AppHeader, centre region, symmetric grid, side-quest]
---

## What this batch did

Executed batch card
`docs/roadmaps/g13/batch-cards/017-app-header-center-region.md` on branch
`thread/g13-017-app-header-center-region`: gave `AppHeader` an optional
`center` region whose **presence** switches the layout — no `layout` prop, no
`columns` prop. With `center`, the grid becomes the symmetric
`minmax(var(--poodle-app-header-side-min, 0), 1fr) auto minmax(var(--poodle-app-header-side-min, 0), 1fr)`
and `actions` + `utility` share a trailing column justified to the end; at
`≤45rem` a centred header reflows to `auto minmax(0, 1fr) auto` (one row,
centre absorbing the free space) while the default header still collapses to
`1fr`. `--poodle-app-header-side-min` is exposed, defaulting to `0`, so a
consumer can set soundcheck's `9rem` collapse guard without hard-coding the
grid. All four runtimes ship it: Svelte, React, `AppHeaderSpec`, and
`poodle-render` (Runtime Parity Authority rule, `docs/contracts/001-working-rules.md`).

The card exists because five Tauri apps are migrating onto `AppHeader` and
four fit: `soundcheck` centres a destination `Tabs` group with
`minmax(9rem, 1fr) auto minmax(9rem, 1fr)` (verified at
`~/Dev/projects/soundcheck/src/App.svelte:809`), and no arrangement of the
existing three regions reproduces it.

## Deliverables (writable paths, plus the two required callers)

- `packages/core/src/styles/app-header.css` — `--poodle-app-header-side-min: 0`
  declared on `.poodle-app-header`; the `.poodle-app-header[data-center]`
  symmetric grid; `__center` added to the shared region block; the
  `__trailing` group (flex, gap `--poodle-app-header-gap`, justify-end); the
  `≤45rem` reflow rule. The default `minmax(0, 1fr) auto auto` rule and the
  default collapse to `1fr` are unchanged.
- `packages/svelte/components/src/AppHeader.svelte` — optional `center` snippet
  prop; `data-center={center ? "" : undefined}`; the centre region; the
  trailing wrapper **only when `center` is present** (default DOM untouched).
  The bindable `element` prop (g13-b014) is intact.
- `packages/react/components/src/AppHeader.tsx` — identical `center` prop,
  same `data-center` gating, same trailing wrapper condition; `ref` forwarding
  intact.
- `packages/contracts/components/src/app_header.rs` — `center: bool` field on
  `AppHeaderSpec` (+ `with_center(bool)`), the declarative presence signal.
- `packages/render/src/app_header.rs` — `center: Option<Node>` render param
  (after `identity`); the presence-driven layout switch keys off the node being
  passed (mirrors the web snippet's presence semantics); centre region between
  identity and the trailing column; actions/utility grouped into a trailing
  **Grow** container (`MainAxisAlignment::End`, gap `grid_gap`) exactly when
  centred, otherwise flat direct children as before. Four new render tests.
- `packages/gpui/preview/src/specimens/app_header.rs` — centred and
  centred-at-narrow-width groups (destination-style centre row).
- `packages/jetstream/preview/src/specimens/app_header.rs` — same two groups.
- `packages/svelte/preview/src/specimens/AppHeaderSpecimen.svelte` +
  `packages/react/preview/src/gallery/specimens/AppHeaderSpecimen.tsx` — same
  two groups with identical labels, using a strip `Tabs` group as the centre
  (mirrors soundcheck's destinations).
- `docs/contracts/components/app-header.md` — anatomy, the presence-is-the-
  signal rationale (and why there is no `layout`/`columns` prop), the centred
  grid, `--poodle-app-header-side-min` (+ its specificity note), the narrow
  reflow, exact CSS tables, the `data-center` attribute, and specimen
  definitions.
- `packages/svelte/preview/src/component-docs.ts` — `center` slot row in the
  `app-header` entry.
- Tests: `packages/svelte/components/test/AppHeader.svelte.test.ts` (+
  `AppHeaderCenterHarness.svelte`) and
  `packages/react/components/test/AppHeader.test.tsx` — both assert the
  byte-identical default region markup (a literal pre-g13-b017 string), the
  default computed grid, the centred markup + symmetric computed grid, and the
  `≤45rem` reflow vs default collapse, with identical expected strings across
  the two runtimes.
- `docs/logs/2026-08/17-g13-017-app-header-center-region.md` — this log.
- `PAPERCUTS.md` — two new, non-duplicate frictions (see Findings).

### Required caller edits (outside the listed writable paths)

The render signature change is compile-breaking for its two direct callers, so
they had to move with it (same treatment g13-014 gave its sanctioned
`contract-spec-drift.ts` edit):

- `packages/gpui/preview/src/node_compat.rs` — `AppHeader` gains a
  `center: Option<Node>` field, a `.with_center(...)` builder, and passes it
  through `into_node()`.
- `packages/jetstream/preview/src/compat.rs` — `js_app_header_with_slots`
  gains the `center: Option<El>` parameter (after `identity`); `js_app_header`
  passes `None`.

## The web shapes used

Svelte (snippet presence is the signal; `data-center` omitted when absent so
the default DOM is byte-identical):

```svelte
{#if center}
  <div class="poodle-app-header__center">{@render center()}</div>
{/if}

{#if center}
  <div class="poodle-app-header__trailing">
    {#if actions}<div class="poodle-app-header__actions">{@render actions()}</div>{/if}
    {#if utility}<div class="poodle-app-header__utility">{@render utility()}</div>{/if}
  </div>
{:else}
  {#if actions}…{/if}
  {#if utility}…{/if}
{/if}
```

React mirrors it exactly (`data-center={center ? "" : undefined}` renders
`data-center=""` in both runtimes when present, omitted when absent). The
trailing wrapper is emitted whenever `center` is present — even with empty
actions/utility — so the third `1fr` track always exists and the centre stays
truly centred (a CSS grid would keep the empty track anyway; the native flex
emulation needs the empty Grow container to match).

## Proving ruling 3 (default byte-identical DOM and computed grid)

- **DOM**: both suites assert `header.innerHTML` equals a literal string of
  the pre-g13-b017 markup (three flat siblings — identity, actions, utility —
  no `__center`, no `__trailing`, no `data-center` attribute). Svelte's
  `<!---->` hydration markers and whitespace-only text nodes are stripped
  before comparison (compiler plumbing, not region markup); the same expected
  strings are used verbatim in the React suite.
- **Computed grid**: vitest stubs CSS imports, so the tests inject the real
  `app-header.css` and assert `getComputedStyle(...).gridTemplateColumns`:
  `minmax(0, 1fr) auto auto` for the default, the symmetric triple for a
  centred header, `1fr` vs `auto minmax(0, 1fr) auto` at `≤45rem`. (happy-dom
  evaluates `@media` rules at stylesheet parse time, so the narrow cases set
  the window width to 600px before injecting — see PAPERCUTS.)
- **Pixel proof**: the visual gate failure set is identical at HEAD and after
  the change (see below), and `app-header` is not failing in either run — the
  without-centre headers render identically in Svelte and React.

## Findings

**The jetstream preview cannot build from a g13 worktree (pre-existing,
g13-013 hit the same wall).** The sibling `jetstream-poodle` crate hard-points
its poodle deps at `../../../poodle/packages/…` (the main checkout), which does
not exist alongside the worktrees, so
`cargo build --manifest-path packages/jetstream/preview/Cargo.toml` fails at
path resolution before reaching any of this card's code. Verified the jetstream
**adapter** crate builds clean, and verified the preview compiles by
temporarily repointing `jetstream-poodle/Cargo.toml` at this worktree (then
reverting the file — `git diff` clean). The gpui preview compiles directly.
Recorded in `PAPERCUTS.md`.

**happy-dom `@media` evaluation is parse-time only.** Changing the window width
after a stylesheet is parsed updates `matchMedia` but not the cascade, so
`getComputedStyle` keeps the pre-resize rules. The tests set the width before
injecting the stylesheet; recorded in `PAPERCUTS.md`.

**Stale dev servers from the g13-016 worktree squatted the visual gate's fixed
ports (4174/4180)** — the exact squatting hazard already logged in `PAPERCUTS`
and by g13-013. The gate would have silently reused them and diffed old code;
killed both as environment cleanup, ran the enumeration, and left the ports
free.

No stop conditions reached: the default header keeps its pixel-identical grid
and DOM without any `layout` prop; the trailing wrapper never appears when
`center` is absent; no visual diff outside `app-header` and none on the
without-centre headers; `poodle-node` expresses the optional region fine
(plain `Node` children).

## Visual enumeration (step 8) — report mode, no `--update`

Gate: `effigy visual:report` (`bun test/visual/run.ts --tier=sweep --report`),
run at HEAD (changes stashed) and again after the change; summaries diffed by
slug/axis/kind.

| run | compared | failing | app-header failures |
|---|---|---|---|
| HEAD | 308 | 54 | 0 |
| after change | 308 | 54 | 0 |

Failure-set delta: **0 new, 0 gone** — no diff appeared on any component other
than `app-header`, and `app-header` itself (which now contains the two new
centred groups on top of the unchanged without-centre groups) renders
pixel-identical Svelte↔React. The 54 pre-existing failures are the gate's
documented baseline debt (unchanged set and details; `scroll-shell` remains the
single allowlisted delta). No baseline file modified; no `--update` passed.

## Validation (step 9)

| Command | Exit state |
|---------|-----------|
| `bun install` | 0 — 234 packages |
| `effigy test:components` | 0 — 44 files / 863 tests (baseline 855; +8 = 4 new per runtime) |
| `effigy test:parity` | 0 — 163 |
| `effigy docs:lint` | 0 — 170 contracts, 43 operator guides, 12 parity targets, … |
| `effigy docs:contract-drift` | 0 — checked 129 |
| `effigy docs:spec-drift` | 0 — checked 113 |
| `cargo test -p poodle-render` | 0 — 156 (baseline 152; +4 new app_header render tests) |
| `cargo build` (gpui preview) | 0 |
| `cargo build` (jetstream adapter) | 0 |
| `cargo build` (jetstream preview, worktree-repointed sibling) | 0 — see Findings |
| `effigy docs:check` | 0 — rewrote `packages/tokens/artifacts/rust/*` and `packages/react/preview/artifacts/component-docs.json` |
| `git checkout -- packages/tokens/artifacts/rust/` | 0 — restored; nothing from that directory committed |
| `git diff --check` | 0 |
| `git status --porcelain` | only the writable paths + the two required caller files + this log |

## Not done

Per batch card and worker rules: no change to the default three-region layout,
DOM, or grid; no `layout` or `columns` prop; no soundcheck or other consumer
migration; no visual baseline refresh; no `poodle-ir` change; no Tabs /
NavigationMenu / `004-shared-control-types.md` files touched (the Tabs
*component* is used by the specimens, never edited); no roadmap/status/dispatch
edits; no merge; no `git add -A` (staged by explicit path).
