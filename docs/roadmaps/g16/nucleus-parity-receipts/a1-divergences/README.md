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
