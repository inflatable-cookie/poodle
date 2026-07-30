# Public Dock Drag Extension And Preview Artifact

Date: 2026-07-29
Roadmap: `g12.016`
External requester: Longhorn `g01.007` Card 038

## Changed

- Added public typed `DockRegion` external source and target seams.
- Added Tabs prepare/start/end callbacks for asynchronous host preparation
  before the browser's synchronous `dragstart` payload window.
- Preserved Poodle-owned same-region reorder and legacy cross-region behavior
  when no external source is configured.
- Added mounted lifecycle, eligibility, reorder, and accessible-name coverage.
- Aligned the five Svelte-facing package manifests at exact `0.1.0`
  dependencies and Svelte peer range `>=5.38.6 <6`.
- Added a clean pack-install consumer proof with no aliases, sibling source
  resolution, private selectors, or private MIME knowledge.

## Contract Boundary

Poodle owns pointer and HTML5 drag presentation, local reorder, markup, and
drop affordances. The host owns payload preparation, session policy, and
cross-window meaning. Public types do not name Longhorn concepts.

The web-only `DataTransfer` callbacks are an explicit accepted delta. They do
not enter the renderer-neutral Rust `DockRegionSpec`.

## Artifact Evidence

Artifact set:
`39f08c04fa2579ae709db412c28221c04f22b89f09e633cef93764e5d49f8c74`

Evidence:
`.artifacts/g12.016-A698XB/evidence.json`

| Package | Tarball | SHA-256 |
| --- | --- | --- |
| `@poodle/headless@0.1.0` | `.artifacts/g12.016-A698XB/packs/poodle-headless-0.1.0.tgz` | `f6132a3fbb44f795bdc7775586e08438321124163678bf3b99ad68958923cfe2` |
| `@poodle/styles@0.1.0` | `.artifacts/g12.016-A698XB/packs/poodle-styles-0.1.0.tgz` | `9523011c14e00bbd57fe6bce95cf481d35ca5441d990449eddff7733de5bc0f9` |
| `@poodle/svelte-tokens@0.1.0` | `.artifacts/g12.016-A698XB/packs/poodle-svelte-tokens-0.1.0.tgz` | `59630dfacfcd802b221dfb6368a38b8f7f4217129a5f8482e1d2983648b8c175` |
| `@poodle/icons-lucide@0.1.0` | `.artifacts/g12.016-A698XB/packs/poodle-icons-lucide-0.1.0.tgz` | `88df4087c5cb2403b8da308cc59ad392d94be0fc16d81a72fa556e1f24cb8e70` |
| `@poodle/svelte@0.1.0` | `.artifacts/g12.016-A698XB/packs/poodle-svelte-0.1.0.tgz` | `d0ab2f25ba31050d8b2dbf903ea90f5714b7f1337635fc5bdafd40d239a0b75a` |

## Validation

- `effigy test:components` — 21 files, 655 tests passed
- `effigy check:svelte` — 0 errors; 7 pre-existing warnings
- `effigy docs:contract-drift` — 111 checked, 31 skipped
- `effigy docs:check` — passed; preview build retains pre-existing warnings
- `effigy test:svelte-pack-install` — passed at Svelte `5.38.6`
- clean consumer mounted `DockRegion`, wrote an external payload, reordered a
  local tab, and exposed an accessible region name through public imports only

## Current State

`g12.016` is complete. Poodle is strict-paused with no implementation card
ready. Longhorn can consume only the recorded artifact set in its next card.
