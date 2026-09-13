# 032 — Fast validation and npm release

Status: ready
Owner: Poodle validation and web release infrastructure
Created: 2026-09-13
Governing refs: `../../contracts/001-working-rules.md`,
`../../specs/022-packaging-versioning-and-release-channel-rules.md`,
`../../specs/070-compiled-web-distribution-contract.md`,
`../../specs/071-fast-validation-and-npm-release-pipeline.md`,
`../../../effigy.toml`, `../../../tasks/effigy.tasks.toml`,
`../../../.github/workflows/release.yml`
Depends on: none; `v0.4.0` and g18.009 are complete

## Outcome

Replace the emergency `0.4.0` wrapper and opaque serial validation graph with
one observable, deduplicated system. A future web release needs one ordinary
candidate PR, one sub-three-minute hosted archive proof and one sub-two-minute
publish run. The complete headless web/Rust/GPUI board remains available but
finishes inside a hard fifteen-minute cold ceiling with live child timing.

## Ready-State Rubric

- [x] `v0.4.0` is published and its release evidence is closed.
- [x] Exact timings identify the 5h45m aggregate failure, unused Cargo setup
  and three repeated successful package proofs.
- [x] Core and Svelte are the only npm publications; React is private and
  paired; Cargo packages are unpublished.
- [x] The current full board's nested duplicate leaves and silent aggregate
  execution are identified in `tasks/effigy.tasks.toml`.
- [x] The operator explicitly made release and full-suite optimization the
  next blocking Poodle work and authorized workflow simplification.
- [x] Spec 071 fixes release isolation, generic admission, observability,
  runtime limits and the review oracle.

## Decisions

- Separate npm/web and native release trains. `vX.Y.Z` and the root version
  name the web train; npm candidates do not touch Cargo or native evidence.
- Extend `packages/release-manifest.json` with minimal machine-readable npm
  publication authority. Derive package paths from it.
- Replace active `g16.054-candidate` / `g18.006-candidate` branching with one
  version-independent web-candidate classifier.
- Split changed-range admission from archive certification.
- Build, pack and source-free-test one archive set. Candidate mode uploads
  those exact bytes; publish mode consumes them by run ID and never rebuilds.
- Use two hosted invocations: candidate, then tagged publish. No tag dry run.
- Make the root Effigy release gate npm-only. Keep aggregate `qa` independent.
- Profile every full-board leaf once, remove transitive duplicates, reuse build
  products and parallelize only independent read-only groups. Do not trade
  determinism or failure visibility for concurrency.
- Give every aggregate child live timing and an owned-process timeout. The
  complete board hard-stops at fifteen minutes cold; npm hard-stops at ten.

## Dispatch manifest

- **State:** ready; operator-prioritized ahead of all other Poodle work
- **Completion:** one independently reviewed PR merged after focused laws,
  exact-head CI, one baseline/full-board comparison and one authorized
  non-publishing hosted candidate drill meet spec 071
- **Owned mutable paths:** `.github/workflows/release.yml`, `effigy.toml`,
  `tasks/effigy.tasks.toml`, `packages/release-manifest.json`,
  `scripts/check-release-automation.ts`, validation/release runners and focused
  tests under `scripts/` and `test/package-install/`, `AGENTS.md`,
  `docs/contracts/001-working-rules.md`, specs 022/071, package-install docs,
  `PAPERCUTS.md`, and one execution log
- **Reserved closeout surfaces:** g18 README, roadmap root/index/dispatch, this
  task, this handoff, package versions, changelog, release notes, tags,
  registries and downstream repositories
- **Worker:** complex build/release infrastructure worker comfortable with
  GitHub Actions, Effigy, process lifecycle, npm packing and profiling
- **Excluded:** version changes, release preparation, tag creation, npm
  publication, product source, native release design, foreground/windowed
  execution, Desktop mutation and unrelated test rewrites
- **Escalation:** Chatterbox only if trusted publishing cannot publish a
  verified tarball, prior-run identity cannot bind to the tag, or a measured
  slow leaf needs product-level redesign outside validation infrastructure

## Work

1. Record a compact hosted timing baseline from runs `34728955353`,
   `34743528777`, `34743709181`, `34743884234` and recent `ci:web`. Run the
   current full headless board at most once with child-level timing and a
   fifteen-minute external kill; retain partial timings if it hits the cap.
2. Add npm publication authority to the release manifest. Make package paths,
   versions and archive expectations derive from it.
3. Split scope classification from archive certification. Remove duplicate
   builds/packs; install the same archives that candidate mode uploads.
