<script lang="ts">
  import type { DragDropCommitResult, DropIntent } from "@inflatable-cookie/poodle-core";

  import DragDropProvider from "../src/DragDropProvider.svelte";
  import DragDropReorderList from "./DragDropReorderList.svelte";

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
  }

  let {
    itemsA = [{ id: "a", label: "Alpha" }],
    itemsB = [{ id: "b", label: "Beta" }],
    rejectA = false,
    onDropA,
    onDropB,
  }: Props = $props();
</script>

<div data-testid="scope-a">
  <DragDropProvider>
    {#snippet preview(pose)}
      <span data-testid="custom-preview-x">{pose.x}</span>
      {pose.label}
    {/snippet}
    <DragDropReorderList items={itemsA} kind="scope-a" reject={rejectA} onDrop={onDropA} />
  </DragDropProvider>
</div>
<div data-testid="scope-b">
  <DragDropProvider>
    <DragDropReorderList items={itemsB} kind="scope-b" onDrop={onDropB} />
  </DragDropProvider>
</div>
