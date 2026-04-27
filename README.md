# Poodle

Poodle is a multi-renderer design system with one shared contract surface and multiple runtime implementations.
The repo currently ships and validates:

- shared Rust contract crates for tokens, layout, style, primitives, composites, and workstation surfaces
- a Svelte package set and browser preview surface
- a GPUI native adapter, component library, and preview app
- a Jetstream adapter, component library, and preview app
- an Underlay bridge for host adoption without leaking Poodle-specific APIs into app code

## Repo Shape

Key package groups:

- `packages/contracts/*`
  shared renderer-agnostic crates such as `poodle-specs`, `poodle-workstation`, and `poodle-tokens`
- `packages/svelte/*`
  published web packages such as `@poodle/svelte`, `@poodle/svelte-tokens`, `@poodle/icons-lucide`, and the docs or preview app
- `packages/gpui/*`
  native GPUI adapter, renderable components, and preview app
- `packages/jetstream/*`
  Jetstream adapter, renderable components, and preview app
- `packages/bridges/underlay`
  token and wrapper bridge for Underlay-hosted apps
- `packages/tokens`
  canonical token schema and artifact generation pipeline

## Consuming Poodle

Start with the developer guide for your target runtime:

- **Svelte** — [docs/guides/svelte-developer-guide.md](docs/guides/svelte-developer-guide.md)
- **GPUI** — [docs/guides/gpui-developer-guide.md](docs/guides/gpui-developer-guide.md)
- **Jetstream** — [docs/guides/jetstream-developer-guide.md](docs/guides/jetstream-developer-guide.md)

All components are defined by contracts in [docs/contracts/components/](docs/contracts/components/).
The contract is the source of truth for every implementation.

## Canonical Docs

Internal planning and architecture hierarchy:

1. [docs/vision/001-poodle-vision.md](docs/vision/001-poodle-vision.md)
2. [docs/architecture/001-poodle-system-shape.md](docs/architecture/001-poodle-system-shape.md)
3. [docs/roadmaps/README.md](docs/roadmaps/README.md)
4. [docs/specs/README.md](docs/specs/README.md)

## Local Workflow

Install dependencies and generate token artifacts:

```sh
bun install
bun packages/tokens/scripts/build-tokens.ts
```

`bun install` at the repo root is the canonical JS hydration step. Mounted
consumer repos should hydrate the root workspace, not run separate
package-local installs under `packages/svelte/*` unless a package explicitly
documents that requirement.

Common repo tasks:

```sh
effigy health
bun run --cwd packages/svelte/preview dev
cargo run -p poodle-gpui-preview --manifest-path packages/gpui/preview/Cargo.toml
cargo run -p poodle-jetstream-preview --manifest-path packages/jetstream/preview/Cargo.toml
```

The default validation pass is `effigy health`.
That runs docs lint, parity and accessibility artifact generation, and the Svelte production build.

## Naming

Current package and crate namespaces use `poodle` and `@poodle/*`.
Historical `pug` and `flint` references should be treated as migration leftovers unless they appear in explicit rename handoff docs.

## Next Task

Recover the live `g10` queue from current evidence, then compile the next
bounded Jetstream or component-overhaul milestone before more freeform work
continues.
