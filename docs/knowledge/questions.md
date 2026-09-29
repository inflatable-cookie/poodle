# Questions

Questions that block or shape work. Reference them by ID from the plan and from
briefs. An answered question keeps only its pointer to where the answer lives.

## Q-002 — Does the Nucleus receipt pin still bind?

Status: open (asked 2026-09-29)

The Nucleus that the V1 receipts in `docs/evidence/nucleus/` describe has
been archived; the rebuild has no Poodle yet (operator, 2026-09-29).
`scripts/nucleus-parity-receipts.ts` `SOURCE_PATHS` still pins
`packages/{gpui/preview,gpui/adapter,render,contracts}` byte for byte, so
every native fix there waits for an evidence repin
(`lane:pinned-source-paths`, 11 papercuts plus the Rust keyboard base note).
Options: keep the pin and batch one repin; freeze the receipts as history
and drop the pin (retiring it in `retired.toml`); or re-anchor parity
evidence on the rebuilt Nucleus once it adopts Poodle. The planner's
recommendation is to freeze and drop the pin. The operator has not answered.

## Q-001 — Which product frontier comes next?

Status: open (asked 2026-09-24)

No Poodle work is queued. The candidates are the Queue plan's lanes (`lane:next-frontier`). The
planner's recommendation is a small GPUI repair tranche on basic
selection-navigation controls (Button, Checkbox, Radio, ToggleGroup,
Accordion), compiled from the
[GPUI functionality census](../evidence/gpui/gpui-functionality-census.md).
The operator has not answered.
