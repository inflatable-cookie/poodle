<script lang="ts">
  import { Eyebrow, UiPresentationProvider } from "@poodle/svelte-primitives";
  import { ActionDiscoveryPanel } from "@poodle/svelte-composites";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Grouped actions</Eyebrow>
    <div class="specimen__frame">
      <ActionDiscoveryPanel
        items={[
          { id: "save", title: "Save", shortcut: "Ctrl+S", group: "File" },
          { id: "open", title: "Open File", shortcut: "Ctrl+O", group: "File" },
          { id: "close", title: "Close Tab", shortcut: "Ctrl+W", group: "File" },
          { id: "find", title: "Find in Files", shortcut: "Ctrl+Shift+F", group: "Edit" },
          { id: "replace", title: "Find and Replace", shortcut: "Ctrl+H", group: "Edit" },
          { id: "terminal", title: "Toggle Terminal", shortcut: "Ctrl+`", group: "View" },
          { id: "sidebar", title: "Toggle Sidebar", shortcut: "Ctrl+B", group: "View" },
        ]}
        ariaLabel="Demo actions"
      />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>With descriptions and badges</Eyebrow>
    <div class="specimen__frame">
      <ActionDiscoveryPanel
        items={[
          { id: "deploy", title: "Deploy to Production", description: "Push current branch to production environment", badge: "Dangerous", group: "CI/CD" },
          { id: "preview", title: "Open Preview", description: "Launch preview in a new tab", shortcut: "Ctrl+Shift+P", group: "CI/CD" },
          { id: "lint", title: "Run Linter", shortcut: "Ctrl+Shift+L", group: "Tools" },
        ]}
        ariaLabel="CI actions"
      />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Empty state</Eyebrow>
    <div class="specimen__frame">
      <ActionDiscoveryPanel items={[]} state="empty" ariaLabel="Empty actions" />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Semantic presentation</Eyebrow>
    <UiPresentationProvider density="compact" sizeScale="sm">
      <div class="specimen__stack">
        <div class="specimen__frame">
          <ActionDiscoveryPanel
            items={[
              { id: "save", title: "Save", shortcut: "Ctrl+S", group: "File" },
              { id: "deploy", title: "Deploy", badge: "Dangerous", group: "Release" },
            ]}
          />
        </div>
        <div class="specimen__frame">
          <ActionDiscoveryPanel
            items={[
              { id: "save", title: "Save", shortcut: "Ctrl+S", group: "File" },
              { id: "deploy", title: "Deploy", badge: "Dangerous", group: "Release" },
            ]}
            sizeRole="prominent"
          />
        </div>
      </div>
    </UiPresentationProvider>
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
    max-width: 32rem;
    max-height: 20rem;
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    padding: 0.75rem;
    overflow: hidden;
  }

  .specimen__stack {
    display: grid;
    gap: 0.75rem;
  }
</style>
