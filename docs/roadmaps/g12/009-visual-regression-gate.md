# g12.009 Visual Regression Gate

Status: complete (2026-07-25) — waves 1-4 done; wave 5 (committed baselines) remains optional
Owner: Poodle core
Depends on: `g12.008` (both previews route identically at
`#components/<slug>`), `g12.001` (single shared stylesheet)

## Why

The existing web gates check structure, not pixels:

- `test:parity` diffs the `poodle-*` anatomy classes each framework emits
- `test:a11y` runs axe over every Svelte component
- `docs:contract-drift` diffs contract prop surfaces against Svelte

None of them can see a component render at the wrong size. The ListCard
`data-size` bug (g12.008) proves the hole: the anchor root resolved its
size from the `chrome` role while the div root used `control`, so identical
props rendered at different heights depending on `href`. Both frameworks
emitted identical classes and passed axe. It was found by eye.

Because both previews now serve the same 131 slugs from the same
stylesheet, a cross-framework pixel diff is close to free evidence: same
browser, same CSS, same tokens, so any real difference is a bug in one
shell. That makes it a stronger gate than a committed-baseline suite and it
needs no stored images.

## Scope

Two diff modes, in this order:

1. **Cross-framework diff (the gate).** Same slug, same axis, Svelte vs
   React. Self-verifying — no baseline artifacts to store or churn.
2. **Baseline regression (phase 2, opt-in).** Committed PNGs for a small
   axis-sample tier, to catch unintended drift over time in the shared CSS
   and token layers. Deliberately narrow: full-sweep baselines for 131
   slugs would add hundreds of PNGs to the repo and generate review noise
   on every legitimate style change.

Non-goals: GPUI/Jetstream visual capture (native previews are not
headlessly verifiable here — see `parity-campaign-state`), and any
screenshot review UI.

## Axis Coverage

Full cross-product is 12 themes x 5 sizes x 2 densities x contrast — far
too large. Two tiers instead:

- **Sweep tier** — all 131 slugs at `eclipse` + `iceberg`, density
  `comfortable`, size `md`. 262 captures per framework.
- **Axis tier** — ~15 representative components (Button, IconButton,
  TextInput, Select, Checkbox, Switch, Tabs, Pill, ListCard, DataTable,
  Menu, Dialog, Toolbar, Card, Slider) across all 5 size stops, both
  densities, and the high-contrast axis. This is where size/density bugs
  of the ListCard class actually surface.

Slug list comes from the preview's own registry, not a hand-maintained
array, so new components join the sweep automatically.

## Determinism

Screenshot gates fail on noise unless the page is pinned. Required before
any capture:

- inject a stylesheet zeroing `transition`/`animation` duration and delay
  (Button transitions its background over ~400ms — a known probe gotcha)
- `await document.fonts.ready`, and assert the same font stack resolved in
  both apps
- freeze time via Playwright's clock API — `TimeAgo`,
  `format-display-date`, Calendar/DatePicker "today" highlighting, and
  LogList timestamps are all wall-clock dependent
- disable caret blink; blur focus before capture unless the specimen is a
  focus-state case
- mask or defer the inherently non-deterministic surfaces: Spinner,
  Skeleton, Progress (indeterminate), VideoPlayer, AudioPlayer,
  MediaThumbnail, PageLoading

Anything masked must be listed in the run output, not silently skipped —
a gate that quietly covers less than it claims is worse than no gate.

## Implementation Shape

- New root workspace `test/visual/` (sibling to `test/parity`,
  `test/a11y`), Playwright-driven. Not a vitest project — it needs a real
  browser and two running dev servers, which the happy-dom projects do not.
- `@playwright/test` + chromium headless shell as a devDependency, plus a
  documented install step. Prior runtime verification used a scratchpad
  install; this makes it a first-class dependency.
- Harness boots both vite previews on fixed ports (Svelte 4174, React
  4180), navigates each to `#components/<slug>` with theme/size/density/
  contrast driven through the existing URL sync the preview shells already
  support, and captures the specimen root only — not the app chrome.
- Diff via `pixelmatch`, per-slug threshold, failures written to
  `test/visual/out/<slug>-<axis>-{svelte,react,diff}.png` (gitignored).
- Known-divergence allowlist as a checked-in TS map with a required reason
  per entry, so accepted deltas are documented, not just suppressed.

## CI Placement

- New effigy selectors: `test:visual` (axis tier — the gate),
  `test:visual-smoke`, `test:visual-sweep`, and `visual:report`
  (capture + write diffs without failing). `ci:visual` wraps the gate.
