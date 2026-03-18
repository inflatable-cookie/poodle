<script lang="ts">
  import { Eyebrow } from "@pug/svelte-primitives";
  import { ShellStatusBar } from "@pug/svelte-workstation";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <div class="specimen__frame">
      <ShellStatusBar summary="Ready">
        <svelte:fragment slot="leading">
          <span class="specimen__status-item">main</span>
          <span class="specimen__status-item">0 errors</span>
        </svelte:fragment>
        <svelte:fragment slot="trailing">
          <span class="specimen__status-item">Ln 42, Col 18</span>
          <span class="specimen__status-item">UTF-8</span>
          <span class="specimen__status-item">TypeScript</span>
        </svelte:fragment>
      </ShellStatusBar>
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

  .specimen__frame {
    border: 0.0625rem solid var(--pug-color-border-subtle);
    border-radius: var(--pug-radius-surface);
    overflow: hidden;
  }

  .specimen__status-item {
    font-size: 0.75rem;
    color: var(--pug-color-text-secondary);
    padding: 0 0.375rem;
  }
</style>
