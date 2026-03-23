# 2026-03-11 Poodle Northstar Bootstrap, Roadmap Densification, And Checklist Contracts

## Changed

- added a Northstar-shaped `docs/` surface to the Poodle repo
- wrote the initial repo vision and architecture
- replaced the lightweight roadmap surface with denser generations and explicit
  ranges
- added `docs/roadmaps/generation-index.md`
- expanded `g01` into a 14-milestone foundation generation
- expanded `g02` into a 16-milestone advanced-composite and adoption generation
- expanded `g03` into a 14-milestone hardening and maturity generation
- aligned the roadmap posture more closely to the substantive generation style
  used in Chorus, while still keeping Poodle smaller than Loophole-scale programs
- converted the milestone files from brief scopes into actionable checklist
  contracts with explicit goals, execution checklists, acceptance criteria, and
  deliverables where appropriate
- added a concrete token/package layout note and first normative token-source
  and artifact spec so `g01.002` and `g01.003` can move from abstract planning
  into implementation-shaped planning
- integrated the rule that one semantic theme should be translatable across
  browser/Svelte and GPUI consumers from the same source rather than redefined
  separately per runtime
- integrated the rule that Svelte and GPUI are the first required
  implementations, but the token and contract layers must remain open to future
  targets such as React or other desktop UI kits

## Validation

- `git diff --check`

## Remaining

- decide which `g01` milestones should immediately promote into implementation
  packages
- add initial research source hubs for GPUI, Bits, and Underlay bridge design

## Next Task

Open `docs/roadmaps/g01/002-token-system-and-artifact-emission.md` and convert
the token-model tranche into concrete implementation package planning.
