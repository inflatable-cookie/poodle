# Rename: Flint -> Flint

## Context

This project is a multi-platform design system previously called **Flint**. It needs to be renamed to **Flint** due to naming conflicts with the Flint template language (formerly Jade). The rename is comprehensive — it touches Rust crate names, NPM package scopes, CSS custom property prefixes, file names, import paths, config files, documentation, and UI strings.

The project lives at `/Users/betterthanclay/Dev/projects/flint` and should be moved to `/Users/betterthanclay/Dev/projects/flint` after the rename is complete.

## Project Structure

```
flint/
├── package.json              # Root workspace: name "flint"
├── effigy.toml               # Task runner: alias "flint", refs flint-gpui-preview etc.
├── packages/
│   ├── contracts/             # Shared Rust spec crates (platform-agnostic)
│   │   ├── adapter/           # flint-adapter
│   │   ├── composites/        # flint-composites
│   │   ├── events/            # flint-events
│   │   ├── layout/            # flint-layout
│   │   ├── primitives/        # flint-primitives
│   │   ├── style/             # flint-style
│   │   ├── tokens/            # flint-tokens
│   │   └── workstation/       # flint-workstation
│   ├── gpui/                  # GPUI (macOS native) renderer
│   │   ├── adapter/           # flint-gpui
│   │   ├── components/        # flint-gpui-components
│   │   └── preview/           # flint-gpui-preview
│   ├── jetstream/             # Jetstream (game engine) renderer
│   │   ├── adapter/           # flint-jetstream
│   │   ├── components/        # flint-jetstream-components
│   │   └── preview/           # flint-jetstream-preview
│   ├── svelte/                # Svelte web renderer
│   │   ├── composites/        # @flint/svelte-composites
│   │   ├── icons-lucide/      # @flint/icons-lucide
│   │   ├── install-smoke/     # @flint/install-smoke
│   │   ├── primitives/        # @flint/svelte-primitives
│   │   ├── preview/           # @flint/svelte-preview
│   │   └── tokens/            # @flint/svelte-tokens
│   ├── tokens/                # @flint/tokens (token pipeline)
│   └── bridges/
│       └── underlay/          # @flint/bridge-underlay
└── docs/                      # Documentation and research
```

## What Needs to Change

### 1. Rust Crate Names (14 Cargo.toml files)

Every `Cargo.toml` has a `name = "flint-*"` and dependencies like `flint-adapter = { path = "..." }`.

| File | Old Name | New Name |
|------|----------|----------|
| `packages/contracts/adapter/Cargo.toml` | `flint-adapter` | `flint-adapter` |
| `packages/contracts/composites/Cargo.toml` | `flint-composites` | `flint-composites` |
| `packages/contracts/events/Cargo.toml` | `flint-events` | `flint-events` |
| `packages/contracts/layout/Cargo.toml` | `flint-layout` | `flint-layout` |
| `packages/contracts/primitives/Cargo.toml` | `flint-primitives` | `flint-primitives` |
| `packages/contracts/style/Cargo.toml` | `flint-style` | `flint-style` |
| `packages/contracts/tokens/Cargo.toml` | `flint-tokens` | `flint-tokens` |
| `packages/contracts/workstation/Cargo.toml` | `flint-workstation` | `flint-workstation` |
| `packages/gpui/adapter/Cargo.toml` | `flint-gpui` | `flint-gpui` |
| `packages/gpui/components/Cargo.toml` | `flint-gpui-components` | `flint-gpui-components` |
| `packages/gpui/preview/Cargo.toml` | `flint-gpui-preview` | `flint-gpui-preview` |
| `packages/jetstream/adapter/Cargo.toml` | `flint-jetstream` | `flint-jetstream` |
| `packages/jetstream/components/Cargo.toml` | `flint-jetstream-components` | `flint-jetstream-components` |
| `packages/jetstream/preview/Cargo.toml` | `flint-jetstream-preview` | `flint-jetstream-preview` |

**Dependencies within these files** also reference `flint-*` crate names and must be updated.

### 2. Rust Source Code Imports

All `use flint_*::` imports become `use flint_*::`. Rust converts hyphens to underscores for module names, so `flint-adapter` -> `flint_adapter` in code becomes `flint-adapter` -> `flint_adapter`.

Key patterns to find and replace:
- `use flint_adapter` -> `use flint_adapter`
- `use flint_composites` -> `use flint_composites`
- `use flint_events` -> `use flint_events`
- `use flint_gpui` -> `use flint_gpui`
- `use flint_layout` -> `use flint_layout`
- `use flint_primitives` -> `use flint_primitives`
- `use flint_style` -> `use flint_style`
- `use flint_tokens` -> `use flint_tokens`
- `use flint_workstation` -> `use flint_workstation`
- `flint_gpui::` -> `flint_gpui::` (qualified paths in code)
- `flint_primitives::` -> `flint_primitives::` (etc.)

Also check for any string literals referencing `"flint"` in Rust code (e.g., `"flint-accordion"` element IDs — these should become `"flint-accordion"`).

