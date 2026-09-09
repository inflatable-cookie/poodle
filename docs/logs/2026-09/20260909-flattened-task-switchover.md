# Flattened Northstar Task Switchover

Date: 2026-09-09
Repository: `inflatable-cookie/poodle`
Integration branch: `main`
Migration base: `85741961d60ac557974f4a4f3e2244b61c8f0b74`
Migration commit: `16d6d93dbd8b63405477af4e083125f163baec54`
Status: complete
Authority: operator-provided flattened-task switchover prompt

## Preflight

The integration checkout was clean and matched `origin/main` at the migration
base. Northstar Queue had no unfinished Poodle record: `g17.003` and `g17.004`
were both merged, closed, and workspace-archived. Paseo had no active Poodle
worker, reviewer, or coordinator; only the Chatterbox integration workspace was
active. GitHub had no open Poodle PR.

Old-format dispatch is suspended until this migration is reviewed, committed,
and pushed. Historical `.t3` worktrees are not dispatch authority. The two
dirty `g16.105` attempt worktrees are superseded by merged PR #210; their
uncommitted contents are retained and were not modified or deleted. The old
drag-substrate worktree belongs to merged PR #108 and was also left untouched.

## Historic-generation classification

| Generation | Classification | Preserved outcome |
| --- | --- | --- |
| `g01`–`g09` | safely closed | foundations through native consolidation and semantic sizing |
| `g10` | safely closed | Jetstream feasibility result and GPUI hardening; stale queued rows were superseded by later generations |
| `g11` | safely closed | Svelte modernization and shared web machinery |
| `g12` | safely closed | React parity, verification depth, native hardening, and audio family |
| `g13` | safely closed | Rust-authored component/scene IR pilot rejected and unwound |
| `g14` | safely closed | executable-conformance pilot rejected; retained component fixes and evidence doctrine |
| `g15` | safely closed | v0.2.x release/adoption programme and corrected v0.2.2 publication |
| `g16` | safely closed | v0.3.0, 15-consumer adoption, M1/A1 29/29, visual-lab and capture foundations |
| `g17` | active | Nucleus switch evidence and operator decision path |

Generation-index closure records, generation READMEs, delivery logs, and queue
history settle the stale task-local labels inside `g04`, `g10`–`g16`. None of
those labels remains executable after compaction.

## Preservation manifest

### Sources removed

The exact classified trees are `docs/roadmaps/g01/` through
`docs/roadmaps/g16/`. No other roadmap tree is in the deletion set.

### Authority and destinations

| Source meaning | Destination |
| --- | --- |
| durable product and package rules from `g01`–`g12` | current architecture, contracts, guides, component contracts, and implementation |
| rejected g13 IR result | `docs/specs/archive/063-rust-authored-component-and-scene-ir.md`, current architecture, and `docs/roadmaps/archive/g13.md` |
| rejected g14 conformance result | architecture 009, retained logs, and `docs/roadmaps/archive/g14.md` |
| release/adoption rules from g15/g16 | `CHANGELOG.md`, working rules, release logs, and `docs/roadmaps/archive/g15.md` / `g16.md` |
| frozen web-package denominator still consumed by package checks | `docs/evidence/releases/web-package-roster.md` |
| current Nucleus cohort, M1/A1 receipts, schemas, and generated ledger | `docs/evidence/nucleus/` |
| Nucleus switch programme, V1/V2/M2/A2 sequencing, and held gates | active `docs/roadmaps/g17/README.md`, `g17.001`, current triage notes, and `docs/evidence/nucleus/README.md` |
| exact historic roadmap detail | git history at migration base `85741961d60ac557974f4a4f3e2244b61c8f0b74` |

### Open commitments

- `g17.001` owns V1 receipt import and ledger advancement.
- V2, M2, A2, the switch packet, GPUI keyboard-origin focus, and web-pair
  extraction remain in the `g17` generation runway.
- Jetstream admission, citations, nested menus, HistoryCenter policy,
  keyboard geometry, consumer intake, repository settings, and
  `gpui-unofficial` gates remain in their current triage notes and the g17
  held/watchlist sections.
- The contributor design-guidance pilot remains operator-gated and is recorded
  in `g17/README.md`; no old task file remains executable.

### Material evidence

The roll-ups retain selected closeout, release, PR, validation, and rollover
references. Git remains the detailed archive. Current Nucleus receipt artifacts
move byte-for-byte to `docs/evidence/nucleus/`; their generators and checks move
with them.

## Active-generation mapping

The active generation already used top-level `NNN-<slug>.md` files. Its mapping
is therefore an identity map, with terminology and authority flattened:

| Old path and ID | New Northstar task |
| --- | --- |
| `g17/001-nucleus-v1-visual-receipts.md` (`g17.001`) | same path and ID; held prerequisite now satisfied and task becomes ready |
| `g17/002-web-focus-ring-input-modality.md` (`g17.002`) | same path and ID; compact completed task |
| `g17/003-background-safe-nonactivation-proof.md` (`g17.003`) | same path and ID; compact completed task |
| `g17/004-gpui-cohort-programmatic-append.md` (`g17.004`) | same path and ID; compact completed task |

No active milestone wrapper or `batch-cards/` hierarchy existed in `g17`.
The generation README becomes the sole roadmap and approved-frontier surface;
`dispatch.md` mirrors its ready queue-task boundary without becoming planning
authority.

## Validation and second pass

- `effigy docs:check` passed after the final path moves and generated-report
  refresh.
- `effigy test:nucleus-parity-receipts` passed 11/11.
- `effigy test:parity-evidence-ledger` passed 6/6.
- `effigy check:parity-evidence-ledger` validated all 176 rows.
- Direct package-roster loading found 176 frozen names and 171 component names
  in each web root from `docs/evidence/releases/web-package-roster.md`.
- Changed-current-doc link inspection resolved every local Markdown target.
- `git diff --check` passed.
- `effigy qa` reached 1,265 core passes and 57 build/scope passes, then stopped
  on the pre-existing detached React-preview positive control's fixed
  120-second timeout. A focused rerun reproduced 2 pass / 1 timeout. The
  migration does not touch that preview or build path; the friction is recorded
  in `PAPERCUTS.md`.

Adversarial semantic review found one live dependency missed by the initial
tree deletion: `test/package-install/roster.ts` still consumed the frozen g15
roster. The roster was moved to stable release evidence and the checker was
repointed before landing. No other active executable surface reads a removed
roadmap path; remaining source comments, schema IDs, old queue modes, logs,
archived specs, and archived handoffs retain historical gNN/card vocabulary.

The Chatterbox adversarial review accepted the migration at exact commit
`16d6d93dbd8b63405477af4e083125f163baec54`. This canonical planning
promotion used the integration checkout directly; no worker PR or product-code
merge was created.

The second lifecycle pass leaves only `g17` expanded, with unique task files
`001`–`004`; no active `batch-cards/` file; no top-level stale handoff; and one
frontier, `g17.001`, named consistently by the roadmap README, generation
index, and queue projection.
