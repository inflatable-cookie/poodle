# Compiled Web Distribution Prerequisite

Status: open — recommendation for orchestrator acceptance; implementation not dispatched
Captured: 2026-09-02
Owner: Poodle Northstar orchestrator
Source handoff: `../handoffs/20260902-021250-web-distribution-prerequisite.md`
Base inspected: `99cc55711464edb` (`origin/main`)
Canonical release card: `../roadmaps/g16/054-historycenter-v030-release-candidate.md`

## Decision

Promote one separately accepted web-distribution prerequisite before `g16.054`
may freeze the `0.3.0` candidate.

The recommended implementation is a repo-owned build driver around Vite library
mode, the existing Svelte and React Vite plugins, and a separate declaration
emit. The package boundary is `dist/`: compiled JavaScript, declarations, CSS,
the existing package documentation/licence surface, and a deterministic build
receipt. No release tarball may expose `src/`, TypeScript, TSX, or Svelte source.

The public release set remains core plus Svelte. React gets the same compiled
distribution shape for the installed-tarball proof, but remains `private` and
unpublished until a named consumer exists. This prerequisite does not change
that admission decision.

The one public-surface choice is `marked`. Preferred shape: remove the two
markdown components from the Svelte and React root value barrels, add a
`./markdown` entry that exports them, keep direct component subpaths only as
explicitly admitted existing routes, and make `marked` an optional peer of the
shell that owns those components. The markdown entry externalizes `marked`.
The root Button/Select path then has no markdown-parser dependency or module
edge. Removing root names is a breaking export change even before `1.0`; it
must be named in the contract, roster, migration note, and `0.3.0` release
notes. Do not add a compatibility alias to hide the decision. If the operator
does not admit that break, retain the current root shape as a separate
decision, but do not call the result “marked-isolated”.

The exact CSS policy is:

```json
"sideEffects": ["**/*.css"]
```

Apply it to core and Svelte. Mirror it in the private React validation package
so the two shells have one bundler contract. Do not use `false`, a JavaScript
wildcard, or a package-wide side-effect claim.

## Current evidence

| Surface | Current package boundary | Current packed evidence | Consequence |
| --- | --- | --- | --- |
| Core | `exports` and `files` point at `src`; root is `src/index.ts` | 407 files; no compiled `.js` or declaration output | A consumer receives source modules and source CSS. |
| Svelte | root, `svelte`, and `./*.svelte` point at `src` | 227 files; wildcard resolves 178 `.svelte` files | A consumer must compile the library itself. |
| React | `private: true`; root types/default point at `src/index.ts` | 219 files; no compiled `.js` or declarations | It is source-only in practice, even though it is packed for validation. |
| CSS | no package declares `sideEffects` | 175 core-style import lines in each shell, across 173 files; core has 167 component CSS files and 22 generated token CSS files | Tree-shakers have no declared CSS side-effect boundary. |
| Markdown | `marked` is a hard shell dependency; core has it only as a test/dev dependency and no runtime import | `AgentMessage` and `MarkdownEditor` are the only shell runtime users | A root import can pull markdown machinery into a non-markdown consumer. |

The inspected manifests are [`packages/core/package.json`](../../packages/core/package.json),
[`packages/svelte/components/package.json`](../../packages/svelte/components/package.json),
and [`packages/react/components/package.json`](../../packages/react/components/package.json).
The handoff names `docs/triage/20260901-230400-history-release-adoption-decision.md`,
but that path is absent on this base; the matching release-adoption decision is
present as `docs/handoffs/20260901-230400-history-release-adoption-planning.md`.
This packet treats that as path drift and does not reopen the settled release
decision or create a second deliverable.
The architecture keeps shared behavior and styles in core, so the distribution
must preserve core CSS subpaths rather than move app-specific styles into a
shell bundle: [`docs/architecture/001-poodle-system-shape.md`](../architecture/001-poodle-system-shape.md)
and [`docs/architecture/002-token-system-and-package-layout.md`](../architecture/002-token-system-and-package-layout.md).

