<script lang="ts">
  import { Button, IconButton, Menubar } from "@poodle/svelte";
  import { AppHeader } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

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

<SpecimenLayout bareVariants>
  {#snippet children()}
    <div class="poodle-specimen">
      <SpecimenGroup label="Full app window header (title + menubar + utility)" bare>
        <div class="poodle-specimen__app-shell">
          <AppHeader title="Poodle Studio">
            {#snippet actions()}
              <div class="poodle-specimen__menubar-inline">
                <Menubar items={menuItems} ariaLabel="Application menu" />
              </div>
            {/snippet}
            {#snippet utility()}
              <IconButton icon="search" sizeRole="chrome" variant="ghost" ariaLabel="Search" />
              <IconButton icon="bell" sizeRole="chrome" variant="ghost" ariaLabel="Notifications" />
              <IconButton icon="settings" sizeRole="chrome" variant="ghost" ariaLabel="Settings" />
            {/snippet}
          </AppHeader>
          <div class="poodle-specimen__app-body">
            <p>Application content area</p>
          </div>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="With title, actions, and utility" bare>
        <AppHeader title="My Application">
          {#snippet actions()}
            <Button sizeRole="chrome" variant="ghost">New</Button>
            <Button sizeRole="chrome" variant="ghost">Open</Button>
          {/snippet}
          {#snippet utility()}
            <IconButton icon="settings" sizeRole="chrome" variant="ghost" ariaLabel="Settings" />
          {/snippet}
        </AppHeader>
      </SpecimenGroup>

      <SpecimenGroup label="Title only" bare>
        <AppHeader title="Poodle Workstation" />
      </SpecimenGroup>

      <SpecimenGroup label="Custom identity slot" bare>
        <AppHeader>
          {#snippet identity()}
            <span class="poodle-specimen__logo">P</span>
            <strong>Poodle Studio</strong>
          {/snippet}
          {#snippet utility()}
            <IconButton icon="bell" sizeRole="chrome" variant="ghost" ariaLabel="Notifications" />
            <IconButton icon="user" sizeRole="chrome" variant="ghost" ariaLabel="Account" />
          {/snippet}
        </AppHeader>
      </SpecimenGroup>
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-app-header-specimen__variant-block">
      <div class="poodle-app-header-specimen__label">{density.toUpperCase()}</div>
      <AppHeader title="My Application" density={density}>
        {#snippet actions()}
          <Button sizeRole="chrome" variant="ghost">New</Button>
          <Button sizeRole="chrome" variant="ghost">Open</Button>
        {/snippet}
        {#snippet utility()}
          <IconButton icon="settings" sizeRole="chrome" variant="ghost" ariaLabel="Settings" />
        {/snippet}
      </AppHeader>
    </div>
  {/snippet}

  {#snippet sizes(size)}
    <div class="poodle-app-header-specimen__variant-block">
      <div class="poodle-app-header-specimen__label">{size.toUpperCase()}</div>
      <AppHeader title="My Application" size={size}>
        {#snippet actions()}
          <Button sizeRole="chrome" variant="ghost">New</Button>
          <Button sizeRole="chrome" variant="ghost">Open</Button>
        {/snippet}
        {#snippet utility()}
          <IconButton icon="settings" sizeRole="chrome" variant="ghost" ariaLabel="Settings" />
        {/snippet}
      </AppHeader>
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-specimen__app-shell {
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    overflow: hidden;
  }

  /* Strip the menubar's container chrome when inline in AppHeader */
  .poodle-specimen__menubar-inline :global(.poodle-menubar__list) {
    border: 0;
    background: transparent;
    padding: 0;
    gap: 0;
  }

  .poodle-specimen__app-body {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 8rem;
    color: var(--poodle-color-text-muted);
    font-size: 0.8125rem;
    background: var(--poodle-color-background-panel);
  }

  .poodle-specimen__app-body p {
    margin: 0;
  }

  .poodle-specimen__logo {
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

  .poodle-app-header-specimen__variant-block {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    width: min(40rem, 100%);
  }

  .poodle-app-header-specimen__label {
    color: var(--poodle-color-text-muted);
    font-size: 0.75rem;
    font-weight: 700;
    letter-spacing: 0.16em;
    text-transform: uppercase;
  }
</style>
