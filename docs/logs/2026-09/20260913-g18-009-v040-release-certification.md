# g18.009 — v0.4.0 release certification and Desktop unblock (2026-09-13)

Status: released — `v0.4.0` published, immutable evidence recorded
Card: `docs/roadmaps/g18/009-v040-release-certification-and-desktop-unblock.md`
Queue task: `4a9eb48a-300c-4398-8669-a832145c7683`
Branch: `ns-4a9eb48a-300c-4398-8669-a832145c7683`
Operator ruling: 2026-09-13 (npm/web-only wrapper repair authorized; aggregate
GPUI/native gate replaced by the accepted g18.006 npm certificate)

## Outcome

`@inflatable-cookie/poodle-core@0.4.0` and
`@inflatable-cookie/poodle-svelte@0.4.0` are published to npm with `latest`
pointing at `0.4.0` and attested GitHub provenance. The immutable lightweight
tag `v0.4.0` resolves to the released commit. A fresh source-free consumer
installed the exact registry versions and proved the Svelte `./editor` entry in
declared-type, SSR, and browser modes. The Desktop capsule is recorded below.

## Superseded first branch dry run

The first hosted branch dry run (`34728955353`, candidate branch
`ns-17ac3fee-de90-4b32-9672-1134770bb086` at `9d18a21b`) spent six hours inside
the aggregate `effigy release gates` step and never produced a gate result. The
operator cancelled it and ruled that this npm/web release must not be gated by
the aggregate headless native/GPUI lane. That run was **not** retried, and its
`Release gates` step no longer exists in the workflow.

## Identity

| Identity | Value |
| --- | --- |
| Planning commit (`main`) | `e40485ab17af865ad72976b010a2d00b40cfa26b` |
| Accepted g18.006 candidate head | `9d18a21bdf344d4b659e5a56ab6b6b4b80d6891a` |
| g18.006 merge commit on `main` | `567fe01c33e7e80514cff6f7d14c516a9a9a788b` |
| Frozen release-input commit | `a797ce413d378795a8698d6782b003560878315a` |
| Released wrapper commit (`HEAD` of branch) | `4a39055f3e2ebf2ed2e220c4a59e10ee7458927f` |
| Tag | `v0.4.0` → `4a39055f3e2ebf2ed2e220c4a59e10ee7458927f` |

The wrapper commit differs from the accepted candidate only in the release
workflow wrapper and its guards:

| Path | Change |
| --- | --- |
| `.github/workflows/release.yml` | aggregate gate step replaced by the npm/web proof |
| `scripts/check-release-automation.ts` | release-wrapper law + planted negatives |
| `test/package-install/scope.ts` | bounded ordinary admission for the wrapper range |
| `test/package-install/scope.test.ts` | wrapper-range coverage |

Package trees are byte-identical to the frozen release-input commit
`a797ce413` (`git rev-parse <commit>:<path>`):

| Path | Tree |
| --- | --- |
| `packages/core` | `48c62d9afa0f8e0a59a61032f56288c5d00c2627` |
| `packages/svelte/components` | `e1c24eb3b81300155977d3448fa7e48a5fc00041` |
| `packages/react/components` | `530bf9ce8f4b010accca728a69668900e2b15cc0` |
| `packages/tokens` | `742ac9c846fbab1e7e3c96bff973bf9ec5b1aa49` |
| `packages/contracts` | `f810cccf6e79edd6d6400242089e91bde3e663ec` |
| `packages/render` | `d7bd58d6d3cb459fedd4cd5ac9b6edb4d6ef14a8` |
| `packages/release-manifest.json` | `ec0148b2b350aa4f8e85d55267ab08c3b365e596` |
| `packages/release-operations.json` | `60bc2df381fbc47cd0d99acf1caf73266e5ea6eb` |

Version set stays lockstep `0.4.0`: the private root manifest, both published
web manifests, the private React manifest, all 17 Cargo manifests with their
intra-repository `poodle-*` requirements, and both tracked locks. The workflow's
own branch lockstep step passed on the wrapper commit.

## Ordered release sequence

