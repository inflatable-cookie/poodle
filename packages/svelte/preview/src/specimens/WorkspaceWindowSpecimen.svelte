<script lang="ts">
  import { Eyebrow } from "@pug/svelte-primitives";
  import { WorkspaceWindow } from "@pug/svelte-workstation";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>With surfaces</Eyebrow>
    <div class="specimen__frame">
      <WorkspaceWindow
        windowId="main"
        surfaces={[
          { id: "editor", label: "Editor" },
          { id: "preview", label: "Preview", isClosable: true },
        ]}
        activeSurfaceId="editor"
      >
        <div class="specimen__window-content">
          <p>Window content — receives slot props for surfaces, active state, etc.</p>
        </div>
      </WorkspaceWindow>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Empty window</Eyebrow>
    <div class="specimen__frame">
      <WorkspaceWindow windowId="secondary" surfaces={[]} activeSurfaceId={null}>
        <div class="specimen__window-content specimen__window-content--empty">
          <p>No surfaces</p>
        </div>
      </WorkspaceWindow>
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
    height: 10rem;
    border: 0.0625rem solid var(--pug-color-border-subtle);
    border-radius: var(--pug-radius-surface);
    overflow: hidden;
  }

  .specimen__window-content {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding: 1rem;
    font-size: 0.8125rem;
    color: var(--pug-color-text-secondary);
  }

  .specimen__window-content--empty {
    color: var(--pug-color-text-muted);
    font-style: italic;
  }

  .specimen__window-content p {
    margin: 0;
  }
</style>
