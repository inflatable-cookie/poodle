# g12.016 Public Dock Drag Extension And Preview Artifact

Status: complete (2026-07-29)
Owner: Poodle core
Depends on: `docs/contracts/components/dock-region.md`,
`docs/contracts/components/tabs.md`
External requester: Longhorn `g01.007` Card 038

## Why

`DockRegion` owns useful tab and panel drag presentation, but a desktop host
currently has no public way to prepare an asynchronous cross-window session
before the browser's synchronous `dragstart` payload-write window. Loophole
compensates with generated ids, CSS classes, and private MIME knowledge. That
is not a reusable Poodle boundary.

Poodle also declares a preview release posture while its packed Svelte package
still points at `workspace:*` dependencies and `0.0.0` peers. A sibling source
link can hide that mismatch. Longhorn needs one exact artifact set installed
without sibling resolution before it can build its private adapter.

## Contract

- Poodle owns pointer/HTML5 presentation, local reorder, markup, and drop
  affordances.
- `DockRegion.externalDragSource` begins async preparation on primary pointer
  down. A ready result writes the public host payload synchronously through its
  `start` method at native `dragstart`.
- Pending preparation at `dragstart` is aborted with `"not-ready"` and writes
  no external payload.
- Abort reasons plus preparation `cancel`/`end` make supersession,
  cancellation, and completion observable without leaking host state into
  Poodle.
- `DockRegion.externalDropTarget.canDrop` is synchronous and drives the existing
  affordance. `drop` receives only accepted external drops.
- Enabling the external source suppresses Poodle's cross-region panel MIME but
  preserves Tabs' same-region reorder machinery.
- No public type names Longhorn, windows, surfaces, leases, or app policy.

The full API, event order, and race matrix are canonical in
`docs/contracts/components/dock-region.md`.

## Scope

1. Add the typed source/target seam to `@inflatable-cookie/poodle-svelte`.
2. Add Tabs' owning-composite pre-drag callback without changing its local
   reorder contract.
3. Prove ready, pending, superseded, cancelled, ended, local-reorder, external
   eligibility/drop, and accessible-name behavior in mounted tests.
4. Align the Svelte component, token, icon, headless, and style package
   metadata at the existing `0.1.0` preview baseline.
5. Pack all five packages, install them in a clean consumer outside the repo,
   mount `DockRegion`, and record every tarball SHA-256 plus the supported
   Svelte floor.

## Non-Goals

- Poodle depending on Longhorn
- cross-window session policy
- React or native renderer parity for the web-only `DataTransfer` extension
- registry publication
- consumer migration
- redesign of DockRegion, Tabs, or SplitView

## Acceptance

- canonical component contracts authorize the API before implementation
- no unready external payload
- cancellation and end are exactly-once and mutually exclusive
- local same-region reorder remains green
- target eligibility controls `preventDefault`, `dropEffect`, and the drop zone
- public root/type exports include the seam
- packed manifests resolve exact preview versions without `workspace:*`
- clean install has no Vite alias, sibling source dependency, or private DOM
  selector
- exact artifact identity and Svelte floor are logged for Longhorn

## Validation

- `effigy test:components`
- `effigy check:svelte`
- `effigy docs:contract-drift`
- `effigy ci:web`
- exact packed-artifact clean-install fixture
- `git diff --check`

## Result

- `DockRegion` now exposes public, typed external source and target seams while
  retaining Poodle-owned local reorder and presentation.
- Tabs exposes the owning-composite prepare/start/end callbacks needed to begin
  asynchronous work before native `dragstart`.
- Mounted tests cover ready, pending, superseded, cancelled, ended, local
  reorder, external eligibility/drop, and accessible-name behavior.
- All five Svelte-facing packages use the existing exact `0.1.0` preview
  baseline. `@inflatable-cookie/poodle-svelte` supports Svelte `>=5.38.6 <6`.
- `test:svelte-pack-install` packs and installs the exact artifacts in a clean
  consumer without aliases, sibling source resolution, private selectors, or
  private MIME knowledge.
- Poodle contains no Longhorn dependency or host session policy.

## Artifact Evidence

Artifact set:
`39f08c04fa2579ae709db412c28221c04f22b89f09e633cef93764e5d49f8c74`

Evidence:
`.artifacts/g12.016-A698XB/evidence.json`

| Package | SHA-256 |
| --- | --- |
| `@inflatable-cookie/poodle-headless@0.1.0` | `f6132a3fbb44f795bdc7775586e08438321124163678bf3b99ad68958923cfe2` |
| `@inflatable-cookie/poodle-styles@0.1.0` | `9523011c14e00bbd57fe6bce95cf481d35ca5441d990449eddff7733de5bc0f9` |
| `@inflatable-cookie/poodle-svelte-tokens@0.1.0` | `59630dfacfcd802b221dfb6368a38b8f7f4217129a5f8482e1d2983648b8c175` |
| `@inflatable-cookie/poodle-icons-lucide@0.1.0` | `88df4087c5cb2403b8da308cc59ad392d94be0fc16d81a72fa556e1f24cb8e70` |
| `@inflatable-cookie/poodle-svelte@0.1.0` | `d0ab2f25ba31050d8b2dbf903ea90f5714b7f1337635fc5bdafd40d239a0b75a` |

## Next Task

Return the exact artifact identity to Longhorn Card 038, then stop at Poodle's
planning gate. Do not start Longhorn Card 039 automatically.
