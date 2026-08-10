# Contributing to Poodle

Poodle is contract-first and cross-runtime. A change is complete when its
public behavior, implementations, documentation, and relevant evidence agree.

The project is currently a pre-1.0 source preview. Discuss large API, package,
or component additions before investing in a full implementation.
Participation is governed by the [code of conduct](CODE_OF_CONDUCT.md).

## Set Up the Repository

Install [Effigy](https://github.com/inflatable-cookie/effigy), Bun 1.3.14, and
Rust 1.95, then run:

```sh
bun install
effigy tokens:build
effigy icons:build
effigy tasks
```

Effigy is the supported task surface. Use `effigy doctor` when task routing or
the local environment is unclear.

## Choose the Right Owner

- Reusable component semantics belong in Poodle.
- Product vocabulary, data access, routing, persistence, and workflow policy
  stay in the application.
- Shared web state and CSS belong in `poodle-core`.
- Shared native component recipes belong in `poodle-render`.
- GPUI and Jetstream backends own runtime interpretation only.
- Underlay-facing APIs remain Underlay-owned behind the bridge.

Read the [product guardrails](docs/architecture/product-guardrails.md) and
[system architecture](docs/architecture/001-poodle-system-shape.md) before
changing a package boundary.

## Change a Component

1. Update the contract under `docs/contracts/components/`.
2. Update shared specs or headless behavior where required.
3. Implement the behavior in each affected web or native shared layer.
4. Add focused tests and preview coverage.
5. Update parity or accessibility evidence when the change affects it.

Contracts define observable behavior. Framework-specific syntax may differ,
but inputs, state transitions, events, keyboard behavior, layout intent, and
token use should remain aligned.

## Generated Files

Do not edit generated token, icon, parity, or accessibility artifacts by hand.
Use the owning Effigy task. Token changes begin in `packages/tokens/schema/`;
default icon changes begin in
`packages/core/src/icons/default-icons.json`:

```sh
effigy tokens:build
effigy icons:build
```

## Validate

Run the narrow task for the code you changed, then an appropriate broader gate.
For documentation work:

```sh
effigy docs:check
effigy docs:contract-drift
effigy docs:spec-drift
git diff --check
```

For broader implementation work, inspect `effigy test --plan` and use
`effigy ci` or the relevant native/visual task when its dependencies are
available.

For public web package, export, or dependency changes, verify the actual
tarballs in a clean consumer:

```sh
effigy test:web-pack-install
```

Before a public release or after dependency-source changes, run the security
hygiene, advisory, and license gates. They require
[`cargo-deny`](https://github.com/EmbarkStudios/cargo-deny):

```sh
effigy audit:security
effigy audit:licenses
```

## Submit a Change

Keep a pull request focused on one coherent outcome. Explain:

- the operator or maintainer problem
- the contract or ownership decision
- affected runtimes
- validation performed
- known or intentional differences

Do not include unrelated generated timestamps or local preview artifacts.

Report vulnerabilities privately according to [SECURITY.md](SECURITY.md), not
through a public issue.