- Workflow: `.github/workflows/ci-visual.yml`, nightly behind the same
  cheap "did the default branch move?" guard as `ci-native`, plus
  `workflow_dispatch` with a tier input. Diff images upload as an artifact
  on failure.
- Stays out of `ci:web` — it needs a browser download and two preview
  builds, which `ci:web` deliberately excludes. Lands in a new `ci:visual`
  selector, run locally and on the scheduled workflow, mirroring how
  `ci:native` handles the native crates.
- Add `test/visual/out/` to gitignore.

## Waves

1. Harness + determinism controls; prove it on the 15 axis-tier components
   at one theme. Establishes the capture and pinning story. **Done —
   see "Wave 1 result" below.**
2. Axis tier at all size/density/contrast combinations. This is the wave
   that would have caught the ListCard bug. **Done — 180/180 pairs green
   (15 components x 12 axes), zero failures, zero allowlist entries.**
3. Sweep tier across all 131 slugs, both themes. Triage the first run —
   expect genuine shell divergences, since nothing has diffed pixels
   before. Every one is either a fix or an allowlist entry with a reason.
   **Done — see "Wave 3 result" below. 256/256 green on the confirming
   re-run.**
4. Effigy selectors, gitignore, scheduled workflow, docs. **Done:**
   `test:visual` / `test:visual-smoke` / `test:visual-sweep` /
   `visual:report` / `ci:visual` in `tasks/effigy.tasks.toml`,
   `.github/workflows/ci-visual.yml` (nightly, guarded, artifacts on
   failure), `test/visual/README.md`. `out/` was already gitignored.
5. (Optional, phase 2) Committed baselines for the axis tier only, with a
   documented `--update-baselines` flow.

## Acceptance

- `test:visual` runs headlessly from a clean checkout after a documented
  install step, with no scratchpad scripts
- all 131 slugs captured in both frameworks; masked/skipped surfaces
  reported explicitly in the run summary
- cross-framework diffs are green or allowlisted with a written reason
- at least one real divergence found and fixed during wave 3 triage, or an
  explicit statement in this file that the sweep came back clean
- two consecutive runs on unchanged code produce zero diffs (no flake)

## Wave 1 Result (2026-07-25)

