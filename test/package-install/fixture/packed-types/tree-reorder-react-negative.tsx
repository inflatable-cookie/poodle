// Expected-failure assignability half of the packed TreeReorderProps
// proof. Authority and onReorder are mutually exclusive on exported `TreeProps`.
// It MUST fail with a real diagnostic: no compiler-suppression comment,
// escape-hatch type, or cast. Public-root resolution is proved separately in
// the pack harness.
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
