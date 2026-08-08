<script lang="ts">
  import { ActionDiscoveryPanel } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const groupedItems = [
    { id: "save", title: "Save", shortcut: "Ctrl+S", group: "File" },
    { id: "open", title: "Open File", shortcut: "Ctrl+O", group: "File" },
    { id: "close", title: "Close Tab", shortcut: "Ctrl+W", group: "File" },
    { id: "find", title: "Find in Files", shortcut: "Ctrl+Shift+F", group: "Edit" },
    { id: "replace", title: "Find and Replace", shortcut: "Ctrl+H", group: "Edit" },
    { id: "terminal", title: "Toggle Terminal", shortcut: "Ctrl+`", group: "View" },
    { id: "sidebar", title: "Toggle Sidebar", shortcut: "Ctrl+B", group: "View" },
  ];

  const descriptiveItems = [
    { id: "deploy", title: "Deploy to Production", description: "Push current branch to production environment", badge: "Dangerous", group: "CI/CD" },
    { id: "preview", title: "Open Preview", description: "Launch preview in a new tab", shortcut: "Ctrl+Shift+P", group: "CI/CD" },
    { id: "lint", title: "Run Linter", shortcut: "Ctrl+Shift+L", group: "Tools" },
  ];

  const variantItems = [
    { id: "save", title: "Save", shortcut: "Ctrl+S", group: "File" },
    { id: "deploy", title: "Deploy", badge: "Dangerous", group: "Release" },
  ];
</script>

<SpecimenLayout bareVariants>
  {#snippet children()}
    <div class="poodle-specimen">
      <SpecimenGroup label="Grouped actions">
        <div class="poodle-specimen__frame poodle-specimen__frame--bare">
          <ActionDiscoveryPanel items={groupedItems} ariaLabel="Demo actions" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="With descriptions and badges">
        <div class="poodle-specimen__frame poodle-specimen__frame--bare">
          <ActionDiscoveryPanel items={descriptiveItems} ariaLabel="CI actions" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Empty state">
        <div class="poodle-specimen__frame poodle-specimen__frame--bare">
          <ActionDiscoveryPanel items={[]} state="empty" ariaLabel="Empty actions" />
        </div>
      </SpecimenGroup>
    </div>
  {/snippet}

  {#snippet sizes(size)}
    <div class="poodle-adp-specimen__variant-block">
      <SpecimenGroup label={size.toUpperCase()}>
        <div class="poodle-specimen__frame poodle-specimen__frame--bare">
          <ActionDiscoveryPanel items={variantItems} size={size} ariaLabel={`${size} actions`} />
        </div>
      </SpecimenGroup>
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-adp-specimen__variant-block">
      <SpecimenGroup label={density.toUpperCase()}>
        <div class="poodle-specimen__frame poodle-specimen__frame--bare">
          <ActionDiscoveryPanel items={variantItems} density={density} ariaLabel={`${density} actions`} />
        </div>
      </SpecimenGroup>
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-specimen__frame {
    width: min(32rem, 100%);
    max-width: 32rem;
    max-height: 20rem;
    overflow: hidden;
  }

  .poodle-specimen__frame--bare {
    border: 0;
    border-radius: 0;
    padding: 0;
    max-height: none;
  }

  .poodle-specimen__frame:not(.poodle-specimen__frame--bare) {
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    padding: 0.75rem;
  }

  .poodle-adp-specimen__variant-block {
    width: min(32rem, 100%);
  }

</style>
