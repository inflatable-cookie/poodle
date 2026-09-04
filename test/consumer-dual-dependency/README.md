# Downstream dual-dependency proof (g16.005)

One question: can an ordinary crates.io consumer depend on `gpui` **and** on
Poodle's GPUI node backend, and pass GPUI values between them?

Published `v0.2.1` could not. It resolved `gpui` from the
`inflatable-cookie/zed` fork so an internal capture tool could reach
unpublished headless-renderer APIs. Cargo treats a Git source and a registry
source as different crate identities, so a consumer on crates.io `gpui`
received two incompatible sets of GPUI types. Longhorn's prototypes proved it
during `g16.002` adoption.

```sh
effigy drift:gpui-consumer-identity
```

## What runs

`consumer/` is written the way a real consumer would write it: it declares
`gpui = "0.2.2"` for itself, depends on `poodle-gpui-node-backend` by path,
and threads GPUI values across the boundary in both directions —
`to_gpui` → `gpui::AnyElement`, `color` → `gpui::Hsla`, `focus_handle_for` →
`gpui::FocusHandle`, `bounds_for` → `gpui::Bounds<Pixels>`, and a Poodle
element composed into a tree the consumer builds with its own `gpui`. There is
no `[patch]`, no `[replace]`, and no override: the proof is that none is
needed.

`run.ts` stages the crate in a temporary directory with the `path`
dependencies rewritten to absolute paths, so no lockfile and no target
directory ever land in this checkout. It then asserts:

1. the staged manifest declares its own `gpui` and carries no override;
2. the consumer compiles;
3. the resolved lockfile contains exactly one `gpui`, and every `gpui*` crate
   comes from the registry;
4. the resolved graph enables `tinyvec`'s `std` feature — tinyvec `1.13.0`
   cannot compile its alloc-only path (`cannot find macro vec`), so an
   alloc-only tinyvec in the fresh resolution is the g16.092 break;
5. **negative control** — the same crate with one deliberately wrong GPUI type
   annotation fails to compile, with a type mismatch. A proof that cannot fail
   proves nothing.

Compilation reuses a stable target directory under the system temp directory,
so repeat runs are seconds rather than a cold GPUI build.

This is a type-identity proof. It opens no window, renders nothing, and needs
no window server or Screen Recording permission, so it runs on the ordinary
headless native board.

## Related

- Source policy: `scripts/repository-security-policy.ts` (`registryOnlyCrates`)
  rejects a Git-sourced `gpui`/`gpui_platform` in any active manifest or lock,
  and `deny.toml` admits no Git source at all.
- Card: `docs/roadmaps/g16/005-gpui-cratesio-recovery.md`.
