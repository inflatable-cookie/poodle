# Poodle

Poodle is a contract-first design system for web and native applications. It
provides the same component semantics, design tokens, themes, and interaction
rules across Svelte, React, GPUI, and Jetstream.

Poodle is intended for teams that need one UI language across several
renderers. Applications choose the runtime package they need; they do not need
to understand the other implementations.

> **Project status:** Poodle is pre-1.0. `@inflatable-cookie/poodle-core` and
> `@inflatable-cookie/poodle-svelte` publish to npm on the preview channel;
> `@inflatable-cookie/poodle-react` is packed and certified but stays
> source-only; the Rust crates are source/tag distribution and are not on
> crates.io. Preview means pre-release: breaking changes may ship in `0.x`
> minor releases and no `stable` channel exists yet, so pin an exact version
> and read the [release notes](docs/release-notes/README.md) before upgrading.

## Choose a Runtime

| Runtime | Use | Status | Start here |
| --- | --- | --- | --- |
| Svelte 5 | Web applications | Preview | [Svelte developer guide](docs/guides/svelte-developer-guide.md) |
| React 18+ | Web applications | Experimental | [React package guide](packages/react/components/README.md) |
| GPUI | Native Rust applications | Preview | [GPUI developer guide](docs/guides/gpui-developer-guide.md) |
| Jetstream | Jetstream applications | Deferred integration | [Jetstream developer guide](docs/guides/jetstream-developer-guide.md) |

The web packages are:

- `@inflatable-cookie/poodle-core` for framework-free state, tokens, styles,
  and the default icon adapter
- `@inflatable-cookie/poodle-svelte` for Svelte components
- `@inflatable-cookie/poodle-react` for React components

## How Poodle Works

Every component starts with a renderer-neutral contract. The contract defines
its inputs, states, events, accessibility behavior, layout, and token usage.
Each runtime implements that contract using the same generated token data.

```text
component contracts + DTCG tokens
             |
       shared semantics
        /           \
Svelte / React    poodle-render (Rust)
                       |
                 poodle-node tree
                  /           \
          GPUI backend    Jetstream backend
```

This split keeps parity focused on what operators and users can observe:
meaning, state, behavior, keyboard support, and token usage. Rendering details
remain native to each target.

### Tokens and themes

Poodle's W3C DTCG token schema generates CSS, TypeScript, and Rust artifacts.
Themes, density, and control size are independent axes. Web applications apply
them with inherited attributes:

```html
<main data-theme="eclipse" data-density="default" data-control-size="sm">
  <!-- Poodle components inherit this configuration. -->
</main>
```

The available themes are Iceberg, Eclipse, Graphite, Midnight, Nord, Rose,
Forest, Solarized, Hornet, Cobalt, Clay, and Meadow. Density supports compact,
default, and comfortable modes; control size runs from `xs` through `xl`.

### Icons

Lucide is the default icon source, not a built-in general catalogue. Poodle
ships only the small Lucide set required by its own components. An application
that needs more icons owns a list and generates a scoped module at build time:

```sh
bun x poodle-icons --names icons.json --out src/icons.generated.ts
```

This keeps the default dependency light while preserving familiar Lucide icon
names. See the [Svelte icon setup](docs/guides/svelte-developer-guide.md#icons)
for the provider pattern.

### Application boundaries

Poodle owns reusable primitives, composites, and general workstation shells.
Product-specific screens and domain widgets stay in the application. Underlay
integrations live behind Underlay-owned adapters so application code does not
need to know that Poodle is underneath.

## Explore the Repository

```text
packages/core/              shared web behavior, tokens, styles, and icons
packages/svelte/            Svelte package and preview
packages/react/             React package and preview
packages/contracts/         renderer-neutral Rust contracts and node model
packages/render/            shared Rust component implementation
packages/gpui/              GPUI adapter, node backend, and preview
packages/jetstream/         Jetstream adapter and preview
packages/tokens/            canonical token schema and generated artifacts
docs/contracts/components/  component contracts
docs/guides/                operator and integration guides
```

The [documentation index](docs/README.md) separates operator guides from
architecture, contracts, and project history.

## Work on Poodle Locally

Poodle uses [Effigy](https://github.com/inflatable-cookie/effigy) as its task
runner. Install Effigy, Bun 1.3.14, and Rust 1.95 before setting up the
repository.

```sh
bun install
effigy tokens:build
effigy icons:build
effigy docs:dev
```

Useful preview tasks:

```sh
effigy svelte:preview
effigy react:preview
effigy gpui:preview
effigy jetstream:preview
```

The Jetstream preview is an opt-in paired-repository integration. Normal
Poodle worktrees and `effigy qa` do not require a sibling Jetstream checkout.
Use `effigy qa:jetstream` only in a prepared paired workspace.

Before submitting a change, run the checks relevant to it. The broad repository
check is:

```sh
effigy qa
```

Use `effigy tasks` to see narrower validation and build tasks.

## Documentation

- [Documentation index](docs/README.md) — operator paths and project reference
- [System architecture](docs/architecture/001-poodle-system-shape.md) — package
  boundaries and renderer flow
- [Token and package architecture](docs/architecture/002-token-system-and-package-layout.md)
- [Component contracts](docs/contracts/components/README.md) — normative
  component behavior
- [Application recipes](docs/guides/README.md) — reusable composition patterns
- [Contributing](CONTRIBUTING.md) — ownership rules and validation workflow
- [Code of conduct](CODE_OF_CONDUCT.md) — participation expectations and
  private reporting
- [Security policy](SECURITY.md) — private vulnerability reporting

Roadmaps, logs, specs, and research are kept in the repository for contributors
and historical traceability. They are not required reading for adopting Poodle.

## License

Poodle is available under the [MIT License](LICENSE). Report security issues
privately according to the [security policy](SECURITY.md).

## Support

Use [GitHub issues](https://github.com/inflatable-cookie/poodle/issues) for
bugs and focused feature requests. For general or private project questions,
email [tom@inflatablecookie.com](mailto:tom@inflatablecookie.com). Report
security issues only through the private route in [SECURITY.md](SECURITY.md).