### 3. NPM Package Names (8 package.json files + root)

| File | Old Name | New Name |
|------|----------|----------|
| `package.json` (root) | `flint` | `flint` |
| `packages/tokens/package.json` | `@flint/tokens` | `@flint/tokens` |
| `packages/svelte/tokens/package.json` | `@flint/svelte-tokens` | `@flint/svelte-tokens` |
| `packages/svelte/primitives/package.json` | `@flint/svelte-primitives` | `@flint/svelte-primitives` |
| `packages/svelte/composites/package.json` | `@flint/svelte-composites` | `@flint/svelte-composites` |
| `packages/svelte/preview/package.json` | `@flint/svelte-preview` | `@flint/svelte-preview` |
| `packages/svelte/icons-lucide/package.json` | `@flint/icons-lucide` | `@flint/icons-lucide` |
| `packages/svelte/install-smoke/package.json` | `@flint/install-smoke` | `@flint/install-smoke` |
| `packages/bridges/underlay/package.json` | `@flint/bridge-underlay` | `@flint/bridge-underlay` |

Also update:
- All `"@flint/*"` dependency references within these files
- Any `"flintRelease"` custom config fields -> `"flintRelease"`

### 4. CSS Custom Properties (hundreds of files)

All CSS variables use the `--flint-` prefix. These appear in:

**Generated token CSS files** (in `packages/tokens/artifacts/css/` and `packages/svelte/tokens/src/generated/css/`):
- `flint-tokens.css` -> `flint-tokens.css`
- `flint-theme-light.css` -> `flint-theme-light.css`
- `flint-theme-dark.css` -> `flint-theme-dark.css`
- `flint-theme-loophole-studio.css` -> `flint-theme-loophole-studio.css`
- `flint-control-size-sm.css` -> `flint-control-size-sm.css`
- `flint-control-size-md.css` -> `flint-control-size-md.css`
- `flint-control-size-lg.css` -> `flint-control-size-lg.css`
- `flint-density-comfortable.css` -> `flint-density-comfortable.css`
- `flint-density-compact.css` -> `flint-density-compact.css`

**Inside these files**, every CSS variable declaration and reference:
- `--flint-semantic-color-*` -> `--flint-semantic-color-*`
- `--flint-semantic-size-*` -> `--flint-semantic-size-*`
- `--flint-semantic-space-*` -> `--flint-semantic-space-*`
- etc.

**Svelte component files** (200+ `.svelte` files) reference `--flint-*` in `<style>` blocks via `var(--flint-...)`.

**Bridge CSS**: `packages/bridges/underlay/css/flint-to-underlay.css` -> `flint-to-underlay.css`

**IMPORTANT**: The token pipeline (`packages/tokens/scripts/build-tokens.ts`) generates these CSS files. The prefix is likely configured there — find and update the prefix source so regenerated tokens use `--flint-*`. Updating the generator is more important than manually editing generated output files.

### 5. Token Pipeline / Build Scripts

- `packages/tokens/scripts/build-tokens.ts` — likely contains the `"flint"` prefix used when generating CSS variable names
- `packages/svelte/tokens/src/generated/ts/metadata.ts` — generated TypeScript metadata
- `packages/svelte/tokens/src/generated/ts/index.ts` — generated TypeScript exports
- `packages/tokens/artifacts/ts/index.ts` and `metadata.ts`
- `packages/tokens/artifacts/rust/metadata.rs` — generated Rust token metadata

Search these for `"flint"` string literals that control the CSS variable prefix generation.

### 6. Config Files

**`effigy.toml`** (task runner):
- `alias = "flint"` -> `alias = "flint"`
- All `-p flint-gpui-preview` -> `-p flint-gpui-preview`
- All `-p flint-jetstream-preview` -> `-p flint-jetstream-preview`
- All `-p flint-gpui` -> `-p flint-gpui`

**`packages/svelte/preview/vite.config.ts`**:
- `"@flint/svelte-primitives"` -> `"@flint/svelte-primitives"`
- `"@flint/svelte-composites"` -> `"@flint/svelte-composites"`

**`packages/svelte/tokens/package.json`** exports:
- `"./styles.css": "./src/generated/css/flint-tokens.css"` -> `"./styles.css": "./src/generated/css/flint-tokens.css"`

### 7. UI Strings and Preview App

The GPUI preview app title bar and Svelte preview app header display "Flint" — search for display strings:
- Preview app titles, headers, logos
- `packages/svelte/preview/src/App.svelte` — likely contains "Flint" in the UI
- GPUI preview main window title

### 8. Documentation

All files in `docs/` reference "Flint" in prose. Update:
- `docs/roadmaps/GPUI_VISUAL_PASS_HANDOVER.md`
- `docs/guides/svelte-developer-guide.md`
- `docs/research/**` — various research memos
- Any README.md files
- This rename document itself

### 9. Element IDs in GPUI Components

