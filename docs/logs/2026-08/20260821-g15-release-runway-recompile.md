# g15 Release Runway Recompile

Date: 2026-08-21
Planning base: `ac1155733ca05ab1af7cdc7d82a4e7144ae5d5b9`
Posture: strict release-first; `g15.041` worker in flight

## Outcome

The remaining v0.2.0 path is no longer hidden inside two broad cards.

- Stepper's inert GPUI selection/re-run gap has ready owner `g15.042`.
- UiPresentationProvider's false native passthrough has planned owner
  `g15.043`, held behind an orchestrator architecture decision.
- Visual conformance parent `g15.012` is exact children `g15.044`–`g15.047`:
  offscreen GPUI feasibility, adoption, bounded fixture inventory, then the
  first renderer-aware comparison.
- Release preparation is exact children `g15.048`–`g15.050`: packed 175-name
  reachability, truthful automation, then one pinned v0.2.0 candidate.
- `g15.013` is now only the final operator tag/publication gate.

`g15.042` and `g15.044` are independent ready lanes and may run beside the
current Popover worker. `g15.033` remains behind `g15.041`/`g15.032`; numeric
review order is not bypassed.

## Longhorn/Tauri Conformance-Lab Assessment

The proposed app is useful, but only after separating control from capture and
authority.

Evidence inspected:

- Longhorn `docs/contracts/022-agent-app-control.md` and
  `crates/longhorn-{,tauri-}agent-control/README.md`;
- Poodle's GPUI 0.2.2 test platform and native visual capture;
- current upstream GPUI source already present in the local cargo checkout;
- Tauri's official static-frontend, multiwebview, and sidecar documentation.

What is already true:

- Longhorn can semantically drive opted-in Tauri webviews and compose fresh
  screenshots while the app is unfocused, occluded, or minimized.
- Svelte and React can therefore live in separate labelled webviews as
  precompiled static assets. Bun builds them; it does not need to ship in the
  application.
- Tauri can bundle a GPUI executable as a sidecar.

What is not solved:

- GPUI is genuinely native and cannot render inside a webview.
- Longhorn contract 022 explicitly excludes genuinely native content from its
  screenshot result; no native provider ships.
- Poodle's pinned GPUI 0.2.2 test window has no raster readback. Its headless
  platform proves geometry and interaction only. The current pixel gate opens
  a real window and invokes macOS `screencapture`.
- Newer upstream GPUI source contains a headless renderer and offscreen image
  APIs, but Poodle has not proved or costed that migration.

Decision: do not block v0.2.0 on building the full Tauri lab. First prove GPUI
offscreen pixels in `g15.044`, adopt them in `g15.045`, and land the first
bounded comparator. The future lab may then compose Svelte/React webviews plus
a bundled GPUI sidecar that returns pixels over local IPC. It remains internal
tooling and never becomes Poodle package, component, or behavior authority.

The unresolved full-app choices are retained in
`docs/triage/20260821-165500-longhorn-conformance-lab.md`.

## Release Risks Now Explicit

- `g15.043`: native presentation cascade architecture is unresolved.
- `g15.044`: no proved headless GPUI pixel path at the current pin.
- `g15.049`: native pre-tag workflow points at a deleted GPUI crate, and
  `effigy release gates` is vacuously green with zero configured gates.
- `g15.050`: manifests remain 0.1.0 and no 0.2.0 release note exists.

No release blocker is waived. Jetstream stays program-deferred.

## Changed Files

- `docs/roadmaps/g15/012-visual-conformance-lane.md`
- `docs/roadmaps/g15/013-v020-release-certification.md`
- `docs/roadmaps/g15/033-review-composition-forms-data-media.md`
- `docs/roadmaps/g15/042-...md` through `050-...md`
- `docs/roadmaps/g15/{README,release-gap-register}.md`
- `docs/roadmaps/{README,generation-index}.md`
- `docs/triage/20260821-165500-longhorn-conformance-lab.md`
- this log

No component, package, workflow, lockfile, release artifact, or sibling
repository changed.

## Validation

- `effigy docs:check` — pass
- `git diff --check` — pass
