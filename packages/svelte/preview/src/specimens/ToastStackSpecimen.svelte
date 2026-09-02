<script lang="ts">
  import { Button, ToastStack, type ToastItem } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let nextId = 4;
  let items = $state<ToastItem[]>([
    { id: "1", title: "Changes saved", message: "Your settings have been updated.", tone: "success" },
    { id: "2", title: "New version available", message: "Update to v2.1 for the latest features.", tone: "info", actionLabel: "Update" },
    { id: "3", title: "Rate limit warning", message: "You are approaching your API limit.", tone: "warning" },
    { id: "publish", title: "Publishing", message: "Still working.", tone: "info" },
  ]);

  function addToast(): void {
    const tones: Array<"info" | "success" | "warning" | "danger"> = ["info", "success", "warning", "danger"];
    const tone = tones[nextId % tones.length];
    items = [
      ...items,
      { id: String(nextId), title: `Notification #${nextId}`, message: "This is a new toast message.", tone },
    ];
    nextId += 1;
  }

  function settlePublish(): void {
    items = items.map((item) =>
      item.id === "publish"
        ? { id: "publish", title: "Published", message: "Your article is live.", tone: "success" }
        : item,
    );
  }
</script>

<SpecimenLayout showSizes={true} showDensities={true} bareVariants>
  {#snippet children()}
    <div class="poodle-specimen">
      <SpecimenGroup label="Interactive stack" bare>
        <div class="poodle-specimen__actions">
          <Button variant="secondary" sizeRole="chrome" onClick={addToast}>Add toast</Button>
          <Button variant="secondary" sizeRole="chrome" onClick={settlePublish}>Settle publish</Button>
        </div>
        <div class="poodle-toast-stack-specimen__variant">
          <ToastStack
            {items}
            onDismiss={(id) => (items = items.filter((item) => item.id !== id))}
            onAction={(id) => (items = items.filter((item) => item.id !== id))}
          />
        </div>
      </SpecimenGroup>
    </div>
  {/snippet}

  {#snippet sizes(size)}
    <div class="poodle-toast-stack-specimen__variant">
      <ToastStack
        items={[
          { id: `${size}-1`, title: `Toast at ${size}`, message: "Chrome scales with size.", tone: "info" },
          { id: `${size}-2`, title: "Action available", message: "Dismiss and action controls follow the same ladder.", tone: "success", actionLabel: "View" },
        ]}
        {size}
      />
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-toast-stack-specimen__variant">
      <ToastStack
        items={[
          { id: `${density}-1`, title: "Density example", message: "Spacing changes between compact, default, and comfortable.", tone: "warning" },
          { id: `${density}-2`, title: "Retry failed", message: "Action row and body spacing should ladder correctly.", tone: "danger", actionLabel: "Retry" },
        ]}
        {density}
      />
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-specimen__actions {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .poodle-toast-stack-specimen__variant {
    width: min(100%, 24rem);
  }
</style>