The existing selectors do not build release packages:

- `effigy svelte:build` and `effigy react:build` build preview applications.
- `test:core`, `check:svelte-components`, and `test:components` validate source
  trees or source-linked test graphs.
- `test:web-pack-install` is the only installed-tarball path, and currently
  packs the raw `src` boundary before installing it.

The baseline `test:web-pack-install` run on this branch completed 20 tests on
2026-09-02 at `01:19:14.488Z`, with artifact set
`3c525bb2ff2308d3a4a5b86c6cb1f568710a3f33f9b9fd6966ffbbed79a67daa`:

| Package | Bytes | SHA-256 | Archive files |
| --- | ---: | --- | ---: |
| `@inflatable-cookie/poodle-core` | 343,419 | `e058e7c6713b92fc02ec7402673165f42180292c36842bddc2b72b2e8f76b839` | 407 |
| `@inflatable-cookie/poodle-svelte` | 277,669 | `4f8e0587ed9a6f3d58636e12cca6a5c0db04e7f0b6601773333b6b5a074990fc` | 227 |
| `@inflatable-cookie/poodle-react` | 274,748 | `1021b5c6b82ed4957b0c4acb653dff20bb0fbba697e9076c15abf9a1275a178` | 219 |

That run is useful boundary evidence, not a compiled-package receipt. Its
installed proof has no Vite aliases, sibling-source resolution, workspace
dependencies, private DOM selectors, or private MIME knowledge. Its Svelte
`HistoryEntry` positives pass and its two unsuppressed `branchCount` cases fail
with the expected TS2339 diagnostic. React root resolution is honest about
resolving the installed package, but its assignability proof maps to
`src/types.ts` because the source `src/index.ts` value barrel does not compile;
the future proof must have `valueBarrelCompiled: true` and no source mapping.

There is also a proof-surface inconsistency to repair in the implementation
card: [`test/package-install/roster.ts`](../../test/package-install/roster.ts)
and the live Svelte/React root sources use a 176-name roster, while
`test/package-install/web-preview.ts` writes `denominator: 175` into its JSON
receipt. [`g15.048`](../roadmaps/g15/048-packed-roster-reachability.md) already
records the older 175-name disagreement. The new proof must derive one
denominator from one canonical source and fail on disagreement; it must not
silently change the roster while moving markdown exports.

## Build tool and deterministic output

Use Vite 8 library mode as the JavaScript/CSS compiler. Vite is already the
repository's preview compiler, and its library mode supports explicit single
or multiple entries plus dependency externalization. Use the official Svelte
Vite plugin to compile `.svelte` files and the React plugin to compile TSX.
Move the build-time Vite/plugin/TypeScript dependencies to one internal build
surface that the package build can import; preview packages remain consumers of
the built packages, not the source of release build semantics.

Use explicit, sorted entry inventories. Do not rely on Vite's default hashed
filenames or on an entry list discovered from filesystem iteration. Public
entry filenames are stable; internal shared modules may be stable chunks under
`dist/chunks/`. Externalize runtime dependencies:

- core has no runtime dependency on `marked`; its `marked` dependency remains
  test-only unless the source changes.
- Svelte externalizes `svelte`, core, and `marked` for the markdown entry.
- React externalizes `react`, `react-dom`, core, and `marked` for the markdown
  entry.

The build driver owns a clean staging directory and writes this shape:

```text
dist/
  index.js
  index.d.ts
  icons/index.js
  icons/index.d.ts
  icons/icons/*.js
  icons/icons/*.d.ts
  icons/build.mjs
  icons/build.d.mts
  styles/*.css
  tokens/*.js
  tokens/*.d.ts
  tokens/generated/css/*.css
  <public-component>.js
  <public-component>.d.ts
  chunks/*.js                 # only when required by the bundler
  .poodle-build.json
```

