# Compiled Web Distribution Prerequisite

Status: open — recommendation for orchestrator acceptance; implementation not dispatched
Captured: 2026-09-02
Owner: Poodle Northstar orchestrator
Source handoff: [`20260902-021250-web-distribution-prerequisite.md`](../handoffs/20260902-021250-web-distribution-prerequisite.md)
Base refreshed to: `1b0d4032984a65c64398fbcd71572a9093a47ace` (`origin/main`)
Canonical release card: [`g16.054`](../roadmaps/g16/054-historycenter-v030-release-candidate.md)
Blocking reviews: [initial evidence review](https://github.com/inflatable-cookie/poodle/pull/158#issuecomment-5505928659), [SSR follow-up](https://github.com/inflatable-cookie/poodle/pull/158#issuecomment-5506072722)

## Decision

Promote one separately accepted web-distribution prerequisite before `g16.054`
may freeze the `0.3.0` candidate. This packet is planning and evidence only. It
does not add manifests, build scripts, workflows, generated output, versions,
release tags, or implementation code.

The operator accepted the breaking `0.3.0` boundary. In both web shells,
`AgentMessage` and `MarkdownEditor` leave the root value barrel and are exposed
from an explicit `./markdown` entry:

```ts
// before
import { AgentMessage } from "@inflatable-cookie/poodle-svelte";

// after
import { AgentMessage } from "@inflatable-cookie/poodle-svelte/markdown";
```

The React migration is the corresponding
`@inflatable-cookie/poodle-react/markdown` import. The root Button/Select
surface is intentionally parser-free. The direct component subpaths remain
only where they are explicitly included in the export inventory. No root alias,
compatibility shim, conditional “if admitted” branch, or silent fallback may
preserve the old markdown imports. The roster, migration note, and `0.3.0`
release notes must name this break.

The public release set remains core plus Svelte. React gets the same compiled
shape in the certification plan, but stays `private` and unpublished until a
named consumer is admitted.

The package boundary is `dist/`: compiled JavaScript, declarations, core CSS,
the existing package documentation/licence surface, and one deterministic
build receipt. No release tarball may expose `src/`, non-declaration TypeScript,
TSX, or raw Svelte source.

The Svelte result requires two compiler lanes. The browser lane emits
`*.client.js`; the SSR lane emits `*.server.js`. Installed export conditions
select those lanes. A client artifact is not an SSR fallback, and raw `.svelte`
source is never a fallback.

The CSS contract is exact:

```json
"sideEffects": ["**/*.css"]
```

Apply that array to core, Svelte, and the private React validation package.
Keep CSS in core and retain the exact core style subpaths in shell imports. Do
not create an unrelated all-components shell stylesheet.

## Current boundary and prior review repairs

The inspected manifests are [`packages/core/package.json`](../../packages/core/package.json),
[`packages/svelte/components/package.json`](../../packages/svelte/components/package.json),
and [`packages/react/components/package.json`](../../packages/react/components/package.json).
They currently expose `src` and therefore do not establish a release boundary.
The matching release-adoption decision is
[`20260901-230400-history-release-adoption-planning.md`](../handoffs/20260901-230400-history-release-adoption-planning.md);
the handoff's alternate path is absent on this base and is not reopened here.
The architecture keeps shared behavior and styles in core, so the distribution
must preserve core CSS subpaths rather than move app-specific styles into a
shell bundle: [`001-poodle-system-shape.md`](../architecture/001-poodle-system-shape.md)
and [`002-token-system-and-package-layout.md`](../architecture/002-token-system-and-package-layout.md).

`effigy svelte:build` and `effigy react:build` build preview applications;
`test:core`, `check:svelte-components`, and `test:components` validate source
graphs. `test:web-pack-install` is the installed-tarball selector and is the
right certification surface to upgrade.

The existing source-only installed-pack baseline on this branch completed 20
tests at `2026-09-02 01:19:14.488Z`, artifact set
`3c525bb2ff2308d3a4a5b86c6cb1f568710a3f33f9b9fd6966ffbbed79a67daa`:

| Package | Bytes | SHA-256 | Archive files |
| --- | ---: | --- | ---: |
| `@inflatable-cookie/poodle-core` | 343,419 | `e058e7c6713b92fc02ec7402673165f42180292c36842bddc2b72b2e8f76b839` | 407 |
| `@inflatable-cookie/poodle-svelte` | 277,669 | `4f8e0587ed9a6f3d58636e12cca6a5c0db04e7f0b6601773333b6b5a074990fc` | 227 |
| `@inflatable-cookie/poodle-react` | 274,748 | `1021b5c6b82ed4957b0c4acb653dff20bb0fbba697e9076c15abf9a1275a178` | 219 |

That is boundary evidence, not release evidence: it packs raw `src`, uses no
compiled client/server lanes, and does not prove public imports from a
source-free tarball. Its React assignability fixture maps to `src/types.ts`
because the source value barrel does not compile; the future proof must compile
the installed value barrel without that mapping.

The proof surface also has one denominator drift to repair. The canonical
roster and the live Svelte/React root sources use 176 names, while
`test/package-install/web-preview.ts` records 175. The implementation must
derive one denominator from one canonical source, fail on disagreement, and
record the accepted markdown root-barrel change without silently changing the
roster.

## Disposable compile/install spike

The uncommitted spike ran from the rebase-refreshed checkout at source commit
`93129e6da819616c3228dd90d14640abfdb7c01e` with:

```sh
env -u KEEP_SPIKE bun .spike-web-distribution.mjs
```

The script and all temporary directories were removed after the run. It built
only temporary staging trees, packed those trees, and installed the resulting
tarballs into fresh consumers with `file:` references. The core dependency was
also pinned through the consumer's explicit `overrides` entry so the staged
core tarball, not a workspace package or registry copy, satisfied the staged
Svelte package's concrete `0.2.3` dependency.

The tested toolchain was Bun `1.3.14`, Vite `8.2.1`, Svelte
`5.56.8`, `@sveltejs/vite-plugin-svelte` `7.3.0`, `@sveltejs/package`
`2.5.8`, TypeScript `7.0.2`, Playwright `1.62.1`, and `marked` `18.0.9`.
The Svelte plugin declares peer `svelte: ^5.46.4`; the spike showed that peer
range is not a sufficient runtime floor for output compiled by Svelte 5.56.8.

### Build findings

The ordinary Vite library build produced hashed internal files:
`Button-C2seD57f.js`, `Icon-BFY1a9K2.js`, `Select-BoQTx72Y.js`, and
`presentation-CEYLS1uR.js`. Public entry names stayed stable only after
explicit `entryFileNames`, `chunkFileNames`, and `assetFileNames` templates
were supplied. The recommendation therefore uses Vite library mode with an
explicit sorted entry inventory and stable public/chunk names.

The client build produced these files:

```text
Button.client.js
Select.client.js
index.client.js
markdown.client.js
chunks/Button.client.js
chunks/Icon.client.js
chunks/Select.client.js
chunks/presentation.client.js
```

The SSR build produced the same entry inventory with `.server.js` names. Its
compiled graph imports `svelte/internal/server`; the client graph imports
`svelte/internal/client` and `svelte/internal/disclose-version`. Passing
`compilerOptions.generate` to the Svelte Vite plugin did not select the lane;
the tested driver uses Vite `build.ssr: true` for the server build. This is why
one client artifact cannot be the distribution's honest SSR target.

The core build used Vite for JavaScript/CSS and TypeScript declaration-only
emit. The proven declaration command was:

```sh
bun x tsc --ignoreConfig --target ES2022 --module ESNext \
  --moduleResolution Bundler --strict --skipLibCheck --declaration \
  --emitDeclarationOnly --declarationMap false --outDir <temporary-dist> \
  --rootDir packages/core/src --allowImportingTsExtensions false \
  packages/core/src/index.ts
```

The Svelte declaration command was:

```sh
bunx --bun @sveltejs/package --input src --output <temporary-types> \
  --tsconfig tsconfig.json
```

`@sveltejs/package` emits raw `.svelte` files as part of its normal package
operation. The spike copied only its declaration files into the staged
package; it never copied the raw files. Its actual component declaration names
are `Button.svelte.d.ts`, `Select.svelte.d.ts`, and so on. The export wildcard
must therefore map `./*.svelte` types to `./dist/*.svelte.d.ts`, not to the
nonexistent `./dist/*.d.ts` shape.

### Installed browser and SSR oracle

The installed consumer imported all of the following from the staged Svelte
tarball, with no source alias:

- root `Button` and `Select`;
- direct `Button.svelte` and `Select.svelte` public subpaths;
- `AgentMessage` from `./markdown`.

The browser fixture used Vite's `browser` condition and `mount`. Against the
matching Svelte `5.56.8` runtime it rendered 2 Buttons, 2 Selects, and the
markdown strong element, with no page errors.

The server fixture used `svelte/server` and the package's Node resolution. It
called `render` for root Button/Select, direct Button/Select, and markdown.
All five checks returned the expected `poodle-button`, `poodle-select`, and
markdown markup. The output lengths were `[296, 736, 296, 736, 390]`.

The two independent export lanes were not inferred from filenames alone. The
spike also tried to render the installed `Button.client.js` directly through
`svelte/server`; it failed with `Error: https://svelte.dev/e/effect_orphan`,
as required. The server condition must select `Button.server.js`; client
output must not be reused for SSR.

The separate revised-floor consumer installed Svelte `5.56.8` and passed the
same browser and server oracle. A below-floor Svelte `5.38.6` consumer could
install but failed the browser mount with
`TypeError: target.exclude.includes is not a function`. A separate
`5.46.4` repetition produced the same failure. The earlier `>=5.38.6 <6`
recommendation is therefore falsified for this compiler output. The honest
floor for this toolchain is `>=5.56.8 <6`; lowering it requires a separately
proven older compiler/runtime build, not a relaxed peer range.

The declaration fixture imported root, direct, and markdown declarations from
the installed tarball. Both TypeScript `Bundler` and `NodeNext` checks passed.
The markdown-without-`marked` fixture exited status 1 and mentioned `marked`.

### Graph, CSS, source, and tarball findings

The ordinary root graph contained core, Svelte, and these exact core style
subpaths:

```text
@inflatable-cookie/poodle-core/styles/anchored-surface.css
@inflatable-cookie/poodle-core/styles/button.css
@inflatable-cookie/poodle-core/styles/icon.css
@inflatable-cookie/poodle-core/styles/select.css
@inflatable-cookie/poodle-core/styles/spinner.css
```

It did not contain `marked`. The markdown graph contained `marked` and only
the markdown component's core style subpaths (`agent-message.css`, `code.css`,
`separator.css`, and `text-link.css`). It did not contain `list-card.css` or
`tabs.css`. The shell build emitted no CSS assets: CSS stayed as external core
style imports for the consumer bundler to load.

