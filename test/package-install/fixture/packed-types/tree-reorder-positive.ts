// Positive packed proof for g16.036 Tree reorder authority types.
//
// A real installed consumer must see one `TreeReorderSubject`,
// `TreeReorderCandidate`, `TreeReorderAuthority`, and `TreeReorderProps`
// across core and both Svelte public import paths. React re-exports the same
// core types from its package root; that root is a TSX graph, so this fixture
// does not import it. Nothing here reaches into `src/` or reads a declaration
// file as text.
import type {
  TreeReorderAuthority as CoreAuthority,
  TreeReorderCandidate as CoreCandidate,
  TreeReorderProps as CoreProps,
  TreeReorderSubject as CoreSubject,
} from "@inflatable-cookie/poodle-core";
import type {
  TreeReorderAuthority as SvelteRootAuthority,
  TreeReorderCandidate as SvelteRootCandidate,
  TreeReorderProps as SvelteRootProps,
  TreeReorderSubject as SvelteRootSubject,
} from "@inflatable-cookie/poodle-svelte";
import type {
  TreeReorderAuthority as SvelteTypesAuthority,
  TreeReorderCandidate as SvelteTypesCandidate,
  TreeReorderProps as SvelteTypesProps,
  TreeReorderSubject as SvelteTypesSubject,
} from "@inflatable-cookie/poodle-svelte/types";

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

const svelteRootAuthority: SvelteRootAuthority = authority;
const svelteTypesAuthority: SvelteTypesAuthority = authority;
const svelteRootSubject: SvelteRootSubject = subject;
const svelteTypesSubject: SvelteTypesSubject = subject;
const svelteRootCandidate: SvelteRootCandidate = candidate;
const svelteTypesCandidate: SvelteTypesCandidate = candidate;

const convenience: CoreProps = {
  onReorder: (_from, _to, _position) => {
    void _from;
    void _to;
    void _position;
  },
};
const exclusive: CoreProps = { reorderAuthority: authority };
const svelteRootConvenience: SvelteRootProps = convenience;
const svelteTypesConvenience: SvelteTypesProps = convenience;
const svelteRootExclusive: SvelteRootProps = exclusive;
const svelteTypesExclusive: SvelteTypesProps = exclusive;

export const movingCounts: number[] = [
  svelteRootAuthority.projectMovingValues("a.ts", []).length,
  svelteTypesAuthority.projectMovingValues("a.ts", []).length,
  svelteRootSubject.movingValues.length,
  svelteTypesSubject.movingValues.length,
  svelteRootCandidate.subject.movingValues.length,
  svelteTypesCandidate.subject.movingValues.length,
  svelteRootConvenience.onReorder ? 1 : 0,
  svelteTypesConvenience.onReorder ? 1 : 0,
  svelteRootExclusive.reorderAuthority ? 1 : 0,
  svelteTypesExclusive.reorderAuthority ? 1 : 0,
];
