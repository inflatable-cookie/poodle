// Positive packed proof for g16.036 Tree reorder types from the React root.
//
// The installed package `types` condition is `src/index.ts`, a value barrel.
// Following it typechecks the whole React TSX graph, which is not tsc-clean.
// This compile maps the public `@inflatable-cookie/poodle-react` specifier onto
// the installed tarball's `src/types.ts` — the same module the root re-exports
// — so the three shared identities and the exclusive `TreeProps` boundary are
// proved from the artifact, not from a workspace path or a handwritten .d.ts.
import type {
  TreeReorderAuthority as CoreAuthority,
  TreeReorderCandidate as CoreCandidate,
  TreeReorderProps as CoreProps,
  TreeReorderSubject as CoreSubject,
} from "@inflatable-cookie/poodle-core";
import type {
  TreeProps,
  TreeReorderAuthority as ReactAuthority,
  TreeReorderCandidate as ReactCandidate,
  TreeReorderProps as ReactProps,
  TreeReorderSubject as ReactSubject,
} from "@inflatable-cookie/poodle-react";

const authority: CoreAuthority = {
  projectMovingValues: (source) => [source],
  canDrop: (candidate) => ({ accepted: true, intent: candidate.intent }),
  onDrop: () => ({ status: "committed" }),
};

const subject: CoreSubject = {
  sourceValue: "a.ts",
  movingValues: ["a.ts", "b.ts"],
};

const candidate: CoreCandidate = {
  subject,
  intent: { targetId: "c.ts", position: "after", operation: "move" },
};

const reactAuthority: ReactAuthority = authority;
const reactSubject: ReactSubject = subject;
const reactCandidate: ReactCandidate = candidate;

const convenience: CoreProps = {
  onReorder: (_from, _to, _position) => {
    void _from;
    void _to;
    void _position;
  },
};
const exclusive: CoreProps = { reorderAuthority: authority };
const reactConvenience: ReactProps = convenience;
const reactExclusive: ReactProps = exclusive;
const reactTreeConvenience: TreeProps = convenience;
const reactTreeExclusive: TreeProps = exclusive;

export const movingCounts: number[] = [
  reactAuthority.projectMovingValues("a.ts", []).length,
  reactSubject.movingValues.length,
  reactCandidate.subject.movingValues.length,
  reactConvenience.onReorder ? 1 : 0,
  reactExclusive.reorderAuthority ? 1 : 0,
  reactTreeConvenience.onReorder ? 1 : 0,
  reactTreeExclusive.reorderAuthority ? 1 : 0,
];
