# g16.118 — A1 overlay structure projection

Status: implementation complete — pending independent exact-head review
Date: 2026-09-05
Card: `docs/roadmaps/g16/118-a1-overlay-structure-projection.md`
Base: `origin/main` at `ef483d029` (dispatch manifest revision 7)
Branch: `worker/g16-118-a1-overlay-structure-projection`
Recovery input: exact PR #224 head `d286cdd81e37db1b2f41a4c02069c4236c4cfcb6`

Operator ruling: revision 20 adds `NodeRole::SearchBox` beside `Heading` and
`Banner`, finishes the CommandPalette structure, assigns the five remaining
focus-only stores to `g16.119`, and emits no receipts for those stores.

## Outcome

The operator ruling landed: `NodeRole::SearchBox` was added beside
`NodeRole::Heading` and `NodeRole::Banner`, mapped exhaustively in the GPUI
backend record path (new `packages/gpui/node-backend/src/a11y.rs`; paint stays
a no-op on crates.io GPUI), the Jetstream AccessKit projection, and the
mounted A1 projection.

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
- **CommandPalette** — heading role on the title, SearchBox role and explicit
  actual-value projection on the query, `described_by` on the surface rather
  than the overlay root, group `<ul>`s projected as lists, nested action
  buttons inside options, and the shared Dialog backdrop node demoted because
  the web palette's overlay is an `aria-hidden` div.
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
- `commandpalette--nucleus-attention-command-palette--a1.json`
- `toasthost--nucleus-attention-toast-host--a1.json`

Five rows keep a refreshed focus-only store under
`docs/roadmaps/g16/nucleus-parity-receipts/a1-divergences/`, each with
`<row>.a1-diff.json`, `<row>.gpui.json`, `svelte.json` and
`attributes.json` from one executed run:

| Row | Entries | Residual cause |
| --- | --- | --- |
| Dialog | 1 | overlay surface not focused on open |
| Popover | 1 | overlay surface not focused on open |
| ConfirmAction | 1 | overlay surface not focused on open |
| MessageCenter | 1 | overlay surface not focused on open |
| ModelPicker | 2 | trigger and selected-radio initial focus |

## Escalations

1. **`g16.119` owns the remaining focus-only stores.** Dialog, Popover,
   ConfirmAction, MessageCenter and ModelPicker are structurally aligned; the
   native lane does not own overlay autofocus.
2. **Three A1 receipts have no live selector.** `callout`,
   `editable-label` and `text-input` A1 receipts are committed but no GPUI
   test emits them; only their `source_commit` could be moved in this repin.
   Their emitting tests need restoring before the next cohort re-emit.
3. **Two `poodle-render` unit tests are red on `origin/main`** (see
   `PAPERCUTS.md`); not repaired here.

## Validation

- `effigy regressions:native` — 226 passed, 0 ignored
- `cargo test --manifest-path packages/render/Cargo.toml` — 639 passed, the
  two failures that are already red on `origin/main`
- `effigy test:jetstream-adapter` — 163 passed
- `effigy test:nucleus-a11y` — 30 passed
- `effigy test:parity-evidence-ledger` — 6 passed, 0 failed; 176 rows validated
- `effigy test:nucleus-parity-receipts` — 11 pass
- `effigy test:a11y` — 179 passed
- `effigy docs:check` — passed
- `effigy ci:web` — 3740 passed, 386 files
- `git diff --check` — clean before final commit

The cohort was repinned to implementation head
`7ad28f7b9716f22f9a28cc29b6a81b97d4d2e59b` with the existing lock digest
`c86c2d11c36c9fcf9326bae438ee6acc3bcedacbaf01ac017a298c1bd3c2a34c` and
re-emitted through `effigy regressions:native`; the ledger was regenerated and
validated. The initial parallel ledger run timed out once under concurrent docs
build load; the isolated rerun passed. `cargo test --manifest-path
packages/render/Cargo.toml` still has the two pre-existing unrelated failures
listed in `PAPERCUTS.md`.
No windowed selector was run.

## Review state

The worker pushes one PR and stops. Merge and independent exact-head review
remain orchestrator-owned.
