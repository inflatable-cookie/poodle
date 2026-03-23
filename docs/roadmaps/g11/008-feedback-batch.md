# g11.008 Feedback Batch

Status: planned
Owner: Flint Core
Depends on: contract audit

## Components

progress, meter, skeleton, status_indicator, rating, badge, pill, callout,
banner, eyebrow, code, time_ago, status_bar

## Structural Issues

- [ ] `badge` — Rust spec `badge.rs` exists but **no contract markdown** `badge.md`.
      Determine if badge has been renamed or merged into another component in the
      contract audit. If removed, delete the Rust spec and GPUI component.
- [ ] `banner` — Rust spec `banner.rs` exists but **no contract markdown** `banner.md`.
      Same as badge — determine if renamed or removed.
- [ ] `callout` — contract is `callout.md` but Rust spec file is named `call_out.rs`.
      Rename Rust spec to `callout.rs` or verify the mapping is intentional.
- [ ] `status_bar` — contract exists (`status-bar.md`) but **no Rust spec**.
      GPUI has `status_bar.rs`. Need to create `StatusBarSpec` or verify the
      GPUI component uses `ShellStatusBarSpec` from composites.

## Per-Component Compliance

- [ ] progress — audit against `docs/contracts/foundation/progress.md`
- [ ] meter — audit against `docs/contracts/foundation/meter.md`
- [ ] skeleton — audit against `docs/contracts/foundation/skeleton.md`
- [ ] status_indicator — audit against `docs/contracts/foundation/status-indicator.md`
- [ ] rating — audit against `docs/contracts/foundation/rating.md`
- [ ] badge — audit against contract (if exists after audit)
- [ ] pill — audit against `docs/contracts/foundation/pill.md`
- [ ] callout — audit against `docs/contracts/foundation/callout.md`
- [ ] banner — audit against contract (if exists after audit)
- [ ] eyebrow — audit against `docs/contracts/foundation/eyebrow.md`
- [ ] code — audit against `docs/contracts/foundation/code.md`
- [ ] time_ago — audit against `docs/contracts/foundation/time-ago.md`
- [ ] status_bar — audit against `docs/contracts/foundation/status-bar.md`
