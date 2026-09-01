# g16.036 — Tree External Drop Authority

Status: in-review
Date: 2026-09-01
Card: `docs/roadmaps/g16/036-tree-external-drop-authority.md`
Handoff: `docs/handoffs/20260901-202401-g16-036-tree-external-drop-authority.md`
Governing refs: `docs/contracts/components/tree.md`,
`docs/architecture/011-drag-and-drop-substrate.md`,
`docs/specs/069-dependable-drag-and-drop-substrate.md`
Branch: `feature/g16-036-tree-external-drop-authority`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-036-tree-external-drop-authority`
Base: rebased onto `origin/main` at `83e290cb996b202c0b7fa43bdf9eec6e7b3590e4`
(PR #131 nested-menu research plus PR #124 motion closeout). Planning commit
`169651f51b3443ffad08f4dd435198384deacebd` is an ancestor.

## Outcome

Svelte and React Tree gained one generic `reorderAuthority` adapter over the
landed drag substrate. The host projects an ordered moving set at semantic
activation, accepts/rewrites/refuses the Tree candidate before accepted paint,
and returns its real sync or async commit result. Ordinary `onReorder` is
unchanged when no authority is installed.

Public types live in `@inflatable-cookie/poodle-core` and are re-exported from
`@inflatable-cookie/poodle-svelte`, `@inflatable-cookie/poodle-svelte/types`,
and `@inflatable-cookie/poodle-react`:

- `TreeReorderSubject { sourceValue, movingValues }`
- `TreeReorderCandidate { subject, intent }`
- `TreeReorderAuthority { projectMovingValues, canDrop, onDrop }`
- `TreeReorderProps` mutually exclusive with `onReorder`
- React `TreeProps` is `TreeCommonProps & TreeReorderProps` on the installed
  types module

Reuse `DropIntent`, `DropEligibility`, `DragDropCommitResult`. Generic
`DragSubject` stays the source row id. The richer Tree subject is
component-owned session context.

## Native delta

Honest and unchanged. Local Node commit is synchronous. Native intent
presentation does not carry a rewritten destination. The Node subject has no
durable multi-row payload. This card does not encode moving values into an id
or add Tree-local durable session storage. `reorderAuthority` is web-only in
contract-spec-drift. No Rust, GPUI, or Jetstream source in the diff.

## Evidence

- Core helpers: validity, latch, generic-then-host eligibility, rewrite keeps
  hover fields, new-object and in-place hover-field mutation refuse with
  `unavailable` (caller intent unchanged), rewritten dest into a moving
  subtree refuses, `treeAcceptedDropDepth` uses commit dest from the full
  tree (including collapsed dest rows).
- Paired Svelte/React Tree tests: selection changes after pickup and before
  release keep the latched pair; host withhold before accepted paint;
  rewritten depth/announcement/commit; collapsed rewritten dest still paints
  dest depth; live `canDrop` at release; mid-session authority removal does
  not fall through to `onReorder`; a pending Promise calls `onDrop` once and
  surfaces the exact rejected/failed live-region text; source-loss then a
  later dropping session ignores the old Promise; Alt+↑/↓; invalid
  projection not normalized to `[source]`; convenience `onReorder` pointer +
  Alt+↑/↓.
- Mounted Chromium + WebKit: latched moving set on pointer drop, host refuse
  never paints accepted, Alt+Arrow through the same authority.
- Packed tarball types: positive identity across core, both Svelte public
  paths, and the installed React `TreeProps` boundary; exclusive union fails
  with `Types of property 'onReorder' are incompatible.` on all four
  import paths, unsuppressed. React value-barrel `index.ts` is not a
  tsc-clean graph; the proof maps the public specifier onto the installed
  tarball `src/types.ts` (the module the root re-exports).

## Oracle falsification

Prior four-blocker plants remain at `5829bf7af` / `62f395010`. This round's
plants were applied on the dirty repair tree and restored in place (HEAD did
not yet contain the snapshot/`TreeProps` proofs).

| Invariant | Plant | Failure |
| --- | --- | --- |
| Illegal hover edits are refused | skip hover-field compare; dest taken from mutated `targetId` | accepted with `destination.targetId` `"src/a.ts"` |
| Collapsed dest keeps dest depth | `treeAcceptedDropDepth` walks only `flattenVisibleTreeRows(nodes, [])` | core `null`; shells `--poodle-tree-drop-depth` `""` not `"1"` |
| Subject is a session snapshot | re-project on every `canDrop`/`onDrop` from live selection | `projectMovingValues` called 2 times at first hover, not 1 |
| Terminal result is not invented | skip promise generation guard and `currentSession` sessionId match | later source posture `null`, not `dropping` |
| In-place hover mutation is refused | pass the live `intent` into `canDrop` and build accepted state from it | accepted with `targetId` `"src/a.ts"`; caller object mutated |
| Matching Promise keeps its terminal | remap host Promise to `{ status: "committed" }` in `handleDrop` | live region `"Dropped a.ts on c.ts"`, not `"Drop rejected: late"` / `"Drop failed: late"` |
| React `TreeProps` stays exclusive | `TreeProps` as an open intersection of optional `reorderAuthority` and `onReorder` | packed React negative would compile (harness: still accepts both) |

Earlier card-oracle plants remain in the history at `8d074c279`.

## Validation

- `bun test packages/core/test/tree.test.ts packages/core/test/drag-drop-geometry.test.ts` — 40 pass
- `bunx vitest run packages/svelte/components/test/Tree.test.ts packages/react/components/test/Tree.test.tsx` — 80 pass
- `effigy test:web-pack-install` — pass, `packedTreeReorderProof` includes
  `@inflatable-cookie/poodle-react`
- `effigy ci:web` / `effigy docs:check` — rerun after rebase onto `83e290cb9`
- `git diff --check origin/main...HEAD` — rerun after rebase
- writable scope: no Rust/GPUI/Jetstream/headless/render/contracts crate files;
  no g16 README or generation-index edits in the repair commit
