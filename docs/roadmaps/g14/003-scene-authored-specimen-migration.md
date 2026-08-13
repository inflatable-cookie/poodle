# g14.003 Scene-authored Specimen Migration

Status: planned
Owner: Poodle core
Depends on: `g14.001` (spec promotion), `g14.002` (specimen inventory)
Governing refs: spec 063's scene half (promoted at `g14.001`),
`../g13/pilot-verdict-evidence.md` §1 (the four-runtime scene proof),
`../g13/batch-cards/035-shell-scene-rust-authoring-and-web.md`,
`../g13/batch-cards/036-shell-scene-native-shells.md`

## Objective

Make specimen fixtures one definition across four runtimes. The scene IR is
the one pilot surface that replaced duplication instead of adding to it:
four hand-written preview shells became one 178-line Rust source
(b035/b036). Specimens are the same shape of problem — four hand-written
copies that demonstrably drift (the React preview died on the lineage
because Svelte's registry listed specimens React never had). One fixture
definition makes implementation differences diagnosable instead of
confounded: same props, same states, same axes in every runtime.

## Deliverables

- Tranche one, measured: migrate the static specimen surface (states rows,
  prop demos, size/density matrices, axes) for one family, reporting the
  per-specimen authoring cost before anything wider. The b052 rule applies
  here too: measure a tranche, then decide.
- Scene schema additions the tranche proves necessary — and none before
  then. No scene evaluator; bindings are literals and declared axes.
- Interactive specimens (host harnesses, machine-driven demos) classified
  separately: the scene declares the fixture, the runtime keeps a thin
  harness. HistoryCentre-class specimens are named explicitly.
- Per-runtime specimen absence, declared not silent, while `g14.008`
  closes the registration gap.
- `ir:check` covers the migrated fixtures.

## Acceptance

- [ ] The same specimen renders in all four runtimes from one definition;
  a planted fixture divergence fails `ir:check`.
- [ ] Per-specimen cost measured and reported; the family rollout is gated
  on it.
- [ ] No scene schema creep: anything the static tier cannot express is
  classified harness or absence, not a schema extension.

## Next

Feeds `g14.008` (gap closure needs shared fixtures) and `g14.009`
(evidence gates consume them).
