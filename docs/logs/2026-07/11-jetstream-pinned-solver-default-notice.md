# Notice: Jetstream physics solver default changed (pinned-contact solver)

Created: 2026-07-11
Source: Jetstream consolidation thread (g06.114), cross-repo heads-up
Affects: `packages/jetstream/*` (path-deps on `../jetstream` at HEAD)

---

## What changed upstream

Jetstream's rigid-body engine consolidated on a new solver: the
**pinned-contact solver** (formerly the box3d-parity path) is now the
engine default (`WorldConfig::pinned_solver: true` in
`jetstream-rigidbody`). The legacy solver is deprecated and will be
DELETED shortly, along with its config flags and diagnostics surfaces.

Related renames in `jetstream-rigidbody` (in case any future Poodle code
reaches for them): `box3d_*` identifiers are gone — the solver flag is
`pinned_solver`, debug env vars moved `B3_*` → `RB_*`.

## Impact on Poodle: none expected

- Poodle's Jetstream crates (`adapter`, `components`, `preview`) depend on
  `jetstream-runtime` / `renderer` / `platform` / `input` / `text` /
  `simulation` / `world`. **None of that chain depends on
  `jetstream-rigidbody`**, so the solver change is behaviorally invisible
  to Poodle today.
- Build compatibility verified 2026-07-11: `cargo check` clean for
  `poodle-jetstream`, `poodle-jetstream-components`, and
  `poodle-jetstream-preview` against Jetstream HEAD (commit with the
  default flip).

## What to watch

- If Poodle ever adds physics-backed UI (drag inertia, spring surfaces)
  via `jetstream-rigidbody`, the default solver is the pinned one: soft
  contacts, `substeps >= 1` safe (stability clamp built in), sleeping is
  aggressive (legacy-calibrated thresholds), and contact events surface
  through `world.contact_cache()`.
- The Jetstream tree is shared. During the upcoming legacy-solver
  deletion (consolidation #13) transient breakage in
  `jetstream-rigidbody` is possible; Poodle's dependency chain avoids
  that crate, but a full-workspace `cargo check` from the Poodle side
  after big Jetstream pushes stays the cheap safety net.
