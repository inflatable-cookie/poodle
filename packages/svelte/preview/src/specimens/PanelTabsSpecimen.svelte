<script lang="ts">
  import { Eyebrow } from "@pug/svelte-primitives";
  import { PanelTabs } from "@pug/svelte-composites";

  let activeTab = "explorer";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <div class="specimen__frame">
      <PanelTabs
        items={[
          { value: "explorer", label: "Explorer", icon: "folder" },
          { value: "search", label: "Search", icon: "search" },
          { value: "git", label: "Source Control", icon: "git-branch" },
          { value: "debug", label: "Debug", icon: "bug" },
        ]}
        value={activeTab}
        ariaLabel="Panel tabs"
        on:valueChange={(e) => (activeTab = e.detail.value)}
      />
    </div>
    <p class="specimen__hint">Active: {activeTab}</p>
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

  .specimen__hint {
    font-size: 0.75rem;
    color: var(--pug-color-text-secondary);
    margin: 0;
  }
</style>
