<script lang="ts">
  import { Tooltip, Button, Eyebrow } from "@poodle/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <div class="specimen__row">
      <Tooltip content="Save your changes">
        <Button variant="secondary">Hover me</Button>
      </Tooltip>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Placements</Eyebrow>
    <div class="specimen__row">
      <Tooltip content="Top tooltip" placement="top">
        <Button variant="ghost">Top</Button>
      </Tooltip>
      <Tooltip content="Bottom tooltip" placement="bottom">
        <Button variant="ghost">Bottom</Button>
      </Tooltip>
      <Tooltip content="Left tooltip" placement="left">
        <Button variant="ghost">Left</Button>
      </Tooltip>
      <Tooltip content="Right tooltip" placement="right">
        <Button variant="ghost">Right</Button>
      </Tooltip>
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

  .specimen__row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: center;
  }
</style>
