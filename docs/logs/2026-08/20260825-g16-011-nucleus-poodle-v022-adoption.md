# g16.011 — Nucleus Poodle 0.2.2 Adoption

Date: 2026-08-25
Verdict: **complete — public-registry adoption over local Longhorn packages**

Nucleus PR [#1](https://github.com/inflatable-cookie/nucleus/pull/1) merged at
`9b3f67c9c7d57700449ef26b5124d1b092093925`. Nucleus Desktop now pins Poodle
core and Svelte to exact public 0.2.2 while retaining its intentional local
Longhorn packages. The Bun lock carries the published npm integrity hashes and
contains no Poodle 0.1.0, 0.2.1, sibling path, or duplicate runtime. The local
`longhorn-poodle-svelte` peer resolves that same Svelte 0.2.2 identity.

Compatibility work stayed bounded to current public dependencies: settings
tests follow Poodle 0.2.2 labelling, `SurfaceDocument` comes through Longhorn's
public layout export, and the consumer verifier reflects Card 179's absorption
of layout into the Surface crates without admitting hosted-Surface renderer
imports or transfer/windowing crates. The Cargo lock gained the one `rustix`
edge required by current Longhorn.

Independent orchestrator validation passed:

- `effigy check:longhorn-consumer`, including an outside-workspace packed
  Longhorn install against registry Poodle 0.2.2;
- `effigy desktop:check` and `effigy desktop:build`;
- `effigy desktop:test`: 71 Bun tests and 38 Vitest tests;
- the broad headless `effigy qa` board;
- registry-integrity, dependency-graph, and complete lock-diff inspection;
- `git diff --check`.

GitHub exposed no hosted checks. The
[canonical review comment](https://github.com/inflatable-cookie/nucleus/pull/1#issuecomment-5408030177)
and local headless evidence formed the merge gate. Underlay Reference `013` is
now the only remaining first-wave lane before compilation of the rest of the
product estate.
