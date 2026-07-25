# Cross-framework visual gate

Pixel-diffs the Svelte and React previews against each other at the same
specimen slug and the same display axis. Roadmap: `docs/roadmaps/g12/009`.

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
effigy visual:report       # sweep without failing the process
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
| `server.ts` | boots/reuses/restarts the two vite previews |
| `capture.ts` | determinism pinning + specimen capture |
| `run.ts` | drives the matrix, diffs, writes the summary |
| `probe.ts` | side-by-side measurement helper for triage |
| `allowlist.ts` | accepted deltas — each needs a written reason |

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
when its anchor scrolls out of the pane. Roadmap: `docs/roadmaps/g12/011`.
