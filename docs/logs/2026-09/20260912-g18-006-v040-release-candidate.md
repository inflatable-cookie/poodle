# g18.006 — v0.4.0 web editor release candidate

Status: candidate prepared — npm production-path proof green; independent
review follows
Date: 2026-09-12
Branch: `ns-17ac3fee-de90-4b32-9672-1134770bb086`
Card: `docs/roadmaps/g18/006-v040-web-editor-release-and-desktop-unblock.md`
Handoff: `docs/handoffs/20260911-075648-g18-006-v040-web-editor-release.md`
Base: pushed `main` at `a68730de6ce770d481d4baf5d707b5d5339abe2c` (g18.031
closeout)
Frozen candidate commit: `a797ce413427b3bfd447d75341f99c62c4665628`
Governing refs: `docs/contracts/001-working-rules.md`,
`docs/specs/022-packaging-versioning-and-release-channel-rules.md`,
`docs/specs/044-deprecation-change-control-and-release-channel-operations.md`,
`docs/specs/070-compiled-web-distribution-contract.md`,
`packages/release-manifest.json`, `packages/release-operations.json`

## Outcome

The immutable Poodle `0.4.0` preview-channel release candidate is prepared on
the retained Queue branch: every lockstep manifest, intra-repository
requirement, tracked lock, changelog entry, release note, generated version
stamp and both evidence cohorts bind one frozen commit. No tag, publication,
workflow dispatch, registry mutation, Desktop change or stable-channel claim
exists. `g18.009` owns release mutation after this candidate merges.

## Version set

`0.4.0` across the whole repository lockstep set:

- root `package.json` (`poodle`, private), `@inflatable-cookie/poodle-core`,
  `@inflatable-cookie/poodle-svelte`, private
  `@inflatable-cookie/poodle-react` (`private: true` preserved);
- all 17 Cargo manifests and every intra-repository `poodle-*` requirement;
- both tracked GPUI locks (`packages/gpui/node-backend/Cargo.lock`,
  `packages/gpui/preview/Cargo.lock`);
- `bun.lock` workspace metadata (version and intra-repo specifier leaves).

The `release.yml` branch-dry-run lockstep law was reproduced locally: all 17
crates and the three web manifests report `0.4.0`, status `0`. Internal
preview/tooling TypeScript packages remain `0.0.0`; React stays private and
unpublished; Rust crates remain source/tag distribution.

`bun.lock` cannot be refreshed by `bun install` for a workspace version bump
(the documented `PAPERCUTS.md` friction: `install`, `--force`, and
`--lockfile-only` all leave the `workspaces` block stale). The five
version/specifier leaves were updated to exactly what a regeneration produces,
matching the shape the published `0.3.0` candidate used.

## Candidate identity and lineage

| Identity | Value |
| --- | --- |
| Candidate commit | `a797ce413427b3bfd447d75341f99c62c4665628` |
| Base (pushed main) | `a68730de6ce770d481d4baf5d707b5d5339abe2c` |
| g18.027 frozen head (ancestor) | `31d529a6f0dd4dc632b11010cf1e16a29e6eb80b` |
| g18.002 editor merge (ancestor) | `308fa52c5cd68d9c776f320c368e4fb0896e4d4d` |
| g18.003 rich text merge (ancestor) | `fb0b73732b5c0a2a9361fddd75962eccd2710b0f` |
| g18.004 Tabs merge (ancestor) | `ed6ed66050c5ba8bf62aaf27eee795ca5be052fa` |
| g18.008 preview admission (ancestor) | `998b6ddc69f94e405b515f6bddd682a2e8916ea5` |

Package trees (`git rev-parse <commit>:<path>`):

| Path | `a797ce413` |
| --- | --- |
| `packages/core` | `48c62d9afa0f8e0a59a61032f56288c5d00c2627` |
| `packages/svelte/components` | `e1c24eb3b81300155977d3448fa7e48a5fc00041` |
| `packages/react/components` | `530bf9ce8f4b010accca728a69668900e2b15cc0` |
| `packages/tokens` | `742ac9c846fbab1e7e3c96bff973bf9ec5b1aa49` |
| `packages/contracts` | `f810cccf6e79edd6d6400242089e91bde3e663ec` |
| `packages/render` | `d7bd58d6d3cb459fedd4cd5ac9b6edb4d6ef14a8` |
| `packages/release-manifest.json` | `ec0148b2b350aa4f8e85d55267ab08c3b365e596` (unchanged) |
| `packages/release-operations.json` | `60bc2df381fbc47cd0d99acf1caf73266e5ea6eb` (unchanged) |

