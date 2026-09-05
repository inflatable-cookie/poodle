# g16.109 — v0.3.0 Consumer Adoption Wave

Status: ready — 15 sibling-repository lanes in two tiers; amended
2026-09-05 after the operator caught a sequencing defect: Underlay apps
consume Underlay by git tag, so tier 2 waits for an Underlay release, not
for a merged pin
Type: consumer adoption — sibling repositories; Poodle unchanged
Opened: 2026-09-05
Depends on: `g16.097` complete (npm `latest` core and Svelte `0.3.0`, tag
`v0.3.0` at `85609d941`, publish run `33952493234`)
Governing refs: `../../release-notes/0.3.0.md` (breaking: HistoryEntry v3 and
five rejection codes; root markdown-family imports move to `/markdown`),
`../g15/055-consumer-adoption-inventory.md` (ordering precedent),
`../g15/062-longhorn-poodle-v022-adoption.md` and `065-nucleus-poodle-v022-adoption.md`
(lane shape precedent), each repository's `AGENTS.md`
Consumer evidence: `../../triage/20260904-151947-consumer-sweep-intake.md` —
ten open consumer papercuts are already fixed on Poodle `main` and close
when pins move
Dispatch manifest: `../dispatch.md`

## Outcome

Every consumer resolves exact public Poodle core and Svelte `0.3.0` from the
registry with a lockfile diff limited to Poodle, absorbs the three documented
breaks where they apply, and passes its own headless board. Longhorn's Rust
`poodle-specs` pin moves to tag `v0.3.0`.

## Per-Repository Work

| Tier | Repository | Manifests | Extra work beyond the pin | Board |
| --- | --- | --- | --- | --- |
| 1 | Longhorn | root `package.json`; `packages/longhorn-poodle-svelte` peer; `crates/longhorn-poodle/Cargo.toml` `poodle-specs` tag `v0.2.2` → `v0.3.0` | prove the Svelte adapter peer converges on one `0.3.0` identity | `effigy qa` |
| 1 | Underlay | root `package.json` (Svelte) | 2 files import markdown-family components from the package root → `/markdown`; then **cut release `v0.9.8`** (below) | `effigy qa`, then `effigy release` gates |
| 1 | Soundcheck Library | root `package.json` (Svelte) | none | `effigy qa` |
| 2 (after Underlay `v0.9.8`) | Acowtancy | `apps/cream`, `apps/dairy` | move `@inflatable-cookie/underlay` from `#v0.9.7` to `#v0.9.8`; 19 files import markdown-family components from the root → `/markdown` | repo check/test selectors |
| 2 (after Underlay `v0.9.8`) | Compli Me | `apps/admin`, `apps/front` | Underlay ref → `#v0.9.8` | repo check/test selectors |
| 2 (after Underlay `v0.9.8`) | Contact Patch | `apps/cp-admin` | Underlay ref → `#v0.9.8` | repo check/test selectors |
| 2 (after Underlay `v0.9.8`) | Songsprout | `apps/bloom`, `apps/greenhouse` | Underlay ref → `#v0.9.8` | repo check/test selectors |
| 2 (after Underlay `v0.9.8`) | Underlay Reference | `apps/acme-admin`, `apps/acme-front` | Underlay ref → `#v0.9.8`; 1 root markdown-family import → `/markdown` | repo check/test selectors |
| 2 (after Longhorn) | Bovine Accelerator Desktop | root `package.json` | 81 files use `poodle-svelte/<Name>.svelte` subpaths; those remain exported and resolve to `dist/`, so no rewrite; prove one representative resolves | `effigy qa` |
| 2 (after Longhorn) | Figmatic | `studio/package.json` | none known; note the Select ghost-variant and licence-type papercuts close | `effigy qa` |
| 2 (after Longhorn) | Finch | `app-tauri/package.json` | none known | repo check/test selectors |
| 2 (after Longhorn) | Jetstream | `editor-ui/package.json` | none known; Rust path deps unchanged | repo check/test selectors |
| 2 (after Longhorn) | Nucleus | `apps/desktop/package.json` | none known; `check:longhorn-consumer` must pass | desktop check/build/test |
| 2 (after Longhorn) | Loophole | `apps/desktop/package.json` | HistoryCenter v3: `pages`/`continuationCount`, host-fed continuation ops, and map deletion failures onto `AlreadyAtTarget`, `UnknownEntry`, `StaleHistory`, `ProtectedEntry`, `DeletionUnavailable` (`src/renderer/history/{map.ts,hub.svelte.ts}`, `LoopholeShell.svelte`); 1 subpath import stays | `effigy qa` |
| 2 (after Longhorn and Soundcheck Library) | Soundcheck | root `package.json` | none known | repo check/test selectors |

