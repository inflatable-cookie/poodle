# poodle-codegen

Deterministic emitter turning a validated [`poodle-ir`](../contracts/ir/)
model into committed TypeScript artifacts — the compiler boundary of
[`docs/specs/063-rust-authored-component-and-scene-ir.md`](../../docs/specs/063-rust-authored-component-and-scene-ir.md)
(`IR-07` deterministic generation, `IR-11` pilot gate).

Nothing depends on this crate. It is a tool (`publish = false`, channel
`internal`, stability `internal-tooling`, per `g13-b003` R1).

## Invocation

```sh
cargo run --manifest-path packages/codegen/Cargo.toml --bin poodle-codegen -- \
  packages/codegen/fixtures/synthetic-model.json \
  --out packages/codegen/generated
```

- `FIXTURE` — repo-relative path of the serialized `IrModel` JSON. The
  fixture is validated before emission; any finding refuses the run.
- `--out` — committed artifact root; each target owns a subdirectory
  (`ts/` for TypeScript).
- `--check` — regenerate in memory and byte-compare against the committed
  files. Fails on drift, whitespace-only differences (classified separately),
  missing files, and stale orphans. **Read-only by construction**: the check
  path contains no write call.
- Exit codes: 0 clean, 1 drift/validation/IO failure, 2 usage.

## Design

- **The emitter owns every byte** (ruling R2). No formatter runs over the
  output; the generated header is a pure function of the source path with no
  timestamp, absolute path, or machine value.
- **Deterministic ordering**: top-level collections sort by id; prop and
  member order stay as authored (the contract's own order).
- **One target today**: TypeScript types for each component's prop surface —
  `shared-types.ts`, `<component-id>.ts`, `index.ts`. Shared-typed props
  with a permitted subset emit the subset union inline (g13-b003 R6.2);
  web-only props are marked in the doc comment.
- **Check/write split** (ruling R3): `ir:build` writes, `ir:check` compares.
  Gates compose only `*:check` selectors; `ir:build` is never part of a gate.
- The remaining targets (JSON schema, registry, conformance vectors, docs
  fragments) are a follow-up card; the machinery here is target-independent.

## Selectors

- `effigy ir:build` — regenerate the committed artifacts (write mode).
- `effigy ir:check` — read-only drift gate.

## Tests

`cargo test --manifest-path packages/codegen/Cargo.toml` covers byte-identical
double generation, drift detection with whitespace-only classification, the
read-only property, stale-orphan detection, panic-free malformed/invalid
input, and `tsc --noEmit --strict` over the emitted TypeScript.
