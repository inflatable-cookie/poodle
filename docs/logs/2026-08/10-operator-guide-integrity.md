# Operator Guide Integrity

Poodle remains `strict-ready`. The public guides described the right system,
but several executable examples had drifted behind the current package APIs.

## Findings

- Four Svelte guides still used legacy slot composition instead of Svelte 5
  snippets.
- The media recipe named a nonexistent `MediaBrowseItem` type and passed stale
  `MediaBrowsePanel`, `MediaThumbnail`, and `MediaPreview` props.
- The Svelte guide showed an old ranged Lucide dependency, obsolete callback
  names, legacy task commands, and a manually maintained component inventory.
- Both native developer guides and the render package README used the removed
  `ButtonVariant::Solid` variant.
- The Underlay bridge README used the old BlockEditor type-picker slot shape.

## Repaired

- Migrated operator examples to named Svelte 5 snippets and current media
  types, props, and content composition.
- Aligned Lucide setup with the canonical exact version and changed preview
  commands to Effigy selectors.
- Replaced the stale component inventory with links to the package reference,
  contracts, and preview.
- Updated native button examples to `ButtonVariant::Primary`.
- Added docs lint rules for the retired example shapes and for Lucide version
  drift between the root dependency, icon manifest, and Svelte guide.

## Validated

- `effigy docs:check`
- `git diff --check`
