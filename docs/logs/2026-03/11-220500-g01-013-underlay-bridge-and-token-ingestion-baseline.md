# 2026-03-11 g01.013 Underlay Bridge And Token-Ingestion Baseline

## Changed

- completed the `g01.013` Underlay bridge baseline above the now-explicit token
  and contract surfaces
- added the Underlay bridge architecture note:
  - `docs/architecture/004-underlay-bridge-and-adapter-ownership.md`
- added the normative bridge and wrapper-preservation spec:
  - `docs/specs/007-underlay-bridge-and-wrapper-preservation-rules.md`
- converted the reserved bridge package into real baseline scaffolding under:
  - `packages/bridges/underlay/`
- added concrete seed bridge artifacts for:
  - CSS variable mapping
  - TypeScript token mapping
  - theme and mode translation
  - wrapper-preservation manifest guidance
- updated roadmap, index, and bridge-facing docs so the Underlay posture is
  visible from the main surfaces
- closed `g01.013` in the active roadmap

## Bridge Posture

- Flint remains the canonical home of tokens and contracts
- Underlay remains the public API surface for Underlay apps
- the bridge package owns alias maps, theme translation, wrapper-preservation
  rules, and migration pressure documentation
- Underlay apps should not need direct Flint imports, Flint token names, or Flint
  component names

## Accessibility

- made accessibility preservation explicit inside the bridge layer instead of
  treating it as a wrapper detail
- token maps and wrapper adaptation are allowed to rename or repackage
  semantics, but not to drop:
  - accessible names
  - roles
  - state/value exposure
  - keyboard behavior
  - focus restoration
  - announcement behavior where relevant

## Validation

- `bun packages/tokens/scripts/build-tokens.ts`
- `git diff --check`

## Remaining

- execute `g01.014` to freeze parity evidence, downstream extension rules, and
  the `g02` cutover posture
- keep the zero-leak Underlay goal explicit as later adoption work becomes
  concrete

## Next Task

Open `docs/roadmaps/g01/014-parity-evidence-downstream-extension-and-g02-cutover.md`
and close the generation with parity evidence rules plus the downstream
extension contract.
