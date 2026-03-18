<script lang="ts">
  import { Eyebrow } from "@pug/svelte-primitives";
  import { SplitDivider } from "@pug/svelte-workstation";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Horizontal with collapse affordances</Eyebrow>
    <div class="specimen__row">
      <div class="specimen__pane">Left</div>
      <SplitDivider
        orientation="horizontal"
        showCollapseBefore
        showCollapseAfter
      />
      <div class="specimen__pane">Right</div>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Vertical with collapse affordances</Eyebrow>
    <div class="specimen__col">
      <div class="specimen__pane">Top</div>
      <SplitDivider
        orientation="vertical"
        showCollapseBefore
        showCollapseAfter
      />
      <div class="specimen__pane">Bottom</div>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Resize only (no collapse)</Eyebrow>
    <div class="specimen__row">
      <div class="specimen__pane">Left</div>
      <SplitDivider orientation="horizontal" />
      <div class="specimen__pane">Right</div>
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
    border: 0.0625rem solid var(--pug-color-border-subtle);
    border-radius: var(--pug-radius-surface);
    overflow: hidden;
  }

  .specimen__col {
    display: flex;
    flex-direction: column;
    height: 10rem;
    border: 0.0625rem solid var(--pug-color-border-subtle);
    border-radius: var(--pug-radius-surface);
    overflow: hidden;
  }

  .specimen__pane {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.8125rem;
    color: var(--pug-color-text-secondary);
    background: color-mix(in srgb, var(--pug-color-background-panel) 50%, transparent);
  }
</style>
