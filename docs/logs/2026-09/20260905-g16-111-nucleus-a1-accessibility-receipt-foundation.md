# g16.111 — Nucleus A1 Accessibility Receipt Foundation

Status: complete with one reported divergence — awaiting orchestrator review
Date: 2026-09-05
Card: `docs/roadmaps/g16/111-nucleus-a1-accessibility-receipt-foundation.md`
Dispatch: `docs/roadmaps/dispatch.md` revision 7
Base: `d8aea4aea639642f834c95261e63250b27f4dd60` (`origin/main`)
Receipt source pin: `7c2ab2030835fbd268bee04c9d44fbf449ffe404`
Worker branch: `worker/g16.111-nucleus-a1-accessibility-receipt-foundation`

## Outcome

The paired A1 receipt exists once and is proven on two of the three rows.
Switch and Tabs have validated A1 receipts; the ledger moves their "GPUI
accessibility" cell from `manual` to `mounted`. Select executed on both
runtimes and diverged on real semantics; its receipt is not emitted, its
proof is `#[ignore]` with the reason inline, and the diff below is the first
NP-2 repair candidate. No component, contract, or backend behaviour changed.

## What was built

- **Shared scenarios.** `test/nucleus-a11y/scenarios/<row>.json`: web-named
  props, actions targeted by role and accessible name (never a runtime id),
  the contract-declared state list, fixture slot text, and web-only
  exclusions. Both extractors deserialise the file; the Rust side rejects
  unknown props (`deny_unknown_fields`) and both hash the exact bytes.
- **Snapshot shape.** Ordered nodes (document order), each with `role`
  (ARIA string), `name`, `value`, `value_text`, declared `states`,
  `relationships` by index (`-1` for a declared target absent from the
  snapshot), `level`, `orientation`, `focus_order`, and `focused`. Nodes
  without a role and `aria-hidden` subtrees are excluded on both sides.
- **GPUI extractor.** `HeadlessDriver::accessibility_nodes` walks the mounted
  node tree after production dispatch and reads real focus: gpui's focused
  handle is attributed through the backend focus registry. Names come from
  the record (`labelled_by` resolved to the referenced node's label, else
  `label`); there is no name-from-content fallback, so a missing record
  label surfaces as `null`. `HeadlessDriver::focus_traversal` executes
  gpui's real tab traversal and the proof asserts the tracked stops match
  the declared order.
- **Svelte extractor.** `test/nucleus-a11y/extract.ts` uses
  `dom-accessibility-api` (`getRole`, `computeAccessibleName`,
  `isInaccessible`) and ARIA attributes; actions replay as DOM events with
  the browser default of focusing on pointer down applied explicitly.
- **Receipt.** `proof_level: "A1"` plus an `accessibility` block carrying
  the scenario hash, both snapshot paths and hashes (also in
  `artifact_paths`), the exclusions, and the diff. Emitted only from an
  executed run with an empty diff (`POODLE_NUCLEUS_RECEIPT_DIR`). Files are
  `<component>--<scenario>--a1.json`; the validator enforces the stem.
- **Validator.** Recomputes the scenario hash, both artifact hashes, both
  run records, the scenario hash inside each snapshot, and the diff itself.
- **Ledger.** A1 receipts move the component row's "GPUI accessibility"
  cell and the Nucleus table's A1 column; nothing else moves.

## Review oracle

| Invariant | Counterexample run | Result |
| --- | --- | --- |
| Both sides are executed | validator requires the run record in each snapshot and the `production_path_observation` in the receipt; `receipt fails validation without a run record` covered by `scripts/nucleus-parity-receipts.test.ts` | rejected |
| Scenario cannot drift | removed the second Tabs action from `tabs.json`, ran the Tabs proof | Rust rejected: `tabs.svelte.json was produced from a different scenario file (hash mismatch)` |
| Divergence bites | set `selected` to `null` on the Svelte Tabs snapshot's tab "Three", ran the Tabs proof | red with one diff entry (`index 4, field states`) and no receipt emitted |
| Names are computed, not read | Svelte tabpanel name "Three" via `aria-labelledby`; GPUI tabpanel name via `labelled_by` resolution of the record | equal; GPUI has no content fallback |
| Ledger moves only on validated receipts | A1 receipt for an unmanifested component, tampered hashes, non-empty diff, wrong snapshot row | all rejected (`g16.111 Nucleus A1 paired accessibility receipts` tests) |
| M1 untouched | 29 M1 receipts re-emitted at the pinned commit | only `source_commit` differs from the previous receipts; all validate |

## Row results

