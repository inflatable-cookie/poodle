# Package Publication Integrity

Poodle remains `strict-ready`. Public package payloads and release inventory
now have complete, repeatable checks before the release pipeline takes over.

## Findings

- The three public-intent npm packages packed only allowlisted source,
  documentation, license, and manifest files. No tests, previews, artifacts,
  or workspace dependencies leaked into the tarballs.
- All 13 public-intent Rust crates produced bounded `cargo package --list`
  payloads with complete license, repository, readme, version, and release
  metadata.
- `@inflatable-cookie/poodle-react` had no clean tarball-consumer proof.
- The canonical release manifest omitted the internal React preview and Svelte
  install-smoke packages, despite the rule that every package has a release
  class.

## Repaired

- Expanded the packed-install proof to install `poodle-core`, `poodle-svelte`,
  and `poodle-react` tarballs together with React 18.0.0 and Svelte 5.38.6.
- Added mounted React coverage through public `Button`, `Icon`, and
  `IconProvider` imports plus the scoped core icon export.
- Classified both missing internal tools in the release manifest and release
  operations record, with matching package metadata.
- Made docs lint reject any first-party package manifest absent from the
  canonical release inventory, even when the new package forgot release
  metadata too.
- Made docs lint resolve every concrete public npm export and require wildcard
  targets to match at least one packaged source file.

The public npm packages remain private until the release pipeline deliberately
enables publication. This batch changes readiness evidence, not publication
state.

## Validated

- `effigy test:web-pack-install`
- `effigy docs:check`
- `effigy doctor`
- `git diff --check`
