<script lang="ts">
  import { PageLoading } from "@poodle/svelte";
  import { Button } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  let showIndeterminate = $state(false);
  let showDeterminate = $state(false);
  let showWithCancel = $state(false);
  let showInline = $state(true);
  let demoProgress = $state(0);
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

<div class="poodle-specimen">
  <SpecimenGroup label="Inline">
    <Button variant="secondary" onClick={() => (showInline = !showInline)}>
      Toggle inline loading
    </Button>
    {#if showInline}
      <div class="poodle-specimen__inline-shell">
        <PageLoading
          visible
          presentation="inline"
          message="Loading section content..."
        />
      </div>
    {/if}
  </SpecimenGroup>

  <SpecimenGroup label="Indeterminate (spinner only)">
    <Button variant="secondary" onClick={() => (showIndeterminate = true)}>
      Show loading overlay
    </Button>
    <PageLoading
      visible={showIndeterminate}
      message="Loading data..."
    />
  </SpecimenGroup>

  <SpecimenGroup label="Determinate (with progress bar)">
    <Button variant="secondary" onClick={startDeterminate}>
      Show progress overlay
    </Button>
    <PageLoading
      visible={showDeterminate}
      value={demoProgress}
      message="Uploading files... {demoProgress}%"
    />
  </SpecimenGroup>

  <SpecimenGroup label="With cancel button">
    <Button variant="secondary" onClick={() => (showWithCancel = true)}>
      Show cancellable loading
    </Button>
    <PageLoading
      visible={showWithCancel}
      message="Processing request..."
      canCancel
      onCancel={closeAll}
    />
  </SpecimenGroup>

  {#if showIndeterminate || showDeterminate || showWithCancel}
    <button class="poodle-specimen__dismiss" onclick={closeAll}>
      Dismiss overlay (click here if stuck)
    </button>
  {/if}
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-specimen__inline-shell {
    min-height: 18rem;
    border: 1px dashed var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 94%, transparent);
  }

  .poodle-specimen__dismiss {
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
