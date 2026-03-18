<script lang="ts">
  import { Eyebrow } from "@pug/svelte-primitives";
  import { DockRegion } from "@pug/svelte-workstation";

  let activePanel = "explorer";
  let isCollapsed = false;
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Left dock with panels</Eyebrow>
    <div class="specimen__frame">
      <DockRegion
        edge="left"
        items={[
          { value: "explorer", label: "Explorer" },
          { value: "search", label: "Search" },
          { value: "git", label: "Source Control" },
        ]}
        value={activePanel}
        {isCollapsed}
        on:valueChange={(e) => (activePanel = e.detail.value)}
        on:collapsedChange={(e) => (isCollapsed = e.detail.collapsed)}
      >
        <div class="specimen__dock-content">
          <p>Panel: {activePanel}</p>
        </div>
      </DockRegion>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Empty dock</Eyebrow>
    <div class="specimen__frame">
      <DockRegion edge="right" items={[]} />
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
    height: 16rem;
    border: 0.0625rem solid var(--pug-color-border-subtle);
    border-radius: var(--pug-radius-surface);
    overflow: hidden;
  }

  .specimen__dock-content {
    padding: 0.75rem;
    font-size: 0.8125rem;
    color: var(--pug-color-text-secondary);
  }

  .specimen__dock-content p {
    margin: 0;
  }
</style>
