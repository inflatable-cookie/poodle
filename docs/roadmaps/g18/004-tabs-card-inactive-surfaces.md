# 004 — Tabs card inactive surfaces

Status: ready — operator approved concurrent dispatch with g18.003 review
Owner: Poodle Tabs active cohort
Created: 2026-09-10
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/tabs.md`,
`../../architecture/001-poodle-system-shape.md`,
`../../specs/070-compiled-web-distribution-contract.md`
Depends on: none; `g18.003` is an approved concurrent sibling

## Outcome

Make every item in the `card` Tabs variant read as a card across Svelte, React,
shared Rust composition, and GPUI. Inactive cards use the semantic surface
background with no border. Selected cards replace that base with the existing
`activeFill` treatment: dimmed accent for `tint`, full accent with inverse text
for `solid`, and the base surface for `none`.

Do not add a new prop, change Tabs behaviour, or start a release.

## Ready-State Rubric

- [x] The operator confirmed every card item should retain a visible card shape.
- [x] The operator selected fill-only inactive cards; no inactive border.
- [x] Existing `activeFill` already owns tint/solid/none selection strength.
- [x] Web and native currently share the same missing inactive-fill behaviour.
- [x] Active-cohort scope, package limits, negative cases, and stop gates are explicit.
- [x] The g18.003 PR is in independent review and its rich-text/package paths
  are distinct from the Tabs implementation paths.
- [x] The operator explicitly approved concurrent queue dispatch with g18.003.

## Decisions

- Reuse `color.background.surface`; do not introduce a new semantic token.
- Add one appearance-recipe hook for the base card-item fill so downstream
  theming can tune it without overriding component selectors.
- Apply the base fill to the item wrapper, which encloses the tab and optional
  close button. Do not paint only the inner tab button.
- `activeFill` suppresses or replaces selection emphasis, not the card
  variant's base identity. Under `none`, selected card background equals the
  inactive surface and text or `activeEdge` carries selection.
- Disabled, pinned, reorderable, closable, vertical, full-width, overflow,
  drag-source, drop-target, focus, and panel behaviour remain unchanged.

## Dispatch manifest

- **State:** ready for independent dispatch alongside g18.003 review; keep
  rich-text/package-manifest paths reserved to g18.003 and let Queue serialize
  integration/closeout against current main
- **Completion:** one open non-draft PR at a clean pushed head with exact-head
  independent review; never merge
- **Owned mutable paths:** Tabs contract; shared Tabs styles and focused tests;
  Svelte and React Tabs shells/tests only where needed; Tabs Rust spec/render
  mapping and focused headless/GPUI tests; Tabs-scoped generated component
  docs/evidence; required current receipt source-commit repin; one execution log
- **Reserved closeout surfaces:** g18 README, generation index, dispatch
  projection, task status/evidence, release records
- **Worker:** cross-runtime component worker comfortable with CSS recipe hooks,
  Rust node styling, GPUI interpretation, and packed web validation
- **Excluded:** new public props or tokens; borders on inactive cards; changes
  to pill/block; interaction or state-machine changes; visual capture/windowed
  selectors; unrelated GPUI gap repair; release/tag/publish; workflow changes
- **Escalation:** Chatterbox for any public API, token, variant, interaction,
  receipt semantics, or scope change

## Work

1. Plant focused web and Rust assertions that every inactive card item has the
   semantic surface fill and no border, including closable and disabled cases.
2. Add the base card-item recipe hook and apply it to the shared item wrapper;
   preserve tint, solid, outline, underline, focus, hover, and drag precedence.
3. Apply the same base surface in shared Rust composition and prove GPUI
   receives it on the item that encloses label, count, and close affordances.
4. Cover `activeFill="none"`: card retains the base surface while pill and
   block keep their current unfilled idle state.
5. Refresh generated docs/evidence and perform only the receipt source-commit
   repin required by the existing ledger gate. Do not create new visual claims.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Every card is visible | inactive item background is transparent/absent | paired web computed-style and Rust node assertions for active plus inactive rows |
| Fill encloses the whole card | label button is filled but close sits outside it | closable fixture asserts the item wrapper owns the fill |
| Inactive means surface, not selection | inactive uses an accent mix or gains an outline | exact semantic surface value and zero inactive border in web/Rust |
| Tint and solid still win | base surface overrides the selected style | `tint` and `solid` selected/inactive pairs in both rendering paths |
| None removes only selection fill | selected card becomes transparent, or block gains a surface | cross-variant `none` assertions: card surface, pill/block unfilled |
| Interaction stays unchanged | pointer, keyboard, close, reorder, focus, or drag state changes | existing focused Tabs behaviour suites stay green |
| Recipe remains bounded | consumer must override selectors or hook affects pill/block | recipe-hook assertion scoped to card items only |
| GPUI receives the contract | Rust is fixed but mounted adapter drops the background | headless GPUI/node projection assertion on selected and inactive card items |
| No unsupported visual claim | stale receipts are silently treated as new capture proof | ledger passes through source repin only and records no new capture evidence |

## Stop conditions

- Stop if the correction requires a new public prop, token, or variant.
- Stop if the fill cannot live on the item wrapper across web and native.
- Stop if selection precedence would require changing `activeFill` semantics
  outside the clarified base-versus-selection distinction.
- Stop before a windowed capture, release, unrelated Tabs redesign, or broader
  GPUI repair.

## Evidence

Pending execution.

## Next task

After merge and accepted review, return to Chatterbox to recheck and seek the
explicit release go for `g18.006`. No successor auto-starts.