The release manifest and operations files are byte-identical to the g18.027
frozen audit, so the admitted package set, channels and change-control rules
did not move. The g18.002 editor-bearing trees remain lineage evidence, not
the final trees.

## Release-input freeze

One commit (`a797ce413`) changes the complete closed release-input set:

- `CHANGELOG.md` — new `[0.4.0] - 2026-09-12` section (breaking Slider and
  RangeSlider presentation replacement with the accepted migration text, the
  eighteen removed recipe hooks with successors, safe-by-default markdown
  preview, Rust source/tag breaks, additive editor entries, tokens and icons,
  behavioral rows) plus the `[0.4.0]` reference link;
- `docs/release-notes/0.4.0.md` — full preview-channel note carrying the
  g18.027 freeze report tables and the downstream checklist;
- `docs/release-notes/README.md` — index entry;
- the four JS manifests, the 17 Cargo manifests and the two tracked locks;
- `bun.lock`.

`CHANGELOG.md: valid` under the repaired Keep a Changelog grammar, and
`effigy docs:lint` is green. The 45 generated version stamps produced by
`effigy ir:build` and `effigy catalogue:build` live in the same frozen commit:
the Nucleus source pin binds `packages/gpui/preview`, `packages/gpui/adapter`,
`packages/render` and `packages/contracts` wholesale, so the emitted stamps
are part of the source identity the receipts name.

## In-lane receipt-test repair

The `g18.030` lock-provenance laws planted `"0.4.0"` into a disposable copy of
the committed lock. Once the candidate lock *is* `0.4.0`, every replacement was
a no-op and all ten laws failed with a misleading fixture error, which failed
the native lane and blocked the mandatory Nucleus emission.

Repair, confined to the `#[cfg(test)] mod receipt_lock_tests` module of
`packages/gpui/preview/src/nucleus_receipts.rs`:

- `PLANTED_RELEASE_VERSION: &str = "9.9.9"` replaces every planted literal, so
  the laws are independent of whichever release is in flight;
- `receipt_lock_planted_version_never_collides` fails first, naming the
  package, if the sentinel ever equals a committed lock version or
  `CARGO_PKG_VERSION`.

Production source is untouched. The closed admission gained the narrowest
content-bound rule that admits this repair, plus five focused laws proving it
is fail-closed:

- the emitter's production source (everything before `#[cfg(test)]`) must be
  byte-identical, the head must still declare `mod receipt_lock_tests`, and the
  test module must change;
- `test/package-install/scope.ts` and `scope.test.ts` may only grow: every base
  line must survive in order and the growth is capped, so the pre-existing
  arbitrary-source plant still rejects a rewritten guard.

Negative laws: production-line change, dropped module, broader preview source,
rewritten guard law and unbounded guard growth all reject.

## Nucleus and GPUI census repin

`effigy regressions:native` at the frozen commit: 255 passed, 0 failed
(242 regressions plus the 13 lock-provenance laws). The emission repinned:

- 29 M1 and 29 A1 receipts plus `nucleus-parity-manifest.json` to
  `package_version 0.4.0`, `source_commit a797ce413`, lock
  `4cf00ae2dd80c976e57dc9db49cc7cd462e5d841bf51c993e0482fc33e9fa277` and the
  four workspace packages at `0.4.0`; the A1 snapshots under
  `test/nucleus-a11y/snapshots` are unchanged (the mounted projection did not
  move);
- the derived ledger line `poodle-gpui-preview@0.4.0`;
- 65 mounted census receipts, the execution record
  (`run_id 2026-09-12-g18-006-v040-release-candidate-expected`), the census
  pair, the capability manifest and the missing-capability group file. Census
  `package_version` is now derived from `packages/gpui/preview/Cargo.toml`, so
  every receipt reads `0.4.0`.