Harness lives in `test/visual/`: `config.ts` (tiers, axes, skip list),
`server.ts` (boots or reuses both vite previews), `capture.ts` (pinning +
capture), `run.ts` (diff + summary), `probe.ts` (triage helper that prints
one selector's box and computed styles side by side). Deps: `playwright`,
`pixelmatch`, `pngjs`.

Smoke tier (15 components, eclipse/compact/md): **15/15 green, identical
across two consecutive runs.**

Determinism controls that turned out to be load-bearing — each one was
found by a false failure, not by theory:

- **`clock.setFixedTime`, not `clock.install`.** `install()` pauses the
  clock, which starves timer-driven work; React's render then landed
  differently between runs (a 50px phantom diff on TextInput).
- **Hide the page hero and trailing sections.** Preview chrome text can put
  the specimen's origin on a fractional pixel; every glyph below it then
  rasterises half a pixel off and the whole image reads as changed. This
  alone was 9 of the first 15 failures.
- **Snap the specimen to an integer y** after that, for the same reason.
- **Unwind scrolling ancestors and grow the viewport to fit.** Both
  previews scroll inside the app shell, not the document, so an element
  screenshot of a tall specimen captured blank space where the clipped
  content should have been — which read as "React is missing the toggle
  row" when React was rendering it fine.
- **Settle before capture**: hold until the specimen's height stops
  changing, then require two byte-identical screenshots.
- **Capture sequentially, recycle pages every 20 captures, retry once on a
  fresh page.** Driving both previews at once starved heavy specimens
  (ListCard, DataTable) into render timeouts, and a page degrades after a
  few dozen SPA loads.
- Deterministic `Math.random` seed via init script; transitions,
  animations and caret blink zeroed.
- **Server health checks must not be trigger-happy.** The first version
  restarted a preview after one 1s-timeout probe. Under browser-driven
  load a slow response is not a dead server, and each restart threw away
  vite's warm module graph, which made the next captures slower — a
  self-sustaining false positive that thrashed a whole axis run. Now: 5s
  probe, a second 10s confirmation probe, then restart.

Real divergences found and fixed (all Svelte-authoritative):

1. **React Tabs `display: contents` wrapper** — `Tabs.tsx` wrapped each
   item in `<span style={{display:"contents"}}>` for keying. The span has
   no box but still counts for CSS structural selectors, so
   `.poodle-tabs__item:first-child` / `:last-child` matched *every* item:
   vertical strip tabs got the end padding on all four tabs and rendered
   16px too tall. Replaced with keyed `<Fragment>` in both Tabs lists and
   in `RelationPicker`'s breadcrumb map — the only other site of the
   pattern in the package.
2. **ListCard specimen stack gap** — Svelte scopes `0.25rem`, the React
   gallery's shared rule is `0.5rem` (+4px per row, 64px over the page).
3. **ListCard specimen `p` margin** — Svelte's specimen scopes
   `p { margin: 0 }`; React inherited the UA margin (+32px). Fixed
   locally, not in `gallery.css`: other specimens legitimately keep the UA
   margin, and a global reset broke Pill instead.
4. **Chevron spacing in `titleContent`** — Svelte templates keep the
   newline whitespace between elements, JSX drops it, so React's
   breadcrumb chevrons rendered flush. Explicit `{" "}` restores parity.

Findings 2–4 are preview-harness divergences rather than library bugs, but
they are still real: the two galleries are supposed to be the same page.
The allowlist is still empty — nothing so far has needed suppressing.

## Wave 2 Result (2026-07-25)

Axis tier: **180/180 pairs green** — the 15 axis-tier components across all
five size stops in both densities, plus the two contrast extremes
(`eclipse` at 0.9, `iceberg` at 0.1). Zero failures, zero allowlist
entries, no flake. With the four wave-1 fixes in, the size and density
ladders agree between the two shells everywhere the gate can see.

## Wave 3 Result (2026-07-25)

Sweep tier: 128 specimen slugs x 2 themes = 512 captures. The first full
pass found divergences in 10 components; after the fixes below, the
confirming re-run is **256/256 green — zero failures, one allowlist entry,
seven documented skips.**

Four root-cause classes, none of which the structural gates could see:

1. **Svelte scoped CSS vs React global CSS (the big one).** A Svelte
   specimen can tighten a shared helper class for itself alone; React's
   gallery CSS is global, so those overrides silently did not apply.
   Diffing every Svelte specimen's scoped rules against the React defaults
   found **39 mismatches across 31 components** — only a handful had
   surfaced as failures so far. Fixed centrally: `data-component` on the
   React page root plus a generated per-specimen override block in
   `gallery.css`. This closed split-button, drawer and collapse-toggle at
   once.
2. **Inline styles cannot express pseudo-classes.** Three separate
   findings (ListCard's `p { margin: 0 }`, the stack gap, and
   dock-region's `.static-panel:last-child { border-right: 0 }`) all came
   from porting a scoped Svelte rule to a React `CSSProperties` object.
   The `data-component` block is the right home for these.
3. **JSX drops the newline whitespace Svelte templates keep.** Hit
   meta-bar/meta-item (specimen) and `LogList` (the component itself —
   "project" ran flush into the quoted resource name). Fixed with explicit
   `{" "}`.
4. **Real component and data bugs.** Svelte's `MediaBrowsePanel` always
   passed a children snippet to `MediaThumbnail` — present but rendering
   nothing when there was no `thumbnailUrl` — which suppressed the
   contract-required placeholder icon. `ThemeSelectSpecimen` still used
   the pre-rename `"dark"` theme id. `AgentChatInput` measured its
   textarea before the web font applied and never re-measured, leaving it
   short of `minRows` (fixed in both frameworks; React only avoided it by
   winning a race).

Cross-cutting lesson: a `.poodle-meta-bar .poodle-code--inline` rule tied
on specificity (0-2-0) with `.poodle-code--inline[data-size="sm"]`, so the
winner came down to stylesheet **import order** — which differs between the
two apps. That one was a genuine consumer-facing bug in `@inflatable-cookie/poodle-styles`,
not a preview artifact, and no amount of DOM-level checking would have
caught it.

`dock-region` failed in `iceberg` but not `eclipse`: the stray border sat
close enough to the dark background to fall under the diff threshold.
Keeping both themes in the sweep is what exposed it.

Allowlist (1): `scroll-shell`, sub-pixel text antialiasing inside the
composited scroll viewport — reason recorded in `test/visual/allowlist.ts`.
Skips (7): Spinner, Skeleton, indeterminate Progress, PageLoading,
VideoPlayer, AudioPlayer, MediaThumbnail — listed in every run summary.
