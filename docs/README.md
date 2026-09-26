# Poodle — current state

Poodle is pre-1.0. `@inflatable-cookie/poodle-core` and
`@inflatable-cookie/poodle-svelte` publish to npm on the preview channel;
`@inflatable-cookie/poodle-react` is packed and certified but stays
source-only; the Rust crates are source/tag distribution. Preview means
breaking changes may ship in `0.x` minor releases, and no `stable` channel
exists yet. See the [release notes](release-notes/README.md).

What works today: the web pair (Svelte and React) covers the public component
set, including the web-only CodeEditor, RichTextEditor, RichTextRenderer and
MarkdownRenderer admissions. Every portable component constructs in GPUI, but
functional GPUI parity is partial: the
[GPUI functionality census](evidence/gpui/gpui-functionality-census.md) and the
[Nucleus parity ledger](evidence/nucleus/parity-evidence-ledger.md) say which
capabilities have mounted proof. Closing that gap is the product direction; see
[vision](knowledge/vision.md#direction).

## Adopt Poodle

- [Svelte developer guide](guides/svelte-developer-guide.md) — package setup,
  themes, icons, components, and application integration
- [React package guide](../packages/react/components/README.md) — current React
  surface and experimental-package constraints
- [GPUI developer guide](guides/gpui-developer-guide.md) — Rust contracts,
  themes, node rendering, and the GPUI backend
- [Jetstream developer guide](guides/jetstream-developer-guide.md) — deferred
  paired integration for Rust contracts, node rendering, and conversion
- [Application pattern recipes](guides/README.md) — forms, lists, dialogs,
  media workflows, and admin shells
- [Component contracts](contracts/components/README.md) — the source of truth
  for public inputs, states, events, accessibility, layout, and token usage

Contracts describe observable behavior. Framework and engine implementation
details may differ when the contract permits it.

## By topic

- Knowledge index: [knowledge/README.md](knowledge/README.md)
- Vision: [knowledge/vision.md](knowledge/vision.md)
- Architecture: [knowledge/architecture/](knowledge/architecture/README.md)
- Specs: [knowledge/specs/](knowledge/specs/README.md)
- Working rules and release: [knowledge/contracts/](knowledge/contracts/README.md)
- Generated evidence: [evidence/](evidence/README.md)
- Open leads: [triage/](triage/README.md)

## Run the docs locally

```sh
bun install
effigy docs:dev
```

`docs:dev` is `svelte:preview`: it rebuilds package distributions before Vite.
Use `svelte:run` only when you already trust the ignored dist trees. Validate
documentation changes with `effigy docs:check`.

## What's next

See [plan.md](plan.md).