| Order | Step | Run / artifact | Ref | Head | Result |
| --- | --- | --- | --- | --- | --- |
| 1 | Hosted branch dry run | [`34743528777`](https://github.com/inflatable-cookie/poodle/actions/runs/34743528777) | `ns-4a9eb48a-300c-4398-8669-a832145c7683` | `4a39055f3` | success, `06:45:20Z → 06:49:01Z` |
| 2 | Immutable tag `v0.4.0` | `git push origin refs/tags/v0.4.0` | `refs/tags/v0.4.0` | `4a39055f3` | pushed `2026-09-13T06:49:41Z` |
| 3 | Tag dry run | [`34743709181`](https://github.com/inflatable-cookie/poodle/actions/runs/34743709181) | `v0.4.0` | `4a39055f3` | success, `06:49:45Z → 06:53:42Z` |
| 4 | Publish | [`34743884234`](https://github.com/inflatable-cookie/poodle/actions/runs/34743884234) | `v0.4.0` | `4a39055f3` | success, `06:53:58Z → 06:57:44Z` |

The branch dry run ran the narrow wrapper steps and passed on the first
attempt: `Build the published web packages`, `Release automation guard`,
`Hosted npm/web package proof`, `Versions agree with each other`,
`Pack and verify contents`. Both tag runs reported `Versions agree with the
tag` success, and the publish run's `Publish` step emitted both publications
with provenance.

Workflow wrapper law (asserted by `effigy check:release-automation`, which the
workflow runs before packing, and by the ordinary scope admission):

- required: `effigy svelte:package`, `effigy check:release-automation`,
  `effigy test:web-pack-install`;
- forbidden: `effigy release ...`, `effigy qa`, `effigy ci...`,
  `effigy check:gpui`, `effigy gpui:test`, `effigy regressions:native`,
  `effigy probe:gpui-specimens`, `effigy test:contracts`, `effigy test:core`,
  `effigy test:components`, `effigy audit:security`, `effigy audit:licenses`;
- planted negatives prove the guard bites: aggregate gate, `effigy qa`,
  `effigy ci:native`, `effigy regressions:native`, and a dropped
  installed-package proof all fail the check.

## Published tarball integrity

`packed-tarballs` artifact of publish run `34743884234`, compared byte-for-byte
with the registry tarballs:

| Package | Version | Bytes | SHA-256 | SHA-1 (`dist.shasum`) |
| --- | --- | --- | --- | --- |
| `@inflatable-cookie/poodle-core` | `0.4.0` | 440719 | `7b59cf2b380765d3303c8dd0481edc2fc878b4fde811c1dbb8cf87a01a97b06b` | `f0e5ae8711982856d25f2d5e7bf8005cbc845097` |
| `@inflatable-cookie/poodle-svelte` | `0.4.0` | 556844 | `c590fe7c1972e66038ee922d92f528fdbd7f4df663781699680520d3c1565ce2` | `a55714b1001d2a947103e409a89a4f0cc297516f` |

Registry metadata:

| Package | `latest` | `dist.integrity` | Files | Unpacked | Provenance |
| --- | --- | --- | --- | --- | --- |
| core | `0.4.0` | `sha512-70GKvK4luj3IIRWzNNiNzYhjue6bnlUjrtFqADoXYtQ7pg9BScfTU6dETN7bfriHqC24nTD5Lx8y+2e4yOVnAA==` | 543 | 2168145 | attested (log `2814967395`) |
| svelte | `0.4.0` | `sha512-Zb1yFtm1ESbVgSfn/amqWaKpDjRTJT12l/7vNiSZ4qX0Wp2ITc6oe9V+aTpOFAd/Bdv42UK0XtZsq+QYsbkgVg==` | 670 | 3057046 | attested (log `2814968683`) |

Both registry downloads are byte-identical to the artifact (`cmp` clean), and
both integrity digests match the packument, the lockfile, and the publish log.
npm reported `poodle-svelte` as "being processed and may take a few minutes";
the packument and tarball became available at `07:01Z` and `07:04Z`
respectively. No retry or re-publish was performed.

Published build receipts (`dist/.poodle-build.json`) inside the registry
tarballs record `version 0.4.0` and `sourceCommit 4a39055f3e2ebf2ed2e220c4a59e10ee7458927f`
(the wrapper commit). The receipts stamp the checkout `HEAD`; the package trees
they were built from are the byte-identical `a797ce413` trees above.

Compiled members required by the Desktop consumer are present in the Svelte
tarball: `dist/editor.d.ts`, `dist/editor.client.js`, `dist/editor.server.js`,
`dist/CodeEditor.svelte.d.ts`, `dist/rich-text.d.ts`,
`dist/rich-text.client.js`, `dist/rich-text.server.js`,
`dist/editor-codemirror.{js,d.ts}`. The core tarball carries its licence,
readme, manifest, receipt, 114 compiled icon modules, and generated token CSS.

## Bounded public set

- npm `@inflatable-cookie/poodle-core` and `@inflatable-cookie/poodle-svelte`
  are the only packages with a new version (`0.4.0`, `latest`).
- `poodle-react`, `poodle-tokens`, `poodle-svelte-preview`,
  `poodle-install-smoke` remain unpublished (registry `404`).
- No Poodle crate is published on crates.io; Rust stays on source/tag
  distribution.
- The publish step's package set is still exactly `packages/core` and
  `packages/svelte/components`.

## Fresh source-free consumer proof

Fresh temporary directory, no workspace, no source links, no path aliases.
Ordinary npm install resolved both packages from the registry (peer resolution
enabled; no overrides):

```text
g18-009-runtime-consumer@ /private/tmp/g18-009-runtime-only
├── @inflatable-cookie/poodle-core@0.4.0
├─┬ @inflatable-cookie/poodle-svelte@0.4.0
│ ├── @inflatable-cookie/poodle-core@0.4.0 deduped
│ └── svelte@5.56.8 deduped
└── svelte@5.56.8
```

```text
node_modules/@inflatable-cookie/poodle-core    resolved .../poodle-core-0.4.0.tgz    integrity sha512-70GKvK4l…
node_modules/@inflatable-cookie/poodle-svelte  resolved .../poodle-svelte-0.4.0.tgz  integrity sha512-Zb1yFtm1…
node_modules/svelte                            resolved .../svelte-5.56.8.tgz         integrity sha512-PY8LOw7x…
```

The harness devDependencies were added afterwards with
`npm install --legacy-peer-deps` because the local npm 10.9.8 arborist crashes
on `vitest`'s optional browser peer set (`Cannot read properties of null
(reading 'edgesOut')`); the released runtime graph was unchanged. The browser
lane then ran under `vitest` 4.1.10 with `happy-dom` 20.11.1 and the Svelte
plugin's `browser` condition.

| Mode | Command | Result |
| --- | --- | --- |
| default resolution | `node resolve.mjs` | `./editor → dist/editor.server.js` |
| browser resolution | `node --conditions=browser resolve.mjs` | `./editor → dist/editor.client.js` |
| SSR | `node ssr.mjs` (`svelte/server` `render`) | `SSR ok: body 144 bytes, head 0 bytes`, body carries `.poodle-code-editor` |
| declared types (Bundler) | `tsc -p tsconfig.bundler.json` | exit 0 (`CodeEditor` + re-exported core types) |
| declared types (NodeNext) | `tsc -p tsconfig.nodenext.json` | exit 0 |
| browser mount | `vitest run` (happy-dom, `browser` condition) | 1 file / 1 test passed; `.poodle-code-editor` root and CodeMirror `.cm-editor` engine present |

The proof imports `CodeEditor` from `@inflatable-cookie/poodle-svelte/editor`
exactly as a consumer does, and the SSR/browser lanes load the packed
`editor.server.js` / `editor.client.js` artifacts. `poodle-svelte@0.3.0` has no
`./editor` entry, so this is the first published version that satisfies the
Desktop consumer.

## Boundaries held

No Desktop repository, task, workspace, or PR was touched. No React or crate
publication. No workflow change beyond the operator-authorized wrapper repair,
no `effigy.toml` gate change, no candidate rebuild or amendment, no retag, and
no retry of the cancelled aggregate run. The evidence and closeout commit
changes only documentation.

## Desktop capsule

```text
version:                 0.4.0
candidate (g18.006):     9d18a21bdf344d4b659e5a56ab6b6b4b80d6891a
merge on main:           567fe01c33e7e80514cff6f7d14c516a9a9a788b
released wrapper commit: 4a39055f3e2ebf2ed2e220c4a59e10ee7458927f
tag:                     v0.4.0 -> 4a39055f3e2ebf2ed2e220c4a59e10ee7458927f
package trees:           core 48c62d9afa0f8e0a59a61032f56288c5d00c2627
                         svelte e1c24eb3b81300155977d3448fa7e48a5fc00041
                         (byte-identical to a797ce413)
npm latest:              @inflatable-cookie/poodle-core 0.4.0
                         @inflatable-cookie/poodle-svelte 0.4.0
tarball sha256:          core 7b59cf2b380765d3303c8dd0481edc2fc878b4fde811c1dbb8cf87a01a97b06b
                         svelte c590fe7c1972e66038ee922d92f528fdbd7f4df663781699680520d3c1565ce2
workflow runs:           branch dry run 34743528777 (green, 4a39055f3)
                         tag dry run    34743709181 (green, 4a39055f3)
                         publish        34743884234 (green, 4a39055f3)
installed result:        fresh registry install resolves ./editor in type,
                         SSR, and browser modes; svelte 5.56.8 floor satisfied
```

Desktop resumes retained task `dbea8cc2-1872-40f3-b6d8-ba91b9d37618` and PR
#215 in its existing workspace: unlink the source alias, repin core/Svelte to
`0.4.0`, and finish Phase B. Poodle did not edit Desktop or its queue record.
