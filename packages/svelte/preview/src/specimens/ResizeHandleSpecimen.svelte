<script lang="ts">
  import { Eyebrow } from "@poodle/svelte-primitives";
  import { ResizeHandle } from "@poodle/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Horizontal split (vertical handle — drag left/right)</Eyebrow>
    <div class="specimen__row">
      <div class="specimen__pane">Left</div>
      <div class="specimen__handle-wrapper specimen__handle-wrapper--horizontal">
        <ResizeHandle orientation="horizontal" ariaLabel="Resize horizontal" />
      </div>
      <div class="specimen__pane">Right</div>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Vertical split (horizontal handle — drag up/down)</Eyebrow>
    <div class="specimen__col">
      <div class="specimen__pane">Top</div>
      <div class="specimen__handle-wrapper specimen__handle-wrapper--vertical">
        <ResizeHandle orientation="vertical" ariaLabel="Resize vertical" />
      </div>
      <div class="specimen__pane">Bottom</div>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled (horizontal split)</Eyebrow>
    <div class="specimen__row">
      <div class="specimen__pane">Left</div>
      <div class="specimen__handle-wrapper specimen__handle-wrapper--horizontal">
        <ResizeHandle orientation="horizontal" disabled ariaLabel="Disabled resize" />
      </div>
      <div class="specimen__pane">Right</div>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled (vertical split)</Eyebrow>
    <div class="specimen__col">
      <div class="specimen__pane">Top</div>
      <div class="specimen__handle-wrapper specimen__handle-wrapper--vertical">
        <ResizeHandle orientation="vertical" disabled ariaLabel="Disabled resize vertical" />
      </div>
      <div class="specimen__pane">Bottom</div>
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

  .specimen__row {
    display: flex;
    align-items: stretch;
    height: 6rem;
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    overflow: hidden;
  }

  .specimen__col {
    display: flex;
    flex-direction: column;
    height: 10rem;
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    overflow: hidden;
  }

  .specimen__pane {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary);
    background: color-mix(in srgb, var(--poodle-color-background-panel) 50%, transparent);
  }

  .specimen__handle-wrapper--horizontal {
    height: 100%;
  }

  .specimen__handle-wrapper--vertical {
    width: 100%;
  }
</style>
