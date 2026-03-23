# Rename: Pug -> Flint

## Context

This project is a multi-platform design system previously called **Pug**. It needs to be renamed to **Flint** due to naming conflicts with the Pug template language (formerly Jade). The rename is comprehensive — it touches Rust crate names, NPM package scopes, CSS custom property prefixes, file names, import paths, config files, documentation, and UI strings.

The project lives at `/Users/betterthanclay/Dev/projects/pug` and should be moved to `/Users/betterthanclay/Dev/projects/flint` after the rename is complete.

## Project Structure

```
pug/
├── package.json              # Root workspace: name "pug"
├── effigy.toml               # Task runner: alias "pug", refs pug-gpui-preview etc.
├── packages/
│   ├── contracts/             # Shared Rust spec crates (platform-agnostic)
│   │   ├── adapter/           # pug-adapter
│   │   ├── composites/        # pug-composites
│   │   ├── events/            # pug-events
│   │   ├── layout/            # pug-layout
│   │   ├── primitives/        # pug-primitives
│   │   ├── style/             # pug-style
│   │   ├── tokens/            # pug-tokens
│   │   └── workstation/       # pug-workstation
│   ├── gpui/                  # GPUI (macOS native) renderer
│   │   ├── adapter/           # pug-gpui
│   │   ├── components/        # pug-gpui-components
│   │   └── preview/           # pug-gpui-preview
│   ├── jetstream/             # Jetstream (game engine) renderer
│   │   ├── adapter/           # pug-jetstream
│   │   ├── components/        # pug-jetstream-components
│   │   └── preview/           # pug-jetstream-preview
│   ├── svelte/                # Svelte web renderer
│   │   ├── composites/        # @pug/svelte-composites
│   │   ├── icons-lucide/      # @pug/icons-lucide
│   │   ├── install-smoke/     # @pug/install-smoke
│   │   ├── primitives/        # @pug/svelte-primitives
│   │   ├── preview/           # @pug/svelte-preview
│   │   └── tokens/            # @pug/svelte-tokens
│   ├── tokens/                # @pug/tokens (token pipeline)
│   └── bridges/
│       └── underlay/          # @pug/bridge-underlay
└── docs/                      # Documentation and research
```

## What Needs to Change

### 1. Rust Crate Names (14 Cargo.toml files)

Every `Cargo.toml` has a `name = "pug-*"` and dependencies like `pug-adapter = { path = "..." }`.

| File | Old Name | New Name |
|------|----------|----------|
| `packages/contracts/adapter/Cargo.toml` | `pug-adapter` | `flint-adapter` |
| `packages/contracts/composites/Cargo.toml` | `pug-composites` | `flint-composites` |
| `packages/contracts/events/Cargo.toml` | `pug-events` | `flint-events` |
| `packages/contracts/layout/Cargo.toml` | `pug-layout` | `flint-layout` |
| `packages/contracts/primitives/Cargo.toml` | `pug-primitives` | `flint-primitives` |
| `packages/contracts/style/Cargo.toml` | `pug-style` | `flint-style` |
| `packages/contracts/tokens/Cargo.toml` | `pug-tokens` | `flint-tokens` |
| `packages/contracts/workstation/Cargo.toml` | `pug-workstation` | `flint-workstation` |
| `packages/gpui/adapter/Cargo.toml` | `pug-gpui` | `flint-gpui` |
| `packages/gpui/components/Cargo.toml` | `pug-gpui-components` | `flint-gpui-components` |
| `packages/gpui/preview/Cargo.toml` | `pug-gpui-preview` | `flint-gpui-preview` |
| `packages/jetstream/adapter/Cargo.toml` | `pug-jetstream` | `flint-jetstream` |
| `packages/jetstream/components/Cargo.toml` | `pug-jetstream-components` | `flint-jetstream-components` |
| `packages/jetstream/preview/Cargo.toml` | `pug-jetstream-preview` | `flint-jetstream-preview` |

**Dependencies within these files** also reference `pug-*` crate names and must be updated.

### 2. Rust Source Code Imports

All `use pug_*::` imports become `use flint_*::`. Rust converts hyphens to underscores for module names, so `pug-adapter` -> `pug_adapter` in code becomes `flint-adapter` -> `flint_adapter`.

Key patterns to find and replace:
- `use pug_adapter` -> `use flint_adapter`
- `use pug_composites` -> `use flint_composites`
- `use pug_events` -> `use flint_events`
- `use pug_gpui` -> `use flint_gpui`
- `use pug_layout` -> `use flint_layout`
- `use pug_primitives` -> `use flint_primitives`
- `use pug_style` -> `use flint_style`
- `use pug_tokens` -> `use flint_tokens`
- `use pug_workstation` -> `use flint_workstation`
- `pug_gpui::` -> `flint_gpui::` (qualified paths in code)
- `pug_primitives::` -> `flint_primitives::` (etc.)

