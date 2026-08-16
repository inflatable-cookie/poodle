# g15.001 — Release-Baseline Roster Inventory

Status: complete — accepted in PR #24
Depends on: `g14.022` (closeout), `g14.021` (cleanup evidence)
Governing refs: `../g14/022-generation-closeout.md`,
`../g14/conformance-estate.md`, `../../contracts/001-working-rules.md`,
`../../logs/2026-08/16-g14-022-generation-closeout.md`

## Outcome

Freeze the Poodle v0.2.0 denominator from source and package exports, then
inventory every public Svelte component against that roster with explicit
posture per surface. The result is the release baseline: what is certified,
what is missing, and who owns each gap. It is not a third parity
architecture and it does not compile rollout work from memory.

This card is the first executable step of the v0.2.0 release runway.

## Denominator Ruling (operator-approved, `g14.022`)

- The v0.2.0 denominator is **every public Svelte component export**,
  enumerated from component-valued exports in
  `packages/svelte/components/src/index.ts` and checked for packed reachability
  through the package `exports` map — not a representative subset. Public
  types and helpers are recorded separately; they are not components in the
  denominator.
- React remains tightly paired with Svelte through shared CSS and
  framework-free web behaviour. Record React mirror coverage honestly; do not
  let an experimental React gap make the Svelte denominator vague.
- Native release evidence names an explicit certified GPUI subset until the
  full Rust roster is complete. Do not call that full cross-runtime parity.
- Svelte v0.2.0 certification is not active-cohort parity completion. React,
  Rust, and GPUI gaps remain open under the working rules and experimental
  packages keep honest labels.
- Jetstream is program-deferred. Its eventual admission is renderer and
  backend work, not an assumed consequence of `poodle-node` reuse.
- Contracts remain semantic authority. Curated specimens remain
  human-facing documentation, not parity snapshots or exhaustive variant
  matrices.

## Goals

- [ ] Enumerate the complete Svelte public roster from source/package exports
      and freeze it as the v0.2.0 denominator.
- [ ] For every component, record posture on each surface:
      contract, implementation, export, specimen, focused test,
      package-install, and downstream use.
- [ ] Record React mirror coverage, the certified GPUI subset, and deferred
      Jetstream separately. Missing evidence remains missing; one runtime
      does not borrow another runtime's pass.
- [ ] Produce the release-gap register: every component whose surfaces are
      not all present or not all evidenced, with a named owner.
- [ ] Compile a visible next runway of bounded family tranches from measured
      gaps only after this inventory exists.

## Surface Posture Fields

For each roster component, record one of `complete` / `partial` / `missing` /
`not-applicable` with evidence:

| Surface | Authority |
| --- | --- |
| Contract | `docs/contracts/components/<name>.md`; `docs:contract-drift`, `docs:spec-drift`, `docs:value-domain-drift`, `docs:callback-drift` gates |
| Implementation | Svelte component in `packages/svelte/components/src/`; `check:svelte` |
| Export | `packages/svelte/components/src/index.ts` and package `exports` map |
| Specimen | Svelte preview catalogue entry; React gallery mirror where applicable |
| Focused Svelte evidence | exact component test file/case, or `missing`; an aggregate green selector is not component proof |
| React mirror | exact implementation, export, specimen, and focused test where the component is in scope |
| Rust declaration/render | exact declaration, render path, and focused test; record each independently |
| GPUI surface | exact backend/specimen/headless test; `regressions:native` counts only where a named regression exercises this component |
| Package-install | `test:web-pack-install` consumer smoke over the packed tarball |
| Downstream use | read-only contextual evidence from known consumers; absence is not a release failure; direction Longhorn → Poodle |

Record whole-board validation separately from the per-component inventory.
One runtime never borrows another runtime's test, export, or specimen pass.

## Deliverables

- `release-baseline-roster.md` — frozen denominator and per-surface evidence
- `release-gap-register.md` — every incomplete surface, owner, and disposition
- bounded family-tranche roadmap cards compiled from the measured register;
  do not pre-author their contents from memory
- one August batch log recording method, commands, uncertainties, and result

## Carry-Forward Requirements (recorded, not implemented)

- Approved Licence and model-connection web suites stay approved; their
  native completion recompiles under g15 (`g14.017`, `g14.020` requirements).
- The human-centred specimen catalogue audit (`g14.026`) carries forward with
  its rubric and bounded shared specimen-plan boundary.
- A primitive-first visual conformance lane may later reuse the retained
  headless/native capture foundation (`conformance-estate.md`). This card
  records that seam; it does not design or build the harness.
- Native completion of any component the inventory finds incomplete is
  sequenced after this card, never assumed.

## Acceptance

- [ ] The frozen denominator equals the full Svelte export set; no silent
      sampling.
- [ ] Every component has an explicit per-surface posture; nothing is left
      implied.
- [ ] React, GPUI, and Jetstream posture is recorded separately and honestly.
- [ ] The release-gap register names a live owner per gap.
- [ ] No component API, runtime code, specimen, or workflow was changed to
      produce the inventory.
- [ ] The g15 runway contains bounded family tranches compiled from measured
      gaps, with the first executable tranche clearly identified.

## Stop Conditions

- The roster silently samples the Svelte export set.
- A surface posture is inferred from another runtime's pass.
- Work starts designing a replacement component authority, shared corpus,
  comparator, or visual harness.
- Any change reaches component/runtime source, curated specimens,
  `.github/workflows/`, Jetstream, or downstream application repositories.

## Writable Scope

- the release inventory and gap register
- this roadmap, the measured family-tranche cards, and the g15 index
- one August batch log
- `PAPERCUTS.md` only for newly discovered execution friction

## Validation

- inventory consistency check against the exported roster
- `effigy docs:check`
- `effigy check:svelte`
- `effigy react:build`
- `effigy test:components`
- `effigy test:web-pack-install`
- `effigy qa` — headless local release board
- `effigy doctor` — record baseline findings without fixing them
- `git diff --check`

Never run a `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or
any Jetstream selector.
