# g16.118 — A1 Overlay Structure Projection

Status: ready
Type: native accessibility repair — `poodle-node` vocabulary, `poodle-render`
composition, backend/Jetstream role mapping; A1 receipts for eight rows
Opened: 2026-09-05
Depends on: merged `g16.111`–`g16.117` (A1 receipts and divergence store)
Governing refs: `../../contracts/003-native-accessibility.md`,
`../../contracts/components/{dialog,popover,confirm-action,detail-item,command-palette,model-picker,message-center,toast-host}.md`
(accessibility sections), `packages/contracts/node/src/lib.rs` (`NodeRole`,
accessibility record), `packages/render/src/*.rs` for the rows,
`nucleus-parity-receipts/a1-divergences/{dialog,popover,confirm-action,detail-item,command-palette,model-picker,message-center,toast-host}/`
Dispatch manifest: `../dispatch.md`

## Rows and recorded causes (Svelte is the reference; the contract decides)

| Row | What GPUI projects | What Svelte projects |
| --- | --- | --- |
| Dialog | `dialog` role on the wrong node; no backdrop control; no title heading | container `dialog` named by its title, a `button` "Dismiss dialog backdrop", first focus inside |
| ConfirmAction | `alertdialog` on the button; names shifted | container `alertdialog` named "Delete workspace?", heading, backdrop button, actions in order |
| Popover | `dialog` on the trigger; `controls` unlinked (`-1`); wrong name | trigger `button` "Settings" controlling a `dialog` "Quick settings" |
| DetailItem | no `dialog` node for the description popover; `controls` unlinked | trigger controls a `dialog` |
| CommandPalette | 26 entries: no heading "Workspace commands", close button and search input order and names shifted, relationships unlinked | heading, close `button`, search `textbox`, list |
| ModelPicker | no `dialog`, no `radiogroup`/`radio` nodes; `expanded` false while open | dialog containing a radiogroup of radios; trigger `expanded` true |
| MessageCenter | `list`/`listitem` where Svelte has `banner`, heading, list, listitems; name "Notifications" missing | banner landmark with heading and list |
| ToastHost | `alert` per toast; dismiss/retry names shifted | `listitem` per toast with named "Dismiss …" and "Retry" buttons |

## Fixed Boundary

- **Vocabulary (one bounded addition, this card owns it):** add
  `NodeRole::Heading` (uses the existing `level` field) and
  `NodeRole::Banner` to `poodle-node`; map both in the GPUI backend (no-op
  paint, record only) and in the Jetstream AccessKit projection so the
  quarantined adapter still compiles. No other vocabulary change; if a row
  needs more, stop and report.
- **Composition:** in `poodle-render`, give every overlay the structure the
  contract and Svelte agree on: a container node with the `dialog` or
  `alertdialog` role named from its title, a heading node for the title, a
  backdrop dismiss `button` node where Svelte has one, `controls` and
  `labelled_by` relationships resolved to real node indices (never `-1`),
  trigger `expanded` reflecting open state, and list/listitem/alert
  structure matching the Svelte DOM for MessageCenter and ToastHost.
  ModelPicker gets its `radiogroup`/`radio` nodes.
- **Svelte:** unchanged unless the contract contradicts it; then the
  contract decides and the card records the ruling before repairing.
- **Proof:** re-run the A1 receipt for each of the eight rows through the
  paired runner; each diff must be empty; delete the row's divergence store;
  repin and re-emit the cohort at the final head; regenerate the ledger.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Vocabulary is minimal | a third new role | reviewer rejects |
| Relationships resolve | any `-1` target left | receipt validation fails |
| Structure matches, not names alone | roles right but order shifted | positional diff non-empty |
| Jetstream still compiles | new role unmapped | `effigy test:jetstream-adapter` red |
| M1 untouched | any M1 receipt changes semantically | cohort re-emit shows only the resolution block moved |

## Validation

`cargo test -p poodle-node -p poodle-render`, `effigy regressions:native`,
`effigy test:jetstream-adapter`, `effigy check:parity-evidence-ledger`,
`effigy docs:check`, `git diff --check origin/main...HEAD`.

## Owned Paths

`packages/contracts/node/src/lib.rs` (two roles), the GPUI backend role
mapping, `packages/jetstream/adapter` role mapping,
`packages/render/src/{dialog,popover,confirm_action,detail_item,command_palette,model_picker,message_center,toast_host}.rs`
and tests, the eight rows' A1 tests, scenarios, receipts and divergence
stores, manifest `resolution`, ledger, execution log, `PAPERCUTS.md` (append).

## Stop Conditions

Stop when a row needs a behaviour change (focus, dismissal) rather than
structure — that belongs to `g16.119` — or when the contract and Svelte
disagree and the ruling is not obvious. Escalation owner: Chatterbox.
