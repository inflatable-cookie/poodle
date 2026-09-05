# NP-1 A1 divergences

These are executed NP-1 paired projections that did not match. They are not
receipts and do not move the ledger; the diffs preserve the repair boundary.

- `app-header.gpui.json` / `app-header.a1-diff.json`: the Svelte header has
  implicit `banner` role; `poodle-node` has no Banner role, so the native
  projection is role-less.
- `split-view.gpui.json` / `split-view.a1-diff.json`: the native separator has
  no host-owned current value, and its collapse toggle uses generic `Collapse`
  while Svelte names it `Collapse primary`.

Both were produced by `effigy regressions:native` from the shared scenarios and
remain outside this tranche's repair boundary.
