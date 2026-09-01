// Expected-failure half of the packed TreeReorderProps proof, Svelte root.
//
// Authority and onReorder are mutually exclusive. This file is compiled on its
// own and MUST fail with a real diagnostic: no compiler-suppression comment,
// escape-hatch type, or cast.
import type {
  TreeReorderAuthority,
  TreeReorderProps,
} from "@inflatable-cookie/poodle-svelte";

const authority: TreeReorderAuthority = {
  projectMovingValues: (source) => [source],
  canDrop: (candidate) => ({ accepted: true, intent: candidate.intent }),
  onDrop: () => ({ status: "committed" }),
};

export const both: TreeReorderProps = {
  reorderAuthority: authority,
  onReorder: (_from: string, _to: string, _position: "before" | "after" | "inside") => {
    void _from;
    void _to;
    void _position;
  },
};
