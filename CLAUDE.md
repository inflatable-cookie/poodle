# Claude Instructions

Follow [AGENTS.md](AGENTS.md) for repository rules, validation, and the Effigy
command surface.

Current architecture:

- [System shape](docs/architecture/001-poodle-system-shape.md)
- [Working rules](docs/contracts/001-working-rules.md)
- [Component contracts](docs/contracts/components/README.md)
- [Product guardrails](docs/architecture/product-guardrails.md)

Native components have one implementation in `packages/render`. GPUI and
Jetstream backends interpret the resulting `poodle-node` tree; do not create a
second runtime-specific component tier.

Use the repository's [internal writing style](docs/policy/internal-writing-style.md)
for internal docs and normal thread replies.
