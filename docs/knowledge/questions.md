# Questions

Questions that block or shape work. Reference them by ID from the plan and from
briefs. An answered question keeps only its pointer to where the answer lives.

## Q-004 — What is the route for the Rust renderers?

Status: open, raised 2026-11-25. Jetstream is barely usable, GPUI needs a lot
of work, and parity across the four renderers has been hard to hold. The
operator wants a separate conversation on this. One constraint is already
known: a rendering style that is token data, not CSS selectors, reaches the
Rust renderers through the same build that emits theme artefacts.

## Q-003 — How far can a rendering style reach?

Status: answered 2026-11-25 — token modes plus a small fixed set of treatment
switches. See [vision](vision.md#direction).

## Q-002 — Does the Nucleus receipt pin still bind?

Status: answered 2026-09-29 — no. See [vision](vision.md#direction).

## Q-001 — Which product frontier comes next?

Status: answered 2026-09-29 — finish `lane:native-admission`, then a small
GPUI repair tranche on Button, Checkbox, Radio, ToggleGroup and Accordion from
the [GPUI functionality census](../evidence/gpui/gpui-functionality-census.md)
(`lane:gpui-selection-repair`). The Queue plan holds the order.
