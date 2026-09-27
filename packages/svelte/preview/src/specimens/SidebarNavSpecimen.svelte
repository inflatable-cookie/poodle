<script lang="ts">
  import { SidebarNav } from "@inflatable-cookie/poodle-svelte";
  import type { SidebarNavGroup } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let catalogueValue = $state("dock-region");
  let harnessValue = $state("pulse-runtime-foundation");
  let libraryValue = $state("videos");

  const catalogueGroups: SidebarNavGroup[] = [
    {
      id: "catalogue",
      items: [
        { value: "button", label: "Button" },
        { value: "dock-region", label: "DockRegion" },
        { value: "split-view", label: "SplitView" },
        { value: "tabs", label: "Tabs" },
      ],
    },
  ];

  const harnessGroups: SidebarNavGroup[] = [
    {
      id: "commands",
      label: "Commands",
      items: [{ value: "shared-commands", label: "Shared commands" }],
    },
    {
      id: "runtime",
      label: "Runtime",
      items: [
        { value: "device-monitor", label: "Device + monitor control" },
        { value: "pulse-runtime-foundation", label: "Pulse runtime foundation" },
        { value: "support-history", label: "Support + historical observability" },
      ],
    },
    {
      id: "shell",
      label: "Shell",
      items: [{ value: "shell-kernel", label: "Shell kernel" }],
    },
  ];

  const libraryGroups: SidebarNavGroup[] = [
    {
      id: "library",
      label: "Library",
      items: [
        { value: "videos", label: "Videos", endLabel: "198" },
        { value: "audio", label: "Audio", endLabel: "42" },
        { value: "images", label: "Images", endLabel: "1,204" },
        { value: "archive", label: "Archive", endLabel: "0", disabled: true },
      ],
    },
  ];
</script>

<SpecimenLayout>
  <div class="poodle-specimen">
    <SpecimenGroup label="Single-group catalogue">
      <div class="poodle-specimen__frame">
        <SidebarNav
          ariaLabel="Catalogue navigation"
          groups={catalogueGroups}
          value={catalogueValue}
          onValueChange={(value) => (catalogueValue = value)}
        />
      </div>
    </SpecimenGroup>

    <SpecimenGroup label="Grouped verification nav">
      <div class="poodle-specimen__frame">
        <SidebarNav
          ariaLabel="Verification navigation"
          groups={harnessGroups}
          value={harnessValue}
          onValueChange={(value) => (harnessValue = value)}
        />
      </div>
    </SpecimenGroup>

    <SpecimenGroup label="Library counts (endLabel)">
      <div class="poodle-specimen__frame">
        <SidebarNav
          ariaLabel="Library navigation"
          groups={libraryGroups}
          value={libraryValue}
          onValueChange={(value) => (libraryValue = value)}
        />
      </div>
    </SpecimenGroup>
  </div>

  {#snippet sizes(size)}
    <div class="poodle-specimen__frame">
      <SidebarNav
        ariaLabel={`${size} sidebar navigation`}
        groups={harnessGroups}
        value={harnessValue}
        {size}
      />
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-specimen__frame">
      <SidebarNav
        ariaLabel={`${density} sidebar navigation`}
        groups={harnessGroups}
        value={harnessValue}
        {density}
      />
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen {
    display: grid;
    gap: 1rem;
  }

  .poodle-specimen__frame {
    width: 16rem;
    min-height: 20rem;
    border-right: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 60%, transparent);
    overflow: auto;
  }
</style>
