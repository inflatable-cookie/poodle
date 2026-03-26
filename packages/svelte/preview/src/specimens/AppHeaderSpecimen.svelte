<script lang="ts">
  import { Eyebrow, Button, IconButton, Menubar } from "@poodle/svelte-primitives";
  import { AppHeader } from "@poodle/svelte-composites";

  const menuItems = [
    {
      value: "file",
      label: "File",
      items: [
        { value: "new", label: "New File", shortcutLabel: "Ctrl+N" },
        { value: "open", label: "Open...", shortcutLabel: "Ctrl+O" },
        { value: "save", label: "Save", shortcutLabel: "Ctrl+S" },
        { value: "sep1", label: "", kind: "separator" as const },
        { value: "exit", label: "Exit" },
      ],
    },
    {
      value: "edit",
      label: "Edit",
      items: [
        { value: "undo", label: "Undo", shortcutLabel: "Ctrl+Z" },
        { value: "redo", label: "Redo", shortcutLabel: "Ctrl+Shift+Z" },
        { value: "sep1", label: "", kind: "separator" as const },
        { value: "cut", label: "Cut", shortcutLabel: "Ctrl+X" },
        { value: "copy", label: "Copy", shortcutLabel: "Ctrl+C" },
        { value: "paste", label: "Paste", shortcutLabel: "Ctrl+V" },
      ],
    },
    {
      value: "view",
      label: "View",
      items: [
        { value: "sidebar", label: "Toggle Sidebar", shortcutLabel: "Ctrl+B" },
        { value: "terminal", label: "Toggle Terminal", shortcutLabel: "Ctrl+`" },
        { value: "sep1", label: "", kind: "separator" as const },
        { value: "zoom-in", label: "Zoom In", shortcutLabel: "Ctrl+=" },
        { value: "zoom-out", label: "Zoom Out", shortcutLabel: "Ctrl+-" },
      ],
    },
    {
      value: "help",
      label: "Help",
      items: [
        { value: "docs", label: "Documentation" },
        { value: "about", label: "About" },
      ],
    },
  ];
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Full app window header (title + menubar + utility)</Eyebrow>
    <div class="specimen__frame specimen__frame--app">
      <AppHeader title="Poodle Studio">
        <svelte:fragment slot="actions">
          <div class="specimen__menubar-inline">
            <Menubar items={menuItems} ariaLabel="Application menu" />
          </div>
        </svelte:fragment>
        <svelte:fragment slot="utility">
          <IconButton icon="search" sizeRole="chrome" variant="ghost" ariaLabel="Search" />
          <IconButton icon="bell" sizeRole="chrome" variant="ghost" ariaLabel="Notifications" />
          <IconButton icon="settings" sizeRole="chrome" variant="ghost" ariaLabel="Settings" />
        </svelte:fragment>
      </AppHeader>
      <div class="specimen__app-body">
        <p>Application content area</p>
      </div>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>With title, actions, and utility</Eyebrow>
    <div class="specimen__frame">
      <AppHeader title="My Application">
        <svelte:fragment slot="actions">
          <Button sizeRole="chrome" variant="ghost">New</Button>
          <Button sizeRole="chrome" variant="ghost">Open</Button>
        </svelte:fragment>
        <svelte:fragment slot="utility">
          <IconButton icon="settings" sizeRole="chrome" variant="ghost" ariaLabel="Settings" />
        </svelte:fragment>
      </AppHeader>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Title only</Eyebrow>
    <div class="specimen__frame">
      <AppHeader title="Poodle Workstation" />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Custom identity slot</Eyebrow>
    <div class="specimen__frame">
      <AppHeader>
        <svelte:fragment slot="identity">
          <span class="specimen__logo">P</span>
          <strong>Poodle Studio</strong>
        </svelte:fragment>
        <svelte:fragment slot="utility">
          <IconButton icon="bell" sizeRole="chrome" variant="ghost" ariaLabel="Notifications" />
          <IconButton icon="user" sizeRole="chrome" variant="ghost" ariaLabel="Account" />
        </svelte:fragment>
      </AppHeader>
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
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    overflow: visible;
  }

  .specimen__frame--app {
    overflow: hidden;
  }

  /* Strip the menubar's container chrome when inline in AppHeader */
  .specimen__menubar-inline :global(.menubar__list) {
    border: 0;
    background: transparent;
    padding: 0;
    gap: 0;
  }

  .specimen__app-body {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 8rem;
    color: var(--poodle-color-text-muted);
    font-size: 0.8125rem;
    background: var(--poodle-color-background-panel);
  }

  .specimen__app-body p {
    margin: 0;
  }

  .specimen__logo {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.5rem;
    height: 1.5rem;
    border-radius: 0.25rem;
    background: var(--poodle-color-accent-base);
    color: #fff;
    font-size: 0.75rem;
    font-weight: 700;
  }
</style>
