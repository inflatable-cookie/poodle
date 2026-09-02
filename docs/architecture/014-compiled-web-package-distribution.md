# 014 Compiled Web Package Distribution

Status: accepted target architecture
Recorded: 2026-09-02
Owner: Poodle web packages and release tooling
Depends on: [System shape](001-poodle-system-shape.md),
[token and package layout](002-token-system-and-package-layout.md),
[packaging/versioning rules](../specs/022-packaging-versioning-and-release-channel-rules.md),
and [compiled web distribution contract](../specs/070-compiled-web-distribution-contract.md)

## Decision

Poodle's web package boundary is `dist/`. Release tarballs contain compiled
JavaScript, declarations, shared CSS/assets, package documentation and
licensing, and one deterministic build receipt. They do not expose `src/`, raw
Svelte, non-declaration TypeScript/TSX, source maps, workspace metadata, or a
source fallback.

Core and Svelte remain the public release set. React must have the same
compiled and installed-certification shape, but stays private and unpublished
until a named consumer is admitted. Preview applications consume built package
outputs; they do not define release semantics.

The breaking `0.3.0` boundary moves `AgentMessage`, `AgentPlan`,
`AgentPlanRecord`, `AgentTranscript`, and `MarkdownEditor` out of the Svelte
and React root barrels and into explicit `./markdown` entries. Root
Button/Select stays parser-free. No alias, compatibility shim, or silent
fallback preserves the retired root markdown imports.

## Package boundary

All three web packages use exact file and side-effect policies:

```json
// core
"files": ["dist", "README.md", "LICENSE", "THIRD_PARTY_NOTICES.md"],
"sideEffects": ["**/*.css"]

// Svelte and private React
"files": ["dist", "README.md", "LICENSE"],
"sideEffects": ["**/*.css"]
```

Core uses one JavaScript lane with declarations. Every core JS export uses
`types`, then `import`, then `default`, with `import` and `default` targeting
the same compiled JavaScript. That `import` is a module-format fallback, not an
environment selector. CSS-only entries stay conditionless strings. The complete
existing public entry and token inventory is rewritten mechanically to `dist`;
no target may contain `src`, a source alias, or a `main` fallback. `./icons`
resolves to compiled JavaScript and declarations:

```text
./styles/*          -> ./dist/styles/*
./tokens/styles.css -> ./dist/tokens/generated/css/poodle-tokens.css
```

Shell builds externalize their runtime dependencies. Svelte externalizes
`svelte`, core, and `marked`; React externalizes `react`, `react-dom`, core,
and `marked`. Core has no `marked` edge. Only `./markdown` has the parser edge.
`marked: ^18.0.9` is an optional shell peer documented as required when that
entry is imported. Svelte's peer floor is `>=5.56.8 <6`; lowering it requires
a separately proven older compiler/runtime build.

CSS stays in core. Shell output retains exact core style imports and emits no
unrelated all-components stylesheet. Focused ordinary Button/Select graphs may
contain only the style subpaths they use and must not contain `marked`.
Markdown graphs contain `marked` and only their required markdown styles.

## Svelte client and server lanes

Svelte is compiled twice from one explicit, sorted entry inventory. Vite
library mode produces stable public and chunk names. The browser lane emits
`*.client.js`; Vite `build.ssr: true` produces `*.server.js`. Declaration emit
is separate. `@sveltejs/package` output is staging input only: only declaration
files are copied, never raw `.svelte` files. The distribution toolchain pins
TypeScript `6.0.3`, the newest supported major for the locked `svelte2tsx`
emitter; TypeScript 7 returns only after upstream declaration support exists.

The target shape is:

```text
dist/
  index.client.js       index.server.js
  Button.client.js      Button.server.js
  Select.client.js      Select.server.js
  markdown.client.js    markdown.server.js
  chunks/*.client.js    chunks/*.server.js
  index.d.ts            markdown.d.ts
  types.js              types.d.ts
  *.svelte.d.ts
  .poodle-build.json
```

Public `*.svelte` names are import subpaths, not permission to ship Svelte
source. The `./*.svelte` key may resolve only the 176 roster names. Internal
Svelte files such as `DragDropProvider` and `MenuSurface` compile as chunks,
not public `dist/<Name>.client.js` basenames. Export conditions are exact:

```json
{
  ".": {
    "types": "./dist/index.d.ts",
    "browser": "./dist/index.client.js",
    "default": "./dist/index.server.js"
  },
  "./*.svelte": {
    "types": "./dist/*.svelte.d.ts",
    "browser": "./dist/*.client.js",
    "default": "./dist/*.server.js"
  },
  "./markdown": {
    "types": "./dist/markdown.d.ts",
    "browser": "./dist/markdown.client.js",
    "default": "./dist/markdown.server.js"
  },
  "./types": {
    "types": "./dist/types.d.ts",
    "browser": "./dist/types.js",
    "default": "./dist/types.js"
  }
}
```

