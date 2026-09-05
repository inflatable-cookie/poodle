# g16.118 — A1 overlay structure projection

Status: implementation complete — pending independent exact-head review
Date: 2026-09-05
Card: `docs/roadmaps/g16/118-a1-overlay-structure-projection.md`
Base: `origin/main` at `ef483d029` (dispatch manifest revision 7)
Branch: `worker/g16-118-a1-overlay-structure-projection`

## Outcome

The bounded vocabulary addition landed: `NodeRole::Heading` and
`NodeRole::Banner`, mapped exhaustively in the GPUI backend record path
(new `packages/gpui/node-backend/src/a11y.rs`; paint stays a no-op on
crates.io GPUI) and in the Jetstream AccessKit projection. No third role was
added.

Eight overlay rows were repaired against their executed Svelte snapshots:

- **Dialog** — the `dialog`/`alertdialog` role moves off the backdrop onto
  the surface panel, which is named from and `labelled_by` its title; the
  backdrop gains its own "Dismiss dialog backdrop" node and owns backdrop
  dismissal. No heading node: `dialog.svelte.json` has none, and its
  `aria-labelledby` resolves to `-1` on the web too.
- **ConfirmAction** — inherits that shape; `AlertDialog` no longer overrides
  the role on the way out.
- **Popover / DetailItem** — the surface carries a stable scoped id so
  `controls` resolves to a real index. DetailItem's description moves inside
  its info Popover, as `DetailItem.svelte` has it, and the composition now
  projects exactly one trigger button.
- **CommandPalette** — heading role on the title, `described_by` on the
  surface rather than the overlay root, group `<ul>`s projected as lists, and
  the shared Dialog backdrop node demoted because the web palette's overlay is
  an `aria-hidden` div.
- **ModelPicker** — trigger `expanded` and `controls` follow open state;
  radios carry their Svelte accessible name.
- **MessageCenter** — `Banner` landmark on the header, `Heading` on the
  title, `Button` on message rows.
- **ToastHost** — every toast row is a `listitem` (danger no longer projects
  `alert`); the dismiss button leads the row as it does in the DOM.

Three A1 scenarios carried scenario ids no manifest row had
(`nucleus-toast-host`, `nucleus-command-palette`, `nucleus-message-center`);
they now use `nucleus.attention.*` and their Svelte snapshots were
regenerated (id and hash only, no node changed). The stale `#[ignore]` on the
Select A1 test was removed — `g16.117` aligned that row, so it now runs and
emits with the rest of the cohort.

## Receipts

Empty-diff A1 receipts, divergence stores deleted:

- `detailitem--nucleus-settings-detail-item--a1.json`
- `toasthost--nucleus-attention-toast-host--a1.json`

Six rows keep a refreshed store under
`docs/roadmaps/g16/nucleus-parity-receipts/a1-divergences/`, each with
`<row>.a1-diff.json`, `<row>.gpui.json`, `svelte.json` and
`attributes.json` from one executed run:

| Row | Entries | Residual cause |
| --- | --- | --- |
| Dialog | 1 | overlay surface not focused on open |
| Popover | 1 | overlay surface not focused on open |
| ConfirmAction | 1 | overlay surface not focused on open |
| MessageCenter | 1 | overlay surface not focused on open |
| ModelPicker | 3 | trigger keeps focus; first radio is not a tab stop |
| CommandPalette | 14 | `searchbox` role, TextInput placeholder as `value_text`, missing ActionDiscoveryPanel card button, initial focus |

CommandPalette fell to 34 → 14 entries. Its remaining causes are all outside
the card's fixed boundary; the exact attributes are in the divergence README.

## Escalations

1. **Initial overlay focus has no lane.** `g16.119` owns focus for Menu,
   AgentQuestion, AgentTranscript, RadioGroup and SegmentedControl. Dialog,
   Popover, ConfirmAction, MessageCenter and ModelPicker are structurally
   aligned and cannot reach an empty diff until a lane owns overlay autofocus.
   `poodle-node` has no autofocus channel; GPUI focus routing is
   `g16.119`'s owned path, so nothing was changed here.
2. **CommandPalette needs `NodeRole::SearchBox`.** The contract already
   specifies `TextInput type="search"`. The card forbids a third role, so the
   row was left short by design.
3. **Three A1 receipts have no live selector.** `callout`,
   `editable-label` and `text-input` A1 receipts are committed but no GPUI
   test emits them; only their `source_commit` could be moved in this repin.
   Their emitting tests need restoring before the next cohort re-emit.
4. **Two `poodle-render` unit tests are red on `origin/main`** (see
   `PAPERCUTS.md`); not repaired here.

## Validation

- `effigy regressions:native` — 226 passed, 0 ignored
- `cargo test --manifest-path packages/render/Cargo.toml` — 639 passed, the
  two failures that are already red on `origin/main`
- `effigy test:jetstream-adapter` — 163 passed
- `effigy test:nucleus-a11y` — 30 passed
- `effigy check:parity-evidence-ledger` — 176 rows validated
- `effigy test:nucleus-parity-receipts` — 11 pass
- `effigy test:a11y` — 179 passed
- `effigy docs:check` — passed
- `git diff --check` — clean
- `effigy ci:web` — 3740 passed, 386 files. A first run failed only on the
  known flaky React Tabs controlled-focus case, on a `packages/react` tree
  byte-identical to `origin/main`; recorded in `PAPERCUTS.md`.

The cohort was repinned to `71f9175d99bfbfcaf26446e536db0c191bb79eb8` with lock digest
`c86c2d11c36c9fcf9326bae438ee6acc3bcedacbaf01ac017a298c1bd3c2a34c` and
re-emitted through `effigy regressions:native`; the ledger was regenerated.
No windowed selector was run.

## Review state

The worker pushes one PR and stops. Merge and independent exact-head review
remain orchestrator-owned.
