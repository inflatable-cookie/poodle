<script lang="ts">
  import { TriStateSwitch, Eyebrow, type TriStateValue } from "@poodle/svelte-primitives";

  let filter: TriStateValue = "default";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <TriStateSwitch
      value={filter}
      ariaLabel="Filter mode"
      on:valueChange={(e) => (filter = e.detail.value)}
    />
    <p>Value: <strong>{filter}</strong></p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Custom labels</Eyebrow>
    <TriStateSwitch
      options={{ excluded: "Hide", default: "All", included: "Show" }}
      ariaLabel="Visibility filter"
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <TriStateSwitch value="included" disabled ariaLabel="Disabled switch" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Custom semantic colors</Eyebrow>
    <TriStateSwitch
      value={filter}
      excludedColor="#ef4444"
      defaultColor="#64748b"
      includedColor="#22c55e"
      ariaLabel="Filter mode with custom semantic colors"
      on:valueChange={(e) => (filter = e.detail.value)}
    />
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

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