`browser` selects client output. Node, worker-like, and unknown SSR consumers
fall through to the server `default`. `import` is not an environment selector
and is absent from the Svelte condition map. `./types` keeps both compiled
runtime and declaration reachability. There is no top-level `svelte` field or
`svelte` condition until both browser and SSR resolution are separately
proven. A tool that cannot express the dual shape is a stop condition, not a
reason to point at source or reuse a client artifact.

React uses one JavaScript lane and stays private. Its public keys are `.`,
`./markdown`, `./types`, and one `./<Name>` per frozen roster name. It has no
`./*` wildcard. Exact maps, CSS inventories, icon modules, receipt JSON, and
the markdown migration live in spec 070.

## Build ownership and determinism

A repo-owned driver wraps Vite library mode, existing shell plugins, and
separate declaration emit. It owns clean staging, explicit sorted entries,
stable names, dependency externalization, asset copying, source rejection, and
deterministic receipts. Core JavaScript/CSS and TypeScript declarations form
the substrate; Svelte and React compile on top of it.

Declarations use `emitDeclarationOnly` with `declarationMap: false` and must
resolve under TypeScript Bundler and NodeNext. Raw `.svelte`, non-declaration
`.ts`/`.tsx`, every `.map`, and `src/` are forbidden in staged and packed
outputs. Permitted declaration suffixes are `.d.ts`, `.d.mts`, and `.d.cts`.

Every package writes `dist/.poodle-build.json` without timestamps or absolute
paths. The receipt records package/version, source commit, locked tool
versions, sorted input/output inventories, client/server choice, CSS/markdown
policy, and source-map policy. Two clean builds must have identical file
inventories and hashes. Two clean packs must have identical archive hashes.
Certification must inspect the actual archive member
`package/dist/.poodle-build.json`; packer dotfile behavior is never assumed.

## Installed certification

`test:web-pack-install` is the only permanent installed-distribution
certification harness. It runs from a clean temporary checkout of one exact
commit and owns the accepted receipt. Earlier browser/SSR probes are disposable
implementation smokes, not certification.

The harness must:

1. build and pack core, Svelte, and private React from clean staging;
2. inspect every export and wildcard target, rejecting source, maps, missing
   JavaScript/declarations, workspace metadata, and sibling paths;
3. install archive `file:` references into a fresh no-workspace consumer with
   concrete peers and no TypeScript paths or source aliases;
4. mount installed root/direct Button and Select plus `./markdown` under the
   browser condition, with expected DOM and no page errors;
5. render those installed Svelte entries through `svelte/server` under normal
   Node resolution, and prove a client artifact itself fails SSR;
6. pass browser and SSR at Svelte `5.56.8`, while retaining a visible
   below-floor negative such as `5.38.6`;
7. compile root, direct, markdown, React, HistoryEntry, and Tree declarations
   under Bundler and NodeNext, including unsuppressed expected failures;
8. prove exact CSS side effects/subpaths, parser isolation, and a clear missing
   `marked` failure for `./markdown`; and
9. compare two builds and packs, receipt membership/provenance, one canonical
   public-roster denominator, artifact-set identity, and exact source commit.

The canonical roster, spec 070, and package-install fixture must agree on one
derived 176-name denominator. A hand-maintained 175/176 disagreement is a
blocking defect.

## Release boundary

Cards `g16.056` through `g16.059` implement this architecture serially.
`g16.059` must accept an installed browser/SSR receipt from exact main before
`g16.054` may freeze the immutable `0.3.0` candidate. `g16.054` owns version
changes, changelog and release-note history, candidate identity, tags,
registry/release checks, and later release authority. These distribution cards
must not tag, publish, dispatch workflows, mutate registries, edit sibling
repositories, or claim that a green package is released.

React remains validation-only. Actual `0.3.0` notes must name the root markdown
break, but only `g16.054` edits those release notes and candidate history.

## Fail-closed rules

Stop if browser/SSR selects the wrong lane; a client artifact renders through
`svelte/server`; a package requires raw source, workspace resolution, paths,
diagnostic suppression, or a compatibility alias; `./types` loses runtime or
declaration reachability; an export is missing or exposes an unreviewed file;
CSS or parser isolation drifts; declarations/maps/source violate the boundary;
the declared Svelte floor fails; builds or packs differ; receipt provenance is
nondeterministic; the roster denominator disagrees; React becomes publishable;
or any workflow, release, tag, registry, or sibling mutation appears.