The GPUI Rust components use `"flint-"` prefixed element IDs:
- `"flint-accordion"`, `"flint-pill"`, `"flint-pill-remove"`, `"flint-number-entry"`, `"flint-input"`, etc.
- These should become `"flint-accordion"`, `"flint-pill"`, etc.
- Search all `.rs` files for string literals containing `"flint-"` or `"flint_"`.

### 10. Lock Files and Build Artifacts

After renaming, these need to be regenerated (not manually edited):
- `bun.lock` — run `bun install` after renaming all package.json files
- `packages/*/node_modules/@flint/` — will be recreated by `bun install`
- `target/` directories — run `cargo clean` then rebuild
- `dist/` directories — rebuild after rename

## Execution Order

1. **Token pipeline source** — Update the CSS variable prefix in `build-tokens.ts` first
2. **Cargo.toml files** (all 14) — Update package names and dependency names
3. **Rust source files** — Find/replace all `flint_` module references and `"flint-"` string literals
4. **package.json files** (all 9) — Update package names, dependencies, exports, custom fields
5. **Config files** — `effigy.toml`, `vite.config.ts`
6. **Regenerate tokens** — Run the token build to regenerate CSS files with `--flint-*` prefix
7. **Svelte components** — Find/replace `--flint-` with `--flint-` in all `.svelte` files
8. **CSS files** (if any weren't regenerated) — Find/replace `--flint-` with `--flint-`
9. **Bridge CSS** — Rename file and update contents
10. **UI strings** — Update display names in preview apps
11. **Documentation** — Update all prose references
12. **Clean and rebuild**:
    - `cargo clean` (in gpui and jetstream workspace roots)
    - `rm -rf node_modules` in affected packages
    - `bun install`
    - `cargo build`
13. **Rename the project directory**: `mv /Users/betterthanclay/Dev/projects/flint /Users/betterthanclay/Dev/projects/flint`
14. **Verify**: Run `cargo build` and `bun run --cwd packages/svelte/preview dev` to confirm everything works

## Bulk Find/Replace Summary

These are the mechanical replacements that cover ~95% of the work:

| Find | Replace | Scope |
|------|---------|-------|
| `flint-adapter` | `flint-adapter` | Cargo.toml files |
| `flint-composites` | `flint-composites` | Cargo.toml files |
| `flint-events` | `flint-events` | Cargo.toml files |
| `flint-layout` | `flint-layout` | Cargo.toml files |
| `flint-primitives` | `flint-primitives` | Cargo.toml files |
| `flint-style` | `flint-style` | Cargo.toml files |
| `flint-tokens` | `flint-tokens` | Cargo.toml, CSS filenames |
| `flint-workstation` | `flint-workstation` | Cargo.toml files |
| `flint-gpui` | `flint-gpui` | Cargo.toml, effigy.toml |
| `flint-jetstream` | `flint-jetstream` | Cargo.toml, effigy.toml |
| `flint_adapter` | `flint_adapter` | Rust source (*.rs) |
| `flint_composites` | `flint_composites` | Rust source (*.rs) |
| `flint_events` | `flint_events` | Rust source (*.rs) |
| `flint_gpui` | `flint_gpui` | Rust source (*.rs) |
| `flint_layout` | `flint_layout` | Rust source (*.rs) |
| `flint_primitives` | `flint_primitives` | Rust source (*.rs) |
| `flint_style` | `flint_style` | Rust source (*.rs) |
| `flint_tokens` | `flint_tokens` | Rust source (*.rs) |
| `flint_workstation` | `flint_workstation` | Rust source (*.rs) |
| `@flint/` | `@flint/` | package.json, vite.config.ts, imports |
| `--flint-` | `--flint-` | CSS, Svelte style blocks |
| `var(--flint-` | `var(--flint-` | Svelte components |
| `"flint-` | `"flint-` | Rust string literals (element IDs) |
| `flintRelease` | `flintRelease` | package.json custom fields |
| `alias = "flint"` | `alias = "flint"` | effigy.toml |

## Ongoing Visual Pass

After the rename is complete, there is an ongoing GPUI visual parity pass in progress. Key context for continuing that work:

- **Goal**: Make GPUI preview pixel-equivalent to Svelte preview
- **Pattern**: Spec crates define data/tokens, GPUI components in `packages/gpui/components/src/` resolve tokens via `GpuiThemeProvider`
- **Two specimen signatures**: `render(theme: &GpuiThemeProvider)` (static) vs `render(state: &AppState, cx: &mut Context<PreviewRoot>)` (interactive with state)
- **cx.listener pattern**: `cx.listener(|this, val: &T, _w, cx| { ... })` — reference parameter
- **Focus ring pattern**: Approximate Svelte `outline` with `border_color` + `shadow` ring (spread_radius 2px, 28% opacity)
- **Color mixing**: `color_mix(a, b, ratio)` blends ratio% of `a` with (1-ratio)% of `b`
- **GPUI limitations**: No CSS text-transform, letter-spacing, or gradients
- **Handover doc**: `docs/roadmaps/GPUI_VISUAL_PASS_HANDOVER.md` has full component-by-component status
