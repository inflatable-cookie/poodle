<script lang="ts">
  import { GridShell, Card } from "@pug/svelte-composites";
  import { Eyebrow } from "@pug/svelte-primitives";

  const items = [
    { id: "1", title: "Dashboard", desc: "Overview and metrics" },
    { id: "2", title: "Analytics", desc: "Traffic and engagement" },
    { id: "3", title: "Reports", desc: "Generated summaries" },
    { id: "4", title: "Settings", desc: "Configuration options" },
    { id: "5", title: "Users", desc: "Team management" },
    { id: "6", title: "Billing", desc: "Invoices and plans" },
  ];
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Ready state</Eyebrow>
    <div class="shell-container">
      <GridShell state="ready" ariaLabel="Projects grid" minColumnWidth="sm">
        {#each items as item (item.id)}
          <Card title={item.title} description={item.desc} />
        {/each}
      </GridShell>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Empty state</Eyebrow>
    <div class="shell-container">
      <GridShell state="empty" ariaLabel="Empty grid" stateTitle="No items" stateMessage="Create your first item to get started." />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Loading state</Eyebrow>
    <div class="shell-container">
      <GridShell state="loading" ariaLabel="Loading grid" />
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

  .shell-container {
    height: 14rem;
    border: 1px solid var(--pug-color-border-default);
    border-radius: 4px;
    overflow: hidden;
  }
</style>
