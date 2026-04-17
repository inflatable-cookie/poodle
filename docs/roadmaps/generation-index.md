# Roadmap Generation Index

## Active Execution Track

- `g10`
  - Status: active
  - Range: `001` to `011` on disk (all complete)
  - Notes: `g10.001` proved Jetstream feasibility. `g10.002` recovered the
    live queue and classified three seams (Svelte overhaul, Jetstream
    implementation, parity). `g10.003` closed the Svelte overhaul.
    `g10.004` unified the component package. `g10.005` to `g10.007` closed the
    checkpointed GPUI parity tranche. `g10.008` delivered list grid and list
    card counter GPUI coverage plus token additions.     `g10.009` tightened GPUI
    token resolution for button, region, and field plus schema additions
    (`color.text.tertiary`, button gap/inset).     `g10.010` delivered interactive
    pagination (full variant + limit selector) and spec-derived adapter handles
    for `ConfirmActionSpec` under the components↔adapter cycle constraint.
    `g10.011` added `ButtonSpec::aria_expanded` with contract scoping for web-only
    form props and documented GPUI/Jetstream ARIA emission gaps (D-002). **`g10.012`**
    retires the historical `g08/delta-register.md`, records **real** GPUI 0.2.2
    limits vs Poodle debt, and tracks execution for text input, select overlay,
    sliders, token literals, and adapter mounting. Choose next work via `g10/README.md`.

## Completed Foundations

- `g09`
  - Status: completed
  - Range: `001` to `009`
  - Notes: architecture unification, GPUI continuation, semantic sizing or
    density rollout, and the original cross-runtime verification tranche are
    complete enough that `g09` no longer acts as the live queue

- `g01`
  - Status: completed
  - Range: `001` to `014`
  - Notes: repository bootstrap, token model, contract system, primitive suite, workstation shells, Underlay bridge, and first parity baseline

- `g02`
  - Status: completed
  - Range: `001` to `016`
  - Notes: advanced composites, product and workstation depth, docs and preview cleanup, API cleanup, packaging, and release baseline

- `g03`
  - Status: completed
  - Range: `001` to `014`
  - Notes: migration policy, parity automation, docs publishing, downstream adoption, ecosystem validation, change control, and extension support

- `g04`
  - Status: completed
  - Range: `001` to `018`
  - Notes: Underlay component parity, new component families, feature depth, and specialist editing or media surfaces

- `g05`
  - Status: completed
  - Range: `001` to `014`
  - Notes: GPUI foundation, spec crates, cross-runtime parity baseline, and demo alignment

- `g06`
  - Status: completed
  - Range: `001` to `015`
  - Notes: shared multi-renderer contract layer, crate restructuring, typed token resolution, layout and event abstractions, style descriptors, adapter traits, and full component-surface expansion

- `g07`
  - Status: completed
  - Range: `001` to `015`
  - Notes: GPUI rendering build-out, adapter crate, theme integration, primitive and composite rendering, workstation shell updates, and cross-runtime parity reporting

- `g08`
  - Status: completed
  - Range: `001` to `011`
  - Notes: consolidated GPUI production-quality, contract-compliance, specimen, accessibility, and visual-parity work

## Removed Generations

- `g11`
  - Status: removed
  - Notes: this generation no longer exists as part of the live roadmap structure; its former scope was merged down and should be treated as superseded

## Working Rule

When roadmap files disagree:

1. treat this index as the top-level source of truth
2. treat `docs/roadmaps/README.md` as the entrypoint
3. treat `g10` as the active generation until it is explicitly closed

## Rollover policy

Create a new generation only when maintainers explicitly decide the sequencing
baseline needs a real reset.

Generations should be substantial. As a healthy default, expect something
closer to 20 to 40 roadmap files before rollover is worth discussing. Treat
that as a judgment guardrail, not an automatic counter.

Rollover is a closeout event, not a convenience move. Before opening the next
generation:

- close, pause, supersede, or rehome every roadmap in the current generation
- refresh the roadmap front doors so the old generation is visibly closed
- purge stale generation-specific strict-planning artifacts from the active
  `docs/specs/` tree

If that cleanup has not happened, stay in the current generation and finish the
closeout there first.

    `g10.013`–`g10.017` delivered a systematic GPUI component correctness and
    token-fidelity sweep (formula literals, composite/primitive parity vs Svelte).
    `g10.018` completed the formula sweep third pass (skeleton, datetime pickers,
    switch, tabs, and others). `g10.019`–`g10.021` are queued: contract sync
    priority sweep, spec struct coverage gaps, and GPUI accessibility baseline.

## Next Task

Open `g10/README.md`. Active seam: `g10.019` contract sync — start with TextInput
multiline props. Or open `g10/021` to begin the accessibility investigation.
generation rollover, or backlog; the on-disk `g10.009`–`g10.011` tranche is
complete.