Also check for any string literals referencing `"pug"` in Rust code (e.g., `"pug-accordion"` element IDs — these should become `"flint-accordion"`).

### 3. NPM Package Names (8 package.json files + root)

| File | Old Name | New Name |
|------|----------|----------|
| `package.json` (root) | `pug` | `flint` |
| `packages/tokens/package.json` | `@pug/tokens` | `@flint/tokens` |
| `packages/svelte/tokens/package.json` | `@pug/svelte-tokens` | `@flint/svelte-tokens` |
| `packages/svelte/primitives/package.json` | `@pug/svelte-primitives` | `@flint/svelte-primitives` |
| `packages/svelte/composites/package.json` | `@pug/svelte-composites` | `@flint/svelte-composites` |
| `packages/svelte/preview/package.json` | `@pug/svelte-preview` | `@flint/svelte-preview` |
| `packages/svelte/icons-lucide/package.json` | `@pug/icons-lucide` | `@flint/icons-lucide` |
| `packages/svelte/install-smoke/package.json` | `@pug/install-smoke` | `@flint/install-smoke` |
| `packages/bridges/underlay/package.json` | `@pug/bridge-underlay` | `@flint/bridge-underlay` |

Also update:
- All `"@pug/*"` dependency references within these files
- Any `"pugRelease"` custom config fields -> `"flintRelease"`

### 4. CSS Custom Properties (hundreds of files)

All CSS variables use the `--pug-` prefix. These appear in:

**Generated token CSS files** (in `packages/tokens/artifacts/css/` and `packages/svelte/tokens/src/generated/css/`):
- `pug-tokens.css` -> `flint-tokens.css`
- `pug-theme-light.css` -> `flint-theme-light.css`
- `pug-theme-dark.css` -> `flint-theme-dark.css`
- `pug-theme-loophole-studio.css` -> `flint-theme-loophole-studio.css`
- `pug-control-size-sm.css` -> `flint-control-size-sm.css`
- `pug-control-size-md.css` -> `flint-control-size-md.css`
- `pug-control-size-lg.css` -> `flint-control-size-lg.css`
- `pug-density-comfortable.css` -> `flint-density-comfortable.css`
- `pug-density-compact.css` -> `flint-density-compact.css`

**Inside these files**, every CSS variable declaration and reference:
- `--pug-semantic-color-*` -> `--flint-semantic-color-*`
- `--pug-semantic-size-*` -> `--flint-semantic-size-*`
- `--pug-semantic-space-*` -> `--flint-semantic-space-*`
- etc.

**Svelte component files** (200+ `.svelte` files) reference `--pug-*` in `<style>` blocks via `var(--pug-...)`.

**Bridge CSS**: `packages/bridges/underlay/css/pug-to-underlay.css` -> `flint-to-underlay.css`

**IMPORTANT**: The token pipeline (`packages/tokens/scripts/build-tokens.ts`) generates these CSS files. The prefix is likely configured there — find and update the prefix source so regenerated tokens use `--flint-*`. Updating the generator is more important than manually editing generated output files.

### 5. Token Pipeline / Build Scripts

- `packages/tokens/scripts/build-tokens.ts` — likely contains the `"pug"` prefix used when generating CSS variable names
- `packages/svelte/tokens/src/generated/ts/metadata.ts` — generated TypeScript metadata
- `packages/svelte/tokens/src/generated/ts/index.ts` — generated TypeScript exports
- `packages/tokens/artifacts/ts/index.ts` and `metadata.ts`
- `packages/tokens/artifacts/rust/metadata.rs` — generated Rust token metadata

Search these for `"pug"` string literals that control the CSS variable prefix generation.

### 6. Config Files

**`effigy.toml`** (task runner):
- `alias = "pug"` -> `alias = "flint"`
- All `-p pug-gpui-preview` -> `-p flint-gpui-preview`
- All `-p pug-jetstream-preview` -> `-p flint-jetstream-preview`
- All `-p pug-gpui` -> `-p flint-gpui`

**`packages/svelte/preview/vite.config.ts`**:
- `"@pug/svelte-primitives"` -> `"@flint/svelte-primitives"`
- `"@pug/svelte-composites"` -> `"@flint/svelte-composites"`

**`packages/svelte/tokens/package.json`** exports:
- `"./styles.css": "./src/generated/css/pug-tokens.css"` -> `"./styles.css": "./src/generated/css/flint-tokens.css"`