Core's existing generated token and icon inputs remain source-of-truth inputs;
`effigy audit:tokens` and `effigy audit:icons` stay the pre-build drift checks.
The build copies the checked-in generated CSS and token surfaces into `dist`
and copies the already-JavaScript icon CLI to `dist/icons/build.mjs`. It does
not regenerate token or icon source as a side effect of packing.

The output contract is byte-stable for the same commit and tool lock:

- sort every entry, copied asset, and manifest list;
- use stable public filenames and stable chunk names, with no content hashes in
  export targets;
- write no wall-clock timestamp into output or provenance;
- normalize source paths and never write an absolute worktree path;
- build twice from clean temporary copies and compare the file list and every
  file hash; compare the tarball SHA-256 too, using the packer behavior already
  proven by the existing Bun pack path;
- fail if an output file is stale, orphaned, or not represented by the export
  inventory.

This follows [Vite library mode](https://vite.dev/guide/build#library-mode):
the library build must explicitly externalize dependencies and define its
entries. The preview `vite build` command is not sufficient evidence for this
contract.

## Declarations and Svelte type emission

Emit declarations in the same build transaction as JavaScript:

- core: TypeScript `emitDeclarationOnly` into the mirrored `dist` tree;
- React: TypeScript `emitDeclarationOnly` over the React source graph, after
  the value barrel is clean; no `paths` mapping to source is permitted;
- Svelte: run the Svelte package type emitter based on `svelte2tsx` and emit
  component declarations beside the compiled JavaScript names, then validate
  the declaration graph with TypeScript. The Svelte declaration emitter may be
  used in a disposable staging directory, but its uncompiled `.svelte` output
  must never be copied to the package.

`@sveltejs/package` is suitable for Svelte declaration generation and package
validation, but it is not the runtime compiler for this prerequisite. The
official packaging docs state that it copies Svelte files into `dist` while
preprocessing them; that is the normal Svelte package contract, not this
operator's compiled-JavaScript contract. Vite plus
`@sveltejs/vite-plugin-svelte` supplies the missing compile step. See
[Svelte packaging](https://svelte.dev/docs/kit/packaging) and the SvelteKit
discussion describing the uncompiled `.svelte` output from `svelte-package`
([#8825](https://github.com/sveltejs/kit/discussions/8825)).

Use TypeScript's declaration-only mode for the TS/TSX graphs; it is designed
for a separate JavaScript transpiler. See
[`emitDeclarationOnly`](https://www.typescriptlang.org/tsconfig/emitDeclarationOnly.html).
Set `declarationMap: false` for the release build. Svelte's own documentation
notes that usable declaration maps require publishing the source files they
point to, which conflicts with the no-raw-source boundary.

The declaration gate must prove all of the following from installed tarballs:

- every JavaScript export target has a corresponding declaration target where
  the public API is typed;
- Svelte root and direct component paths resolve to `dist/*.d.ts` under modern
  `bundler`, `node16`, or `nodenext` module resolution;
- React root declarations resolve through the package export map without
  `paths` or sibling source;
- declaration files contain no import of `src`, workspace path, or generated
  source-only substitute;
- the positive `continuationCount` and expected `branchCount` failures remain
  unsuppressed.

Do not add a `typesVersions` compatibility map in this prerequisite. It would
duplicate the export map and create a second drift surface. If support for
legacy TypeScript module resolution is required, record that as an explicit
public compatibility decision before implementation; it is not a silent
fallback.

## Export map and consumer resolution

All runtime and type conditions point into `dist`. The shape is:

```json
{
  ".": {
    "types": "./dist/index.d.ts",
    "svelte": "./dist/index.js",
    "import": "./dist/index.js",
    "default": "./dist/index.js"
  },
  "./*.svelte": {
    "types": "./dist/*.d.ts",
    "svelte": "./dist/*.js",
    "import": "./dist/*.js",
    "default": "./dist/*.js"
  },
  "./types": {
    "types": "./dist/types.d.ts",
    "import": "./dist/types.js",
    "default": "./dist/types.js"
  }
}
```

That is the Svelte shape; core uses the same `types`/`import`/`default`
pattern for JavaScript subpaths and preserves its CSS-only token/style
subpaths. The Svelte `svelte` field remains `./dist/index.js` only for older
tooling; the export condition is authoritative. React has the root equivalent
without the `svelte` condition. The `./markdown` entry is explicit and points
to compiled `dist/markdown.js` plus `dist/markdown.d.ts`.

The `.svelte` in a Svelte import specifier is a public subpath name, not a
permission to ship a `.svelte` file. Its export target is compiled `.js`.
Every declared wildcard must have a finite installed match; a wildcard must
not accidentally expose internal helpers.

Use explicit CSS targets for core:

```text
./styles/*                  -> ./dist/styles/*.css
./tokens/styles.css         -> ./dist/tokens/generated/css/poodle-tokens.css
./tokens/themes.css         -> ./dist/tokens/generated/css/poodle-themes.css
./tokens/<mode>.css         -> ./dist/tokens/generated/css/<generated>.css
```

The complete current token export inventory remains in the package manifest;
the implementation card must mechanically rewrite each target to `dist` and
test each exact subpath from an installed tarball. No `src` target, `main`
fallback, or source alias is admitted.

This shape follows the Svelte package guidance that `types` and `svelte` are
export conditions and that package entry points should target `dist`. Removal
of a current export condition or path is a breaking change, so the markdown
root-barrel decision must be recorded rather than smuggled into packaging
mechanics.

## CSS delivery

Keep CSS ownership in core. Core publishes the 167 component stylesheets and
22 generated token stylesheets under `dist`; Svelte and React compiled modules
retain imports of the exact core style subpaths they use. The package build
must not create one unrelated all-components CSS bundle as the public API.

The `sideEffects` contract is the exact array above in core, Svelte, and the
private React validation package. The installed consumer proof must:

- resolve `@inflatable-cookie/poodle-core/styles/button.css` and the other
  declared style subpaths from the tarball;
- preserve CSS when a component import is tree-shaken by a webpack-compatible
  consumer;
- prove a Button/Select consumer does not pull unrelated `list-card` or `tabs`
  CSS merely because the package root was installed;
- prove token CSS remains a direct CSS import, not a JavaScript-generated
  replacement;
- fail if a compiled JS file refers to a source CSS path or a CSS file is
  absent from the tarball.

The exact array is also the configuration recommended by the
[Svelte packaging guide](https://svelte.dev/docs/kit/packaging#sideeffects),
which calls out CSS compatibility with webpack.

## `marked` isolation

The core boundary is already structurally correct: `markdown-blocks.ts` owns a
structural token type and accepts lexer output; it does not import `marked`.
Core tests may continue to use `marked` as a dev dependency.

The preferred shell arrangement is:

1. create a markdown entry for `AgentMessage` and `MarkdownEditor` in each web
   shell;
2. remove those two names from the root value barrels;
3. make `marked` an optional peer dependency of each shell, not a normal
   dependency; keep it external in the markdown build;
4. document `marked` as required when the markdown entry is imported;
5. keep the two markdown components' CSS imports on core style subpaths;
6. update the frozen roster and release notes as a named breaking change.

The proof must contain a clean Button/Select consumer with no `marked` entry at
all, and a separate markdown consumer that installs `marked` explicitly. A
third fixture that imports markdown without `marked` must fail at module
resolution with a clear missing-peer diagnostic. A root import that still
requires `marked` fails the prerequisite even if a bundler happens to tree
shake the parser later.

## Package contents, notices, provenance

The public package `files` policy is:

```json
["dist", "README.md", "LICENSE", "THIRD_PARTY_NOTICES.md"]
```

for core, and:

```json
["dist", "README.md", "LICENSE"]
```

for Svelte and the private React validation package unless either shell
actually bundles a new third-party asset. The package manager includes
`package.json`; the proof must still inspect it. The tarballs contain no
`src`, tests, previews, tsconfigs, workspace metadata, sibling path, or
unrelated generated tree.

Core retains its existing `THIRD_PARTY_NOTICES.md` because the generated icon
surface carries the Lucide/Feather-derived notice. Do not copy that notice into
a shell that does not distribute those assets. Conversely, if a shell build
bundles code or assets from a notice-bearing dependency, add the exact notice
before acceptance. The notice is a statement about the current emitted graph,
not an append-only historical list. The repository's current audit requires
the core notice and the Lucide markers; see
[`scripts/audit-license-compliance.ts`](../../scripts/audit-license-compliance.ts).

Ship no JavaScript or declaration source maps in the `0.3.0` tarballs:
`sourcemap: false`, `declarationMap: false`. A map that embeds source violates
the operator's boundary; a map without source content points at files the
package deliberately does not ship. Local debugging may generate maps in the
temporary staging directory, but the pack-content gate must reject `.map`
files. Revisit this only as an explicit distribution decision.

Write a deterministic `dist/.poodle-build.json` receipt with no timestamp or
absolute path. It records:

- package name and manifest version;
- the full source commit;
- build-tool and runtime versions from the lockfile;
- a canonical hash of the sorted build inputs;
- sorted public entries and output files;
- the markdown/CSS decisions and source-map policy.

The installed-tarball evidence remains the authoritative external receipt. It
records each archive's byte count and SHA-256, the exact source commit, the
consumer tool versions, the output file inventory, and the negative proofs.
The receipt must be generated from a clean temporary checkout of the exact
commit, not from an operator's dirty worktree.

The build may write only a package-local ignored `dist` in ordinary development
or a temporary staging directory in release proof. It must not modify tracked
source, manifests, locks, release notes, workflows, tags, registries, or
sibling repositories. The pack proof removes its temporary output and checks
the tracked tree before and after the run. It must fail if a build starts from
a dirty release checkout or if a stale output survives in the staging tree.

## Installed-tarball proof

Upgrade `test:web-pack-install` rather than creating a second pack proof. Its
future receipt should use a new schema version and contain these gates:

1. Build core, Svelte, and private React into clean staging directories from
   the exact checkout. Pack only the staged package boundary.
2. Inspect every manifest export target and wildcard match. Every target must
   exist under `dist`; no target may contain `src`, `.ts`, `.tsx`, or `.svelte`.
   Every typed target must resolve to a real `.d.ts`.
3. Install all tarballs into a fresh consumer with `file:` tarball references,
   concrete dependency versions, no workspaces, no `paths`, and no sibling
   source. Keep the existing React 18 and Svelte `5.38.6` floor proof; add the
   current repository versions only as a second compatibility leg if needed.
4. Run the existing mounted component tests through public imports, core exact
   subpaths, Svelte root/direct component/types paths, and the private React
   root. The React root value barrel must compile; the existing `src/types.ts`
   mapping is an expected-failure condition, not a permitted fixture.
5. Compile the positive `HistoryEntry.continuationCount` case and retain both
   unsuppressed `branchCount` TS2339 expected failures. Retain the Tree reorder
   positive and four expected failures, including React, with no diagnostic
   suppression.
6. Add the CSS and `marked` fixtures described above. Assert the installed
   manifest has the exact CSS `sideEffects` array, no `workspace:` or local
   dependency, and the expected optional peer shape.
7. Build twice and compare output inventories, file hashes, provenance, and
   tarball SHA-256. Record the artifact set ID and exact commit in the receipt.
8. Prove the frozen roster from one canonical denominator. If the markdown
   root-barrel break is admitted, the changed root set and migration are part
   of that proof; no alias is added to keep the old set green.

The current proof's clean-consumer constraints are worth retaining. Its
positive and expected-failure fixtures become stronger once the package
targets are compiled output rather than source output.

## Interaction with `g16.054`

`g16.054` is currently blocked only on the accepted separately promoted
compiled-JavaScript/declarations prerequisite. It owns the later immutable
`0.3.0` candidate, lockstep versioning, release-history honesty, release
notes, candidate receipt, and final release gates. It must not redesign the
build, exports, CSS, or dependency mechanics selected here.

The order is:

1. accept the prerequisite packet and its serial implementation cards;
2. land the compiled outputs/build contract and the upgraded installed-tarball
   receipt on main;
3. verify the prerequisite from the exact mainline commit;
4. let `g16.054` apply the lockstep `0.3.0` candidate changes and re-run the
   compiled pack proof against that candidate;
5. keep React private/validate-only and publish core plus Svelte only;
6. certify tag, registry, and npm state later under the orchestrator's release
   authority. Loophole adoption waits for independently proven npm
   `latest=0.3.0`.

No `0.3.0` or `0.2.4` tag, workflow dispatch, npm publish, registry mutation,
or sibling-repository write belongs in this prerequisite.

## Serial implementation cards

Keep the decomposition to four cards:

| Order | Card | Scope | Exit gate |
| ---: | --- | --- | --- |
| 1 | Web distribution contract | Freeze `dist` layout, sorted entry inventory, export conditions, CSS ownership/`sideEffects`, source-map policy, provenance schema, and the explicit markdown root-barrel decision. Update the roster and release notes only for the admitted public break. | A reviewed contract has no unresolved export or dependency choice. |
| 2 | Build and emit substrate | Add the internal Vite library driver, core JS/CSS output, TypeScript declarations, Svelte declaration emitter integration, clean staging, deterministic receipt, and package-local build selectors. | Core and Svelte output inventories are compiled, typed, deterministic, and free of raw source. |
| 3 | Shell distributions | Compile Svelte components with the Svelte Vite plugin, compile private React with the React plugin, apply the markdown boundary and optional peer, externalize runtime dependencies, and mirror CSS semantics. | Svelte mounts at the peer floor; React root declarations compile without a source mapping. |
| 4 | Tarball certification and promotion receipt | Upgrade `test:web-pack-install` with manifest/content/CSS/marked checks, positive and expected-failure consumer fixtures, repeated-build hashes, notice checks, and exact commit/artifact evidence. | One clean installed-tarball receipt is accepted and can be handed to `g16.054`. |

Do not split a workflow edit, a release mutation, or a component behavior repair
out of these cards. Those are separate authority surfaces.

## Stop and rollback

Stop the prerequisite on any of these conditions:

- a tarball contains `src`, raw `.ts`, `.tsx`, `.svelte`, a source export, or a
  missing declaration;
- a compiled Svelte component does not mount against the declared Svelte floor;
- the React value barrel needs `paths`, source imports, or a diagnostic
  suppression;
- an export target is missing, a wildcard exposes an unreviewed file, or a
  CSS/token subpath resolves only through a workspace;
- the CSS side-effect array differs from the exact contract or unrelated CSS
  enters the focused consumer bundle;
- a non-markdown consumer resolves or ships `marked`, or the markdown peer
  failure is hidden;
- two clean builds differ, provenance has a timestamp/absolute path, or the
  tarball receipt cannot identify its exact commit;
- the 175/176 roster disagreement is still present;
- core/Svelte are not the only web release packages, React becomes publishable
  without a named consumer, or a workflow/release/registry mutation appears.

Before `g16.054` is accepted, rollback is a normal revert of the prerequisite
implementation cards. Keep the last published `0.2.2` registry state as the
adoption baseline; `0.2.3` remains prepared-but-unpublished and `0.3.0` stays
untagged/unpublished. Do not repair a failed prerequisite with a source alias,
compatibility shim, hand-edited tarball, or a weakened negative proof. A failed
card leaves `g16.054` blocked until a new clean receipt is accepted.

## Promotion route

This note is the requested evidence packet. It changes no manifests, build
scripts, exports, workflows, release files, source, generated output, or
versions. The orchestrator may promote the four cards only after reviewing the
packet and the exact clean-consumer gates above. The current branch is otherwise
ready for the required docs lint, diff check, single-PR handoff, and no merge.
