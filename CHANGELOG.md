# Changelog

Notable changes to Poodle are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow
[Semantic Versioning](https://semver.org/spec/v2.0.0.html). Poodle is pre-1.0,
so minor releases may contain documented breaking changes.

## [Unreleased]

### Added

- **Button `controls` prop** in Svelte and React, rendered as `aria-controls`
  when non-null, with `ButtonSpec::controls` / `with_controls` in
  `poodle-specs` projected to `NodeA11y.controls` on the shared render path
  (the existing IconButton seam). Both web roots also re-export the
  core-authored `PopoverTriggerState` type.

### Fixed

- **Tabs drag dead in WebKit hosts.** `draggable` and `dragstart` sat on the
  `__item` chip div wrapping the `__tab` button; WebKit does not initiate a
  native drag when the press target is a form control inside a draggable
  ancestor, so WKWebView (Tauri) hosts could not drag any tab. The drag source
  moved onto `.poodle-tabs__tab` itself (Svelte and React); drop-target
  handling stays on the item. Consumers styling `[draggable]` on
  `.poodle-tabs__item` should target `.poodle-tabs__tab` instead.

### Removed

- **Breaking — Tabs `variant`.** The five-member `TabVariant` union is now three
  members: `"card" | "pill" | "block"`. `"text"`, `"underline"`, and `"strip"`
  are gone. `"underline"` was never a distinct look — it aliased `"text"` and
  had no stylesheet rules of its own.

  Migrate as follows. Note that `"card"` is a **reused name, not a preserved
  one**: the old `"card"` filled the tab chip, and the new `"card"` is the old
  `"text"`. Applying this table before upgrading will change how your tabs look.

  | Before | After |
  |---|---|
  | `variant="text"` | `variant="card" bordered` |
  | `variant="underline"` | `variant="card" bordered` |
  | `variant="strip"` | `variant="block" activeEdge="underline" activeFill="none"` |
  | `variant="card"` | `variant="card"` — appearance changed; see `bordered`, `activeEdge`, `activeFill` |
  | `variant="pill"`, `variant="block"` | unchanged |

  `TabStrip` is a separate component and is unaffected.

### Changed

- **Breaking — Popover interactive trigger composition.** Composing a real
  Button or IconButton as a Popover trigger no longer forces a choice between
  nested interactive semantics and a missing disclosure relationship. In
  interactive mode (`triggerIsInteractive`) the trigger is now a state-aware
  render that receives the core-authored `PopoverTriggerState` (`expanded`,
  `controls`, `disabled`): Svelte `trigger: Snippet<[PopoverTriggerState]>`,
  React `trigger: (state: PopoverTriggerState) => ReactNode`. The actual
  control owns `aria-expanded`, `aria-controls`, and the disabled state — in
  server output and hydrated DOM alike — while the wrapper stays a roleless,
  untabbable layout host. The old interactive shape (a static node or
  zero-argument snippet beside `triggerIsInteractive`) is gone: React rejects
  it at compile time; Svelte's discriminated snippet typing rejects a
  wrongly-typed payload and wrong-branch usage but cannot reject a
  zero-argument snippet (TypeScript function assignability), so Svelte
  migration is enforced by search and review.

  Migrate: give every interactive trigger the state parameter and apply all
  three fields to the real control — `Button` takes `ariaExpanded` /
  `controls` / `disabled`, `IconButton` takes `expanded` / `controls` /
  `disabled`, a native button takes `aria-expanded` / `aria-controls` /
  `disabled`. Direct Rust `ButtonSpec` struct literals must initialize the new
  `controls` field; builder callers are source-compatible.
- **Breaking — Tabs `bordered` now defaults to `false`.** This is a silent
  visual change: tabs rendered above a panel lose their separating line with no
  type or build error. Add `bordered` explicitly to any usage that draws tabs
  over content. The old default assumed "tabs above content", a layout Tabs
  cannot see, and every other usage paid for it in dead space.
- **Tabs selection decoration is now two orthogonal axes.** `activeEdge`
  (`"none" | "outline" | "underline"`, default `"none"`) and `activeFill`
  (`"none" | "tint" | "solid"`, default `"tint"`) compose freely and replace the
  former per-variant treatments. `NavigationMenu` takes the same two props. Both
  types are defined once in
  [`docs/contracts/004-shared-control-types.md`](docs/contracts/004-shared-control-types.md).
- Prepared the repository, package documentation, licensing, security policy,
  and validation surfaces for public access.
- Completed the shared Rust render-tree migration. GPUI and Jetstream now
  interpret the same `poodle-node` output instead of maintaining duplicate
  component implementations.
- Completed the native accessibility naming audit across the Jetstream
  specimen catalogue.

## [0.1.0] - 2026-07-24

### Added

- Established the first documented preview baseline: framework-free core,
  Svelte and experimental React component packages, shared tokens and themes,
  Rust contracts, the shared render tree, and GPUI and Jetstream adapters.
  This was a source/version baseline, not a registry publication or GitHub
  release tag.

### Changed

- Renamed theme IDs and removed the obsolete `poodle-workstation` crate. See
  the [full 0.1.0 release notes](docs/release-notes/0.1.0.md) for package lists,
  migration guidance, and downstream checks.

[Unreleased]: https://github.com/inflatable-cookie/poodle/compare/f8fac6a6...HEAD
[0.1.0]: docs/release-notes/0.1.0.md