| Row | Nodes | Result |
| --- | --- | --- |
| Switch | 1 (`switch` "Enabled", checked after activation, focused, focus order 0) | equal; receipt `switch--nucleus-settings-switch--a1.json` |
| Tabs | 6 (`tablist` "Files" horizontal; four `tab`s with selected, controls, roving focus; `tabpanel` labelled by the selected tab, focus order 1) | equal; receipt `tabs--nucleus-navigation-tabs--a1.json` |
| Select | GPUI 5 / Svelte 6 | diverged (below); no receipt; proof ignored |

## Select divergence (repair candidate for `g16.113`, NP-2)

Scenario: options Apple, Banana, Cherry (disabled), default value
`banana`, `ariaLabel` "Fruit", custom (non-native) mode, one pointer
activation of the trigger. Both runtimes opened the listbox. Svelte is the
reference. The positional diff has 16 entries; the semantic causes are:

1. **Trigger role.** GPUI projects `combobox`; the Svelte non-searchable
   trigger is a `<button>` (role `button`) with `aria-expanded`,
   `aria-haspopup`, and `aria-controls`. GPUI's combobox fallback also
   reports `value_text` "Banana" where Svelte reports none.
2. **Indicator button.** Svelte renders a second focusable
   `button` "Close options" / "Open options" next to the trigger; GPUI
   renders the chevron as a role-less icon.
3. **Listbox name.** Svelte's listbox carries `aria-label` "Fruit"; GPUI's
   `ListBox` node has no label.
4. **Option focusability.** Svelte options are `<button role="option">`
   and enter sequential focus (orders 2 and 3); GPUI option rows are
   pointer targets only (`focusable` false).

Reproduce: `cargo test --manifest-path packages/gpui/preview/Cargo.toml --test headless_regressions select_a1 -- --ignored`;
with `POODLE_NUCLEUS_RECEIPT_DIR` set the run writes `select.gpui.json` and
`select.a1-diff.json` beside the receipts. The Svelte snapshot is committed
at `test/nucleus-a11y/snapshots/select.svelte.json`; no GPUI Select snapshot
is committed because the pair does not agree.

## Stop conditions met

- Node record fields: none missing for Switch or Tabs. For Select in custom
  non-searchable mode nothing on the Svelte side needs a field the record
  lacks; `aria-activedescendant` appears only in searchable mode, which this
  scenario does not exercise. The Select gap is a projection divergence, not
  a missing record field.
- Focus order: gpui's traversal is attributable only for nodes the backend
  tracks (`tracks_focus`). The Tabs panel is focusable but untracked, so its
  stop is executed but unattributed; the snapshot's `focus_order` is derived
  from the node record's `focusable`/`tab_index`/`disabled` (the same rule
  the backend maps to gpui) and `focused` is read from the real focused
  handle. No backend change was made.

## Validation

- `effigy regressions:native`: pass, 205 passed, 1 ignored (Select A1);
  emitted 29 M1 + 2 A1 receipts at `7c2ab203…`.
- `effigy test:nucleus-a11y`: pass, 4 tests (three rows plus the roster).
- `bun test scripts/nucleus-parity-receipts.test.ts scripts/parity-evidence-ledger.test.ts`: pass, 17 tests.
- `effigy check:parity-evidence-ledger`: pass, 176 rows.
- `effigy docs:check`: pass.
- `effigy ci:web`: pass on the final head (385 vitest files, 3710 tests; pack-install 22 tests). A first run failed only on the package-install certification scope for the root manifest edit described below; the edit was reverted.
- `git diff --check origin/main...HEAD`: clean.
- Cargo.lock unchanged by the `sha2` dev-dependency (already in the graph).
- No `*-windowed`, native-visual, or capture selector was run.

## Scope notes

- `dom-accessibility-api@0.5.16` is consumed through its declared place in
  the graph (a dependency of `@testing-library/dom`) via a resolve alias in
  the `nucleus-a11y` vitest project. A first attempt added it to the root
  `package.json`; `effigy ci:web` rejected that through the package-install
  certification scope (`test/package-install/scope.ts` forbids any manifest
  change on a worker branch), so the manifest and lockfile are unchanged.
  `vitest.config.ts` and `tasks/effigy.tasks.toml` gained the `nucleus-a11y`
  project and `test:nucleus-a11y` selector; `test:components` (in `ci:web`)
  already runs every vitest project. These are outside the card's owned-path
  list and are the minimum wiring for the new project.
- The manifest `resolution.source_commit` was repinned and all 29 M1 receipts
  re-emitted because the validator's source-match covers the whole preview
  crate (already recorded in `PAPERCUTS.md` by g16.107). After rebasing onto
  `origin/main` (g16.106 had repinned to `93c4ef19…`) the whole cohort was
  re-emitted once more at the rebased head.

## Closeout

Reserved for the coordinator at merge: `docs/roadmaps/g16/README.md`,
`docs/roadmaps/generation-index.md`, and `docs/roadmaps/dispatch.md`.
