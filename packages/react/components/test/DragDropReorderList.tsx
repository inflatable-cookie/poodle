import type { DragDropCommitResult, DropIntent } from "@inflatable-cookie/poodle-core";

import { useDragDrop, useDragSource, useDropTarget } from "../src/drag-drop";

interface Item {
  id: string;
  label: string;
}

interface Props {
  items: Item[];
  kind?: string;
  reject?: boolean;
  onDrop?: (intent: DropIntent) => DragDropCommitResult | Promise<DragDropCommitResult>;
}

function SourceRow({ item, kind }: { item: Item; kind: string }) {
  const { getSourceProps } = useDragSource({
    sourceId: item.id,
    subject: { kind, id: item.id },
    allowedOperations: ["move"],
    label: item.label,
    keyboardOrder: 0,
  });

  return <li className="poodle-drag-drop-item" data-testid={`source-${item.id}`} {...getSourceProps()}>{item.label}</li>;
}

export function DragDropReorderList({ items, kind = "item", reject = false, onDrop }: Props) {
  const { snapshot } = useDragDrop();
  const { getTargetProps } = useDropTarget({
    targetId: `${kind}-list`,
    acceptedKinds: [kind],
    label: "List",
    resolvePosition: () => "inside",
    canDrop: (intent) => (reject ? { accepted: false, reason: "occupied" } : { accepted: true, intent }),
    onDrop: (intent) => onDrop?.(intent) ?? { status: "committed" },
  });

  return (
    <>
      <ul className="poodle-drag-drop-list" data-testid="drop-list" {...getTargetProps()}>
        {items.map((item) => <SourceRow key={item.id} item={item} kind={kind} />)}
      </ul>
      <div data-testid="announcement">{snapshot.announcement ?? ""}</div>
      <div data-testid="preview-x">{snapshot.preview?.x ?? ""}</div>
    </>
  );
}
