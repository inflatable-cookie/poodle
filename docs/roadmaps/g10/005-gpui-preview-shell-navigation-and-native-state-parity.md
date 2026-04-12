# g10.005 GPUI Preview Shell, Navigation, And Native State Parity

Status: ready
Owner: Poodle core
Depends on: g10.004
Updated: 2026-04-12

## Context

`g10.004` unified the component package and preview IA on the Svelte side.
Recent GPUI parity work removed a large amount of stale shell drift, but the
latest audit still shows three direct shell-level gaps:

- the GPUI catalogue sidebar is still bespoke instead of using `SidebarNav`
- component selection and search continuity are still in-memory only
- GPUI still exposes shell framing that does not match the current Svelte
  preview shell closely enough

This milestone closes the visible preview-shell gap before deeper component-page
and specimen work continues.

## Governing Refs

- `docs/specs/002-component-contract-template-and-parity-rules.md`
- `docs/specs/019-advanced-catalog-accessibility-focus-keyboard-and-state-rules.md`
- `docs/specs/020-docs-site-example-and-component-discoverability-rules.md`
- `docs/specs/058-cross-runtime-parity-report-delta-register-and-acceptance-harness-expansion.md`
- `docs/contracts/components/sidebar-nav.md`

## Goals

- replace bespoke GPUI catalogue navigation with the real `SidebarNav` shell
- add native continuity for active section, search, selected component, and
  other review-critical preview state without pretending GPUI has browser routes
- trim or demote remaining GPUI-only shell chrome so the visible preview reads
  like the current Svelte shell first
- leave GPUI preview state reproducible enough for audit and regression review

## Non-Goals

- generated usage docs on component pages
- another broad specimen-depth sweep
- Jetstream implementation work

## Execution Plan

### Batch 5.1 - SidebarNav Adoption

- [ ] replace the custom grouped component sidebar with the shared `SidebarNav`
      component
- [ ] preserve the current unified registry grouping, filtering, and count
      behavior inside that shell
- [ ] confirm keyboard/focus behavior remains coherent after the shell swap

### Batch 5.2 - Native State Continuity

- [ ] promote the current launch-command/replay affordance into the canonical
      native continuity mechanism
- [ ] preserve active section, search query, selected component, token panel,
      token query, and demo screen in one reproducible launch state
- [ ] make the continuity mechanism visible enough for operators without
      turning the preview into an internal tooling dashboard

### Batch 5.3 - Shell Trim And Validation

- [ ] remove or demote remaining GPUI-only shell framing that overstates parity
- [ ] keep the hidden demo surface clearly marked as a separate contract target
- [ ] validate with `cargo check --manifest-path packages/gpui/preview/Cargo.toml`
      and `git diff --check`

## Exit Criteria

- GPUI catalogue navigation uses `SidebarNav`
- GPUI preview state can be relaunched or reproduced natively for review
- visible shell drift against the current Svelte preview is materially reduced
- no new fake browser-route story is introduced in GPUI

## Next Task

Execute Batch 5.1 in `g10.005`: replace the bespoke GPUI component sidebar with
the real `SidebarNav` shell and keep unified search/group behavior intact.
