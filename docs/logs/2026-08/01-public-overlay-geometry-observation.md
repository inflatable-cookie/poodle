# Public Overlay Geometry Observation

Date: 2026-08-01
Roadmap: `g12.018`
External requester: Longhorn `g01.014` pre-Card 095

## Changed

- Added immutable CSS-viewport geometry payloads to `@inflatable-cookie/poodle-headless`.
- Added opt-in geometry lifecycle to Svelte `anchored` and React
  `AnchoredSurface`.
- Added `onSurfaceGeometryChange` to built-in Popover and Menu.
- Relayed recursive Svelte submenu surfaces under independent opaque ids.
- Added shared observer, Svelte, React, and packed-consumer tests.
- Extended the clean Svelte pack-install proof through public imports only.

## Contract Boundary

Poodle owns private surface geometry and reports copied viewport numbers. A
consumer owns which component instances it subscribes to and what the geometry
means. The payload exposes no surface element, DOMRect, selector, portal target,
Tauri type, native-content concept, or product policy.

Observation is explicit per component. There is no global registry, so an
unrelated Tooltip, Select, or picker cannot silently enter a host policy.

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

## Validation

- `effigy ci:web` — passed; 295 headless and 677 component tests
- `effigy check:svelte` — 0 errors, 0 warnings
- `effigy docs:contract-drift` — 113 checked, 31 skipped
- React preview production build — passed
- `effigy test:svelte-pack-install` — two mounted clean-consumer tests passed
- `git diff --check` — passed

## Current State

`g12.018` is complete. Poodle is strict-paused with no implementation card
ready. Registry publication remains separate external release work.
