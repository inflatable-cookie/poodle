# g16.058 — Shell Distributions

Status: implementation repair — PR #162; operator boundary decisions promoted
PR: https://github.com/inflatable-cookie/poodle/pull/162
Type: implementation — compiled Svelte and private React packages
Opened: 2026-09-02
Depends on: accepted `g16.057`
Governing refs: `../../architecture/014-compiled-web-package-distribution.md`,
`056-web-distribution-contract.md`, `057-core-build-substrate.md`

## Goal

Build source-free Svelte client/server distributions and the matching private
React distribution on the core substrate. Apply the accepted root/markdown
break, dependency externalization, CSS policy, declaration shape, and one
disposable installed browser/SSR exit smoke. Leave permanent certification to
`g16.059`.

## Fixed Boundary

- Compile Svelte browser output as `*.client.js` and Vite `build.ssr: true`
  output as `*.server.js` from one sorted entry inventory.
- Pin the distribution declaration toolchain to TypeScript `6.0.3`. Copy the
  real `@sveltejs/package` declarations from staging. Public `*.svelte`
  subpaths target `*.svelte.d.ts` and compiled JavaScript, never source. Bare
  generic `Component` declarations fail.
- Use `browser` → client and non-browser `default` → server. Do not add an
  `import` environment branch, raw-source fallback, top-level `svelte` field,
  or unproved `svelte` condition.
- Compile private React to the same source-free/declaration/CSS standard. It
  stays private and unpublished.
- Root shells exclude AgentMessage, AgentPlan, AgentPlanRecord,
  AgentTranscript, and MarkdownEditor; `./markdown` owns them and the optional
  `marked` peer. Ordinary root graphs stay parser-free. Do not defer markdown
  through a lazy import: all five render synchronously in browser and SSR.
- Card 3's installed smoke is disposable and narrow. It must not edit or own
  the permanent receipt or add certification behavior. This repair may update
  only the existing package-install roster, compiled-path, Svelte-floor,
  CSS-execution, and markdown-import retargeting for the
  171-root/five-markdown split so the current web gate stays green. Existing
  runtime proofs move to the new subpath; they are not deleted.
- No version, release-note, workflow, tag, registry, or sibling mutation.

## Ordered Work

1. Add Svelte client/server builds on the core driver with stable public/chunk
   names and exact external dependencies.
2. Emit/copy real Svelte declarations with TypeScript `6.0.3` and
   `@sveltejs/package`, without raw `.svelte`; implement root, direct, markdown,
   and `./types` exports exactly as contracted.
3. Compile the private React package and its root/direct/markdown/type barrels
   without source mapping or publication authority.
4. Apply exact files/sideEffects arrays, Svelte peer floor, optional `marked`
   peer, core CSS imports, and parser isolation.
5. In a disposable no-workspace consumer, install staged archives and prove
   Svelte 5.56.8 browser mount plus Node/worker-like SSR rendering for
   root/direct Button/Select and markdown. Prove direct client output fails SSR
   and React public value declarations compile.
6. Remove every disposable smoke artifact before handoff.

## Acceptance

- Installed Svelte browser and SSR fixtures pass at `5.56.8`; browser resolves
  client and Node/worker-like SSR resolves server default.
- Directly rendering client output through `svelte/server` fails.
- Root/direct/markdown and `./types` runtime/declaration targets exist inside
  `dist`; no raw source or map is packed.
- Invalid public props, callbacks, snippets, and bindable assignments fail
  unsuppressed installed-package type fixtures under Bundler and NodeNext.
- Root graphs remain parser-free, markdown missing-peer behavior is clear, and
  focused CSS graphs contain no unrelated component styles.
- Private React value/declaration imports compile without source mapping, and
  React remains unpublished.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Browser and server lanes are distinct | client artifact renders in SSR | negative fixture fails acceptance |
| Conditions describe environment | `import` selects client | worker/Node resolution proof fails |
| Svelte output is source-free | wildcard points to `.svelte` | archive/export audit fails |
| Markdown is isolated | Button root resolves `marked` | graph assertion fails |
| Root is parser-free | any of the five markdown components remains in root, or uses a lazy parser import | fresh root import or synchronous browser/SSR render fails |
| Declarations preserve the API | a public component emits bare `Component` | negative prop/callback fixture compiles |
| Permanent harness ownership is serial | card adds 059 receipt, below-floor negative, or new certification probe | scope check fails |

## Writable Scope

Svelte/React build and staging configuration, their package manifests/exports,
shell root/markdown barrel migration, peer/CSS declarations, focused build and
disposable smoke fixtures, only the existing package-install roster,
compiled-path, Svelte-floor, CSS-execution, and markdown-import retargeting,
this card, one log, and new papercuts. Do not add permanent certification behavior or edit its
receipt, version/release surfaces,
workflows, tags, registries, sibling repositories, or React publication state.

## Validation

Run focused shell builds, declaration checks under Bundler and NodeNext,
source/export/content audits, the disposable browser and SSR installed smoke,
the client-artifact SSR negative, Svelte floor check, unsuppressed installed
Svelte declaration negatives, React value/declaration compile, the existing
`test:web-pack-install`, `effigy ci:web`, `effigy docs:check`, and `git diff
--check origin/main...HEAD`. Remove all disposable consumers/tarballs. Never
run release or windowed selectors.

## Stop Conditions

Stop if either lane needs raw source; browser/SSR resolution is ambiguous; a
client artifact serves SSR; `./types` loses runtime or types; the Svelte floor
fails; declarations need paths/sibling source/suppression; CSS or parser
isolation drifts; React becomes publishable; permanent certification is edited;
or any release/workflow/registry mutation appears. Stop if
`@sveltejs/package` cannot emit the real public surface under the pinned
TypeScript `6.0.3` toolchain.

## Continuation

Accepted merge unlocks `g16.059`. The disposable smoke is not sufficient to
unblock `g16.054`.
