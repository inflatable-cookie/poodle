# g16.026 — Drag-And-Drop Cross-Window Bridge And DockRegion

Status: planned — blocked on g16.025 and the DockRegion public migration decision
Depends on: `025-drag-drop-rust-gpui-substrate.md`,
`../../triage/20260828-221415-drag-drop-public-migration-boundary.md`
Governing refs: architecture 011, spec 069, and the DockRegion contract

## Goal

Implement the capability-based host bridge for same-application cross-window
transfer and migrate DockRegion without importing Longhorn, Tauri, Electron,
window topology, or durable layout authority into Poodle.

## Required Boundary

- Host preparation completes before native drag activation and yields only an
  opaque armed receipt/session id.
- Poodle advertises protocol version plus opaque token across the bounded
  DataTransfer adapter; it never stores the authoritative transaction there.
- Targets project host-supplied eligibility, revalidate before commit, and
  report one terminal success/refusal/cancel result.
- The host owns leases, window geometry, target resolution, authorization,
  mutation, rollback, expiry, and recovery.
- Cross-window keyboard movement uses a host target picker. Touch capability is
  advertised only when the host can observe it outside the source window.
- DockRegion preserves within-region reorder, zones, collapse, tab callbacks,
  and current mounted evidence while replacing its global side channel.

Before ready status, resolve the old DockRegion exported controller/types. Use
the approved clean public migration; no aliases or dual session paths.

## Acceptance Criteria

- [ ] Paired TypeScript/Rust bridge contracts carry only opaque authority.
- [ ] A deterministic host simulator proves prepare, moving target geometry,
      stale lease, rejection, commit, cancel, window close, and late completion.
- [ ] Svelte, React, and GPUI DockRegion projections preserve component
      behavior and use the same lifecycle semantics.
- [ ] Headless web multi-context and GPUI host-stub tests take no operator focus.
- [ ] Poodle imports no Longhorn/shell package and owns no window transaction.
- [ ] Existing DockRegion ledger claim remains honest; no unrelated row moves.

## Writable Scope

- focused core/headless bridge types and host simulators;
- bounded native DataTransfer opaque-token adapter;
- DockRegion web, render, GPUI, types, contracts, tests, and specimens;
- old DockRegion external-drag modules/exports only under the operator decision;
- focused headless multi-context/native host-stub evidence;
- this card, migration triage, one log, g16 closeout, and `PAPERCUTS.md`.

Do not edit Longhorn, Loophole, Tauri/Electron packages, application window
policy, file drag-out, package versions, releases, workflows, or siblings.

## Validation

Run bridge/kernel/DockRegion tests, headless Chromium/WebKit multi-context
proof, mounted GPUI regressions, active-cohort drift and ledger checks, web/Rust/
native/docs boards, one final headless `effigy qa`, and diff check. Never run a
windowed/native visual or Jetstream selector.

## Stop Conditions

- Poodle must know window topology, filesystem paths, credentials, or durable
  mutation to complete the bridge.
- Longhorn-owned semantics need to be copied rather than represented by opaque
  host capabilities.
- DockRegion cannot migrate without a compatibility path or public behavior
  break not approved by the operator.
- Proof needs sibling-repository mutation or focus-taking automation.

## Continuation

After merge, promote `g16.027` for inbound files and native file drag-out.
