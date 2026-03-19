# g09.008 Generation Closeout

Status: planned
Owner: Pug Core
Depends on: g09.007

## Actions

- [ ] Verify all g09 milestones complete
- [ ] Tally:
  - Crates eliminated (expect: 5 — gpui-tokens, gpui-primitives,
    gpui-composites, gpui-workstation, contracts-workstation)
  - Import paths simplified
  - Component names simplified (Pug prefix dropped)
- [ ] Confirm crate dependency graph is clean:
  - `pug-tokens` ← `pug-primitives` ← `pug-composites`
  - `pug-gpui-components` → `pug-primitives`, `pug-composites`
  - `pug-jetstream-components` → `pug-primitives`
  - No circular or redundant dependencies
- [ ] Confirm g10 (Jetstream Production Quality) can begin
- [ ] Close generation

## Acceptance Criteria

- [ ] Every claim verifiable by reading the code
- [ ] g09 explicitly closed
- [ ] g10 ready to begin from unified baseline
