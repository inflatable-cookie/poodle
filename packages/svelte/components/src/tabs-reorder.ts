import type { TabItem } from "./types";

export interface ReorderState {
  dragSourceIndex: number | null;
  dropTargetIndex: number | null;
}

export function createReorderState(): ReorderState {
  return {
    dragSourceIndex: null,
    dropTargetIndex: null,
  };
}

export function applyReorder(
  items: TabItem[],
  fromIndex: number,
  toIndex: number
): { items: TabItem[]; focusIndex: number } {
  if (fromIndex === toIndex) {
    return { items: [...items], focusIndex: fromIndex };
  }

  const nextItems = [...items];
  const [moved] = nextItems.splice(fromIndex, 1);
  nextItems.splice(toIndex, 0, moved);

  return { items: nextItems, focusIndex: toIndex };
}

export function handleDragStart(
  event: DragEvent,
  index: number,
  reorderable: boolean
): { dragSourceIndex: number | null; dataTransferSet: boolean } {
  if (!reorderable) {
    return { dragSourceIndex: null, dataTransferSet: false };
  }

  if (event.dataTransfer) {
    event.dataTransfer.effectAllowed = "move";
    event.dataTransfer.setData("text/plain", String(index));
  }

  return { dragSourceIndex: index, dataTransferSet: true };
}

export function handleDragOver(
  event: DragEvent,
  index: number,
  dragSourceIndex: number | null
): { dropTargetIndex: number | null; prevented: boolean } {
  if (dragSourceIndex === null) {
    return { dropTargetIndex: null, prevented: false };
  }

  event.preventDefault();
  if (event.dataTransfer) {
    event.dataTransfer.dropEffect = "move";
  }

  return { dropTargetIndex: index, prevented: true };
}

export function handleDrop(
  event: DragEvent,
  index: number,
  dragSourceIndex: number | null
): { fromIndex: number | null; toIndex: number | null } {
  event.preventDefault();

  if (dragSourceIndex !== null) {
    return { fromIndex: dragSourceIndex, toIndex: index };
  }

  return { fromIndex: null, toIndex: null };
}
