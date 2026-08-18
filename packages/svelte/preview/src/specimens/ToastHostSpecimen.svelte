<script lang="ts">
  import { writable } from "svelte/store";

  import { ToastHost, type ToastHostStoreItem } from "@inflatable-cookie/poodle-svelte";
  import { Button } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  type ToneSeed = "info" | "success" | "warning" | "error";

  let nextId = 4;
  const seedTones: ToneSeed[] = ["info", "success", "warning", "error"];

  const toasts = writable<ToastHostStoreItem[]>([
    { id: "1", variant: "success", title: "Saved", message: "Your changes have been stored." },
    { id: "2", variant: "warning", title: "Retry later", message: "Background sync is delayed." },
    { id: "3", variant: "error", message: "Publishing failed. Check your connection." },
  ]);

  const store = {
    toasts,
    dismiss(id: string) {
      toasts.update((items) => items.filter((item) => item.id !== id));
    }
  };

  function pushToast() {
    const variant = seedTones[nextId % seedTones.length];
    const id = String(nextId++);
    toasts.update((items) => [
      ...items,
      {
        id,
        variant,
        title: variant === "error" ? undefined : `Toast #${id}`,
        message: variant === "error" ? "This one stays until you dismiss it." : "A new runtime-host toast was added."
      }
    ]);
  }
</script>

<SpecimenLayout>
  <SpecimenGroup label="Runtime host">
    <p class="poodle-specimen__copy">
      The host owns timer policy and fixed positioning while `ToastStack` stays presentational.
    </p>
    <Button variant="secondary" onClick={pushToast}>Add toast</Button>
  </SpecimenGroup>

  <div class="poodle-specimen__surface">
    <ToastHost {store} />
  </div>

  {#snippet sizes(size)}
    <div class="poodle-specimen__surface">
      <ToastHost {store} {size} />
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-specimen__surface">
      <ToastHost {store} {density} />
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen__copy {
    margin: 0;
    color: var(--poodle-color-text-secondary);
  }

  .poodle-specimen__surface {
    position: relative;
    min-height: 16rem;
    border: 1px dashed color-mix(in srgb, var(--poodle-color-border-default) 82%, transparent);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-panel) 96%, transparent);
  }

  .poodle-specimen__surface :global(.poodle-toast-host) {
    position: absolute;
  }
</style>
