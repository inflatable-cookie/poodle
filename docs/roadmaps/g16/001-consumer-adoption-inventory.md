# g16.001 — Consumer Adoption Inventory

Status: **complete — 16 authoritative consumer repositories frozen**
Depends on: published `v0.2.1` and completed `g15.013`
Governing refs: `README.md`,
`../../../README.md`, `../../README.md`,
`../../logs/2026-08/20260823-g15-013-v021-release-certification.md`

## Outcome

Identify every authoritative repository under `~/Dev/projects` that declares
or resolves a Poodle web or Rust dependency. Freeze the upgrade policy and
dependency order before changing consumer manifests.

## Authoritative Consumers

| Repository | Direct surface | Current shape | Ordering |
| --- | --- | --- | --- |
| Acowtancy | core + Svelte in Cream/Dairy; Svelte peer in Froyo | registry `^0.1.0` | after Underlay |
| Bovine Accelerator Desktop | core + Svelte | committed `file:` links/overrides | after Longhorn |
| Compli Me | core + Svelte in admin/front | `^0.1.0` plus `file:` overrides | after Underlay |
| Composer | core/Svelte across admin/front | `^0.1.0` plus `file:` overrides | after Underlay |
| Contact Patch | core/Svelte across admin/front | `^0.1.0` plus `file:` overrides | after Underlay |
| Figmatic | core + Svelte | committed `file:` links/overrides | after Longhorn |
| Finch | core + Svelte in active Tauri app | registry `^0.1.0` | after Longhorn |
| Jetstream | core + Svelte web; paired Rust paths | `^0.1.0` plus `file:` overrides | after Longhorn |
| Longhorn | core/Svelte workspace and examples; Svelte peer; Rust git tags | registry/tag `0.1.0` | foundation |
| Loophole | core + Svelte | committed `file:` links/overrides | after Longhorn |
| Nucleus | core + Svelte | registry `^0.1.0` | after Longhorn |
| Songsprout | core + Svelte in Bloom/Greenhouse | `^0.1.0` plus `file:` overrides | after Underlay |
| Soundcheck | core + Svelte | `^0.1.0` plus `file:` overrides | after Longhorn and Soundcheck Library |
| Soundcheck Library | Svelte root dependency and two peers | `^0.1.0` plus `file:` overrides | foundation |
| Underlay | Svelte | `^0.1.0` plus `file:` overrides | foundation |
| Underlay Reference | core + Svelte across three apps | `^0.1.0` plus `file:` overrides | after Underlay |

## Exclusions

- Poodle's own workspace dependencies are release internals, not consumers.
- `acowtancy-consolidation.*` contains temporary imports/worktrees that
  duplicate Acowtancy's authoritative manifests.
- `finch/archive/app-electron` is a frozen historical application.
- The operator removed the Loophole Legacy repository on 2026-08-25. It is no
  longer an authoritative consumer; its historical references remain evidence.
- Historical docs, fixtures that record an old release claim, generated
  artifacts, and vendored trees are evidence rather than active declarations.

## Acceptance

- [x] Web manifests and lockfiles were searched across `~/Dev/projects`.
- [x] Rust Poodle paths and git tags were searched across Longhorn and
      Jetstream.
- [x] Authoritative repositories were separated from temporary and archived
      copies.
- [x] Registry pins, peer ranges, local overrides, Rust tags, and ordering were
      classified.
- [x] Three independent foundation cards are ready.

## Stop Conditions

- Do not bulk-edit source trees from the Poodle planning checkout.
- Do not treat a local `file:` resolution as published-package evidence.
- Do not update archived or temporary copies to make the inventory look clean.
- Do not compile application cards before their shared foundation result is
  known.

## Validation

The inventory used exact manifest/lockfile searches plus repository-root Git
status checks. All foundation repositories are clean on `main` and aligned
with `origin/main`. Bovine Accelerator Desktop has unrelated active work in
its main checkout; its later adoption must use a separate worker worktree and
must not touch or depend on those changes.
