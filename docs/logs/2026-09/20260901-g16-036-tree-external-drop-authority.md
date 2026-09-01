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
Base: `origin/main` at `8f60700ffdde723c2e0ea009296cc20aacfe3e4f`
Planning commit `169651f51b3443ffad08f4dd435198384deacebd` is an ancestor.

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
  hover fields, rewritten dest into a moving subtree refuses,
  `treeAcceptedDropDepth` uses commit dest not hovered row.
- Paired Svelte/React Tree tests: latch across two sessions, host withhold
  before accepted paint, rewritten depth/announcement/commit, live `canDrop`
  at release, mid-session authority removal does not fall through to
  `onReorder`, pending Promise stays dropping and a stale answer is inert,
  Alt+↑/↓, invalid projection not normalized to `[source]`, convenience
  `onReorder` pointer + Alt+↑/↓.
- Mounted Chromium + WebKit: latched moving set on pointer drop, host refuse
  never paints accepted, Alt+Arrow through the same authority.
- Packed tarball types: positive identity across core and both Svelte public
  paths; exclusive union fails with
  `Types of property 'onReorder' are incompatible.` React package root is a
  TSX graph, so the packed `tsc` proof does not import it; React `TreeProps`
  is `TreeCommonProps & TreeReorderProps` in source.

## Oracle falsification

Commit the green proofs, then plant each pre-fix row, confirm the named proof
fails for the intended reason, restore from that commit (not `git checkout --`
on a dirty index), rerun green. Recorded in the PR.

## Validation

- `bun test packages/core/test/tree.test.ts packages/core/test/drag-drop-geometry.test.ts` — 37 pass
- `bunx vitest run packages/svelte/components/test/Tree.test.ts packages/react/components/test/Tree.test.tsx` — 74 pass
- `effigy test:drag-drop-browser-chromium` — pass, including Tree authority rows
- `effigy test:drag-drop-browser-webkit` — pass, including Tree authority rows
- `effigy test:web-pack-install` — pass, `packedTreeReorderProof` present
