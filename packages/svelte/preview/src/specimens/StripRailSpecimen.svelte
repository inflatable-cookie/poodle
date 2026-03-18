<script lang="ts">
  import { Eyebrow } from "@pug/svelte-primitives";
  import { StripRail } from "@pug/svelte-workstation";

  let activeItem = "explorer";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Vertical (left edge, icon mode)</Eyebrow>
    <div class="specimen__row">
      <div class="specimen__strip-frame specimen__strip-frame--vertical">
        <StripRail
          edge="left"
          mode="icon"
          items={[
            { id: "explorer", label: "Explorer", icon: "folder" },
            { id: "search", label: "Search", icon: "search" },
            { id: "git", label: "Source Control", icon: "git-branch", badge: "3" },
            { id: "debug", label: "Debug", icon: "bug" },
            { id: "extensions", label: "Extensions", icon: "puzzle", group: "secondary" },
          ]}
          value={activeItem}
          on:valueChange={(e) => (activeItem = e.detail.value)}
        />
      </div>
      <span class="specimen__hint">Active: {activeItem}</span>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Vertical (mixed mode)</Eyebrow>
    <div class="specimen__strip-frame specimen__strip-frame--vertical">
      <StripRail
        edge="left"
        mode="mixed"
        items={[
          { id: "files", label: "Files", icon: "folder" },
          { id: "search", label: "Search", icon: "search" },
          { id: "settings", label: "Settings", icon: "settings" },
        ]}
        value="files"
      />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Horizontal (top edge)</Eyebrow>
    <div class="specimen__strip-frame specimen__strip-frame--horizontal">
      <StripRail
        edge="top"
        mode="icon"
        items={[
          { id: "home", label: "Home", icon: "home" },
          { id: "docs", label: "Docs", icon: "book-open" },
          { id: "api", label: "API", icon: "code" },
        ]}
        value="home"
      />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Collapsed</Eyebrow>
    <div class="specimen__strip-frame specimen__strip-frame--vertical">
      <StripRail
        edge="left"
        mode="icon"
        items={[
          { id: "explorer", label: "Explorer", icon: "folder" },
          { id: "search", label: "Search", icon: "search" },
        ]}
        isCollapsed
      />
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
    align-items: center;
    gap: 0.75rem;
  }

  .specimen__strip-frame {
    border: 0.0625rem solid var(--pug-color-border-subtle);
    border-radius: var(--pug-radius-surface);
    overflow: hidden;
  }

  .specimen__strip-frame--vertical {
    height: 16rem;
    width: fit-content;
  }

  .specimen__strip-frame--horizontal {
    width: 20rem;
  }

  .specimen__hint {
    font-size: 0.75rem;
    color: var(--pug-color-text-secondary);
  }
</style>
