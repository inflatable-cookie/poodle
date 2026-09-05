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

The eight overlay rows were repaired for structure. DetailItem and ToastHost
reach an empty diff and hold receipts; their stores are deleted. Six rows keep
a store because their residual cause is outside this card's fixed boundary.
Each directory holds `<row>.a1-diff.json`, `<row>.gpui.json`, `svelte.json`,
and `attributes.json` from the same executed run.

Reproduce any row with
`POODLE_NUCLEUS_RECEIPT_DIR=$PWD/target/nucleus-receipts cargo test
--manifest-path packages/gpui/preview/Cargo.toml --test headless_regressions
<row>_a1`.

### Initial overlay focus (`g16.119` class, not owned by `g16.118`)

The overlay structure now matches; what remains is where focus lands when the
overlay opens. `poodle-node` has no autofocus channel and GPUI focus routing
is `g16.119`'s owned path, so no repair was applied here. `g16.119` as written
owns five other rows, so these four need a lane decision.

| Row | Node | Attribute | GPUI | Svelte |
| --- | --- | --- | --- | --- |
| Dialog | 1, `dialog` "Delete file" | `focused` | `false` | `true` |
| Popover | 2, `dialog` "Quick settings" | `focused` | `false` | `true` |
| ConfirmAction | 2, `alertdialog` "Delete workspace?" | `focused` | `false` | `true` |
| MessageCenter | 1, `dialog` "Notifications" | `focused` | `false` | `true` |

### ModelPicker

Structure matches (dialog, radiogroup, radios, `expanded` true). The residual
is focus only, same class as above.

| Node | Attribute | GPUI | Svelte |
| --- | --- | --- | --- |
| 0, `button` "Model: Atlas" | `focused` | `true` | `false` |
| 3, `radio` "Atlas Balanced model" | `focus_order` | `null` | `1` |
| 3, `radio` "Atlas Balanced model" | `focused` | `false` | `true` |

### CommandPalette — blocked on vocabulary

The dialog, heading, close button, status, listbox and group lists now agree.
Fourteen entries remain from three causes; the first is a card stop condition
(`g16.118` owns exactly two new roles, `Heading` and `Banner`).

| Cause | Attribute | GPUI | Svelte | Owner |
| --- | --- | --- | --- | --- |
| The contract's `TextInput type="search"` has no `poodle-node` role | node 3 `role` | `textbox` | `searchbox` | needs a third role; escalated |
| TextInput projects its placeholder as the value | node 3 `value_text` | `"Search commands, panels, and actions"` | `null` | TextInput, out of this card's owned paths |
| `ActionDiscoveryPanel` rows have no inner interactive card node; Svelte nests a `button` named by the item title inside each `option` | nodes 8–13 `role`, `name`, `focus_order` | option/list only | option then button, each button a tab stop | `ActionDiscoveryPanel`, out of this card's owned paths |
| Initial focus | node 3 `focused` | `false` | `true` | same class as above |