4. Replace release-specific scope policies with spec 071's generic law. Plant
   patch/minor transitions plus partial, stale, source, workflow, registry,
   native and lockstep failures.
5. Define the single npm Effigy release gate. Keep aggregate `qa` separate and
   prove the release graph contains no native or broad board.
6. Rebuild `release.yml` as candidate/publish modes on `ubuntu-latest` with a
   ten-minute timeout. Candidate uploads archives plus identity manifest;
   publish downloads by run ID, verifies tag/commit/version/hash and publishes
   tarball paths.
7. Rewrite the automation guard around invariants. Add workflow negatives for
   aggregate/native setup, missing timeout, directory publish,
   rebuild-on-publish and artifact drift.
8. Use measured timings to flatten the full board: remove transitive repeats,
   combine shared setup/builds, add check-mode generators where validation
   currently writes, and run independent read-only web/native groups in
   parallel with bounded child processes. Emit start/end/elapsed events and a
   machine-readable summary. Preserve all existing assertions.
9. Update working rules and package-install/validation docs. Delete old
   three-run, native-lockstep and silent-wait instructions.
10. Run focused tests, then the optimized complete headless board exactly once
    under its fifteen-minute ceiling. Do not stack `ci:web`, `docs:check` or
    another broad board around it. Push one PR and report without polling CI.
11. Invoke candidate mode exactly once with publication disabled on the exact
    pushed head. This drill is explicitly authorized by Tom. Record timings
    and hashes; cancel and report a process failure if it reaches ten minutes.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| npm is independent of native | workflow installs Rust or reaches GPUI/Cargo | workflow/source scan plus release-graph assertion fails the plant |
| Full proof remains available | speed-up deletes a native/GPUI assertion | before/after leaf and assertion inventory is identical or explicitly stronger |
| Full board is bounded and visible | child hangs behind an opaque selector | planted hang names the child, emits progress and is killed with descendants |
| Full board avoids duplicate work | nested board rebuilds or reruns a leaf | graph/invocation-counter law proves each owned leaf once |
| Candidate policy survives next version | source names `g18.006`, `0.4.0` or fixed predecessor | source scan; synthetic `0.4.1` and `0.5.0` candidates pass |
| Unsafe candidates fail closed | partial lockstep, source, transport, registry or Cargo rides bump | focused range negatives fail before build |
| Packages build once | proof or publish rebuilds core/Svelte | counter permits one candidate build, zero publish builds |
| Published bytes are proved bytes | publish consumes rebuilt/different archive | identity mismatch fails before npm invocation |
| Only two hosted runs exist | protocol retains tag dry run | checker rejects a third proof phase |
| Workflow has one authority | wrapper lists several leaves | workflow calls one Effigy npm gate; manifest owns graph |
| Runtime improves materially | refactor only adds wrappers | full warm target <10m, cold hard cap 15m; candidate drill <3m |

## Validation budget

- Baseline: at most one current full-board run with a fifteen-minute external
  cap. No retries.
- Focused: classifier, release automation, archive certificate, validation
  runner process/graph tests, Effigy graph inspection, docs lint, diff check.
- Final: exactly one optimized complete headless board. It subsumes local web,
  Rust, native/GPUI, package and policy boards. Do not run overlapping broad
  selectors before or after it.
- Hosted: exactly one non-publishing candidate drill. No tag or registry
  mutation. Queue owns asynchronous CI observation.

## Stop conditions

- Stop before any version, tag, registry or downstream mutation.
- Stop if publish cannot verify prior candidate-run identity before npm.
- Stop if npm needs native setup or cannot hard-stop at ten minutes.
- Stop if generic admission permits product or transport changes.
- Stop and return a measured blocker if one leaf alone exceeds five minutes
  and fixing it needs product behavior changes.

## Evidence

`0.4.0` run `34728955353` spent 5h45m in the aggregate gate without child
progress. Repaired candidate run `34743528777` took 3m41s: unused `cargo-deny`
installation consumed 1m52s and npm/web proof took 49s. Tag dry run
`34743709181` and publish `34743884234` rebuilt and re-proved the same source.
Current GitHub `ci:web` takes roughly eight minutes. Candidate preparation also
needed g18.029–031 because policy encoded one version, coupled npm to Cargo
lockstep and embedded native receipt identity.

## Next task

After merge and timed proof, resume the post-`0.4.0` compatible
consumer/specimen sweep. Its fixes feed `0.4.1` through this generic pipeline.

