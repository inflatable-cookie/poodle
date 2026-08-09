# Icon Catalogue Boundary

Card 021 is complete.

## Changed

- Replaced the 1,703-icon core catalogue with Poodle's verified 54-icon
  default Lucide adapter.
- Removed the catalogue namespace lookup and `lucide-static` core dependency.
- Added loud unresolved-name handling with a visible error glyph.
- Added `poodle-icons`, which turns an application-owned JSON name list into a
  self-contained TypeScript icon set with direct named exports.
- Migrated the sixteen audited consumers. Reusable libraries own direct nodes;
  only applications with non-default names add a root provider.

## Measured

- Core tarball: 291,471 → 159,553 bytes.
- Soundcheck JavaScript: 1,400.40 → 1,033.35 KB.
- Soundcheck gzip: 366.36 → 282.97 KB.
- Soundcheck contains none of `biohazard`, `cassette`, `origami`, or
  `squirrel` after a clean build.

## Validated

- `effigy ci`
- `effigy test:svelte-pack-install`
- Poodle core and component tests
- Nucleus, Loophole, Soundcheck, Jetstream, Figmatic, Finch, and Underlay
  targeted checks or production builds
