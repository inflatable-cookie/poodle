# License and Attribution Integrity

Poodle remains `strict-ready`. First-party license declarations, Rust
dependency policy, and redistributed third-party assets now have one
repeatable release gate.

## Findings

- Poodle's root and three public npm packages already carried identical MIT
  license text. All 15 Cargo manifests declared MIT.
- Three internal npm manifests omitted a license field.
- The scoped icon data in `poodle-core` comes from Lucide, but its upstream ISC
  and Feather MIT notices were absent from the published package.
- The GPUI preview bundles Lucide 0.577.0 SVGs and Inter 4.001 font files
  without adjacent upstream license text.
- All dependencies in the four public Rust graphs use reviewed permissive
  licenses. No unlicensed or disallowed dependency was found.

## Repaired

- Added a repository third-party inventory and complete upstream terms beside
  the Lucide and Inter assets.
- Included Lucide's complete notice in the public `poodle-core` tarball.
- Normalized all first-party npm manifests to MIT.
- Added a fail-closed `cargo-deny` license allowlist for the four public Rust
  graphs.
- Added `effigy audit:licenses` to check package and crate metadata, public
  package license copies, required notices, and Rust dependency licenses.
- Extended the clean-consumer package proof to require license files and the
  core third-party notice in installed tarballs.

## Validated

- `effigy audit:licenses`
- `effigy test:web-pack-install`
- `effigy docs:check`
- `effigy doctor`
- `git diff --check`
