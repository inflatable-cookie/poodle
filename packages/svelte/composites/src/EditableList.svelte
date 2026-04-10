<!-- @deprecated Use ReorderableList with editable={true} instead. -->
<script lang="ts">
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "@poodle/svelte-primitives";
  import ReorderableList from "./ReorderableList.svelte";
  import type { ReorderableItem } from "./types";

  export let items: ReorderableItem[] = [];
  export let addLabel = "Add item";
  export let placeholder = "New item";
  export let maxItems: number | null = null;
  export let disabled = false;
  export let ariaLabel = "List";
  export let reorderable = true;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;
</script>

<ReorderableList
  bind:items
  {ariaLabel}
  {disabled}
  {reorderable}
  editable
  {addLabel}
  addPlaceholder={placeholder}
  {maxItems}
  removable
  {size}
  {sizeRole}
  {density}
  on:change
  on:reorder
/>
