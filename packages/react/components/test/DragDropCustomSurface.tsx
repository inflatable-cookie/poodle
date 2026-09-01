import type { DragDropCommitResult, DropIntent } from "@inflatable-cookie/poodle-core";

import { DragDropProvider } from "../src/drag-drop";
import { DragDropReorderList } from "./DragDropReorderList";

interface Item {
  id: string;
  label: string;
}

interface Props {
  itemsA?: Item[];
  itemsB?: Item[];
  rejectA?: boolean;
  onDropA?: (intent: DropIntent) => DragDropCommitResult | Promise<DragDropCommitResult>;
  onDropB?: (intent: DropIntent) => DragDropCommitResult | Promise<DragDropCommitResult>;
  onPreviewX?: (x: number) => void;
}

export function DragDropCustomSurface({
  itemsA = [{ id: "a", label: "Alpha" }],
  itemsB = [{ id: "b", label: "Beta" }],
  rejectA = false,
  onDropA,
  onDropB,
  onPreviewX,
}: Props) {
  return (
    <>
      <div data-testid="scope-a">
        <DragDropProvider
          preview={
            onPreviewX
              ? (pose) => {
                  onPreviewX(pose.x);
                  return pose.label;
                }
              : undefined
          }
        >
          <DragDropReorderList items={itemsA} kind="scope-a" reject={rejectA} onDrop={onDropA} />
        </DragDropProvider>
      </div>
      <div data-testid="scope-b">
        <DragDropProvider>
          <DragDropReorderList items={itemsB} kind="scope-b" onDrop={onDropB} />
        </DragDropProvider>
      </div>
    </>
  );
}
