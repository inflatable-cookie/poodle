<script lang="ts">
  import type { DragDropCommitResult, DropIntent } from "@inflatable-cookie/poodle-core";

  import { useDragDrop } from "../src/drag-drop";

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

  let { items, kind = "item", reject = false, onDrop }: Props = $props();

  const { dragSource, dropTarget, snapshot } = useDragDrop();

  function sourceRegistration(item: Item) {
    return {
      sourceId: item.id,
      subject: { kind, id: item.id },
      allowedOperations: ["move"] as const,
      label: item.label,
      keyboardOrder: 0,
    };
  }

  function targetRegistration() {
    return {
      targetId: `${kind}-list`,
      acceptedKinds: [kind],
      label: "List",
      resolvePosition: () => "inside" as const,
      canDrop: (intent: DropIntent) =>
        reject ? { accepted: false as const, reason: "occupied" } : { accepted: true as const, intent },
      onDrop: (intent: DropIntent) => onDrop?.(intent) ?? { status: "committed" as const },
    };
  }
</script>

<ul
  class="poodle-drag-drop-list"
  data-testid="drop-list"
  use:dropTarget={targetRegistration()}
>
  {#each items as item (item.id)}
    <li
      class="poodle-drag-drop-item"
      data-testid={`source-${item.id}`}
      use:dragSource={sourceRegistration(item)}
    >
      {item.label}
    </li>
  {/each}
</ul>

<div data-testid="announcement">{$snapshot.announcement ?? ""}</div>
<div data-testid="preview-x">{$snapshot.preview?.x ?? ""}</div>
