# g12.018 Public Overlay Geometry Observation

Status: complete (2026-08-01)
Owner: Poodle core
Depends on: contract 002; Popover and Menu component contracts
External requester: Longhorn g01.014 pre-Card 095 admission

## Why

Poodle portals anchored surfaces to the theme root and correctly keeps their
elements private. A desktop host with an independently layered native viewport
still needs explicit geometry to decide whether one registered product overlay
intersects that viewport.

Nucleus currently queries private Popover/Menu selectors below the component's
old DOM ancestor. The live surfaces have already moved to the theme root, so
the lookup cannot reach them. Moving that selector into Longhorn would preserve
neither encapsulation nor behavior. Hiding every native viewport for every open
overlay would also regress the product policy.

Longhorn records the consumer boundary in
`../../../../longhorn/docs/architecture/poodle-overlay-geometry-boundary.md`.
Poodle
owns the public API and its web-renderer parity.

## Contract

- Export plain `OverlayViewportRect`, `OverlaySurfaceGeometry`, and
  `OverlaySurfaceGeometryChange` types.
- `Popover` and `Menu` expose optional `onSurfaceGeometryChange` callbacks in
  Svelte and React.
- Changes carry copied CSS-viewport numbers, an opaque per-mount surface id,
  resolved placement, and visibility. They expose no element, DOMRect,
  selector, portal target, host runtime, or consumer policy.
- Initial positioning and every bounds, placement, or visibility change emit
  an `upsert`. Teardown emits one `remove` per reported surface.
- Menu relays root and nested submenu surfaces independently.
- Observation is explicit per component. There is no default global registry.

The canonical lifecycle is in `docs/contracts/002-anchored-overlays.md`.

## Scope

1. Add shared public geometry types and copy/equality helpers.
2. Extend Svelte `anchored` and React `AnchoredSurface` observation without
   changing consumers that omit the callback.
3. Forward the callback through built-in Popover and Menu.
4. Observe Svelte nested submenu surfaces that deliberately remain unanchored.
5. Add shared observer tests, a recursive Svelte Menu trace, and matching
   Svelte/React Popover and root-Menu lifecycle tests.
6. Add declaration and private-boundary checks.
7. Pack the Svelte artifact graph needed by Longhorn and prove the callback
   from a clean consumer without sibling source resolution. Build the React
   preview as its produced-consumer check.

## Non-Goals

- Longhorn, Nucleus, Browser, Tauri, or native-content policy in Poodle
- a global overlay registry or provider
- public surface elements, refs, selectors, ids, or portal internals
- changing placement, dismissal, focus, styling, or open-state behavior
- Nucleus migration or registry publication
- native-renderer CSS viewport callbacks

## Acceptance

- contract changes precede implementation
- Svelte and React public payloads are semantically identical
- initial snapshots arrive after final portal positioning
- scroll, window resize, anchor resize, and surface resize update bounds
- placement-only deduplication does not suppress a moved rectangle
- anchor-hidden and zero-area surfaces report `visible: false`
- close and destruction remove each reported id exactly once
- one Svelte Menu trace distinguishes root and nested surfaces by opaque id
- callback omission produces no observation work or behavior change
- public declarations contain no `HTMLElement`, `DOMRect`, selector, or host
  runtime type in the geometry payload
- packed Svelte artifacts work in a clean consumer; the React preview builds

## Validation

- `effigy test:components`
- `effigy check:svelte`
- `effigy docs:contract-drift`
- focused React component tests
- exact packed-artifact clean-install proof
- `effigy ci:web`
- `git diff --check`

## Stop Conditions

- observation requires exposing the surface element or portal DOM
- a global subscription is required to make one component observable
- the callback cannot represent nested Menu surfaces without generated public
  DOM identity
- Svelte and React need different public semantics
- placement, focus, dismissal, or unrelated overlay behavior changes

## Next Task

Return the public contract and packed artifact evidence to Longhorn g01.014,
then stop at Poodle's planning gate.

## Result

- `@inflatable-cookie/poodle-headless` now owns immutable geometry payloads, equality, copied
  viewport rectangles, and opt-in observation lifecycle.
- Svelte `anchored` and React `AnchoredSurface` report initial positioning,
  movement, placement, hidden/zero-area visibility, and teardown.
- Built-in Popover and Menu expose `onSurfaceGeometryChange`; Svelte recursively
  relays nested submenu surfaces under independent opaque ids.
- Callback omission attaches no geometry observers or listeners.
- Shared observer, Svelte recursive Menu, React root lifecycle, declaration,
  and packed-consumer tests pass.
- Poodle contains no Longhorn, Nucleus, Browser, Tauri, selector, element, or
  host-policy type in the public geometry payload.

## Artifact Evidence

Artifact set:
`ed9d800843a5d008a812a29000cbe2fcd3d619ea53e231627a1f253449c4d41d`

Evidence:
`.artifacts/svelte-pack-install-M8KH8d/evidence.json`

| Package | SHA-256 |
| --- | --- |
| `@inflatable-cookie/poodle-headless@0.1.0` | `1e1e79d83230f01387b0a213c51a448b5f2fa4ec4d3db0ec00bf50c9c343aa31` |
| `@inflatable-cookie/poodle-styles@0.1.0` | `6c2eebebd784f8ea9fa0081dc6167b64b3ee44d360f772306dc53813c969c0df` |
| `@inflatable-cookie/poodle-svelte-tokens@0.1.0` | `59630dfacfcd802b221dfb6368a38b8f7f4217129a5f8482e1d2983648b8c175` |
| `@inflatable-cookie/poodle-icons-lucide@0.1.0` | `88df4087c5cb2403b8da308cc59ad392d94be0fc16d81a72fa556e1f24cb8e70` |
| `@inflatable-cookie/poodle-svelte@0.1.0` | `9dc95956d5f5133eff6d457dd2e8efd64dab61a76e159b4ba3f05eeacf5e9ddb` |
