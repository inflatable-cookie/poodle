# Plan

Updated: 2026-09-26

The goal is real GPUI parity ([vision](knowledge/vision.md#direction)). Every
portable component constructs in GPUI; far fewer have mounted functional proof
([census](evidence/gpui/gpui-functionality-census.md)). Repair work comes in
bounded tranches grouped by dependency and interaction substrate, never by
arbitrary component count. A specimen route or test name never marks a
component complete.

## Now

1. **Choose the next product frontier** (lane `next-frontier`) — Q-001.
   Nothing is queued until the operator picks. The recommendation is item 1
   under Next.

## Next

1. **GPUI selection-navigation repair tranche** (lane
   `gpui-selection-repair`) — Button, Checkbox, Radio, ToggleGroup and
   Accordion. These are basic controls with census refusals and heavy consumer
   use; a small tranche proves the repair loop before larger substrates.
2. **Remaining mounted-behaviour tranches** (lane `gpui-mounted-tranches`) —
   compiled from the census's missing-capability groups, bounded by substrate
   (overlay, text input, drag, and so on). Known gaps include IconButton and
   Collapsible triggers with no focus patch, and IconButton `expanded` and
   `controls` never reaching `Node.a11y`.
3. **GPUI keyboard-origin focus** (lane `gpui-keyboard-focus`) — native focus
   treatment must follow the keyboard-origin rule in
   [working rules](knowledge/contracts/working-rules.md#focus-visibility); the
   census names the affected focus-bearing rows.
4. **Sweep removed records for rulings** (lane `ruling-sweep`) — the lean cut
   removed roadmaps, logs and handoffs. Operator rulings buried there that no
   knowledge file states should be promoted when found. Git history has them.
5. **Retire historical generation baselines** (lane `baseline-retirement`) —
   `packages/g03-closeout.json`, the `g04.00x` GPUI baselines and similar JSON
   records still validated by `docs:lint`, and the archived specs they cite.
   Decide which still guard a live surface; delete the rest with their checks.
6. **Next Nucleus evidence repin** (lane `pinned-source-paths`) — Nucleus
   receipts pin `packages/{gpui/preview,gpui/adapter,render,contracts}` and
   `packages/gpui/preview/Cargo.lock` byte-for-byte, so these changes land
   together with one repin:
   - Rust comments that still cite pre-cut paths (`docs/architecture/`,
     `docs/specs/`, removed roadmap and log records); then drop their `allow`
     entries in [retired.toml](knowledge/retired.toml);
   - `rustls` 0.23.45 in the preview lock (RUSTSEC-2026-0285);
   - A1 probes lost in later merges and A1 receipts with no emitting selector;
   - whether `SOURCE_PATHS` should pin whole crates or only runtime source.
7. **Red and unrun checks** (lane `check-health`) — checks that exist but are
   red or never run on `main`: two `poodle-render` unit tests, `check:react`
   and the `packages/core` strict type-check, `drift:roles`, `rustfmt` on
   `packages/render`, plus a single fresh-checkout validation command for
   Queue. A red check that nobody runs hides real regressions.
8. **Small web defects** (lane `web-defects`) — CodeEditor's active line uses
   an undefined `--poodle-color-surface-hover` token, React Button/TextInput
   miss contract-listed web-native props, Svelte `TextInput` doesn't export
   `focus()`, and React `SplitView` lacks the contract's `divider` prop.

## Not now

- **GPUI visual expansion** — waits for a stable functional tranche and
  operator-approved background-safe capture
  ([capture research](knowledge/research/gpui-offscreen-capture-feasibility.md)).
- **A2 platform accessibility** — held until `gpui-apple` publishes a buildable
  crate and a live non-activating accessibility-tree proof exists
  ([triage](triage/20260905-111233-gpui-unofficial-adoption-gates.md)).
- **Nucleus V2/M2 and switch packet** — Nucleus-owned journeys plus lab V2 and
  A2; external to Poodle.
- **Native CodeEditor and rich text** — named future admissions; the web
  admissions earn no native parity credit.
- **Jetstream admission** — separate held programme
  ([triage](triage/20260902-000959-jetstream-admission-hold.md)).
- **Web-pair composite extraction** — operator checkpoint when a shared
  composite is next touched or a second React consumer arrives.
- **Contributor design-guidance pilot** — operator-gated on named reviewers,
  approvals and run custody.
