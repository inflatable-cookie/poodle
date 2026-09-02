# g16.065 — Tabs Native Tooltip Parity

Status: in review
Date: 2026-09-03
Card: `docs/roadmaps/g16/065-tabs-native-tooltip-parity.md`
Handoff: `docs/handoffs/20260902-225241-g16-065-tabs-tooltips.md`
Governing refs: `docs/contracts/001-working-rules.md`,
`docs/contracts/components/tabs.md`
Branch: `fix/g16-065-tabs-native-tooltips`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-065-tabs-native-tooltips-resume`
Planning base: `d0e07ed65` (`origin/main` after g16.066 / PR #171)

## Outcome

`showTooltips` has one public meaning across Svelte, React, the Rust spec,
the renderer, and mounted GPUI. False stays inert after 300ms. True, and
every vertical strip, projects the trimmed tab label onto `Node.tooltip`.
The merged g16.066 backend owns delay and dismiss. No new Node field.

## What landed

- Spec: `TabsSpec::with_shows_tooltips`.
- Renderer: `tab_tooltip_text` from `wire_collection_semantics` (card, pill,
  block). Vertical strips project when the flag is false. Empty labels are
  omitted. Disabled tabs keep the label; the backend does not start a timer.
- Mounted proof: compact/`sm` chrome fixture (`Explorer`/`Search`/`Git`/
  `Terminal`), no Nucleus data. False inert at 300ms; true silent at 299ms
  then `Search` at 300ms; leave, blur-while-hovered, Escape, disabled Git,
  removal-while-pending, and teardown leave no late paint.
- Paired Svelte/React `TabsTooltips` suites with fake timers.

## Falsification

Green proofs first. Plants restored after each row.

| Row | Plant | Result |
| --- | --- | --- |
| Dropped projection | `node.tooltip = None` | `show_tooltips_true_projects_each_tab_label` expected `Some("Explorer")`, got `None` |
| Label-only projection | ignore `shows_tooltips` / vertical | `show_tooltips_false_does_not_project_hover_text` saw a tooltip |
| Skip delay | `advance_clock(0)` after the 299ms None row | mounted test expected visible `Search`, got none |
| Skip hide | omit leave hover | mounted test expected none, got `Search` |

## Validation

Pending focused Tabs suites and `effigy ci:web` / `ci:rust` / `ci:native` /
`docs:check` after the rebase onto g16.066.

## Limits

- No Nucleus edit, windowed/native-visual run, Jetstream, workflow, or shared
  g16 front-door change.
- Nucleus Tabs stays a later card.