### 7. UI Strings and Preview App

The GPUI preview app title bar and Svelte preview app header display "Pug" — search for display strings:
- Preview app titles, headers, logos
- `packages/svelte/preview/src/App.svelte` — likely contains "Pug" in the UI
- GPUI preview main window title

### 8. Documentation

All files in `docs/` reference "Pug" in prose. Update:
- `docs/roadmaps/GPUI_VISUAL_PASS_HANDOVER.md`
- `docs/guides/svelte-developer-guide.md`
- `docs/research/**` — various research memos
- Any README.md files
- This rename document itself

### 9. Element IDs in GPUI Components

The GPUI Rust components use `"pug-"` prefixed element IDs:
- `"pug-accordion"`, `"pug-pill"`, `"pug-pill-remove"`, `"pug-number-entry"`, `"pug-input"`, etc.
- These should become `"flint-accordion"`, `"flint-pill"`, etc.
- Search all `.rs` files for string literals containing `"pug-"` or `"pug_"`.

### 10. Lock Files and Build Artifacts

After renaming, these need to be regenerated (not manually edited):
- `bun.lock` — run `bun install` after renaming all package.json files
- `packages/*/node_modules/@pug/` — will be recreated by `bun install`
- `target/` directories — run `cargo clean` then rebuild
- `dist/` directories — rebuild after rename

## Execution Order

1. **Token pipeline source** — Update the CSS variable prefix in `build-tokens.ts` first
2. **Cargo.toml files** (all 14) — Update package names and dependency names
3. **Rust source files** — Find/replace all `pug_` module references and `"pug-"` string literals
4. **package.json files** (all 9) — Update package names, dependencies, exports, custom fields
5. **Config files** — `effigy.toml`, `vite.config.ts`
6. **Regenerate tokens** — Run the token build to regenerate CSS files with `--flint-*` prefix
7. **Svelte components** — Find/replace `--pug-` with `--flint-` in all `.svelte` files
8. **CSS files** (if any weren't regenerated) — Find/replace `--pug-` with `--flint-`
9. **Bridge CSS** — Rename file and update contents
10. **UI strings** — Update display names in preview apps
11. **Documentation** — Update all prose references
12. **Clean and rebuild**:
    - `cargo clean` (in gpui and jetstream workspace roots)
    - `rm -rf node_modules` in affected packages
    - `bun install`
    - `cargo build`
13. **Rename the project directory**: `mv /Users/betterthanclay/Dev/projects/pug /Users/betterthanclay/Dev/projects/flint`
14. **Verify**: Run `cargo build` and `bun run --cwd packages/svelte/preview dev` to confirm everything works

## Bulk Find/Replace Summary

These are the mechanical replacements that cover ~95% of the work:

| Find | Replace | Scope |
|------|---------|-------|
| `pug-adapter` | `flint-adapter` | Cargo.toml files |
| `pug-composites` | `flint-composites` | Cargo.toml files |
| `pug-events` | `flint-events` | Cargo.toml files |
| `pug-layout` | `flint-layout` | Cargo.toml files |
| `pug-primitives` | `flint-primitives` | Cargo.toml files |
| `pug-style` | `flint-style` | Cargo.toml files |
| `pug-tokens` | `flint-tokens` | Cargo.toml, CSS filenames |
| `pug-workstation` | `flint-workstation` | Cargo.toml files |
| `pug-gpui` | `flint-gpui` | Cargo.toml, effigy.toml |
| `pug-jetstream` | `flint-jetstream` | Cargo.toml, effigy.toml |
| `pug_adapter` | `flint_adapter` | Rust source (*.rs) |
| `pug_composites` | `flint_composites` | Rust source (*.rs) |
| `pug_events` | `flint_events` | Rust source (*.rs) |
| `pug_gpui` | `flint_gpui` | Rust source (*.rs) |
| `pug_layout` | `flint_layout` | Rust source (*.rs) |
| `pug_primitives` | `flint_primitives` | Rust source (*.rs) |
| `pug_style` | `flint_style` | Rust source (*.rs) |
| `pug_tokens` | `flint_tokens` | Rust source (*.rs) |
| `pug_workstation` | `flint_workstation` | Rust source (*.rs) |
| `@pug/` | `@flint/` | package.json, vite.config.ts, imports |
| `--pug-` | `--flint-` | CSS, Svelte style blocks |
| `var(--pug-` | `var(--flint-` | Svelte components |
| `"pug-` | `"flint-` | Rust string literals (element IDs) |
| `pugRelease` | `flintRelease` | package.json custom fields |
| `alias = "pug"` | `alias = "flint"` | effigy.toml |

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
