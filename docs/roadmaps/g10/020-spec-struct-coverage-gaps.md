# g10.020 Spec Struct Coverage Gaps

Status: queued
Owner: Poodle core
Depends on: g10.019
Updated: 2026-04-17

## Purpose

Audit found gaps between component contracts and the corresponding Rust spec
structs in `packages/contracts/components/src/`. Missing props mean
cross-platform implementations (GPUI, Jetstream) cannot support contract-
required features even when the underlying component can render them.

---

## Known gaps

**TextInput** (`packages/contracts/components/src/text_input.rs`)
- Missing: `source: Option<String>` — slug mode auto-generation source field
- Missing: `show_clear_button: bool` — search mode clear button visibility
- Note: async validation props (`validate`, `validationDebounce`, etc.) are
  callback-based; handle separately if/when a cross-platform validation model
  is defined

**Select** (`packages/contracts/components/src/select.rs`)
- Missing: `id: Option<String>` — form integration and label association
- Missing: `name: Option<String>` — form field submission in custom mode

**RadioGroup** (`packages/contracts/components/src/radio_group.rs`)
- Missing: `name: Option<String>` — form field group name for submission

**Dialog** (`packages/contracts/components/src/dialog.rs`)
- Current spec uses `kind: DialogKind` which the contract marks as deprecated
- Contract primary prop is `role: "dialog" | "alertdialog"`
- Resolution: add `role` field (or rename `kind`), map deprecated `kind` to
  `role` with a note; decide whether to keep `kind` as an alias

---

## Orphan spec files (no contract)

These spec files exist but have no matching contract; confirm each is
intentional:
- `badge.rs`, `banner.rs`, `call_out.rs`
- `composite_types.rs`, `shell_status_bar.rs`, `time_field.rs`

---

## Execution checklist

- [x] RadioGroup spec: added `name: Option<String>` + `with_name()` builder
- [x] Select spec: added `id: Option<String>`, `name: Option<String>` + builders
- [x] TextInput spec: added `source: Option<String>` (slug mode) + `show_clear_button: bool` (default true) + builders
- [x] Dialog spec: renamed `kind` → `role`; `with_kind()` kept as `#[deprecated]` alias; `is_alert_dialog()` updated; GPUI dialog component updated to match
- [x] Orphan spec file review: badge (Jetstream-only, no web contract), banner (unconnected early spec — candidate for deletion), call_out (maps to callout.md — naming mismatch only), composite_types (shared types, not a component), shell_status_bar (Jetstream workstation variant, distinct from StatusBar), time_field (maps to time-input.md — naming mismatch only); all annotated with module-level doc comments

## Outcome

All items complete. Both crates compile clean.
One follow-up candidate: `banner.rs` / `BannerSpec` has no contract and no
known renderer usage — likely superseded by `RemediationBannerSpec` or
`CallOutSpec`. Confirm and delete in a future cleanup pass.
