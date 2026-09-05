# A1 divergence records

Executed A1 runs whose paired snapshots did not agree. A record here is not a
receipt and moves no ledger cell; it is the honest output of both extractors
for one shared scenario so the divergence can be repaired by the owning
tranche and re-proven with the receipt unchanged. The validator ignores this
directory; only top-level `*.json` receipts are evidence.

## select (`nucleus.navigation.select`, recorded 2026-09-05 by g16.111)

Scenario: `test/nucleus-a11y/scenarios/select.json` (options Apple, Banana,
Cherry disabled; default value `banana`; `ariaLabel` "Fruit"; custom mode;
one pointer activation of the trigger). Both runtimes opened the listbox.

- `select/select.gpui.json` — the mounted GPUI projection (5 nodes), run
  record `effigy regressions:native` / `HeadlessDriver` /
  `gpui-test-platform-dispatch`, source `7c2ab2030835fbd268bee04c9d44fbf449ffe404`.
- `test/nucleus-a11y/snapshots/select.svelte.json` — the mounted Svelte DOM
  projection (6 nodes), run record `effigy test:nucleus-a11y`.
- `select/select.a1-diff.json` — the positional diff (16 entries).

Semantic causes, Svelte as reference (repair owner: `g16.113`, NP-2):

| # | Attribute | GPUI (`poodle-node` record) | Svelte DOM |
| --- | --- | --- | --- |
| 1 | trigger `role` | `combobox` (`NodeRole::ComboBox` on `select:<scope>:trigger`) | `button` (`<button aria-expanded aria-haspopup="listbox" aria-controls>`) |
| 2 | trigger `value_text` | `"Banana"` (combobox visible-text fallback) | `null` (button role has no value) |
| 3 | indicator | role-less `chevron-down` icon node | `button` "Close options" / "Open options", sequential focus stop |
| 4 | listbox `name` | `null` (no `a11y.label` on the `ListBox` node) | `"Fruit"` (`aria-label` on `role="listbox"`) |
| 5 | option `focus_order` | `null` (option rows are pointer targets, `focusable` false) | sequential stops (`<button role="option">`) |

Reproduce: `POODLE_NUCLEUS_RECEIPT_DIR=$PWD/target/nucleus-receipts cargo test --manifest-path packages/gpui/preview/Cargo.toml --test headless_regressions select_a1 -- --ignored`.

## g16.114 NP-3 rows

Each row directory contains a diff, both runtime snapshots, and exact changed
attributes. NP-4 settings rows use the same evidence contract with named
`<row>.a1-diff.json`, `<row>.gpui.json`, `svelte.json`, and `attributes.json`.

- `agent-chat-input/` — the rendered action has no backend identity, so the
  scenario action cannot be resolved. No renderer projection fix was applied.
- `agent-plan/` — title text is available, but the native node has no contract
  heading role/level. No projection fix was applied.
- `agent-question/` — the label exists but its labelled-by relation is not
  linked; post-action focus is behavior divergence. No behavior repair was
  applied.
- `agent-transcript/` — post-action focus diverges. No behavior repair was
  applied.
- `model-picker/` — combobox semantics and focus order diverge. Select-class
  repair is deferred to `g16.117`; no behavior repair was applied.

## g16.118 rows (recorded 2026-09-05)

The eight overlay rows were repaired for structure. DetailItem, CommandPalette
and ToastHost reach an empty diff and hold receipts; their stores are deleted.
Five rows keep a store because their residual cause is focus ownership assigned
to `g16.119`.
Each directory holds `<row>.a1-diff.json`, `<row>.gpui.json`, `svelte.json`,
and `attributes.json` from the same executed run.

Reproduce any row with
`POODLE_NUCLEUS_RECEIPT_DIR=$PWD/target/nucleus-receipts cargo test
--manifest-path packages/gpui/preview/Cargo.toml --test headless_regressions
<row>_a1`.

### Initial overlay focus (`g16.119` owner)

The overlay structure now matches; what remains is where focus lands when the
overlay opens. `poodle-node` has no autofocus channel and GPUI focus routing
is `g16.119`'s owned path, so no repair was applied here. The operator ruling
assigns all five rows below to `g16.119`.

| Row | Node | Attribute | GPUI | Svelte | Owner |
| --- | --- | --- | --- | --- | --- |
| Dialog | 1, `dialog` "Delete file" | `focused` | `false` | `true` | `g16.119` |
| Popover | 2, `dialog` "Quick settings" | `focused` | `false` | `true` | `g16.119` |
| ConfirmAction | 2, `alertdialog` "Delete workspace?" | `focused` | `false` | `true` | `g16.119` |
| MessageCenter | 1, `dialog` "Notifications" | `focused` | `false` | `true` | `g16.119` |
| ModelPicker | 0, `button` "Model: Atlas" | `focused` | `true` | `false` | `g16.119` |
| ModelPicker | 3, `radio` "Atlas Balanced model" | `focused` | `false` | `true` | `g16.119` |

### CommandPalette — empty-diff receipt

The operator ruling added `NodeRole::SearchBox` beside `Heading` and `Banner`.
The production TextInput now reports SearchBox and only a non-empty actual
value as `value_text`; ActionDiscoveryPanel now nests its interactive button
inside each option. The mounted paired snapshots agree with an empty diff, so
CommandPalette emits an A1 receipt.

Receipt: `commandpalette--nucleus-attention-command-palette--a1.json`.
