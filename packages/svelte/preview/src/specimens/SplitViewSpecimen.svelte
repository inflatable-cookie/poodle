<script lang="ts">
  import { Eyebrow } from "@pug/svelte-primitives";
  import { SplitView } from "@pug/svelte-workstation";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Horizontal split</Eyebrow>
    <div class="specimen__frame">
      <SplitView orientation="horizontal">
        <div slot="primary" class="specimen__pane">
          <p>Primary pane</p>
        </div>
        <div slot="secondary" class="specimen__pane">
          <p>Secondary pane</p>
        </div>
      </SplitView>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Vertical split</Eyebrow>
    <div class="specimen__frame">
      <SplitView orientation="vertical">
        <div slot="primary" class="specimen__pane">
          <p>Primary pane</p>
        </div>
        <div slot="secondary" class="specimen__pane">
          <p>Secondary pane</p>
        </div>
      </SplitView>
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
    height: 12rem;
    border: 0.0625rem solid var(--pug-color-border-subtle);
    border-radius: var(--pug-radius-surface);
    overflow: hidden;
  }

  .specimen__pane {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    font-size: 0.8125rem;
    color: var(--pug-color-text-secondary);
  }

  .specimen__pane p {
    margin: 0;
  }
</style>