Composer was removed by the operator and is not a consumer.

## Foundation Release Gates

- **Underlay.** Its nine consumer apps depend on
  `@inflatable-cookie/underlay` as `git+ssh://…/underlay.git#v0.9.7`. A
  merged pin on Underlay `main` reaches no app. After the Underlay pin PR
  merges, the coordinator cuts `v0.9.8` in the Underlay checkout with
  Underlay's own release flow (`effigy release` with its gates:
  `check:release-version-sync`, `validate`, `rust:clippy`, `rust:check`;
  version lives at `Cargo.toml` `workspace.package.version`; tag
  `v{version}`; changelog entry under `[0.9.8]` naming the Poodle `0.3.0`
  dependency and the `/markdown` import moves), pushes the tag, and verifies
  `git ls-remote --tags origin v0.9.8`. Operator authorization for this
  release mutation: 2026-09-05 ("you've started tier 2 without cutting a new
  Underlay release"). Underlay's `0.x` rule: the Poodle bump is a dependency
  change, so patch `0.9.8` unless the worker finds an Underlay public
  surface change, in which case stop and report before tagging.
- **Longhorn.** Its six consumer apps use `file:../longhorn/packages/*`
  links, so a merged pin on Longhorn `main` is what they consume, provided
  the sibling checkout is at or after that merge. No tag is needed for the
  web packages; the Rust `poodle-specs` tag move is inside the Longhorn lane.
- **Soundcheck Library.** Soundcheck consumes it by `file:` link; no tag.

Tier 2 gates are therefore: Underlay apps → `v0.9.8` exists on origin;
Longhorn apps → Longhorn pin PR merged and the sibling checkout at that
commit; Soundcheck → both.

## Lane Rules (every repository)

- Pin exact `0.3.0` for `@inflatable-cookie/poodle-core` and
  `@inflatable-cookie/poodle-svelte` wherever declared; regenerate the lock
  with the repository's Bun version; the lock diff contains no unrelated
  upgrade.
- Resolve from the public registry only. No sibling path, `file:`, or
  duplicate Poodle resolution; prove with the repository's dependency status
  command (`effigy deps status bun` where available) and a grep of the lock.
- Repair only compatibility fallout the bump causes, in the consumer's own
  code, as the release notes prescribe. No shim, no alias, no re-export of
  removed root names.
- Do not edit Poodle, Longhorn (except in the Longhorn lane), or any other
  sibling. Do not launch a desktop app or run native proofs.
- One PR per repository against its `main`, opened by the worker; the Poodle
  coordinator reviews and merges, as in the `v0.2.2` wave.
- Close the consumer's own `PAPERCUTS.md` entries the bump resolves, citing
  the Poodle release note line.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| One identity | a second Poodle version or a `file:` source in the lock | lock grep and dependency status output in the PR |
| Breaks absorbed, not shimmed | a re-export of a removed root name | reviewer rejects |
| Foundations first | an app lane merged before its foundation is consumable | coordinator gate: Underlay apps wait on tag `v0.9.8`; Longhorn apps on the merged pin and sibling checkout |
| No duplicate Poodle through Underlay | an app on Underlay `#v0.9.7` with Poodle `0.3.0` | lock shows two Poodle versions; lane red |
| Board green | the repository's board red | PR carries the board transcript |
| Papercuts closed honestly | an entry closed without the fix being in 0.3.0 | reviewer checks against the release note |

## Stop Conditions

Stop a lane, not the wave, when the bump exposes a consumer defect that is
not compatibility fallout, or when a board is red for an unrelated reason.
Report it; the other lanes continue. Escalation owner: Chatterbox.

## Continuation

When the wave completes, re-run the consumer sweep and expect the ten
"fixed on main" entries closed. Poodle's README and generation index record
the adoption count.
