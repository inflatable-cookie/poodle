# Release notes

Per-version release notes for Poodle's public-intent (preview-channel) packages.
Policy: `docs/specs/022-packaging-versioning-and-release-channel-rules.md` and
`docs/specs/044-deprecation-change-control-and-release-channel-operations.md`.

Every preview package on a real version (anything past the `0.0.0` baseline)
must have a `docs/release-notes/<version>.md` that lists the package and its
change class. This is enforced by `docs:lint`.

Each note records, per the Release Note Rule:

- which packages changed
- whether the change affects public-intent entry points
- whether the change is additive, behavioral, or breaking
- what downstream evaluators should re-check

Still pre-release: breaking changes may occur in `0.x`; no `stable` channel
exists yet.

## Versions

- [0.3.0](0.3.0.md) — 2026-09-02 — immutable candidate with HistoryCenter
  v3 and the root markdown migration; not published
- [0.2.3](0.2.3.md) — 2026-08-30 — prepared but unpublished triggerless
  `ContextMenu` overlay, carried into the `0.3.0` candidate
  composition (`trigger={false}`)
- [0.2.2](0.2.2.md) — 2026-08-24 — corrects the `0.2.1` public GPUI
  dependency-identity defect; restores crates.io GPUI 0.2.2 and replaces
  fork-only offscreen capture with a non-activating window diagnostic
- [0.2.1](0.2.1.md) — 2026-08-23 — registry-release replacement for the
  unpublished `0.2.0` tag; same product payload, repaired npm bootstrap
- [0.2.0](0.2.0.md) — 2026-08-23 — tagged candidate on the frozen 175-component
  Svelte denominator (42 new components, Breadcrumb item icons, focus-ring
  channel; Tabs/Popover/Pill/Icon and Rust presentation-cascade breaks); npm
  workflow stopped before publication
- [0.1.0](0.1.0.md) — 2026-07-24 — first documented preview release (12-theme
  set, ThemeSelect, FilterBuilder, `@inflatable-cookie/poodle-react`; theme id renames)
