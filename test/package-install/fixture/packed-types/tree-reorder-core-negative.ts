// Expected-failure half of the packed TreeReorderProps proof (core).
import type {
  TreeReorderAuthority,
  TreeReorderProps,
} from "@inflatable-cookie/poodle-core";

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
