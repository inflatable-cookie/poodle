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
  shared renderer-agnostic crates such as `poodle-primitives`, `poodle-composites`, and `poodle-tokens`
- `packages/svelte/*`
  published web packages such as `@poodle/svelte-primitives`, `@poodle/svelte-composites`, `@poodle/svelte-tokens`, and the docs or preview app
- `packages/gpui/*`
  native GPUI adapter, renderable components, and preview app
- `packages/jetstream/*`
  Jetstream adapter, renderable components, and preview app
- `packages/bridges/underlay`
  token and wrapper bridge for Underlay-hosted apps
- `packages/tokens`
  canonical token schema and artifact generation pipeline

## Canonical Docs

Start with these:

1. [docs/vision/001-poodle-vision.md](/Users/betterthanclay/Dev/projects/poodle/docs/vision/001-poodle-vision.md)
2. [docs/architecture/001-poodle-system-shape.md](/Users/betterthanclay/Dev/projects/poodle/docs/architecture/001-poodle-system-shape.md)
3. [docs/roadmaps/README.md](/Users/betterthanclay/Dev/projects/poodle/docs/roadmaps/README.md)
4. [docs/specs/README.md](/Users/betterthanclay/Dev/projects/poodle/docs/specs/README.md)

## Local Workflow

Install dependencies and generate token artifacts:

```sh
bun install
bun packages/tokens/scripts/build-tokens.ts
```

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

The repository root is `/Users/betterthanclay/Dev/projects/poodle`.
Current package and crate namespaces use `poodle` and `@poodle/*`.
Historical `pug` and `flint` references should be treated as migration leftovers unless they appear in explicit rename handoff docs.

## Next Task

Do one broad downstream dependency sweep so consuming repos move from `flint` to `poodle` in package scopes, crate imports, Git remotes, and local path references.
