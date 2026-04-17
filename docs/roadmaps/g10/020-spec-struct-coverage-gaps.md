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

- [ ] TextInput spec: add `source` and `show_clear_button` fields + builder methods
- [ ] Select spec: add `id` and `name` fields + builder methods
- [ ] RadioGroup spec: add `name` field + builder method
- [ ] Dialog spec: reconcile `kind` vs `role`; add `role` field
- [ ] Orphan spec file review

## Next task

Start with RadioGroup (smallest change), then Select, TextInput, Dialog in that
order.
