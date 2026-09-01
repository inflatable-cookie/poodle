# g16.043 — In-Place Toast Lifecycle Research

Status: research-complete — PR #133; same-id pending-to-settled updates remain
consumer-owned, no promise helper or lifecycle field
Opened: 2026-09-01
Depends on: merged `g16.034` at `369a24f8c`; current ToastHost and ToastStack
contracts and motion lifecycle
Governing refs: `../../architecture/012-semantic-motion-policy.md`,
`../../contracts/components/toast-host.md`,
`../../contracts/components/toast-stack.md`
Intake: DesEngs candidate 7, merged in PR #126
Source lead: [Sonner promise toast](https://sonner.emilkowal.ski/toast#promise)

## Goal

Research an additive lifecycle in which one toast identity may remain pending
and later settle in place. Preserve host-owned operation state, existing toast
semantics, focus, announcements, and expiry. Do not import Sonner's imperative
promise helper as the contract.

This card authorizes research only. `pending | settled`, sticky pending state,
and same-id resolution are hypotheses.

## Questions

- Is lifecycle a ToastHost item field, a host update convention, or a separate
  progress/status composition?
- Which pending, success, error, cancellation, retry, progress, and expiry
  meanings are genuinely reusable?
- When copy or tone changes on one id, what is announced and what must not be
  reannounced?
- How do action focus, dismissal, replacement, removal, and auto-dismiss clocks
  behave across settlement?
- What do reduced and frozen motion modes change visually without changing the
  semantic lifecycle?

## Required Evidence

- Inspect Sonner's promise-toast behavior from pinned primary source and record
  lifecycle, identity, timing, and licensing.
- Audit current ToastHost/ToastStack stores, focus rules, live regions, timers,
  motion pilot behavior, and at least two real async consumers.
- Cover progress churn, duplicate completion, cancellation, host removal,
  action replacement, focused toasts, error persistence, and unmount.
- Map one result across Svelte, React, shared Rust, and GPUI.

## Deliverable And Promotion Gate

Write `docs/research/value-tracks/in-place-toast-lifecycle.md` with an
extend/compose/consumer-owned/reject recommendation and state machine sketch.
Promotion requires accepted `g16.034` behavior and operator acceptance of
identity, announcement, focus, and timer ownership.

## Writable Scope

The dossier only, plus `PAPERCUTS.md` for new execution friction. Do not edit
architecture, contracts, source, packages, roadmaps, triage, or consumers.

## Validation

Run `effigy docs:lint` and `git diff --check`.
