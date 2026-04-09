<script lang="ts">
  import { PageLoading } from "@poodle/svelte-composites";
  import { Button } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  let showIndeterminate = false;
  let showDeterminate = false;
  let showWithCancel = false;
  let showInline = true;
  let demoProgress = 0;
  let progressTimer: ReturnType<typeof setInterval> | null = null;

  function startDeterminate() {
    demoProgress = 0;
    showDeterminate = true;
    progressTimer = setInterval(() => {
      demoProgress += 8;
      if (demoProgress >= 100) {
        demoProgress = 100;
        if (progressTimer) clearInterval(progressTimer);
        setTimeout(() => {
          showDeterminate = false;
          demoProgress = 0;
        }, 600);
      }
    }, 300);
  }

  function closeAll() {
    showIndeterminate = false;
    showDeterminate = false;
    showWithCancel = false;
    if (progressTimer) clearInterval(progressTimer);
    demoProgress = 0;
  }
</script>

<div class="specimen">
  <SpecimenGroup label="Inline">
    <Button variant="secondary" on:click={() => (showInline = !showInline)}>
      Toggle inline loading
    </Button>
    {#if showInline}
      <div class="specimen__inline-shell">
        <PageLoading
          visible
          presentation="inline"
          message="Loading section content..."
        />
      </div>
    {/if}
  </SpecimenGroup>

  <SpecimenGroup label="Indeterminate (spinner only)">
    <Button variant="secondary" on:click={() => (showIndeterminate = true)}>
      Show loading overlay
    </Button>
    <PageLoading
      visible={showIndeterminate}
      message="Loading data..."
    />
  </SpecimenGroup>

  <SpecimenGroup label="Determinate (with progress bar)">
    <Button variant="secondary" on:click={startDeterminate}>
      Show progress overlay
    </Button>
    <PageLoading
      visible={showDeterminate}
      value={demoProgress}
      message="Uploading files... {demoProgress}%"
    />
  </SpecimenGroup>

  <SpecimenGroup label="With cancel button">
    <Button variant="secondary" on:click={() => (showWithCancel = true)}>
      Show cancellable loading
    </Button>
    <PageLoading
      visible={showWithCancel}
      message="Processing request..."
      canCancel
      on:cancel={closeAll}
    />
  </SpecimenGroup>

  {#if showIndeterminate || showDeterminate || showWithCancel}
    <button class="specimen__dismiss" on:click={closeAll}>
      Dismiss overlay (click here if stuck)
    </button>
  {/if}
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen__inline-shell {
    min-height: 18rem;
    border: 1px dashed var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 94%, transparent);
  }

  .specimen__dismiss {
    position: fixed;
    bottom: 1rem;
    right: 1rem;
    z-index: 10000;
    padding: 0.5rem 1rem;
    border: 1px solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-elevated);
    color: var(--poodle-color-text-primary);
    font: inherit;
    font-size: 0.75rem;
    cursor: pointer;
  }
</style>
