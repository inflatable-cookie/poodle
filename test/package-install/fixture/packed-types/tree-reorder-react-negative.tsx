// Expected-failure half of the packed TreeReorderProps proof, React root.
//
// Authority and onReorder are mutually exclusive on the exported `TreeProps`
// JSX boundary, not only on the standalone alias. This file is compiled on
// its own and MUST fail with a real diagnostic: no compiler-suppression
// comment, escape-hatch type, or cast.
import type { TreeProps, TreeReorderAuthority } from "@inflatable-cookie/poodle-react";

const authority: TreeReorderAuthority = {
  projectMovingValues: (source) => [source],
  canDrop: (candidate) => ({ accepted: true, intent: candidate.intent }),
  onDrop: () => ({ status: "committed" }),
};

export const bothProps: TreeProps = {
  reorderAuthority: authority,
  onReorder: (_from: string, _to: string, _position: "before" | "after" | "inside") => {
    void _from;
    void _to;
    void _position;
  },
};

function TreeBoundary(props: TreeProps) {
  void props;
  return null;
}

export const bothJsx = (
  <TreeBoundary
    reorderAuthority={authority}
    onReorder={(_from: string, _to: string, _position: "before" | "after" | "inside") => {
      void _from;
      void _to;
      void _position;
    }}
  />
);
