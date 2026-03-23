<script lang="ts">
  import { ToastStack, type ToastItem } from "@poodle/svelte-composites";
  import { Eyebrow, Button } from "@poodle/svelte-primitives";

  let nextId = 4;
  let items: ToastItem[] = [
    { id: "1", title: "Changes saved", message: "Your settings have been updated.", tone: "success" },
    { id: "2", title: "New version available", message: "Update to v2.1 for the latest features.", tone: "info", actionLabel: "Update" },
    { id: "3", title: "Rate limit warning", message: "You are approaching your API limit.", tone: "warning" },
  ];

  function addToast() {
    const tones: Array<"info" | "success" | "warning" | "danger"> = ["info", "success", "warning", "danger"];
    const tone = tones[nextId % tones.length];
    items = [
      ...items,
      { id: String(nextId++), title: `Notification #${nextId - 1}`, message: "This is a new toast message.", tone },
    ];
  }
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Interactive stack</Eyebrow>
    <Button variant="secondary" size="sm" on:click={addToast}>Add toast</Button>
    <div class="toast-container">
      <ToastStack
        {items}
        on:dismiss={(e) => (items = items.filter((t) => t.id !== e.detail.id))}
        on:action={(e) => (items = items.filter((t) => t.id !== e.detail.id))}
      />
    </div>
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .toast-container {
    position: relative;
    min-height: 12rem;
  }
</style>