Every `source_commit` recorded anywhere under `docs/evidence/nucleus/**` and
`docs/evidence/gpui/**` (126 occurrences) resolves to the frozen commit. The v1
Lab receipts are unchanged.

## Source-free package proof

`effigy svelte:package` and `effigy react:package` at the frozen commit, then
one `bun pm pack` per package:

| Package | Version | Archive SHA-256 |
| --- | --- | --- |
| `@inflatable-cookie/poodle-core` | `0.4.0` | `8cc9ac950eddfaa40793ed3e90406b38f5e5040ea5e4289346288451f321ab45` |
| `@inflatable-cookie/poodle-svelte` | `0.4.0` | `464f77d5ae9d53e16d6495393d1ab01d9f5c36669698bf06793073f67c9c52f1` |
| `@inflatable-cookie/poodle-react` | `0.4.0` | `195236cd7b49bdbf6c82429408e454edaabcb0bab0444bbf1f540f6f7a9120ae` |

All three `dist/.poodle-build.json` receipts record `version 0.4.0` and
`sourceCommit a797ce413`. `effigy test:web-pack-install` then re-proves the npm
production path from a source-free clean checkout of this exact commit: two
clean builds and packs per package with matching inventories and archive bytes,
the archives installed into a fresh no-workspace consumer, `./editor` and
`./rich-text` browser and SSR imports, packed declarations under Bundler and
NodeNext, the Svelte `5.56.8` floor with the visible `5.38.6` below-floor
negative, CSS/parser isolation, exact dependencies, and the frozen 176-name
roster. Ordinary mode certifies the closed `0.4.0` range with no receipt, and
every scope falsification plant still bites.

## Public-surface freeze re-run

`effigy audit:public-surface` against `v0.3.0..a797ce413`: 515 rows. The only
rows beyond the g18.027 frozen 510 are the five expected lockstep consequences —
three `package-version/breaking` (core, Svelte, React `0.3.0` → `0.4.0`, which
the frozen report assigns to g18.006) and two `package-dependency/behavioral`
(the internal `@inflatable-cookie/poodle-core` specifier moving to `0.4.0`). No
unexpected public-surface change.

## Validation

- `cargo test ... --test headless_regressions receipt_lock` — 13 passed, 0
  failed (the affected laws first).
- `effigy regressions:native` — 255 passed, 0 failed.
- `bun test test/package-install/scope.test.ts` — 54 passed, 0 failed.
- `effigy test:nucleus-parity-receipts` — 17 passed, 0 failed.
- `effigy check:parity-evidence-ledger` — 176 rows validated.
- `effigy check:gpui-census` — checked-in artifacts match the generator.
- `effigy ir:check` and `effigy catalogue:check` — generated artifacts current.
- `effigy test:web-pack-install` (ordinary mode) — pass; 12 test files, 25
  tests, two deterministic build/pack rounds, fresh source-free consumer
  install, browser plus SSR, declarations, floor and roster, plus every
  closed-candidate falsification plant.
- `effigy changelog validate CHANGELOG.md`, `effigy docs:lint`,
  `effigy check:release-automation`, `effigy audit:public-surface` — pass.
- `git diff --check` — clean.

Scope note. `v0.4.0` is an npm/web package release. The repository-wide
`effigy release gates` board and the native/GPUI lanes (`ci:native`,
`regressions:native`, `probe:gpui-specimens`, the windowed selectors) are not
part of this candidate's certificate; GPUI is not released here and its
specimen-probe wall-clock behaviour is not a release blocker. The npm path
above is the certificate this candidate owes. No redundant `qa`, `ci:web`,
`docs:check`, windowed selector or `release status` run is stacked on it.

## Boundaries held

No tag, no publication, no registry mutation, no workflow edit or dispatch, no
Desktop change, no gate bypass, no `0.4.0`-other-than-planned mutation, and no
candidate-policy widening beyond the narrow content-bound test-repair
admission recorded above. The hosted branch dry run, tag, tag dry run and
publication remain `g18.009` work after this candidate merges.
