# 071 Fast Validation And npm Release Pipeline

Status: active
Updated: 2026-09-13
Depends on: `022-packaging-versioning-and-release-channel-rules.md`,
`044-deprecation-change-control-and-release-channel-operations.md`,
`070-compiled-web-distribution-contract.md`

## Purpose

Make validation observable and bounded, and make npm release a short packaging
operation over an already reviewed web candidate. The complete headless board
may include native and GPUI proof, but the npm lane never does.

## Release trains

Poodle has separate release trains:

- the npm/web train contains the root release version, core, Svelte and the
  private React validation package;
- the native train contains Cargo packages and needs separate future release
  and tag authority.

The npm tag `vX.Y.Z` names the web train. An npm candidate must not bump Cargo
manifests, Cargo locks, GPUI receipts, native census artifacts or native
evidence. React remains unpublished but follows the web version so paired web
certification stays truthful.

The machine-readable npm publication set belongs in
`packages/release-manifest.json`. Scripts and workflow checks derive it from
that authority instead of repeating package paths.

## Candidate admission

Candidate admission is generic. Active policy must not contain a generation
ID, task ID, source version or target version.

The classifier derives base and target versions from the compared commits and
admits a candidate only when:

- the target is a valid greater pre-1.0 semantic version;
- root, core, Svelte and React move in exact lockstep;
- internal web dependency requirements and `bun.lock` match that target;
- the changelog and one matching release note describe the target;
- every other change is a bounded release input or execution record declared
  by release policy.

Partial bumps, arbitrary source, workflow, registry, package-membership or
native changes fail closed with the specific violated rule. Historical
release-specific modes are evidence, not templates for the next release.

## Validation graph

Validation has named leaves and explicit ownership:

- required PR CI proves source behavior;
- the web archive certificate builds core/Svelte once, packs once and installs
  those same archives into a source-free consumer;
- the complete headless board composes web, Rust, native/GPUI and policy leaves
  without repeating a leaf already covered transitively;
- foreground/windowed capture stays separately operator-authorized.

Changed-range admission and archive certification are separate selectors.
Read-only independent groups may run concurrently. Generators run once before
their check consumers or support a read-only check mode. Package builds and
Cargo targets are reused inside a board, not rebuilt by each nested selector.

Every board emits a child start line, completion line and elapsed time while it
runs. A silent aggregate process is invalid. Each expensive child has a bounded
timeout and terminates owned descendants on failure. A final machine-readable
summary names the slowest leaves, cached/reused work and failure.

## Hosted npm protocol

One retained `.github/workflows/release.yml` supports two explicit modes:

1. **candidate** — on the exact merged candidate commit, run the npm archive
   certificate and upload the two tarballs plus a manifest containing source
   commit, version and SHA-256 values;
2. **publish** — on the matching `vX.Y.Z` tag, download the named candidate-run
   artifact, verify tag, source commit, version, package names and hashes, then
   publish those exact tarballs with npm trusted publishing and verify registry
   availability.

If the tagged workflow wrapper itself blocks before publication, a reviewed
wrapper-only repair may dispatch publish from the default branch with an
explicit existing `release-tag`. Checkout and identity verification still
resolve that immutable tag, and the same successful candidate run supplies the
archives. This recovery route cannot rebuild, move the tag, or substitute a
different source commit.

There is no tag dry run and no rebuild during publish. A publish invocation
without a successful candidate run ID or exact identity fails before npm
mutation.

## Runtime budgets

- The npm workflow uses `ubuntu-latest`, installs no Rust/Cargo tooling and has
  a ten-minute hard job timeout. Candidate target is under three minutes;
  publish target is under two.
- The complete headless board target is under ten minutes warm and fifteen
  minutes cold on the supported development/CI machines. No child may run
  silently for more than five minutes.
- A board still active at its hard ceiling is a failed process. Stop and
  escalate; never wait hours for an unexplained selector.
- A clean timeout is containment, not acceptance. Every required leaf must
  pass before validation optimization is complete; an over-budget leaf stays
  an open repair obligation with its coverage intact.
- Workers run a baseline board at most once and a final board at most once for
  an optimization task. They use focused leaves between those points.

## Automation guard

The release checker proves structure rather than freezing incidental workflow
steps. It requires:

- one Effigy npm certificate entry in candidate mode;
- no aggregate, Rust, native, GPUI or Jetstream selector or setup;
- Linux runner and the hard timeout;
- candidate artifact and publish-time identity checks;
- publication of archives rather than package directories;
- tag plus explicit publish-mode mutation guards;
- core and Svelte only in the publication set.

The root Effigy release gate points at the same npm certificate. Aggregate
repository QA keeps its own name and is never release authority.

## Evidence

The failed `0.4.0` run `34728955353` spent 5h45m inside the former aggregate
gate and produced no child progress before cancellation. Repaired candidate
run `34743528777` completed in 3m41s, including 1m52s installing unused
`cargo-deny`; its npm/web proof took 49s. Tag dry run `34743709181` and publish
run `34743884234` repeated the same build and proof.

Current `ci:web` takes about eight minutes on GitHub. The current `qa` graph
serially nests `ci`, `ci:native`, installed-package proof and audits, including
work already performed by child boards. This spec requires measured graph
deduplication, reuse and visible timing rather than another wrapper around the
same work.

This spec supersedes the three-hosted-run, native-lockstep and opaque aggregate
assumptions in `docs/contracts/001-working-rules.md`.
