<script lang="ts">
  import { SidebarNav } from "@poodle/svelte";
  import type { SidebarNavGroup } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  let catalogueValue = "dock-region";
  let harnessValue = "pulse-runtime-foundation";

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
</script>

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
</div>

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
