<script lang="ts">
  import { PageLoading } from "@flint/svelte-composites";
  import { Eyebrow, Button } from "@flint/svelte-primitives";

  let showIndeterminate = false;
  let showDeterminate = false;
  let showWithCancel = false;
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
  <div class="specimen__group">
    <Eyebrow>Indeterminate (spinner only)</Eyebrow>
    <Button variant="secondary" on:click={() => (showIndeterminate = true)}>
      Show loading overlay
    </Button>
    <PageLoading
      isVisible={showIndeterminate}
      message="Loading data..."
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Determinate (with progress bar)</Eyebrow>
    <Button variant="secondary" on:click={startDeterminate}>
      Show progress overlay
    </Button>
    <PageLoading
      isVisible={showDeterminate}
      value={demoProgress}
      message="Uploading files... {demoProgress}%"
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>With cancel button</Eyebrow>
    <Button variant="secondary" on:click={() => (showWithCancel = true)}>
      Show cancellable loading
    </Button>
    <PageLoading
      isVisible={showWithCancel}
      message="Processing request..."
      canCancel
      on:cancel={closeAll}
    />
  </div>

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
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__dismiss {
    position: fixed;
    bottom: 1rem;
    right: 1rem;
    z-index: 10000;
    padding: 0.5rem 1rem;
    border: 1px solid var(--flint-color-border-default);
    border-radius: var(--flint-radius-control);
    background: var(--flint-color-background-elevated);
    color: var(--flint-color-text-primary);
    font: inherit;
    font-size: 0.75rem;
    cursor: pointer;
  }
</style>
