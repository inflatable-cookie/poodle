# g16.118 — A1 Overlay Structure Projection

Status: implementation complete — pending independent exact-head review
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

- **Vocabulary (this card owns it):** add `NodeRole::Heading` (uses the
  existing `level` field), `NodeRole::Banner`, and `NodeRole::SearchBox`
  (the Select/CommandPalette search editor; the TextInput contract's
  `type="search"` maps to it) to `poodle-node`; map all three in the GPUI
  backend (record only) and in the Jetstream AccessKit projection so the
  quarantined adapter still compiles. Decision 2026-09-05: a role the
  contract names is never a scope violation; anything beyond these three,
  stop and report.
- **Initial overlay focus is not this card.** Dialog, Popover,
  ConfirmAction, MessageCenter, and ModelPicker also diverge on where focus
  lands when the overlay opens. That is focus routing and belongs to
  `g16.119`. This card lands the structure; for those rows the divergence
  store is reduced to the focus-only remainder and no receipt is emitted;
  `g16.119` emits them.
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
  paired runner. A row with an empty diff gets its receipt and its store
  deleted. A row whose only remaining diff entries are `focused`/`focus_order`
  on overlay open keeps a reduced store naming `g16.119`. Repin and re-emit
  the cohort at the final head; regenerate the ledger.

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

## Rulings And Outcome (2026-09-05)

Both new roles landed: `NodeRole::Heading` and `NodeRole::Banner`, mapped in
the GPUI backend record path (`packages/gpui/node-backend/src/a11y.rs`, total
match, paint is a no-op) and in the Jetstream AccessKit projection.

Rulings taken against the executed snapshots, which outrank the card's prose
table where they disagree:

- **Dialog gets no heading node.** `dialog.svelte.json` projects backdrop
  button, `dialog`, close button — the `.dialog__title` element carries no
  role, and the surface's `aria-labelledby` resolves to `-1` there too. The
  native surface now matches exactly: the title keeps its id, the surface
  carries both `labelled_by` and the computed name. ConfirmAction inherits the
  same shape, and its Svelte snapshot has no heading either.
- **Heading is used where Svelte projects one**: the CommandPalette `<h3>` and
  the MessageCenter header `<h2>`. `Banner` is used for the MessageCenter
  header landmark.
- **A `-1` relationship target is a defect only when Svelte resolves it.**
  Where the reference itself projects `-1` (Dialog and ConfirmAction
  `labelled_by`, CommandPalette `described_by`), matching it is the parity
  result.
- **DetailItem's description moves inside its info Popover**, as
  `DetailItem.svelte` has it. The description text no longer paints inline;
  the trigger does. Mounted geometry assertions were moved to the new
  `info` / `info-trigger` parts.
- **CommandPalette does not compose Dialog on the web.** Its overlay is an
  `aria-hidden` div, so the shared native backdrop's dismiss node is demoted
  in `command_palette.rs` rather than projected.
- **ConfirmAction keeps no synthesised default trigger while open.** Svelte
  renders one, but the native backdrop is absolute inside its wrapper rather
  than the window, so adding a sibling trigger shrinks the overlay to the
  trigger row (`confirm_action_composition_…` proves it). Recorded as an
  out-of-scope divergence; a caller-supplied trigger is still preserved.

Receipts: **DetailItem** and **ToastHost** are empty-diff and hold A1
receipts; their divergence stores are deleted. Six rows keep a refreshed
store — Dialog, Popover, ConfirmAction, MessageCenter and ModelPicker are
structurally aligned and diverge only on initial overlay focus, which the card
routes to `g16.119`; CommandPalette is blocked on a third role
(`searchbox`), which the card's fixed boundary forbids. Details and exact
attributes: `nucleus-parity-receipts/a1-divergences/README.md`.

Two `poodle-render` unit tests fail identically on `origin/main`
(`context::tests::the_provider_adds_no_wrapper_node_layout_or_accessibility_entry`,
`segmented_control::tests::icon_only_without_an_icon_keeps_the_visible_label`);
both assert a button's `a11y.label` is `None`. Recorded in `PAPERCUTS.md`, not
repaired here.

## Escalations For Chatterbox

1. `g16.119` owns focus for Menu, AgentQuestion, AgentTranscript, RadioGroup
   and SegmentedControl. Initial overlay focus for Dialog, Popover,
   ConfirmAction, MessageCenter and ModelPicker has no lane. Five rows cannot
   reach an empty diff until one exists.
2. CommandPalette needs `NodeRole::SearchBox`. The contract already names
   `TextInput type="search"`; the role is the only missing piece, plus the
   TextInput placeholder-as-`value_text` projection and the
   `ActionDiscoveryPanel` inner card button, both outside this card's owned
   paths.

