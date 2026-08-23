# g16.003 — Underlay Poodle 0.2.1 Adoption

Date: 2026-08-23
Verdict: **complete — public registry adoption, no compatibility migration**

Underlay PR [#4](https://github.com/inflatable-cookie/underlay/pull/4)
merged at `750005eb`. It pins `@inflatable-cookie/poodle-svelte` to exact
`0.2.1`, removes both committed `file:../poodle` overrides, and resolves
Poodle core and Svelte `0.2.1` from npm in `bun.lock`.

No Underlay adapter, template, contract, test, or documentation change was
needed. Orchestrator review found no unrelated lockfile churn.

Validation passed `effigy validate`: 125 unit files / 770 tests, 12 component
files / 49 tests, Svelte and TypeScript checks, exports, guardrails, and the
Poodle prop-name check. `git diff --check` was clean.
