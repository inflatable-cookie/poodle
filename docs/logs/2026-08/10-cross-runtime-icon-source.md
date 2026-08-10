# Cross-Runtime Icon Source

Poodle remains `strict-ready`. Web and native default icons now come from one
pinned source instead of independent catalogues.

## Findings

- Core shipped 54 Lucide 1.31.0 node modules; the GPUI preview carried 63
  Lucide 0.577.0 SVGs.
- Only 31 names overlapped. `dot` had different geometry across those outputs.
- Native components emitted names such as `spinner`, `calendar`, `folder`, and
  `monitor-play` that the web default did not expose. Several legacy web aliases
  targeted icons absent from its own default set.
- Jetstream loaded assets directly from the GPUI preview package.

## Repaired

- Added one canonical manifest with 85 Lucide names and 15 compatibility
  aliases, pinned to `lucide-static` 1.31.0.
- Added `effigy icons:build` to generate both core `IconNodes` modules and
  shared `poodle-render` SVG assets.
- Repointed GPUI and Jetstream previews to the shared Rust assets.
- Added `effigy audit:icons` to compare every generated name and byte of
  geometry against the manifest; `ci:web` now runs it.
- Removed the older GPUI-owned SVG set and consolidated Lucide attribution on
  version 1.31.0.

## Validated

- `effigy audit:icons`
- `effigy test:core`
- `effigy test:components`
- `effigy check:gpui`
- `effigy test:jetstream-visual`
- `effigy docs:check`
- `effigy doctor`
- `git diff --check`
