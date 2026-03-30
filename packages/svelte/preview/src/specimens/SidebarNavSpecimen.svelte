<script lang="ts">
  import { Eyebrow, Surface } from "@poodle/svelte-primitives";
  import { SidebarNav } from "@poodle/svelte-composites";
  import type { SidebarNavGroup } from "@poodle/svelte-composites";

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

<div class="specimen">
  <Surface>
    <div class="specimen__section">
      <Eyebrow>Single-group catalogue</Eyebrow>
      <div class="specimen__frame">
        <SidebarNav
          ariaLabel="Catalogue navigation"
          groups={catalogueGroups}
          value={catalogueValue}
          on:valueChange={(event) => (catalogueValue = event.detail.value)}
        />
      </div>
    </div>
  </Surface>

  <Surface>
    <div class="specimen__section">
      <Eyebrow>Grouped verification nav</Eyebrow>
      <div class="specimen__frame">
        <SidebarNav
          ariaLabel="Verification navigation"
          groups={harnessGroups}
          value={harnessValue}
          on:valueChange={(event) => (harnessValue = event.detail.value)}
        />
      </div>
    </div>
  </Surface>
</div>

<style>
  .specimen {
    display: grid;
    gap: 1rem;
  }

  .specimen__section {
    display: grid;
    gap: 0.75rem;
  }

  .specimen__frame {
    width: 16rem;
    min-height: 20rem;
    border-right: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 60%, transparent);
    overflow: auto;
  }
</style>
