<script lang="ts">
  import EditableList from "../src/EditableList.svelte";

  interface Item {
    id: string;
    label?: string;
  }

  interface Props {
    items?: Item[];
    onReorder?: ((next: Item[]) => void) | null;
    onRemove?: ((id: string) => void) | null;
  }

  let { items = [], onReorder = null, onRemove = null }: Props = $props();
</script>

<EditableList {items} embeddedHandle editable {onReorder} {onRemove}>
  {#snippet item(entry)}
    <div contenteditable="plaintext-only" data-testid={`edit-${entry.id}`}>{entry.label}</div>
    <button type="button" data-testid={`action-${entry.id}`}>Action</button>
  {/snippet}
</EditableList>
