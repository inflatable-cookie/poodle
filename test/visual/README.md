# Cross-framework visual gate

Pixel-diffs the Svelte and React previews against each other at the same
specimen slug and the same display axis. Roadmap: task g12.009 (Git history).

Both previews serve the same `#components/<slug>` routes from the same
stylesheet, so the two images should be identical. Any real difference is a
bug in one of the shells — no committed baselines needed.

## Setup

```sh
bun install
bunx playwright install chromium
```

## Running

```sh
effigy test:visual-smoke   # 15 components, 1 axis — fast sanity pass
effigy test:visual         # axis tier: 15 components x 12 size/density/contrast axes
effigy test:visual-sweep   # every specimen slug x 2 themes
effigy visual:report       # sweep, diffed against the committed debt inventory
```

Or directly:

```sh
bun test/visual/run.ts --tier=axis
bun test/visual/run.ts --slug=list-card,tabs --report
```

The run boots both vite previews itself (Svelte 4174, React 4180), reuses
them if they are already listening, and restarts one that dies mid-run.
Failures write `test/visual/out/<slug>-<axis>-{svelte,react,diff}.png` plus
`summary.json` (gitignored).

In `--report` mode every failing pair is classified against the committed
parity-debt inventory (`debt.ts`): a recorded pair reports as known debt;
any other failing pair is a new regression and fails the process. The strict
tiers never read the inventory — a gate failure is always a failure.

## Shared harness

`session.ts` is the one headless-browser helper every multi-capture harness
and agent-driven preview uses. It absorbs native file choosers and dialogs,
recycles a page before vite's client state degrades it, restarts a preview
that dies mid-batch, and puts a hard deadline on every attempt:

```ts
import { captureSession } from "./session";

const session = captureSession({ context });
const capture = await session.run(`${framework} ${slug}`, (page) =>
  captureSpecimen(page, base, slug, axis),
);
if (!capture.ok && (await session.recover(framework))) {
  // a dead preview was restarted; retry once on a young page
}
```

`run` recycles the page on the shared `RECYCLE_AFTER` cadence and rejects
with `PageDeadlineError` when an attempt exceeds `PAGE_DEADLINE_MS`, so a
wedged specimen can never consume the whole run. Recovery is the caller's
decision: only infrastructure failures (dead preview, degraded page) may be
retried — a specimen that cannot settle is evidence, not a flake.

Previews are always booted through `startPreviews()` in `server.ts`: it binds
the configured ports with `--strictPort`, checks the listen table first, and
fails fast naming a squatter rather than trusting the "ready" banner. Do not
reach for `bun run --cwd packages/*/preview dev` from a harness: it takes
whatever port is free and a stale server can shadow it.

The planted cases live in `session.test.ts` (`effigy test:visual-harness`).

## Triage

`probe.ts` prints one selector's box and computed styles side by side in
both previews — the fastest way to turn "16px taller" into a cause:

```sh
bun test/visual/probe.ts tabs '.poodle-tabs__tab' padding-block gap
PROBE_LIMIT=40 bun test/visual/probe.ts list-card '.poodle-specimen-group'
```

Order of suspicion, learned from wave 1:

1. a specimen-harness difference (gallery CSS vs a Svelte scoped style, JSX
   dropping template whitespace)
2. a React shell difference that breaks CSS structural selectors — a
   wrapper element, even `display: contents`, changes `:first-child`
3. an actual component divergence

## Files

| file | role |
| --- | --- |
| `config.ts` | tiers, axes, skip list, ports, capture selector |
| `server.ts` | boots/reuses/restarts the two vite previews on strict ports |
| `session.ts` | shared headless-browser session: recycling, deadlines, chooser/dialog handling, preview restart |
| `capture.ts` | determinism pinning + specimen capture |
| `run.ts` | drives the matrix, diffs, writes the summary |
| `probe.ts` | side-by-side measurement helper for triage |
| `allowlist.ts` | accepted deltas — each needs a written reason |
| `debt.ts` | sweep-tier known-failing pairs — the report diffs against it; each entry names its failure class and reason |
| `fixtures/` | g15.046 Button visual fixture inventory — named cases for the g15.047 comparator, no captures |

## What the gate does not cover

Components whose paint is inherently non-deterministic are skipped and
listed in every run summary: Spinner, Skeleton, indeterminate Progress,
PageLoading, VideoPlayer, AudioPlayer, MediaThumbnail.

## Overlay containment probe

```sh
bun test/visual/overlay-portal-probe.ts
```

A pixel diff cannot see this class of bug — the markup is identical whether or
not an ancestor clips the surface. The probe wraps a specimen in a hostile
ancestor (scrolling + transformed + low stacking context, with a higher-z
sibling beside it), opens each anchored overlay, and asserts it portalled out,
fits the viewport, is the topmost painted element at its own centre, and hides
when its anchor scrolls out of the pane. Roadmap: task g12.011 (Git history).