Two clean builds and two clean Bun packs produced these results. The source
check permits declaration suffixes `.d.ts`, `.d.mts`, and `.d.cts`; it rejects
non-declaration `.ts`, `.tsx`, `.svelte`, and every `.map` file.

| Package | Bytes | SHA-256 | Files | Declarations | Raw source | Maps | Packed receipt |
| --- | ---: | --- | ---: | ---: | --- | ---: | --- |
| `@inflatable-cookie/poodle-core` | 340,432 | `092648f2535509fba447b34724d230fa9bdab779b4661aebc5b4b129f62a86d5` | 281 | 83 | none | 0 | `package/dist/.poodle-build.json` present |
| `@inflatable-cookie/poodle-svelte` | 79,716 | `7cc353e50e26f0ad743b1372e972b986e82fa7876249b6c7ac353c44d6c941ea` | 245 | 225 | none | 0 | `package/dist/.poodle-build.json` present |

For both packages, build A and build B had identical file hashes and identical
archive SHA-256 values. Bun `1.3.14` was tested directly: the dotfile receipt
was present in the tarball, and no duplicate non-dot `poodle-build.json` was
used. The implementation must retain that exact archive-membership assertion;
it must not assume packer behavior.

## Recommended build and package contract

Use a repo-owned build driver around [Vite library mode](https://vite.dev/guide/build#library-mode),
the existing Svelte and React Vite plugins, and a separate declaration emit.
Vite's [`build.ssr`](https://vite.dev/config/build-options.html#build-ssr)
must drive the Svelte server lane. Preview applications remain consumers of
the built packages; they do not define release semantics.

Use explicit sorted entries and externalize runtime dependencies:

- core has no runtime `marked` edge;
- Svelte externalizes `svelte`, core, and `marked`;
- React externalizes `react`, `react-dom`, core, and `marked`;
- only the `./markdown` entry has the parser edge.

The Svelte package's proposed runtime/declaration shape is:

```text
dist/
  index.client.js       index.server.js
  Button.client.js      Button.server.js
  Select.client.js      Select.server.js
  markdown.client.js    markdown.server.js
  chunks/*.client.js    chunks/*.server.js
  index.d.ts             markdown.d.ts  types.d.ts
  *.svelte.d.ts
  .poodle-build.json
```

The `*.svelte` part of a public import is only a subpath name. It never grants
permission to ship a `.svelte` file.

The Svelte export map must select the two compiled lanes explicitly:

```json
{
  ".": {
    "types": "./dist/index.d.ts",
    "browser": "./dist/index.client.js",
    "node": "./dist/index.server.js",
    "import": "./dist/index.client.js",
    "default": "./dist/index.server.js"
  },
  "./*.svelte": {
    "types": "./dist/*.svelte.d.ts",
    "browser": "./dist/*.client.js",
    "node": "./dist/*.server.js",
    "import": "./dist/*.client.js",
    "default": "./dist/*.server.js"
  },
  "./markdown": {
    "types": "./dist/markdown.d.ts",
    "browser": "./dist/markdown.client.js",
    "node": "./dist/markdown.server.js",
    "import": "./dist/markdown.client.js",
    "default": "./dist/markdown.server.js"
  },
  "./types": {
    "types": "./dist/types.d.ts"
  }
}
```

Do not add a top-level `svelte` field pointing at one lane. Do not add a
`svelte` condition until its resolver behavior is proven for both browser and
SSR consumers. If a tool requires that field and cannot express the dual
shape, stop for an explicit compatibility decision; do not point it at raw
source or silently reuse the client artifact.

Core uses the same `types`/`import`/`default` pattern for JavaScript entries,
with `./icons` targeting `dist/icons.js` and its declarations. Its CSS-only
subpaths remain explicit:

```text
./styles/*          -> ./dist/styles/*
./tokens/styles.css -> ./dist/tokens/generated/css/poodle-tokens.css
```

Rewrite the complete existing token inventory mechanically and test every
target from the installed tarball. No target may contain `src`, a source alias,
or a `main` fallback.

The exact package content and side-effect policies are:

```json
// core
"files": ["dist", "README.md", "LICENSE", "THIRD_PARTY_NOTICES.md"],
"sideEffects": ["**/*.css"]

// Svelte and private React validation packages
"files": ["dist", "README.md", "LICENSE"],
"sideEffects": ["**/*.css"]
```

Svelte's peer must be `svelte: ">=5.56.8 <6"`. `marked: "^18.0.9"` is an
optional peer of the shell, documented as required when `./markdown` is
imported. The root Button/Select graph must remain parser-free. The package
manager's `files` behavior must be tested, including the actual packed
`package/dist/.poodle-build.json` member.

Emit TypeScript declarations with `emitDeclarationOnly` and
`declarationMap: false`; the same declaration gate must pass Bundler and
NodeNext resolution. See [TypeScript declaration-only emit](https://www.typescriptlang.org/tsconfig/emitDeclarationOnly.html).
The Svelte declaration emitter is a staging input only. The packaging guide's
normal raw `.svelte` behavior is not a release fallback; see
[Svelte packaging](https://svelte.dev/docs/kit/packaging) and the
[Svelte packaging discussion](https://github.com/sveltejs/kit/discussions/8825).

Write one deterministic `dist/.poodle-build.json` receipt with no timestamp or
absolute path. It records package/version, source commit, locked tool versions,
sorted output/input inventory, the dual-lane decision, CSS/markdown decisions,
and source-map policy. The installed-tarball receipt records archive bytes,
SHA-256, content inventory, export checks, consumer versions, negative proofs,
and exact source commit.

## Installed-tarball oracle

`test:web-pack-install` remains the certification selector. It must be upgraded
to run this oracle from a clean temporary checkout of the exact commit:

1. Build core, Svelte, and private React into clean staging directories. Pack
   only the staged `dist` boundary and package documents.
2. Inspect every export target and wildcard match. Require existing `dist`
   JavaScript and declaration targets; reject raw `.svelte`, non-declaration
   `.ts`/`.tsx`, `.map`, `src`, workspace metadata, and sibling paths.
3. Install the tarballs into a fresh no-workspace consumer with concrete peer
   versions and `file:` archive references. No `paths`, workspace links, or
   source aliases are allowed.
4. Run the browser fixture through installed public imports: root and direct
   Button/Select plus `./markdown`, with the browser export condition. Assert
   the mounted DOM and no page errors.
5. Run the server fixture through installed public imports with `svelte/server`:
   root and direct Button/Select plus `./markdown`, with the Node export
   condition. Assert rendered HTML. Also assert that directly rendering a
   `.client.js` artifact fails, so SSR cannot silently reuse the client lane.
6. Run the revised Svelte `5.56.8` floor leg. Keep a below-floor negative leg
   such as `5.38.6`; a failure is expected and must remain visible. A declared
   floor that fails either browser mount or SSR blocks acceptance.
7. Compile root, direct, markdown, and React declarations without source
   mapping. Keep the positive `continuationCount` case and the unsuppressed
   `branchCount` expected failures, plus the Tree reorder positives and four
   expected failures.
8. Assert the exact CSS `sideEffects` array, exact core style/token subpaths,
   no unrelated CSS in the focused Button/Select graph, no `marked` in the
   ordinary graph, and a clear missing-peer failure for markdown without
   `marked`.
9. Build twice and compare sorted output inventories and every file hash. Pack
   twice and compare tarball SHA-256, receipt membership, provenance, and the
   one canonical roster denominator. Record the artifact set ID and exact
   source commit.

This oracle owns installed browser and SSR behavior. It must not be weakened by
calling a preview app, importing sibling source, adding a hand-edited tarball,
or suppressing a diagnostic.

## Interaction with `g16.054`

`g16.054` remains blocked until this prerequisite's installed oracle has passed
both browser mount and server render from the exact mainline commit. It owns
the later immutable `0.3.0` candidate, lockstep versioning, release-history
honesty, release notes, candidate receipt, tags, registry checks, and final
release gates. It must not redesign the dual Svelte lanes, export conditions,
CSS ownership, markdown break, or dependency mechanics selected here.

The sequence is:

1. accept this contract and its evidence;
2. implement the four serial cards below;
3. verify the installed browser/SSR oracle from main;
4. let `g16.054` apply the lockstep `0.3.0` candidate changes and rerun the
   oracle against that candidate;
5. keep React private and publish core plus Svelte only.

No tag, workflow dispatch, npm publish, registry mutation, or sibling-repository
write belongs in this prerequisite.

## Serial implementation cards

The prior card overlap is removed. Card 2 stops at the core substrate; card 3
owns shell compilation; card 4 owns installed certification. Each card has one
consumer-facing exit gate.

| Order | Card | Scope | Exit gate |
| ---: | --- | --- | --- |
| 1 | Web distribution contract | Freeze the `dist` inventories, stable names, dual client/server export conditions, revised Svelte floor, declaration suffixes, core CSS ownership and exact `sideEffects`, receipt schema, source-free rule, and the accepted `./markdown` breaking migration. Update the roster/release notes only for that admitted break. | A reviewed contract has no unresolved export, SSR, floor, CSS, tarball, or dependency choice. |
| 2 | Core build substrate | Add the repo-owned build driver foundation for core JavaScript, core CSS/token/icon assets, core declarations, clean staging, sorted inventories, and deterministic `.poodle-build.json`. Do not compile Svelte or React components in this card. | Core staged output is typed, source-free, CSS-complete, deterministic, and receipt-backed. |
| 3 | Shell distributions | Compile Svelte client and server lanes with Vite and `build.ssr`, emit Svelte declarations into staging without copying raw `.svelte`, compile private React, apply the root/`./markdown` boundary and optional peer, externalize runtime dependencies, and mirror CSS semantics. | Installed Svelte 5.56.8 browser and SSR Button/Select/markdown fixtures pass; client artifact does not pass SSR; React value declarations compile without source mapping. |
| 4 | Installed certification and promotion receipt | Upgrade `test:web-pack-install` with export/content checks, browser and server oracles, below-floor negative proof, CSS/marked fixtures, roster denominator, notices, repeated-build hashes, repeated tarball hashes, and exact receipt evidence. | One clean installed-tarball receipt is accepted and can be handed to `g16.054`. |

## Stop and rollback

Stop the prerequisite on any of these conditions:

- a declared Svelte floor fails browser mount or server render;
- browser and Node resolution select the same client-only artifact, a direct
  client artifact renders through `svelte/server`, or SSR is covered by raw
  source fallback instead of a server artifact;
- a tarball contains raw `.svelte`, non-declaration `.ts`/`.tsx`, `src`, a
  source export, a missing declaration, or a source map;
- a dual client/server entry, chunk, or export target is missing, a wildcard
  exposes an unreviewed file, or any target resolves only through a workspace;
- TypeScript declarations require `paths`, sibling source, a compatibility
  alias, or diagnostic suppression;
- the exact CSS side-effect array differs, a core style/token subpath is absent,
  or unrelated `list-card`/`tabs` CSS enters the focused graph;
- a non-markdown consumer resolves or ships `marked`, or missing `marked` does
  not fail clearly for `./markdown`;
- two clean builds or packs differ, provenance has a timestamp/absolute path,
  the dotfile receipt is not present in the actual tarball, or the 175/176
  roster disagreement remains;
- root markdown names return through an alias, the accepted `./markdown`
  migration is omitted, React becomes publishable without a named consumer,
  or a workflow/release/registry mutation appears.

Before `g16.054` is accepted, rollback is a normal revert of the prerequisite
implementation cards. Keep the last published `0.2.2` registry state as the
adoption baseline; `0.2.3` remains prepared-but-unpublished and `0.3.0` stays
untagged/unpublished. Do not repair a failed gate with a source alias,
compatibility shim, hand-edited tarball, or weakened negative proof.

## Promotion route

This one-file note is the requested evidence packet. It changes no product
implementation. After review, the orchestrator may dispatch the four cards in
order. The final acceptance record must include the installed browser result,
installed SSR result, revised-floor result, client/server lane selection,
source-free archive inventory, CSS/marked negatives, deterministic hashes,
packed receipt member, canonical roster denominator, and exact mainline commit.
